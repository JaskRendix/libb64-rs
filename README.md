# b64 — Base64 Encoding and Decoding in Rust

A clear and portable Base64 implementation written in safe Rust.  
Includes in‑memory routines, streaming interfaces, optional parallel helpers, URL‑safe encoding, strict decoding, a command‑line tool, and example binaries.

This crate is not yet published on crates.io.  
Use it via Git or build the workspace locally.

---

## Features

- In‑memory encode and decode  
- Streaming encode and decode  
- Optional parallel routines (SIMD)  
- URL‑safe Base64 alphabet (`-` and `_`)  
- Strict decode mode (rejects whitespace, invalid length, invalid padding)  
- Command‑line tool (`base64-cli`)  
- Example binaries  
- Benchmarks  

---

## Workspace Layout

This repository is a Cargo workspace containing:

- `b64/` — the library  
- `base64-cli/` — the command‑line tool  
- `examples/` — example binaries  

Build everything:

```
cargo build --workspace
```

---

## Using the Library

Since the crate is not on crates.io, use a Git dependency:

```toml
[dependencies]
b64 = { git = "https://github.com/your/repo.git" }
```

Or build locally.

---

## In‑Memory API

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

Strict mode rejects:

- whitespace  
- invalid Base64 length  
- invalid padding  
- data after padding  

---

## Streaming API

```rust
use b64::encode_reader_to_writer;
use std::fs::File;

let mut input = File::open("in.bin")?;
let mut output = File::create("out.b64")?;
encode_reader_to_writer(&mut input, &mut output, None)?;
```

### URL‑safe streaming encode

```rust
use b64::encode_url_safe_reader_to_writer;
encode_url_safe_reader_to_writer(&mut input, &mut output, None)?;
```

### Strict streaming decode

```rust
use b64::{decode_reader_to_writer_mode, DecodeMode};
decode_reader_to_writer_mode(&mut input, &mut output, DecodeMode::Strict)?;
```

---

## Command‑Line Tool

Build the CLI:

```
cargo build --release -p base64-cli
```

---

## Encode

```
base64-cli encode --input input.bin --output output.b64
```

Use stdin/stdout:

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

---

## Decode

```
base64-cli decode --input input.b64 --output output.bin
```

Use stdin/stdout:

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

Strict mode rejects:

- whitespace  
- invalid length  
- invalid padding  
- data after padding  

---

## Check Mode

Validate Base64 without writing output:

```
base64-cli decode --check --input file.b64
```

Exit code is nonzero on invalid input.

---

## CLI Options

```
USAGE:
    base64-cli encode [OPTIONS]
    base64-cli decode [OPTIONS]

OPTIONS:
    -i, --input <FILE>       Input file or "-" for stdin
    -o, --output <FILE>      Output file or "-" for stdout
        --wrap <N>           Wrap output every N characters (0 disables wrap)
        --parallel           Use SIMD parallel encoder or decoder
        --url-safe           Use URL-safe Base64 alphabet (- and _)
        --strict             Reject whitespace, invalid length, invalid padding
        --check              Validate Base64 input without writing output
```

---

## Examples

```
cargo run --example encode_file -- input.bin out.b64
cargo run --example decode_file -- input.b64 out.bin
```

---

## Tests and Benchmarks

Run tests:

```
cargo test --workspace
```

Run benchmarks:

```
cargo bench -p b64
```

Benchmark results are in `BENCHMARKS.md`.

---

## Platform Support

Tested on:

- Linux (Ubuntu 22.04)  
- Windows 10 (MSVC)  
- macOS Ventura  

Requires stable Rust.

---

## License

Released into the Public Domain.  
See `LICENSE.md`.
