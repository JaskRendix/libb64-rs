use crate::decode::{DecodeError, Decoder};
use crate::encode::Encoder;
use rayon::prelude::*;

pub fn encode_parallel(input: &[u8]) -> String {
    let (main, tail) = input.split_at(input.len() - input.len() % 3);

    let mut out: String = main
        .par_chunks(3)
        .map(|chunk| {
            let mut enc = Encoder::new(None);
            let mut s = String::new();
            enc.encode_block(chunk, &mut s);
            s
        })
        .collect();

    if !tail.is_empty() {
        let mut enc = Encoder::new(None);
        enc.encode_block(tail, &mut out);
        enc.encode_end(&mut out);
    }

    out
}

pub fn decode_parallel(input: &str) -> Result<Vec<u8>, DecodeError> {
    let cleaned: Vec<u8> = input.bytes().filter(|b| !b.is_ascii_whitespace()).collect();

    if cleaned.len() < 4 {
        return crate::decode::decode_to_vec(input);
    }

    let (main, tail) = cleaned.split_at(cleaned.len() - 4);

    let mut decoded: Vec<u8> = main
        .par_chunks(4)
        .map(|chunk| {
            let mut dec = Decoder::new();
            let mut out = Vec::new();
            dec.decode_block(chunk, &mut out)?;
            Ok(out)
        })
        .collect::<Result<Vec<_>, DecodeError>>()?
        .into_iter()
        .flatten()
        .collect();

    let mut dec = Decoder::new();
    dec.decode_block(tail, &mut decoded)?;

    Ok(decoded)
}
