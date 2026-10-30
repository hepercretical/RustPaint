//! Remaining usable header declarations, with explicit checked wire views.
#![allow(non_camel_case_types)]
use crate::declaration_support::*;
use crate::recovered::call_support::{MemoryFault, MemoryImage};
/// Source *32 image-base-relative pointer aliases.
pub type RttiBaseRva = Relative32<crate::declarations30::t_a12ab21829aa>;
pub type RttiBaseArrayRva = Relative32<RttiBaseRva>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7e18d64b2a05 {
    pub bytes: [u8; 16],
}
impl Wire for t_7e18d64b2a05 {
    const SIZE: usize = 16;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_7e18d64b2a05 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const SIGNATURE: Field<u32> = Field::new(0);
    pub const ATTRIBUTES: Field<u32> = Field::new(4);
    pub const NUM_BASE_CLASSES: Field<u32> = Field::new(8);
    pub const BASE_CLASS_ARRAY: Field<RttiBaseArrayRva> = Field::new(12);
}

pub mod alias_l1508 {
    pub type A0 = crate::declarations30::t_999833a83ad0;
    pub type A1 = crate::declaration_support::Address<A0>;
}

pub mod alias_l1513 {
    pub type A0 = crate::declarations30::t_a7f85eef12dc;
    pub type A1 = crate::declaration_support::Address<A0>;
}

pub mod alias_l1825 {
    pub type A0 = crate::declarations30::t_0a271b6f7116;
    pub type A1 = crate::declaration_support::Address<A0>;
}

/// Logical callback signature; addresses are image words, not host pointers.
pub type ExceptionRoutine = fn(
    Address<crate::declarations30::t_e7ed2387b2bc>,
    Address<Unknown>,
    Address<crate::declarations30::t_ea23a4932299>,
    Address<Unknown>,
) -> Enum32;

pub const G_L3922: TextGlobal<u8> =
    TextGlobal::new("s_string_too_long_140105720", Some(0x140105720));

pub const G_L4411: TextGlobal<u16> =
    TextGlobal::new("u_MessageFlattenLayers_140113938", Some(0x140113938));

pub const G_L4471: TextGlobal<u16> = TextGlobal::new("u_MSPaintApp_1401790e0", Some(0x1401790e0));

pub const G_L4653: TextGlobal<u16> = TextGlobal::new("u_MSPaintView_1401790f8", Some(0x1401790f8));

pub const G_L4684: TextGlobal<u16> = TextGlobal::new("u_EditColors_1401089b0", Some(0x1401089b0));
