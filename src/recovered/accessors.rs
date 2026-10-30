//! Recovered field operations. Offsets are bytes in the original x64 records.
//! Pointer-returning setters are represented by mutation of a Rust reference.

use super::{put32, put64, read16, read32, read64, require, ShortBuffer};

/// `FUN_140001b50`: assign the supplied error word; C returns the same pointer.
pub fn fun_140001b50(output: &mut u32, value: u32) {
    *output = value;
}

/// `FUN_140002190`: return the pointed-to word.
pub fn fun_140002190(value: &u32) -> u32 {
    *value
}

/// `FUN_1400069f0`: copy the 32-bit field at byte offset 12.
pub fn fun_1400069f0(record: &[u8], output: &mut u32) -> Result<(), ShortBuffer> {
    require(record, 16)?;
    *output = read32(record, 12);
    Ok(())
}

/// `FUN_1400095b0`: write one and return S_OK. The unused C argument is omitted.
pub fn fun_1400095b0(output: &mut u32) -> u64 {
    *output = 1;
    0
}

/// `FUN_14000a870`: zero a single 64-bit word.
pub fn fun_14000a870(output: &mut u64) {
    *output = 0;
}

/// `FUN_1400122b0`: preserve the supplied word.
pub fn fun_1400122b0(value: u32) -> u32 {
    value
}

/// `FUN_140018e70`: initialize code and category, preserving padding bytes 4..8.
pub fn fun_140018e70(record: &mut [u8], code: u32, category: u64) -> Result<(), ShortBuffer> {
    require(record, 16)?;
    put32(record, 0, code);
    put64(record, 8, category);
    Ok(())
}

/// `FUN_140024c50`: clear code and category, preserving padding bytes 4..8.
pub fn fun_140024c50(record: &mut [u8]) -> Result<(), ShortBuffer> {
    require(record, 16)?;
    put32(record, 0, 0);
    put64(record, 8, 0);
    Ok(())
}

/// `FUN_140031740`: return the 16-bit field at byte offset 24.
pub fn fun_140031740(record: &[u8]) -> Result<u16, ShortBuffer> {
    require(record, 26)?;
    Ok(read16(record, 24))
}

/// `FUN_140031750`: return the 16-bit field at byte offset 26.
pub fn fun_140031750(record: &[u8]) -> Result<u16, ShortBuffer> {
    require(record, 28)?;
    Ok(read16(record, 26))
}

/// `FUN_14003b150`: test whether the 64-bit field at byte offset 16 is nonzero.
pub fn fun_14003b150(record: &[u8]) -> Result<bool, ShortBuffer> {
    require(record, 24)?;
    Ok(read64(record, 16) != 0)
}

/// `FUN_14003b1a0`: return the pointed-to 64-bit word.
pub fn fun_14003b1a0(value: &u64) -> u64 {
    *value
}

/// `FUN_14003c7a0`: calculate rectangle area times four, preserving signed widths.
/// The coordinate subtractions wrap at 32 bits; the products wrap at 64 bits.
pub fn fun_14003c7a0(record: &[u8]) -> Result<i64, ShortBuffer> {
    require(record, 32)?;
    let height = (read32(record, 28) as i32).wrapping_sub(read32(record, 20) as i32);
    let width = (read32(record, 24) as i32).wrapping_sub(read32(record, 16) as i32);
    Ok(i64::from(height)
        .wrapping_mul(i64::from(width))
        .wrapping_mul(4))
}

/// `FUN_140051b60`: return the 64-bit field at byte offset 72.
pub fn fun_140051b60(record: &[u8]) -> Result<u64, ShortBuffer> {
    require(record, 80)?;
    Ok(read64(record, 72))
}

/// `FUN_140052360`: return the byte at offset 209, without bool normalization.
pub fn fun_140052360(record: &[u8]) -> Result<u8, ShortBuffer> {
    require(record, 210)?;
    Ok(record[209])
}

/// `FUN_140066450`: return the 64-bit field at byte offset 16.
pub fn fun_140066450(record: &[u8]) -> Result<u64, ShortBuffer> {
    require(record, 24)?;
    Ok(read64(record, 16))
}

/// `FUN_140066ee0`: assign the 32-bit field at byte offset 144.
pub fn fun_140066ee0(record: &mut [u8], value: u32) -> Result<(), ShortBuffer> {
    require(record, 148)?;
    put32(record, 144, value);
    Ok(())
}

/// `FUN_140066f00`: return the 32-bit field at byte offset 144.
pub fn fun_140066f00(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 148)?;
    Ok(read32(record, 144))
}

/// `FUN_140069e90`: return the 32-bit field at byte offset 360.
pub fn fun_140069e90(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 364)?;
    Ok(read32(record, 360))
}

/// `FUN_14009bd50`: set byte 64 iff the signed argument equals one.
pub fn fun_14009bd50(record: &mut [u8], value: i32) -> Result<(), ShortBuffer> {
    require(record, 65)?;
    record[64] = u8::from(value == 1);
    Ok(())
}

/// `FUN_14009bd60`: set byte 65 iff the signed argument equals one.
pub fn fun_14009bd60(record: &mut [u8], value: i32) -> Result<(), ShortBuffer> {
    require(record, 66)?;
    record[65] = u8::from(value == 1);
    Ok(())
}

/// `FUN_14007c280`: return masked flag 0x20 at byte offset 332.
pub fn fun_14007c280(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 336)?;
    Ok(read32(record, 332) & 0x20)
}
/// `FUN_14007c290`: return masked flag 0x10 at byte offset 332.
pub fn fun_14007c290(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 336)?;
    Ok(read32(record, 332) & 0x10)
}
/// `FUN_14007c2a0`: return masked flag 8 at byte offset 332.
pub fn fun_14007c2a0(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 336)?;
    Ok(read32(record, 332) & 8)
}
/// `FUN_14007c2b0`: return masked flag 4 at byte offset 332.
pub fn fun_14007c2b0(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 336)?;
    Ok(read32(record, 332) & 4)
}
/// `FUN_14007c2c0`: return masked flag 2 at byte offset 332.
pub fn fun_14007c2c0(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 336)?;
    Ok(read32(record, 332) & 2)
}
/// `FUN_14007c2d0`: return masked flag 1 at byte offset 332.
pub fn fun_14007c2d0(record: &[u8]) -> Result<u32, ShortBuffer> {
    require(record, 336)?;
    Ok(read32(record, 332) & 1)
}

/// `FUN_14005dbb0`: classify an image format descriptor by GUID and bit depth.
/// The C object's descriptor pointer at offset 0x298 is flattened into this
/// explicit argument. The first two words hold the GUID in x64 memory order.
pub fn fun_14005dbb0(descriptor: [u64; 3]) -> u64 {
    let [first, second, properties] = descriptor;
    if second != 0x2ef3_1ef8_0000_7b9d {
        return 0;
    }
    match first {
        0x11d3_0728_b96b_3cab => match (properties >> 8) & 0xff {
            1 => 1,
            4 => 2,
            8 => 4,
            24 => 8,
            _ => 15,
        },
        0x11d3_0728_b96b_3cae => 0x10,
        0x11d3_0728_b96b_3cb0 => 0x20,
        0x11d3_0728_b96b_3cb1 => 0x40,
        0x11d3_0728_b96b_3caf => 0x80,
        _ => 0,
    }
}
