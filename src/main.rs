use clap::Parser;

mod node;

#[derive(Parser)]
enum Cli {
    #[clap(subcommand)]
    Node(node::Commands),
}
fn main() {
    let matches = Cli::parse();

    match matches {
        Cli::Node(inner) => inner.run(),
    }
}
