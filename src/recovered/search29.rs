//! Checked scalar lowering of the recovered SSE/AVX search operations.
//! The feature word and mask table are caller-supplied image storage. Vector
//! branches preserve full-block reads; masked tails access only enabled dwords.
use super::call_support::{MemoryFault, MemoryImage, MissingArguments, RecordCall, RelocateCall};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    Arguments(MissingArguments),
    Memory(MemoryFault),
}
impl From<MemoryFault> for SearchError {
    fn from(value: MemoryFault) -> Self {
        Self::Memory(value)
    }
}
fn block_mask(
    memory: &MemoryImage,
    address: u64,
    size: usize,
    width: usize,
    needle: u64,
) -> Result<u32, MemoryFault> {
    let mut mask = 0u32;
    for offset in (0..size).step_by(width) {
        let value = memory.read_word(address.wrapping_add(offset as u64), width)?;
        if value == needle {
            mask |= ((1u32 << width) - 1) << offset;
        }
    }
    Ok(mask)
}
fn masked_mask(
    memory: &MemoryImage,
    address: u64,
    mask_address: u64,
    width: usize,
    needle: u64,
) -> Result<u32, MemoryFault> {
    let mut mask_bytes = [0u8; 32];
    for (offset, byte) in mask_bytes.iter_mut().enumerate() {
        *byte = memory.read_u8(mask_address.wrapping_add(offset as u64))?;
    }
    let mut loaded = [0u8; 32];
    for lane in 0..8 {
        if mask_bytes[lane * 4 + 3] & 0x80 != 0 {
            let value = memory.read_u32(address.wrapping_add((lane * 4) as u64))?;
            loaded[lane * 4..lane * 4 + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    let mut result = 0;
    for offset in (0..32).step_by(width) {
        let value = if width == 1 {
            u64::from(loaded[offset])
        } else {
            u64::from(u16::from_le_bytes([loaded[offset], loaded[offset + 1]]))
        };
        if value == needle {
            for byte in 0..width {
                result |= u32::from(mask_bytes[offset + byte] >> 7) << (offset + byte);
            }
        }
    }
    Ok(result)
}
fn found(cursor: u64, mask: u32, reverse: bool) -> u64 {
    if reverse {
        cursor
            .wrapping_add(u64::from(31 - mask.leading_zeros()))
            .wrapping_sub(1)
    } else {
        cursor.wrapping_add(u64::from(mask.trailing_zeros()))
    }
}
fn search(
    arguments: &[u64],
    memory: &MemoryImage,
    relocate: &mut RelocateCall<'_>,
    width: usize,
    reverse: bool,
) -> Result<Option<u64>, SearchError> {
    if arguments.len() < 3 {
        return Err(SearchError::Arguments(MissingArguments {
            required: 3,
            available: arguments.len(),
        }));
    }
    let (start, end) = (arguments[0], arguments[1]);
    let needle = if width == 1 {
        arguments[2] & 255
    } else {
        arguments[2] & 65535
    };
    let length = end.wrapping_sub(start);
    let avx = length & !31 != 0 && memory.read_u32(relocate("DAT_14017909c"))? & 0x20 != 0;
    let block = if avx {
        32
    } else if length & !15 != 0 && memory.read_u32(relocate("DAT_14017909c"))? & 4 != 0 {
        16
    } else {
        0
    };
    let mut cursor = if reverse { end } else { start };
    if block != 0 {
        let bulk = length & !(block as u64 - 1);
        let mut traversed = 0;
        while traversed != bulk {
            if reverse {
                cursor = cursor.wrapping_sub(block as u64);
            }
            let mask = block_mask(memory, cursor, block, width, needle)?;
            if mask != 0 {
                return Ok(Some(found(cursor, mask, reverse)));
            }
            if !reverse {
                cursor = cursor.wrapping_add(block as u64);
            }
            traversed += block as u64;
        }
        if avx {
            let tail = length & 0x1c;
            if tail != 0 {
                if reverse {
                    cursor = cursor.wrapping_sub(tail);
                }
                let table = relocate("DAT_1400e7ed0").wrapping_sub(tail);
                let mask = masked_mask(memory, cursor, table, width, needle)?;
                if mask != 0 {
                    return Ok(Some(found(cursor, mask, reverse)));
                }
                if !reverse {
                    cursor = cursor.wrapping_add(tail);
                }
            }
        }
    }
    if reverse {
        loop {
            if cursor == start {
                return Ok(Some(end));
            }
            cursor = cursor.wrapping_sub(width as u64);
            if memory.read_word(cursor, width)? == needle {
                return Ok(Some(cursor));
            }
        }
    } else {
        while cursor != end && memory.read_word(cursor, width)? != needle {
            cursor = cursor.wrapping_add(width as u64);
        }
        Ok(Some(cursor))
    }
}
pub type Handler = fn(
    &[u64],
    &mut MemoryImage,
    &mut RelocateCall<'_>,
    &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError>;

/// Translation of `FUN_1400b0ff0`; delegates SIMD lowering to checked byte operations.
pub fn fun_1400b0ff0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, relocate, 2, true)
}

/// Translation of `FUN_1400b1150`; delegates SIMD lowering to checked byte operations.
pub fn fun_1400b1150(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, relocate, 1, false)
}

/// Translation of `FUN_1400b1250`; delegates SIMD lowering to checked byte operations.
pub fn fun_1400b1250(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, relocate, 2, false)
}

/// Translation of `NAMED_L156226`; delegates SIMD lowering to checked byte operations.
pub fn named_l156226(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, relocate, 2, true)
}

/// Translation of `NAMED_L156384`; delegates SIMD lowering to checked byte operations.
pub fn named_l156384(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, relocate, 1, false)
}

/// Translation of `NAMED_L156532`; delegates SIMD lowering to checked byte operations.
pub fn named_l156532(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, SearchError> {
    search(arguments, memory, relocate, 2, false)
}

pub const HANDLERS: &[(&str, usize, Handler)] = &[
    ("FUN_1400b0ff0", 3, fun_1400b0ff0),
    ("FUN_1400b1150", 3, fun_1400b1150),
    ("FUN_1400b1250", 3, fun_1400b1250),
    ("NAMED_L156226", 3, named_l156226),
    ("NAMED_L156384", 3, named_l156384),
    ("NAMED_L156532", 3, named_l156532),
];
