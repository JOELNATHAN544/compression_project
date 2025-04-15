const rle = require('../rle');

describe('RLE Compression', () => {
    test('compresses simple string', () => {
        expect(rle.compress('AAAABBBCCDAA')).toBe('4A3B2C1D2A');
    });

    test('decompresses simple string', () => {
        expect(rle.decompress('4A3B2C1D2A')).toBe('AAAABBBCCDAA');
    });

    test('handles empty string', () => {
        expect(rle.compress('')).toBe('');
        expect(rle.decompress('')).toBe('');
    });

    test('handles single character', () => {
        expect(rle.compress('A')).toBe('1A');
        expect(rle.decompress('1A')).toBe('A');
    });
}); 