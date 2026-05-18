use base64_simd::STANDARD;
use rayon::prelude::*;

use crate::decode::DecodeError;

/// SIMD + parallel Base64 encoding.
/// Safe because Base64 encoding is block-aligned (3 → 4 bytes).
pub fn encode_parallel(input: &[u8]) -> String {
    // Use a large chunk size that is a multiple of 3.
    // 192 KiB (3 * 64 KiB) is a good balance for SIMD + Rayon.
    const CHUNK: usize = 3 * 64 * 1024;

    // Split into 3-byte aligned region + tail
    let aligned_len = input.len() - (input.len() % 3);
    let (main, tail) = input.split_at(aligned_len);

    // Encode large aligned chunks in parallel
    let parts: Vec<String> = main
        .par_chunks(CHUNK)
        .map(|chunk| STANDARD.encode_to_string(chunk))
        .collect();

    // Join parallel output
    let mut out = parts.concat();

    // Encode tail sequentially (0–2 bytes)
    if !tail.is_empty() {
        out.push_str(&STANDARD.encode_to_string(tail));
    }

    out
}

/// SIMD-accelerated Base64 decoding (single-threaded).
/// This is the *only* fully correct SIMD decode path.
/// Parallel SIMD decode is not safe with base64-simd.
pub fn decode_parallel(input: &str) -> Result<Vec<u8>, DecodeError> {
    // Remove whitespace
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    STANDARD
        .decode_to_vec(cleaned.as_bytes())
        .map_err(|_| DecodeError::InvalidLength)
}
