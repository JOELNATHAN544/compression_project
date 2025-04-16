// do_rle.rs

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use crate::lz::{compress_lz, decompress_lz};

pub fn handle_cli() {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];
    let input_path = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);

    let result = match command.as_str() {
        "compress" => compress_file(input_path, output_path),
        "decompress" => decompress_file(input_path, output_path),
        _ => {
            eprintln!("Invalid command. Use 'compress' or 'decompress'");
            return;
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

pub fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    let compressed = compress_lz(&data);
    
    let mut output = File::create(output_path)?;
    output.write_all(&compressed)?;
    
    Ok(())
}

pub fn decompress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    let mut input = File::open(input_path)?;
    let mut data = Vec::new();
    input.read_to_end(&mut data)?;

    let decompressed = decompress_lz(&data);
    
    let mut output = File::create(output_path)?;
    output.write_all(&decompressed)?;
    
    Ok(())
}
