#![cfg(target_arch = "x86_64")]

use b64::parallel_scalar::encode_parallel_scalar;

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::{Rng, SeedableRng};

//
// BASIC CORRECTNESS
//
#[test]
fn scalar_basic_cases() {
    let cases: &[&[u8]] = &[b"", b"a", b"ab", b"abc", b"hello", b"hello world"];

    for input in cases {
        let scalar = encode_parallel_scalar(input);
        let reference = STANDARD.encode(input);
        assert_eq!(scalar, reference);
    }
}

//
// CHUNK BOUNDARIES
//
#[test]
fn scalar_chunk_boundaries() {
    for len in 0..200 {
        let data: Vec<u8> = (0..len).map(|x| (x * 37 % 256) as u8).collect();
        let scalar = encode_parallel_scalar(&data);
        let reference = STANDARD.encode(&data);
        assert_eq!(scalar, reference);
    }
}

//
// PADDING CASES
//
#[test]
fn scalar_padding_cases() {
    let samples: &[&[u8]] = &[b"", b"f", b"fo", b"foo", b"foob", b"fooba", b"foobar"];

    for s in samples {
        let scalar = encode_parallel_scalar(s);
        let reference = STANDARD.encode(s);
        assert_eq!(scalar, reference);
    }
}

//
// TAIL HANDLING (0–2 BYTES)
//
#[test]
fn scalar_tail_cases() {
    for tail_len in 0..3 {
        let mut data = vec![42u8; 1000];
        data.truncate(1000 - tail_len);

        let scalar = encode_parallel_scalar(&data);
        let reference = STANDARD.encode(&data);
        assert_eq!(scalar, reference);
    }
}

//
// WHITESPACE ROBUSTNESS
//
#[test]
fn scalar_whitespace_robustness() {
    let data = b"hello world";
    let encoded = encode_parallel_scalar(data);

    let variants = [
        encoded.replace("", " "),
        encoded.replace("", "\n"),
        encoded.replace("", "\t"),
        format!("  {}\n\n", encoded),
        format!("{}\n\n{}", &encoded[..4], &encoded[4..]),
    ];

    for v in variants {
        let cleaned: String = v.chars().filter(|c| !c.is_whitespace()).collect();
        let dec = STANDARD.decode(cleaned.as_bytes()).unwrap();
        assert_eq!(dec, data);
    }
}

//
// FUZZ TESTS
//
#[test]
fn scalar_fuzz_random_roundtrip() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(12345);

    for _ in 0..5000 {
        let len = rng.gen_range(0..5000);
        let data: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

        let scalar = encode_parallel_scalar(&data);
        let dec = STANDARD.decode(&scalar).unwrap();

        assert_eq!(dec, data);
    }
}

//
// LARGE INPUTS
//
#[test]
fn scalar_large_roundtrip() {
    let data = vec![7u8; 10_000_000]; // 10 MB
    let scalar = encode_parallel_scalar(&data);
    let dec = STANDARD.decode(&scalar).unwrap();
    assert_eq!(dec, data);
}

//
// EQUIVALENCE WITH base64::encode
//
#[test]
fn scalar_matches_reference() {
    let data = (0..10_000).map(|x| (x % 256) as u8).collect::<Vec<_>>();

    let scalar = encode_parallel_scalar(&data);
    let reference = STANDARD.encode(&data);

    assert_eq!(scalar, reference);
}
