// do_rle.rs

pub fn compress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count = 1;

        while i + count < data.len() && data[i + count] == byte && count < 255 {
            count += 1;
        }

        result.push(byte);
        result.push(count as u8);

        i += count;
    }

    result
}

pub fn decompress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i + 1 < data.len() {
        let byte = data[i];
        let count = data[i + 1];
        result.extend(std::iter::repeat(byte).take(count as usize));
        i += 2;
    }

    result
}

use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::{self, Read, Write};
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
