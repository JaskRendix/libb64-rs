# b64: Base64 Encoding/Decoding Routines
======================================

## Overview
b64 is a Rust library for encoding and decoding data in Base64 format.  
It provides in‑memory routines, streaming interfaces, file‑based tools, and optional parallel functions.  
Small example binaries are included.

Base64 is ASCII text. It is useful for storing binary data in text files or sending binary data through text‑only channels.

## References
* Wikipedia:  
  https://en.wikipedia.org/wiki/Base64  
* Original C libb64 project:  
  https://sourceforge.net/projects/libb64/  

## Why?
The goal is a clear and portable Base64 implementation without licensing issues.  
The original libb64 codebase used a coroutine‑style C technique that is uncommon today.  
This Rust version keeps the spirit of the original project while providing a safe and modern API.

The crate is released under a permissive license so it can be used in any project.

## License
This work is released into the Public Domain.  
See the LICENSE file for details.

## History
This library is based on the original libb64 project on SourceForge.  
Development there stopped in 2010.  
The code was forked to GitHub, patches were collected, and the project continued.

GitHub project home:  
https://github.com/libb64/libb64

This Rust version re‑implements the functionality with a focus on correctness, clarity, and portability.

## Commandline Use
The repository includes a small CLI tool named `base64-cli`.  
It can encode and decode files.

To encode a file:

```
$ base64-cli -e input.bin output.b64
```

To decode a file:

```
$ base64-cli -d input.b64 output.bin
```

## Programming
The library exposes simple functions for in‑memory use:

```rust
use b64::{encode_to_string, decode_to_vec};

let encoded = encode_to_string(b"hello");
let decoded = decode_to_vec(&encoded).unwrap();
```

Streaming interfaces are also available:

```rust
use b64::encode_reader_to_writer;
use std::fs::File;

let mut input = File::open("in.bin")?;
let mut output = File::create("out.b64")?;
encode_reader_to_writer(&mut input, &mut output, None)?;
```

Parallel routines are optional and provided for experimentation.

## Example code
The `examples` directory contains small programs that show how to use the library for file encoding and decoding.

## Implementation
The original C libb64 used a coroutine‑style switch‑based technique.  
It was fast and compact, but it relied on static state and required careful handling in multithreaded code.

This Rust version does not use coroutines.  
It uses straightforward state machines and safe Rust code.  
The streaming API mirrors the structure of the original library while avoiding global state.

Benchmark results are available in `BENCHMARK.md`.  
They include in‑memory throughput, file throughput, mmap performance, and a comparison with the original C libb64 numbers.
