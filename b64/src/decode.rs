use std::fmt;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    InvalidLength,
    InvalidByte(u8, usize),
    InvalidPadding,
    UnexpectedPadding,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::InvalidLength => write!(f, "invalid Base64 length"),
            DecodeError::InvalidByte(b, pos) => {
                write!(f, "invalid Base64 byte 0x{:02X} at position {}", b, pos)
            }
            DecodeError::InvalidPadding => write!(f, "invalid Base64 padding"),
            DecodeError::UnexpectedPadding => write!(f, "unexpected padding or data after padding"),
        }
    }
}

impl std::error::Error for DecodeError {}

#[derive(Debug)]
pub enum DecodeIoError {
    Io(std::io::Error),
    Decode(DecodeError),
}

impl From<std::io::Error> for DecodeIoError {
    fn from(e: std::io::Error) -> Self {
        DecodeIoError::Io(e)
    }
}

impl fmt::Display for DecodeIoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeIoError::Io(e) => write!(f, "IO error: {}", e),
            DecodeIoError::Decode(e) => write!(f, "Decode error: {}", e),
        }
    }
}

impl std::error::Error for DecodeIoError {}

#[derive(Debug)]
pub struct Decoder {
    buf: [u8; 4],
    buf_len: usize,
    offset: usize,
    padding: u8,
    ended: bool,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            buf: [0; 4],
            buf_len: 0,
            offset: 0,
            padding: 0,
            ended: false,
        }
    }

    #[inline]
    fn is_whitespace(b: u8) -> bool {
        matches!(b, b' ' | b'\n' | b'\r' | b'\t')
    }

    #[inline]
    fn decode_value(b: u8) -> Option<u8> {
        match b {
            b'A'..=b'Z' => Some(b - b'A'),
            b'a'..=b'z' => Some(b - b'a' + 26),
            b'0'..=b'9' => Some(b - b'0' + 52),
            b'+' | b'-' => Some(62), // standard '+' and URL-safe '-'
            b'/' | b'_' => Some(63), // standard '/' and URL-safe '_'
            _ => None,
        }
    }

    /// Feed a block of input into the decoder, appending decoded bytes to `out`.
    /// This can be called repeatedly for streaming; call `finalize` at the end.
    pub fn decode_block(&mut self, input: &[u8], out: &mut Vec<u8>) -> Result<(), DecodeError> {
        for b in input.iter().copied() {
            if Self::is_whitespace(b) {
                continue;
            }

            // Only count non-whitespace for error offsets.
            self.offset += 1;

            if self.ended {
                // After we've seen padding and finished the last quartet,
                // any further non-whitespace is invalid.
                return Err(DecodeError::UnexpectedPadding);
            }

            if b == b'=' {
                // Padding: only allowed in the final quartet, positions 3 or 4.
                self.padding += 1;
                if self.padding > 2 {
                    return Err(DecodeError::UnexpectedPadding);
                }

                // Padding fills remaining slots in the quartet as zeros.
                self.buf[self.buf_len] = 0;
                self.buf_len += 1;

                if self.buf_len == 4 {
                    // Final quartet with padding.
                    match self.padding {
                        1 => {
                            // "xxx=" → 2 bytes
                            let b0 = self.buf[0];
                            let b1 = self.buf[1];
                            let b2 = self.buf[2];

                            out.push((b0 << 2) | (b1 >> 4));
                            out.push((b1 << 4) | (b2 >> 2));
                        }
                        2 => {
                            // "xx==" → 1 byte
                            let b0 = self.buf[0];
                            let b1 = self.buf[1];

                            out.push((b0 << 2) | (b1 >> 4));
                        }
                        _ => return Err(DecodeError::UnexpectedPadding),
                    }

                    self.buf_len = 0;
                    self.ended = true;
                }

                continue;
            }

            if self.padding > 0 {
                // Data after padding is not allowed.
                return Err(DecodeError::UnexpectedPadding);
            }

            let v = match Self::decode_value(b) {
                Some(v) => v,
                None => return Err(DecodeError::InvalidByte(b, self.offset)),
            };

            self.buf[self.buf_len] = v;
            self.buf_len += 1;

            if self.buf_len == 4 {
                let b0 = self.buf[0];
                let b1 = self.buf[1];
                let b2 = self.buf[2];
                let b3 = self.buf[3];

                out.push((b0 << 2) | (b1 >> 4));
                out.push((b1 << 4) | (b2 >> 2));
                out.push((b2 << 6) | b3);

                self.buf_len = 0;
            }
        }

        Ok(())
    }

    /// Finalize decoding after all input has been fed.
    /// Ensures there is no incomplete quartet left.
    pub fn finalize(&mut self, _out: &mut [u8]) -> Result<(), DecodeError> {
        if self.ended {
            // Already finished cleanly.
            if self.buf_len != 0 {
                return Err(DecodeError::InvalidLength);
            }
            return Ok(());
        }

        if self.buf_len == 0 {
            return Ok(());
        }

        // If we have leftover sextets without padding, that's invalid.
        Err(DecodeError::InvalidLength)
    }
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn decode_to_vec(input: &str) -> Result<Vec<u8>, DecodeError> {
    let mut dec = Decoder::new();
    let mut out = Vec::new();
    dec.decode_block(input.as_bytes(), &mut out)?;
    dec.finalize(&mut out[..])?;
    Ok(out)
}

pub fn decode_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), DecodeIoError> {
    let mut dec = Decoder::new();
    let mut buf = [0u8; 4096];
    let mut out = Vec::new();

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        dec.decode_block(&buf[..n], &mut out)
            .map_err(DecodeIoError::Decode)?;
        writer.write_all(&out)?;
        out.clear();
    }

    dec.finalize(&mut out).map_err(DecodeIoError::Decode)?;
    if !out.is_empty() {
        writer.write_all(&out)?;
    }

    Ok(())
}
