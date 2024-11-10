use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Parse provided HTML file
    Parse,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "HTML Parser",
    about = "A command line html parser written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// The path to the HTML file is going to be parsed
    #[structopt(parse(from_os_str), short, long)]
    pub file: Option<PathBuf>,
}
