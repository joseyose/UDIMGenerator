mod udim;

use std::error::Error;
use crate::udim::UDIM;

use std::fs::File;
use std::io::{self, Write};
use std::path::{PathBuf};

use clap::{Parser};

/// Struct representing the command-line arguments.
/// Using `Option<PathBuf>` allows flexibility in input handling;
/// `None` signifies the absence of a provided file.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILEPATH", required = true)]
    input_file: Option<PathBuf>,

    // Allowing for an optional output file gives flexibility to the user
    // to either write to a file or, by default, print to the console.
    #[arg(short, long, value_name = "FILEPATH", required = false)]
    output_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello...you are running UDIMGenerator!");
    let cli = Args::parse();

    // Check if the user provided an input file.
    if let Some(input_path) = cli.input_file.as_deref() {

        // By using a `Box<dyn Write>`, we abstract the specifics of the writer.
        // This means we can easily switch between writing to stdout and a file
        // without changing the core logic that generates the data.
        let mut writer: Box<dyn Write> = if let Some(output_path) = cli.output_file.as_deref() {
            println!("User provided an output_filepath so we are writing to it");
            Box::new(File::create(output_path)?)
        } else {
            println!("User didn't provide an output_filepath so writing to stdout");
            Box::new(io::stdout())
        };

        // The `generate` function is expected to handle the logic of processing the input
        // and producing the desired output, which is then written to our writer.
        UDIM::generate(input_path)?.write_data(&mut writer)?;

        Ok(()) // <- Explicitly return Ok at the end of this branch
    } else {
        Ok(())
    }
}
