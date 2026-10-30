//! Exception/Value record initialization. The caller supplies unresolved vtable
//! and constant-message relocations. These byte records are not live C++ objects.

use super::{put64, require, ShortBuffer};

/// `FUN_140001fb0`: construct bad_alloc; message must point to "bad allocation".
pub fn fun_140001fb0(record: &mut [u8], vtable: u64, message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 16, 0);
    put64(record, 8, message);
    put64(record, 0, vtable);
    Ok(())
}
/// `FUN_140005790`: bad_array_new_length; message is "bad array new length".
pub fn fun_140005790(record: &mut [u8], vtable: u64, message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 16, 0);
    put64(record, 8, message);
    put64(record, 0, vtable);
    Ok(())
}
/// `FUN_1400186a0`: initialize std::exception with no message or ownership.
pub fn fun_1400186a0(record: &mut [u8], vtable: u64, _message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 0, vtable);
    put64(record, 8, 0);
    put64(record, 16, 0);
    Ok(())
}
/// `FUN_1400186c0`: initialize bad_optional_access with no stored message.
pub fn fun_1400186c0(record: &mut [u8], vtable: u64, _message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 8, 0);
    put64(record, 16, 0);
    put64(record, 0, vtable);
    Ok(())
}
/// `FUN_140031b60`: construct bad_cast; message must point to "bad cast".
pub fn fun_140031b60(record: &mut [u8], vtable: u64, message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 16, 0);
    put64(record, 8, message);
    put64(record, 0, vtable);
    Ok(())
}
/// `FUN_14003ae40`: initialize Concurrency::task_canceled.
pub fn fun_14003ae40(record: &mut [u8], vtable: u64, _message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 8, 0);
    put64(record, 16, 0);
    put64(record, 0, vtable);
    Ok(())
}
/// `FUN_14005b550`: initialize Concurrency::invalid_operation.
pub fn fun_14005b550(record: &mut [u8], vtable: u64, _message: u64) -> Result<(), ShortBuffer> {
    require(record, 24)?;
    put64(record, 8, 0);
    put64(record, 16, 0);
    put64(record, 0, vtable);
    Ok(())
}
/// `FUN_14001a910`: install the Value vtable, preserving the entire payload.
pub fn fun_14001a910(record: &mut [u8], vtable: u64, _message: u64) -> Result<(), ShortBuffer> {
    require(record, 8)?;
    put64(record, 0, vtable);
    Ok(())
}

pub type ExceptionConstructor = fn(&mut [u8], u64, u64) -> Result<(), ShortBuffer>;
pub const CONSTRUCTORS: &[(&str, usize, ExceptionConstructor)] = &[
    ("FUN_140001fb0", 24, fun_140001fb0),
    ("FUN_140005790", 24, fun_140005790),
    ("FUN_1400186a0", 24, fun_1400186a0),
    ("FUN_1400186c0", 24, fun_1400186c0),
    ("FUN_140031b60", 24, fun_140031b60),
    ("FUN_14003ae40", 24, fun_14003ae40),
    ("FUN_14005b550", 24, fun_14005b550),
    ("FUN_14001a910", 8, fun_14001a910),
];
