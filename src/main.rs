mod args;

use clap::Parser;

#[macro_use]
extern crate simple_log;

fn main() {
    let args = args::Args::parse();

    let file = match args.file {
        Some(file) => file.into_os_string().into_string().unwrap(),
        None => {
            error!("No file provided");
            return;
        },
    };

    let input_string = args.input_string;

    run_cli(file, input_string);
}

// run_cli will print the file and input_string values
fn run_cli(file: String, input_string: String) {
    print!("File: {:?}\n", file);
    print!("Input string: {:?}\n", input_string);
}