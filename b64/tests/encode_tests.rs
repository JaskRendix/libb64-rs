use b64::{decode_to_vec, encode_reader_to_writer, encode_to_string};
use std::io::Cursor;

#[test]
fn encode_basic() {
    assert_eq!(encode_to_string(b"hello"), "aGVsbG8=");
    assert_eq!(encode_to_string(b"foo"), "Zm9v");
    assert_eq!(encode_to_string(b"f"), "Zg==");
    assert_eq!(encode_to_string(b"fo"), "Zm8=");
}

#[test]
fn encode_empty() {
    assert_eq!(encode_to_string(b""), "");
}

#[test]
fn encode_multiblock() {
    let input = b"The quick brown fox jumps over the lazy dog";
    let encoded = encode_to_string(input);
    let decoded = decode_to_vec(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn encode_wrapped_8() {
    let input = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut reader = Cursor::new(input);
    let mut out = Vec::new();

    encode_reader_to_writer(&mut reader, &mut out, Some(8)).unwrap();

    let output = String::from_utf8(out).unwrap();

    // Every line must be <= 8 chars
    for line in output.trim_end().split('\n') {
        assert!(line.len() <= 8);
    }

    // And decoding must still work
    let decoded = decode_to_vec(&output.replace('\n', "")).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn encode_streaming() {
    let input = b"streaming test data";
    let mut reader = Cursor::new(input);
    let mut out = Vec::new();

    encode_reader_to_writer(&mut reader, &mut out, None).unwrap();

    let encoded = String::from_utf8(out).unwrap();
    let decoded = decode_to_vec(&encoded).unwrap();

    assert_eq!(decoded, input);
}

#[test]
fn encode_roundtrip_small() {
    let samples: &[&[u8]] = &[
        &b""[..],
        &b"a"[..],
        &b"ab"[..],
        &b"abc"[..],
        &b"hello"[..],
        &b"hello world"[..],
    ];

    for s in samples {
        let enc = encode_to_string(s);
        let dec = decode_to_vec(&enc).unwrap();
        assert_eq!(dec, *s);
    }
}

#[test]
fn encode_roundtrip_large() {
    let data = vec![42u8; 100_000];
    let enc = encode_to_string(&data);
    let dec = decode_to_vec(&enc).unwrap();
    assert_eq!(dec, data);
}

#[test]
fn encode_padding_cases() {
    assert_eq!(encode_to_string(b"f"), "Zg==");
    assert_eq!(encode_to_string(b"fo"), "Zm8=");
    assert_eq!(encode_to_string(b"foo"), "Zm9v");
    assert_eq!(encode_to_string(b"foob"), "Zm9vYg==");
    assert_eq!(encode_to_string(b"fooba"), "Zm9vYmE=");
    assert_eq!(encode_to_string(b"foobar"), "Zm9vYmFy");
}
