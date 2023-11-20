use clap::Parser;
use subspace_farmer::{NodeClient, NodeRpcClient};
use tokio;

#[derive(Parser, Debug)]
pub struct FarmerAppInfo {
    #[arg(long)]
    url: String,
}

impl FarmerAppInfo {
    pub fn run(&self) {
        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        let farmer_app_info = runtime.block_on(async {
            let node_client = NodeRpcClient::new(&self.url).await.unwrap();
            node_client.farmer_app_info().await.unwrap()
        });

        println!("{:?}", farmer_app_info);
    }
}
