use b64::{
    decode_parallel, decode_reader_to_writer, decode_to_vec, encode_parallel,
    encode_reader_to_writer, encode_to_string,
};
use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use memmap2::Mmap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

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

    // Pre‑encode once
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
    bench_encode_decode_loop,
);

criterion_main!(benches);
