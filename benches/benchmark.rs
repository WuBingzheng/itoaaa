use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::fmt::Write;
use std::hint::black_box;

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");

    let inputs = vec![
        0x0_u64,
        0x1,
        0x1f,
        0x12f,
        0x1f2f,
        0x1f2f3,
        0x1f2f3f,
        0x1f2f3f4,
        0x1f2f3f4f,
        0x1f2f3f4f5,
        0x1f2f3f4f5f,
        0x1f2f3f4f5f6,
        0x1f2f3f4f5f6f,
        0x1f2f3f4f5f6f7,
        0x1f2f3f4f5f6f7f,
        0x1f2f3f4f5f6f7f8,
        0x1f2f3f4f5f6f7f8f,
    ];

    let mut s = String::with_capacity(100);
    let mut buf: [u8; 100] = [0; 100];

    for (i, n) in inputs.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("std", i), n, |b, n| {
            b.iter(|| {
                s.clear();
                write!(&mut s, "{}", *n).unwrap();
                black_box(&s);
            })
        });

        group.bench_with_input(BenchmarkId::new("itoa", i), n, |b, n| {
            b.iter(|| {
                let mut buffer = itoa::Buffer::new();
                black_box(buffer.format(*n as u64));
            })
        });

        group.bench_with_input(BenchmarkId::new("itoa + memcpy", i), n, |b, n| {
            b.iter(|| {
                let mut buffer = itoa::Buffer::new();
                let s = buffer.format(*n);
                buf[..s.len()].copy_from_slice(s.as_bytes());
                black_box(buf);
            })
        });

        group.bench_with_input(BenchmarkId::new("itoa-slice", i), n, |b, n| {
            b.iter(|| {
                let pos = itoa_slice::dump(*n, &mut buf).unwrap();
                black_box(&buf[..pos]);
            })
        });

        group.bench_with_input(BenchmarkId::new("itoa-slice: to_string", i), n, |b, n| {
            b.iter(|| {
                s.clear();
                itoa_slice::dump_to_string(*n, &mut s);
                black_box(&s);
            })
        });
    }

    // done
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_encode(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
