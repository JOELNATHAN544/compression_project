function compress(input) {
    if (!input) return '';
    
    const dictionary = new Map();
    let nextCode = 256;
    let result = [];
    let current = '';
    
    for (let i = 0; i < input.length; i++) {
        const char = input[i];
        const combined = current + char;
        
        if (dictionary.has(combined)) {
            current = combined;
        } else {
            result.push(dictionary.has(current) ? dictionary.get(current) : current.charCodeAt(0));
            dictionary.set(combined, nextCode++);
            current = char;
        }
    }
    
    if (current) {
        result.push(dictionary.has(current) ? dictionary.get(current) : current.charCodeAt(0));
    }
    
    return result.join(',');
}

function decompress(input) {
    if (!input) return '';
    
    const dictionary = new Map();
    let nextCode = 256;
    const codes = input.split(',').map(Number);
    let result = '';
    let previous = String.fromCharCode(codes[0]);
    result += previous;
    
    for (let i = 1; i < codes.length; i++) {
        const code = codes[i];
        let current;
        
        if (dictionary.has(code)) {
            current = dictionary.get(code);
        } else if (code === nextCode) {
            current = previous + previous[0];
        } else {
            current = String.fromCharCode(code);
        }
        
        result += current;
        dictionary.set(nextCode++, previous + current[0]);
        previous = current;
    }
    
    return result;
}

module.exports = {
    compress,
    decompress
}; 