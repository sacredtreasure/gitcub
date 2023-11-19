use clap::Subcommand;

#[derive(Subcommand)]

pub enum Pr {
    /// spawn a pull
    Create {
        #[arg(long, short)]
        title: String,
        draft: bool 
    },
    List,
    }
    /// list pulls
    
impl Pr {
    pub fn exec(&self) {
        match self {
            Pr::Create { title: &String, draft: bool} => {
                println!("PR with title {title} is created and the draft status is {draft}");
            }
            Pr::List => {
                println!("List of Pull Requests:");
            }
        }
    }
}
