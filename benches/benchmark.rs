use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::fmt::Write;
use std::hint::black_box;

fn bench_encode<T>(c: &mut Criterion, inputs: &[T], name: &str)
where
    T: std::fmt::Display + itoa::Integer + itoaaa::Integer + itoap::Integer,
{
    let mut group = c.benchmark_group(name);

    let mut s = String::with_capacity(100);
    let mut v = Vec::new();
    v.resize(100, 0);

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
                black_box(buffer.format(*n));
            })
        });

        group.bench_with_input(BenchmarkId::new("itoap::write_to_ptr()", i), n, |b, n| {
            b.iter(|| {
                let pos = unsafe { itoap::write_to_ptr(v.as_mut_ptr() as *mut u8, *n) };
                black_box(&v[..pos]);
            })
        });

        group.bench_with_input(
            BenchmarkId::new("itoap::write_to_string()", i),
            n,
            |b, n| {
                b.iter(|| {
                    s.clear();
                    itoap::write_to_string(&mut s, *n);
                    black_box(&s);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("itoaaa::write_to_slice()", i),
            n,
            |b, n| {
                b.iter(|| {
                    let pos = itoaaa::write_to_slice(*n, &mut v).unwrap();
                    black_box(&v[..pos]);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("itoaaa::write_to_string()", i),
            n,
            |b, n| {
                b.iter(|| {
                    s.clear();
                    itoaaa::write_to_string(*n, &mut s);
                    black_box(&s);
                })
            },
        );
    }

    // done
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    let inputs = vec![
        0_i128,
        1,
        12,
        123,
        1234,
        12345,
        123456,
        1234567,
        12345678,
        123456789,
        1234567890,
        12345678901,
        123456789012,
        1234567890123,
        12345678901234,
        123456789012345,
        1234567890123456,
        12345678901234567,
        123456789012345678,
        1234567890123456789,
        12345678901234567890,
        123456789012345678901,
        1234567890123456789012,
        12345678901234567890123,
        123456789012345678901234,
        1234567890123456789012345,
        12345678901234567890123456,
        123456789012345678901234567,
        1234567890123456789012345678,
        12345678901234567890123456789,
        123456789012345678901234567890,
        1234567890123456789012345678901,
        12345678901234567890123456789012,
        123456789012345678901234567890123,
        1234567890123456789012345678901234,
        12345678901234567890123456789012345,
        123456789012345678901234567890123456,
        1234567890123456789012345678901234568,
        12345678901234567890123456789012345678,
    ];

    let inputs_u32: Vec<u32> = inputs[..11].iter().map(|x| *x as u32).collect();
    bench_encode(c, &inputs_u32, "u32");

    let inputs_i32: Vec<i32> = inputs[..11].iter().map(|x| -(*x as i32)).collect();
    bench_encode(c, &inputs_i32, "i32");

    let inputs_u64: Vec<u64> = inputs[..21].iter().map(|x| *x as u64).collect();
    bench_encode(c, &inputs_u64, "u64");

    let inputs_i64: Vec<i64> = inputs[..20].iter().map(|x| -(*x as i64)).collect();
    bench_encode(c, &inputs_i64, "i64");

    bench_encode(c, &inputs, "u128");

    let inputs_i128: Vec<i128> = inputs.iter().map(|x| -(*x as i128)).collect();
    bench_encode(c, &inputs_i128, "i128");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
