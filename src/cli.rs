use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[clap(name = "timet", about, version, author)]
pub struct Cli {
    /// Month to get the time entries for, defaults to this month
    #[clap(short, long)]
    pub month: Option<u8>,
    /// Year to get the time entries for, defaults to this year
    #[clap(short, long)]
    pub year: Option<i32>,
    /// Create a new config file
    #[clap(short, long)]
    pub init: bool,
    /// Create shell completions
    #[clap(long, value_enum)]
    pub completions: Shell,
}
