use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Fast Base64 encoder/decoder with SIMD + streaming support"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encode {
        #[arg(short, long)]
        input: Option<String>,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(long)]
        wrap: Option<String>, // "auto" or number

        #[arg(long)]
        parallel: bool,

        #[arg(long)]
        url_safe: bool,

        #[arg(long)]
        no_newline: bool,

        #[arg(long)]
        quiet: bool,
    },

    Decode {
        #[arg(short, long)]
        input: Option<String>,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(long, alias = "validate")]
        check: bool,

        #[arg(long)]
        parallel: bool,

        #[arg(long)]
        url_safe: bool,

        #[arg(long)]
        strict: bool,

        #[arg(long)]
        ignore_garbage: bool,

        #[arg(long)]
        quiet: bool,
    },
}

fn open_input(path: Option<String>) -> anyhow::Result<Box<dyn Read>> {
    match path.as_deref() {
        Some("-") => Ok(Box::new(io::stdin())),
        Some(p) => Ok(Box::new(File::open(p)?)),
        None => Ok(Box::new(io::stdin())),
    }
}

fn open_output(path: Option<String>) -> anyhow::Result<Box<dyn Write>> {
    match path.as_deref() {
        Some("-") => Ok(Box::new(io::stdout())),
        Some(p) => Ok(Box::new(File::create(p)?)),
        None => Ok(Box::new(io::stdout())),
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        // ---------------------------------------------------------
        // ENCODE
        // ---------------------------------------------------------
        Command::Encode {
            input,
            output,
            wrap,
            parallel,
            url_safe,
            no_newline,
            quiet: _,
        } => {
            let mut reader = open_input(input)?;
            let mut writer = open_output(output)?;

            // wrap logic
            let wrap = match wrap.as_deref() {
                Some("auto") => Some(76),
                Some(s) => s.parse::<usize>().ok().filter(|&n| n > 0),
                None => None,
            };

            if parallel {
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                let encoded = if url_safe {
                    b64::encode_parallel_url_safe(&buf)
                } else {
                    b64::encode_parallel(&buf)
                };

                if no_newline {
                    write!(writer, "{}", encoded)?;
                } else {
                    writeln!(writer, "{}", encoded)?;
                }
            } else {
                if url_safe {
                    b64::encode_url_safe_reader_to_writer(&mut reader, &mut writer, wrap)?;
                } else {
                    b64::encode_reader_to_writer(&mut reader, &mut writer, wrap)?;
                }
            }
        }

        // ---------------------------------------------------------
        // DECODE
        // ---------------------------------------------------------
        Command::Decode {
            input,
            output,
            check,
            parallel,
            url_safe,
            strict,
            ignore_garbage,
            quiet,
        } => {
            let mut reader = open_input(input)?;

            // -------------------------
            // CHECK MODE
            // -------------------------
            if check {
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;

                let result = if parallel {
                    if url_safe {
                        b64::decode_parallel_url_safe(&buf)
                    } else {
                        b64::decode_parallel(&buf)
                    }
                } else {
                    if ignore_garbage {
                        let cleaned: String = buf.chars().filter(|c| !c.is_whitespace()).collect();
                        b64::decode_to_vec(&cleaned)
                    } else {
                        b64::decode_to_vec(&buf)
                    }
                };

                match result {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        if !quiet {
                            eprintln!("Decode error: {}", e);
                        }
                        std::process::exit(1);
                    }
                }
            }

            let mode = if strict {
                b64::DecodeMode::Strict
            } else {
                b64::DecodeMode::Lenient
            };

            let mut writer = open_output(output)?;

            // -------------------------
            // PARALLEL DECODE
            // -------------------------
            if parallel {
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;

                let result = if url_safe {
                    b64::decode_parallel_url_safe(&buf)
                } else {
                    b64::decode_parallel(&buf)
                };

                match result {
                    Ok(decoded) => writer.write_all(&decoded)?,
                    Err(e) => {
                        if !quiet {
                            eprintln!("Decode error: {}", e);
                        }
                        std::process::exit(1);
                    }
                }
            }
            // -------------------------
            // STREAMING DECODE
            // -------------------------
            else {
                if ignore_garbage {
                    let mut buf = String::new();
                    reader.read_to_string(&mut buf)?;
                    let cleaned: String = buf.chars().filter(|c| !c.is_whitespace()).collect();

                    match b64::decode_to_vec(&cleaned) {
                        Ok(decoded) => writer.write_all(&decoded)?,
                        Err(e) => {
                            if !quiet {
                                eprintln!("Decode error: {}", e);
                            }
                            std::process::exit(1);
                        }
                    }
                } else {
                    if let Err(e) =
                        b64::decode_reader_to_writer_mode(&mut reader, &mut writer, mode)
                    {
                        if !quiet {
                            eprintln!("Decode error: {}", e);
                        }
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    Ok(())
}
