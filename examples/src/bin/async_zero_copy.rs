use b64::{decode_reader_to_writer_async, encode_reader_to_writer_async};
use std::io::Cursor;

#[tokio::main]
async fn main() {
    let input = b"async example";

    let mut reader = Cursor::new(input);
    let mut encoded = Vec::new();

    encode_reader_to_writer_async(&mut reader, &mut encoded, None)
        .await
        .unwrap();

    println!("encoded: {}", String::from_utf8_lossy(&encoded));

    let mut reader2 = Cursor::new(encoded);
    let mut decoded = Vec::new();

    decode_reader_to_writer_async(&mut reader2, &mut decoded)
        .await
        .unwrap();

    println!("decoded: {}", String::from_utf8_lossy(&decoded));
}
