
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Write};
use std::path::{Path, PathBuf};


// Okay, I think I have an idea of what I want.
// I think I want to create a UDIM object that will store the
// input_file and output_file destinations.
// It will also store the final parsed data
// And then based on the users needs it will either print to std or to file
pub struct UDIM {
    input_file: PathBuf,
    output_file: PathBuf,
    process_data: String,

}

impl UDIM {
    pub fn new(input_file: &Path, output_file: &Path) -> Self {
        let input_file_buf = input_file.to_path_buf();
        let process_data = start(&input_file_buf);

        UDIM {
            input_file: input_file_buf,
            output_file: output_file.to_path_buf(),
            process_data,
        }
    }

    pub fn print(&self) {
        println!("{}", self.process_data);
    }

    pub fn write_data<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writer.write_all(self.process_data.as_bytes())
    }


}

fn start(input_file: &Path) -> String {
    let mut final_data = String::new();
    //println!("{}", input_file.display());
    if file_exists(&input_file) {
        println!("File: {} exists!", input_file.display());
        // TODO: improve error handling here!
        let get_data = read_input(&input_file).unwrap();
        let parsed_data = parse(get_data);
        process_data(parsed_data, &mut final_data);
    } else {
        eprintln!("File: {} does not exist!", input_file.display());
    }

    final_data
}

// Read input
fn read_input(input_file: &Path) -> Result<BufReader<File>, Error> {
    let input = File::open(input_file)?;
    let buffered = BufReader::new(input);

    Ok(buffered)
}

fn file_exists(file_path: &Path) -> bool {
    file_path.exists()
}


// Parsing
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
// Now that I have the data parsed I want to assemble it into the makefile
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