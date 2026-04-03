use std::fs::File;

fn main() -> anyhow::Result<()> {
    let mut input = File::open("input.bin")?;
    let mut output = File::create("output.b64")?;
    b64::encode_reader_to_writer(&mut input, &mut output, None)?;
    Ok(())
}
