use clap::Subcommand;

#[derive(Subcommand)]
pub enum Gitcub {
    /// co-ordinate pulls
    Pr,
    /// log (in/out)
    Auth,
}