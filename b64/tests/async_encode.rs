use b64::{
    encode_reader_to_writer_async, encode_url_safe_reader_to_writer_async,
};
use std::io::Cursor;

#[tokio::test]
async fn async_encode_roundtrip() {
    let input = b"hello async world with custom stream buffers!";
    let mut reader = Cursor::new(input.to_vec());
    let mut encoded = Vec::new();

    encode_reader_to_writer_async(&mut reader, &mut encoded, None)
        .await
        .unwrap();

    let decoded = b64::decode_to_vec(&String::from_utf8(encoded).unwrap()).unwrap();
    assert_eq!(decoded, input);
}

#[tokio::test]
async fn async_encode_url_safe_variant() {
    // Bytes that produce '+' and '/' in standard base64
    let input = &[0xFB, 0xEF, 0xBE, 0xFF, 0xAA];
    
    let mut reader_std = Cursor::new(input.to_vec());
    let mut encoded_std = Vec::new();
    encode_reader_to_writer_async(&mut reader_std, &mut encoded_std, None).await.unwrap();
    let std_str = String::from_utf8(encoded_std).unwrap();

    let mut reader_url = Cursor::new(input.to_vec());
    let mut encoded_url = Vec::new();
    encode_url_safe_reader_to_writer_async(&mut reader_url, &mut encoded_url, None).await.unwrap();
    let url_str = String::from_utf8(encoded_url).unwrap();

    // Verify URL-safe substitution rules (- and _)
    assert!(!url_str.contains('+') && !url_str.contains('/'));
    assert!(url_str.contains('-') || url_str.contains('_') || std_str != url_str);
}

#[tokio::test]
async fn async_encode_with_line_wrapping() {
    let input = b"The quick brown fox jumps over the lazy dog repeatedly to test line wrapping constraints.";
    let mut reader = Cursor::new(input.to_vec());
    let mut encoded = Vec::new();

    // Wrap every 16 characters
    encode_reader_to_writer_async(&mut reader, &mut encoded, Some(16))
        .await
        .unwrap();

    let encoded_str = String::from_utf8(encoded).unwrap();
    
    // Ensure newlines are injected
    assert!(encoded_str.contains('\n'));
    
    // Roundtrip verification to guarantee wrapper compliance doesn't break decoders
    let decoded = b64::decode_to_vec(&encoded_str).unwrap();
    assert_eq!(decoded, input);
}

#[tokio::test]
async fn async_encode_empty_stream() {
    let mut reader = Cursor::new(Vec::new());
    let mut encoded = Vec::new();

    encode_reader_to_writer_async(&mut reader, &mut encoded, None)
        .await
        .unwrap();

    assert!(encoded.is_empty());
}
