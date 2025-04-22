# Compression Project

This project implements compression algorithms in both Rust and JavaScript, with Docker support for both implementations.

## Features

- RLE (Run-Length Encoding) compression
- LZ (Lempel-Ziv) compression
- CLI interface for both implementations
- Docker support
- Benchmarking capabilities

## Rust Implementation

### Building

```bash
cd rust-compressor
cargo build --release
```

### Running

```bash
# Compress a file
./target/release/compression compress input.txt output.txt rle

# Decompress a file
./target/release/compression decompress output.txt decompressed.txt rle
```

### Docker

```bash
# Build the image
docker build -t rust-compression .

# Run compression
docker run -v $(pwd):/data rust-compression compress -i /data/input.txt -o /data/output.txt -a rle

# Run decompression
docker run -v $(pwd):/data rust-compression decompress -i /data/output.txt -o /data/decompressed.txt -a rle
```

## JavaScript Implementation

### Installation

```bash
cd js-compressor
npm install
```

### Running

```bash
# Compress a file
node src/cli.js compress  input.txt output.txt rle

# Decompress a file
node src/cli.js decompress output.txt decompressed.txt rle
```

### Docker

```bash
# Build the image
docker build -t js-compression .

# Run compression
docker run -v $(pwd):/data js-compression compress /data/input.txt /data/output.txt rle

# Run decompression
docker run -v $(pwd):/data js-compression decompress /data/output.txt /data/decompressed.txt rle
```

## Benchmarking

Run the benchmark script to compare compression algorithms:

```bash
./benchmark.sh
```

This will generate a benchmark report in `benchmark_report.md`.

## Algorithms

### RLE (Run-Length Encoding)
- Simple compression algorithm
- Good for data with many repeated characters
- Example: "AAAABBBCCD" -> "4A3B2C1D"

### LZ (Lempel-Ziv)
- More complex compression algorithm
- Good for general-purpose compression
- Creates a dictionary of repeated patterns

