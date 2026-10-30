//! Typed declarations for the decompiler's explicit memory model.
//! Addresses are integers in a caller-owned MemoryImage, never native pointers.
use crate::recovered::call_support::{MemoryFault, MemoryImage};
use core::marker::PhantomData;
fn range(memory: &MemoryImage, address: u64, width: usize) -> Result<usize, MemoryFault> {
    let fault = MemoryFault { address, width };
    if width != 0 {
        address.checked_add((width - 1) as u64).ok_or(fault)?;
    }
    let offset =
        usize::try_from(address.checked_sub(memory.base).ok_or(fault)?).map_err(|_| fault)?;
    if offset
        .checked_add(width)
        .is_some_and(|end| end <= memory.bytes.len())
    {
        Ok(offset)
    } else {
        Err(fault)
    }
}
pub trait Wire: Sized {
    const SIZE: usize;
    fn load(memory: &MemoryImage, address: u64) -> Result<Self, MemoryFault>;
    fn store(&self, memory: &mut MemoryImage, address: u64) -> Result<(), MemoryFault>;
}
/// Unsigned C bit field within a little-endian allocation unit.
/// Stores truncate to the field width and preserve every neighboring bit.
#[derive(Debug, Clone, Copy)]
pub struct BitField {
    pub offset: u64,
    width: usize,
    shift: u32,
    bits: u32,
}
impl BitField {
    pub const fn new(offset: u64, width: usize, shift: u32, bits: u32) -> Self {
        assert!(width > 0 && width <= 8 && bits > 0 && shift + bits <= width as u32 * 8);
        Self {
            offset,
            width,
            shift,
            bits,
        }
    }
    fn mask(self) -> u64 {
        u64::MAX >> (64 - self.bits)
    }
    fn address(self, base: u64) -> Result<u64, MemoryFault> {
        base.checked_add(self.offset).ok_or(MemoryFault {
            address: base,
            width: self.width,
        })
    }
    pub fn load(self, memory: &MemoryImage, base: u64) -> Result<u64, MemoryFault> {
        Ok((memory.read_word(self.address(base)?, self.width)? >> self.shift) & self.mask())
    }
    pub fn store(self, memory: &mut MemoryImage, base: u64, value: u64) -> Result<(), MemoryFault> {
        let address = self.address(base)?;
        let previous = memory.read_word(address, self.width)?;
        let mask = self.mask() << self.shift;
        memory.write_word(
            address,
            self.width,
            (previous & !mask) | ((value & self.mask()) << self.shift),
        )
    }
}
macro_rules! integer {($($ty:ty),*)=>{$(impl Wire for $ty {const SIZE:usize=core::mem::size_of::<Self>();fn load(m:&MemoryImage,a:u64)->Result<Self,MemoryFault>{Ok(m.read_word(a,Self::SIZE)? as Self)}fn store(&self,m:&mut MemoryImage,a:u64)->Result<(),MemoryFault>{m.write_word(a,Self::SIZE,*self as u64)}})*};}
integer!(u8, i8, u16, i16, u32, i32, u64, i64);
impl Wire for f32 {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self::from_bits(m.read_u32(a)?))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        m.write_u32(a, self.to_bits())
    }
}
impl Wire for f64 {
    const SIZE: usize = 8;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self::from_bits(m.read_u64(a)?))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        m.write_u64(a, self.to_bits())
    }
}
impl<T: Wire, const N: usize> Wire for [T; N] {
    const SIZE: usize = T::SIZE * N;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        range(m, a, Self::SIZE)?;
        let mut values = Vec::with_capacity(N);
        for i in 0..N {
            values.push(T::load(m, a + i as u64 * T::SIZE as u64)?);
        }
        Ok(values.try_into().unwrap_or_else(|_| unreachable!()))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        range(m, a, Self::SIZE)?;
        for (i, value) in self.iter().enumerate() {
            value.store(m, a + i as u64 * T::SIZE as u64)?;
        }
        Ok(())
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Address<T: ?Sized> {
    pub value: u64,
    marker: PhantomData<fn() -> T>,
}
impl<T: ?Sized> Copy for Address<T> {}
impl<T: ?Sized> Clone for Address<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T: ?Sized> Address<T> {
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}
impl<T: Wire> Address<T> {
    pub fn load(self, m: &MemoryImage) -> Result<T, MemoryFault> {
        T::load(m, self.value)
    }
    pub fn store(self, m: &mut MemoryImage, value: &T) -> Result<(), MemoryFault> {
        value.store(m, self.value)
    }
}
impl<T: ?Sized> Wire for Address<T> {
    const SIZE: usize = 8;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self::new(m.read_u64(a)?))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        m.write_u64(a, self.value)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relative32<T: ?Sized> {
    pub offset: u32,
    marker: PhantomData<fn() -> T>,
}
impl<T: ?Sized> Relative32<T> {
    pub const fn new(offset: u32) -> Self {
        Self {
            offset,
            marker: PhantomData,
        }
    }
    pub fn resolve(self, image_base: u64) -> Option<Address<T>> {
        image_base
            .checked_add(u64::from(self.offset))
            .map(Address::new)
    }
}
impl<T: ?Sized> Wire for Relative32<T> {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self::new(m.read_u32(a)?))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        m.write_u32(a, self.offset)
    }
}
#[derive(Debug)]
pub enum Unknown {}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodeAddress(pub u64);
impl Wire for CodeAddress {
    const SIZE: usize = 8;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self(m.read_u64(a)?))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        m.write_u64(a, self.0)
    }
}
#[derive(Debug)]
pub struct Field<T> {
    pub offset: usize,
    marker: PhantomData<fn() -> T>,
}
impl<T> Field<T> {
    pub const fn new(offset: usize) -> Self {
        Self {
            offset,
            marker: PhantomData,
        }
    }
}
impl<T: Wire> Field<T> {
    pub fn load(&self, m: &MemoryImage, base: u64) -> Result<T, MemoryFault> {
        let address = base.checked_add(self.offset as u64).ok_or(MemoryFault {
            address: base,
            width: T::SIZE,
        })?;
        T::load(m, address)
    }
    pub fn store(&self, m: &mut MemoryImage, base: u64, value: &T) -> Result<(), MemoryFault> {
        let address = base.checked_add(self.offset as u64).ok_or(MemoryFault {
            address: base,
            width: T::SIZE,
        })?;
        value.store(m, address)
    }
}
#[derive(Debug)]
pub struct Global<T> {
    pub symbol: &'static str,
    pub preferred_address: Option<u64>,
    marker: PhantomData<fn() -> T>,
}
impl<T> Global<T> {
    pub const fn new(symbol: &'static str, preferred_address: Option<u64>) -> Self {
        Self {
            symbol,
            preferred_address,
            marker: PhantomData,
        }
    }
    pub fn address(&self, resolve: &mut dyn FnMut(&'static str) -> u64) -> Address<T> {
        Address::new(resolve(self.symbol))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Enum32(pub u32);
/// A bounded, NUL-terminated declaration view, retaining raw code units.
#[derive(Debug)]
pub struct TextGlobal<T> {
    pub symbol: &'static str,
    pub preferred_address: Option<u64>,
    marker: PhantomData<fn() -> T>,
}
pub trait TextUnit: Wire + Copy + PartialEq {
    const ZERO: Self;
}
impl TextUnit for u8 {
    const ZERO: Self = 0;
}
impl TextUnit for u16 {
    const ZERO: Self = 0;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextFault {
    Memory(MemoryFault),
    Unterminated { address: u64, limit: usize },
}
impl<T> TextGlobal<T> {
    pub const fn new(symbol: &'static str, preferred_address: Option<u64>) -> Self {
        Self {
            symbol,
            preferred_address,
            marker: PhantomData,
        }
    }
}
impl<T: TextUnit> TextGlobal<T> {
    pub fn read(
        &self,
        memory: &MemoryImage,
        resolve: &mut dyn FnMut(&'static str) -> u64,
        limit: usize,
    ) -> Result<Vec<T>, TextFault> {
        let start = resolve(self.symbol);
        let mut address = start;
        let mut units = Vec::new();
        for _ in 0..limit {
            let fault = MemoryFault {
                address,
                width: T::SIZE,
            };
            range(memory, address, T::SIZE).map_err(TextFault::Memory)?;
            let value = T::load(memory, address).map_err(TextFault::Memory)?;
            if value == T::ZERO {
                return Ok(units);
            }
            units.push(value);
            address = address
                .checked_add(T::SIZE as u64)
                .ok_or(TextFault::Memory(fault))?;
        }
        Err(TextFault::Unterminated {
            address: start,
            limit,
        })
    }
}
impl Wire for Enum32 {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self(m.read_u32(a)?))
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        m.write_u32(a, self.0)
    }
}
pub fn load_bytes<const N: usize>(m: &MemoryImage, a: u64) -> Result<[u8; N], MemoryFault> {
    let at = range(m, a, N)?;
    let mut bytes = [0; N];
    bytes.copy_from_slice(&m.bytes[at..at + N]);
    Ok(bytes)
}
pub fn store_bytes(m: &mut MemoryImage, a: u64, bytes: &[u8]) -> Result<(), MemoryFault> {
    let at = range(m, a, bytes.len())?;
    m.bytes[at..at + bytes.len()].copy_from_slice(bytes);
    Ok(())
}
