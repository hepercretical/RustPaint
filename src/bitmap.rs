//! DIB allocation sizing recovered from `FUN_140024720`.
//!
//! The input is the DIB header, not the 14-byte BMP file header. Reads are
//! little-endian and bounds checked, replacing the original unchecked casts.

/// A recognized header did not contain all of its declared header bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TruncatedHeader {
    pub required: usize,
    pub available: usize,
}

impl std::fmt::Display for TruncatedHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DIB header requires {} bytes; got {}",
            self.required, self.available
        )
    }
}

impl std::error::Error for TruncatedHeader {}

/// Translation of `FUN_140024720` (`mspaint.c:35766`).
///
/// Returns zero for unsupported header sizes or compression without an explicit
/// image size, as the source does. Arithmetic intentionally wraps at 32 bits.
/// This reproduces the source's size calculation; it does not validate a bitmap
/// or guarantee that the returned size is safe for allocation.
pub fn dib_size(header: &[u8]) -> Result<u32, TruncatedHeader> {
    require(header, 4)?;
    let header_size = read_u32(header, 0);
    if !matches!(header_size, 12 | 40 | 108 | 124) {
        return Ok(0);
    }
    require(header, header_size as usize)?;

    if header_size == 12 {
        let width = u32::from(read_u16(header, 4));
        let height = u32::from(read_u16(header, 6));
        let bits = u32::from(read_u16(header, 10));
        // Preserve the decompilation's unusual '+ 4', not an assumed DIB rule.
        let prefix = if bits < 9 {
            ((1u32 << bits) + 4) * 3
        } else {
            12
        };
        return Ok(prefix.wrapping_add(stride(width, bits).wrapping_mul(height)));
    }

    let width = read_u32(header, 4);
    let height = read_u32(header, 8);
    let bits = u32::from(read_u16(header, 14));
    let compression = read_u32(header, 16);
    let explicit_size = read_u32(header, 20);
    let colors_used = read_u32(header, 32);
    let palette_entries = if colors_used == 0 && bits <= 8 {
        1u32 << bits
    } else {
        colors_used
    };
    let prefix = header_size.wrapping_add(palette_entries.wrapping_mul(4));
    let pixels = if explicit_size != 0 {
        explicit_size
    } else {
        if compression != 0 && compression != 3 {
            return Ok(0);
        }
        let negated_height = height.wrapping_neg();
        let absolute_height = if (negated_height as i32) < 0 {
            height
        } else {
            negated_height
        };
        absolute_height.wrapping_mul(stride(width, bits))
    };
    let total = prefix.wrapping_add(pixels);
    Ok(match header_size {
        40 if compression == 3 => total.wrapping_add(12),
        124 => total
            .max(read_u32(header, 112))
            .wrapping_add(read_u32(header, 116)),
        _ => total,
    })
}

/// Failure when inferring missing DIB dimensions from a supplied buffer size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferHeightError {
    Truncated(TruncatedHeader),
    ZeroStride,
}

impl From<TruncatedHeader> for InferHeightError {
    fn from(error: TruncatedHeader) -> Self {
        Self::Truncated(error)
    }
}

impl std::fmt::Display for InferHeightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated(error) => error.fmt(f),
            Self::ZeroStride => f.write_str("cannot infer DIB height with a zero row stride"),
        }
    }
}

impl std::error::Error for InferHeightError {}

/// Translation of `FUN_140024810` (`mspaint.c:35831`).
///
/// Fill a zero height using the caller's DIB buffer size. Modern headers get a
/// negative (top-down) height and, when absent, an inferred image size. Existing
/// heights and unsupported header sizes are unchanged. The buffer size retains
/// the original signed 32-bit parameter's bit pattern and wrapping arithmetic.
///
/// Unlike the raw-pointer source, truncated headers and division by zero return
/// errors. On error no bytes are modified. Extra buffer bytes are untouched.
pub fn infer_missing_height(header: &mut [u8], buffer_size: i32) -> Result<(), InferHeightError> {
    require(header, 4)?;
    let size = read_u32(header, 0);
    if !matches!(size, 12 | 40 | 108 | 124) {
        return Ok(());
    }
    require(header, size as usize)?;
    if size == 12 {
        if read_u16(header, 6) != 0 {
            return Ok(());
        }
        let bits = u32::from(read_u16(header, 10));
        let mut pixels = (buffer_size as u32).wrapping_sub(12);
        if bits < 9 {
            pixels = pixels.wrapping_sub((1u32 << bits) * 3);
        }
        let row = stride(u32::from(read_u16(header, 4)), bits);
        if row == 0 {
            return Err(InferHeightError::ZeroStride);
        }
        let height = (pixels / row) as u16;
        header[6..8].copy_from_slice(&height.to_le_bytes());
        return Ok(());
    }
    if read_u32(header, 8) != 0 {
        return Ok(());
    }
    let bits = u32::from(read_u16(header, 14));
    let row = stride(read_u32(header, 4), bits);
    if row == 0 {
        return Err(InferHeightError::ZeroStride);
    }
    let mut pixels = read_u32(header, 20);
    if pixels == 0 {
        pixels = (buffer_size as u32).wrapping_sub(size);
        let colors = read_u32(header, 32);
        if colors != 0 {
            pixels = pixels.wrapping_sub(colors.wrapping_mul(4));
        } else if bits < 9 {
            pixels = pixels.wrapping_sub((1u32 << bits) * 4);
        }
        if size == 40 && read_u32(header, 16) == 3 {
            pixels = pixels.wrapping_sub(12);
        } else if size == 124 {
            // The source subtracts profile size only; it ignores profile offset.
            pixels = pixels.wrapping_sub(read_u32(header, 116));
        }
        header[20..24].copy_from_slice(&pixels.to_le_bytes());
    }
    header[8..12].copy_from_slice(&(pixels / row).wrapping_neg().to_le_bytes());
    Ok(())
}

fn require(bytes: &[u8], required: usize) -> Result<(), TruncatedHeader> {
    if bytes.len() < required {
        Err(TruncatedHeader {
            required,
            available: bytes.len(),
        })
    } else {
        Ok(())
    }
}

fn read_u16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([bytes[offset], bytes[offset + 1]])
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn stride(width: u32, bits: u32) -> u32 {
    (width.wrapping_mul(bits).wrapping_add(31) >> 3) & 0x1fff_fffc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn info_header(size: u32, width: u32, height: i32, bits: u16) -> Vec<u8> {
        let mut bytes = vec![0; size as usize];
        put32(&mut bytes, 0, size);
        put32(&mut bytes, 4, width);
        put32(&mut bytes, 8, height as u32);
        bytes[12..14].copy_from_slice(&1u16.to_le_bytes());
        bytes[14..16].copy_from_slice(&bits.to_le_bytes());
        bytes
    }

    #[test]
    fn aligned_rows_and_top_down_images() {
        assert_eq!(dib_size(&info_header(40, 3, 2, 24)), Ok(64));
        assert_eq!(dib_size(&info_header(40, 3, -2, 24)), Ok(64));
        assert_eq!(dib_size(&info_header(108, 1, 1, 32)), Ok(112));
        assert_eq!(dib_size(&info_header(40, 1, 1, 1)), Ok(52));
    }

    #[test]
    fn core_header_keeps_original_palette_calculation() {
        let bytes = [12, 0, 0, 0, 3, 0, 2, 0, 1, 0, 1, 0];
        assert_eq!(dib_size(&bytes), Ok(26)); // (2 + 4) * 3 + 4 * 2
    }

    #[test]
    fn bitfields_explicit_sizes_and_palette_counts() {
        let mut bytes = info_header(40, 1, 1, 16);
        put32(&mut bytes, 16, 3);
        assert_eq!(dib_size(&bytes), Ok(56));
        put32(&mut bytes, 16, 1);
        assert_eq!(dib_size(&bytes), Ok(0));
        put32(&mut bytes, 20, 17);
        put32(&mut bytes, 32, 2);
        assert_eq!(dib_size(&bytes), Ok(65));
    }

    #[test]
    fn v5_profiles_follow_source_maximum_and_offset_rules() {
        let mut bytes = info_header(124, 1, 1, 24);
        put32(&mut bytes, 112, 200);
        put32(&mut bytes, 116, 10);
        assert_eq!(dib_size(&bytes), Ok(210));
        put32(&mut bytes, 112, 100);
        assert_eq!(dib_size(&bytes), Ok(138));
    }

    #[test]
    fn overflow_matches_32_bit_arithmetic() {
        // MAX * 32 + 31 wraps to MAX; shifting and masking yields 0x1fff_fffc.
        assert_eq!(dib_size(&info_header(40, u32::MAX, 1, 32)), Ok(0x2000_0024));
    }

    #[test]
    fn truncated_and_unknown_headers_do_not_panic() {
        assert_eq!(
            dib_size(&[]),
            Err(TruncatedHeader {
                required: 4,
                available: 0
            })
        );
        assert_eq!(
            dib_size(&[40, 0, 0, 0]),
            Err(TruncatedHeader {
                required: 40,
                available: 4
            })
        );
        assert_eq!(dib_size(&[16, 0, 0, 0]), Ok(0));
        for size in [12, 40, 108, 124] {
            let mut bytes = vec![0; size as usize];
            put32(&mut bytes, 0, size);
            for length in 0..size as usize {
                assert!(dib_size(&bytes[..length]).is_err());
            }
        }
    }

    #[test]
    fn infer_height_for_all_modern_header_sizes() {
        for size in [40, 108, 124] {
            let mut bytes = info_header(size, 3, 0, 24);
            infer_missing_height(&mut bytes, size as i32 + 24).unwrap();
            assert_eq!(read_u32(&bytes, 8) as i32, -2);
            assert_eq!(read_u32(&bytes, 20), 24);
            assert_eq!(dib_size(&bytes), Ok(size + 24));
        }
    }

    #[test]
    fn inference_subtracts_palettes_masks_and_profiles() {
        let mut palette = info_header(40, 3, 0, 8);
        infer_missing_height(&mut palette, 40 + 1024 + 8).unwrap();
        assert_eq!(read_u32(&palette, 8) as i32, -2);
        put32(&mut palette, 8, 0);
        put32(&mut palette, 20, 0);
        put32(&mut palette, 32, 2);
        infer_missing_height(&mut palette, 56).unwrap();
        assert_eq!(read_u32(&palette, 20), 8);

        let mut masks = info_header(40, 1, 0, 16);
        put32(&mut masks, 16, 3);
        infer_missing_height(&mut masks, 60).unwrap();
        assert_eq!(read_u32(&masks, 8) as i32, -2);
        let mut profile = info_header(124, 1, 0, 24);
        put32(&mut profile, 112, 200);
        put32(&mut profile, 116, 10);
        infer_missing_height(&mut profile, 142).unwrap();
        assert_eq!(read_u32(&profile, 20), 8);
    }

    #[test]
    fn inference_preserves_explicit_fields_and_truncates_partial_rows() {
        let mut bytes = info_header(40, 1, 0, 24);
        put32(&mut bytes, 20, 11);
        infer_missing_height(&mut bytes, 0).unwrap();
        assert_eq!(read_u32(&bytes, 8) as i32, -2);
        assert_eq!(read_u32(&bytes, 20), 11);
        let original = bytes.clone();
        infer_missing_height(&mut bytes, 500).unwrap();
        assert_eq!(bytes, original);
    }

    #[test]
    fn core_inference_preserves_source_asymmetry_and_height_truncation() {
        let mut bytes = [12, 0, 0, 0, 3, 0, 0, 0, 1, 0, 1, 0];
        infer_missing_height(&mut bytes, 26).unwrap();
        assert_eq!(read_u16(&bytes, 6), 2);
        // Inference subtracts 12 + 6; dib_size instead uses (2 + 4) * 3.
        bytes[6..8].copy_from_slice(&0u16.to_le_bytes());
        infer_missing_height(&mut bytes, 18 + 4 * 65537).unwrap();
        assert_eq!(read_u16(&bytes, 6), 1);
    }

    #[test]
    fn inference_errors_are_atomic_and_underflow_wraps() {
        let mut bytes = info_header(40, 0, 0, 24);
        let original = bytes.clone();
        assert_eq!(
            infer_missing_height(&mut bytes, 40),
            Err(InferHeightError::ZeroStride)
        );
        assert_eq!(bytes, original);
        let mut short = [124, 0, 0, 0];
        assert!(matches!(
            infer_missing_height(&mut short, 0),
            Err(InferHeightError::Truncated(_))
        ));
        assert_eq!(short, [124, 0, 0, 0]);
        let mut unknown = [16, 0, 0, 0];
        assert_eq!(infer_missing_height(&mut unknown, 0), Ok(()));
        let mut wrapped = info_header(40, 1, 0, 24);
        infer_missing_height(&mut wrapped, 39).unwrap();
        assert_eq!(read_u32(&wrapped, 20), u32::MAX);
        assert_eq!(read_u32(&wrapped, 8), (u32::MAX / 4).wrapping_neg());
    }
}
