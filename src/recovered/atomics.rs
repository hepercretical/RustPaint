//! Atomic fields are explicit references instead of fields behind raw pointers.
//! SeqCst ordering conservatively implements the decompiled LOCK/UNLOCK regions.

use std::sync::atomic::{AtomicI32, AtomicI64, AtomicU32, Ordering};

/// `FUN_140004310`: increment the reference count originally at offset 0x1c.
pub fn fun_140004310(count: &AtomicI32) -> i32 {
    count.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}
/// `FUN_140004980`: increment a directly supplied reference count.
pub fn fun_140004980(count: &AtomicI32) -> i32 {
    count.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}
/// `FUN_140004a00`: increment the reference count originally at offset 0x18.
pub fn fun_140004a00(count: &AtomicI32) -> i32 {
    count.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}
/// `FUN_140005d40`: decrement the pointed-to 64-bit counter; no release callback.
pub fn fun_140005d40(count: &AtomicI64) {
    count.fetch_sub(1, Ordering::SeqCst);
}

fn saturating_increment(count: &AtomicI32) -> i32 {
    let mut previous = count.load(Ordering::SeqCst);
    loop {
        if previous == i32::MAX {
            return previous;
        }
        let next = previous.wrapping_add(1);
        match count.compare_exchange(previous, next, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return next,
            Err(observed) => previous = observed,
        }
    }
}

/// `FUN_140011240`: saturating reference increment, originally at offset 0xc.
pub fn fun_140011240(count: &AtomicI32) -> i32 {
    saturating_increment(count)
}
/// `FUN_140011770`: saturating reference increment, originally at offset 0x2c.
pub fn fun_140011770(count: &AtomicI32) -> i32 {
    saturating_increment(count)
}
/// `FUN_14003b160`: atomically exchange a 32-bit word, returning the old bits.
pub fn fun_14003b160(word: &AtomicU32, value: u32) -> u32 {
    word.swap(value, Ordering::SeqCst)
}
/// `FUN_140066460`: increment the reference count originally at offset eight.
pub fn fun_140066460(count: &AtomicI32) -> i32 {
    count.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}
