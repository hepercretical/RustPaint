//! Additional checked source layouts, including packed bit fields.
#![allow(non_camel_case_types, non_upper_case_globals, unused_imports)]
use crate::declaration_support::*;
use crate::declarations30::*;
use crate::recovered::call_support::{MemoryFault, MemoryImage};
/// Explicit source layout IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_33f191c4e08a {
    pub bytes: [u8; 4],
}
impl Wire for t_33f191c4e08a {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_33f191c4e08a {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_OFFSETTODIRECTORY: BitField = BitField::new(0, 4, 0, 31);
    pub const F_DATAISDIRECTORY: BitField = BitField::new(0, 4, 31, 1);
}

/// Explicit source layout IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_0a25ff837faa {
    pub bytes: [u8; 4],
}
impl Wire for t_0a25ff837faa {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_0a25ff837faa {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_OFFSETTODATA: Field<u32> = Field::new(0);
    pub const F_IMAGE_RESOURCE_DIRECTORY_ENTRY_DIRECTORYSTRUCT: Field<t_33f191c4e08a> =
        Field::new(0);
}

/// Explicit source layout _struct_317.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_68762d4fce91 {
    pub bytes: [u8; 16],
}
impl Wire for t_68762d4fce91 {
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
impl t_68762d4fce91 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_DEPTH: BitField = BitField::new(0, 8, 0, 16);
    pub const F_SEQUENCE: BitField = BitField::new(0, 8, 16, 48);
    pub const F_HEADERTYPE: BitField = BitField::new(8, 8, 0, 1);
    pub const F_RESERVED: BitField = BitField::new(8, 8, 1, 3);
    pub const F_NEXTENTRY: BitField = BitField::new(8, 8, 4, 60);
}

/// Explicit source layout _struct_316.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_6650f3d6fe52 {
    pub bytes: [u8; 16],
}
impl Wire for t_6650f3d6fe52 {
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
impl t_6650f3d6fe52 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_DEPTH: BitField = BitField::new(0, 8, 0, 16);
    pub const F_SEQUENCE: BitField = BitField::new(0, 8, 16, 48);
    pub const F_HEADERTYPE: BitField = BitField::new(8, 8, 0, 1);
    pub const F_INIT: BitField = BitField::new(8, 8, 1, 1);
    pub const F_RESERVED: BitField = BitField::new(8, 8, 2, 2);
    pub const F_NEXTENTRY: BitField = BitField::new(8, 8, 4, 60);
}

/// Explicit source layout _struct_315.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_1d67573570b5 {
    pub bytes: [u8; 16],
}
impl Wire for t_1d67573570b5 {
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
impl t_1d67573570b5 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_DEPTH: BitField = BitField::new(0, 8, 0, 16);
    pub const F_SEQUENCE: BitField = BitField::new(0, 8, 16, 9);
    pub const F_NEXTENTRY: BitField = BitField::new(0, 8, 25, 39);
    pub const F_HEADERTYPE: BitField = BitField::new(8, 8, 0, 1);
    pub const F_INIT: BitField = BitField::new(8, 8, 1, 1);
    pub const F_RESERVED: BitField = BitField::new(8, 8, 2, 59);
    pub const F_REGION: BitField = BitField::new(8, 8, 61, 3);
}

/// Explicit source layout _struct_402.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4f57b4157ebe {
    pub bytes: [u8; 4],
}
impl Wire for t_4f57b4157ebe {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4f57b4157ebe {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_LONGFUNCTION: BitField = BitField::new(0, 4, 0, 1);
    pub const F_PERSISTENT: BitField = BitField::new(0, 4, 1, 1);
    pub const F_PRIVATE: BitField = BitField::new(0, 4, 2, 30);
}

/// Explicit source layout _union_401.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_6283f260653f {
    pub bytes: [u8; 4],
}
impl Wire for t_6283f260653f {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_6283f260653f {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_FLAGS: Field<u32> = Field::new(0);
    pub const F_S: Field<t_4f57b4157ebe> = Field::new(0);
}

/// Explicit source layout _TP_CALLBACK_ENVIRON_V3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_15f3b5475a02 {
    pub bytes: [u8; 72],
}
impl Wire for t_15f3b5475a02 {
    const SIZE: usize = 72;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_15f3b5475a02 {
    pub const SIZE: usize = 72;
    pub const ALIGN: usize = 8;
    pub const F_VERSION: Field<u32> = Field::new(0);
    pub const F_POOL: Field<Address<Unknown>> = Field::new(8);
    pub const F_CLEANUPGROUP: Field<Address<Unknown>> = Field::new(16);
    pub const F_CLEANUPGROUPCANCELCALLBACK: Field<CodeAddress> = Field::new(24);
    pub const F_RACEDLL: Field<Address<Unknown>> = Field::new(32);
    pub const F_ACTIVATIONCONTEXT: Field<Address<Unknown>> = Field::new(40);
    pub const F_FINALIZATIONCALLBACK: Field<CodeAddress> = Field::new(48);
    pub const F_U: Field<t_6283f260653f> = Field::new(56);
    pub const F_CALLBACKPRIORITY: Field<Enum32> = Field::new(60);
    pub const F_SIZE: Field<u32> = Field::new(64);
}

/// Explicit source layout _SLIST_HEADER.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5be79252e409 {
    pub bytes: [u8; 16],
}
impl Wire for t_5be79252e409 {
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
impl t_5be79252e409 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_S: Field<t_15dd9826f96e> = Field::new(0);
    pub const F_HEADER8: Field<t_1d67573570b5> = Field::new(0);
    pub const F_HEADER16: Field<t_6650f3d6fe52> = Field::new(0);
    pub const F_HEADERX64: Field<t_68762d4fce91> = Field::new(0);
}

/// Explicit source layout IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4467f88a5a08 {
    pub bytes: [u8; 4],
}
impl Wire for t_4467f88a5a08 {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4467f88a5a08 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_NAMEOFFSET: BitField = BitField::new(0, 4, 0, 31);
    pub const F_NAMEISSTRING: BitField = BitField::new(0, 4, 31, 1);
}

/// Explicit source layout IMAGE_OPTIONAL_HEADER64.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4c02c7411dc1 {
    pub bytes: [u8; 240],
}
impl Wire for t_4c02c7411dc1 {
    const SIZE: usize = 240;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4c02c7411dc1 {
    pub const SIZE: usize = 240;
    pub const ALIGN: usize = 8;
    pub const F_MAGIC: Field<u16> = Field::new(0);
    pub const F_MAJORLINKERVERSION: Field<u8> = Field::new(2);
    pub const F_MINORLINKERVERSION: Field<u8> = Field::new(3);
    pub const F_SIZEOFCODE: Field<u32> = Field::new(4);
    pub const F_SIZEOFINITIALIZEDDATA: Field<u32> = Field::new(8);
    pub const F_SIZEOFUNINITIALIZEDDATA: Field<u32> = Field::new(12);
    pub const F_ADDRESSOFENTRYPOINT: Field<u32> = Field::new(16);
    pub const F_BASEOFCODE: Field<u32> = Field::new(20);
    pub const F_IMAGEBASE: Field<Address<Unknown>> = Field::new(24);
    pub const F_SECTIONALIGNMENT: Field<u32> = Field::new(32);
    pub const F_FILEALIGNMENT: Field<u32> = Field::new(36);
    pub const F_MAJOROPERATINGSYSTEMVERSION: Field<u16> = Field::new(40);
    pub const F_MINOROPERATINGSYSTEMVERSION: Field<u16> = Field::new(42);
    pub const F_MAJORIMAGEVERSION: Field<u16> = Field::new(44);
    pub const F_MINORIMAGEVERSION: Field<u16> = Field::new(46);
    pub const F_MAJORSUBSYSTEMVERSION: Field<u16> = Field::new(48);
    pub const F_MINORSUBSYSTEMVERSION: Field<u16> = Field::new(50);
    pub const F_WIN32VERSIONVALUE: Field<u32> = Field::new(52);
    pub const F_SIZEOFIMAGE: Field<u32> = Field::new(56);
    pub const F_SIZEOFHEADERS: Field<u32> = Field::new(60);
    pub const F_CHECKSUM: Field<u32> = Field::new(64);
    pub const F_SUBSYSTEM: Field<u16> = Field::new(68);
    pub const F_DLLCHARACTERISTICS: Field<u16> = Field::new(70);
    pub const F_SIZEOFSTACKRESERVE: Field<u64> = Field::new(72);
    pub const F_SIZEOFSTACKCOMMIT: Field<u64> = Field::new(80);
    pub const F_SIZEOFHEAPRESERVE: Field<u64> = Field::new(88);
    pub const F_SIZEOFHEAPCOMMIT: Field<u64> = Field::new(96);
    pub const F_LOADERFLAGS: Field<u32> = Field::new(104);
    pub const F_NUMBEROFRVAANDSIZES: Field<u32> = Field::new(108);
    pub const F_DATADIRECTORY: Field<[t_7342e1154646; 16]> = Field::new(112);
}

/// Explicit source layout IMAGE_NT_HEADERS64.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_807803698433 {
    pub bytes: [u8; 264],
}
impl Wire for t_807803698433 {
    const SIZE: usize = 264;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_807803698433 {
    pub const SIZE: usize = 264;
    pub const ALIGN: usize = 8;
    pub const F_SIGNATURE: Field<[i8; 4]> = Field::new(0);
    pub const F_FILEHEADER: Field<t_47fc86ee7976> = Field::new(4);
    pub const F_OPTIONALHEADER: Field<t_4c02c7411dc1> = Field::new(24);
}

/// Explicit source layout IMAGE_LOAD_CONFIG_DIRECTORY64.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_286b2bf6be2e {
    pub bytes: [u8; 256],
}
impl Wire for t_286b2bf6be2e {
    const SIZE: usize = 256;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_286b2bf6be2e {
    pub const SIZE: usize = 256;
    pub const ALIGN: usize = 8;
    pub const F_SIZE: Field<u32> = Field::new(0);
    pub const F_TIMEDATESTAMP: Field<u32> = Field::new(4);
    pub const F_MAJORVERSION: Field<u16> = Field::new(8);
    pub const F_MINORVERSION: Field<u16> = Field::new(10);
    pub const F_GLOBALFLAGSCLEAR: Field<u32> = Field::new(12);
    pub const F_GLOBALFLAGSSET: Field<u32> = Field::new(16);
    pub const F_CRITICALSECTIONDEFAULTTIMEOUT: Field<u32> = Field::new(20);
    pub const F_DECOMMITFREEBLOCKTHRESHOLD: Field<u64> = Field::new(24);
    pub const F_DECOMMITTOTALFREETHRESHOLD: Field<u64> = Field::new(32);
    pub const F_LOCKPREFIXTABLE: Field<Address<Unknown>> = Field::new(40);
    pub const F_MAXIMUMALLOCATIONSIZE: Field<u64> = Field::new(48);
    pub const F_VIRTUALMEMORYTHRESHOLD: Field<u64> = Field::new(56);
    pub const F_PROCESSAFFINITYMASK: Field<u64> = Field::new(64);
    pub const F_PROCESSHEAPFLAGS: Field<u32> = Field::new(72);
    pub const F_CSDVERSION: Field<u16> = Field::new(76);
    pub const F_DEPENDENTLOADFLAGS: Field<u16> = Field::new(78);
    pub const F_EDITLIST: Field<Address<Unknown>> = Field::new(80);
    pub const F_SECURITYCOOKIE: Field<Address<Unknown>> = Field::new(88);
    pub const F_SEHANDLERTABLE: Field<Address<Unknown>> = Field::new(96);
    pub const F_SEHANDLERCOUNT: Field<u64> = Field::new(104);
    pub const F_GUARDCFCCHECKFUNCTIONPOINTER: Field<Address<Unknown>> = Field::new(112);
    pub const F_GUARDCFDISPATCHFUNCTIONPOINTER: Field<Address<Unknown>> = Field::new(120);
    pub const F_GUARDCFFUNCTIONTABLE: Field<Address<Unknown>> = Field::new(128);
    pub const F_GUARDCFFUNCTIONCOUNT: Field<u64> = Field::new(136);
    pub const F_GUARDFLAGS: Field<Enum32> = Field::new(144);
    pub const F_CODEINTEGRITY: Field<t_12a47bfd922d> = Field::new(148);
    pub const F_GUARDADDRESSTAKENIATENTRYTABLE: Field<Address<Unknown>> = Field::new(160);
    pub const F_GUARDADDRESSTAKENIATENTRYCOUNT: Field<u64> = Field::new(168);
    pub const F_GUARDLONGJUMPTARGETTABLE: Field<Address<Unknown>> = Field::new(176);
    pub const F_GUARDLONGJUMPTARGETCOUNT: Field<u64> = Field::new(184);
    pub const F_DYNAMICVALUERELOCTABLE: Field<Address<Unknown>> = Field::new(192);
    pub const F_CHPEMETADATAPOINTER: Field<Address<Unknown>> = Field::new(200);
    pub const F_GUARDRFFAILUREROUTINE: Field<Address<Unknown>> = Field::new(208);
    pub const F_GUARDRFFAILUREROUTINEFUNCTIONPOINTER: Field<Address<Unknown>> = Field::new(216);
    pub const F_DYNAMICVALUERELOCTABLEOFFSET: Field<u32> = Field::new(224);
    pub const F_DYNAMICVALUERELOCTABLESECTION: Field<u16> = Field::new(228);
    pub const F_RESERVED1: Field<u16> = Field::new(230);
    pub const F_GUARDRFVERIFYSTACKPOINTERFUNCTIONPOINTER: Field<Address<Unknown>> = Field::new(232);
    pub const F_HOTPATCHTABLEOFFSET: Field<u32> = Field::new(240);
    pub const F_RESERVED2: Field<u32> = Field::new(244);
    pub const F_RESERVED3: Field<u64> = Field::new(248);
}

/// Explicit source layout IMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_38ebef70f22f {
    pub bytes: [u8; 4],
}
impl Wire for t_38ebef70f22f {
    const SIZE: usize = 4;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_38ebef70f22f {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_IMAGE_RESOURCE_DIRECTORY_ENTRY_NAMESTRUCT: Field<t_4467f88a5a08> = Field::new(0);
    pub const F_NAME: Field<u32> = Field::new(0);
    pub const F_ID: Field<u16> = Field::new(0);
}

/// Explicit source layout IMAGE_RESOURCE_DIRECTORY_ENTRY.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4245c30be2a4 {
    pub bytes: [u8; 8],
}
impl Wire for t_4245c30be2a4 {
    const SIZE: usize = 8;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4245c30be2a4 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_NAMEUNION: Field<t_38ebef70f22f> = Field::new(0);
    pub const F_DIRECTORYUNION: Field<t_0a25ff837faa> = Field::new(4);
}

pub const G_L3870: Global<[Address<Unknown>; 2]> = Global::new("vftable", None);

pub const G_L3891: Global<[Address<Unknown>; 8]> = Global::new("vftable", None);

pub const G_L3895: Global<[Address<Unknown>; 3]> = Global::new("vftable", None);

pub const G_L3901: Global<[Address<Unknown>; 4]> = Global::new("vftable", None);

pub const G_L3914: Global<[Address<Unknown>; 9]> = Global::new("vftable", None);

pub const G_L3932: Global<[Address<Unknown>; 6]> = Global::new("vftable", None);

pub const G_L3933: Global<[Address<Unknown>; 1]> = Global::new("vftable", None);

pub const G_L3942: Global<[Address<Unknown>; 5]> = Global::new("vftable", None);

pub const G_L3975: Global<[Address<Unknown>; 11]> = Global::new("vftable", None);

pub const G_L4057: Global<[Address<Unknown>; 15]> = Global::new("vftable", None);

pub const G_L4110: Global<[Address<Unknown>; 7]> = Global::new("vftable", None);

pub const G_L4111: Global<[Address<Unknown>; 10]> = Global::new("vftable", None);

pub const G_L4112: Global<[Address<Unknown>; 18]> = Global::new("vftable", None);

pub const G_L4254: Global<[Address<Unknown>; 20]> = Global::new("vftable", None);

pub const G_L4270: Global<[Address<Unknown>; 101]> = Global::new("vftable", None);

pub const G_L4319: Global<[Address<Unknown>; 91]> = Global::new("vftable", None);

pub const G_L4322: Global<[Address<Unknown>; 115]> = Global::new("vftable", None);

pub const G_L4398: Global<[Address<Unknown>; 106]> = Global::new("vftable", None);

pub const G_L4450: Global<[Address<Unknown>; 13]> = Global::new("vftable", None);

pub const G_L4467: Global<[Address<Unknown>; 114]> = Global::new("vftable", None);

pub const G_L4579: Global<[Address<Unknown>; 70]> = Global::new("vftable", None);

pub const G_L4580: Global<[Address<Unknown>; 31]> = Global::new("vftable", None);

pub const G_L4581: Global<[Address<Unknown>; 17]> = Global::new("vftable", None);

pub const G_L4590: Global<[Address<Unknown>; 38]> = Global::new("vftable", None);

pub const G_L4650: Global<[Address<Unknown>; 111]> = Global::new("vftable", None);

pub const G_L4655: Global<[Address<Unknown>; 105]> = Global::new("vftable", None);

pub const G_L4746: Global<[Address<Unknown>; 44]> = Global::new("vftable", None);

pub const G_L4845: Global<[Address<Unknown>; 63]> = Global::new("vftable", None);

pub const G_L4877: Global<[Address<Unknown>; 12]> = Global::new("vftable", None);

pub const G_L4904: Global<[Address<Unknown>; 14]> = Global::new("vftable", None);
