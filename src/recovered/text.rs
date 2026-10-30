//! UTF-16 decoding and Windows path-root scanning recovered from the source.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DecodedUnit {
    pub consumed: usize,
    pub codepoint: u32,
    pub valid: bool,
}

/// Translation of `FUN_14002c790` (mspaint.c:42294).
/// Replace raw next/end pointers with a consumed count. Empty input returns
/// None instead of performing the source's unchecked first-unit read.
pub fn fun_14002c790(input: &[u16]) -> Option<DecodedUnit> {
    let &first = input.first()?;
    let mut decoded = DecodedUnit {
        consumed: 1,
        codepoint: u32::from(first),
        valid: true,
    };
    match first {
        0xd800..=0xdbff => {
            if let Some(&second @ 0xdc00..=0xdfff) = input.get(1) {
                decoded.consumed = 2;
                decoded.codepoint = u32::from(first - 0xd800) * 0x400 + u32::from(second) + 0x2400;
            } else {
                decoded.codepoint = 0xfffd;
                decoded.valid = false;
            }
        }
        0xdc00..=0xdfff => {
            decoded.codepoint = 0xfffd;
            decoded.valid = false;
        }
        _ => {}
    }
    Some(decoded)
}

fn separator(unit: u16) -> bool {
    unit == u16::from(b'\\') || unit == u16::from(b'/')
}

/// Translation of `FUN_140019de0` (mspaint.c:27053).
/// Return the original end-of-root-name pointer as an index into UTF-16 input.
/// Retains the decompilation's handling of drive, UNC and device prefixes.
pub fn fun_140019de0(path: &[u16]) -> usize {
    if path.len() < 2 {
        return 0;
    }
    let packed = u32::from(path[0]) | (u32::from(path[1]) << 16);
    if (packed & 0xffff_ffdf).wrapping_sub(0x003a_0041) < 26 {
        return 2;
    }
    if !separator(path[0]) {
        return 0;
    }
    if path.len() > 3
        && separator(path[3])
        && !(path.len() != 4 && separator(path[4]))
        && ((separator(path[1]) && matches!(path[2], 0x3f | 0x2e))
            || (path[1] == 0x3f && path[2] == 0x3f))
    {
        return 3;
    }
    if path.len() < 3 {
        return 0;
    }
    if separator(path[1]) && !separator(path[2]) {
        let mut end = 3;
        while end < path.len() && !separator(path[end]) {
            end += 1;
        }
        end
    } else {
        0
    }
}
