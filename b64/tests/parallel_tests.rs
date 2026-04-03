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
