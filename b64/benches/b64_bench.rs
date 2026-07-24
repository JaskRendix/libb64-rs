use b64::{
    decode_parallel, decode_reader_to_writer, decode_reader_to_writer_async, decode_to_vec,
    encode_parallel, encode_reader_to_writer, encode_reader_to_writer_async,
    encode_url_safe_reader_to_writer_async, encode_to_string,
};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use memmap2::Mmap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use tokio::runtime::Runtime;

const FILE_SIZE: usize = 18_000_000;

fn ensure_bigfile() {
    if !Path::new("bigfile.bin").exists() {
        let mut f = File::create("bigfile.bin").unwrap();
        f.write_all(&vec![42u8; FILE_SIZE]).unwrap();
    }
}

fn bench_encode(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];

    let mut group = c.benchmark_group("encode_in_memory");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("encode_to_string", |b| {
        b.iter(|| {
            let _ = encode_to_string(&data);
        });
    });

    group.finish();
}

fn bench_decode(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];
    let encoded = encode_to_string(&data);

    let mut group = c.benchmark_group("decode_in_memory");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("decode_to_vec", |b| {
        b.iter(|| {
            let _ = decode_to_vec(&encoded).unwrap();
        });
    });

    group.finish();
}

fn bench_encode_file(c: &mut Criterion) {
    ensure_bigfile();

    let mut group = c.benchmark_group("encode_file");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("encode_file_unbuffered", |b| {
        b.iter(|| {
            let mut input = File::open("bigfile.bin").unwrap();
            let mut output = File::create("out.b64").unwrap();
            encode_reader_to_writer(&mut input, &mut output, None).unwrap();
        });
    });

    group.bench_function("encode_file_buffered", |b| {
        b.iter(|| {
            let input = BufReader::with_capacity(128 * 1024, File::open("bigfile.bin").unwrap());
            let output = BufWriter::with_capacity(128 * 1024, File::create("out.b64").unwrap());
            let mut input = input;
            let mut output = output;
            encode_reader_to_writer(&mut input, &mut output, None).unwrap();
        });
    });

    group.finish();
}

fn bench_decode_file(c: &mut Criterion) {
    ensure_bigfile();

    {
        let mut input = File::open("bigfile.bin").unwrap();
        let mut output = File::create("out.b64").unwrap();
        encode_reader_to_writer(&mut input, &mut output, None).unwrap();
    }

    let mut group = c.benchmark_group("decode_file");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("decode_file_unbuffered", |b| {
        b.iter(|| {
            let mut input = File::open("out.b64").unwrap();
            let mut output = File::create("decoded.bin").unwrap();
            decode_reader_to_writer(&mut input, &mut output).unwrap();
        });
    });

    group.bench_function("decode_file_buffered", |b| {
        b.iter(|| {
            let input = BufReader::with_capacity(128 * 1024, File::open("out.b64").unwrap());
            let output = BufWriter::with_capacity(128 * 1024, File::create("decoded.bin").unwrap());
            let mut input = input;
            let mut output = output;
            decode_reader_to_writer(&mut input, &mut output).unwrap();
        });
    });

    group.finish();
}

fn bench_encode_mmap(c: &mut Criterion) {
    ensure_bigfile();

    let mut group = c.benchmark_group("encode_mmap");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("encode_mmap", |b| {
        b.iter(|| {
            let file = File::open("bigfile.bin").unwrap();
            let mmap = unsafe { Mmap::map(&file).unwrap() };
            let _ = encode_to_string(&mmap);
        });
    });

    group.finish();
}

fn bench_decode_mmap(c: &mut Criterion) {
    ensure_bigfile();

    let encoded = {
        let file = File::open("bigfile.bin").unwrap();
        let mmap = unsafe { Mmap::map(&file).unwrap() };
        encode_to_string(&mmap)
    };

    let mut group = c.benchmark_group("decode_mmap");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("decode_mmap", |b| {
        b.iter(|| {
            let _ = decode_to_vec(&encoded).unwrap();
        });
    });

    group.finish();
}

fn bench_parallel(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];
    let encoded = encode_parallel(&data);

    let mut group = c.benchmark_group("parallel");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("encode_parallel", |b| {
        b.iter(|| {
            let _ = encode_parallel(&data);
        });
    });

    group.bench_function("decode_parallel", |b| {
        b.iter(|| {
            let _ = decode_parallel(&encoded).unwrap();
        });
    });

    group.finish();
}

fn bench_encode_decode_roundtrip(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];

    let mut group = c.benchmark_group("encode_decode_roundtrip");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("encode_then_decode", |b| {
        b.iter(|| {
            let encoded = encode_to_string(&data);
            let _decoded = decode_to_vec(&encoded).unwrap();
        });
    });

    group.finish();
}

fn bench_parallel_roundtrip(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];

    let mut group = c.benchmark_group("parallel_roundtrip");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("parallel_encode_then_decode", |b| {
        b.iter(|| {
            let encoded = encode_parallel(&data);
            let _decoded = decode_parallel(&encoded).unwrap();
        });
    });

    group.finish();
}

fn bench_scalar_vs_parallel(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];

    let mut group = c.benchmark_group("scalar_vs_parallel");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("scalar_encode", |b| {
        b.iter(|| {
            let _ = encode_to_string(&data);
        });
    });

    group.bench_function("parallel_encode", |b| {
        b.iter(|| {
            let _ = encode_parallel(&data);
        });
    });

    group.finish();
}

fn bench_url_safe(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];

    let mut group = c.benchmark_group("url_safe");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("encode_url_safe_into", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            b64::encode_url_safe_into(&data, &mut out);
        });
    });

    let mut encoded = Vec::new();
    b64::encode_url_safe_into(&data, &mut encoded);
    let encoded_str = String::from_utf8(encoded).unwrap();

    group.bench_function("decode_url_safe_to_vec", |b| {
        b.iter(|| {
            let _ = decode_to_vec(&encoded_str).unwrap();
        });
    });

    group.finish();
}

//
// Async benchmarks: file streaming encode/decode
//
fn bench_async_streaming(c: &mut Criterion) {
    ensure_bigfile();

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("async_streaming");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("async_encode_file", |b| {
        b.to_async(&rt).iter(|| async {
            let mut input = File::open("bigfile.bin").unwrap();
            let mut output = Vec::new();
            encode_reader_to_writer_async(&mut input, &mut output, None)
                .await
                .unwrap();
        });
    });

    group.bench_function("async_decode_file", |b| {
        b.to_async(&rt).iter(|| async {
            let mut input = File::open("bigfile.bin").unwrap();
            let mut encoded = Vec::new();
            encode_reader_to_writer_async(&mut input, &mut encoded, None)
                .await
                .unwrap();

            let mut decoded = Vec::new();
            decode_reader_to_writer_async(&mut &encoded[..], &mut decoded)
                .await
                .unwrap();
        });
    });

    group.finish();
}

//
// Async URL‑safe streaming benchmarks
//
fn bench_async_url_safe(c: &mut Criterion) {
    ensure_bigfile();

    let rt = Runtime::new().unwrap();

    let mut group = c.benchmark_group("async_url_safe");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("async_encode_url_safe_file", |b| {
        b.to_async(&rt).iter(|| async {
            let mut input = File::open("bigfile.bin").unwrap();
            let mut output = Vec::new();
            encode_url_safe_reader_to_writer_async(&mut input, &mut output, None)
                .await
                .unwrap();
        });
    });

    group.bench_function("async_decode_url_safe_file", |b| {
        b.to_async(&rt).iter(|| async {
            let mut input = File::open("bigfile.bin").unwrap();
            let mut encoded = Vec::new();
            encode_url_safe_reader_to_writer_async(&mut input, &mut encoded, None)
                .await
                .unwrap();

            let mut decoded = Vec::new();
            decode_reader_to_writer_async(&mut &encoded[..], &mut decoded)
                .await
                .unwrap();
        });
    });

    group.finish();
}

//
// Combined encode/decode throughput dashboard
//
fn bench_throughput_dashboard(c: &mut Criterion) {
    let data = vec![42u8; FILE_SIZE];
    let encoded_scalar = encode_to_string(&data);
    let encoded_parallel = encode_parallel(&data);

    let mut group = c.benchmark_group("throughput_dashboard");
    group.throughput(Throughput::Bytes(FILE_SIZE as u64));

    group.bench_function("scalar_encode", |b| {
        b.iter(|| {
            let _ = encode_to_string(&data);
        });
    });

    group.bench_function("scalar_decode", |b| {
        b.iter(|| {
            let _ = decode_to_vec(&encoded_scalar).unwrap();
        });
    });

    group.bench_function("parallel_encode", |b| {
        b.iter(|| {
            let _ = encode_parallel(&data);
        });
    });

    group.bench_function("parallel_decode", |b| {
        b.iter(|| {
            let _ = decode_parallel(&encoded_parallel).unwrap();
        });
    });

    group.finish();
}

fn bench_encode_decode_loop(c: &mut Criterion) {
    ensure_bigfile();

    let mut group = c.benchmark_group("encode_decode_loop");
    group.sample_size(10);

    group.bench_function("encode+decode 50x", |b| {
        b.iter(|| {
            for _ in 0..50 {
                let mut input = File::open("bigfile.bin").unwrap();
                let mut encoded = Vec::new();
                encode_reader_to_writer(&mut input, &mut encoded, None).unwrap();

                let mut decoded = Vec::new();
                decode_reader_to_writer(&mut &encoded[..], &mut decoded).unwrap();
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_encode,
    bench_decode,
    bench_encode_file,
    bench_decode_file,
    bench_encode_mmap,
    bench_decode_mmap,
    bench_parallel,
    bench_encode_decode_roundtrip,
    bench_parallel_roundtrip,
    bench_scalar_vs_parallel,
    bench_url_safe,
    bench_async_streaming,
    bench_async_url_safe,
    bench_throughput_dashboard,
    bench_encode_decode_loop,
);

criterion_main!(benches);
