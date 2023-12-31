use clap::Command;

use crate::pr::Pr;

#[derive(Command)]
pub enum Gitcub {
    /// co-ordinate pulls
    Pr {
        #[command(command)]
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