mod gitcub;

use clap::Parser;
use gitcub::Gitcub;

#[derive(Parser)]

struct Cli {
    #[command(subcommand)]
    command: Gitcub,
}

fn main() {
    let cli = Cli::parse(); 
}


