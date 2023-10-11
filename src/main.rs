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
            println!("Writing to: {}", output_path.display());
            Box::new(File::create(output_path)?)
        } else {
            println!("Writing to stdout");
            Box::new(io::stdout())
        };

        // The `generate` function is expected to handle the logic of processing the input
        // and producing the desired output, which is then written to our writer.
        UDIM::generate(input_path)?.write_data(&mut writer)?;

        let emoji = 0x1F525;
        let emoji = char::from_u32(emoji).expect("Not a valid code point");
        println!("Success {}", emoji);

        Ok(()) // <- Explicitly return Ok at the end of this branch
    } else {
        Ok(())
    }
}
