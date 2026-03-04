use core::mem::MaybeUninit;

#[inline(always)]
pub fn dump(n: u64, buf: &mut [u8]) -> Result<usize, usize> {
    let len = dump_len(n);
    if buf.len() < len {
        return Err(len);
    }

    unsafe {
        // SAFETY: `buf[..len]` fits.
        unchecked_dump(n, &mut buf[..len]);
    }
    Ok(len)
}

#[inline(always)]
pub fn dump_to_uninit(n: u64, buf: &mut [MaybeUninit<u8>]) -> Result<usize, usize> {
    // SAFETY: only write the buf, but no read.
    let buf = unsafe { buf.assume_init_mut() };
    dump(n, buf)
}

#[inline(always)]
pub fn dump_to_string(n: u64, s: &mut String) {
    let dump_len = dump_len(n);

    // SAFETY: all valid charactors.
    // SAFETY: `v[origin_len..]` fits.
    unsafe {
        let v = s.as_mut_vec();
        let origin_len = v.len();
        v.reserve(dump_len);
        v.set_len(origin_len + dump_len);
        unchecked_dump(n, &mut v[origin_len..]);
    }
}

// SAFETY: The caller must make sure the `buf` just fits the dumped
// string, no more no less.
#[inline(always)]
unsafe fn unchecked_dump(n: u64, buf: &mut [u8]) {
    let mut offset = buf.len();
    let mut remain = n;

    // Format per two digits from the lookup table.
    while remain > 999 {
        offset -= 4;

        // pull two pairs
        let quad = remain as usize % 10000;
        remain /= 10000;

        let pair1 = quad / 100;
        let pair2 = quad % 100;
        unsafe {
            *buf.get_unchecked_mut(offset) = digit(pair1 * 2);
            *buf.get_unchecked_mut(offset + 1) = digit(pair1 * 2 + 1);
            *buf.get_unchecked_mut(offset + 2) = digit(pair2 * 2);
            *buf.get_unchecked_mut(offset + 3) = digit(pair2 * 2 + 1);
        }
    }
    if remain >= 10 {
        offset -= 2;

        let pair = remain as usize % 100;
        remain = remain / 100;
        unsafe {
            *buf.get_unchecked_mut(offset) = digit(pair * 2);
            *buf.get_unchecked_mut(offset + 1) = digit(pair * 2 + 1);
        }
    }

    // Format the last remaining digit, if any.
    if remain != 0 || n == 0 {
        offset -= 1;
        unsafe {
            *buf.get_unchecked_mut(offset) = b'0' + remain as u8;
        }
    }

    debug_assert_eq!(offset, 0);
}

static DECIMAL_PAIRS: &[u8; 200] = b"\
        0001020304050607080910111213141516171819\
        2021222324252627282930313233343536373839\
        4041424344454647484950515253545556575859\
        6061626364656667686970717273747576777879\
        8081828384858687888990919293949596979899";
#[inline(always)]
fn digit(p: usize) -> u8 {
    unsafe { *DECIMAL_PAIRS.get_unchecked(p) }
}

struct BitDigit(u64, usize);

const BITDIGIT: [BitDigit; 65] = [
    BitDigit(10_u64.pow(19), 20),
    BitDigit(0, 19),
    BitDigit(0, 19),
    BitDigit(0, 19),
    BitDigit(10_u64.pow(18), 19),
    BitDigit(0, 18),
    BitDigit(0, 18),
    BitDigit(10_u64.pow(17), 18),
    BitDigit(0, 17),
    BitDigit(0, 17),
    BitDigit(10_u64.pow(16), 17),
    BitDigit(0, 16),
    BitDigit(0, 16),
    BitDigit(0, 16),
    BitDigit(10_u64.pow(15), 16),
    BitDigit(0, 15),
    BitDigit(0, 15),
    BitDigit(10_u64.pow(14), 15),
    BitDigit(0, 14),
    BitDigit(0, 14),
    BitDigit(10_u64.pow(13), 14),
    BitDigit(0, 13),
    BitDigit(0, 13),
    BitDigit(0, 13),
    BitDigit(10_u64.pow(12), 13),
    BitDigit(0, 12),
    BitDigit(0, 12),
    BitDigit(10_u64.pow(11), 12),
    BitDigit(0, 11),
    BitDigit(0, 11),
    BitDigit(10_u64.pow(10), 11),
    BitDigit(0, 10),
    BitDigit(0, 10),
    BitDigit(0, 10),
    BitDigit(10_u64.pow(9), 10),
    BitDigit(0, 9),
    BitDigit(0, 9),
    BitDigit(10_u64.pow(8), 9),
    BitDigit(0, 8),
    BitDigit(0, 8),
    BitDigit(10_u64.pow(7), 8),
    BitDigit(0, 7),
    BitDigit(0, 7),
    BitDigit(0, 7),
    BitDigit(10_u64.pow(6), 7),
    BitDigit(0, 6),
    BitDigit(0, 6),
    BitDigit(10_u64.pow(5), 6),
    BitDigit(0, 5),
    BitDigit(0, 5),
    BitDigit(10_u64.pow(4), 5),
    BitDigit(0, 4),
    BitDigit(0, 4),
    BitDigit(0, 4),
    BitDigit(10_u64.pow(3), 4),
    BitDigit(0, 3),
    BitDigit(0, 3),
    BitDigit(10_u64.pow(2), 3),
    BitDigit(0, 2),
    BitDigit(0, 2),
    BitDigit(10_u64.pow(1), 2),
    BitDigit(0, 1),
    BitDigit(0, 1),
    BitDigit(0, 1),
    BitDigit(0, 1),
];

fn dump_len(n: u64) -> usize {
    let zeros = n.leading_zeros();
    let bd = unsafe { BITDIGIT.get_unchecked(zeros as usize) };
    if n >= bd.0 { bd.1 } else { bd.1 - 1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(n: u64) {
        let mut buf: [u8; 20] = [0; 20];
        let pos = dump(n, &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf[..pos]).unwrap(), format!("{n}"));
    }

    #[test]
    fn it_works() {
        test(0);
        test(1);
        test(12);
        test(123);
        test(1234);
        test(12345);
        test(123456);
        test(1234567);
        test(12345678);
        test(123456789);
        test(1234567890);
        test(12345678901);
        test(123456789012);
        test(1234567890123);
        test(12345678901234);
        test(123456789012345);
        test(1234567890123456);
        test(12345678901234567);
        test(123456789012345678);
        test(1234567890123456789);
        test(12345678901234567890);
        test(9);
        test(92);
        test(923);
        test(9234);
        test(92345);
        test(923456);
        test(9234567);
        test(92345678);
        test(923456789);
        test(9234567890);
        test(92345678901);
        test(923456789012);
        test(9234567890123);
        test(92345678901234);
        test(923456789012345);
        test(9234567890123456);
        test(92345678901234567);
        test(923456789012345678);
        test(9234567890123456789);
    }
}
