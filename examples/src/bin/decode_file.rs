use std::fs::File;

fn main() -> anyhow::Result<()> {
    let mut input = File::open("input.b64")?;
    let mut output = File::create("output.bin")?;
    b64::decode_reader_to_writer(&mut input, &mut output)?;
    Ok(())
}
