# b64 — Base64 Encoding and Decoding in Rust

A Base64 implementation written in safe Rust.  
Provides in‑memory routines, streaming interfaces, async I/O, parallel encoders, optional SIMD backends, URL‑safe encoding, strict decoding, a command‑line tool, and example binaries.

Use it through Git or build the workspace locally.

---

## Installation

This crate is not published on crates.io.  
Use it via Git:

```toml
[dependencies]
b64 = { git = "https://github.com/JaskRendix/libb64-rs" }
```

---

## Features

- In‑memory encode and decode  
- Streaming encode and decode  
- Async encode and decode (Tokio)  
- Parallel encode and decode (Rayon)  
- SIMD‑accelerated encode (AVX2) with runtime dispatch  
- URL‑safe alphabet (`-` and `_`)  
- Strict decode mode  
- Command‑line tool  
- Example binaries  
- Benchmarks  

---

## Feature flags

- `simd` — enable SIMD support  
- `simd-avx2` — AVX2 backend on x86_64  
- `simd-sse2` — reserved for future SSE2 backend  
- `simd-neon` — reserved for future NEON backend  

The default feature set enables SIMD.

---

## Workspace layout

- `b64/` — library  
- `base64-cli/` — command‑line tool  
- `examples/` — example binaries  

Build everything:

```
cargo build --workspace
```

---

## In‑memory API

```rust
use b64::{encode_to_string, decode_to_vec};

let encoded = encode_to_string(b"hello");
let decoded = decode_to_vec(&encoded).unwrap();
```

### URL‑safe encoding

```rust
use b64::encode_url_safe_to_string;

let encoded = encode_url_safe_to_string(b"hello?");
```

### Strict decode mode

```rust
use b64::{decode_to_vec_mode, DecodeMode};

let decoded = decode_to_vec_mode("aGVsbG8=", DecodeMode::Strict).unwrap();
```

Strict mode rejects whitespace, invalid length, invalid padding, and data after padding.

---

## Streaming API

```rust
use b64::encode_reader_to_writer;
use std::fs::File;

let mut input = File::open("in.bin")?;
let mut output = File::create("out.b64")?;
encode_reader_to_writer(&mut input, &mut output, None)?;
```

URL‑safe streaming encode:

```rust
use b64::encode_url_safe_reader_to_writer;
encode_url_safe_reader_to_writer(&mut input, &mut output, None)?;
```

Strict streaming decode:

```rust
use b64::{decode_reader_to_writer_mode, DecodeMode};
decode_reader_to_writer_mode(&mut input, &mut output, DecodeMode::Strict)?;
```

---

## Async API (Tokio)

Async functions mirror the sync API.

```rust
use b64::{
    encode_reader_to_writer_async,
    decode_reader_to_writer_async,
};
use tokio::io::Cursor;

#[tokio::main]
async fn main() {
    let mut reader = Cursor::new(b"hello async".to_vec());
    let mut encoded = Vec::new();

    encode_reader_to_writer_async(&mut reader, &mut encoded, None)
        .await
        .unwrap();

    let mut reader2 = Cursor::new(encoded);
    let mut decoded = Vec::new();

    decode_reader_to_writer_async(&mut reader2, &mut decoded)
        .await
        .unwrap();

    assert_eq!(decoded, b"hello async");
}
```

---

## Parallel and SIMD API

Parallel encode uses Rayon for chunked multi‑threaded processing.  
On x86_64, AVX2 is used when available; otherwise the scalar backend is used.

```rust
use b64::encode_parallel;

let encoded = encode_parallel(b"hello world");
```

URL‑safe and decode variants are available.

---

## Command‑line tool

Build:

```
cargo build --release -p base64-cli
```

### Encode

```
base64-cli encode --input input.bin --output output.b64
```

Stdin/stdout:

```
cat input.bin | base64-cli encode --wrap 76 > out.b64
```

Parallel encode:

```
base64-cli encode --parallel --input in.bin --output out.b64
```

URL‑safe encode:

```
base64-cli encode --url-safe --input in.bin --output out.b64
```

No trailing newline:

```
base64-cli encode --no-newline --input in.bin --output out.b64
```

### Decode

```
base64-cli decode --input input.b64 --output output.bin
```

Stdin/stdout:

```
cat input.b64 | base64-cli decode > out.bin
```

Parallel decode:

```
base64-cli decode --parallel --input in.b64 --output out.bin
```

Strict decode:

```
base64-cli decode --strict --input in.b64 --output out.bin
```

Ignore whitespace:

```
base64-cli decode --ignore-garbage --input in.b64 --output out.bin
```

### Check / Validate mode

```
base64-cli decode --check --input file.b64
base64-cli decode --validate --input file.b64
```

Exit code is nonzero on invalid input.

### Quiet mode

```
base64-cli decode --quiet --input bad.b64
```

Suppresses error messages.

---

## CLI options

```
USAGE:
    base64-cli encode [OPTIONS]
    base64-cli decode [OPTIONS]

OPTIONS:
    -i, --input <FILE>       Input file or "-" for stdin
    -o, --output <FILE>      Output file or "-" for stdout
        --wrap <N>           Wrap output every N characters (0 disables wrap)
        --wrap auto          Wrap at 76 characters
        --no-newline         Do not print trailing newline (parallel encode)
        --parallel           Use parallel encoder or decoder
        --url-safe           Use URL-safe alphabet
        --strict             Reject invalid input
        --ignore-garbage     Strip whitespace before decode
        --check              Validate input without writing output
        --validate           Alias for --check
        --quiet              Suppress error messages
```

---

## Examples

```
cargo run --example encode_file -- input.bin out.b64
cargo run --example decode_file -- input.b64 out.bin
cargo run --example async_zero_copy
```

---

## Tests and benchmarks

Run tests:

```
cargo test --workspace
```

Parallel scalar and AVX2 backends have dedicated test suites.

Run benchmarks:

```
cargo bench -p b64
```

Benchmark results are in `BENCHMARKS.md`.

---

## Platform support

Parallel encode works on all platforms.  
SIMD acceleration is available on x86_64 with AVX2.  
Requires stable Rust.

---

## License

Public Domain.  
See `LICENSE.md`.
