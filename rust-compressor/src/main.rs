use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

use rust_compressor::{compress_lz, decompress_lz, compress_rle, decompress_rle};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <algorithm> <operation> <input_file>", args[0]);
        eprintln!("  algorithm: lz or rle");
        eprintln!("  operation: compress or decompress");
        std::process::exit(1);
    }

    let algorithm = &args[1];
    let operation = &args[2];
    let input_file = &args[3];

    // Validate algorithm and operation
    if !["lz", "rle"].contains(&algorithm.as_str()) {
        eprintln!("Error: algorithm must be 'lz' or 'rle'");
        std::process::exit(1);
    }

    if !["compress", "decompress"].contains(&operation.as_str()) {
        eprintln!("Error: operation must be 'compress' or 'decompress'");
        std::process::exit(1);
    }

    // Read input file
    let mut input_data = Vec::new();
    let mut file = fs::File::open(input_file)?;
    file.read_to_end(&mut input_data)?;

    // Process based on algorithm and operation
    let output_data = match (algorithm.as_str(), operation.as_str()) {
        ("lz", "compress") => compress_lz(&input_data),
        ("lz", "decompress") => decompress_lz(&input_data),
        ("rle", "compress") => compress_rle(&input_data),
        ("rle", "decompress") => decompress_rle(&input_data),
        _ => unreachable!(), // We've already validated the inputs
    };

    // Create output filename
    let input_path = Path::new(input_file);
    let output_file = match operation.as_str() {
        "compress" => input_path.with_extension(format!("{}.compressed", algorithm)),
        "decompress" => {
            let stem = input_path.file_stem().unwrap().to_str().unwrap();
            let stem = stem.trim_end_matches(&format!(".{}", algorithm));
            input_path.with_file_name(format!("{}.decompressed", stem))
        }
        _ => unreachable!(),
    };

    // Write output file
    let mut output_file = fs::File::create(output_file)?;
    output_file.write_all(&output_data)?;

    Ok(())
}
