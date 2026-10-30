//! Address-named translations whose original class and field names are unknown.
//!
//! These are portable models, not ABI-compatible replacements for C++ objects.
//! Addresses are integers and are never dereferenced. Byte records are checked
//! before access; capture constructors take the unresolved vtable relocation as
//! an explicit argument. Disjoint Rust borrows exclude overlapping records.

pub mod accessors;
pub mod adapters10;
pub mod adapters11;
pub mod adapters12;
pub mod adapters13;
pub mod adapters14;
pub mod adapters15;
pub mod adapters16;
pub mod adapters17;
pub mod adapters18;
pub mod adapters19;
pub mod adapters20;
pub mod adapters21;
pub mod adapters22;
pub mod adapters23;
pub mod adapters9;
pub mod addresses;
pub mod atomics;
pub mod call_support;
pub mod captures;
pub mod captures2;
pub mod cleanup;
pub mod dispatch7;
pub mod dispatch8;
pub mod errors4;
pub mod exceptions;
pub mod forward3;
pub mod frames4;
pub mod frames5;
pub mod frames6;
pub mod interfaces;
pub mod memory7;
pub mod memory8;
pub mod runtime;
pub mod streams;
pub mod text;
pub mod unwind3;
pub mod virtual3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortBuffer {
    pub required: usize,
    pub available: usize,
}

impl std::fmt::Display for ShortBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "record requires {} bytes; got {}",
            self.required, self.available
        )
    }
}

impl std::error::Error for ShortBuffer {}

fn require(bytes: &[u8], required: usize) -> Result<(), ShortBuffer> {
    if bytes.len() < required {
        Err(ShortBuffer {
            required,
            available: bytes.len(),
        })
    } else {
        Ok(())
    }
}

fn read16(bytes: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap())
}

fn read32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

fn read64(bytes: &[u8], offset: usize) -> u64 {
    u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
}

fn put32(bytes: &mut [u8], offset: usize, value: u32) {
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

fn put64(bytes: &mut [u8], offset: usize, value: u64) {
    bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
}

pub mod adapters24;

pub mod adapters25;

pub mod adapters26;

pub mod adapters27;

pub mod adapters28;
pub mod adapters35;
pub mod adapters36;
pub mod guard40;
pub mod primitives31;
pub mod runtime32;
pub mod search29;
pub mod sections34;
pub mod substring37;
