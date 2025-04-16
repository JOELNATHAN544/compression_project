function compress(input) {
    // Handle Buffer input
    if (Buffer.isBuffer(input)) {
        input = input.toString();
    }
    if (typeof input !== 'string') {
        throw new Error('Input must be a string');
    }

    let dictionary = new Map();
    let current = '';
    let result = [];
    let nextCode = 256; // Start after ASCII codes

    // Initialize dictionary with single characters
    for (let i = 0; i < 256; i++) {
        dictionary.set(String.fromCharCode(i), i);
    }
    
    for (let i = 0; i < input.length; i++) {
        const char = input[i];
        const phrase = current + char;
        
        if (dictionary.has(phrase)) {
            current = phrase;
        } else {
            // Output the code for current
            result.push(dictionary.get(current) || dictionary.get(char));
            // Add phrase to dictionary
            dictionary.set(phrase, nextCode++);
            current = char;
        }
    }
    
    // Output the code for any remaining input
    if (current.length > 0) {
        result.push(dictionary.get(current));
    }
    
    return result.join(',');
}

function decompress(input) {
    if (typeof input !== 'string') {
        throw new Error('Input must be a string');
    }

    if (input === '') return '';

    const codes = input.split(',').map(x => parseInt(x));
    let dictionary = new Map();
    let nextCode = 256;

    // Initialize dictionary with single characters
    for (let i = 0; i < 256; i++) {
        dictionary.set(i, String.fromCharCode(i));
    }

    let result = [dictionary.get(codes[0])];
    let current = result[0];
    
    for (let i = 1; i < codes.length; i++) {
        const code = codes[i];
        let entry;
        
        if (dictionary.has(code)) {
            entry = dictionary.get(code);
        } else if (code === nextCode) {
            entry = current + current[0];
        } else {
            throw new Error('Invalid compressed data');
        }
        
        result.push(entry);
        dictionary.set(nextCode++, current + entry[0]);
        current = entry;
    }
    
    return result.join('');
}

module.exports = {
    compress,
    decompress
}; 