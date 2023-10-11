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
    if !file_exists(&input_file) {
        // Return early if the file doesn't exist.
        let error_message = format!("File: {} not found.", input_file.display());
        return Err(Box::new(io::Error::new(io::ErrorKind::NotFound, error_message)));
    }

    // No need to check file_exists again; if we're here, it exists.
    if is_file_empty(&input_file)? {
        // Handle the case where the file is empty.
        let error_message = format!("File: {} exists but has no data", input_file.display());
        return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, error_message)));
    }

    // println!("File: {} exists!", input_file.display());

    let get_data = read_input(&input_file)?;
    let parsed_data = parse(get_data);

    let mut final_data = String::new();
    process_data(parsed_data, &mut final_data);
    Ok(final_data)
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

fn is_file_empty(filepath: &Path) -> Result<bool, io::Error> {
    let metadata = filepath.metadata()?;
    Ok(metadata.len() == 0)
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

/// TESTS
#[cfg(test)]
mod tests {
    // Import necessary items from the parent module
    use super::*;

    #[test]
    fn test_parse_line_basecolor() {
        let line = "hairpin_001_basecolor_1001.tga";
        let texture = parse_line(line);
        assert_eq!(texture.texture_type, TextureType::BaseColor);

        /// TODO:
        /// Decide if a texture that has no flags return a vec of Flag::None,
        /// or simply return an emtpy vec.
        /// And if there's a flag I haven't recognized right now its set as Flag::None
        /// but in the texture parsing I use TextureType::Unknown which might make more
        /// sense to use here as well.
        // assert_eq!(texture.flags, vec![Flag::None]);  // Assumes no flags for basecolor
        assert_eq!(texture.flags, vec![]);
    }

    #[test]
    fn test_parse_line_ao() {
        let line = "hairpin_001_ao_1001.tga -ao";
        let texture = parse_line(line);

        assert_eq!(texture.filename, "hairpin_001_ao_1001.tga");
        assert_eq!(texture.texture_type, TextureType::AO);
        assert_eq!(texture.flags, vec![Flag::AO]);
    }

    #[test]
    fn test_parse_line_nrm_highprec() {
        let line = "hairpin_001_nrm_1001.tga -nrm -highprec";
        let texture = parse_line(line);

        assert_eq!(texture.filename, "hairpin_001_nrm_1001.tga");
        assert_eq!(texture.texture_type, TextureType::Normal);
        assert_eq!(texture.flags, vec![Flag::NRM, Flag::HighPrec]);
    }

    #[test]
    fn test_parse_line_unknown_type_with_flags() {
        let line = "hairpin_001_mystery_1001.tga -highprec";
        let texture = parse_line(line);

        assert_eq!(texture.filename, "hairpin_001_mystery_1001.tga");
        assert_eq!(texture.texture_type, TextureType::Unknown);
        assert_eq!(texture.flags, vec![Flag::HighPrec]);
    }

    // Tests for extract_texture_type
    #[test]
    fn test_extract_texture_type_ao() {
        let texture_type = extract_texture_type("hairpin_001_ao_1001.tga");
        assert_eq!(texture_type, TextureType::AO);
    }

    #[test]
    fn test_extract_texture_type_unknown() {
        let texture_type = extract_texture_type("hairpin_001_mystery_1001.tga");
        assert_eq!(texture_type, TextureType::Unknown);
    }

    // Tests for extract_flags
    #[test]
    fn test_extract_flags_single() {
        let tokens = vec!["-ao"];
        let flags = extract_flags(tokens);
        assert_eq!(flags, vec![Flag::AO]);
    }

    #[test]
    fn test_extract_flags_multiple() {
        let tokens = vec!["-nrm", "-highprec"];
        let flags = extract_flags(tokens);
        assert_eq!(flags, vec![Flag::NRM, Flag::HighPrec]);
    }

    #[test]
    fn test_extract_flags_unknown() {
        let tokens = vec!["-mystery"];
        let flags = extract_flags(tokens);
        assert_eq!(flags, vec![Flag::None]);
    }

    // Tests for file_exists
    #[test]
    fn test_file_exists_true() {
        // Create a temporary file for this test
        let _file = File::create("temp_test_file.txt").unwrap();
        assert!(file_exists(&Path::new("temp_test_file.txt")));

        // Clean up the temporary file
        std::fs::remove_file("temp_test_file.txt").unwrap();
    }

    #[test]
    fn test_file_exists_false() {
        assert!(!file_exists(&Path::new("non_existent_file.txt")));
    }

    // Tests for read_input
    #[test]
    fn test_read_input_success() {
        // Create a temporary file for this test
        let _file = File::create("temp_read_input.txt").unwrap();

        let result = read_input(&Path::new("temp_read_input.txt"));
        assert!(result.is_ok());

        // Clean up the temporary file
        std::fs::remove_file("temp_read_input.txt").unwrap();
    }

    #[test]
    fn test_read_input_fail() {
        let result = read_input(&Path::new("non_existent_read_input.txt"));
        assert!(result.is_err());
    }

    // Tests for process_data
    #[test]
    fn test_process_data_valid_input() {
        let texture = TextureInfo {
            filename: "test_filename.tga".to_string(),
            texture_type: TextureType::AO,
            flags: vec![Flag::AO],
        };

        let mut final_data = String::new();
        process_data(vec![texture], &mut final_data);

        assert!(!final_data.is_empty());
    }

    #[test]
    fn test_process_data_empty_input() {
        let mut final_data = String::new();
        process_data(vec![], &mut final_data);

        // If the input file is actually empty we will never get to
        // use this function really, but in the case of testing the
        // process_data function directly with an empty vec then the
        // returning string will still contain data.
        // Mainly the headers like: ##### MIPS = --- etc
        assert_eq!(final_data.is_empty(), false);
    }

    // Tests for write_data
    #[test]
    fn test_write_data_to_file() {
        let udim = UDIM {
            input_file: Path::new("test.txt").to_path_buf(),
            process_data: "test data".to_string(),
        };

        // Write to a temp file
        let mut file = File::create("temp_write_output.txt").unwrap();
        let result = udim.write_data(&mut file);

        assert!(result.is_ok());

        // Clean up the temporary file
        std::fs::remove_file("temp_write_output.txt").unwrap();
    }

    #[test]
    fn test_write_data_to_stdout() {
        let udim = UDIM {
            input_file: Path::new("test.txt").to_path_buf(),
            process_data: "test data".to_string(),
        };

        let mut stdout = io::stdout();
        let result = udim.write_data(&mut stdout);

        assert!(result.is_ok());
    }

}

