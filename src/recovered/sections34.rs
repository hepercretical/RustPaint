//! The section-permission scan printed in FUN_1400b0538.
//! Section storage and the end sentinel are caller-supplied image addresses.
use super::call_support::{MemoryFault, MemoryImage, RelocateCall};

/// Preserve the complete returned word: the source changes only its low byte.
/// This ports the printed fixed-table body; it does not add PE-header checks
/// that are absent from that pseudocode.
pub fn fun_1400b0538(
    address: u64,
    memory: &MemoryImage,
    resolve: &mut RelocateCall<'_>,
) -> Result<u64, MemoryFault> {
    let relative = address.wrapping_sub(0x140000000);
    let mut result = 0u64;
    let mut section = resolve("IMAGE_SECTION_HEADER_140000230");
    let end = resolve("DAT_140000348");
    while section != end {
        let virtual_address = memory.read_u32(section.wrapping_add(12))?;
        if u64::from(virtual_address) <= relative {
            let size = memory.read_u32(section.wrapping_add(8))?;
            result = u64::from(size.wrapping_add(virtual_address));
            if relative < result {
                let flags = memory.read_u32(section.wrapping_add(36))?;
                return Ok((result & !0xff) | u64::from(flags as i32 >= 0));
            }
        }
        section = section.wrapping_add(40);
    }
    Ok(result & !0xff)
}
