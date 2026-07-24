pub mod async_decode;
pub mod async_encode;
pub mod decode;
pub mod encode;
pub mod parallel;

pub use crate::decode::{
    decode_reader_to_writer, decode_reader_to_writer_mode, decode_to_vec, decode_to_vec_into,
    decode_to_vec_mode, decode_to_vec_mode_into, DecodeError, DecodeIoError, DecodeMode,
};

pub use encode::{
    encode_into, encode_reader_to_writer, encode_to_string, encode_url_safe_into,
    encode_url_safe_reader_to_writer, encode_url_safe_to_string, Encoder,
};

pub use parallel::{
    decode_parallel, decode_parallel_url_safe, encode_parallel, encode_parallel_url_safe,
};

pub use async_encode::{encode_reader_to_writer_async, encode_url_safe_reader_to_writer_async};

pub use async_decode::{decode_reader_to_writer_async, decode_reader_to_writer_mode_async};
