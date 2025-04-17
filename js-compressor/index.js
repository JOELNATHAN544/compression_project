const { compress: lzCompress, decompress: lzDecompress } = require('./lz');
const { compress: rleCompress, decompress: rleDecompress } = require('./rle');
const fs = require('fs');

// If running directly (not required as a module)
if (require.main === module) {
    const args = process.argv.slice(2);
    const operation = args[0];
    const inputFile = args[1];
    const outputFile = args[2];
    const algorithm = args[3] === '--rle' ? 'rle' : args[3] === '--lz' ? 'lz' : null;

    if (!operation || !inputFile || !outputFile || !algorithm) {
        console.log('Usage: node index.js compress|decompress /path/to/input/file /path/to/output/file --rle|--lz');
        console.log('  operation: compress or decompress');
        console.log('  input: path to input file');
        console.log('  output: path to output file');
        console.log('  algorithm: --rle or --lz');
        process.exit(1);
    }

    try {
        // Read the input file
        const input = fs.readFileSync(inputFile, 'utf8');
        
        // Process the content
        let result;
        if (algorithm === 'lz') {
            result = operation === 'compress' ? lzCompress(input) : lzDecompress(input);
        } else if (algorithm === 'rle') {
            result = operation === 'compress' ? rleCompress(input) : rleDecompress(input);
        }

        // Write the result to the output file
        fs.writeFileSync(outputFile, result);
        console.log(`File ${operation}ed successfully using ${algorithm.toUpperCase()}`);
    } catch (error) {
        console.error('Error:', error.message);
        process.exit(1);
    }
}

// Export the compression functions for use as a module
module.exports = {
    lz: {
        compress: lzCompress,
        decompress: lzDecompress
    },
    rle: {
        compress: rleCompress,
        decompress: rleDecompress
    }
}; 