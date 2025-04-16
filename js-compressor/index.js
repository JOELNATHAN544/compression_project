const { compress: lzCompress, decompress: lzDecompress } = require('./lz');
const { compress: rleCompress, decompress: rleDecompress } = require('./rle');

// If running directly (not required as a module)
if (require.main === module) {
    const args = process.argv.slice(2);
    const algorithm = args[0];
    const operation = args[1];
    const input = args[2];

    if (!algorithm || !operation || !input) {
        console.log('Usage: node index.js <algorithm> <operation> <input>');
        console.log('  algorithm: lz or rle');
        console.log('  operation: compress or decompress');
        console.log('  input: string to process');
        process.exit(1);
    }

    try {
        let result;
        if (algorithm === 'lz') {
            result = operation === 'compress' ? lzCompress(input) : lzDecompress(input);
        } else if (algorithm === 'rle') {
            result = operation === 'compress' ? rleCompress(input) : rleDecompress(input);
        } else {
            console.log('Invalid algorithm. Use "lz" or "rle"');
            process.exit(1);
        }
        console.log(result);
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