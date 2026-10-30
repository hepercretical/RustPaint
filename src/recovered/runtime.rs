//! Recovered runtime operations with pointers flattened into explicit arguments.

use super::{put32, put64, read64, require, ShortBuffer};

/// `FUN_140002020`: return the exception message or its null-pointer fallback.
pub fn fun_140002020(message: Option<&str>) -> &str {
    message.unwrap_or("Unknown exception")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingNul;
impl std::fmt::Display for MissingNul {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("byte string has no NUL terminator")
    }
}
impl std::error::Error for MissingNul {}

/// `FUN_1400052e0`: form a borrowed string view up to the first NUL byte.
/// The returned slice models the original pointer/length pair. Missing NUL is
/// reported instead of continuing the source's unbounded memory read.
pub fn fun_1400052e0(bytes: &[u8]) -> Result<&[u8], MissingNul> {
    bytes
        .iter()
        .position(|&byte| byte == 0)
        .map(|length| &bytes[..length])
        .ok_or(MissingNul)
}
/// `FUN_1400186e0`: bad_optional_access diagnostic.
pub const fn fun_1400186e0() -> &'static str {
    "Bad optional access"
}
/// `FUN_140018cb0`: system error-category name.
pub const fn fun_140018cb0() -> &'static str {
    "system"
}
/// `FUN_140018d70`: generic error-category name.
pub const fn fun_140018d70() -> &'static str {
    "generic"
}
/// `FUN_140047f50`: recovered unconditional success value.
pub const fn fun_140047f50() -> u64 {
    1
}
/// `FUN_14007c270`: recovered constant timeout value; units are unknown.
pub const fn fun_14007c270() -> u64 {
    5000
}
/// `FUN_14009b370`: recovered unconditional byte value.
pub const fn fun_14009b370() -> u8 {
    1
}
/// `FUN_1400b08a8`: recovered runtime constant.
pub const fn fun_1400b08a8() -> u64 {
    0x4000
}

/// `FUN_140041c00`: fill a valid u64 range with the pointed-to source value.
pub fn fun_140041c00(destination: &mut [u64], value: u64) {
    destination.fill(value);
}

/// `FUN_1400b02b4`: destroy `count` elements in reverse address order.
/// The supplied callback replaces the original destructor function pointer.
/// `count` models the valid nonnegative source domain; addresses wrap at 64 bits.
pub fn fun_1400b02b4(mut end: u64, stride: u64, count: usize, mut destroy: impl FnMut(u64)) {
    for _ in 0..count {
        end = end.wrapping_sub(stride);
        destroy(end);
    }
}

/// `FUN_1400613a0`: write the recovered interface GUID and return S_OK.
pub fn fun_1400613a0(output: &mut [u64; 2]) -> u64 {
    *output = [0x3000a, 0x4600_0000_0000_00c0];
    0
}

/// `FUN_140094330`: read the 64-bit field at byte offset 0x40.
pub fn fun_140094330(record: &[u8]) -> Result<u64, ShortBuffer> {
    require(record, 72)?;
    Ok(read64(record, 64))
}

/// `FUN_1400cc35c`: write E_OUTOFMEMORY through the flattened output pointer.
pub fn fun_1400cc35c(output: &mut u32) -> u64 {
    *output = 0x8007_000e;
    0
}

/// `FUN_1400cccac`: set the pointed-to object's 32-bit state at offset 0x60 to two.
pub fn fun_1400cccac(record: &mut [u8]) -> Result<u64, ShortBuffer> {
    require(record, 100)?;
    put32(record, 96, 2);
    Ok(0)
}
/// `FUN_1400cccd5`: set that state to three.
pub fn fun_1400cccd5(record: &mut [u8]) -> Result<u64, ShortBuffer> {
    require(record, 100)?;
    put32(record, 96, 3);
    Ok(0)
}

/// `FUN_1400d2512`: copy the word at 0xf0 to 0x120 and return zero.
pub fn fun_1400d2512(record: &mut [u8]) -> Result<u64, ShortBuffer> {
    require(record, 296)?;
    put64(record, 288, read64(record, 240));
    Ok(0)
}

/// `FUN_1400d83cf`: test a flattened exception code for access violation.
pub fn fun_1400d83cf(code: &i32) -> bool {
    *code == -0x3fff_fffb
}

/// `FUN_140024b70`: commit a signed 32-bit position only when its flag is nonzero.
/// The source's indirect target field and byte flag are explicit arguments.
pub fn fun_140024b70(position: i32, commit: u8, target: &mut i64) {
    if commit != 0 {
        *target = i64::from(position);
    }
}
