//! Checked scalar lowering of UTF-16 vector comparisons and the startup lock.
use super::call_support::{MemoryFault, MemoryImage, RecordCall, RelocateCall};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimitiveError {
    Memory(MemoryFault),
    /// The source spin loop has not acquired ownership within the supplied budget.
    SpinLimit,
}
impl From<MemoryFault> for PrimitiveError {
    fn from(value: MemoryFault) -> Self {
        Self::Memory(value)
    }
}
fn comparison(memory: &MemoryImage, a: u64, b: u64) -> Result<u32, MemoryFault> {
    Ok(if memory.read_u16(b)? <= memory.read_u16(a)? {
        1
    } else {
        u32::MAX
    })
}
fn equal_mask(memory: &MemoryImage, a: u64, b: u64, lanes: u32) -> Result<u32, MemoryFault> {
    // Materialize both complete vector operands before testing mismatch bits.
    let mut left = [0u16; 16];
    let mut right = [0u16; 16];
    for i in 0..lanes {
        left[i as usize] = memory.read_u16(a.wrapping_add(u64::from(i) * 2))?;
    }
    for i in 0..lanes {
        right[i as usize] = memory.read_u16(b.wrapping_add(u64::from(i) * 2))?;
    }
    let mut mask = 0;
    for i in 0..lanes {
        if left[i as usize] == right[i as usize] {
            mask |= 3u32 << (i * 2);
        }
    }
    Ok(mask)
}
/// FUN_14003bc20: unsigned UTF-16 lexicographic comparison, including embedded zeroes.
pub fn fun_14003bc20(
    a: u64,
    b: u64,
    count: u64,
    memory: &MemoryImage,
    resolve: &mut RelocateCall<'_>,
) -> Result<u32, MemoryFault> {
    let mut index = 0u64;
    let mut pa = a;
    let mut pb = b;
    let feature = memory.read_u32(resolve("DAT_1401d03a4"))?;
    if feature != 0 && count > 15 {
        let mut end = 16u64;
        loop {
            let mask = equal_mask(memory, pa, pb, 16)?;
            if mask != u32::MAX {
                let offset = index
                    .wrapping_add(u64::from((!mask).trailing_zeros() >> 1))
                    .wrapping_mul(2);
                return comparison(memory, a.wrapping_add(offset), b.wrapping_add(offset));
            }
            index = index.wrapping_add(16);
            end = end.wrapping_add(16);
            pa = pa.wrapping_add(32);
            pb = pb.wrapping_add(32);
            if end > count {
                break;
            }
        }
    }
    while index.wrapping_add(8) <= count {
        let mask = equal_mask(memory, pa, pb, 8)?;
        if mask != 0xffff {
            let offset = index
                .wrapping_add(u64::from((!mask).trailing_zeros() >> 1))
                .wrapping_mul(2);
            return comparison(memory, a.wrapping_add(offset), b.wrapping_add(offset));
        }
        index = index.wrapping_add(8);
        pa = pa.wrapping_add(16);
        pb = pb.wrapping_add(16);
    }
    if index.wrapping_add(4) <= count {
        let left = memory.read_u64(pa)?;
        let right = memory.read_u64(pb)?;
        if left != right {
            let offset = index
                .wrapping_add(u64::from((left ^ right).trailing_zeros() >> 4))
                .wrapping_mul(2);
            return comparison(memory, a.wrapping_add(offset), b.wrapping_add(offset));
        }
        index = index.wrapping_add(4);
    }
    while index < count {
        let offset = index.wrapping_mul(2);
        let left = memory.read_u16(a.wrapping_add(offset))?;
        let right = memory.read_u16(b.wrapping_add(offset))?;
        if left != right {
            return Ok(if right <= left { 1 } else { u32::MAX });
        }
        index = index.wrapping_add(1);
    }
    Ok(0)
}
/// __scrt_acquire_startup_lock. StackBase is explicit thread context.
/// Exclusive MemoryImage access models the source LOCK/UNLOCK serially.
pub fn named_l154839(
    stack_base: u64,
    spin_budget: usize,
    memory: &mut MemoryImage,
    resolve: &mut RelocateCall<'_>,
    call: &mut RecordCall<'_>,
) -> Result<u64, PrimitiveError> {
    if call("__scrt_is_ucrt_dll_in_use", &[], memory) as i32 == 0 {
        return Ok(0);
    }
    for _ in 0..spin_budget {
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        let address = resolve("DAT_1401cf478");
        let previous = memory.read_u64(address)?;
        let owner = if previous == 0 { stack_base } else { previous };
        memory.write_u64(address, owner)?;
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        if previous == 0 {
            return Ok(0);
        }
        if stack_base == previous {
            return Ok(1);
        }
    }
    Err(PrimitiveError::SpinLimit)
}
