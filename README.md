This crate prints primitive integers to slice or string in decimal format.

Compared to [standard format](https://doc.rust-lang.org/std/fmt/index.html),
this is much faster (about 5X).

Compared to [`itoa`](https://docs.rs/itoa) crate which prints to an internal
buffer, this crate prints to specified buffer directly. In some cases, this
avoids one memory copy which is as expensive as the conversion itself.

Compared to [`itoap`](https://docs.rs/itoap) crate which also prints to
specified buffer, this crate has same functionality and similar APIs.
They are different implementation algorithms, and this crate is slightly
faster.


# Usage

There are four APIs. Two of them are safe, while the other two are unsafe
because they do not check the length of specified slice or string, but
they are slightly faster.

Use [`write_to_slice`] to print to slice:

```rust
let mut buf: [u8; 10] = [0; 10];

let len = itoaaa::write_to_slice(1234, &mut buf).unwrap();
assert_eq!(str::from_utf8(&buf[..len]).unwrap(), "1234");
```

Use [`write_to_string`] to append to string:

```rust
let mut s = String::new();

itoaaa::write_to_string(1234, &mut s);
assert_eq!(s, "1234");
```

`write_to_slice` is `no-std/no-alloc` and slightly faster,
while `write_to_string` has a more convenient API.


# Benchmark

This presents the benchmark results for printing u64 values using three crates:
`itoa`, `itoap`, and `itoaaa`. The vertical axis represents the execution time
of each function, lower values indicate higher speed. The horizontal axis denotes
the number of decimal digits in the test integers.

![Benchmark result](https://raw.githubusercontent.com/WuBingzheng/itoaaa/refs/heads/main/benches/bench-u64.svg)

Overall, the three crates have similar performance. The execution times of `itoa`
and `itoaaa` show a nearly linear correlation with the number of digits. In
contrast, `itoap` exhibits performance spikes at 9 and 17 digits due to its
underlying implementation algorithm. Additionally, `itoaaa` has noticeable
advantage for small numbers with up to 3 decimal digits.

This result was tested under:

- OS: Ubuntu 22.04 LTS
- CPU: AMD EPYC 9754 128-Core Processor
- libc: Ubuntu GLIBC 2.35-0ubuntu3.8
- rustc: 1.93.0 (254b59607 2026-01-19)

The result highly depends on the environment. It is recommended that you run
the tests in your own environment:

```bash
git clone https://github.com/WuBingzheng/itoaaa.git
cd itoaaa
cargo bench
open target/criterion/reports/index.html # or use your browser to open this
```

By running the tests yourself, you can see more results, about more integer
types and more APIs.

# License

MIT
