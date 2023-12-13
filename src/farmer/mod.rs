use clap::Parser;

mod audit;

#[derive(Parser, Debug)]
pub enum Commands {
    Audit(audit::Audit),
}

impl Commands {
    pub fn run(&self) {
        match &self {
            Commands::Audit(inner) => inner.run(),
        }
    }
}
