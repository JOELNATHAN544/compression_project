// Function to print usage instructions
function printUsage() {
    console.error('Usage:');
    console.error('Single file:  node index.js compress|decompress <input> <output> --rle|--lz');
    console.error('Multiple:     node index.js compress|decompress "input/*.txt" output_dir --rle|--lz');
    console.error('Stdin:        echo "text" | node index.js compress > output.bin');
    console.error('Stdout:       node index.js decompress input.bin');
    console.error('\nFor more details, use: node index.js --help');
    process.exit(1);
}

// Function to print detailed help
function printHelp() {
    console.error('File Compression Tool\n');
    console.error('A tool that supports Run-Length Encoding (RLE) and Lempel-Ziv (LZ) compression algorithms.');
    console.error('Can work with single files, multiple files using glob patterns, or stdin/stdout.\n');
    console.error('Usage: node index.js <operation> [input-pattern] [output-dir] [--rle|--lz]\n');
    console.error('Arguments:');
    console.error('  operation     Required. Either "compress" or "decompress"');
    console.error('  input-pattern Optional. File or glob pattern. Uses stdin if not provided');
    console.error('  output-dir    Optional. Output file/directory. Must be directory for glob patterns');
    console.error('  --rle|--lz    Optional. Compression algorithm to use. Auto-detects if not provided\n');
    console.error('Examples:');
    console.error('1. Single File Operations:');
    console.error('   node index.js compress input.txt output.txt --rle');
    console.error('   node index.js decompress input.compressed output.txt --lz\n');
    console.error('2. Multiple File Operations:');
    console.error('   # Compress all text files in current directory');
    console.error('   node index.js compress "*.txt" compressed_files --rle\n');
    console.error('   # Compress files in subdirectories');
    console.error('   node index.js compress "src/**/*.json" dist --lz\n');
    console.error('   # Decompress multiple files');
    console.error('   node index.js decompress "*.compressed" decompressed_files\n');
    console.error('3. Using Stdin/Stdout:');
    console.error('   echo "Hello World" | node index.js compress --rle > output.bin');
    console.error('   cat myfile.txt | node index.js compress > output.bin\n');
    console.error('4. Using Stdout as Output:');
    console.error('   node index.js decompress input.bin --rle');
    console.error('   node index.js decompress input.bin > output.txt\n');
    console.error('5. Auto-detection:');
    console.error('   # Auto-selects RLE for .txt files');
    console.error('   node index.js compress document.txt output.bin');
    console.error('   # Auto-selects LZ for binary files');
    console.error('   node index.js compress image.jpg output.bin\n');
    console.error('Note: When using glob patterns (*, **), the output argument must be a directory\n');
    console.error('Algorithm Selection:');
    console.error('  RLE: Better for text files (txt, json, xml, etc.)');
    console.error('  LZ:  Better for binary files (pdf, jpg, exe, etc.)');
    process.exit(0);
}

module.exports = { printUsage, printHelp }; 