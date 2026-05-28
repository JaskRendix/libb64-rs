use b64::{decode_reader_to_writer_async, encode_reader_to_writer_async};
use std::io::Cursor;

#[tokio::test]
async fn async_roundtrip() {
    let input = b"hello async world";

    let mut reader = Cursor::new(input.to_vec());
    let mut encoded = Vec::new();

    encode_reader_to_writer_async(&mut reader, &mut encoded, None)
        .await
        .unwrap();

    let mut reader2 = Cursor::new(encoded);
    let mut decoded = Vec::new();

    decode_reader_to_writer_async(&mut reader2, &mut decoded)
        .await
        .unwrap();

    assert_eq!(decoded, input);
}
