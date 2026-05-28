pub mod decode;
pub mod encode;
pub mod parallel;

pub use decode::{
    decode_reader_to_writer, decode_reader_to_writer_mode, decode_to_vec, decode_to_vec_mode,
    DecodeError, DecodeIoError, DecodeMode,
};

pub use encode::{
    encode_reader_to_writer, encode_to_string, encode_url_safe_reader_to_writer,
    encode_url_safe_to_string,
};
pub use parallel::{
    decode_parallel, decode_parallel_url_safe, encode_parallel, encode_parallel_url_safe,
};
