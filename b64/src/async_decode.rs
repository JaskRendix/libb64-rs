use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::decode::{DecodeError, DecodeMode, Decoder};

/// Async Base64 decoding (lenient mode).
pub async fn decode_reader_to_writer_async<R, W>(
    reader: &mut R,
    writer: &mut W,
) -> Result<(), DecodeError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    decode_reader_to_writer_mode_async(reader, writer, DecodeMode::Lenient).await
}

/// Async Base64 decoding with strict/lenient mode.
pub async fn decode_reader_to_writer_mode_async<R, W>(
    reader: &mut R,
    writer: &mut W,
    mode: DecodeMode,
) -> Result<(), DecodeError>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut dec = Decoder::new_with_mode(mode);
    let mut buf = [0u8; 4096];
    let mut decoded = Vec::with_capacity(4096);

    loop {
        let n = reader.read(&mut buf).await.map_err(DecodeError::Io)?;
        if n == 0 {
            break;
        }

        // Feed bytes into the persistent stateful decoder
        dec.decode_block(&buf[..n], &mut decoded)?;

        if !decoded.is_empty() {
            writer.write_all(&decoded).await.map_err(DecodeError::Io)?;
            decoded.clear();
        }
    }

    // Finalize after stream ends
    dec.finalize(&mut decoded)?;
    if !decoded.is_empty() {
        writer.write_all(&decoded).await.map_err(DecodeError::Io)?;
    }

    Ok(())
}
