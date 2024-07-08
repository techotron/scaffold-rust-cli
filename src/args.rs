use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(about = "A generic CLI example")]
pub struct Args {
    // Input file to read in
    #[arg(short, long, value_name = "FILE", group = "input")]
    pub file: Option<PathBuf>,

    // Some input string to use
    #[arg(short, long, default_value = "my-default-value")]
    pub input_string: String,
}