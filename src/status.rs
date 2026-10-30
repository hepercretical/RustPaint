//! Error conversions recovered from the executable, with no Windows dependency.

/// Translation of `FUN_14000a540` (`mspaint.c:13868`).
/// Preserves zero and values whose signed i32 interpretation is negative.
pub const fn hresult_from_win32(code: u32) -> u32 {
    if (code as i32) < 1 {
        code
    } else {
        (code & 0xffff) | 0x8007_0000
    }
}

/// Translation of `FUN_140012f40` (`mspaint.c:21301`).
/// This is the mapping present in this decompilation, not a general OS lookup.
pub const fn ntstatus_from_hresult(code: u32) -> u32 {
    match code {
        0 => return 0,
        0x8007_0001 => return 0xc000_0002,
        0x8000_4005 => return 0xc000_0001,
        0x8007_0002 => return 0xc000_0034,
        0x8007_0003 => return 0xc000_003a,
        0x8007_000e => return 0xc000_0017,
        0x8007_0057 => return 0xc000_000d,
        0x8007_0070 => return 0xc000_007f,
        0x8007_007a => return 0xc000_0023,
        0x8007_007b => return 0xc000_0033,
        0x8007_007e => return 0xc000_0135,
        0x8007_00ea => return 0x8000_0005,
        0x8007_0216 => return 0xc000_0095,
        0x8007_023e => return 0xc000_0025,
        0x8007_0246 => return 0xc000_0161,
        0x8007_0247 => return 0xc000_0163,
        0x8007_0272 => return 0xc000_0273,
        0x8007_047e => return 0xc000_0059,
        0x8007_050c => return 0xc000_042b,
        0x8007_054f => return 0xc000_00e5,
        0x8007_05b9 => return 0xc000_a083,
        _ => {}
    }
    if code & 0x1000_0000 != 0 {
        return code & 0xefff_ffff;
    }
    match code & 0x1fff_0000 {
        0x0007_0000 => {
            if code & 0xffff == 0 {
                0
            } else {
                (code & 0xffff) | 0xc007_0000
            }
        }
        0x0009_0000 => {
            if (code as i32) < 1 {
                code
            } else {
                (code & 0xffff) | 0xc009_0000
            }
        }
        _ => 0xc000_00e5,
    }
}

/// The three initialized words in `FUN_1400131e0` (`mspaint.c:21391`).
/// The original enclosing type is unknown; this models only those words.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorWords {
    pub hresult: u32,
    pub ntstatus: u32,
    pub reserved: u32,
}

impl ErrorWords {
    pub const fn new(hresult: u32) -> Self {
        Self {
            hresult,
            ntstatus: ntstatus_from_hresult(hresult),
            reserved: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn win32_conversion_preserves_signed_cases_and_truncates_codes() {
        assert_eq!(hresult_from_win32(0), 0);
        assert_eq!(hresult_from_win32(5), 0x8007_0005);
        assert_eq!(hresult_from_win32(0x1234_5678), 0x8007_5678);
        assert_eq!(hresult_from_win32(0x8000_0001), 0x8000_0001);
        assert_eq!(hresult_from_win32(u32::MAX), u32::MAX);
    }

    #[test]
    fn status_mapping_and_fallback_precedence() {
        assert_eq!(ntstatus_from_hresult(0), 0);
        assert_eq!(ntstatus_from_hresult(0x8007_0002), 0xc000_0034);
        assert_eq!(ntstatus_from_hresult(0x8007_00ea), 0x8000_0005);
        assert_eq!(ntstatus_from_hresult(0xd000_0005), 0xc000_0005);
        assert_eq!(ntstatus_from_hresult(0x8007_1234), 0xc007_1234);
        assert_eq!(ntstatus_from_hresult(0x8007_0000), 0);
        assert_eq!(ntstatus_from_hresult(0x0009_0005), 0xc009_0005);
        assert_eq!(ntstatus_from_hresult(0x8009_0005), 0x8009_0005);
        assert_eq!(ntstatus_from_hresult(0x8000_4001), 0xc000_00e5);
        assert_eq!(
            ErrorWords::new(0x8007_0002),
            ErrorWords {
                hresult: 0x8007_0002,
                ntstatus: 0xc000_0034,
                reserved: 0,
            }
        );
    }
}
