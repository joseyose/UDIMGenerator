
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn create_udim() {

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
fn process(parsed: Vec<TextureInfo>) {
    print_filenames(&parsed);
    print_commands(&parsed);
}

fn print_filenames(parsed: &[TextureInfo]) {
    println!("MIPS = ");
    for texture in parsed {
        let formatted = format!("\t{:<50}\\", texture.filename);
        println!("{}", formatted);
    }
}

fn print_commands(parsed: &[TextureInfo]) {
    println!("\n##### MIP Commands Below for {} Textures #####", parsed.len());

    for texture in parsed {
        let udim_name = texture.filename.replace("1001", "UDIM").replace(".tga", ".mip");
        println!("{} : {}", udim_name, texture.filename);

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
        println!("\t$(MAKEMIP) {} -tile -priority {} -cal 1.0 {}", texture.filename, priority, flag_str);
    }
}