mod gitcub;
mod pr;

use crate::gitcub::Gitcub;
use clap::Parser;

#[derive(Parser)]

struct Cli {
    #[command(subcommand)]
    command: Gitcub,
}

fn main() {
    let cli = Cli::parse(); 
    cli.command.exec();
}


