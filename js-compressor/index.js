const rle = require('./rle');
const lz = require('./lz');

module.exports = {
    compress: (input, algorithm = 'rle') => {
        switch (algorithm.toLowerCase()) {
            case 'rle':
                return rle.compress(input);
            case 'lz':
                return lz.compress(input);
            default:
                throw new Error('Unsupported compression algorithm');
        }
    },
    decompress: (input, algorithm = 'rle') => {
        switch (algorithm.toLowerCase()) {
            case 'rle':
                return rle.decompress(input);
            case 'lz':
                return lz.decompress(input);
            default:
                throw new Error('Unsupported compression algorithm');
        }
    }
}; 