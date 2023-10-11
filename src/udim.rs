use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::error::Error;

// Represents UDIM texture processing, encapsulating the input file and the processed data
pub struct UDIM {
    input_file: PathBuf,
    process_data: String,
}

impl UDIM {
    /// Generates a new UDIM structure by processing a give input file.
    /// This provides a clean separation between initialization and processing.
    pub fn generate(input_file: &Path) -> Result<Self, Box<dyn Error>> {
        let input_file_buf = input_file.to_path_buf();
        let process_data = start(&input_file_buf)?;

        Ok(UDIM {
            input_file: input_file_buf,
            process_data,
        })
    }

    /// Outputs the processed data to a provided writable stream.
    pub fn write_data<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.process_data.as_bytes())
    }
}

/// Main processing function to validate, read and parse the input file.
fn start(input_file: &Path) -> Result<String, Box<dyn Error>> {

    if file_exists(&input_file) {
        println!("File: {} exists!", input_file.display());

        let get_data = read_input(&input_file)?;
        let parsed_data = parse(get_data);

        let mut final_data = String::new();
        process_data(parsed_data, &mut final_data);
        Ok(final_data)
    } else {
        /// Provide a more specific error message when the input file doesn't exist.
        let error_message = format!("File: {} not found.", input_file.display());
        Err(Box::new(io::Error::new(io::ErrorKind::NotFound, error_message)))
    }
}

/// Reads the content of the input file and returns a buffered reader.
fn read_input(input_file: &Path) -> Result<BufReader<File>, io::Error> {
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);

    Ok(buffered)
}

fn file_exists(file_path: &Path) -> bool {
    file_path.exists()
}


/// Parsing functions to extract texture information from lines of input data.
/// Definitions to categorize the type of texture and additional attributes.
// Data Structures
#[derive(Debug, PartialEq)]
enum TextureType {
    AO,
    BaseColor,
    Normal,
    Spec,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum Flag {
    AO,
    NRM,
    HighPrec,
    Spec,
    None,
}

#[derive(Debug)]
struct TextureInfo {
    filename: String,
    texture_type: TextureType,
    flags: Vec<Flag>,
}

fn parse<R: BufRead>(reader: R) -> Vec<TextureInfo> {
    let mut textures = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            textures.push(parse_line(&l));
        }
    }
    textures
}

fn parse_line(line: &str) -> TextureInfo {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let filename = parts[0].to_string();
    let texture_type = extract_texture_type(&filename);
    let flags = extract_flags(parts[1..].to_vec());

    TextureInfo {
        filename,
        texture_type,
        flags
    }
}

fn extract_texture_type(filename: &str) -> TextureType {
    if filename.contains("_ao_") {
        TextureType::AO
    } else if filename.contains("_basecolor_") {
        TextureType::BaseColor
    } else if filename.contains("_nrm_") {
        TextureType::Normal
    } else if filename.contains("_spec") {
        TextureType::Spec
    } else {
        TextureType::Unknown
    }
}

fn extract_flags(tokens: Vec<&str>) -> Vec<Flag> {
    let mut flags = Vec::new();
    for token in tokens {
        match token {
            "-ao" => flags.push(Flag::AO),
            "-nrm" => flags.push(Flag::NRM),
            "-highprec" => flags.push(Flag::HighPrec),
            "-spec" => flags.push(Flag::Spec),
            _ => flags.push(Flag::None)
        }
    }
    flags
}

// # PROCESS THE DATA
fn process_data(parsed: Vec<TextureInfo>, x: &mut String) {
    x.push_str(&collect_filenames(&parsed));
    x.push_str(&collect_commands(&parsed));
}

fn collect_filenames(parsed: &[TextureInfo]) -> String {
    let mut filenames = format!("MIPS = \n");
    for texture in parsed {
        let formatted = format!("\t{:<50}\\\n", texture.filename);
        filenames += formatted.as_str();
    }

    return filenames;
}

fn collect_commands(parsed: &[TextureInfo]) -> String {
    let mut commands = format!("\n##### MIP Commands Below for {} Textures #####\n", parsed.len());

    for texture in parsed {
        let udim_name = texture.filename.replace("1001", "UDIM").replace(".tga", ".mip");
        let formatted = format!("{} : {}\n", udim_name, texture.filename);
        commands += &formatted;

        let mut priority = 0.1;
        let mut flag_str = String::new();

        for flag in &texture.flags {
            match flag {
                Flag::AO => priority = 0.9,
                Flag::Spec => {
                    priority = 0.1;
                    flag_str.push_str("-specmap ");
                },
                Flag::NRM => {
                    priority = 0.7;
                    flag_str.push_str("-normalmap ");
                }
                Flag::HighPrec => flag_str.push_str("-highprec "),
                _ => eprintln!("Warning: Unexpected flag detected")
            }
        }
        commands += &format!("\t$(MAKEMIP) {} -tile -priority {} -cal 1.0 {}\n", texture.filename, priority, flag_str);
    }
    return commands;
}