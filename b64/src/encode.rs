use std::io::{Read, Write};

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
    wrapped: bool,
    line_pos: usize,
}

impl Encoder {
    pub fn new(chars_per_line: Option<usize>) -> Self {
        Self {
            step: EncodeStep::A,
            result: 0,
            chars_per_line,
            wrapped: false,
            line_pos: 0,
        }
    }

    #[inline]
    fn encode_value(v: u8) -> char {
        const TABLE: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        TABLE[v as usize] as char
    }

    #[inline]
    fn push_char(&mut self, out: &mut String, c: char) {
        self.write_break(out);
        out.push(c);
        self.line_pos += 1;
    }

    pub fn encode_block(&mut self, input: &[u8], out: &mut String) {
        let mut iter = input.iter().copied();

        loop {
            match self.step {
                EncodeStep::A => {
                    let b = match iter.next() {
                        None => return,
                        Some(b) => b,
                    };
                    let v = (b >> 2) & 0b0011_1111;
                    self.push_char(out, Self::encode_value(v));
                    self.result = (b & 0b0000_0011) << 4;
                    self.step = EncodeStep::B;
                }

                EncodeStep::B => {
                    let b = match iter.next() {
                        None => return,
                        Some(b) => b,
                    };
                    let v = self.result | ((b >> 4) & 0b0000_1111);
                    self.push_char(out, Self::encode_value(v));
                    self.result = (b & 0b0000_1111) << 2;
                    self.step = EncodeStep::C;
                }

                EncodeStep::C => {
                    let b = match iter.next() {
                        None => return,
                        Some(b) => b,
                    };
                    let v = self.result | ((b >> 6) & 0b0000_0011);
                    self.push_char(out, Self::encode_value(v));

                    let v2 = b & 0b0011_1111;
                    self.push_char(out, Self::encode_value(v2));

                    self.step = EncodeStep::A;
                }
            }
        }
    }

    pub fn encode_end(&mut self, out: &mut String) {
        match self.step {
            EncodeStep::A => {}

            EncodeStep::B => {
                self.push_char(out, Self::encode_value(self.result));
                out.push('='); // do NOT wrap padding
                out.push('=');
                self.line_pos += 2;
            }

            EncodeStep::C => {
                self.push_char(out, Self::encode_value(self.result));
                out.push('=');
                self.line_pos += 1;
            }
        }

        if self.wrapped {
            out.push('\n');
        }

        self.step = EncodeStep::A;
    }

    #[inline]
    fn write_break(&mut self, out: &mut String) {
        if let Some(n) = self.chars_per_line {
            if self.line_pos >= n {
                out.push('\n');
                self.line_pos = 0;
                self.wrapped = true;
            }
        }
    }
}

pub fn encode_to_string(input: &[u8]) -> String {
    let mut enc = Encoder::new(None);
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
        if out.len() > 4096 {
            writer.write_all(out.as_bytes())?;
            out.clear();
        }
    }

    enc.encode_end(&mut out);
    writer.write_all(out.as_bytes())?;
    Ok(())
}
