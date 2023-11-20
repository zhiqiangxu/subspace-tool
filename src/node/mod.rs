use clap::Parser;

mod farmer_app_info;

#[derive(Parser, Debug)]
pub enum Commands {
    FarmerAppInfo(farmer_app_info::FarmerAppInfo),
}

impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::FarmerAppInfo(inner) => inner.run(),
        }
    }
}
