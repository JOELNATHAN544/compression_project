const { compress: lzCompress, decompress: lzDecompress } = require('./lz');
const { compress: rleCompress, decompress: rleDecompress } = require('./rle');
const fs = require('fs');
const path = require('path');
const fileType = require('file-type');

// Function to detect best compression algorithm based on file type
async function detectBestAlgorithm(buffer) {
    try {
        const type = await fileType.fromBuffer(buffer);
        if (!type) return 'rle'; // Default to RLE if type can't be determined

        // Use RLE for text-based files
        const textTypes = ['txt', 'json', 'xml', 'html', 'css', 'js', 'md'];
        if (textTypes.includes(type.ext)) {
            return 'rle';
        }

        // Use LZ for binary/compressed files
        const binaryTypes = ['pdf', 'doc', 'docx', 'zip', 'exe', 'jpg', 'png', 'gif'];
        if (binaryTypes.includes(type.ext)) {
            return 'lz';
        }

        return 'rle'; // Default to RLE
    } catch (error) {
        return 'rle'; // Default to RLE on error
    }
}

// Function to handle stream processing
async function processStream(inputStream, outputStream, operation, algorithm) {
    return new Promise((resolve, reject) => {
        const chunks = [];
        
        inputStream.on('data', chunk => chunks.push(chunk));
        inputStream.on('error', reject);
        
        inputStream.on('end', async () => {
            try {
                const buffer = Buffer.concat(chunks);
                const input = buffer.toString();
                
                // Auto-detect algorithm if not specified
                if (!algorithm) {
                    algorithm = await detectBestAlgorithm(buffer);
                    console.error(`Auto-detected algorithm: ${algorithm}`);
                }
                
                let result;
                if (algorithm === 'lz') {
                    result = operation === 'compress' ? lzCompress(input) : lzDecompress(input);
                } else {
                    result = operation === 'compress' ? rleCompress(input) : rleDecompress(input);
                }
                
                outputStream.write(result);
                outputStream.end();
                resolve();
            } catch (error) {
                reject(error);
            }
        });
    });
}

// If running directly (not required as a module)
if (require.main === module) {
    const args = process.argv.slice(2);
    const operation = args[0];
    const inputFile = args[1];
    const outputFile = args[2];
    const algorithm = args[3] === '--rle' ? 'rle' : args[3] === '--lz' ? 'lz' : null;

    if (!operation || operation !== 'compress' && operation !== 'decompress') {
        console.error('Usage: node index.js compress|decompress [input-file] [output-file] [--rle|--lz]');
        console.error('  operation: compress or decompress');
        console.error('  input-file: path to input file (optional, uses stdin if not provided)');
        console.error('  output-file: path to output file (optional, uses stdout if not provided)');
        console.error('  algorithm: --rle or --lz (optional, auto-detects if not provided)');
        process.exit(1);
    }

    try {
        const inputStream = inputFile ? fs.createReadStream(inputFile) : process.stdin;
        const outputStream = outputFile ? fs.createWriteStream(outputFile) : process.stdout;

        processStream(inputStream, outputStream, operation, algorithm)
            .then(() => {
                if (outputFile) {
                    console.error(`File ${operation}ed successfully using ${algorithm || 'auto-detected'} algorithm`);
                }
            })
            .catch(error => {
                console.error('Error:', error.message);
                process.exit(1);
            });
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