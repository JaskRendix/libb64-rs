#![cfg(all(target_arch = "x86_64", feature = "simd-avx2"))]

use b64::parallel_avx2::encode_parallel_avx2;
use b64::parallel_scalar::encode_parallel_scalar;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::{Rng, SeedableRng};

//
// BASIC CORRECTNESS
//
#[test]
fn avx2_basic_cases() {
    let cases: &[&[u8]] = &[b"", b"a", b"ab", b"abc", b"hello", b"hello world"];

    for input in cases {
        let simd = encode_parallel_avx2(input);
        let scalar = STANDARD.encode(input);
        assert_eq!(simd, scalar);
    }
}

//
// CHUNK BOUNDARIES
//
#[test]
fn avx2_chunk_boundaries() {
    for len in 0..200 {
        let data: Vec<u8> = (0..len).map(|x| (x * 37 % 256) as u8).collect();
        let simd = encode_parallel_avx2(&data);
        let scalar = STANDARD.encode(&data);
        assert_eq!(simd, scalar);
    }
}

//
// PADDING CASES
//
#[test]
fn avx2_padding_cases() {
    let samples: &[&[u8]] = &[b"", b"f", b"fo", b"foo", b"foob", b"fooba", b"foobar"];

    for s in samples {
        let simd = encode_parallel_avx2(s);
        let scalar = STANDARD.encode(s);
        assert_eq!(simd, scalar);
    }
}

//
// TAIL HANDLING (0–2 BYTES)
//
#[test]
fn avx2_tail_cases() {
    for tail_len in 0..3 {
        let mut data = vec![42u8; 1000];
        data.truncate(1000 - tail_len);

        let simd = encode_parallel_avx2(&data);
        let scalar = STANDARD.encode(&data);
        assert_eq!(simd, scalar);
    }
}

//
// WHITESPACE ROBUSTNESS
//
#[test]
fn avx2_whitespace_robustness() {
    let data = b"hello world";
    let encoded = encode_parallel_avx2(data);

    let variants = [
        encoded.replace("", " "),
        encoded.replace("", "\n"),
        encoded.replace("", "\t"),
        format!("  {}\n\n", encoded),
        format!("{}\n\n{}", &encoded[..4], &encoded[4..]),
    ];

    for v in variants {
        let dec = STANDARD.decode(v).unwrap();
        assert_eq!(dec, data);
    }
}

//
// FUZZ TESTS
//
#[test]
fn avx2_fuzz_random_roundtrip() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(12345);

    for _ in 0..5000 {
        let len = rng.gen_range(0..5000);
        let data: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

        let simd = encode_parallel_avx2(&data);
        let dec = STANDARD.decode(&simd).unwrap();

        assert_eq!(dec, data);
    }
}

//
// LARGE INPUTS
//
#[test]
fn avx2_large_roundtrip() {
    let data = vec![7u8; 10_000_000]; // 10 MB
    let simd = encode_parallel_avx2(&data);
    let dec = STANDARD.decode(&simd).unwrap();
    assert_eq!(dec, data);
}

//
// SCALAR EQUIVALENCE
//
#[test]
fn avx2_matches_scalar_backend() {
    let data = (0..10_000).map(|x| (x % 256) as u8).collect::<Vec<_>>();

    let simd = encode_parallel_avx2(&data);
    let scalar = encode_parallel_scalar(&data);

    assert_eq!(simd, scalar);
}

//
// AVX2 DISPATCH CHECK
//
#[test]
fn avx2_dispatch_detects_avx2() {
    assert!(std::arch::is_x86_feature_detected!("avx2"));
}
