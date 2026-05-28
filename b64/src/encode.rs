use std::io::{Read, Write};

#[derive(Clone, Copy)]
enum Alphabet {
    Standard,
    UrlSafe,
}

#[derive(Clone, Copy)]
pub enum EncodeStep {
    A,
    B,
    C,
}

pub struct Encoder {
    pub step: EncodeStep,
    pub result: u8,
    pub chars_per_line: Option<usize>,
    line_pos: usize,
    alphabet: Alphabet,
}

impl Encoder {
    pub fn new(chars_per_line: Option<usize>) -> Self {
        Self {
            step: EncodeStep::A,
            result: 0,
            chars_per_line,
            line_pos: 0,
            alphabet: Alphabet::Standard,
        }
    }

    pub fn new_url_safe(chars_per_line: Option<usize>) -> Self {
        Self {
            step: EncodeStep::A,
            result: 0,
            chars_per_line,
            line_pos: 0,
            alphabet: Alphabet::UrlSafe,
        }
    }

    #[inline]
    fn encode_value(&self, v: u8) -> u8 {
        const TABLE_STD: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        const TABLE_URL: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

        match self.alphabet {
            Alphabet::Standard => TABLE_STD[v as usize],
            Alphabet::UrlSafe => TABLE_URL[v as usize],
        }
    }

    #[inline]
    fn push_char(&mut self, out: &mut String, c: u8) {
        self.write_break(out);
        out.push(c as char);
        self.line_pos += 1;
    }

    pub fn encode_block(&mut self, input: &[u8], out: &mut String) {
        let mut iter = input.iter().copied();

        loop {
            match self.step {
                EncodeStep::A => {
                    let Some(b) = iter.next() else { return };
                    let v = (b >> 2) & 0b0011_1111;
                    self.push_char(out, self.encode_value(v));
                    self.result = (b & 0b0000_0011) << 4;
                    self.step = EncodeStep::B;
                }

                EncodeStep::B => {
                    let Some(b) = iter.next() else { return };
                    let v = self.result | ((b >> 4) & 0b0000_1111);
                    self.push_char(out, self.encode_value(v));
                    self.result = (b & 0b0000_1111) << 2;
                    self.step = EncodeStep::C;
                }

                EncodeStep::C => {
                    let Some(b) = iter.next() else { return };
                    let v = self.result | ((b >> 6) & 0b0000_0011);
                    self.push_char(out, self.encode_value(v));

                    let v2 = b & 0b0011_1111;
                    self.push_char(out, self.encode_value(v2));

                    self.step = EncodeStep::A;
                }
            }
        }
    }

    pub fn encode_end(&mut self, out: &mut String) {
        match self.step {
            EncodeStep::A => {}

            EncodeStep::B => {
                self.push_char(out, self.encode_value(self.result));
                out.push('=');
                out.push('=');
                self.line_pos += 2;
            }

            EncodeStep::C => {
                self.push_char(out, self.encode_value(self.result));
                out.push('=');
                self.line_pos += 1;
            }
        }

        self.step = EncodeStep::A;
        self.line_pos = 0;
    }

    #[inline]
    fn write_break(&mut self, out: &mut String) {
        if let Some(n) = self.chars_per_line {
            if self.line_pos >= n {
                out.push('\n');
                self.line_pos = 0;
            }
        }
    }

    // ---------------------------------------------------------
    // ZERO-COPY: push a single encoded byte into caller Vec<u8>
    // ---------------------------------------------------------
    #[inline]
    fn push_byte_into(&mut self, out: &mut Vec<u8>, b: u8) {
        // Same wrap logic as push_char(), but for raw bytes.
        if let Some(n) = self.chars_per_line {
            if self.line_pos >= n {
                out.push(b'\n');
                self.line_pos = 0;
            }
        }
        out.push(b);
        self.line_pos += 1;
    }

    // ---------------------------------------------------------
    // ZERO-COPY: encode_block_into
    // Same logic as encode_block(), but writes bytes into Vec<u8>
    // ---------------------------------------------------------
    pub fn encode_block_into(&mut self, input: &[u8], out: &mut Vec<u8>) {
        let mut iter = input.iter().copied();

        loop {
            match self.step {
                EncodeStep::A => {
                    let Some(b) = iter.next() else { return };
                    let v = (b >> 2) & 0b0011_1111;
                    self.push_byte_into(out, self.encode_value(v));
                    self.result = (b & 0b0000_0011) << 4;
                    self.step = EncodeStep::B;
                }

                EncodeStep::B => {
                    let Some(b) = iter.next() else { return };
                    let v = self.result | ((b >> 4) & 0b0000_1111);
                    self.push_byte_into(out, self.encode_value(v));
                    self.result = (b & 0b0000_1111) << 2;
                    self.step = EncodeStep::C;
                }

                EncodeStep::C => {
                    let Some(b) = iter.next() else { return };
                    let v = self.result | ((b >> 6) & 0b0000_0011);
                    self.push_byte_into(out, self.encode_value(v));

                    let v2 = b & 0b0011_1111;
                    self.push_byte_into(out, self.encode_value(v2));

                    self.step = EncodeStep::A;
                }
            }
        }
    }

    // ---------------------------------------------------------
    // ZERO-COPY: encode_end_into
    // Same logic as encode_end(), but writes bytes into Vec<u8>
    // ---------------------------------------------------------
    pub fn encode_end_into(&mut self, out: &mut Vec<u8>) {
        match self.step {
            EncodeStep::A => {}

            EncodeStep::B => {
                self.push_byte_into(out, self.encode_value(self.result));
                out.push(b'=');
                out.push(b'=');
                self.line_pos += 2;
            }

            EncodeStep::C => {
                self.push_byte_into(out, self.encode_value(self.result));
                out.push(b'=');
                self.line_pos += 1;
            }
        }

        self.step = EncodeStep::A;
        self.line_pos = 0;
    }
}

pub fn encode_to_string(input: &[u8]) -> String {
    let mut enc = Encoder::new(None);
    let mut out = String::new();
    enc.encode_block(input, &mut out);
    enc.encode_end(&mut out);
    out
}

// ---------------------------------------------------------
// ZERO-COPY: encode_into
// Caller provides the output Vec<u8>.
// ---------------------------------------------------------
pub fn encode_into(input: &[u8], out: &mut Vec<u8>) {
    let mut enc = Encoder::new(None);
    enc.encode_block_into(input, out);
    enc.encode_end_into(out);
}

// ---------------------------------------------------------
// ZERO-COPY: encode_url_safe_into
// ---------------------------------------------------------
pub fn encode_url_safe_into(input: &[u8], out: &mut Vec<u8>) {
    let mut enc = Encoder::new_url_safe(None);
    enc.encode_block_into(input, out);
    enc.encode_end_into(out);
}

pub fn encode_url_safe_to_string(input: &[u8]) -> String {
    let mut enc = Encoder::new_url_safe(None);
    let mut out = String::new();
    enc.encode_block(input, &mut out);
    enc.encode_end(&mut out);
    out
}

pub fn encode_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    wrap: Option<usize>,
) -> std::io::Result<()> {
    let mut enc = Encoder::new(wrap);
    let mut buf = [0u8; 4096];
    let mut out = String::with_capacity(8192);

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        enc.encode_block(&buf[..n], &mut out);

        if out.len() >= 4096 {
            writer.write_all(out.as_bytes())?;
            out.clear();
        }
    }

    enc.encode_end(&mut out);
    writer.write_all(out.as_bytes())?;
    Ok(())
}

pub fn encode_url_safe_reader_to_writer<R: Read, W: Write>(
    reader: &mut R,
    writer: &mut W,
    wrap: Option<usize>,
) -> std::io::Result<()> {
    let mut enc = Encoder::new_url_safe(wrap);
    let mut buf = [0u8; 4096];
    let mut out = String::with_capacity(8192);

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }

        enc.encode_block(&buf[..n], &mut out);

        if out.len() >= 4096 {
            writer.write_all(out.as_bytes())?;
            out.clear();
        }
    }

    enc.encode_end(&mut out);
    writer.write_all(out.as_bytes())?;
    Ok(())
}
