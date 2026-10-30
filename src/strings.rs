//! Bounded UTF-16 code-unit operations. No Unicode decoding is performed.

pub const S_OK: u32 = 0;
pub const E_INVALIDARG: u32 = 0x8007_0057;
pub const STRSAFE_E_INSUFFICIENT_BUFFER: u32 = 0x8007_007a;

/// Translation of `FUN_140011f10` (`mspaint.c:20306`).
///
/// Destination length replaces the raw pointer's capacity argument. The source
/// slice must include its NUL terminator if it fits in the destination. A source
/// that ends without NUL is treated as invalid input rather than read past its
/// end. The written prefix is retained and terminated in that case.
///
/// A full destination is always NUL terminated and returns insufficient buffer,
/// including when the next source code unit would be NUL. Untouched destination
/// code units after the terminator retain their original contents.
pub fn copy_utf16_z(destination: &mut [u16], source: &[u16]) -> u32 {
    if destination.is_empty() || destination.len() > 0x7fff_ffff {
        if let Some(first) = destination.first_mut() {
            *first = 0;
        }
        return E_INVALIDARG;
    }
    for (index, output) in destination.iter_mut().enumerate() {
        match source.get(index) {
            Some(&0) => {
                *output = 0;
                return S_OK;
            }
            Some(&unit) => *output = unit,
            None => {
                *output = 0;
                return E_INVALIDARG;
            }
        }
    }
    let last = destination.len() - 1;
    destination[last] = 0;
    STRSAFE_E_INSUFFICIENT_BUFFER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_through_nul_without_touching_tail() {
        let mut output = [99; 5];
        assert_eq!(copy_utf16_z(&mut output, &[0xd800, 65, 0, 66]), S_OK);
        assert_eq!(output, [0xd800, 65, 0, 99, 99]);
    }

    #[test]
    fn exact_capacity_requires_room_for_terminator() {
        let mut output = [99; 3];
        assert_eq!(copy_utf16_z(&mut output, &[65, 66, 0]), S_OK);
        assert_eq!(output, [65, 66, 0]);
        assert_eq!(
            copy_utf16_z(&mut output, &[65, 66, 67, 0]),
            STRSAFE_E_INSUFFICIENT_BUFFER
        );
        assert_eq!(output, [65, 66, 0]);
        assert_eq!(
            copy_utf16_z(&mut [99], &[65, 0]),
            STRSAFE_E_INSUFFICIENT_BUFFER
        );
        assert_eq!(copy_utf16_z(&mut [99], &[0]), S_OK);
    }

    #[test]
    fn invalid_inputs_are_bounded_and_terminated() {
        assert_eq!(copy_utf16_z(&mut [], &[0]), E_INVALIDARG);
        let mut output = [99; 3];
        assert_eq!(copy_utf16_z(&mut output, &[65]), E_INVALIDARG);
        assert_eq!(output, [65, 0, 99]);
        assert_eq!(copy_utf16_z(&mut output, &[]), E_INVALIDARG);
        assert_eq!(output, [0, 0, 99]);
    }
}
