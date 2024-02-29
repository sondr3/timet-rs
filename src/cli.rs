use std::io;

use clap::{Command, Parser};
use clap_complete::{generate, Generator, Shell};

#[derive(Parser, Debug)]
#[clap(name = "timet", about, version, author)]
pub struct Cli {
    /// Month to get the time entries for, defaults to this month
    #[clap(short, long)]
    pub month: Option<u8>,
    /// Year to get the time entries for, defaults to this year
    #[clap(short, long)]
    pub year: Option<i32>,
    /// Participated in a fagdag
    #[clap(short, long)]
    pub fagdag: bool,
    /// Create a new config file
    #[clap(short, long)]
    pub init: bool,
    /// Create shell completions
    #[clap(long, value_enum)]
    pub completions: Option<Shell>,
}

pub fn print_completion<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut io::stdout());
}
