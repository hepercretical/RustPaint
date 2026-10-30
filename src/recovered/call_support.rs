//! Safe boundary for reconstructed address-based adapters. A memory image is
//! explicit data, not process memory. Runtime dependencies are supplied calls.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryFault {
    pub address: u64,
    pub width: usize,
}

impl std::fmt::Display for MemoryFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-byte access outside image at {:#x}",
            self.width, self.address
        )
    }
}
impl std::error::Error for MemoryFault {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryImage {
    pub base: u64,
    pub bytes: Vec<u8>,
}

impl MemoryImage {
    /// Copy a recovered record after checking both complete ranges. Overlap
    /// preserves the original source bytes, as a structure value copy does.
    pub fn copy_record(
        &mut self,
        destination: u64,
        source: u64,
        width: usize,
    ) -> Result<(), MemoryFault> {
        if width == 0 {
            return Ok(());
        }
        let from = self.offset(source, width)?;
        let to = self.offset(destination, width)?;
        self.bytes.copy_within(from..from + width, to);
        Ok(())
    }

    /// Read a little-endian word of one through eight bytes, including the
    /// decompiler's 24-, 40- and 56-bit partial-word accesses.
    pub fn read_word(&self, address: u64, width: usize) -> Result<u64, MemoryFault> {
        if !(1..=8).contains(&width) {
            return Err(MemoryFault { address, width });
        }
        let offset = self.offset(address, width)?;
        let mut bytes = [0; 8];
        bytes[..width].copy_from_slice(&self.bytes[offset..offset + width]);
        Ok(u64::from_le_bytes(bytes))
    }

    /// Check the entire destination before changing any bytes.
    pub fn write_word(
        &mut self,
        address: u64,
        width: usize,
        value: u64,
    ) -> Result<(), MemoryFault> {
        if !(1..=8).contains(&width) {
            return Err(MemoryFault { address, width });
        }
        let offset = self.offset(address, width)?;
        self.bytes[offset..offset + width].copy_from_slice(&value.to_le_bytes()[..width]);
        Ok(())
    }

    fn offset(&self, address: u64, width: usize) -> Result<usize, MemoryFault> {
        let fault = MemoryFault { address, width };
        address.checked_add((width - 1) as u64).ok_or(fault)?;
        let offset =
            usize::try_from(address.checked_sub(self.base).ok_or(fault)?).map_err(|_| fault)?;
        if offset
            .checked_add(width)
            .is_some_and(|end| end <= self.bytes.len())
        {
            Ok(offset)
        } else {
            Err(fault)
        }
    }
    pub fn read_u8(&self, address: u64) -> Result<u8, MemoryFault> {
        let offset = self.offset(address, 1)?;
        Ok(self.bytes[offset])
    }
    pub fn read_u16(&self, address: u64) -> Result<u16, MemoryFault> {
        let offset = self.offset(address, 2)?;
        Ok(u16::from_le_bytes(
            self.bytes[offset..offset + 2].try_into().unwrap(),
        ))
    }
    pub fn read_u32(&self, address: u64) -> Result<u32, MemoryFault> {
        let offset = self.offset(address, 4)?;
        Ok(u32::from_le_bytes(
            self.bytes[offset..offset + 4].try_into().unwrap(),
        ))
    }
    pub fn read_u64(&self, address: u64) -> Result<u64, MemoryFault> {
        let offset = self.offset(address, 8)?;
        Ok(u64::from_le_bytes(
            self.bytes[offset..offset + 8].try_into().unwrap(),
        ))
    }
    pub fn write_u8(&mut self, address: u64, value: u8) -> Result<(), MemoryFault> {
        let offset = self.offset(address, 1)?;
        self.bytes[offset] = value;
        Ok(())
    }
    pub fn write_u16(&mut self, address: u64, value: u16) -> Result<(), MemoryFault> {
        let offset = self.offset(address, 2)?;
        self.bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }
    pub fn write_u32(&mut self, address: u64, value: u32) -> Result<(), MemoryFault> {
        let offset = self.offset(address, 4)?;
        self.bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }
    pub fn write_u64(&mut self, address: u64, value: u64) -> Result<(), MemoryFault> {
        let offset = self.offset(address, 8)?;
        self.bytes[offset..offset + 8].copy_from_slice(&value.to_le_bytes());
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingArguments {
    pub required: usize,
    pub available: usize,
}
impl std::fmt::Display for MissingArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "adapter needs {} arguments; got {}",
            self.required, self.available
        )
    }
}
impl std::error::Error for MissingArguments {}

pub type CleanupCall<'a> = dyn FnMut(&'static str, u64, &mut MemoryImage) + 'a;
pub type ForwardCall<'a> = dyn FnMut(&'static str, &[u64]) -> u64 + 'a;
pub type VirtualCall<'a> = dyn FnMut(u64, usize, &[u64]) + 'a;
pub type RecordCall<'a> = dyn FnMut(&'static str, &[u64], &mut MemoryImage) -> u64 + 'a;
/// Supplies relocated symbol addresses without fabricating their contents.
pub type RelocateCall<'a> = dyn FnMut(&'static str) -> u64 + 'a;
