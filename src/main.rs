use clap::Parser;

mod farmer;
mod node;

#[derive(Parser)]
enum Cli {
    #[clap(subcommand)]
    Node(node::Commands),
    #[clap(subcommand)]
    Farmer(farmer::Commands),
}
fn main() {
    let matches = Cli::parse();

    match matches {
        Cli::Node(inner) => inner.run(),
        Cli::Farmer(inner) => inner.run(),
    }
}
