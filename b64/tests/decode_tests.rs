use b64::{decode_parallel, decode_reader_to_writer, decode_to_vec, encode_to_string};
use std::io::Cursor;

#[test]
fn decode_rfc_vectors() {
    assert_eq!(decode_to_vec("").unwrap(), b"");
    assert_eq!(decode_to_vec("Zg==").unwrap(), b"f");
    assert_eq!(decode_to_vec("Zm8=").unwrap(), b"fo");
    assert_eq!(decode_to_vec("Zm9v").unwrap(), b"foo");
    assert_eq!(decode_to_vec("Zm9vYg==").unwrap(), b"foob");
    assert_eq!(decode_to_vec("Zm9vYmE=").unwrap(), b"fooba");
    assert_eq!(decode_to_vec("Zm9vYmFy").unwrap(), b"foobar");
}

#[test]
fn decode_with_whitespace() {
    let input = "Z m 9 v Ym Fy\n";
    assert_eq!(decode_to_vec(input).unwrap(), b"foobar");
}

#[test]
fn decode_invalid_char() {
    let err = decode_to_vec("Zm9v$").unwrap_err();
    match err {
        b64::DecodeError::InvalidByte(b, _) => assert_eq!(b, b'$'),
        _ => panic!("expected InvalidByte"),
    }
}

#[test]
fn decode_invalid_padding_middle() {
    let err = decode_to_vec("Z=m9v").unwrap_err();
    matches!(err, b64::DecodeError::UnexpectedPadding);
}

#[test]
fn decode_invalid_padding_extra() {
    let err = decode_to_vec("Zm9v===").unwrap_err();
    matches!(err, b64::DecodeError::UnexpectedPadding);
}

#[test]
fn decode_invalid_padding_after_data() {
    let err = decode_to_vec("Zm9v=Yg==").unwrap_err();
    matches!(err, b64::DecodeError::UnexpectedPadding);
}

#[test]
fn decode_streaming() {
    let encoded = "VGhpcyBpcyBhIHN0cmVhbSB0ZXN0Lg==";
    let mut reader = Cursor::new(encoded.as_bytes());
    let mut out = Vec::new();

    decode_reader_to_writer(&mut reader, &mut out).unwrap();

    assert_eq!(out, b"This is a stream test.");
}

#[test]
fn decode_parallel_matches() {
    let encoded = encode_to_string(b"The quick brown fox jumps over the lazy dog");
    let serial = decode_to_vec(&encoded).unwrap();
    let parallel = decode_parallel(&encoded).unwrap();

    assert_eq!(serial, parallel);
}

#[test]
fn decode_roundtrip_small() {
    for input in [
        &b""[..],
        &b"a"[..],
        &b"ab"[..],
        &b"abc"[..],
        &b"abcd"[..],
        &b"hello world"[..],
    ] {
        let enc = encode_to_string(input);
        let dec = decode_to_vec(&enc).unwrap();
        assert_eq!(dec, input);
    }
}

#[test]
fn decode_roundtrip_random() {
    use rand::RngCore;
    use rand::SeedableRng;

    let mut rng = rand::rngs::StdRng::seed_from_u64(12345);

    for _ in 0..1000 {
        let len = (rng.next_u32() % 200) as usize;
        let mut data = vec![0u8; len];
        rng.fill_bytes(&mut data);

        let enc = encode_to_string(&data);
        let dec = decode_to_vec(&enc).unwrap();

        assert_eq!(dec, data);
    }
}

#[test]
fn decode_large() {
    let data = vec![42u8; 1_000_000];
    let enc = encode_to_string(&data);
    let dec = decode_to_vec(&enc).unwrap();
    assert_eq!(dec, data);
}
