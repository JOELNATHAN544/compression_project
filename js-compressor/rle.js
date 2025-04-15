function compress(input) {
    if (!input) return '';
    
    let result = '';
    let count = 1;
    
    for (let i = 0; i < input.length; i++) {
        if (input[i] === input[i + 1]) {
            count++;
        } else {
            result += count + input[i];
            count = 1;
        }
    }
    
    return result;
}

function decompress(input) {
    if (!input) return '';
    
    let result = '';
    let count = '';
    
    for (let i = 0; i < input.length; i++) {
        if (!isNaN(input[i])) {
            count += input[i];
        } else {
            result += input[i].repeat(parseInt(count));
            count = '';
        }
    }
    
    return result;
}

module.exports = {
    compress,
    decompress
}; 