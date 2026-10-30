//! CRT initialization and stack probing in the checked source memory model.
use super::call_support::{MemoryFault, MemoryImage, RecordCall, RelocateCall};

/// __scrt_initialize_onexit_tables. Runtime operations remain supplied callees.
/// The decompiler's two uRam slots are ordinary eight-byte global words.
pub fn named_l154902(
    parameter: u32,
    memory: &mut MemoryImage,
    resolve: &mut RelocateCall<'_>,
    call: &mut RecordCall<'_>,
) -> Result<u64, MemoryFault> {
    if memory.read_u8(resolve("DAT_1401cf481"))? == 0 {
        if parameter > 1 {
            call("FUN_1400b090c", &[5], memory);
            let target = call("swi", &[3], memory);
            return Ok(call("INDIRECT_CALL", &[target], memory));
        }
        let crt = call("__scrt_is_ucrt_dll_in_use", &[], memory) as i32;
        if crt == 0 || parameter != 0 {
            for symbol in [
                "DAT_1401cf488",
                "uRam00000001401cf490",
                "_DAT_1401cf498",
                "_DAT_1401cf4a0",
                "uRam00000001401cf4a8",
                "_DAT_1401cf4b0",
            ] {
                memory.write_u64(resolve(symbol), u64::MAX)?;
            }
        } else {
            let first = call(
                "_initialize_onexit_table",
                &[resolve("DAT_1401cf488")],
                memory,
            ) as i32;
            if first != 0 {
                return Ok(0);
            }
            let second = call(
                "_initialize_onexit_table",
                &[resolve("DAT_1401cf4a0")],
                memory,
            ) as i32;
            if second != 0 {
                return Ok(0);
            }
        }
        memory.write_u8(resolve("DAT_1401cf481"), 1)?;
    }
    Ok(1)
}

/// Incoming register/thread/frame values used by the printed __chkstk body.
#[derive(Debug, Clone, Copy)]
pub struct ProbeContext {
    /// The incoming RAX allocation-size word.
    pub allocation: u64,
    /// Address of the source local_res8 home area, not a host Rust stack address.
    pub local_res8: u64,
    pub stack_limit: u64,
}

/// __chkstk: touch one byte per downward 4 KiB page, retaining prior writes on fault.
/// This models the source image accesses; it does not grow the host thread stack.
pub fn named_l176343(context: ProbeContext, memory: &mut MemoryImage) -> Result<(), MemoryFault> {
    let target = if context.local_res8 < context.allocation {
        0
    } else {
        context.local_res8.wrapping_sub(context.allocation)
    };
    if target < context.stack_limit {
        let last_page = target & 0xffff_ffff_ffff_f000;
        let mut page = context.stack_limit;
        loop {
            page = page.wrapping_sub(0x1000);
            memory.write_u8(page, 0)?;
            if last_page == page {
                break;
            }
        }
    }
    Ok(())
}
