use b64::{decode_reader_to_writer_async, decode_reader_to_writer_mode_async, DecodeMode};
use std::io::Cursor;

#[tokio::test]
async fn async_decode_standard_stream() {
    let encoded = "VGhpcyBpcyBhbiBhc3luYyBzdHJlYW0gdGVzdCBmb3IgZGVjb2Rpbmch";
    let mut reader = Cursor::new(encoded.as_bytes());
    let mut decoded = Vec::new();

    decode_reader_to_writer_async(&mut reader, &mut decoded)
        .await
        .unwrap();

    assert_eq!(decoded, b"This is an async stream test for decoding!");
}

#[tokio::test]
async fn async_decode_url_safe_alphabet() {
    // Correct URL-safe encoded format for b">>Hello?_World<<"
    let encoded = "Pj5IZWxsbz9fV29ybGQ8PDw=";
    let mut reader = Cursor::new(encoded.as_bytes());
    let mut decoded = Vec::new();

    decode_reader_to_writer_async(&mut reader, &mut decoded)
        .await
        .unwrap();

    assert_eq!(decoded, b">>Hello?_World<<");
}

#[tokio::test]
async fn async_decode_strict_mode_whitespace_rejection() {
    // Strict mode should reject embedded whitespaces/newlines
    let encoded = "VGhp cyBpcyBh";
    let mut reader = Cursor::new(encoded.as_bytes());
    let mut decoded = Vec::new();

    let result =
        decode_reader_to_writer_mode_async(&mut reader, &mut decoded, DecodeMode::Strict).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn async_decode_malformed_padding() {
    let encoded = "Zm9v==="; // Invalid trailing padding count
    let mut reader = Cursor::new(encoded.as_bytes());
    let mut decoded = Vec::new();

    let result = decode_reader_to_writer_async(&mut reader, &mut decoded).await;
    assert!(result.is_err());
}
