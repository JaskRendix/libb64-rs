use std::io::{Read, Write};

#[derive(Debug)]
pub enum DecodeError {
    InvalidByte(u8, usize),
    InvalidLength,
    UnexpectedPadding,
}

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

impl std::fmt::Display for DecodeIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeIoError::Io(e) => write!(f, "IO error: {}", e),
            DecodeIoError::Decode(e) => write!(f, "Decode error: {:?}", e),
        }
    }
}

impl std::error::Error for DecodeIoError {}

pub enum DecodeStep {
    A,
    B,
    C,
    D,
}

pub struct Decoder {
    pub step: DecodeStep,
    pub plain: u8,
    pub offset: usize,
    pub seen_padding: bool,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            step: DecodeStep::A,
            plain: 0,
            offset: 0,
            seen_padding: false,
        }
    }

    #[inline]
    fn decode_value(b: u8) -> Option<i8> {
        const PAD: i8 = -2;

        const TABLE: [i8; 80] = [
            62, -1, -1, -1, 63, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, PAD, -1, -1,
            -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, -1, -1, -1, -1, -1, -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
            39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
        ];

        let idx = b.wrapping_sub(b'+') as usize;
        if idx >= TABLE.len() {
            return None;
        }
        Some(TABLE[idx])
    }

    #[inline]
    fn is_whitespace(b: u8) -> bool {
        matches!(b, b' ' | b'\n' | b'\r' | b'\t')
    }

    #[allow(clippy::never_loop)]
    fn next_valid(
        &mut self,
        iter: &mut impl Iterator<Item = u8>,
    ) -> Result<Option<u8>, DecodeError> {
        for b in iter {
            self.offset += 1;

            if Self::is_whitespace(b) {
                continue;
            }

            match Self::decode_value(b) {
                Some(v) if v >= 0 => {
                    if self.seen_padding {
                        return Err(DecodeError::UnexpectedPadding);
                    }
                    return Ok(Some(v as u8));
                }

                Some(-2) => {
                    // '=' padding is only valid in positions 3 or 4 of a quartet,
                    // i.e. when we're in steps C or D.
                    match self.step {
                        DecodeStep::C | DecodeStep::D => {
                            self.seen_padding = true;
                            return Ok(None);
                        }
                        _ => return Err(DecodeError::UnexpectedPadding),
                    }
                }

                _ => return Err(DecodeError::InvalidByte(b, self.offset)),
            }
        }

        Ok(None)
    }

    pub fn decode_block(&mut self, input: &[u8], out: &mut Vec<u8>) -> Result<(), DecodeError> {
        let mut iter = input.iter().copied();
        let mut plain = self.plain;

        loop {
            match self.step {
                DecodeStep::A => {
                    let frag = match self.next_valid(&mut iter)? {
                        None => {
                            self.plain = plain;
                            return Ok(());
                        }
                        Some(v) => v,
                    };
                    plain = (frag & 0x3F) << 2;
                    self.step = DecodeStep::B;
                }

                DecodeStep::B => {
                    // Padding is not allowed in position 2 of a quartet.
                    let frag = match self.next_valid(&mut iter)? {
                        None => return Err(DecodeError::UnexpectedPadding),
                        Some(v) => v,
                    };
                    out.push(plain | ((frag & 0x30) >> 4));
                    plain = (frag & 0x0F) << 4;
                    self.step = DecodeStep::C;
                }

                DecodeStep::C => {
                    match self.next_valid(&mut iter)? {
                        // Padding in position 3: only 1 byte (from B) was valid.
                        None => {
                            self.step = DecodeStep::A;
                            return Ok(());
                        }
                        Some(frag) => {
                            out.push(plain | ((frag & 0x3C) >> 2));
                            plain = (frag & 0x03) << 6;
                            self.step = DecodeStep::D;
                        }
                    }
                }

                DecodeStep::D => {
                    match self.next_valid(&mut iter)? {
                        // Padding in position 4: 2 bytes (from B and C) were valid.
                        None => {
                            self.step = DecodeStep::A;
                            return Ok(());
                        }
                        Some(frag) => {
                            out.push(plain | (frag & 0x3F));
                            self.step = DecodeStep::A;
                        }
                    }
                }
            }
        }
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

    Ok(())
}
