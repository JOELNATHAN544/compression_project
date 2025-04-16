const assert = require('assert');
const { compress, decompress } = require('../lz');

describe('LZ Compression', () => {
    it('should compress and decompress correctly', () => {
        const input = Buffer.from('AAABBBCCCCCDDDDE');
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed.toString(), input.toString());
    });

    it('compresses simple string', () => {
        const input = 'ABABABAB';
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed, input);
    });

    it('compresses repeated pattern', () => {
        const input = 'ABCABCABC';
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed, input);
    });

    it('handles empty string', () => {
        assert.strictEqual(compress(''), '');
        assert.strictEqual(decompress(''), '');
    });

    it('handles single character', () => {
        const input = 'A';
        const compressed = compress(input);
        const decompressed = decompress(compressed);
        assert.strictEqual(decompressed, input);
    });
}); 