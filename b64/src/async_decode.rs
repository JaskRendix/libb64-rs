use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::decode::{decode_to_vec_mode_into, DecodeError, DecodeMode};

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
    let mut buf = [0u8; 4096];
    let mut decoded = Vec::with_capacity(4096);

    loop {
        let n = reader.read(&mut buf).await.map_err(DecodeError::Io)?;
        if n == 0 {
            break;
        }

        // Decode chunk into caller buffer
        let chunk = std::str::from_utf8(&buf[..n]).map_err(|_| DecodeError::InvalidByte(0, 0))?;
        decode_to_vec_mode_into(chunk, mode, &mut decoded)?;

        // Write decoded bytes
        writer.write_all(&decoded).await.map_err(DecodeError::Io)?;
        decoded.clear();
    }

    Ok(())
}
