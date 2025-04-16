function compress(input) {
    // Handle Buffer input
    if (Buffer.isBuffer(input)) {
        input = input.toString();
    }
    if (typeof input !== 'string') {
        throw new Error('Input must be a string');
    }

    if (input === '') return '';

    let result = '';
    let count = 1;
    let current = input[0];

    for (let i = 1; i <= input.length; i++) {
        if (i < input.length && input[i] === current && count < 255) {
            count++;
        } else {
            result += current + String.fromCharCode(count);
            if (i < input.length) {
                current = input[i];
                count = 1;
            }
        }
    }

    return result;
}

function decompress(input) {
    if (typeof input !== 'string') {
        throw new Error('Input must be a string');
    }

    if (input === '') return '';

    let result = '';
    
    for (let i = 0; i < input.length; i += 2) {
        const char = input[i];
        const count = input[i + 1] ? input[i + 1].charCodeAt(0) : 1;
        result += char.repeat(count);
    }

    return result;
}

module.exports = {
    compress,
    decompress
}; 