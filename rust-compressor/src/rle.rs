pub fn compress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];
        let mut count = 1;

        while i + count < data.len() && data[i + count] == byte && count < 255 {
            count += 1;
        }

        result.push(byte);
        result.push(count as u8);

        i += count;
    }

    result
}

pub fn decompress_rle(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i + 1 < data.len() {
        let byte = data[i];
        let count = data[i + 1];
        result.extend(std::iter::repeat(byte).take(count as usize));
        i += 2;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_simple() {
        let input = b"AABBBCCCC";
        let compressed = compress_rle(input);
        assert_eq!(compressed, vec![b'A', 2, b'B', 3, b'C', 4]);
    }

    #[test]
    fn test_decompress_simple() {
        let compressed = vec![b'A', 2, b'B', 3, b'C', 4];
        let decompressed = decompress_rle(&compressed);
        assert_eq!(decompressed, b"AABBBCCCC");
    }

    #[test]
    fn test_roundtrip() {
        let original = b"AABBBCCCCDDDDEEEE";
        let compressed = compress_rle(original);
        let decompressed = decompress_rle(&compressed);
        assert_eq!(decompressed, original);
    }

    #[test]
    fn test_empty_input() {
        let input = b"";
        let compressed = compress_rle(input);
        assert_eq!(compressed, Vec::<u8>::new());
        let decompressed = decompress_rle(&compressed);
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_single_character() {
        let input = b"A";
        let compressed = compress_rle(input);
        assert_eq!(compressed, vec![b'A', 1]);
        let decompressed = decompress_rle(&compressed);
        assert_eq!(decompressed, input);
    }
}