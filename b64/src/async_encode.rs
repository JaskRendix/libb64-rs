use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::encode::Encoder;

/// Async Base64 encoding using Tokio AsyncRead/AsyncWrite.
pub async fn encode_reader_to_writer_async<R, W>(
    reader: &mut R,
    writer: &mut W,
    wrap: Option<usize>,
) -> tokio::io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut enc = Encoder::new(wrap);
    let mut buf = [0u8; 4096];
    let mut out = String::with_capacity(8192);

    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        enc.encode_block(&buf[..n], &mut out);

        if out.len() >= 4096 {
            writer.write_all(out.as_bytes()).await?;
            out.clear();
        }
    }

    enc.encode_end(&mut out);
    writer.write_all(out.as_bytes()).await?;
    Ok(())
}

/// URL-safe async encoder.
pub async fn encode_url_safe_reader_to_writer_async<R, W>(
    reader: &mut R,
    writer: &mut W,
    wrap: Option<usize>,
) -> tokio::io::Result<()>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    let mut enc = Encoder::new_url_safe(wrap);
    let mut buf = [0u8; 4096];
    let mut out = String::with_capacity(8192);

    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        enc.encode_block(&buf[..n], &mut out);

        if out.len() >= 4096 {
            writer.write_all(out.as_bytes()).await?;
            out.clear();
        }
    }

    enc.encode_end(&mut out);
    writer.write_all(out.as_bytes()).await?;
    Ok(())
}
