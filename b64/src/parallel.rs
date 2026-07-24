use base64_simd::{STANDARD, URL_SAFE};
use rayon::prelude::*;

use crate::decode::DecodeError;

const CHUNK: usize = 3 * 64 * 1024; // 192 KB (must be a multiple of 3)

pub fn encode_parallel(input: &[u8]) -> String {
    let aligned_len = input.len() - (input.len() % 3);
    let (main, tail) = input.split_at(aligned_len);

    let parts: Vec<String> = main
        .par_chunks(CHUNK)
        .map(|chunk| STANDARD.encode_to_string(chunk))
        .collect();

    let mut out = parts.concat();
    if !tail.is_empty() {
        out.push_str(&STANDARD.encode_to_string(tail));
    }
    out
}

pub fn encode_parallel_url_safe(input: &[u8]) -> String {
    let aligned_len = input.len() - (input.len() % 3);
    let (main, tail) = input.split_at(aligned_len);

    let parts: Vec<String> = main
        .par_chunks(CHUNK)
        .map(|chunk| URL_SAFE.encode_to_string(chunk))
        .collect();

    let mut out = parts.concat();
    if !tail.is_empty() {
        out.push_str(&URL_SAFE.encode_to_string(tail));
    }
    out
}

pub fn decode_parallel(input: &str) -> Result<Vec<u8>, DecodeError> {
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    STANDARD
        .decode_to_vec(cleaned.as_bytes())
        .map_err(|_| DecodeError::InvalidLength)
}

pub fn decode_parallel_url_safe(input: &str) -> Result<Vec<u8>, DecodeError> {
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    URL_SAFE
        .decode_to_vec(cleaned.as_bytes())
        .map_err(|_| DecodeError::InvalidLength)
}
