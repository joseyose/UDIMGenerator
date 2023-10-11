mod udim;

use std::error::Error;
use crate::udim::UDIM;

use std::fs::File;
use std::io::{self, Write};
use std::path::{PathBuf};

use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILEPATH", required = true)]
    input_file: Option<PathBuf>,

    #[arg(short, long, value_name = "FILEPATH", required = false)]
    output_file: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello...you are running UDIMGenerator!");
    let cli = Args::parse();

    if let Some(input_path) = cli.input_file.as_deref() {
        // Decide the writer: a file if the user provide an output path, stdout otherwise.
        let mut writer: Box<dyn Write> = if let Some(output_path) = cli.output_file.as_deref() {
            println!("User provided an output_filepath so we are writing to it");
            Box::new(File::create(output_path)?)
        } else {
            println!("User didn't provide an output_filepath so writing to stdout");
            Box::new(io::stdout())
        };

        UDIM::generate(input_path)?.write_data(&mut writer)?;

        Ok(()) // <- Explicitly return Ok at the end of this branch
    } else {
        Ok(())
    }
}
