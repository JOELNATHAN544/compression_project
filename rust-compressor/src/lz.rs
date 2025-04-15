const LZ_WINDOW_SIZE: usize = 20;

pub fn compress_lz(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let start = if i > LZ_WINDOW_SIZE { i - LZ_WINDOW_SIZE } else { 0 };
        let mut offset = 0;
        let mut length = 0;

        for j in start..i {
            let mut match_len = 0;
            while i + match_len < data.len()
                && data[j + match_len] == data[i + match_len]
                && j + match_len < i
                && match_len < 255
            {
                match_len += 1;
            }

            if match_len > length {
                offset = i - j;
                length = match_len;
            }
        }

        if length >= 3 {
            result.push(0x01); // match
            result.push(offset as u8);
            result.push(length as u8);
            i += length;
        } else {
            result.push(0x00); // literal
            result.push(data[i]);
            i += 1;
        }
    }

    result
}

pub fn decompress_lz(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < data.len() {
        if data[i] == 0x00 {
            // literal
            if i + 1 < data.len() {
                result.push(data[i + 1]);
            }
            i += 2;
        } else if data[i] == 0x01 {
            // match
            if i + 2 < data.len() {
                let offset = data[i + 1] as usize;
                let length = data[i + 2] as usize;
                let start = result.len().saturating_sub(offset);
                for j in 0..length {
                    if start + j < result.len() {
                        result.push(result[start + j]);
                    }
                }
            }
            i += 3;
        } else {
            // unknown flag
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_simple() {
        let input = b"ABABABAB";
        let compressed = compress_lz(input);
        let decompressed = decompress_lz(&compressed);
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_compress_with_repeats() {
        let input = b"ABCABCABC";
        let compressed = compress_lz(input);
        let decompressed = decompress_lz(&compressed);
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_empty_input() {
        let input = b"";
        let compressed = compress_lz(input);
        assert_eq!(compressed, Vec::<u8>::new());
        let decompressed = decompress_lz(&compressed);
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_single_character() {
        let input = b"A";
        let compressed = compress_lz(input);
        let decompressed = decompress_lz(&compressed);
        assert_eq!(decompressed, input);
    }

    #[test]
    fn test_no_repetition() {
        let input = b"ABCDEF";
        let compressed = compress_lz(input);
        let decompressed = decompress_lz(&compressed);
        assert_eq!(decompressed, input);
    }
}