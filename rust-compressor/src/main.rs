use clap::{Parser, ValueEnum, error::ErrorKind};
use std::env;
use std::fs;
use std::io::{self, Read, Write, stdin, stdout};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};

#[derive(Parser)]
#[command(
    name = "compression",
    about = "A file compression tool supporting RLE and LZ algorithms",
    long_about = "A file compression tool that supports Run-Length Encoding (RLE) and Lempel-Ziv (LZ) compression algorithms. \
                  Can work with files or standard input/output streams.",
    after_help = "EXAMPLES:\n\
    1. Compress a file using RLE:\n\
       cargo run -- compress input.txt output.txt --algorithm rle\n\
    2. Decompress a file using LZ:\n\
       cargo run -- decompress input.compressed output.txt --algorithm lz\n\
    3. Auto-detect algorithm based on file type:\n\
       cargo run -- compress image.jpg compressed.out\n\
    4. Use stdin/stdout:\n\
       echo 'Hello' | cargo run -- compress > compressed.out\n\
       cat compressed.out | cargo run -- decompress\n\
    5. Chain operations:\n\
       echo 'Test' | cargo run -- compress | cargo run -- decompress"
)]
struct Cli {
    /// Operation to perform (compress or decompress)
    #[arg(value_enum)]
    operation: Operation,

    /// Input file (optional, uses stdin if not provided)
    /// Examples: 
    ///   - 'input.txt' to read from file
    ///   - omit to read from stdin
    #[arg(value_parser)]
    input: Option<PathBuf>,

    /// Output file (optional, uses stdout if not provided)
    /// Examples:
    ///   - 'output.txt' to write to file
    ///   - omit to write to stdout
    #[arg(value_parser)]
    output: Option<PathBuf>,

    /// Compression algorithm to use (optional, auto-detects if not provided)
    /// Examples:
    ///   - '--algorithm rle' for text files
    ///   - '--algorithm lz' for binary files
    ///   - omit to auto-detect based on file type
    #[arg(value_enum, short, long)]
    algorithm: Option<Algorithm>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Operation {
    /// Compress the input data
    Compress,
    /// Decompress the input data
    Decompress,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Algorithm {
    /// Run-Length Encoding (better for text files)
    Rle,
    /// Lempel-Ziv compression (better for binary files)
    Lz,
}

fn detect_best_algorithm(data: &[u8]) -> Algorithm {
    if let Some(kind) = infer::get(data) {
        match kind.mime_type() {
            // Use RLE for text-based files
            "text/plain" | "text/html" | "text/css" | "application/json" | "text/xml" => Algorithm::Rle,
            
            // Use LZ for binary/compressed files
            "application/pdf" | "application/zip" | "image/jpeg" | "image/png" | 
            "application/x-executable" | "application/x-msdos-program" => Algorithm::Lz,
            
            // Default to RLE for unknown types
            _ => Algorithm::Rle,
        }
    } else {
        // If we can't detect the type, check if it looks like text
        if data.iter().all(|&b| b.is_ascii()) {
            Algorithm::Rle
        } else {
            Algorithm::Lz
        }
    }
}

// RLE compression implementation
fn rle_compress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut count = 1;
    let mut current = input.get(0).copied();

    for &next in input.iter().skip(1) {
        if Some(next) == current {
            count += 1;
        } else {
            if let Some(c) = current {
                result.push(count as u8);
                result.push(c);
            }
            current = Some(next);
            count = 1;
        }
    }

    if let Some(c) = current {
        result.push(count as u8);
        result.push(c);
    }

    result
}

// RLE decompression implementation
fn rle_decompress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut iter = input.chunks_exact(2);

    while let Some(&[count, byte]) = iter.next() {
        result.extend(std::iter::repeat(byte).take(count as usize));
    }

    result
}

// LZ compression implementation
fn lz_compress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut dictionary = std::collections::HashMap::new();
    let mut current = Vec::new();
    let mut next_code = 256u16;

    for &byte in input {
        let mut next = current.clone();
        next.push(byte);

        if dictionary.contains_key(&next) {
            current = next;
        } else {
            // Output the code for current
            if !current.is_empty() {
                let default_code = byte as u16;
                let code = dictionary.get(&current).unwrap_or(&default_code);
                result.extend_from_slice(&code.to_be_bytes());
            }
            // Add new sequence to dictionary
            if next_code < 65535 {
                dictionary.insert(next, next_code);
                next_code += 1;
            }
            current = vec![byte];
        }
    }

    // Output any remaining sequence
    if !current.is_empty() {
        if let Some(&code) = dictionary.get(&current) {
            result.extend_from_slice(&code.to_be_bytes());
        } else {
            result.push(current[0]);
        }
    }

    result
}

// LZ decompression implementation
fn lz_decompress(input: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut dictionary = std::collections::HashMap::new();
    let mut next_code = 256u16;

    let mut iter = input.chunks_exact(2);
    while let Some(code_bytes) = iter.next() {
        let code = u16::from_be_bytes([code_bytes[0], code_bytes[1]]);
        
        let sequence = if code < 256 {
            vec![code as u8]
        } else {
            dictionary.get(&code)
                .cloned()
                .unwrap_or_else(|| vec![code as u8])
        };

        result.extend(&sequence);

        if !sequence.is_empty() && next_code < 65535 {
            let mut prev_sequence = sequence.clone();
            if let Some(&next_byte) = input.get(iter.len() * 2) {
                prev_sequence.push(next_byte);
                dictionary.insert(next_code, prev_sequence);
                next_code += 1;
            }
        }
    }

    result
}

fn process_data(operation: Operation, algorithm: Algorithm, input: &[u8]) -> Result<Vec<u8>> {
    Ok(match (operation, algorithm) {
        (Operation::Compress, Algorithm::Rle) => rle_compress(input),
        (Operation::Decompress, Algorithm::Rle) => rle_decompress(input),
        (Operation::Compress, Algorithm::Lz) => lz_compress(input),
        (Operation::Decompress, Algorithm::Lz) => lz_decompress(input),
    })
}

fn main() -> Result<()> {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            // For invalid arguments, show simple help
            if matches!(err.kind(), ErrorKind::UnknownArgument) {
                eprintln!("Error: {}\n", err);
                eprintln!("Usage:");
                eprintln!("Normal:    cargo run -- compress|decompress <input> <output> --algorithm rle|lz");
                eprintln!("Stdin:     echo 'text' | cargo run -- compress > output.bin");
                eprintln!("Stdout:    cargo run -- decompress input.bin");
                eprintln!("\nFor more details, use: cargo run -- --help");
                std::process::exit(1);
            }
            return Err(err.into());
        }
    };
    
    // Read input
    let mut input_data = Vec::new();
    match cli.input {
        Some(path) => {
            let mut file = fs::File::open(&path)
                .with_context(|| format!("Failed to open input file: {}", path.display()))?;
            file.read_to_end(&mut input_data)?;
        }
        None => {
            stdin().lock().read_to_end(&mut input_data)?;
        }
    }
    
    // Detect algorithm if not specified
    let algorithm = match cli.algorithm {
        Some(alg) => alg,
        None => {
            let detected = detect_best_algorithm(&input_data);
            eprintln!("Auto-detected algorithm: {:?}", detected);
            detected
        }
    };
    
    // Process the data
    let output_data = process_data(cli.operation, algorithm, &input_data)?;
    
    // Write output
    match cli.output {
        Some(path) => {
            let mut file = fs::File::create(&path)
                .with_context(|| format!("Failed to create output file: {}", path.display()))?;
            file.write_all(&output_data)?;
            eprintln!("Operation completed successfully using {:?} algorithm", algorithm);
        }
        None => {
            stdout().write_all(&output_data)?;
        }
    }
    
    Ok(())
}
