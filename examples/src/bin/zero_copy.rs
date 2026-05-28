use b64::{
    decode_to_vec_into, decode_to_vec_mode_into, encode_into, encode_url_safe_into, DecodeMode,
};

fn main() {
    // ---------------------------------------------------------
    // Zero-copy ENCODE
    // ---------------------------------------------------------
    let input = b"hello zero-copy world";

    // Caller-owned output buffer
    let mut encoded = Vec::with_capacity(64);

    // Encode into caller buffer (no String allocation)
    encode_into(input, &mut encoded);

    println!("encoded (standard): {}", String::from_utf8_lossy(&encoded));

    // ---------------------------------------------------------
    // Zero-copy URL-safe ENCODE
    // ---------------------------------------------------------
    let mut encoded_url = Vec::new();
    encode_url_safe_into(input, &mut encoded_url);

    println!(
        "encoded (url-safe): {}",
        String::from_utf8_lossy(&encoded_url)
    );

    // ---------------------------------------------------------
    // Zero-copy DECODE
    // ---------------------------------------------------------
    let mut decoded = Vec::new();
    decode_to_vec_into(std::str::from_utf8(&encoded).unwrap(), &mut decoded).unwrap();

    println!("decoded: {}", String::from_utf8_lossy(&decoded));

    // ---------------------------------------------------------
    // Zero-copy STRICT DECODE
    // ---------------------------------------------------------
    let mut strict_out = Vec::new();
    let strict_result = decode_to_vec_mode_into(
        "aGVsbG8gd29ybGQ=", // "hello world"
        DecodeMode::Strict,
        &mut strict_out,
    );

    println!("strict decode ok: {}", strict_result.is_ok());
}
