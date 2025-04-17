use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub struct Compressor {
    algorithm: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompressionResult {
    original_size: usize,
    compressed_size: usize,
    data: String,
}

#[wasm_bindgen]
impl Compressor {
    #[wasm_bindgen(constructor)]
    pub fn new(algorithm: &str) -> Self {
        Compressor {
            algorithm: algorithm.to_string(),
        }
    }

    pub fn compress(&self, input: &str) -> Result<String, JsValue> {
        let result = match self.algorithm.as_str() {
            "rle" => self.rle_compress(input),
            "lz" => self.lz_compress(input),
            _ => return Err(JsValue::from_str("Invalid algorithm")),
        };

        let compression_result = CompressionResult {
            original_size: input.len(),
            compressed_size: result.len(),
            data: result,
        };

        Ok(serde_json::to_string(&compression_result).unwrap())
    }

    pub fn decompress(&self, input: &str) -> Result<String, JsValue> {
        let result = match self.algorithm.as_str() {
            "rle" => self.rle_decompress(input),
            "lz" => self.lz_decompress(input),
            _ => return Err(JsValue::from_str("Invalid algorithm")),
        };
        Ok(result)
    }

    fn rle_compress(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(current) = chars.next() {
            let mut count = 1;
            while let Some(&next) = chars.peek() {
                if next == current {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            result.push_str(&format!("{}{}", count, current));
        }
        result
    }

    fn rle_decompress(&self, input: &str) -> String {
        let mut result = String::new();
        let mut count_str = String::new();
        let mut chars = input.chars();
        
        while let Some(c) = chars.next() {
            if c.is_digit(10) {
                count_str.push(c);
            } else {
                let count = count_str.parse::<usize>().unwrap_or(1);
                result.push_str(&c.to_string().repeat(count));
                count_str.clear();
            }
        }
        result
    }

    fn lz_compress(&self, input: &str) -> String {
        // Simple LZ77-like compression
        let mut result = String::new();
        let mut pos = 0;
        let bytes = input.as_bytes();

        while pos < bytes.len() {
            let mut best_len = 0;
            let mut best_dist = 0;
            
            // Look for matches in the window
            for dist in 1..=std::cmp::min(pos, 255) {
                let start = pos - dist;
                let mut len = 0;
                while pos + len < bytes.len() && 
                      len < 255 && 
                      bytes[start + (len % dist)] == bytes[pos + len] {
                    len += 1;
                }
                if len > best_len {
                    best_len = len;
                    best_dist = dist;
                }
            }

            if best_len > 3 {
                result.push_str(&format!("<{},{}>", best_dist, best_len));
                pos += best_len;
            } else {
                result.push(bytes[pos] as char);
                pos += 1;
            }
        }
        result
    }

    fn lz_decompress(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '<' {
                let mut dist_str = String::new();
                let mut len_str = String::new();
                let mut reading_len = false;

                while let Some(c) = chars.next() {
                    if c == ',' {
                        reading_len = true;
                    } else if c == '>' {
                        break;
                    } else if reading_len {
                        len_str.push(c);
                    } else {
                        dist_str.push(c);
                    }
                }

                let dist = dist_str.parse::<usize>().unwrap();
                let len = len_str.parse::<usize>().unwrap();
                let start = result.len() - dist;
                
                for i in 0..len {
                    let c = result.chars().nth(start + (i % dist)).unwrap();
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }
        result
    }
}

// Helper function to log to console from Rust
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
} 