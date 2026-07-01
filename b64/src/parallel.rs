#[cfg(all(target_arch = "x86_64", feature = "simd-avx2"))]
pub mod parallel_avx2;
pub mod parallel_scalar;

use base64_simd::{STANDARD, URL_SAFE};
use rayon::prelude::*;

use crate::decode::DecodeError;

/// Public API: SIMD + parallel Base64 encoding with runtime autodetection.
pub fn encode_parallel(input: &[u8]) -> String {
    encode_parallel_autodetect(input)
}

fn encode_parallel_autodetect(input: &[u8]) -> String {
    // x86_64: try AVX2, then SSE2, else scalar
    #[cfg(target_arch = "x86_64")]
    {
        // AVX2
        #[cfg(feature = "simd-avx2")]
        {
            if std::arch::is_x86_feature_detected!("avx2") {
                return parallel_avx2::encode_parallel_avx2(input);
            }
        }

        // If you add a separate SSE2 backend:
        // #[cfg(feature = "simd-sse2")]
        // if std::arch::is_x86_feature_detected!("sse2") {
        //     return parallel_sse2::encode_parallel_sse2(input);
        // }

        // Fallback: scalar
        parallel_scalar::encode_parallel_scalar(input)
    }

    // aarch64: NEON or scalar
    #[cfg(target_arch = "aarch64")]
    {
        // If you add a NEON backend:
        // #[cfg(feature = "simd-neon")]
        // {
        //     // NEON is guaranteed on aarch64, but you can still gate it by feature.
        //     return parallel_neon::encode_parallel_neon(input);
        // }

        return parallel_scalar::encode_parallel_scalar(input);
    }

    // Other architectures: scalar only
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        parallel_scalar::encode_parallel_scalar(input)
    }
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

pub fn encode_parallel_url_safe(input: &[u8]) -> String {
    const CHUNK: usize = 3 * 64 * 1024;

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

pub fn decode_parallel_url_safe(input: &str) -> Result<Vec<u8>, DecodeError> {
    decode_parallel(input)
}
