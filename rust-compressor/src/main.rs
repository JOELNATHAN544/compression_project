use clap::{Parser, ValueEnum, error::ErrorKind};
use std::env;
use std::fs;
use std::io::{self, Read, Write, stdin, stdout};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use anyhow::{Result, Context};
use glob::glob;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(
    name = "compression",
    about = "A file compression tool that supports Run-Length Encoding (RLE) and Lempel-Ziv (LZ) compression algorithms. Can work with files, multiple files using glob patterns, or standard input/output streams.",
    long_about = "A file compression tool that supports Run-Length Encoding (RLE) and Lempel-Ziv (LZ) compression algorithms. Can work with files, multiple files using glob patterns, or standard input/output streams.

EXAMPLES:
    1. Single File Operations:
       cargo run -- compress input.txt output.txt --algorithm rle
       cargo run -- decompress input.compressed output.txt --algorithm lz

    2. Multiple File Operations:
       # Compress all text files in current directory
       cargo run -- compress \"*.txt\" compressed_files --algorithm rle
       # Compress files in subdirectories
       cargo run -- compress \"src/**/*.json\" dist --algorithm lz
       # Decompress multiple files
       cargo run -- decompress \"*.compressed\" decompressed_files

    3. Using Stdin/Stdout:
       echo 'Hello' | cargo run -- compress > compressed.out
       cat compressed.out | cargo run -- decompress

    4. Auto-detection:
       # Auto-selects RLE for .txt files
       cargo run -- compress document.txt output.bin
       # Auto-selects LZ for binary files
       cargo run -- compress image.jpg output.bin

    5. Testing:
       cargo test

    Note: When using glob patterns (*, **), the output must be a directory"
)]
struct Cli {
    /// Operation to perform (compress or decompress)
    operation: String,

    /// Input file or glob pattern (e.g., "*.txt", "src/**/*.json"). Uses stdin if not provided
    input: Option<String>,

    /// Output file or directory (must be directory for glob patterns). Uses stdout if not provided
    output: Option<String>,

    /// Compression algorithm to use (auto-detects if not provided)
    #[arg(long, value_enum)]
    algorithm: Option<Algorithm>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
enum Algorithm {
    /// Run-Length Encoding (better for text files)
    Rle,
    /// Lempel-Ziv compression (better for binary files)
    Lz,
}

fn detect_algorithm(path: &Path) -> Algorithm {
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "txt" | "json" | "xml" | "html" | "css" | "js" | "md" => Algorithm::Rle,
        "pdf" | "doc" | "docx" | "zip" | "exe" | "jpg" | "png" | "gif" => Algorithm::Lz,
        _ => Algorithm::Rle,
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

fn process_file(input_path: &Path, output_path: &Path, operation: &str, algorithm: Algorithm) -> Result<()> {
    let mut input = String::new();
    fs::File::open(input_path)?.read_to_string(&mut input)?;

    let result = match (operation, algorithm) {
        ("compress", Algorithm::Rle) => rle_compress(input.as_bytes()),
        ("compress", Algorithm::Lz) => lz_compress(input.as_bytes()),
        ("decompress", Algorithm::Rle) => rle_decompress(input.as_bytes()),
        ("decompress", Algorithm::Lz) => lz_decompress(input.as_bytes()),
        _ => return Err(anyhow::anyhow!("Invalid operation")),
    };

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, result)?;
    eprintln!("Processed {} -> {} using {:?}", input_path.display(), output_path.display(), algorithm);
    Ok(())
}

fn process_multiple_files(pattern: &str, output_dir: &Path, operation: &str, algorithm: Option<Algorithm>) -> Result<()> {
    let paths: Vec<_> = glob(pattern)
        .context("Invalid glob pattern")?
        .filter_map(Result::ok)
        .collect();

    if paths.is_empty() {
        return Err(anyhow::anyhow!("No files found matching pattern: {}", pattern));
    }

    fs::create_dir_all(output_dir)?;

    for input_path in paths {
        let relative_path = input_path.strip_prefix(Path::new(pattern).parent().unwrap_or(Path::new("")))?;
        let mut output_path = output_dir.join(relative_path);
        output_path.set_extension(format!("{}.{}", 
            output_path.extension().unwrap_or_default().to_str().unwrap_or(""),
            if operation == "compress" { "compressed" } else { "decompressed" }
        ));

        let alg = algorithm.unwrap_or_else(|| detect_algorithm(&input_path));
        process_file(&input_path, &output_path, operation, alg)?;
    }

    Ok(())
}

fn process_stream(operation: &str, algorithm: Algorithm) -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result = match (operation, algorithm) {
        ("compress", Algorithm::Rle) => rle_compress(input.as_bytes()),
        ("compress", Algorithm::Lz) => lz_compress(input.as_bytes()),
        ("decompress", Algorithm::Rle) => rle_decompress(input.as_bytes()),
        ("decompress", Algorithm::Lz) => lz_decompress(input.as_bytes()),
        _ => return Err(anyhow::anyhow!("Invalid operation")),
    };

    io::stdout().write_all(&result)?;
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !["compress", "decompress"].contains(&cli.operation.as_str()) {
        eprintln!("Error: Invalid operation");
        eprintln!("\nUsage:");
        eprintln!("Normal:    cargo run compress|decompress <input> <output> --algorithm rle|lz");
        eprintln!("Multiple:  cargo run compress|decompress \"*.txt\" output_dir --algorithm rle|lz");
        eprintln!("Stdin:     echo \"text\" | cargo run compress --algorithm rle > output.bin");
        eprintln!("Stdout:    cargo run decompress input.bin --algorithm rle");
        eprintln!("\nFor more details, use: cargo run -- --help");
        std::process::exit(1);
    }

    match (&cli.input, &cli.output) {
        (None, _) => {
            // Handle stdin/stdout
            process_stream(&cli.operation, cli.algorithm.unwrap_or(Algorithm::Rle))?;
        }
        (Some(input), Some(output)) if input.contains('*') => {
            // Handle multiple files
            process_multiple_files(input, Path::new(output), &cli.operation, cli.algorithm)?;
        }
        (Some(input), output) => {
            // Handle single file
            let input_path = Path::new(input);
            let output_path = output
                .as_ref()
                .map(|o| PathBuf::from(o))
                .unwrap_or_else(|| {
                    let mut path = input_path.to_path_buf();
                    path.set_extension(format!("{}.{}", 
                        path.extension().unwrap_or_default().to_str().unwrap_or(""),
                        if cli.operation == "compress" { "compressed" } else { "decompressed" }
                    ));
                    path
                });

            let algorithm = cli.algorithm.unwrap_or_else(|| detect_algorithm(input_path));
            process_file(input_path, &output_path, &cli.operation, algorithm)?;
        }
    }

    Ok(())
}
