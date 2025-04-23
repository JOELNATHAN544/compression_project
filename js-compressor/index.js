const { compress: lzCompress, decompress: lzDecompress } = require('./lz');
const { compress: rleCompress, decompress: rleDecompress } = require('./rle');
const { printUsage, printHelp } = require('./help');
const fs = require('fs');
const path = require('path');
const { promisify } = require('util');
const globCallback = require('glob');

// Correctly promisify the glob function
const glob = (pattern, options) => {
    return new Promise((resolve, reject) => {
        globCallback(pattern, options, (err, files) => {
            if (err) reject(err);
            else resolve(files);
        });
    });
};

// Function to detect best compression algorithm based on file extension
function detectBestAlgorithm(filename) {
    if (!filename) return 'rle';

    const ext = path.extname(filename).toLowerCase().slice(1);
    
    const textTypes = ['txt', 'json', 'xml', 'html', 'css', 'js', 'md'];
    if (textTypes.includes(ext)) {
        return 'rle';
    }

    const binaryTypes = ['pdf', 'doc', 'docx', 'zip', 'exe', 'jpg', 'png', 'gif'];
    if (binaryTypes.includes(ext)) {
        return 'lz';
    }

    return 'rle';
}

// Function to process a single file
async function processFile(inputPath, outputPath, operation, algorithm) {
    const input = await fs.promises.readFile(inputPath, 'utf8');
    let result;

    if (algorithm === 'lz') {
        result = operation === 'compress' ? lzCompress(input) : lzDecompress(input);
    } else {
        result = operation === 'compress' ? rleCompress(input) : rleDecompress(input);
    }

    await fs.promises.writeFile(outputPath, result);
    console.error(`Processed ${inputPath} -> ${outputPath} using ${algorithm}`);
}

// Function to handle multiple files
async function processMultipleFiles(inputPattern, outputDir, operation, algorithm) {
    const files = await glob(inputPattern);
    if (files.length === 0) {
        throw new Error(`No files found matching pattern: ${inputPattern}`);
    }

    // Create output directory if it doesn't exist
    await fs.promises.mkdir(outputDir, { recursive: true });

    for (const file of files) {
        const relativePath = path.relative(path.dirname(inputPattern), file);
        const outputPath = path.join(outputDir, relativePath);
        const outputExt = operation === 'compress' ? '.compressed' : '.decompressed';
        
        // Create subdirectories if needed
        await fs.promises.mkdir(path.dirname(outputPath), { recursive: true });
        
        // Process each file
        await processFile(
            file,
            outputPath + outputExt,
            operation,
            algorithm || detectBestAlgorithm(file)
        );
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
                const input = Buffer.concat(chunks).toString();
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

// Main function
async function main() {
    const args = process.argv.slice(2);
    
    if (args.includes('--help') || args.includes('-h')) {
        printHelp();
        return;
    }
    
    let operation = null;
    let inputPattern = null;
    let outputPath = null;
    let algorithm = null;
    
    for (let i = 0; i < args.length; i++) {
        const arg = args[i];
        if (arg === '--rle' || arg === '--lz') {
            algorithm = arg.slice(2);
        } else if (!operation) {
            operation = arg;
        } else if (!inputPattern) {
            inputPattern = arg;
        } else if (!outputPath) {
            outputPath = arg;
        }
    }

    if (!operation || !['compress', 'decompress'].includes(operation)) {
        console.error('Error: Invalid or missing operation');
        printUsage();
        return;
    }

    try {
        if (!inputPattern) {
            // Handle stdin/stdout
            await processStream(process.stdin, process.stdout, operation, algorithm);
        } else if (inputPattern.includes('*')) {
            // Handle multiple files
            if (!outputPath) {
                throw new Error('Output directory is required for multiple file processing');
            }
            await processMultipleFiles(inputPattern, outputPath, operation, algorithm);
        } else {
            // Handle single file
            const inputStream = fs.createReadStream(inputPattern);
            const outputStream = outputPath ? fs.createWriteStream(outputPath) : process.stdout;
            await processStream(inputStream, outputStream, operation, algorithm);
            
            if (outputPath) {
                console.error(`File ${operation}ed successfully using ${algorithm || 'auto-detected'} algorithm`);
            }
        }
    } catch (error) {
        console.error('Error:', error.message);
        printUsage();
    }
}

// Run main function if not imported as a module
if (require.main === module) {
    main().catch(error => {
        console.error('Error:', error.message);
        process.exit(1);
    });
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
