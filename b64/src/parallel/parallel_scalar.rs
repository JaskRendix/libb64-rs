use crate::encode::encode_to_string;
use crate::DecodeError;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rayon::prelude::*;

pub fn encode_parallel_scalar(input: &[u8]) -> String {
    const CHUNK: usize = 3 * 64 * 1024;

    let aligned_len = input.len() - (input.len() % 3);
    let (main, tail) = input.split_at(aligned_len);

    let parts: Vec<String> = main.par_chunks(CHUNK).map(encode_to_string).collect();

    let mut out = parts.concat();

    if !tail.is_empty() {
        out.push_str(&encode_to_string(tail));
    }

    out
}

pub fn decode_parallel_scalar(input: &str) -> Result<Vec<u8>, DecodeError> {
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    STANDARD
        .decode(cleaned.as_bytes())
        .map_err(|_| DecodeError::InvalidLength)
}
