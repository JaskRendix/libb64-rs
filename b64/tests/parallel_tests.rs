use b64::decode::Decoder;
use b64::encode::Encoder;
use b64::{decode_parallel, encode_parallel};
use b64::{decode_to_vec, encode_to_string};

#[test]
fn parallel_encode_matches_serial() {
    let inputs: &[&[u8]] = &[
        &b""[..],
        &b"a"[..],
        &b"ab"[..],
        &b"abc"[..],
        &b"hello"[..],
        &b"hello world"[..],
        &b"The quick brown fox jumps over the lazy dog"[..],
    ];

    for input in inputs {
        let serial = encode_to_string(input);
        let parallel = encode_parallel(input);
        assert_eq!(serial, parallel);
    }
}

#[test]
fn parallel_decode_matches_serial() {
    let inputs = [
        "aGVsbG8=",
        "Zm9v",
        "Zg==",
        "Zm8=",
        "VGhpcyBpcyBhIHRlc3Q=",
        "VGhlIHF1aWNrIGJyb3duIGZveCBqdW1wcyBvdmVyIHRoZSBsYXp5IGRvZw==",
    ];

    for input in inputs {
        let serial = decode_to_vec(input).unwrap();
        let parallel = decode_parallel(input).unwrap();
        assert_eq!(serial, parallel);
    }
}

#[test]
fn parallel_large_roundtrip() {
    let data = vec![42u8; 1_000_000];

    let enc_p = encode_parallel(&data);
    let dec_p = decode_parallel(&enc_p).unwrap();

    assert_eq!(dec_p, data);
}

#[test]
fn parallel_padding_cases() {
    let samples: &[&[u8]] = &[
        &b""[..],
        &b"f"[..],
        &b"fo"[..],
        &b"foo"[..],
        &b"foob"[..],
        &b"fooba"[..],
        &b"foobar"[..],
    ];

    for s in samples {
        let enc_s = encode_to_string(s);
        let enc_p = encode_parallel(s);
        assert_eq!(enc_s, enc_p);

        let dec_s = decode_to_vec(&enc_s).unwrap();
        let dec_p = decode_parallel(&enc_s).unwrap();
        assert_eq!(dec_s, dec_p);
    }
}

#[test]
fn parallel_decode_with_whitespace() {
    let encoded = "Z m 9 v Ym Fy\n";
    let serial = decode_to_vec(encoded).unwrap();
    let parallel = decode_parallel(encoded).unwrap();
    assert_eq!(serial, parallel);
}

#[test]
fn parallel_decode_invalid() {
    let invalid_inputs = ["###", "Zm9v$", "Zm9v===", "Z=m9v", "=m9v", "Zm9v=Yg=="];

    for input in invalid_inputs {
        assert!(decode_parallel(input).is_err());
    }
}

#[test]
fn parallel_chunk_boundaries() {
    for len in 0..100 {
        let data: Vec<u8> = (0..len).map(|x| (x * 37 % 256) as u8).collect();

        let enc_s = encode_to_string(&data);
        let enc_p = encode_parallel(&data);
        assert_eq!(enc_s, enc_p);

        let dec_s = decode_to_vec(&enc_s).unwrap();
        let dec_p = decode_parallel(&enc_s).unwrap();
        assert_eq!(dec_s, dec_p);
    }
}

#[test]
fn fuzz_random_roundtrip() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(12345);

    for _ in 0..10_000 {
        let len = rng.gen_range(0..10_000);
        let data: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

        let enc = encode_parallel(&data);
        let dec = decode_parallel(&enc).unwrap();

        assert_eq!(dec, data);
    }
}

#[test]
fn streaming_decode_chunks() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let encoded = encode_to_string(data);

    for chunk_size in 1..20 {
        let mut dec = Decoder::new();
        let mut out = Vec::new();

        for chunk in encoded.as_bytes().chunks(chunk_size) {
            dec.decode_block(chunk, &mut out).unwrap();
        }

        dec.finalize(&mut out).unwrap();
        assert_eq!(out, data);
    }
}

#[test]
fn streaming_encode_chunks() {
    let data = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let expected = encode_to_string(data);

    for chunk_size in 1..20 {
        let mut enc = Encoder::new(None);
        let mut out = String::new();

        for chunk in data.chunks(chunk_size) {
            enc.encode_block(chunk, &mut out);
        }

        enc.encode_end(&mut out);
        assert_eq!(out, expected);
    }
}

#[test]
fn all_padding_patterns() {
    for len in 0..10 {
        let data: Vec<u8> = (0..len).map(|x| x as u8).collect();
        let enc = encode_parallel(&data);
        let dec = decode_parallel(&enc).unwrap();
        assert_eq!(dec, data);
    }
}

#[test]
fn whitespace_torture() {
    let data = b"hello world";
    let enc = encode_to_string(data);

    let variants = [
        enc.replace("", " "),
        enc.replace("", "\n"),
        enc.replace("", "\t"),
        format!("  {}\n\n", enc),
        format!("{}\n\n{}", &enc[..4], &enc[4..]),
    ];

    for v in variants {
        let dec = decode_parallel(&v).unwrap();
        assert_eq!(dec, data);
    }
}

#[test]
fn fuzz_invalid_inputs() {
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(999);

    for _ in 0..5000 {
        let len = rng.gen_range(1..200);

        // Generate raw random bytes
        let bytes: Vec<u8> = (0..len).map(|_| rng.gen()).collect();

        // Convert to lossy UTF‑8 so the decoder always receives valid &str
        let s = String::from_utf8_lossy(&bytes).to_string();

        // Decoder must never panic
        let _ = decode_parallel(&s);
    }
}

#[test]
fn huge_roundtrip() {
    let data = vec![7u8; 100_000_000];
    let enc = encode_parallel(&data);
    let dec = decode_parallel(&enc).unwrap();
    assert_eq!(dec, data);
}

#[test]
fn wrap_exact_boundary() {
    let data = vec![0u8; 57]; // 57 bytes → 76 chars
    let mut enc = Encoder::new(Some(76));
    let mut out = String::new();
    enc.encode_block(&data, &mut out);
    enc.encode_end(&mut out);

    assert!(!out.ends_with('\n'));
}
