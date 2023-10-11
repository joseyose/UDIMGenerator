mod udim;
use crate::udim::UDIM;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error};
use std::path::{Path, PathBuf};

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILEPATH", required = true)]
    input_file: Option<PathBuf>,

    #[arg(short, long, value_name = "FILEPATH", required = false)]
    output_file: Option<PathBuf>,
}

fn main() {
    println!("Hello...you are running UDIMGenerator!");
    let cli = Args::parse();

    if let Some(input_path) = cli.input_file.as_deref() {


        // Write to file is the user provided it otherwise print to console
        if let Some(output_path) = cli.output_file.as_deref() {
            println!("User provided an output_filepath so we are writing to it");
            // write to a file
            let mut file = File::create("output.txt").unwrap();
            UDIM::new(input_path, output_path).write_data(&mut file).unwrap();
        } else {
            // write to std out
            println!("User didn't provide and output_filepath so writing to stdout");
            UDIM::new(input_path, Path::new("")).write_data(&mut io::stdout()).unwrap();
        }
    }
}
