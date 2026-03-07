#![doc = include_str!("../README.md")]

/// Write the integer to the slice in decimal format.
///
/// Return `Ok` if the slice is long enough, containing the length
/// of formatted string. Return `Err` otherwise, containing the
/// needed length too.
///
/// # Examples:
///
/// ```
/// let mut buf: [u8; 100] = [0; 100];
/// let len = itoaaa::write_to_slice(1234, &mut buf).unwrap();
/// assert_eq!(str::from_utf8(&buf[..len]).unwrap(), "1234");
/// ```
#[inline]
pub fn write_to_slice(n: impl Integer, buf: &mut [u8]) -> Result<usize, usize> {
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

/// Write the integer to the slice in decimal format, without checking
/// the length of slice.
///
/// Return the length of formatted string.
///
/// # Safety:
///
/// You must make sure the length of slice is enough.
#[inline]
pub unsafe fn unchecked_write_to_slice(n: impl Integer, buf: &mut [u8]) -> usize {
    let (neg, abs) = n.unsigned_abs();
    let len = neg + abs.dump_len();

    // SAFETY: buf[neg..len].len() == n.dump_len()
    unsafe {
        if neg != 0 {
            *buf.get_unchecked_mut(0) = b'-';
        }
        abs.unchecked_dump(&mut buf[neg..len]);
    }
    len
}

/// Append the integer to the string in decimal format.
///
/// # Examples:
///
/// ```
/// let mut s = String::new();
/// itoaaa::write_to_string(1234, &mut s);
/// assert_eq!(s, "1234");
/// ```
#[inline]
pub fn write_to_string(n: impl Integer, s: &mut String) {
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

/// Append the integer to the string in decimal format, without
/// checking the capacity of string.
///
/// # Safety:
///
/// You must make sure the string have enough available capacity.
#[inline]
pub unsafe fn unchecked_write_to_string(n: impl Integer, s: &mut String) {
    let (neg, abs) = n.unsigned_abs();
    let dump_len = neg + abs.dump_len();

    // SAFETY: digits are valid charactors
    // SAFETY: v[neg+origin_len..].len() == n.dump_len()
    unsafe {
        let v = s.as_mut_vec();
        let origin_len = v.len();
        v.set_len(origin_len + dump_len);
        if neg != 0 {
            *v.get_unchecked_mut(origin_len) = b'-';
        }
        abs.unchecked_dump(&mut v[neg + origin_len..]);
    }
}

// === trait Integer ===
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

impl_integer!(i8, u8);
impl_integer!(i16, u16);
impl_integer!(i32, u32);
impl_integer!(i64, u64);
impl_integer!(i128, u128);

// === trait Unsigned ===
pub trait Unsigned: Copy {
    fn dump_len(self) -> usize;
    unsafe fn unchecked_dump(self, buf: &mut [u8]);
}

// general implements for u16, u32, and u64
macro_rules! impl_unsigned {
    ($ty:ty, $bit_digits:expr) => {
        impl Unsigned for $ty {
            #[inline]
            fn dump_len(self) -> usize {
                let t = (<$ty>::BITS - (self | 1).leading_zeros()) as usize * 1233 >> 12;
                t + (self >= $bit_digits[t]) as usize
            }

            // SAFETY: The caller must make sure: buf.len() == self.dump_len()
            #[inline]
            #[allow(overflowing_literals)]
            #[allow(unused_comparisons)]
            unsafe fn unchecked_dump(self: $ty, buf: &mut [u8]) {
                let mut offset = buf.len();
                let mut remain = self;

                // Format per two digits from the lookup table.
                while core::mem::size_of::<Self>() > 1 && remain >= 1000 {
                    offset -= 4;

                    // pull two pairs
                    let quad = (remain % 10000) as usize;
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

impl_unsigned!(u8, POWERS_U8);
impl_unsigned!(u16, POWERS_U16);
impl_unsigned!(u32, POWERS_U32);
impl_unsigned!(u64, POWERS_U64);

impl Unsigned for u128 {
    #[inline]
    fn dump_len(self) -> usize {
        let t = (128 - (self | 1).leading_zeros()) as usize * 1233 >> 12;
        t + (self >= POWERS_U128[t]) as usize
    }

    #[inline]
    unsafe fn unchecked_dump(self, buf: &mut [u8]) {
        let mut offset = buf.len();
        let mut remain = self;

        loop {
            if let Ok(n64) = u64::try_from(remain) {
                unsafe { n64.unchecked_dump(&mut buf[..offset]) };
                return;
            }

            // Take the 16 least-significant decimals.
            let (quot_1e16, mut mod_1e16) = div_rem_1e16(remain);

            for _ in 0..4 {
                offset -= 4;

                // pull two pairs
                let quad = (mod_1e16 % 1_00_00) as usize;
                mod_1e16 /= 1_00_00;

                let pair1 = quad / 100;
                let pair2 = quad % 100;
                unsafe {
                    *buf.get_unchecked_mut(offset) = digit(pair1 * 2);
                    *buf.get_unchecked_mut(offset + 1) = digit(pair1 * 2 + 1);
                    *buf.get_unchecked_mut(offset + 2) = digit(pair2 * 2);
                    *buf.get_unchecked_mut(offset + 3) = digit(pair2 * 2 + 1);
                }
            }

            remain = quot_1e16;
        }
    }
}

// === powers ===
const POWERS_U8: [u8; 3] = [1, 10_u8.pow(1), 10_u8.pow(2)];
const POWERS_U16: [u16; 5] = [
    0,
    10_u16.pow(1),
    10_u16.pow(2),
    10_u16.pow(3),
    10_u16.pow(4),
];
const POWERS_U32: [u32; 10] = [
    0,
    10_u32.pow(1),
    10_u32.pow(2),
    10_u32.pow(3),
    10_u32.pow(4),
    10_u32.pow(5),
    10_u32.pow(6),
    10_u32.pow(7),
    10_u32.pow(8),
    10_u32.pow(9),
];
const POWERS_U64: [u64; 20] = [
    0,
    10_u64.pow(1),
    10_u64.pow(2),
    10_u64.pow(3),
    10_u64.pow(4),
    10_u64.pow(5),
    10_u64.pow(6),
    10_u64.pow(7),
    10_u64.pow(8),
    10_u64.pow(9),
    10_u64.pow(10),
    10_u64.pow(11),
    10_u64.pow(12),
    10_u64.pow(13),
    10_u64.pow(14),
    10_u64.pow(15),
    10_u64.pow(16),
    10_u64.pow(17),
    10_u64.pow(18),
    10_u64.pow(19),
];
const POWERS_U128: [u128; 39] = [
    0,
    10_u128.pow(1),
    10_u128.pow(2),
    10_u128.pow(3),
    10_u128.pow(4),
    10_u128.pow(5),
    10_u128.pow(6),
    10_u128.pow(7),
    10_u128.pow(8),
    10_u128.pow(9),
    10_u128.pow(10),
    10_u128.pow(11),
    10_u128.pow(12),
    10_u128.pow(13),
    10_u128.pow(14),
    10_u128.pow(15),
    10_u128.pow(16),
    10_u128.pow(17),
    10_u128.pow(18),
    10_u128.pow(19),
    10_u128.pow(20),
    10_u128.pow(21),
    10_u128.pow(22),
    10_u128.pow(23),
    10_u128.pow(24),
    10_u128.pow(25),
    10_u128.pow(26),
    10_u128.pow(27),
    10_u128.pow(28),
    10_u128.pow(29),
    10_u128.pow(20),
    10_u128.pow(31),
    10_u128.pow(32),
    10_u128.pow(33),
    10_u128.pow(34),
    10_u128.pow(35),
    10_u128.pow(36),
    10_u128.pow(37),
    10_u128.pow(38),
];

// SAFETY: caller must make sure: p < 200
#[inline]
unsafe fn digit(p: usize) -> u8 {
    static DECIMAL_PAIRS: &[u8; 200] = b"\
        0001020304050607080910111213141516171819\
        2021222324252627282930313233343536373839\
        4041424344454647484950515253545556575859\
        6061626364656667686970717273747576777879\
        8081828384858687888990919293949596979899";

    unsafe { *DECIMAL_PAIRS.get_unchecked(p) }
}

// === u128 utils ===

fn div_rem_1e16(n: u128) -> (u128, u64) {
    const D: u128 = 1_0000_0000_0000_0000;

    // These constant values are computed with the CHOOSE_MULTIPLIER procedure
    // from the Granlund & Montgomery paper, using N=128, prec=128 and d=1E16.
    const M_HIGH: u128 = 76624777043294442917917351357515459181;
    const SH_POST: u8 = 51;

    // n.widening_mul(M_HIGH).1 >> SH_POST
    let quot = u128_mulhi(n, M_HIGH) >> SH_POST;
    let rem = n - quot * D;
    (quot, rem as u64)
}

fn u128_mulhi(x: u128, y: u128) -> u128 {
    let x_lo = x as u64;
    let x_hi = (x >> 64) as u64;
    let y_lo = y as u64;
    let y_hi = (y >> 64) as u64;

    // handle possibility of overflow
    let carry = (u128::from(x_lo) * u128::from(y_lo)) >> 64;
    let m = u128::from(x_lo) * u128::from(y_hi) + carry;
    let high1 = m >> 64;

    let m_lo = m as u64;
    let high2 = (u128::from(x_hi) * u128::from(y_lo) + u128::from(m_lo)) >> 64;

    u128::from(x_hi) * u128::from(y_hi) + high1 + high2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test<T>(n: T)
    where
        T: Integer + Copy + std::fmt::Display + std::fmt::Debug,
    {
        let mut buf: [u8; 40] = [0; 40];
        let pos = write_to_slice(n, &mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf[..pos]).unwrap(), format!("{n}"));
    }

    #[test]
    fn it_works() {
        test(0);
        test(i64::MIN);
        test(i64::MAX);
        test(-i64::MAX);
        test(i128::MIN);
        test(i128::MAX);
        test(-i128::MAX);

        let mut n = u8::MAX;
        while n != 0 {
            test(n);
            if let Ok(i) = i8::try_from(n) {
                test(i);
                test(-i);
            }
            n /= 10;
        }

        let mut n = u64::MAX;
        while n != 0 {
            test(n);
            if let Ok(i) = i64::try_from(n) {
                test(i);
                test(-i);
            }
            n /= 10;
        }

        let mut n = u128::MAX;
        while n != 0 {
            test(n);
            if let Ok(i) = i128::try_from(n) {
                test(i);
                test(-i);
            }
            n /= 10;
        }
    }
}
