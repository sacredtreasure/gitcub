use clap::Subcommand;

use crate::pr::Pr;

#[derive(Subcommand)]
pub enum Gitcub {
    /// co-ordinate pulls
    Pr {
        #[command(subcommand)]
        command: Pr,
    },
    /// log (in/out)
    Auth,
}

impl Gitcub {
    pub fn exec(&self) {
        match self {
            Gitcub::Auth => {
                println!("Authenticating")
            }
            Gitcub::Pr { command: Pr } => {
                command.exec();
            }

        }
    }
}