use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(author, version, about)]
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
        wrap: Option<usize>,
    },

    Decode {
        #[arg(short, long)]
        input: Option<String>,

        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode {
            input,
            output,
            wrap,
        } => {
            let mut input: Box<dyn Read> = match input {
                Some(path) => Box::new(File::open(path)?),
                None => Box::new(io::stdin()),
            };

            let mut output: Box<dyn Write> = match output {
                Some(path) => Box::new(File::create(path)?),
                None => Box::new(io::stdout()),
            };

            let wrap = wrap.filter(|&n| n > 0);
            b64::encode_reader_to_writer(&mut input, &mut output, wrap)?;
        }

        Command::Decode { input, output } => {
            let mut input: Box<dyn Read> = match input {
                Some(path) => Box::new(File::open(path)?),
                None => Box::new(io::stdin()),
            };

            let mut output: Box<dyn Write> = match output {
                Some(path) => Box::new(File::create(path)?),
                None => Box::new(io::stdout()),
            };

            b64::decode_reader_to_writer(&mut input, &mut output)?;
        }
    }

    Ok(())
}
