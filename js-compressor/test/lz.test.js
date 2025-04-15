const lz = require('../lz');

describe('LZ Compression', () => {
    test('compresses simple string', () => {
        const compressed = lz.compress('ABABABAB');
        expect(lz.decompress(compressed)).toBe('ABABABAB');
    });

    test('compresses repeated pattern', () => {
        const compressed = lz.compress('ABCABCABC');
        expect(lz.decompress(compressed)).toBe('ABCABCABC');
    });

    test('handles empty string', () => {
        expect(lz.compress('')).toBe('');
        expect(lz.decompress('')).toBe('');
    });

    test('handles single character', () => {
        const compressed = lz.compress('A');
        expect(lz.decompress(compressed)).toBe('A');
    });
}); 