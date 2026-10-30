//! Numeric helpers recovered from the formatting runtime.

/// Translation of `FUN_140030bf0` (`mspaint.c:45526`).
///
/// Counts decimal digits, capped at nine exactly as in the source. This helper
/// is intended for a formatting chunk; ten-digit u32 values still return nine.
pub const fn decimal_chunk_digits(value: u32) -> i32 {
    if value > 99_999_999 {
        9
    } else if value > 9_999_999 {
        8
    } else if value > 999_999 {
        7
    } else if value > 99_999 {
        6
    } else if value > 9_999 {
        5
    } else if value > 999 {
        4
    } else if value > 99 {
        3
    } else {
        2 - (value < 10) as i32
    }
}

/// A checked replacement for the formatting helpers' unchecked output pointers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecimalWriteError {
    BufferTooSmall {
        required: usize,
        available: usize,
    },
    EmptyField,
    /// The recovered shifted-integer helper uses four 32-bit working words.
    IntegerOutOfRange,
}

impl std::fmt::Display for DecimalWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BufferTooSmall {
                required,
                available,
            } => {
                write!(
                    f,
                    "decimal output requires {required} bytes; got {available}"
                )
            }
            Self::EmptyField => f.write_str("decimal field must contain at least one byte"),
            Self::IntegerOutOfRange => {
                f.write_str("shifted integer exceeds the recovered 128-bit workspace")
            }
        }
    }
}

impl std::error::Error for DecimalWriteError {}

// DAT_14010c360 is referenced as a packed two-byte lookup for 00 through 99.
// Its initializer is absent from the decompilation. Compute those bytes rather
// than pretending the original table was recovered.
fn digit_pair(value: u32) -> [u8; 2] {
    debug_assert!(value < 100);
    [b'0' + (value / 10) as u8, b'0' + (value % 10) as u8]
}

fn require_output(destination: &[u8], required: usize) -> Result<(), DecimalWriteError> {
    if destination.len() < required {
        Err(DecimalWriteError::BufferTooSmall {
            required,
            available: destination.len(),
        })
    } else {
        Ok(())
    }
}

/// Translation of `FUN_14002f250` (`mspaint.c:44385`).
///
/// Write a fixed-width, zero-padded decimal field. If the value has more digits
/// than the field, only its least significant digits are written. Destination
/// length replaces the original width argument. No NUL terminator is appended.
pub fn write_decimal_field(
    destination: &mut [u8],
    mut value: u32,
) -> Result<(), DecimalWriteError> {
    if destination.is_empty() {
        return Err(DecimalWriteError::EmptyField);
    }
    let mut end = destination.len();
    while end >= 2 {
        let quotient = value / 100;
        destination[end - 2..end].copy_from_slice(&digit_pair(value - quotient * 100));
        value = quotient;
        end -= 2;
    }
    if end == 1 {
        destination[0] = b'0' + (value % 10) as u8;
    }
    Ok(())
}

/// Translation of `FUN_140030c60` (`mspaint.c:45555`).
///
/// Write a u32's decimal digits with no padding or terminator. The original's
/// digit-count argument is derived here, preventing inconsistent pointer writes.
/// Returns bytes written; insufficient space leaves the destination unchanged.
pub fn write_decimal(destination: &mut [u8], mut value: u32) -> Result<usize, DecimalWriteError> {
    let count = if value >= 1_000_000_000 {
        10
    } else {
        decimal_chunk_digits(value) as usize
    };
    require_output(destination, count)?;
    let mut end = count;
    while value >= 10_000 {
        let remainder = value % 10_000;
        value /= 10_000;
        destination[end - 2..end].copy_from_slice(&digit_pair(remainder % 100));
        destination[end - 4..end - 2].copy_from_slice(&digit_pair(remainder / 100));
        end -= 4;
    }
    if value >= 100 {
        destination[end - 2..end].copy_from_slice(&digit_pair(value % 100));
        value /= 100;
        end -= 2;
    }
    if value < 10 {
        destination[0] = b'0' + value as u8;
    } else {
        destination[end - 2..end].copy_from_slice(&digit_pair(value));
    }
    Ok(count)
}

/// Translation of `FUN_140030d70` (`mspaint.c:45598`).
///
/// Write exactly nine bytes. The source assumes a base-1e9 chunk, but for larger
/// u32 values its first byte is still `(value / 100000000) + '0'`, which can be
/// nonnumeric. This behavior is retained rather than silently truncating it.
pub fn write_nine_digit_chunk(
    destination: &mut [u8],
    mut value: u32,
) -> Result<usize, DecimalWriteError> {
    require_output(destination, 9)?;
    let mut end = 9;
    for _ in 0..2 {
        let remainder = value % 10_000;
        value /= 10_000;
        destination[end - 2..end].copy_from_slice(&digit_pair(remainder % 100));
        destination[end - 4..end - 2].copy_from_slice(&digit_pair(remainder / 100));
        end -= 4;
    }
    destination[0] = (value as u8).wrapping_add(b'0');
    Ok(9)
}

/// Translation of `FUN_14002f2f0` (`mspaint.c:44418`).
///
/// Write the decimal digits with a point after the first digit. Even a one-digit
/// input writes a trailing point. The original digit-count parameter is derived
/// from the value here. The destination is unchanged on insufficient space.
pub fn write_decimal_mantissa(
    destination: &mut [u8],
    value: u32,
) -> Result<usize, DecimalWriteError> {
    let mut digits = [0; 10];
    let count = write_decimal(&mut digits, value)?;
    require_output(destination, count + 1)?;
    destination[0] = digits[0];
    destination[1] = b'.';
    destination[2..count + 1].copy_from_slice(&digits[1..count]);
    Ok(count + 1)
}

/// Translation of `FUN_140030e10` (`mspaint.c:45628`).
///
/// Format the exact integer `significand * 2^binary_shift`. The source's caller
/// supplies a 24-bit significand and a shift up to 104; this implementation also
/// accepts other inputs if their product fits the recovered 128-bit workspace.
/// The arithmetic division into base-1e9 chunks replaces the decompiler's
/// reciprocal-multiplication optimization without floating-point rounding.
///
/// Returns bytes written, replacing the result pointer. An undersized output
/// returns an error (the C result's error code was 0x84) and leaves bytes intact.
pub fn write_shifted_integer(
    destination: &mut [u8],
    significand: u32,
    binary_shift: u32,
) -> Result<usize, DecimalWriteError> {
    if binary_shift >= 128 || u128::from(significand) > (u128::MAX >> binary_shift) {
        return Err(DecimalWriteError::IntegerOutOfRange);
    }
    let mut value = u128::from(significand) << binary_shift;
    let mut chunks = [0u32; 4];
    let mut count = 0;
    while value >= 1_000_000_000 {
        chunks[count] = (value % 1_000_000_000) as u32;
        value /= 1_000_000_000;
        count += 1;
    }
    let leading = decimal_chunk_digits(value as u32) as usize;
    let required = leading + count * 9;
    require_output(destination, required)?;
    let mut written = write_decimal(destination, value as u32)?;
    for chunk in chunks[..count].iter().rev() {
        written += write_nine_digit_chunk(&mut destination[written..], *chunk)?;
    }
    Ok(written)
}

/// The pseudocode shifts a 64-bit word by 64 when the masked count is zero.
/// That ambiguous case is excluded rather than assigned an invented result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AmbiguousProductShift;

impl std::fmt::Display for AmbiguousProductShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("product shift must have a nonzero low six-bit count")
    }
}

impl std::error::Error for AmbiguousProductShift {}

/// Translation of `FUN_14002f170` (`mspaint.c:44328`).
///
/// Multiply a scalar by three little-endian u64 words, extract shifted upper
/// product bits, then apply the original reciprocal arithmetic for a decimal
/// chunk. The signed byte shift retains the source's low-six-bit masking.
/// Returns the original int's raw 32-bit pattern, without a signed conversion.
/// The cached multiplier words must be supplied by the caller: their original
/// table contents are not present in the supplied decompilation.
pub fn multiply_shift_decimal_chunk(
    scalar: u64,
    multiplier: [u64; 3],
    shift: i8,
) -> Result<u32, AmbiguousProductShift> {
    let shift = (shift as u8 & 63) as u32;
    if shift == 0 {
        return Err(AmbiguousProductShift);
    }
    let first = u128::from(scalar) * u128::from(multiplier[0]);
    let middle = u128::from(scalar) * u128::from(multiplier[1]);
    let last = u128::from(scalar) * u128::from(multiplier[2]);
    let addend = ((last as u64 as u128) << 64) | (first >> 64);
    let upper_middle = (middle.wrapping_add(addend) >> 64) as u64;
    let upper = ((last >> 64) as u64).wrapping_add(u64::from(upper_middle < last as u64));
    let low = (upper_middle >> shift) | (upper << (64 - shift));
    let high = upper >> shift;

    let low_times_high = u128::from(low) * 0x8970_5f41_36b4_a597u128;
    let high_times_low = u128::from(high) * 0x3168_0a88_f895_3031u128;
    let low_times_low = u128::from(low) * 0x3168_0a88_f895_3031u128;
    let partial = (high_times_low as u64).wrapping_add((low_times_low >> 64) as u64);
    let quotient = high
        .wrapping_mul(0x8970_5f41_36b4_a597)
        .wrapping_add(u64::from(
            partial.wrapping_add(low_times_high as u64) < low_times_high as u64,
        ))
        .wrapping_add((high_times_low >> 64) as u64)
        .wrapping_add(u64::from(partial < high_times_low as u64))
        .wrapping_add((low_times_high >> 64) as u64)
        >> 29;
    Ok((low as u32).wrapping_sub((quotient as u32).wrapping_mul(1_000_000_000)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decimal_boundaries_and_cap() {
        assert_eq!(decimal_chunk_digits(0), 1);
        let mut boundary = 10;
        for digits in 2..=9 {
            assert_eq!(decimal_chunk_digits(boundary - 1), digits - 1);
            assert_eq!(decimal_chunk_digits(boundary), digits);
            boundary *= 10;
        }
        assert_eq!(decimal_chunk_digits(1_000_000_000), 9);
        assert_eq!(decimal_chunk_digits(u32::MAX), 9);
    }

    #[test]
    fn decimal_writers_match_independent_standard_formatting() {
        let mut value = 0u32;
        for _ in 0..10_000 {
            let expected = value.to_string();
            let mut output = [b'!'; 16];
            let count = write_decimal(&mut output, value).unwrap();
            assert_eq!(&output[..count], expected.as_bytes());
            assert!(output[count..].iter().all(|&byte| byte == b'!'));
            let count = write_decimal_mantissa(&mut output, value).unwrap();
            let mantissa = format!("{}.{}", &expected[..1], &expected[1..]);
            assert_eq!(&output[..count], mantissa.as_bytes());
            value = value.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        }
        for value in [
            0,
            9,
            10,
            99,
            100,
            9_999,
            10_000,
            99_999_999,
            100_000_000,
            1_000_000_000,
            u32::MAX,
        ] {
            let mut output = [0; 10];
            let count = write_decimal(&mut output, value).unwrap();
            assert_eq!(&output[..count], value.to_string().as_bytes());
        }
    }

    #[test]
    fn fixed_width_fields_pad_and_truncate() {
        for width in 1..=12 {
            for value in [0, 1, 12, 12345, 1_000_000_000, u32::MAX] {
                let expected = format!("{value:0width$}");
                let mut output = vec![0; width];
                write_decimal_field(&mut output, value).unwrap();
                assert_eq!(output, expected.as_bytes()[expected.len() - width..]);
            }
        }
        assert_eq!(
            write_decimal_field(&mut [], 1),
            Err(DecimalWriteError::EmptyField)
        );
    }

    #[test]
    fn nine_byte_chunks_preserve_large_input_behavior() {
        let mut output = [b'!'; 12];
        for value in [0, 1, 12345, 999_999_999] {
            assert_eq!(write_nine_digit_chunk(&mut output, value), Ok(9));
            assert_eq!(&output[..9], format!("{value:09}").as_bytes());
        }
        write_nine_digit_chunk(&mut output, u32::MAX).unwrap();
        assert_eq!(&output[..9], b"Z94967295");
        assert_eq!(&output[9..], b"!!!");
    }

    #[test]
    fn short_buffers_remain_unchanged() {
        let mut output = [b'!'; 2];
        assert!(matches!(
            write_decimal(&mut output, 100),
            Err(DecimalWriteError::BufferTooSmall {
                required: 3,
                available: 2
            })
        ));
        assert!(write_decimal_mantissa(&mut output, 12).is_err());
        assert!(write_nine_digit_chunk(&mut output, 0).is_err());
        assert!(write_shifted_integer(&mut output, 1, 127).is_err());
        assert_eq!(output, *b"!!");
    }

    #[test]
    fn shifted_integers_match_exact_u128_formatting() {
        // Exercise the full f32 significand/shift domain's boundaries and every
        // shift where bits cross a 32-bit workspace word or a decimal chunk.
        for shift in 0..=104 {
            for significand in [0, 1, 0x80_0000, 0xff_ffff] {
                let expected = (u128::from(significand) << shift).to_string();
                let mut output = [b'!'; 40];
                let count = write_shifted_integer(&mut output, significand, shift).unwrap();
                assert_eq!(&output[..count], expected.as_bytes());
                assert_eq!(output[count], b'!');
            }
        }
        let mut output = [0; 40];
        assert!(write_shifted_integer(&mut output, 1, 127).is_ok());
        assert_eq!(
            write_shifted_integer(&mut output, 2, 127),
            Err(DecimalWriteError::IntegerOutOfRange)
        );
        assert_eq!(
            write_shifted_integer(&mut output, 1, 128),
            Err(DecimalWriteError::IntegerOutOfRange)
        );
    }

    #[test]
    fn multiword_product_matches_schoolbook_multiply_and_remainder() {
        let mut seed = 0u64;
        for shift in 1..=63 {
            for _ in 0..128 {
                let mut words = [0; 4];
                for word in &mut words {
                    seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                    *word = seed;
                }
                let [scalar, a, b, c] = words;
                let mut carry = 0u128;
                let mut product = [0u64; 4];
                for (index, word) in [a, b, c].into_iter().enumerate() {
                    let partial = u128::from(scalar) * u128::from(word) + carry;
                    product[index] = partial as u64;
                    carry = partial >> 64;
                }
                product[3] = carry as u64;
                let upper = (u128::from(product[3]) << 64) | u128::from(product[2]);
                let expected = ((upper >> shift) % 1_000_000_000) as u32;
                assert_eq!(
                    multiply_shift_decimal_chunk(scalar, [a, b, c], shift),
                    Ok(expected)
                );
                assert_eq!(
                    multiply_shift_decimal_chunk(scalar, [a, b, c], shift - 64),
                    Ok(expected)
                );
            }
        }
        assert_eq!(
            multiply_shift_decimal_chunk(1, [0; 3], 0),
            Err(AmbiguousProductShift)
        );
        assert_eq!(
            multiply_shift_decimal_chunk(1, [0; 3], -64),
            Err(AmbiguousProductShift)
        );
    }
}
