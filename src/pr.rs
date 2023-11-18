use clap::Subcommand;

#[derive(Subcommand)]

pub enum Pr {
    /// spawn a pull
    Create,
    /// list pulls
    List,
}

impl Pr {
    pub fn exec(&self) {
        match self {
            Pr::Create => {
                println!("Pull Request Created!")
            }
            Pr::List => {
                println!("List of Pull Requests:")
            }
        }
    }
}