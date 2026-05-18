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
    /// Encode binary data to Base64
    Encode {
        /// Input file (or "-" for stdin)
        #[arg(short, long)]
        input: Option<String>,

        /// Output file (or "-" for stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Wrap output at N columns (0 = no wrap)
        #[arg(long)]
        wrap: Option<usize>,

        /// Use parallel SIMD encoder
        #[arg(long)]
        parallel: bool,
    },

    /// Decode Base64 back to binary
    Decode {
        /// Input file (or "-" for stdin)
        #[arg(short, long)]
        input: Option<String>,

        /// Output file (or "-" for stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Only check validity; do not write output
        #[arg(long)]
        check: bool,

        /// Use parallel SIMD decoder
        #[arg(long)]
        parallel: bool,
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
        } => {
            let mut reader = open_input(input)?;
            let mut writer = open_output(output)?;

            if parallel {
                // Parallel encode requires full input in memory
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                let encoded = b64::encode_parallel(&buf);
                writer.write_all(encoded.as_bytes())?;
            } else {
                let wrap = wrap.filter(|&n| n > 0);
                b64::encode_reader_to_writer(&mut reader, &mut writer, wrap)?;
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
        } => {
            let mut reader = open_input(input)?;

            // -------------------------
            // CHECK MODE
            // -------------------------
            if check {
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;

                let result = if parallel {
                    b64::decode_parallel(&buf)
                } else {
                    b64::decode_to_vec(&buf)
                };

                match result {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        eprintln!("Decode error: {}", e);
                        std::process::exit(1);
                    }
                }
            }

            let mut writer = open_output(output)?;

            // -------------------------
            // PARALLEL DECODE
            // -------------------------
            if parallel {
                let mut buf = String::new();
                reader.read_to_string(&mut buf)?;

                match b64::decode_parallel(&buf) {
                    Ok(decoded) => writer.write_all(&decoded)?,
                    Err(e) => {
                        eprintln!("Decode error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            // -------------------------
            // STREAMING DECODE
            // -------------------------
            else {
                if let Err(e) = b64::decode_reader_to_writer(&mut reader, &mut writer) {
                    eprintln!("Decode error: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
