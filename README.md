# UDIMGenerator 🚀💖
A powerful utility to generate UDIM commands and data from texture filenames.
A CLI-tool I created for generating `Makefile` commands for `UDIM` based textures in our pipeline.

# Overview
`UDIMGenerator` is a cli utility I designed to streamline the generation of `Makefile` entries in our pipeline. 
In our engine the conversion of bitmaps into the game-engine-ready `.mip` files is managed through makefiles. 
This process, although functional, has its own complexities, especially with the incorporation of `UDIM` textures. 
The `UDIM` texture workflow introduced a new layer of intricacy to the already tedious makefile approach.

To alleviate these challenges and to automate parts of the process, I developed this `UDIMGenerator`. 
It primarily focuses on reading `UDIM` texture filenames and generating the precise `Makefile` commands needed to create 
the corresponding `.mip` files.

## Features
- Parse texture filenames and extract their types and associated flags.
- Generate commands and other related data based on our `Makefile` rules.
- Write the generated data to the console or to specified output files.
- Comprehensive error handling ensures that invalid or missing files don't disrupt your workflow.
- Unit test coverage.

## Usage
First, compile and run the program. You'll be prompted to input a file with the list of texture names.

```bash
$ cargo run -- [FLAGS] [OPTIONS]
```

## Flags & Options:
- `-i, --input` - The path to the input file containing texture names. (Required)
- `-o, --output` - The path to the output file where generated data should be saved. If not provided, data will be printed to the console.

Example:

```bash
$ cargo run -- -i ./textures.txt -o ./output.txt
```

This would read texture names from textures.txt, process them, and then save the generated data in output.txt.

## Expected Input Format
Your input file should contain texture filenames, one per line. Each filename can optionally be followed by space-separated flags. For instance:

```bash
hairpin_001_ao_1001.tga -ao
clip_002_basecolor_1001.tga
bezel_003_nrm_1001.tga -nrm -highprec
...
```
