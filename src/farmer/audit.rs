use clap::Parser;
use parity_scale_codec::{Decode, Encode};
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom};
use std::path::PathBuf;
use std::time::Instant;
use subspace_core_primitives::{Blake3Hash, SectorIndex, SolutionRange};
use subspace_farmer::{single_disk_farm::farming::rayon_files::RayonFiles, Identity};
use subspace_farmer_components::auditing::audit_plot_sync;
use subspace_farmer_components::file_ext::{FileExt, OpenOptionsExt};
use subspace_farmer_components::sector::SectorMetadataChecksummed;

#[derive(Debug, Encode, Decode)]
struct PlotMetadataHeader {
    version: u8,
    plotted_sector_count: SectorIndex,
}

impl PlotMetadataHeader {
    #[inline]
    fn encoded_size() -> usize {
        let default = PlotMetadataHeader {
            version: 0,
            plotted_sector_count: 0,
        };

        default.encoded_size()
    }
}

const PLOT_FILE: &'static str = "plot.bin";
const METADATA_FILE: &'static str = "metadata.bin";
const SUPPORTED_PLOT_VERSION: u8 = 0;
const RESERVED_PLOT_METADATA: u64 = 1024 * 1024;

#[derive(Parser, Debug)]
pub struct Audit {
    #[arg(long)]
    dir: String,
}

impl Audit {
    pub fn run(&self) {
        let dir: PathBuf = self.dir.clone().into();
        let identity = Identity::open(dir)
            .expect("fail to open identify fail")
            .expect("fail to open identify fail");
        let public_key = identity.public_key().to_bytes().into();

        let path_buf: PathBuf = [self.dir.clone(), PLOT_FILE.to_string()].iter().collect();
        let plot = RayonFiles::open(path_buf.as_path()).expect("fail to open plot.bin");

        let sectors_metadata = {
            let mut metadata_file = OpenOptions::new()
                .read(true)
                .advise_random_access()
                .open(
                    [self.dir.clone(), METADATA_FILE.to_string()]
                        .iter()
                        .collect::<PathBuf>()
                        .as_path(),
                )
                .expect("fail to open metadata.bin");
            metadata_file
                .advise_random_access()
                .expect("fail to advise_random_access");

            let sector_metadata_size = SectorMetadataChecksummed::encoded_size();

            let mut metadata_header_bytes = vec![0; PlotMetadataHeader::encoded_size()];
            metadata_file
                .read_exact_at(&mut metadata_header_bytes, 0)
                .expect("fail to read meta");
            let metadata_header = PlotMetadataHeader::decode(&mut metadata_header_bytes.as_ref())
                .expect("fail to decode meta");
            if metadata_header.version != SUPPORTED_PLOT_VERSION {
                panic!("unsupported plot version:{}", metadata_header.version);
            }

            let metadata_size = metadata_file.seek(SeekFrom::End(0)).expect("fail to seek");
            let expected_metadata_size = RESERVED_PLOT_METADATA
                + sector_metadata_size as u64 * u64::from(metadata_header.plotted_sector_count);
            if metadata_size != expected_metadata_size {
                panic!(
                    "metadata_size != expected_metadata_size, {} vs {}",
                    metadata_size, expected_metadata_size
                );
            }

            let mut sectors_metadata = Vec::<SectorMetadataChecksummed>::with_capacity(
                usize::from(metadata_header.plotted_sector_count),
            );

            let mut sector_metadata_bytes = vec![0; sector_metadata_size];
            for sector_index in 0..metadata_header.plotted_sector_count {
                metadata_file
                    .read_exact_at(
                        &mut sector_metadata_bytes,
                        RESERVED_PLOT_METADATA
                            + sector_metadata_size as u64 * u64::from(sector_index),
                    )
                    .expect("fail to read_exact_at");
                sectors_metadata.push(
                    SectorMetadataChecksummed::decode(&mut sector_metadata_bytes.as_ref())
                        .expect("fail to decode SectorMetadataChecksummed"),
                );
            }

            sectors_metadata
        };

        println!("sector count:{}", sectors_metadata.len());

        let start = Instant::now();
        audit_plot_sync(
            &public_key,
            &Blake3Hash::default(),
            SolutionRange::default(),
            &plot,
            &sectors_metadata,
            None,
        );
        println!("one audit took: {:?}", start.elapsed());
    }
}
