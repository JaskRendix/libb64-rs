#[cfg(all(target_arch = "x86_64", feature = "simd-avx2"))]
use base64_simd::STANDARD;
use rayon::prelude::*;

pub fn encode_parallel_avx2(input: &[u8]) -> String {
    const CHUNK: usize = 3 * 64 * 1024;

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
