pub fn dump(n: impl Integer, buf: &mut [u8]) -> Result<usize, usize> {
    let (neg, abs) = n.unsigned_abs();
    let len = neg + abs.dump_len();
    if buf.len() < len {
        return Err(len);
    }

    // SAFETY: buf[neg..len].len() == n.dump_len()
    unsafe {
        if neg != 0 {
            *buf.get_unchecked_mut(0) = b'-';
        }
        abs.unchecked_dump(&mut buf[neg..len]);
    }
    Ok(len)
}

pub fn dump_to_string(n: impl Integer, s: &mut String) {
    let (neg, abs) = n.unsigned_abs();
    let dump_len = neg + abs.dump_len();

    // SAFETY: digits are valid charactors
    // SAFETY: v[neg+origin_len..].len() == n.dump_len()
    unsafe {
        let v = s.as_mut_vec();
        let origin_len = v.len();
        v.reserve(dump_len);
        v.set_len(origin_len + dump_len);
        if neg != 0 {
            *v.get_unchecked_mut(origin_len) = b'-';
        }
        abs.unchecked_dump(&mut v[neg + origin_len..]);
    }
}

pub trait Unsigned: Copy {
    fn dump_len(self) -> usize;
    unsafe fn unchecked_dump(self, buf: &mut [u8]);
}

pub trait Integer {
    type Unsigned: Unsigned;
    fn unsigned_abs(self) -> (usize, Self::Unsigned);
}

macro_rules! impl_integer {
    ($signed:ty, $unsigned:ty) => {
        impl Integer for $signed {
            type Unsigned = $unsigned;
            fn unsigned_abs(self) -> (usize, Self::Unsigned) {
                if self < 0 {
                    (1, (!self).wrapping_add(1) as $unsigned)
                } else {
                    (0, self as $unsigned)
                }
            }
        }

        impl Integer for $unsigned {
            type Unsigned = $unsigned;
            fn unsigned_abs(self) -> (usize, Self::Unsigned) {
                (0, self)
            }
        }
    };
}

impl_integer!(i16, u16);
impl_integer!(i32, u32);
impl_integer!(i64, u64);

// general implements for u16, u32, and u64
macro_rules! impl_unsigned {
    ($ty:ty, $bit_digits:expr) => {
        impl Unsigned for $ty {
            fn dump_len(self) -> usize {
                let zeros = self.leading_zeros();
                let bd = unsafe { $bit_digits.get_unchecked(zeros as usize) };
                if self >= bd.0 { bd.1 } else { bd.1 - 1 }
            }

            // SAFETY: The caller must make sure: buf.len() == self.dump_len()
            #[inline(always)]
            unsafe fn unchecked_dump(self: $ty, buf: &mut [u8]) {
                let mut offset = buf.len();
                let mut remain = self;

                // Format per two digits from the lookup table.
                while remain >= 1000 {
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
                if remain != 0 || self == 0 {
                    offset -= 1;
                    unsafe {
                        *buf.get_unchecked_mut(offset) = b'0' + remain as u8;
                    }
                }

                debug_assert_eq!(offset, 0);
            }
        }
    };
}

impl_unsigned!(u64, BITDIGIT_U64);
impl_unsigned!(u32, BITDIGIT_U32);
impl_unsigned!(u16, BITDIGIT_U16);

struct BitDigit<T: PartialOrd>(T, usize);

const BITDIGIT_U16: [BitDigit<u16>; 17] = [
    BitDigit(0, 5),
    BitDigit(0, 5),
    BitDigit(10_u16.pow(4), 5),
    BitDigit(0, 4),
    BitDigit(0, 4),
    BitDigit(0, 4),
    BitDigit(10_u16.pow(3), 4),
    BitDigit(0, 3),
    BitDigit(0, 3),
    BitDigit(10_u16.pow(2), 3),
    BitDigit(0, 2),
    BitDigit(0, 2),
    BitDigit(10_u16.pow(1), 2),
    BitDigit(0, 1),
    BitDigit(0, 1),
    BitDigit(0, 1),
    BitDigit(0, 1),
];

const BITDIGIT_U32: [BitDigit<u32>; 33] = [
    BitDigit(0, 10),
    BitDigit(0, 10),
    BitDigit(10_u32.pow(9), 10),
    BitDigit(0, 9),
    BitDigit(0, 9),
    BitDigit(10_u32.pow(8), 9),
    BitDigit(0, 8),
    BitDigit(0, 8),
    BitDigit(10_u32.pow(7), 8),
    BitDigit(0, 7),
    BitDigit(0, 7),
    BitDigit(0, 7),
    BitDigit(10_u32.pow(6), 7),
    BitDigit(0, 6),
    BitDigit(0, 6),
    BitDigit(10_u32.pow(5), 6),
    BitDigit(0, 5),
    BitDigit(0, 5),
    BitDigit(10_u32.pow(4), 5),
    BitDigit(0, 4),
    BitDigit(0, 4),
    BitDigit(0, 4),
    BitDigit(10_u32.pow(3), 4),
    BitDigit(0, 3),
    BitDigit(0, 3),
    BitDigit(10_u32.pow(2), 3),
    BitDigit(0, 2),
    BitDigit(0, 2),
    BitDigit(10_u32.pow(1), 2),
    BitDigit(0, 1),
    BitDigit(0, 1),
    BitDigit(0, 1),
    BitDigit(0, 1),
];

const BITDIGIT_U64: [BitDigit<u64>; 65] = [
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

// SAFETY: caller must make sure: p < 200
#[inline(always)]
unsafe fn digit(p: usize) -> u8 {
    static DECIMAL_PAIRS: &[u8; 200] = b"\
        0001020304050607080910111213141516171819\
        2021222324252627282930313233343536373839\
        4041424344454647484950515253545556575859\
        6061626364656667686970717273747576777879\
        8081828384858687888990919293949596979899";

    unsafe { *DECIMAL_PAIRS.get_unchecked(p) }
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
