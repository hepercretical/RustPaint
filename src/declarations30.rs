//! Generated typed views of explicit mspaint.c declarations.
//! Source identities and unresolved declarations are listed in declarations-030.json.
#![allow(non_camel_case_types, non_upper_case_globals, unused_imports)]
use crate::declaration_support::*;
use crate::recovered::call_support::{MemoryFault, MemoryImage};
/// struct PMD: 12 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_43395d5dc722 {
    pub bytes: [u8; 12],
}
impl Wire for t_43395d5dc722 {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_43395d5dc722 {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 4;
    pub const F_MDISP: Field<i32> = Field::new(0);
    pub const F_PDISP: Field<i32> = Field::new(4);
    pub const F_VDISP: Field<i32> = Field::new(8);
}

/// struct _s__RTTIBaseClassDescriptor: 28 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a12ab21829aa {
    pub bytes: [u8; 28],
}
impl Wire for t_a12ab21829aa {
    const SIZE: usize = 28;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a12ab21829aa {
    pub const SIZE: usize = 28;
    pub const ALIGN: usize = 4;
    pub const F_PTYPEDESCRIPTOR: Field<u32> = Field::new(0);
    pub const F_NUMCONTAINEDBASES: Field<u32> = Field::new(4);
    pub const F_WHERE: Field<t_43395d5dc722> = Field::new(8);
    pub const F_ATTRIBUTES: Field<u32> = Field::new(20);
    pub const F_PCLASSHIERARCHYDESCRIPTOR: Field<u32> = Field::new(24);
}

/// struct _s__RTTICompleteObjectLocator: 20 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a1b9860cb6ca {
    pub bytes: [u8; 20],
}
impl Wire for t_a1b9860cb6ca {
    const SIZE: usize = 20;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a1b9860cb6ca {
    pub const SIZE: usize = 20;
    pub const ALIGN: usize = 4;
    pub const F_SIGNATURE: Field<u32> = Field::new(0);
    pub const F_OFFSET: Field<u32> = Field::new(4);
    pub const F_CDOFFSET: Field<u32> = Field::new(8);
    pub const F_PTYPEDESCRIPTOR: Field<u32> = Field::new(12);
    pub const F_PCLASSDESCRIPTOR: Field<u32> = Field::new(16);
}

/// struct CLIENT_ID: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_16e802e5e765 {
    pub bytes: [u8; 16],
}
impl Wire for t_16e802e5e765 {
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
impl t_16e802e5e765 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_UNIQUEPROCESS: Field<Address<Unknown>> = Field::new(0);
    pub const F_UNIQUETHREAD: Field<Address<Unknown>> = Field::new(8);
}

/// struct _struct_553: 4 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_019e13be7222 {
    pub bytes: [u8; 4],
}
impl Wire for t_019e13be7222 {
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
impl t_019e13be7222 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 2;
    pub const F_WPROCESSORARCHITECTURE: Field<u16> = Field::new(0);
    pub const F_WRESERVED: Field<u16> = Field::new(2);
}

/// union _union_552: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a37b3dfd76db {
    pub bytes: [u8; 4],
}
impl Wire for t_a37b3dfd76db {
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
impl t_a37b3dfd76db {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_DWOEMID: Field<u32> = Field::new(0);
    pub const F_S: Field<t_019e13be7222> = Field::new(0);
}

/// struct _SYSTEM_INFO: 48 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d3f90483a5cf {
    pub bytes: [u8; 48],
}
impl Wire for t_d3f90483a5cf {
    const SIZE: usize = 48;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d3f90483a5cf {
    pub const SIZE: usize = 48;
    pub const ALIGN: usize = 8;
    pub const F_U: Field<t_a37b3dfd76db> = Field::new(0);
    pub const F_DWPAGESIZE: Field<u32> = Field::new(4);
    pub const F_LPMINIMUMAPPLICATIONADDRESS: Field<Address<Unknown>> = Field::new(8);
    pub const F_LPMAXIMUMAPPLICATIONADDRESS: Field<Address<Unknown>> = Field::new(16);
    pub const F_DWACTIVEPROCESSORMASK: Field<u64> = Field::new(24);
    pub const F_DWNUMBEROFPROCESSORS: Field<u32> = Field::new(32);
    pub const F_DWPROCESSORTYPE: Field<u32> = Field::new(36);
    pub const F_DWALLOCATIONGRANULARITY: Field<u32> = Field::new(40);
    pub const F_WPROCESSORLEVEL: Field<u16> = Field::new(44);
    pub const F_WPROCESSORREVISION: Field<u16> = Field::new(46);
}

/// struct _struct_541: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_9be79d4e6283 {
    pub bytes: [u8; 8],
}
impl Wire for t_9be79d4e6283 {
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
impl t_9be79d4e6283 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_OFFSET: Field<u32> = Field::new(0);
    pub const F_OFFSETHIGH: Field<u32> = Field::new(4);
}

/// union _union_540: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7f6edd60f405 {
    pub bytes: [u8; 8],
}
impl Wire for t_7f6edd60f405 {
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
impl t_7f6edd60f405 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_S: Field<t_9be79d4e6283> = Field::new(0);
    pub const F_POINTER: Field<Address<Unknown>> = Field::new(0);
}

/// struct _OVERLAPPED: 32 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_9028a010187a {
    pub bytes: [u8; 32],
}
impl Wire for t_9028a010187a {
    const SIZE: usize = 32;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_9028a010187a {
    pub const SIZE: usize = 32;
    pub const ALIGN: usize = 8;
    pub const F_INTERNAL: Field<u64> = Field::new(0);
    pub const F_INTERNALHIGH: Field<u64> = Field::new(8);
    pub const F_U: Field<t_7f6edd60f405> = Field::new(16);
    pub const F_HEVENT: Field<Address<Unknown>> = Field::new(24);
}

/// struct _FILETIME: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_dbd699519b88 {
    pub bytes: [u8; 8],
}
impl Wire for t_dbd699519b88 {
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
impl t_dbd699519b88 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_DWLOWDATETIME: Field<u32> = Field::new(0);
    pub const F_DWHIGHDATETIME: Field<u32> = Field::new(4);
}

/// struct _WIN32_FIND_DATAW: 592 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_62c6d3acb4a3 {
    pub bytes: [u8; 592],
}
impl Wire for t_62c6d3acb4a3 {
    const SIZE: usize = 592;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_62c6d3acb4a3 {
    pub const SIZE: usize = 592;
    pub const ALIGN: usize = 4;
    pub const F_DWFILEATTRIBUTES: Field<u32> = Field::new(0);
    pub const F_FTCREATIONTIME: Field<t_dbd699519b88> = Field::new(4);
    pub const F_FTLASTACCESSTIME: Field<t_dbd699519b88> = Field::new(12);
    pub const F_FTLASTWRITETIME: Field<t_dbd699519b88> = Field::new(20);
    pub const F_NFILESIZEHIGH: Field<u32> = Field::new(28);
    pub const F_NFILESIZELOW: Field<u32> = Field::new(32);
    pub const F_DWRESERVED0: Field<u32> = Field::new(36);
    pub const F_DWRESERVED1: Field<u32> = Field::new(40);
    pub const F_CFILENAME: Field<[i16; 260]> = Field::new(44);
    pub const F_CALTERNATEFILENAME: Field<[i16; 14]> = Field::new(564);
}

/// union _RTL_RUN_ONCE: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a9c742fb1d97 {
    pub bytes: [u8; 8],
}
impl Wire for t_a9c742fb1d97 {
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
impl t_a9c742fb1d97 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_PTR: Field<Address<Unknown>> = Field::new(0);
}

/// struct _SECURITY_ATTRIBUTES: 24 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_f2074362ce16 {
    pub bytes: [u8; 24],
}
impl Wire for t_f2074362ce16 {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_f2074362ce16 {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 8;
    pub const F_NLENGTH: Field<u32> = Field::new(0);
    pub const F_LPSECURITYDESCRIPTOR: Field<Address<Unknown>> = Field::new(8);
    pub const F_BINHERITHANDLE: Field<i32> = Field::new(16);
}

/// struct _M128A: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_cbc0908ab753 {
    pub bytes: [u8; 16],
}
impl Wire for t_cbc0908ab753 {
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
impl t_cbc0908ab753 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_LOW: Field<u64> = Field::new(0);
    pub const F_HIGH: Field<i64> = Field::new(8);
}

/// struct _XSAVE_FORMAT: 512 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5cf4a2ad55dd {
    pub bytes: [u8; 512],
}
impl Wire for t_5cf4a2ad55dd {
    const SIZE: usize = 512;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_5cf4a2ad55dd {
    pub const SIZE: usize = 512;
    pub const ALIGN: usize = 8;
    pub const F_CONTROLWORD: Field<u16> = Field::new(0);
    pub const F_STATUSWORD: Field<u16> = Field::new(2);
    pub const F_TAGWORD: Field<u8> = Field::new(4);
    pub const F_RESERVED1: Field<u8> = Field::new(5);
    pub const F_ERROROPCODE: Field<u16> = Field::new(6);
    pub const F_ERROROFFSET: Field<u32> = Field::new(8);
    pub const F_ERRORSELECTOR: Field<u16> = Field::new(12);
    pub const F_RESERVED2: Field<u16> = Field::new(14);
    pub const F_DATAOFFSET: Field<u32> = Field::new(16);
    pub const F_DATASELECTOR: Field<u16> = Field::new(20);
    pub const F_RESERVED3: Field<u16> = Field::new(22);
    pub const F_MXCSR: Field<u32> = Field::new(24);
    pub const F_MXCSR_MASK: Field<u32> = Field::new(28);
    pub const F_FLOATREGISTERS: Field<[t_cbc0908ab753; 8]> = Field::new(32);
    pub const F_XMMREGISTERS: Field<[t_cbc0908ab753; 16]> = Field::new(160);
    pub const F_RESERVED4: Field<[u8; 96]> = Field::new(416);
}

/// struct _struct_55: 416 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_275f2ba3754d {
    pub bytes: [u8; 416],
}
impl Wire for t_275f2ba3754d {
    const SIZE: usize = 416;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_275f2ba3754d {
    pub const SIZE: usize = 416;
    pub const ALIGN: usize = 8;
    pub const F_HEADER: Field<[t_cbc0908ab753; 2]> = Field::new(0);
    pub const F_LEGACY: Field<[t_cbc0908ab753; 8]> = Field::new(32);
    pub const F_XMM0: Field<t_cbc0908ab753> = Field::new(160);
    pub const F_XMM1: Field<t_cbc0908ab753> = Field::new(176);
    pub const F_XMM2: Field<t_cbc0908ab753> = Field::new(192);
    pub const F_XMM3: Field<t_cbc0908ab753> = Field::new(208);
    pub const F_XMM4: Field<t_cbc0908ab753> = Field::new(224);
    pub const F_XMM5: Field<t_cbc0908ab753> = Field::new(240);
    pub const F_XMM6: Field<t_cbc0908ab753> = Field::new(256);
    pub const F_XMM7: Field<t_cbc0908ab753> = Field::new(272);
    pub const F_XMM8: Field<t_cbc0908ab753> = Field::new(288);
    pub const F_XMM9: Field<t_cbc0908ab753> = Field::new(304);
    pub const F_XMM10: Field<t_cbc0908ab753> = Field::new(320);
    pub const F_XMM11: Field<t_cbc0908ab753> = Field::new(336);
    pub const F_XMM12: Field<t_cbc0908ab753> = Field::new(352);
    pub const F_XMM13: Field<t_cbc0908ab753> = Field::new(368);
    pub const F_XMM14: Field<t_cbc0908ab753> = Field::new(384);
    pub const F_XMM15: Field<t_cbc0908ab753> = Field::new(400);
}

/// union _union_54: 512 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bf1bd547dc12 {
    pub bytes: [u8; 512],
}
impl Wire for t_bf1bd547dc12 {
    const SIZE: usize = 512;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_bf1bd547dc12 {
    pub const SIZE: usize = 512;
    pub const ALIGN: usize = 8;
    pub const F_FLTSAVE: Field<t_5cf4a2ad55dd> = Field::new(0);
    pub const F_S: Field<t_275f2ba3754d> = Field::new(0);
}

/// struct _CONTEXT: 1232 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ea23a4932299 {
    pub bytes: [u8; 1232],
}
impl Wire for t_ea23a4932299 {
    const SIZE: usize = 1232;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_ea23a4932299 {
    pub const SIZE: usize = 1232;
    pub const ALIGN: usize = 8;
    pub const F_P1HOME: Field<u64> = Field::new(0);
    pub const F_P2HOME: Field<u64> = Field::new(8);
    pub const F_P3HOME: Field<u64> = Field::new(16);
    pub const F_P4HOME: Field<u64> = Field::new(24);
    pub const F_P5HOME: Field<u64> = Field::new(32);
    pub const F_P6HOME: Field<u64> = Field::new(40);
    pub const F_CONTEXTFLAGS: Field<u32> = Field::new(48);
    pub const F_MXCSR: Field<u32> = Field::new(52);
    pub const F_SEGCS: Field<u16> = Field::new(56);
    pub const F_SEGDS: Field<u16> = Field::new(58);
    pub const F_SEGES: Field<u16> = Field::new(60);
    pub const F_SEGFS: Field<u16> = Field::new(62);
    pub const F_SEGGS: Field<u16> = Field::new(64);
    pub const F_SEGSS: Field<u16> = Field::new(66);
    pub const F_EFLAGS: Field<u32> = Field::new(68);
    pub const F_DR0: Field<u64> = Field::new(72);
    pub const F_DR1: Field<u64> = Field::new(80);
    pub const F_DR2: Field<u64> = Field::new(88);
    pub const F_DR3: Field<u64> = Field::new(96);
    pub const F_DR6: Field<u64> = Field::new(104);
    pub const F_DR7: Field<u64> = Field::new(112);
    pub const F_RAX: Field<u64> = Field::new(120);
    pub const F_RCX: Field<u64> = Field::new(128);
    pub const F_RDX: Field<u64> = Field::new(136);
    pub const F_RBX: Field<u64> = Field::new(144);
    pub const F_RSP: Field<u64> = Field::new(152);
    pub const F_RBP: Field<u64> = Field::new(160);
    pub const F_RSI: Field<u64> = Field::new(168);
    pub const F_RDI: Field<u64> = Field::new(176);
    pub const F_R8: Field<u64> = Field::new(184);
    pub const F_R9: Field<u64> = Field::new(192);
    pub const F_R10: Field<u64> = Field::new(200);
    pub const F_R11: Field<u64> = Field::new(208);
    pub const F_R12: Field<u64> = Field::new(216);
    pub const F_R13: Field<u64> = Field::new(224);
    pub const F_R14: Field<u64> = Field::new(232);
    pub const F_R15: Field<u64> = Field::new(240);
    pub const F_RIP: Field<u64> = Field::new(248);
    pub const F_U: Field<t_bf1bd547dc12> = Field::new(256);
    pub const F_VECTORREGISTER: Field<[t_cbc0908ab753; 26]> = Field::new(768);
    pub const F_VECTORCONTROL: Field<u64> = Field::new(1184);
    pub const F_DEBUGCONTROL: Field<u64> = Field::new(1192);
    pub const F_LASTBRANCHTORIP: Field<u64> = Field::new(1200);
    pub const F_LASTBRANCHFROMRIP: Field<u64> = Field::new(1208);
    pub const F_LASTEXCEPTIONTORIP: Field<u64> = Field::new(1216);
    pub const F_LASTEXCEPTIONFROMRIP: Field<u64> = Field::new(1224);
}

/// struct _EXCEPTION_RECORD: 152 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_e7ed2387b2bc {
    pub bytes: [u8; 152],
}
impl Wire for t_e7ed2387b2bc {
    const SIZE: usize = 152;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_e7ed2387b2bc {
    pub const SIZE: usize = 152;
    pub const ALIGN: usize = 8;
    pub const F_EXCEPTIONCODE: Field<u32> = Field::new(0);
    pub const F_EXCEPTIONFLAGS: Field<u32> = Field::new(4);
    pub const F_EXCEPTIONRECORD: Field<Address<Unknown>> = Field::new(8);
    pub const F_EXCEPTIONADDRESS: Field<Address<Unknown>> = Field::new(16);
    pub const F_NUMBERPARAMETERS: Field<u32> = Field::new(24);
    pub const F_EXCEPTIONINFORMATION: Field<[u64; 15]> = Field::new(32);
}

/// struct _EXCEPTION_POINTERS: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_62fc25d82c1b {
    pub bytes: [u8; 16],
}
impl Wire for t_62fc25d82c1b {
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
impl t_62fc25d82c1b {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_EXCEPTIONRECORD: Field<Address<t_e7ed2387b2bc>> = Field::new(0);
    pub const F_CONTEXTRECORD: Field<Address<t_ea23a4932299>> = Field::new(8);
}

/// struct _STARTUPINFOW: 104 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_e9a7801c89db {
    pub bytes: [u8; 104],
}
impl Wire for t_e9a7801c89db {
    const SIZE: usize = 104;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_e9a7801c89db {
    pub const SIZE: usize = 104;
    pub const ALIGN: usize = 8;
    pub const F_CB: Field<u32> = Field::new(0);
    pub const F_LPRESERVED: Field<Address<i16>> = Field::new(8);
    pub const F_LPDESKTOP: Field<Address<i16>> = Field::new(16);
    pub const F_LPTITLE: Field<Address<i16>> = Field::new(24);
    pub const F_DWX: Field<u32> = Field::new(32);
    pub const F_DWY: Field<u32> = Field::new(36);
    pub const F_DWXSIZE: Field<u32> = Field::new(40);
    pub const F_DWYSIZE: Field<u32> = Field::new(44);
    pub const F_DWXCOUNTCHARS: Field<u32> = Field::new(48);
    pub const F_DWYCOUNTCHARS: Field<u32> = Field::new(52);
    pub const F_DWFILLATTRIBUTE: Field<u32> = Field::new(56);
    pub const F_DWFLAGS: Field<u32> = Field::new(60);
    pub const F_WSHOWWINDOW: Field<u16> = Field::new(64);
    pub const F_CBRESERVED2: Field<u16> = Field::new(66);
    pub const F_LPRESERVED2: Field<Address<u8>> = Field::new(72);
    pub const F_HSTDINPUT: Field<Address<Unknown>> = Field::new(80);
    pub const F_HSTDOUTPUT: Field<Address<Unknown>> = Field::new(88);
    pub const F_HSTDERROR: Field<Address<Unknown>> = Field::new(96);
}

/// struct _RTL_CONDITION_VARIABLE: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5e6052772e22 {
    pub bytes: [u8; 8],
}
impl Wire for t_5e6052772e22 {
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
impl t_5e6052772e22 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_PTR: Field<Address<Unknown>> = Field::new(0);
}

/// struct _RTL_SRWLOCK: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_f7cecc1b772e {
    pub bytes: [u8; 8],
}
impl Wire for t_f7cecc1b772e {
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
impl t_f7cecc1b772e {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_PTR: Field<Address<Unknown>> = Field::new(0);
}

/// struct _RTL_CRITICAL_SECTION: 40 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d263a3913bc6 {
    pub bytes: [u8; 40],
}
impl Wire for t_d263a3913bc6 {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d263a3913bc6 {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 8;
    pub const F_DEBUGINFO: Field<Address<Unknown>> = Field::new(0);
    pub const F_LOCKCOUNT: Field<i32> = Field::new(8);
    pub const F_RECURSIONCOUNT: Field<i32> = Field::new(12);
    pub const F_OWNINGTHREAD: Field<Address<Unknown>> = Field::new(16);
    pub const F_LOCKSEMAPHORE: Field<Address<Unknown>> = Field::new(24);
    pub const F_SPINCOUNT: Field<u64> = Field::new(32);
}

/// struct _LIST_ENTRY: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4022dd7ff700 {
    pub bytes: [u8; 16],
}
impl Wire for t_4022dd7ff700 {
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
impl t_4022dd7ff700 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_FLINK: Field<Address<Unknown>> = Field::new(0);
    pub const F_BLINK: Field<Address<Unknown>> = Field::new(8);
}

/// struct _RTL_CRITICAL_SECTION_DEBUG: 48 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_f7c016150490 {
    pub bytes: [u8; 48],
}
impl Wire for t_f7c016150490 {
    const SIZE: usize = 48;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_f7c016150490 {
    pub const SIZE: usize = 48;
    pub const ALIGN: usize = 8;
    pub const F_TYPE: Field<u16> = Field::new(0);
    pub const F_CREATORBACKTRACEINDEX: Field<u16> = Field::new(2);
    pub const F_CRITICALSECTION: Field<Address<t_d263a3913bc6>> = Field::new(8);
    pub const F_PROCESSLOCKSLIST: Field<t_4022dd7ff700> = Field::new(16);
    pub const F_ENTRYCOUNT: Field<u32> = Field::new(32);
    pub const F_CONTENTIONCOUNT: Field<u32> = Field::new(36);
    pub const F_FLAGS: Field<u32> = Field::new(40);
    pub const F_CREATORBACKTRACEINDEXHIGH: Field<u16> = Field::new(44);
    pub const F_SPAREWORD: Field<u16> = Field::new(46);
}

/// struct IErrorInfo: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7a75b1cebcfc {
    pub bytes: [u8; 8],
}
impl Wire for t_7a75b1cebcfc {
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
impl t_7a75b1cebcfc {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<Unknown>> = Field::new(0);
}

/// struct IErrorInfoVtbl: 64 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_85e75021d649 {
    pub bytes: [u8; 64],
}
impl Wire for t_85e75021d649 {
    const SIZE: usize = 64;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_85e75021d649 {
    pub const SIZE: usize = 64;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_GETGUID: Field<CodeAddress> = Field::new(24);
    pub const F_GETSOURCE: Field<CodeAddress> = Field::new(32);
    pub const F_GETDESCRIPTION: Field<CodeAddress> = Field::new(40);
    pub const F_GETHELPFILE: Field<CodeAddress> = Field::new(48);
    pub const F_GETHELPCONTEXT: Field<CodeAddress> = Field::new(56);
}

/// struct _Cvtvec: 44 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_07c238eba542 {
    pub bytes: [u8; 44],
}
impl Wire for t_07c238eba542 {
    const SIZE: usize = 44;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_07c238eba542 {
    pub const SIZE: usize = 44;
    pub const ALIGN: usize = 4;
    pub const F__PAGE: Field<u32> = Field::new(0);
    pub const F__MBCURMAX: Field<u32> = Field::new(4);
    pub const F__ISCLOCALE: Field<i32> = Field::new(8);
    pub const F__ISLEADBYTE: Field<[u8; 32]> = Field::new(12);
}

/// struct _s_ThrowInfo: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_752b694a7b73 {
    pub bytes: [u8; 16],
}
impl Wire for t_752b694a7b73 {
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
impl t_752b694a7b73 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_ATTRIBUTES: Field<u32> = Field::new(0);
    pub const F_PMFNUNWIND: Field<i32> = Field::new(4);
    pub const F_PFORWARDCOMPAT: Field<i32> = Field::new(8);
    pub const F_PCATCHABLETYPEARRAY: Field<i32> = Field::new(12);
}

/// struct TypeDescriptor: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_085d0cfce539 {
    pub bytes: [u8; 16],
}
impl Wire for t_085d0cfce539 {
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
impl t_085d0cfce539 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_PVFTABLE: Field<Address<Unknown>> = Field::new(0);
    pub const F_SPARE: Field<Address<Unknown>> = Field::new(8);
    pub const F_NAME: Field<[i8; 0]> = Field::new(16);
}

/// struct tagMOUSEINPUT: 32 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_8dd84e59b4c1 {
    pub bytes: [u8; 32],
}
impl Wire for t_8dd84e59b4c1 {
    const SIZE: usize = 32;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_8dd84e59b4c1 {
    pub const SIZE: usize = 32;
    pub const ALIGN: usize = 8;
    pub const F_DX: Field<i32> = Field::new(0);
    pub const F_DY: Field<i32> = Field::new(4);
    pub const F_MOUSEDATA: Field<u32> = Field::new(8);
    pub const F_DWFLAGS: Field<u32> = Field::new(12);
    pub const F_TIME: Field<u32> = Field::new(16);
    pub const F_DWEXTRAINFO: Field<u64> = Field::new(24);
}

/// struct tagKEYBDINPUT: 24 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_0febd922c66b {
    pub bytes: [u8; 24],
}
impl Wire for t_0febd922c66b {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_0febd922c66b {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 8;
    pub const F_WVK: Field<u16> = Field::new(0);
    pub const F_WSCAN: Field<u16> = Field::new(2);
    pub const F_DWFLAGS: Field<u32> = Field::new(4);
    pub const F_TIME: Field<u32> = Field::new(8);
    pub const F_DWEXTRAINFO: Field<u64> = Field::new(16);
}

/// struct tagHARDWAREINPUT: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_0cd516fda9fa {
    pub bytes: [u8; 8],
}
impl Wire for t_0cd516fda9fa {
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
impl t_0cd516fda9fa {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_UMSG: Field<u32> = Field::new(0);
    pub const F_WPARAML: Field<u16> = Field::new(4);
    pub const F_WPARAMH: Field<u16> = Field::new(6);
}

/// union _union_881: 32 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_819011ce0501 {
    pub bytes: [u8; 32],
}
impl Wire for t_819011ce0501 {
    const SIZE: usize = 32;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_819011ce0501 {
    pub const SIZE: usize = 32;
    pub const ALIGN: usize = 8;
    pub const F_MI: Field<t_8dd84e59b4c1> = Field::new(0);
    pub const F_KI: Field<t_0febd922c66b> = Field::new(0);
    pub const F_HI: Field<t_0cd516fda9fa> = Field::new(0);
}

/// struct tagINPUT: 40 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7b839c86f896 {
    pub bytes: [u8; 40],
}
impl Wire for t_7b839c86f896 {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_7b839c86f896 {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 8;
    pub const F_TYPE: Field<u32> = Field::new(0);
    pub const F_FIELD1_0X8: Field<t_819011ce0501> = Field::new(8);
}

/// struct HBRUSH__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_3981d9e73c1e {
    pub bytes: [u8; 4],
}
impl Wire for t_3981d9e73c1e {
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
impl t_3981d9e73c1e {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct HICON__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_06a2c1deaf59 {
    pub bytes: [u8; 4],
}
impl Wire for t_06a2c1deaf59 {
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
impl t_06a2c1deaf59 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct tagWNDCLASSW: 72 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_237fe8721873 {
    pub bytes: [u8; 72],
}
impl Wire for t_237fe8721873 {
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
impl t_237fe8721873 {
    pub const SIZE: usize = 72;
    pub const ALIGN: usize = 8;
    pub const F_STYLE: Field<u32> = Field::new(0);
    pub const F_LPFNWNDPROC: Field<CodeAddress> = Field::new(8);
    pub const F_CBCLSEXTRA: Field<i32> = Field::new(16);
    pub const F_CBWNDEXTRA: Field<i32> = Field::new(20);
    pub const F_HINSTANCE: Field<Address<Unknown>> = Field::new(24);
    pub const F_HICON: Field<Address<t_06a2c1deaf59>> = Field::new(32);
    pub const F_HCURSOR: Field<Address<t_06a2c1deaf59>> = Field::new(40);
    pub const F_HBRBACKGROUND: Field<Address<t_3981d9e73c1e>> = Field::new(48);
    pub const F_LPSZMENUNAME: Field<Address<i16>> = Field::new(56);
    pub const F_LPSZCLASSNAME: Field<Address<i16>> = Field::new(64);
}

/// struct HINSTANCE__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4c0908fcb2fb {
    pub bytes: [u8; 4],
}
impl Wire for t_4c0908fcb2fb {
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
impl t_4c0908fcb2fb {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct HWND__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_c6339fcec9fa {
    pub bytes: [u8; 4],
}
impl Wire for t_c6339fcec9fa {
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
impl t_c6339fcec9fa {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct tagPOINT: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_fd609cf74451 {
    pub bytes: [u8; 8],
}
impl Wire for t_fd609cf74451 {
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
impl t_fd609cf74451 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_X: Field<i32> = Field::new(0);
    pub const F_Y: Field<i32> = Field::new(4);
}

/// struct tagMSG: 48 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ac6f275477d5 {
    pub bytes: [u8; 48],
}
impl Wire for t_ac6f275477d5 {
    const SIZE: usize = 48;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_ac6f275477d5 {
    pub const SIZE: usize = 48;
    pub const ALIGN: usize = 8;
    pub const F_HWND: Field<Address<t_c6339fcec9fa>> = Field::new(0);
    pub const F_MESSAGE: Field<u32> = Field::new(8);
    pub const F_WPARAM: Field<u64> = Field::new(16);
    pub const F_LPARAM: Field<i64> = Field::new(24);
    pub const F_TIME: Field<u32> = Field::new(32);
    pub const F_PT: Field<t_fd609cf74451> = Field::new(36);
}

/// struct tagRECT: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_272e88af6e21 {
    pub bytes: [u8; 16],
}
impl Wire for t_272e88af6e21 {
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
impl t_272e88af6e21 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_LEFT: Field<i32> = Field::new(0);
    pub const F_TOP: Field<i32> = Field::new(4);
    pub const F_RIGHT: Field<i32> = Field::new(8);
    pub const F_BOTTOM: Field<i32> = Field::new(12);
}

/// struct tagMONITORINFO: 40 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_9b8d66b76bc0 {
    pub bytes: [u8; 40],
}
impl Wire for t_9b8d66b76bc0 {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_9b8d66b76bc0 {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 4;
    pub const F_CBSIZE: Field<u32> = Field::new(0);
    pub const F_RCMONITOR: Field<t_272e88af6e21> = Field::new(4);
    pub const F_RCWORK: Field<t_272e88af6e21> = Field::new(20);
    pub const F_DWFLAGS: Field<u32> = Field::new(36);
}

/// struct IMAGE_DOS_HEADER: 128 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4a3399835aa5 {
    pub bytes: [u8; 128],
}
impl Wire for t_4a3399835aa5 {
    const SIZE: usize = 128;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4a3399835aa5 {
    pub const SIZE: usize = 128;
    pub const ALIGN: usize = 4;
    pub const F_E_MAGIC: Field<[i8; 2]> = Field::new(0);
    pub const F_E_CBLP: Field<u16> = Field::new(2);
    pub const F_E_CP: Field<u16> = Field::new(4);
    pub const F_E_CRLC: Field<u16> = Field::new(6);
    pub const F_E_CPARHDR: Field<u16> = Field::new(8);
    pub const F_E_MINALLOC: Field<u16> = Field::new(10);
    pub const F_E_MAXALLOC: Field<u16> = Field::new(12);
    pub const F_E_SS: Field<u16> = Field::new(14);
    pub const F_E_SP: Field<u16> = Field::new(16);
    pub const F_E_CSUM: Field<u16> = Field::new(18);
    pub const F_E_IP: Field<u16> = Field::new(20);
    pub const F_E_CS: Field<u16> = Field::new(22);
    pub const F_E_LFARLC: Field<u16> = Field::new(24);
    pub const F_E_OVNO: Field<u16> = Field::new(26);
    pub const F_E_RES: Field<[u16; 4]> = Field::new(28);
    pub const F_E_OEMID: Field<u16> = Field::new(36);
    pub const F_E_OEMINFO: Field<u16> = Field::new(38);
    pub const F_E_RES2: Field<[u16; 10]> = Field::new(40);
    pub const F_E_LFANEW: Field<u32> = Field::new(60);
    pub const F_E_PROGRAM: Field<[u8; 64]> = Field::new(64);
}

/// struct _POINTL: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_de4125428281 {
    pub bytes: [u8; 8],
}
impl Wire for t_de4125428281 {
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
impl t_de4125428281 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_X: Field<i32> = Field::new(0);
    pub const F_Y: Field<i32> = Field::new(4);
}

/// struct _PRINTER_DEFAULTSW: 24 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5fd3760ac5ed {
    pub bytes: [u8; 24],
}
impl Wire for t_5fd3760ac5ed {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_5fd3760ac5ed {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 8;
    pub const F_PDATATYPE: Field<Address<i16>> = Field::new(0);
    pub const F_PDEVMODE: Field<Address<Unknown>> = Field::new(8);
    pub const F_DESIREDACCESS: Field<u32> = Field::new(16);
}

/// union _union_685: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_28435f27627d {
    pub bytes: [u8; 4],
}
impl Wire for t_28435f27627d {
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
impl t_28435f27627d {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_DMDISPLAYFLAGS: Field<u32> = Field::new(0);
    pub const F_DMNUP: Field<u32> = Field::new(0);
}

/// struct _struct_684: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_fa988a243f90 {
    pub bytes: [u8; 16],
}
impl Wire for t_fa988a243f90 {
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
impl t_fa988a243f90 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_DMPOSITION: Field<t_de4125428281> = Field::new(0);
    pub const F_DMDISPLAYORIENTATION: Field<u32> = Field::new(8);
    pub const F_DMDISPLAYFIXEDOUTPUT: Field<u32> = Field::new(12);
}

/// struct _struct_683: 16 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5484efe72ed7 {
    pub bytes: [u8; 16],
}
impl Wire for t_5484efe72ed7 {
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
impl t_5484efe72ed7 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 2;
    pub const F_DMORIENTATION: Field<i16> = Field::new(0);
    pub const F_DMPAPERSIZE: Field<i16> = Field::new(2);
    pub const F_DMPAPERLENGTH: Field<i16> = Field::new(4);
    pub const F_DMPAPERWIDTH: Field<i16> = Field::new(6);
    pub const F_DMSCALE: Field<i16> = Field::new(8);
    pub const F_DMCOPIES: Field<i16> = Field::new(10);
    pub const F_DMDEFAULTSOURCE: Field<i16> = Field::new(12);
    pub const F_DMPRINTQUALITY: Field<i16> = Field::new(14);
}

/// union _union_682: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4e7262b45ab9 {
    pub bytes: [u8; 16],
}
impl Wire for t_4e7262b45ab9 {
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
impl t_4e7262b45ab9 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_FIELD0: Field<t_5484efe72ed7> = Field::new(0);
    pub const F_FIELD1: Field<t_fa988a243f90> = Field::new(0);
}

/// struct _devicemodeW: 220 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_04d26082a7a2 {
    pub bytes: [u8; 220],
}
impl Wire for t_04d26082a7a2 {
    const SIZE: usize = 220;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_04d26082a7a2 {
    pub const SIZE: usize = 220;
    pub const ALIGN: usize = 4;
    pub const F_DMDEVICENAME: Field<[i16; 32]> = Field::new(0);
    pub const F_DMSPECVERSION: Field<u16> = Field::new(64);
    pub const F_DMDRIVERVERSION: Field<u16> = Field::new(66);
    pub const F_DMSIZE: Field<u16> = Field::new(68);
    pub const F_DMDRIVEREXTRA: Field<u16> = Field::new(70);
    pub const F_DMFIELDS: Field<u32> = Field::new(72);
    pub const F_FIELD6_0X4C: Field<t_4e7262b45ab9> = Field::new(76);
    pub const F_DMCOLOR: Field<i16> = Field::new(92);
    pub const F_DMDUPLEX: Field<i16> = Field::new(94);
    pub const F_DMYRESOLUTION: Field<i16> = Field::new(96);
    pub const F_DMTTOPTION: Field<i16> = Field::new(98);
    pub const F_DMCOLLATE: Field<i16> = Field::new(100);
    pub const F_DMFORMNAME: Field<[i16; 32]> = Field::new(102);
    pub const F_DMLOGPIXELS: Field<u16> = Field::new(166);
    pub const F_DMBITSPERPEL: Field<u32> = Field::new(168);
    pub const F_DMPELSWIDTH: Field<u32> = Field::new(172);
    pub const F_DMPELSHEIGHT: Field<u32> = Field::new(176);
    pub const F_FIELD17_0XB4: Field<t_28435f27627d> = Field::new(180);
    pub const F_DMDISPLAYFREQUENCY: Field<u32> = Field::new(184);
    pub const F_DMICMMETHOD: Field<u32> = Field::new(188);
    pub const F_DMICMINTENT: Field<u32> = Field::new(192);
    pub const F_DMMEDIATYPE: Field<u32> = Field::new(196);
    pub const F_DMDITHERTYPE: Field<u32> = Field::new(200);
    pub const F_DMRESERVED1: Field<u32> = Field::new(204);
    pub const F_DMRESERVED2: Field<u32> = Field::new(208);
    pub const F_DMPANNINGWIDTH: Field<u32> = Field::new(212);
    pub const F_DMPANNINGHEIGHT: Field<u32> = Field::new(216);
}

/// struct GuardCfgTableEntry: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ce36b2111292 {
    pub bytes: [u8; 4],
}
impl Wire for t_ce36b2111292 {
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
impl t_ce36b2111292 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_OFFSET: Field<u32> = Field::new(0);
}

/// struct IAdviseSink: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_482d8d0b333d {
    pub bytes: [u8; 8],
}
impl Wire for t_482d8d0b333d {
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
impl t_482d8d0b333d {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<Unknown>> = Field::new(0);
}

/// struct IAdviseSinkVtbl: 64 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d5f8803400c9 {
    pub bytes: [u8; 64],
}
impl Wire for t_d5f8803400c9 {
    const SIZE: usize = 64;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d5f8803400c9 {
    pub const SIZE: usize = 64;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_ONDATACHANGE: Field<CodeAddress> = Field::new(24);
    pub const F_ONVIEWCHANGE: Field<CodeAddress> = Field::new(32);
    pub const F_ONRENAME: Field<CodeAddress> = Field::new(40);
    pub const F_ONSAVE: Field<CodeAddress> = Field::new(48);
    pub const F_ONCLOSE: Field<CodeAddress> = Field::new(56);
}

/// struct IStreamVtbl: 112 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_03d4f2b2423d {
    pub bytes: [u8; 112],
}
impl Wire for t_03d4f2b2423d {
    const SIZE: usize = 112;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_03d4f2b2423d {
    pub const SIZE: usize = 112;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_READ: Field<CodeAddress> = Field::new(24);
    pub const F_WRITE: Field<CodeAddress> = Field::new(32);
    pub const F_SEEK: Field<CodeAddress> = Field::new(40);
    pub const F_SETSIZE: Field<CodeAddress> = Field::new(48);
    pub const F_COPYTO: Field<CodeAddress> = Field::new(56);
    pub const F_COMMIT: Field<CodeAddress> = Field::new(64);
    pub const F_REVERT: Field<CodeAddress> = Field::new(72);
    pub const F_LOCKREGION: Field<CodeAddress> = Field::new(80);
    pub const F_UNLOCKREGION: Field<CodeAddress> = Field::new(88);
    pub const F_STAT: Field<CodeAddress> = Field::new(96);
    pub const F_CLONE: Field<CodeAddress> = Field::new(104);
}

/// struct tagFORMATETC: 32 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_fa67fefc92a1 {
    pub bytes: [u8; 32],
}
impl Wire for t_fa67fefc92a1 {
    const SIZE: usize = 32;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_fa67fefc92a1 {
    pub const SIZE: usize = 32;
    pub const ALIGN: usize = 8;
    pub const F_CFFORMAT: Field<u16> = Field::new(0);
    pub const F_PTD: Field<Address<Unknown>> = Field::new(8);
    pub const F_DWASPECT: Field<u32> = Field::new(16);
    pub const F_LINDEX: Field<i32> = Field::new(20);
    pub const F_TYMED: Field<u32> = Field::new(24);
}

/// union _union_2284: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bb76df3c134a {
    pub bytes: [u8; 8],
}
impl Wire for t_bb76df3c134a {
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
impl t_bb76df3c134a {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_HBITMAP: Field<Address<Unknown>> = Field::new(0);
    pub const F_HMETAFILEPICT: Field<Address<Unknown>> = Field::new(0);
    pub const F_HENHMETAFILE: Field<Address<Unknown>> = Field::new(0);
    pub const F_HGLOBAL: Field<Address<Unknown>> = Field::new(0);
    pub const F_LPSZFILENAME: Field<Address<i16>> = Field::new(0);
    pub const F_PSTM: Field<Address<Unknown>> = Field::new(0);
    pub const F_PSTG: Field<Address<Unknown>> = Field::new(0);
}

/// struct IEnumStringVtbl: 56 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_8de782ce8af7 {
    pub bytes: [u8; 56],
}
impl Wire for t_8de782ce8af7 {
    const SIZE: usize = 56;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_8de782ce8af7 {
    pub const SIZE: usize = 56;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_NEXT: Field<CodeAddress> = Field::new(24);
    pub const F_SKIP: Field<CodeAddress> = Field::new(32);
    pub const F_RESET: Field<CodeAddress> = Field::new(40);
    pub const F_CLONE: Field<CodeAddress> = Field::new(48);
}

/// struct IStream: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_6f44067859e5 {
    pub bytes: [u8; 8],
}
impl Wire for t_6f44067859e5 {
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
impl t_6f44067859e5 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_03d4f2b2423d>> = Field::new(0);
}

/// struct _struct_20: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_17463d5dad35 {
    pub bytes: [u8; 8],
}
impl Wire for t_17463d5dad35 {
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
impl t_17463d5dad35 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_LOWPART: Field<u32> = Field::new(0);
    pub const F_HIGHPART: Field<i32> = Field::new(4);
}

/// struct _struct_19: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_6d69cbb7374d {
    pub bytes: [u8; 8],
}
impl Wire for t_6d69cbb7374d {
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
impl t_6d69cbb7374d {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_LOWPART: Field<u32> = Field::new(0);
    pub const F_HIGHPART: Field<i32> = Field::new(4);
}

/// union _LARGE_INTEGER: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_627cf47e8ca7 {
    pub bytes: [u8; 8],
}
impl Wire for t_627cf47e8ca7 {
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
impl t_627cf47e8ca7 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_S: Field<t_6d69cbb7374d> = Field::new(0);
    pub const F_U: Field<t_17463d5dad35> = Field::new(0);
    pub const F_QUADPART: Field<i64> = Field::new(0);
}

/// struct IMoniker: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_32a6939c8586 {
    pub bytes: [u8; 8],
}
impl Wire for t_32a6939c8586 {
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
impl t_32a6939c8586 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<Unknown>> = Field::new(0);
}

/// struct IEnumString: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bce96f9772e3 {
    pub bytes: [u8; 8],
}
impl Wire for t_bce96f9772e3 {
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
impl t_bce96f9772e3 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_8de782ce8af7>> = Field::new(0);
}

/// struct IEnumMonikerVtbl: 56 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_c3fd39d62aff {
    pub bytes: [u8; 56],
}
impl Wire for t_c3fd39d62aff {
    const SIZE: usize = 56;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_c3fd39d62aff {
    pub const SIZE: usize = 56;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_NEXT: Field<CodeAddress> = Field::new(24);
    pub const F_SKIP: Field<CodeAddress> = Field::new(32);
    pub const F_RESET: Field<CodeAddress> = Field::new(40);
    pub const F_CLONE: Field<CodeAddress> = Field::new(48);
}

/// struct tagSTGMEDIUM: 24 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_30140b67a92e {
    pub bytes: [u8; 24],
}
impl Wire for t_30140b67a92e {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_30140b67a92e {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 8;
    pub const F_TYMED: Field<u32> = Field::new(0);
    pub const F_U: Field<t_bb76df3c134a> = Field::new(8);
    pub const F_PUNKFORRELEASE: Field<Address<Unknown>> = Field::new(16);
}

/// struct tagBIND_OPTS: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_092ddfd804eb {
    pub bytes: [u8; 16],
}
impl Wire for t_092ddfd804eb {
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
impl t_092ddfd804eb {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_CBSTRUCT: Field<u32> = Field::new(0);
    pub const F_GRFFLAGS: Field<u32> = Field::new(4);
    pub const F_GRFMODE: Field<u32> = Field::new(8);
    pub const F_DWTICKCOUNTDEADLINE: Field<u32> = Field::new(12);
}

/// struct _struct_22: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ca9b7444fc70 {
    pub bytes: [u8; 8],
}
impl Wire for t_ca9b7444fc70 {
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
impl t_ca9b7444fc70 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_LOWPART: Field<u32> = Field::new(0);
    pub const F_HIGHPART: Field<u32> = Field::new(4);
}

/// struct _struct_23: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_1801dc4830e5 {
    pub bytes: [u8; 8],
}
impl Wire for t_1801dc4830e5 {
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
impl t_1801dc4830e5 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_LOWPART: Field<u32> = Field::new(0);
    pub const F_HIGHPART: Field<u32> = Field::new(4);
}

/// union _ULARGE_INTEGER: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ec87f6a0d8aa {
    pub bytes: [u8; 8],
}
impl Wire for t_ec87f6a0d8aa {
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
impl t_ec87f6a0d8aa {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_S: Field<t_ca9b7444fc70> = Field::new(0);
    pub const F_U: Field<t_1801dc4830e5> = Field::new(0);
    pub const F_QUADPART: Field<u64> = Field::new(0);
}

/// struct tagDVTARGETDEVICE: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_2b5c2f17fed6 {
    pub bytes: [u8; 16],
}
impl Wire for t_2b5c2f17fed6 {
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
impl t_2b5c2f17fed6 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_TDSIZE: Field<u32> = Field::new(0);
    pub const F_TDDRIVERNAMEOFFSET: Field<u16> = Field::new(4);
    pub const F_TDDEVICENAMEOFFSET: Field<u16> = Field::new(6);
    pub const F_TDPORTNAMEOFFSET: Field<u16> = Field::new(8);
    pub const F_TDEXTDEVMODEOFFSET: Field<u16> = Field::new(10);
    pub const F_TDDATA: Field<[u8; 1]> = Field::new(12);
}

/// struct IBindCtx: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ac520b3f9a6b {
    pub bytes: [u8; 8],
}
impl Wire for t_ac520b3f9a6b {
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
impl t_ac520b3f9a6b {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<Unknown>> = Field::new(0);
}

/// struct IBindCtxVtbl: 104 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a485dad48787 {
    pub bytes: [u8; 104],
}
impl Wire for t_a485dad48787 {
    const SIZE: usize = 104;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a485dad48787 {
    pub const SIZE: usize = 104;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_REGISTEROBJECTBOUND: Field<CodeAddress> = Field::new(24);
    pub const F_REVOKEOBJECTBOUND: Field<CodeAddress> = Field::new(32);
    pub const F_RELEASEBOUNDOBJECTS: Field<CodeAddress> = Field::new(40);
    pub const F_SETBINDOPTIONS: Field<CodeAddress> = Field::new(48);
    pub const F_GETBINDOPTIONS: Field<CodeAddress> = Field::new(56);
    pub const F_GETRUNNINGOBJECTTABLE: Field<CodeAddress> = Field::new(64);
    pub const F_REGISTEROBJECTPARAM: Field<CodeAddress> = Field::new(72);
    pub const F_GETOBJECTPARAM: Field<CodeAddress> = Field::new(80);
    pub const F_ENUMOBJECTPARAM: Field<CodeAddress> = Field::new(88);
    pub const F_REVOKEOBJECTPARAM: Field<CodeAddress> = Field::new(96);
}

/// struct IRunningObjectTableVtbl: 80 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bb8a112e2f00 {
    pub bytes: [u8; 80],
}
impl Wire for t_bb8a112e2f00 {
    const SIZE: usize = 80;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_bb8a112e2f00 {
    pub const SIZE: usize = 80;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_REGISTER: Field<CodeAddress> = Field::new(24);
    pub const F_REVOKE: Field<CodeAddress> = Field::new(32);
    pub const F_ISRUNNING: Field<CodeAddress> = Field::new(40);
    pub const F_GETOBJECTA: Field<CodeAddress> = Field::new(48);
    pub const F_NOTECHANGETIME: Field<CodeAddress> = Field::new(56);
    pub const F_GETTIMEOFLASTCHANGE: Field<CodeAddress> = Field::new(64);
    pub const F_ENUMRUNNING: Field<CodeAddress> = Field::new(72);
}

/// struct IStorageVtbl: 144 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ef59add06b29 {
    pub bytes: [u8; 144],
}
impl Wire for t_ef59add06b29 {
    const SIZE: usize = 144;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_ef59add06b29 {
    pub const SIZE: usize = 144;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_CREATESTREAM: Field<CodeAddress> = Field::new(24);
    pub const F_OPENSTREAM: Field<CodeAddress> = Field::new(32);
    pub const F_CREATESTORAGE: Field<CodeAddress> = Field::new(40);
    pub const F_OPENSTORAGE: Field<CodeAddress> = Field::new(48);
    pub const F_COPYTO: Field<CodeAddress> = Field::new(56);
    pub const F_MOVEELEMENTTO: Field<CodeAddress> = Field::new(64);
    pub const F_COMMIT: Field<CodeAddress> = Field::new(72);
    pub const F_REVERT: Field<CodeAddress> = Field::new(80);
    pub const F_ENUMELEMENTS: Field<CodeAddress> = Field::new(88);
    pub const F_DESTROYELEMENT: Field<CodeAddress> = Field::new(96);
    pub const F_RENAMEELEMENT: Field<CodeAddress> = Field::new(104);
    pub const F_SETELEMENTTIMES: Field<CodeAddress> = Field::new(112);
    pub const F_SETCLASS: Field<CodeAddress> = Field::new(120);
    pub const F_SETSTATEBITS: Field<CodeAddress> = Field::new(128);
    pub const F_STAT: Field<CodeAddress> = Field::new(136);
}

/// struct IMonikerVtbl: 184 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a0474f8c0811 {
    pub bytes: [u8; 184],
}
impl Wire for t_a0474f8c0811 {
    const SIZE: usize = 184;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a0474f8c0811 {
    pub const SIZE: usize = 184;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_GETCLASSID: Field<CodeAddress> = Field::new(24);
    pub const F_ISDIRTY: Field<CodeAddress> = Field::new(32);
    pub const F_LOAD: Field<CodeAddress> = Field::new(40);
    pub const F_SAVE: Field<CodeAddress> = Field::new(48);
    pub const F_GETSIZEMAX: Field<CodeAddress> = Field::new(56);
    pub const F_BINDTOOBJECT: Field<CodeAddress> = Field::new(64);
    pub const F_BINDTOSTORAGE: Field<CodeAddress> = Field::new(72);
    pub const F_REDUCE: Field<CodeAddress> = Field::new(80);
    pub const F_COMPOSEWITH: Field<CodeAddress> = Field::new(88);
    pub const F_ENUM: Field<CodeAddress> = Field::new(96);
    pub const F_ISEQUAL: Field<CodeAddress> = Field::new(104);
    pub const F_HASH: Field<CodeAddress> = Field::new(112);
    pub const F_ISRUNNING: Field<CodeAddress> = Field::new(120);
    pub const F_GETTIMEOFLASTCHANGE: Field<CodeAddress> = Field::new(128);
    pub const F_INVERSE: Field<CodeAddress> = Field::new(136);
    pub const F_COMMONPREFIXWITH: Field<CodeAddress> = Field::new(144);
    pub const F_RELATIVEPATHTO: Field<CodeAddress> = Field::new(152);
    pub const F_GETDISPLAYNAME: Field<CodeAddress> = Field::new(160);
    pub const F_PARSEDISPLAYNAME: Field<CodeAddress> = Field::new(168);
    pub const F_ISSYSTEMMONIKER: Field<CodeAddress> = Field::new(176);
}

/// struct IStorage: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_633e66262301 {
    pub bytes: [u8; 8],
}
impl Wire for t_633e66262301 {
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
impl t_633e66262301 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_ef59add06b29>> = Field::new(0);
}

/// struct IEnumSTATSTGVtbl: 56 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_f187f44ecbed {
    pub bytes: [u8; 56],
}
impl Wire for t_f187f44ecbed {
    const SIZE: usize = 56;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_f187f44ecbed {
    pub const SIZE: usize = 56;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_NEXT: Field<CodeAddress> = Field::new(24);
    pub const F_SKIP: Field<CodeAddress> = Field::new(32);
    pub const F_RESET: Field<CodeAddress> = Field::new(40);
    pub const F_CLONE: Field<CodeAddress> = Field::new(48);
}

/// struct HBITMAP__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d3401f1f0509 {
    pub bytes: [u8; 4],
}
impl Wire for t_d3401f1f0509 {
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
impl t_d3401f1f0509 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct IEnumSTATSTG: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_b97acc974f3f {
    pub bytes: [u8; 8],
}
impl Wire for t_b97acc974f3f {
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
impl t_b97acc974f3f {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_f187f44ecbed>> = Field::new(0);
}

/// struct IRunningObjectTable: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_409bfe6e453d {
    pub bytes: [u8; 8],
}
impl Wire for t_409bfe6e453d {
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
impl t_409bfe6e453d {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_bb8a112e2f00>> = Field::new(0);
}

/// struct IUnknownVtbl: 24 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_89331bec4bba {
    pub bytes: [u8; 24],
}
impl Wire for t_89331bec4bba {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_89331bec4bba {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
}

/// struct IUnknown: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_39472191c743 {
    pub bytes: [u8; 8],
}
impl Wire for t_39472191c743 {
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
impl t_39472191c743 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_89331bec4bba>> = Field::new(0);
}

/// struct HENHMETAFILE__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_b9e6eda47c42 {
    pub bytes: [u8; 4],
}
impl Wire for t_b9e6eda47c42 {
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
impl t_b9e6eda47c42 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct IEnumMoniker: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_c9c365e9e75d {
    pub bytes: [u8; 8],
}
impl Wire for t_c9c365e9e75d {
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
impl t_c9c365e9e75d {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_c3fd39d62aff>> = Field::new(0);
}

/// struct IEnumSTATDATA: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_525edec8d4e7 {
    pub bytes: [u8; 8],
}
impl Wire for t_525edec8d4e7 {
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
impl t_525edec8d4e7 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<Unknown>> = Field::new(0);
}

/// struct IEnumSTATDATAVtbl: 56 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4a87cf9300df {
    pub bytes: [u8; 56],
}
impl Wire for t_4a87cf9300df {
    const SIZE: usize = 56;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4a87cf9300df {
    pub const SIZE: usize = 56;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_NEXT: Field<CodeAddress> = Field::new(24);
    pub const F_SKIP: Field<CodeAddress> = Field::new(32);
    pub const F_RESET: Field<CodeAddress> = Field::new(40);
    pub const F_CLONE: Field<CodeAddress> = Field::new(48);
}

/// struct tagSTATDATA: 56 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_decbfceef172 {
    pub bytes: [u8; 56],
}
impl Wire for t_decbfceef172 {
    const SIZE: usize = 56;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_decbfceef172 {
    pub const SIZE: usize = 56;
    pub const ALIGN: usize = 8;
    pub const F_FORMATETC: Field<t_fa67fefc92a1> = Field::new(0);
    pub const F_ADVF: Field<u32> = Field::new(32);
    pub const F_PADVSINK: Field<Address<t_482d8d0b333d>> = Field::new(40);
    pub const F_DWCONNECTION: Field<u32> = Field::new(48);
}

/// struct IEnumFORMATETCVtbl: 56 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7cb6d5f557c0 {
    pub bytes: [u8; 56],
}
impl Wire for t_7cb6d5f557c0 {
    const SIZE: usize = 56;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_7cb6d5f557c0 {
    pub const SIZE: usize = 56;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_NEXT: Field<CodeAddress> = Field::new(24);
    pub const F_SKIP: Field<CodeAddress> = Field::new(32);
    pub const F_RESET: Field<CodeAddress> = Field::new(40);
    pub const F_CLONE: Field<CodeAddress> = Field::new(48);
}

/// struct IDataObject: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_69e12c459a52 {
    pub bytes: [u8; 8],
}
impl Wire for t_69e12c459a52 {
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
impl t_69e12c459a52 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<Unknown>> = Field::new(0);
}

/// struct IDataObjectVtbl: 96 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_82730d4ea9ba {
    pub bytes: [u8; 96],
}
impl Wire for t_82730d4ea9ba {
    const SIZE: usize = 96;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_82730d4ea9ba {
    pub const SIZE: usize = 96;
    pub const ALIGN: usize = 8;
    pub const F_QUERYINTERFACE: Field<CodeAddress> = Field::new(0);
    pub const F_ADDREF: Field<CodeAddress> = Field::new(8);
    pub const F_RELEASE: Field<CodeAddress> = Field::new(16);
    pub const F_GETDATA: Field<CodeAddress> = Field::new(24);
    pub const F_GETDATAHERE: Field<CodeAddress> = Field::new(32);
    pub const F_QUERYGETDATA: Field<CodeAddress> = Field::new(40);
    pub const F_GETCANONICALFORMATETC: Field<CodeAddress> = Field::new(48);
    pub const F_SETDATA: Field<CodeAddress> = Field::new(56);
    pub const F_ENUMFORMATETC: Field<CodeAddress> = Field::new(64);
    pub const F_DADVISE: Field<CodeAddress> = Field::new(72);
    pub const F_DUNADVISE: Field<CodeAddress> = Field::new(80);
    pub const F_ENUMDADVISE: Field<CodeAddress> = Field::new(88);
}

/// struct IEnumFORMATETC: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d30ceb7d3a3d {
    pub bytes: [u8; 8],
}
impl Wire for t_d30ceb7d3a3d {
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
impl t_d30ceb7d3a3d {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_LPVTBL: Field<Address<t_7cb6d5f557c0>> = Field::new(0);
}

/// struct tagPRINTPAGERANGE: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5c940ae036a8 {
    pub bytes: [u8; 8],
}
impl Wire for t_5c940ae036a8 {
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
impl t_5c940ae036a8 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_NFROMPAGE: Field<u32> = Field::new(0);
    pub const F_NTOPAGE: Field<u32> = Field::new(4);
}

/// struct HDC__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a46a405b9465 {
    pub bytes: [u8; 4],
}
impl Wire for t_a46a405b9465 {
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
impl t_a46a405b9465 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct tagPDEXW: 136 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_300f5dc89818 {
    pub bytes: [u8; 136],
}
impl Wire for t_300f5dc89818 {
    const SIZE: usize = 136;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_300f5dc89818 {
    pub const SIZE: usize = 136;
    pub const ALIGN: usize = 8;
    pub const F_LSTRUCTSIZE: Field<u32> = Field::new(0);
    pub const F_HWNDOWNER: Field<Address<t_c6339fcec9fa>> = Field::new(8);
    pub const F_HDEVMODE: Field<Address<Unknown>> = Field::new(16);
    pub const F_HDEVNAMES: Field<Address<Unknown>> = Field::new(24);
    pub const F_HDC: Field<Address<t_a46a405b9465>> = Field::new(32);
    pub const F_FLAGS: Field<u32> = Field::new(40);
    pub const F_FLAGS2: Field<u32> = Field::new(44);
    pub const F_EXCLUSIONFLAGS: Field<u32> = Field::new(48);
    pub const F_NPAGERANGES: Field<u32> = Field::new(52);
    pub const F_NMAXPAGERANGES: Field<u32> = Field::new(56);
    pub const F_LPPAGERANGES: Field<Address<t_5c940ae036a8>> = Field::new(64);
    pub const F_NMINPAGE: Field<u32> = Field::new(72);
    pub const F_NMAXPAGE: Field<u32> = Field::new(76);
    pub const F_NCOPIES: Field<u32> = Field::new(80);
    pub const F_HINSTANCE: Field<Address<t_4c0908fcb2fb>> = Field::new(88);
    pub const F_LPPRINTTEMPLATENAME: Field<Address<i16>> = Field::new(96);
    pub const F_LPCALLBACK: Field<Address<t_39472191c743>> = Field::new(104);
    pub const F_NPROPERTYPAGES: Field<u32> = Field::new(112);
    pub const F_LPHPROPERTYPAGES: Field<Address<Address<Unknown>>> = Field::new(120);
    pub const F_NSTARTPAGE: Field<u32> = Field::new(128);
    pub const F_DWRESULTACTION: Field<u32> = Field::new(132);
}

/// struct tagOFNW: 152 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d01078937930 {
    pub bytes: [u8; 152],
}
impl Wire for t_d01078937930 {
    const SIZE: usize = 152;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d01078937930 {
    pub const SIZE: usize = 152;
    pub const ALIGN: usize = 8;
    pub const F_LSTRUCTSIZE: Field<u32> = Field::new(0);
    pub const F_HWNDOWNER: Field<Address<t_c6339fcec9fa>> = Field::new(8);
    pub const F_HINSTANCE: Field<Address<t_4c0908fcb2fb>> = Field::new(16);
    pub const F_LPSTRFILTER: Field<Address<i16>> = Field::new(24);
    pub const F_LPSTRCUSTOMFILTER: Field<Address<i16>> = Field::new(32);
    pub const F_NMAXCUSTFILTER: Field<u32> = Field::new(40);
    pub const F_NFILTERINDEX: Field<u32> = Field::new(44);
    pub const F_LPSTRFILE: Field<Address<i16>> = Field::new(48);
    pub const F_NMAXFILE: Field<u32> = Field::new(56);
    pub const F_LPSTRFILETITLE: Field<Address<i16>> = Field::new(64);
    pub const F_NMAXFILETITLE: Field<u32> = Field::new(72);
    pub const F_LPSTRINITIALDIR: Field<Address<i16>> = Field::new(80);
    pub const F_LPSTRTITLE: Field<Address<i16>> = Field::new(88);
    pub const F_FLAGS: Field<u32> = Field::new(96);
    pub const F_NFILEOFFSET: Field<u16> = Field::new(100);
    pub const F_NFILEEXTENSION: Field<u16> = Field::new(102);
    pub const F_LPSTRDEFEXT: Field<Address<i16>> = Field::new(104);
    pub const F_LCUSTDATA: Field<i64> = Field::new(112);
    pub const F_LPFNHOOK: Field<CodeAddress> = Field::new(120);
    pub const F_LPTEMPLATENAME: Field<Address<i16>> = Field::new(128);
    pub const F_PVRESERVED: Field<Address<Unknown>> = Field::new(136);
    pub const F_DWRESERVED: Field<u32> = Field::new(144);
    pub const F_FLAGSEX: Field<u32> = Field::new(148);
}

/// struct _GUID: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_9bbc0df2157e {
    pub bytes: [u8; 16],
}
impl Wire for t_9bbc0df2157e {
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
impl t_9bbc0df2157e {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_DATA1: Field<u32> = Field::new(0);
    pub const F_DATA2: Field<u16> = Field::new(4);
    pub const F_DATA3: Field<u16> = Field::new(6);
    pub const F_DATA4: Field<[u8; 8]> = Field::new(8);
}

/// struct _DOCINFOW: 40 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7e49763e4bfe {
    pub bytes: [u8; 40],
}
impl Wire for t_7e49763e4bfe {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_7e49763e4bfe {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 8;
    pub const F_CBSIZE: Field<i32> = Field::new(0);
    pub const F_LPSZDOCNAME: Field<Address<i16>> = Field::new(8);
    pub const F_LPSZOUTPUT: Field<Address<i16>> = Field::new(16);
    pub const F_LPSZDATATYPE: Field<Address<i16>> = Field::new(24);
    pub const F_FWTYPE: Field<u32> = Field::new(32);
}

/// struct tagBITMAPINFOHEADER: 40 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_90b4e80c4b85 {
    pub bytes: [u8; 40],
}
impl Wire for t_90b4e80c4b85 {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_90b4e80c4b85 {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 4;
    pub const F_BISIZE: Field<u32> = Field::new(0);
    pub const F_BIWIDTH: Field<i32> = Field::new(4);
    pub const F_BIHEIGHT: Field<i32> = Field::new(8);
    pub const F_BIPLANES: Field<u16> = Field::new(12);
    pub const F_BIBITCOUNT: Field<u16> = Field::new(14);
    pub const F_BICOMPRESSION: Field<u32> = Field::new(16);
    pub const F_BISIZEIMAGE: Field<u32> = Field::new(20);
    pub const F_BIXPELSPERMETER: Field<i32> = Field::new(24);
    pub const F_BIYPELSPERMETER: Field<i32> = Field::new(28);
    pub const F_BICLRUSED: Field<u32> = Field::new(32);
    pub const F_BICLRIMPORTANT: Field<u32> = Field::new(36);
}

/// struct tagRGBQUAD: 4 bytes, source alignment 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_91ac713f053f {
    pub bytes: [u8; 4],
}
impl Wire for t_91ac713f053f {
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
impl t_91ac713f053f {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 1;
    pub const F_RGBBLUE: Field<u8> = Field::new(0);
    pub const F_RGBGREEN: Field<u8> = Field::new(1);
    pub const F_RGBRED: Field<u8> = Field::new(2);
    pub const F_RGBRESERVED: Field<u8> = Field::new(3);
}

/// struct tagBITMAPINFO: 44 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_b094c5277644 {
    pub bytes: [u8; 44],
}
impl Wire for t_b094c5277644 {
    const SIZE: usize = 44;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_b094c5277644 {
    pub const SIZE: usize = 44;
    pub const ALIGN: usize = 4;
    pub const F_BMIHEADER: Field<t_90b4e80c4b85> = Field::new(0);
    pub const F_BMICOLORS: Field<[t_91ac713f053f; 1]> = Field::new(40);
}

/// struct _struct_314: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_15dd9826f96e {
    pub bytes: [u8; 16],
}
impl Wire for t_15dd9826f96e {
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
impl t_15dd9826f96e {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_ALIGNMENT: Field<u64> = Field::new(0);
    pub const F_REGION: Field<u64> = Field::new(8);
}

/// struct _RUNTIME_FUNCTION: 12 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_380b96258bfe {
    pub bytes: [u8; 12],
}
impl Wire for t_380b96258bfe {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_380b96258bfe {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 4;
    pub const F_BEGINADDRESS: Field<u32> = Field::new(0);
    pub const F_ENDADDRESS: Field<u32> = Field::new(4);
    pub const F_UNWINDDATA: Field<u32> = Field::new(8);
}

/// struct _UNWIND_HISTORY_TABLE_ENTRY: 16 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_099cffa171c0 {
    pub bytes: [u8; 16],
}
impl Wire for t_099cffa171c0 {
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
impl t_099cffa171c0 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 8;
    pub const F_IMAGEBASE: Field<u64> = Field::new(0);
    pub const F_FUNCTIONENTRY: Field<Address<t_380b96258bfe>> = Field::new(8);
}

/// struct _struct_62: 128 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_6637dfe39500 {
    pub bytes: [u8; 128],
}
impl Wire for t_6637dfe39500 {
    const SIZE: usize = 128;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_6637dfe39500 {
    pub const SIZE: usize = 128;
    pub const ALIGN: usize = 8;
    pub const F_XMM0: Field<Address<t_cbc0908ab753>> = Field::new(0);
    pub const F_XMM1: Field<Address<t_cbc0908ab753>> = Field::new(8);
    pub const F_XMM2: Field<Address<t_cbc0908ab753>> = Field::new(16);
    pub const F_XMM3: Field<Address<t_cbc0908ab753>> = Field::new(24);
    pub const F_XMM4: Field<Address<t_cbc0908ab753>> = Field::new(32);
    pub const F_XMM5: Field<Address<t_cbc0908ab753>> = Field::new(40);
    pub const F_XMM6: Field<Address<t_cbc0908ab753>> = Field::new(48);
    pub const F_XMM7: Field<Address<t_cbc0908ab753>> = Field::new(56);
    pub const F_XMM8: Field<Address<t_cbc0908ab753>> = Field::new(64);
    pub const F_XMM9: Field<Address<t_cbc0908ab753>> = Field::new(72);
    pub const F_XMM10: Field<Address<t_cbc0908ab753>> = Field::new(80);
    pub const F_XMM11: Field<Address<t_cbc0908ab753>> = Field::new(88);
    pub const F_XMM12: Field<Address<t_cbc0908ab753>> = Field::new(96);
    pub const F_XMM13: Field<Address<t_cbc0908ab753>> = Field::new(104);
    pub const F_XMM14: Field<Address<t_cbc0908ab753>> = Field::new(112);
    pub const F_XMM15: Field<Address<t_cbc0908ab753>> = Field::new(120);
}

/// union _union_61: 128 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_dfb3f4aafa67 {
    pub bytes: [u8; 128],
}
impl Wire for t_dfb3f4aafa67 {
    const SIZE: usize = 128;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_dfb3f4aafa67 {
    pub const SIZE: usize = 128;
    pub const ALIGN: usize = 8;
    pub const F_FLOATINGCONTEXT: Field<[Address<t_cbc0908ab753>; 16]> = Field::new(0);
    pub const F_S: Field<t_6637dfe39500> = Field::new(0);
}

/// struct _struct_64: 128 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_98a0319bda06 {
    pub bytes: [u8; 128],
}
impl Wire for t_98a0319bda06 {
    const SIZE: usize = 128;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_98a0319bda06 {
    pub const SIZE: usize = 128;
    pub const ALIGN: usize = 8;
    pub const F_RAX: Field<Address<u64>> = Field::new(0);
    pub const F_RCX: Field<Address<u64>> = Field::new(8);
    pub const F_RDX: Field<Address<u64>> = Field::new(16);
    pub const F_RBX: Field<Address<u64>> = Field::new(24);
    pub const F_RSP: Field<Address<u64>> = Field::new(32);
    pub const F_RBP: Field<Address<u64>> = Field::new(40);
    pub const F_RSI: Field<Address<u64>> = Field::new(48);
    pub const F_RDI: Field<Address<u64>> = Field::new(56);
    pub const F_R8: Field<Address<u64>> = Field::new(64);
    pub const F_R9: Field<Address<u64>> = Field::new(72);
    pub const F_R10: Field<Address<u64>> = Field::new(80);
    pub const F_R11: Field<Address<u64>> = Field::new(88);
    pub const F_R12: Field<Address<u64>> = Field::new(96);
    pub const F_R13: Field<Address<u64>> = Field::new(104);
    pub const F_R14: Field<Address<u64>> = Field::new(112);
    pub const F_R15: Field<Address<u64>> = Field::new(120);
}

/// union _union_63: 128 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_efa0d9408ae1 {
    pub bytes: [u8; 128],
}
impl Wire for t_efa0d9408ae1 {
    const SIZE: usize = 128;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_efa0d9408ae1 {
    pub const SIZE: usize = 128;
    pub const ALIGN: usize = 8;
    pub const F_INTEGERCONTEXT: Field<[Address<u64>; 16]> = Field::new(0);
    pub const F_S: Field<t_98a0319bda06> = Field::new(0);
}

/// struct _ACL: 8 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_98ebdc684dea {
    pub bytes: [u8; 8],
}
impl Wire for t_98ebdc684dea {
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
impl t_98ebdc684dea {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 2;
    pub const F_ACLREVISION: Field<u8> = Field::new(0);
    pub const F_SBZ1: Field<u8> = Field::new(1);
    pub const F_ACLSIZE: Field<u16> = Field::new(2);
    pub const F_ACECOUNT: Field<u16> = Field::new(4);
    pub const F_SBZ2: Field<u16> = Field::new(6);
}

/// struct _OSVERSIONINFOEXW: 284 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d4858952b580 {
    pub bytes: [u8; 284],
}
impl Wire for t_d4858952b580 {
    const SIZE: usize = 284;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d4858952b580 {
    pub const SIZE: usize = 284;
    pub const ALIGN: usize = 4;
    pub const F_DWOSVERSIONINFOSIZE: Field<u32> = Field::new(0);
    pub const F_DWMAJORVERSION: Field<u32> = Field::new(4);
    pub const F_DWMINORVERSION: Field<u32> = Field::new(8);
    pub const F_DWBUILDNUMBER: Field<u32> = Field::new(12);
    pub const F_DWPLATFORMID: Field<u32> = Field::new(16);
    pub const F_SZCSDVERSION: Field<[i16; 128]> = Field::new(20);
    pub const F_WSERVICEPACKMAJOR: Field<u16> = Field::new(276);
    pub const F_WSERVICEPACKMINOR: Field<u16> = Field::new(278);
    pub const F_WSUITEMASK: Field<u16> = Field::new(280);
    pub const F_WPRODUCTTYPE: Field<u8> = Field::new(282);
    pub const F_WRESERVED: Field<u8> = Field::new(283);
}

/// struct _SLIST_ENTRY: 8 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_ee0beb379500 {
    pub bytes: [u8; 8],
}
impl Wire for t_ee0beb379500 {
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
impl t_ee0beb379500 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 8;
    pub const F_NEXT: Field<Address<Unknown>> = Field::new(0);
}

/// struct _UNWIND_HISTORY_TABLE: 216 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_49f218b6b015 {
    pub bytes: [u8; 216],
}
impl Wire for t_49f218b6b015 {
    const SIZE: usize = 216;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_49f218b6b015 {
    pub const SIZE: usize = 216;
    pub const ALIGN: usize = 8;
    pub const F_COUNT: Field<u32> = Field::new(0);
    pub const F_LOCALHINT: Field<u8> = Field::new(4);
    pub const F_GLOBALHINT: Field<u8> = Field::new(5);
    pub const F_SEARCH: Field<u8> = Field::new(6);
    pub const F_ONCE: Field<u8> = Field::new(7);
    pub const F_LOWADDRESS: Field<u64> = Field::new(8);
    pub const F_HIGHADDRESS: Field<u64> = Field::new(16);
    pub const F_ENTRY: Field<[t_099cffa171c0; 12]> = Field::new(24);
}

/// struct _MEMORY_BASIC_INFORMATION: 48 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_782ce141addd {
    pub bytes: [u8; 48],
}
impl Wire for t_782ce141addd {
    const SIZE: usize = 48;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_782ce141addd {
    pub const SIZE: usize = 48;
    pub const ALIGN: usize = 8;
    pub const F_BASEADDRESS: Field<Address<Unknown>> = Field::new(0);
    pub const F_ALLOCATIONBASE: Field<Address<Unknown>> = Field::new(8);
    pub const F_ALLOCATIONPROTECT: Field<u32> = Field::new(16);
    pub const F_REGIONSIZE: Field<u64> = Field::new(24);
    pub const F_STATE: Field<u32> = Field::new(32);
    pub const F_PROTECT: Field<u32> = Field::new(36);
    pub const F_TYPE: Field<u32> = Field::new(40);
}

/// struct _KNONVOLATILE_CONTEXT_POINTERS: 256 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a194e0e83612 {
    pub bytes: [u8; 256],
}
impl Wire for t_a194e0e83612 {
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
impl t_a194e0e83612 {
    pub const SIZE: usize = 256;
    pub const ALIGN: usize = 8;
    pub const F_U: Field<t_dfb3f4aafa67> = Field::new(0);
    pub const F_U2: Field<t_efa0d9408ae1> = Field::new(128);
}

/// struct HKEY__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5df0ca05dbf4 {
    pub bytes: [u8; 4],
}
impl Wire for t_5df0ca05dbf4 {
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
impl t_5df0ca05dbf4 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct HMONITOR__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_453047c9bfb3 {
    pub bytes: [u8; 4],
}
impl Wire for t_453047c9bfb3 {
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
impl t_453047c9bfb3 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct tagSIZE: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a2484b182201 {
    pub bytes: [u8; 8],
}
impl Wire for t_a2484b182201 {
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
impl t_a2484b182201 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_CX: Field<i32> = Field::new(0);
    pub const F_CY: Field<i32> = Field::new(4);
}

/// struct HMETAFILE__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_81ac937c454e {
    pub bytes: [u8; 4],
}
impl Wire for t_81ac937c454e {
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
impl t_81ac937c454e {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct HMENU__: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_69c576e42fd8 {
    pub bytes: [u8; 4],
}
impl Wire for t_69c576e42fd8 {
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
impl t_69c576e42fd8 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_UNUSED: Field<i32> = Field::new(0);
}

/// struct Var: 6 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_60529803e0f9 {
    pub bytes: [u8; 6],
}
impl Wire for t_60529803e0f9 {
    const SIZE: usize = 6;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_60529803e0f9 {
    pub const SIZE: usize = 6;
    pub const ALIGN: usize = 2;
    pub const F_WLENGTH: Field<u16> = Field::new(0);
    pub const F_WVALUELENGTH: Field<u16> = Field::new(2);
    pub const F_WTYPE: Field<u16> = Field::new(4);
}

/// struct IMAGE_IMPORT_BY_NAME_10: 12 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_61a649e3740b {
    pub bytes: [u8; 12],
}
impl Wire for t_61a649e3740b {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_61a649e3740b {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 10]> = Field::new(2);
}

/// struct IMAGE_LOAD_CONFIG_CODE_INTEGRITY: 12 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_12a47bfd922d {
    pub bytes: [u8; 12],
}
impl Wire for t_12a47bfd922d {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_12a47bfd922d {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 4;
    pub const F_FLAGS: Field<u16> = Field::new(0);
    pub const F_CATALOG: Field<u16> = Field::new(2);
    pub const F_CATALOGOFFSET: Field<u32> = Field::new(4);
    pub const F_RESERVED: Field<u32> = Field::new(8);
}

/// struct IMAGE_DEBUG_DIRECTORY: 28 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_c5ed2037a9da {
    pub bytes: [u8; 28],
}
impl Wire for t_c5ed2037a9da {
    const SIZE: usize = 28;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_c5ed2037a9da {
    pub const SIZE: usize = 28;
    pub const ALIGN: usize = 4;
    pub const F_CHARACTERISTICS: Field<u32> = Field::new(0);
    pub const F_TIMEDATESTAMP: Field<u32> = Field::new(4);
    pub const F_MAJORVERSION: Field<u16> = Field::new(8);
    pub const F_MINORVERSION: Field<u16> = Field::new(10);
    pub const F_TYPE: Field<u32> = Field::new(12);
    pub const F_SIZEOFDATA: Field<u32> = Field::new(16);
    pub const F_ADDRESSOFRAWDATA: Field<u32> = Field::new(20);
    pub const F_POINTERTORAWDATA: Field<u32> = Field::new(24);
}

/// struct IMAGE_IMPORT_BY_NAME_16: 18 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_106fb6e7812c {
    pub bytes: [u8; 18],
}
impl Wire for t_106fb6e7812c {
    const SIZE: usize = 18;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_106fb6e7812c {
    pub const SIZE: usize = 18;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 16]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_15: 18 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_6e1c23316e97 {
    pub bytes: [u8; 18],
}
impl Wire for t_6e1c23316e97 {
    const SIZE: usize = 18;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_6e1c23316e97 {
    pub const SIZE: usize = 18;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 15]> = Field::new(2);
}

/// struct IMAGE_RESOURCE_DIR_STRING_U_58: 60 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a7b30136199b {
    pub bytes: [u8; 60],
}
impl Wire for t_a7b30136199b {
    const SIZE: usize = 60;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a7b30136199b {
    pub const SIZE: usize = 60;
    pub const ALIGN: usize = 2;
    pub const F_LENGTH: Field<u16> = Field::new(0);
    pub const F_NAMESTRING: Field<[u16; 29]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_18: 20 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_1a676e47f244 {
    pub bytes: [u8; 20],
}
impl Wire for t_1a676e47f244 {
    const SIZE: usize = 20;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_1a676e47f244 {
    pub const SIZE: usize = 20;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 18]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_17: 20 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_48dc04407053 {
    pub bytes: [u8; 20],
}
impl Wire for t_48dc04407053 {
    const SIZE: usize = 20;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_48dc04407053 {
    pub const SIZE: usize = 20;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 17]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_19: 22 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_796044dfeb21 {
    pub bytes: [u8; 22],
}
impl Wire for t_796044dfeb21 {
    const SIZE: usize = 22;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_796044dfeb21 {
    pub const SIZE: usize = 22;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 19]> = Field::new(2);
}

/// union Misc: 4 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_048cb0a340e5 {
    pub bytes: [u8; 4],
}
impl Wire for t_048cb0a340e5 {
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
impl t_048cb0a340e5 {
    pub const SIZE: usize = 4;
    pub const ALIGN: usize = 4;
    pub const F_PHYSICALADDRESS: Field<u32> = Field::new(0);
    pub const F_VIRTUALSIZE: Field<u32> = Field::new(0);
}

/// struct IMAGE_SECTION_HEADER: 40 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d6601bc9eca9 {
    pub bytes: [u8; 40],
}
impl Wire for t_d6601bc9eca9 {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d6601bc9eca9 {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 4;
    pub const F_NAME: Field<[i8; 8]> = Field::new(0);
    pub const F_MISC: Field<t_048cb0a340e5> = Field::new(8);
    pub const F_VIRTUALADDRESS: Field<u32> = Field::new(12);
    pub const F_SIZEOFRAWDATA: Field<u32> = Field::new(16);
    pub const F_POINTERTORAWDATA: Field<u32> = Field::new(20);
    pub const F_POINTERTORELOCATIONS: Field<u32> = Field::new(24);
    pub const F_POINTERTOLINENUMBERS: Field<u32> = Field::new(28);
    pub const F_NUMBEROFRELOCATIONS: Field<u16> = Field::new(32);
    pub const F_NUMBEROFLINENUMBERS: Field<u16> = Field::new(34);
    pub const F_CHARACTERISTICS: Field<Enum32> = Field::new(36);
}

/// struct IMAGE_FILE_HEADER: 20 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_47fc86ee7976 {
    pub bytes: [u8; 20],
}
impl Wire for t_47fc86ee7976 {
    const SIZE: usize = 20;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_47fc86ee7976 {
    pub const SIZE: usize = 20;
    pub const ALIGN: usize = 4;
    pub const F_MACHINE: Field<u16> = Field::new(0);
    pub const F_NUMBEROFSECTIONS: Field<u16> = Field::new(2);
    pub const F_TIMEDATESTAMP: Field<u32> = Field::new(4);
    pub const F_POINTERTOSYMBOLTABLE: Field<u32> = Field::new(8);
    pub const F_NUMBEROFSYMBOLS: Field<u32> = Field::new(12);
    pub const F_SIZEOFOPTIONALHEADER: Field<u16> = Field::new(16);
    pub const F_CHARACTERISTICS: Field<u16> = Field::new(18);
}

/// struct IMAGE_DATA_DIRECTORY: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_7342e1154646 {
    pub bytes: [u8; 8],
}
impl Wire for t_7342e1154646 {
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
impl t_7342e1154646 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_VIRTUALADDRESS: Field<u32> = Field::new(0);
    pub const F_SIZE: Field<u32> = Field::new(4);
}

/// struct IMAGE_RESOURCE_DATA_ENTRY: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_c189ba78152a {
    pub bytes: [u8; 16],
}
impl Wire for t_c189ba78152a {
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
impl t_c189ba78152a {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_OFFSETTODATA: Field<u32> = Field::new(0);
    pub const F_SIZE: Field<u32> = Field::new(4);
    pub const F_CODEPAGE: Field<u32> = Field::new(8);
    pub const F_RESERVED: Field<u32> = Field::new(12);
}

/// struct IMAGE_IMPORT_BY_NAME_21: 24 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_72f67176ebb4 {
    pub bytes: [u8; 24],
}
impl Wire for t_72f67176ebb4 {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_72f67176ebb4 {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 21]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_20: 22 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bb03570ab96c {
    pub bytes: [u8; 22],
}
impl Wire for t_bb03570ab96c {
    const SIZE: usize = 22;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_bb03570ab96c {
    pub const SIZE: usize = 22;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 20]> = Field::new(2);
}

/// struct IMAGE_RESOURCE_DIRECTORY: 16 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_907ea54a4919 {
    pub bytes: [u8; 16],
}
impl Wire for t_907ea54a4919 {
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
impl t_907ea54a4919 {
    pub const SIZE: usize = 16;
    pub const ALIGN: usize = 4;
    pub const F_CHARACTERISTICS: Field<u32> = Field::new(0);
    pub const F_TIMEDATESTAMP: Field<u32> = Field::new(4);
    pub const F_MAJORVERSION: Field<u16> = Field::new(8);
    pub const F_MINORVERSION: Field<u16> = Field::new(10);
    pub const F_NUMBEROFNAMEDENTRIES: Field<u16> = Field::new(12);
    pub const F_NUMBEROFIDENTRIES: Field<u16> = Field::new(14);
}

/// struct IMAGE_IMPORT_BY_NAME_23: 26 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_f74128f01a97 {
    pub bytes: [u8; 26],
}
impl Wire for t_f74128f01a97 {
    const SIZE: usize = 26;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_f74128f01a97 {
    pub const SIZE: usize = 26;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 23]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_22: 24 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_8c007ef96404 {
    pub bytes: [u8; 24],
}
impl Wire for t_8c007ef96404 {
    const SIZE: usize = 24;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_8c007ef96404 {
    pub const SIZE: usize = 24;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 22]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_25: 28 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_18a17a434ce1 {
    pub bytes: [u8; 28],
}
impl Wire for t_18a17a434ce1 {
    const SIZE: usize = 28;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_18a17a434ce1 {
    pub const SIZE: usize = 28;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 25]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_24: 26 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_381cd4f10589 {
    pub bytes: [u8; 26],
}
impl Wire for t_381cd4f10589 {
    const SIZE: usize = 26;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_381cd4f10589 {
    pub const SIZE: usize = 26;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 24]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_27: 30 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_319d2f5cd9e6 {
    pub bytes: [u8; 30],
}
impl Wire for t_319d2f5cd9e6 {
    const SIZE: usize = 30;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_319d2f5cd9e6 {
    pub const SIZE: usize = 30;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 27]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_26: 28 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_4adc45268c21 {
    pub bytes: [u8; 28],
}
impl Wire for t_4adc45268c21 {
    const SIZE: usize = 28;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_4adc45268c21 {
    pub const SIZE: usize = 28;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 26]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_28: 30 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a62146bebd95 {
    pub bytes: [u8; 30],
}
impl Wire for t_a62146bebd95 {
    const SIZE: usize = 30;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a62146bebd95 {
    pub const SIZE: usize = 30;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 28]> = Field::new(2);
}

/// struct IMAGE_RESOURCE_DIR_STRING_U_44: 46 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_8567321bec0d {
    pub bytes: [u8; 46],
}
impl Wire for t_8567321bec0d {
    const SIZE: usize = 46;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_8567321bec0d {
    pub const SIZE: usize = 46;
    pub const ALIGN: usize = 2;
    pub const F_LENGTH: Field<u16> = Field::new(0);
    pub const F_NAMESTRING: Field<[u16; 22]> = Field::new(2);
}

/// struct IMAGE_RESOURCE_DIR_STRING_U_46: 48 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_5c51d4f905ca {
    pub bytes: [u8; 48],
}
impl Wire for t_5c51d4f905ca {
    const SIZE: usize = 48;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_5c51d4f905ca {
    pub const SIZE: usize = 48;
    pub const ALIGN: usize = 2;
    pub const F_LENGTH: Field<u16> = Field::new(0);
    pub const F_NAMESTRING: Field<[u16; 23]> = Field::new(2);
}

/// struct _IMAGE_RUNTIME_FUNCTION_ENTRY: 12 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_d05c9f0490ff {
    pub bytes: [u8; 12],
}
impl Wire for t_d05c9f0490ff {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_d05c9f0490ff {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 4;
    pub const F_BEGINADDRESS: Field<u32> = Field::new(0);
    pub const F_ENDADDRESS: Field<u32> = Field::new(4);
    pub const F_UNWINDINFOADDRESSORDATA: Field<u32> = Field::new(8);
}

/// struct IMAGE_IMPORT_BY_NAME_9: 12 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bee8c05912fc {
    pub bytes: [u8; 12],
}
impl Wire for t_bee8c05912fc {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_bee8c05912fc {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 9]> = Field::new(2);
}

/// struct ImgDelayDescr: 32 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_959541cecaf0 {
    pub bytes: [u8; 32],
}
impl Wire for t_959541cecaf0 {
    const SIZE: usize = 32;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_959541cecaf0 {
    pub const SIZE: usize = 32;
    pub const ALIGN: usize = 4;
    pub const F_GRATTRS: Field<u32> = Field::new(0);
    pub const F_SZNAME: Field<u32> = Field::new(4);
    pub const F_PHMOD: Field<u32> = Field::new(8);
    pub const F_PIAT: Field<u32> = Field::new(12);
    pub const F_PINT: Field<u32> = Field::new(16);
    pub const F_PBOUNDIAT: Field<u32> = Field::new(20);
    pub const F_PUNLOADIAT: Field<u32> = Field::new(24);
    pub const F_DWTIMESTAMP: Field<u32> = Field::new(28);
}

/// struct VarFileInfo: 6 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_18a8ecb04ebb {
    pub bytes: [u8; 6],
}
impl Wire for t_18a8ecb04ebb {
    const SIZE: usize = 6;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_18a8ecb04ebb {
    pub const SIZE: usize = 6;
    pub const ALIGN: usize = 2;
    pub const F_WLENGTH: Field<u16> = Field::new(0);
    pub const F_WVALUELENGTH: Field<u16> = Field::new(2);
    pub const F_WTYPE: Field<u16> = Field::new(4);
}

/// struct StringInfo: 6 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a320da0f5889 {
    pub bytes: [u8; 6],
}
impl Wire for t_a320da0f5889 {
    const SIZE: usize = 6;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a320da0f5889 {
    pub const SIZE: usize = 6;
    pub const ALIGN: usize = 2;
    pub const F_WLENGTH: Field<u16> = Field::new(0);
    pub const F_WVALUELENGTH: Field<u16> = Field::new(2);
    pub const F_WTYPE: Field<u16> = Field::new(4);
}

/// struct IMAGE_RESOURCE_DIR_STRING_U_60: 62 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_0a301615ebb4 {
    pub bytes: [u8; 62],
}
impl Wire for t_0a301615ebb4 {
    const SIZE: usize = 62;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_0a301615ebb4 {
    pub const SIZE: usize = 62;
    pub const ALIGN: usize = 2;
    pub const F_LENGTH: Field<u16> = Field::new(0);
    pub const F_NAMESTRING: Field<[u16; 30]> = Field::new(2);
}

/// struct IMAGE_RESOURCE_DIR_STRING_U_10: 12 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_2a1b0958f3ac {
    pub bytes: [u8; 12],
}
impl Wire for t_2a1b0958f3ac {
    const SIZE: usize = 12;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_2a1b0958f3ac {
    pub const SIZE: usize = 12;
    pub const ALIGN: usize = 2;
    pub const F_LENGTH: Field<u16> = Field::new(0);
    pub const F_NAMESTRING: Field<[u16; 5]> = Field::new(2);
}

/// struct StringTable: 6 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_35677a4ddd7e {
    pub bytes: [u8; 6],
}
impl Wire for t_35677a4ddd7e {
    const SIZE: usize = 6;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_35677a4ddd7e {
    pub const SIZE: usize = 6;
    pub const ALIGN: usize = 2;
    pub const F_WLENGTH: Field<u16> = Field::new(0);
    pub const F_WVALUELENGTH: Field<u16> = Field::new(2);
    pub const F_WTYPE: Field<u16> = Field::new(4);
}

/// struct IMAGE_IMPORT_BY_NAME_31: 34 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_2311a28c4d26 {
    pub bytes: [u8; 34],
}
impl Wire for t_2311a28c4d26 {
    const SIZE: usize = 34;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_2311a28c4d26 {
    pub const SIZE: usize = 34;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 31]> = Field::new(2);
}

/// struct IMAGE_IMPORT_BY_NAME_33: 36 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_36cdbcd540ec {
    pub bytes: [u8; 36],
}
impl Wire for t_36cdbcd540ec {
    const SIZE: usize = 36;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_36cdbcd540ec {
    pub const SIZE: usize = 36;
    pub const ALIGN: usize = 2;
    pub const F_HINT: Field<u16> = Field::new(0);
    pub const F_NAME: Field<[i8; 33]> = Field::new(2);
}

/// struct StringFileInfo: 6 bytes, source alignment 2.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_bf3171d9b8d8 {
    pub bytes: [u8; 6],
}
impl Wire for t_bf3171d9b8d8 {
    const SIZE: usize = 6;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_bf3171d9b8d8 {
    pub const SIZE: usize = 6;
    pub const ALIGN: usize = 2;
    pub const F_WLENGTH: Field<u16> = Field::new(0);
    pub const F_WVALUELENGTH: Field<u16> = Field::new(2);
    pub const F_WTYPE: Field<u16> = Field::new(4);
}

/// struct VS_VERSION_INFO: 92 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_14555e1345ce {
    pub bytes: [u8; 92],
}
impl Wire for t_14555e1345ce {
    const SIZE: usize = 92;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_14555e1345ce {
    pub const SIZE: usize = 92;
    pub const ALIGN: usize = 4;
    pub const F_STRUCTLENGTH: Field<u16> = Field::new(0);
    pub const F_VALUELENGTH: Field<u16> = Field::new(2);
    pub const F_STRUCTTYPE: Field<u16> = Field::new(4);
    pub const F_INFO: Field<[u16; 16]> = Field::new(6);
    pub const F_PADDING: Field<[u8; 2]> = Field::new(38);
    pub const F_SIGNATURE: Field<u32> = Field::new(40);
    pub const F_STRUCTVERSION: Field<[u16; 2]> = Field::new(44);
    pub const F_FILEVERSION: Field<[u16; 4]> = Field::new(48);
    pub const F_PRODUCTVERSION: Field<[u16; 4]> = Field::new(56);
    pub const F_FILEFLAGSMASK: Field<[u32; 2]> = Field::new(64);
    pub const F_FILEFLAGS: Field<u32> = Field::new(72);
    pub const F_FILEOS: Field<u32> = Field::new(76);
    pub const F_FILETYPE: Field<u32> = Field::new(80);
    pub const F_FILESUBTYPE: Field<u32> = Field::new(84);
    pub const F_FILETIMESTAMP: Field<u32> = Field::new(88);
}

/// struct IMAGE_BASE_RELOCATION: 8 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_f0a826c4e870 {
    pub bytes: [u8; 8],
}
impl Wire for t_f0a826c4e870 {
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
impl t_f0a826c4e870 {
    pub const SIZE: usize = 8;
    pub const ALIGN: usize = 4;
    pub const F_VIRTUALADDRESS: Field<u32> = Field::new(0);
    pub const F_SIZEOFBLOCK: Field<u32> = Field::new(4);
}

/// struct IMAGE_THUNK_DATA64: 40 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_a1cb950cec4a {
    pub bytes: [u8; 40],
}
impl Wire for t_a1cb950cec4a {
    const SIZE: usize = 40;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_a1cb950cec4a {
    pub const SIZE: usize = 40;
    pub const ALIGN: usize = 8;
    pub const F_STARTADDRESSOFRAWDATA: Field<u64> = Field::new(0);
    pub const F_ENDADDRESSOFRAWDATA: Field<u64> = Field::new(8);
    pub const F_ADDRESSOFINDEX: Field<u64> = Field::new(16);
    pub const F_ADDRESSOFCALLBACKS: Field<u64> = Field::new(24);
    pub const F_SIZEOFZEROFILL: Field<u32> = Field::new(32);
    pub const F_CHARACTERISTICS: Field<u32> = Field::new(36);
}

/// struct tagSTATSTG: 80 bytes, source alignment 8.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_730c5ce0c542 {
    pub bytes: [u8; 80],
}
impl Wire for t_730c5ce0c542 {
    const SIZE: usize = 80;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_730c5ce0c542 {
    pub const SIZE: usize = 80;
    pub const ALIGN: usize = 8;
    pub const F_PWCSNAME: Field<Address<i16>> = Field::new(0);
    pub const F_TYPE: Field<u32> = Field::new(8);
    pub const F_CBSIZE: Field<t_ec87f6a0d8aa> = Field::new(16);
    pub const F_MTIME: Field<t_dbd699519b88> = Field::new(24);
    pub const F_CTIME: Field<t_dbd699519b88> = Field::new(32);
    pub const F_ATIME: Field<t_dbd699519b88> = Field::new(40);
    pub const F_GRFMODE: Field<u32> = Field::new(48);
    pub const F_GRFLOCKSSUPPORTED: Field<u32> = Field::new(52);
    pub const F_CLSID: Field<t_9bbc0df2157e> = Field::new(56);
    pub const F_GRFSTATEBITS: Field<u32> = Field::new(72);
    pub const F_RESERVED: Field<u32> = Field::new(76);
}

/// struct DotNetPdbInfo: 60 bytes, source alignment 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct t_9e38acb35fbe {
    pub bytes: [u8; 60],
}
impl Wire for t_9e38acb35fbe {
    const SIZE: usize = 60;
    fn load(m: &MemoryImage, a: u64) -> Result<Self, MemoryFault> {
        Ok(Self {
            bytes: load_bytes(m, a)?,
        })
    }
    fn store(&self, m: &mut MemoryImage, a: u64) -> Result<(), MemoryFault> {
        store_bytes(m, a, &self.bytes)
    }
}
impl t_9e38acb35fbe {
    pub const SIZE: usize = 60;
    pub const ALIGN: usize = 4;
    pub const F_SIGNATURE: Field<[i8; 4]> = Field::new(0);
    pub const F_GUID: Field<t_9bbc0df2157e> = Field::new(4);
    pub const F_AGE: Field<u32> = Field::new(20);
    pub const F_PDBPATH: Field<[i8; 35]> = Field::new(24);
}

/// Incomplete decompiler type: CDC.
pub enum t_c7c036d7a68e {}

/// Incomplete decompiler type: CPageSetupDialog.
pub enum t_5a2e07c125b0 {}

/// Incomplete decompiler type: IOleInPlaceUIWindow.
pub enum t_12ee6bc4f89c {}

/// Incomplete decompiler type: Value.
pub enum t_8e37953d23da {}

/// Incomplete decompiler type: CCommonDialog.
pub enum t_0cc46c8a09a9 {}

/// Incomplete decompiler type: AFX_EVENTSINKMAP.
pub enum t_d783f65f866b {}

/// Incomplete decompiler type: AFX_CONNECTIONMAP.
pub enum t_ea9f3a76d276 {}

/// Incomplete decompiler type: CScrollBar.
pub enum t_291e456b7c16 {}

/// Incomplete decompiler type: IPropertyStore.
pub enum t_6c3598c1bb33 {}

/// Incomplete decompiler type: AirBrushTool.
pub enum t_58d674b3745e {}

/// Incomplete decompiler type: tagVARIANT.
pub enum t_c84250e745ea {}

/// Incomplete decompiler type: IConnectionPoint.
pub enum t_902a598a35b3 {}

/// Incomplete decompiler type: CFileException.
pub enum t_9d3d102131ad {}

/// Incomplete decompiler type: AFX_DISPMAP.
pub enum t_eaa8e2258470 {}

/// Incomplete decompiler type: GenerativeFillTool.
pub enum t_49f32ee8809f {}

/// Incomplete decompiler type: CPrintPreviewState.
pub enum t_0e973829986b {}

/// Incomplete decompiler type: CTypeLibCache.
pub enum t_a16246e945c4 {}

/// Incomplete decompiler type: CEventPublisher.
pub enum t_ec5129aa08f0 {}

/// Incomplete decompiler type: COleDocument.
pub enum t_abf1d60f1e9d {}

/// Incomplete decompiler type: MagicEraserTool.
pub enum t_b8c854a3590e {}

/// Incomplete decompiler type: tagWINDOWPLACEMENT.
pub enum t_c83dc576a2ca {}

/// Incomplete decompiler type: CArchive.
pub enum t_d382ee6afcf4 {}

/// Incomplete decompiler type: tagPDW.
pub enum t_e8ed33d59614 {}

/// Incomplete decompiler type: CPrintInfo.
pub enum t_59c98388411f {}

/// Incomplete decompiler type: CGdiObject.
pub enum t_84421de86d9f {}

/// Incomplete decompiler type: COleControlContainer.
pub enum t_e56e3528d5cf {}

/// Incomplete decompiler type: AFX_CMDHANDLERINFO.
pub enum t_cf63636a746a {}

/// Incomplete decompiler type: CWinApp.
pub enum t_8b7a903b2153 {}

/// Incomplete decompiler type: CDocItem.
pub enum t_8a8dc73e6912 {}

/// Incomplete decompiler type: TextTool.
pub enum t_c813ac9bdc14 {}

/// Incomplete decompiler type: IAccessibleProxy.
pub enum t_6c5c4deb266a {}

/// Incomplete decompiler type: CBrushTool.
pub enum t_da756b5fc6b0 {}

/// Incomplete decompiler type: ZoomTool.
pub enum t_793133314b97 {}

/// Incomplete decompiler type: CRectTool.
pub enum t_b39a8ac18b27 {}

/// Incomplete decompiler type: tagXFORM.
pub enum t_5f6270895f0a {}

/// Incomplete decompiler type: COleTemplateServer.
pub enum t_6f7763f001b6 {}

/// Incomplete decompiler type: COleServerDoc.
pub enum t_ffa0eee2339a {}

/// Incomplete decompiler type: AFX_MAINTAIN_STATE2.
pub enum t_c4f451916fbc {}

/// Incomplete decompiler type: SelectionTool.
pub enum t_539d6ec608b9 {}

/// Incomplete decompiler type: CFont.
pub enum t_e509236be2db {}

/// Incomplete decompiler type: CCmdTarget.
pub enum t_b4cf56875f70 {}

/// Incomplete decompiler type: CDataRecoveryHandler.
pub enum t_901ec624e73c {}

/// Incomplete decompiler type: CNoTrackObject.
pub enum t_f29f4c4e0d9b {}

/// Incomplete decompiler type: EdpTelemetryInfo.
pub enum t_72baf3a2654f {}

/// Incomplete decompiler type: RecoveryCallback.
pub enum t_0e05000c513d {}

/// Incomplete decompiler type: _AFX_OCC_DIALOG_INFO.
pub enum t_253f094e1e73 {}

/// Incomplete decompiler type: IDispatch.
pub enum t_1ca377704dd5 {}

/// Incomplete decompiler type: tagTOUCHINPUT.
pub enum t_59029f6197c1 {}

/// Incomplete decompiler type: ITextServices.
pub enum t_6a8915ad9c8b {}

/// Incomplete decompiler type: CCmdUI.
pub enum t_adee456ffe46 {}

/// Incomplete decompiler type: CPrintDialog.
pub enum t_b161a946421e {}

/// Incomplete decompiler type: IImageObjectEncoder.
pub enum t_cba2bf16a893 {}

/// Incomplete decompiler type: InactiveTool.
pub enum t_a0ea7fcb2cb3 {}

/// Incomplete decompiler type: CClientDC.
pub enum t_636542005cc6 {}

/// Incomplete decompiler type: CFile.
pub enum t_7ee23c085bc3 {}

/// Incomplete decompiler type: CPreviewView.
pub enum t_7e5ee2efd06b {}

/// Incomplete decompiler type: CDocument.
pub enum t_324ea9b736f3 {}

/// Incomplete decompiler type: AFX_OLECMDMAP.
pub enum t_acc5b7b8f0f6 {}

/// Incomplete decompiler type: CScrollView.
pub enum t_a2a793e11277 {}

/// Incomplete decompiler type: IOleItemContainer.
pub enum t_187b48362a34 {}

/// Incomplete decompiler type: _D3DCOLORVALUE.
pub enum t_5dfb388a4035 {}

/// Incomplete decompiler type: Event.
pub enum t_4e1f49a9c8ae {}

/// Incomplete decompiler type: CDialog.
pub enum t_285a6063891d {}

/// Incomplete decompiler type: PickColorTool.
pub enum t_ed7d31c5ac72 {}

/// Incomplete decompiler type: CRuntimeClass.
pub enum t_4e6a35ef1c3d {}

/// Incomplete decompiler type: COleIPFrameWnd.
pub enum t_fe5dba535169 {}

/// Incomplete decompiler type: CSize.
pub enum t_f0624d661ef9 {}

/// Incomplete decompiler type: CWnd.
pub enum t_57e057c985b3 {}

/// Incomplete decompiler type: HACCEL__.
pub enum t_4a4ffecd6d74 {}

/// Incomplete decompiler type: CFileDialog.
pub enum t_eeb105301392 {}

/// Incomplete decompiler type: RestartHandler.
pub enum t_6ab2e5ea63f2 {}

/// Incomplete decompiler type: AFX_MSGMAP.
pub enum t_ba72d61f4d41 {}

/// Incomplete decompiler type: MRECmpImpl.
pub enum t_b5571fed7f64 {}

/// Incomplete decompiler type: CSingleDocTemplate.
pub enum t_2bce61ab09c3 {}

/// Incomplete decompiler type: IOleDocumentSite.
pub enum t_0d2c2444c702 {}

/// Incomplete decompiler type: CBitmap.
pub enum t_0c6033a2a79b {}

/// Incomplete decompiler type: CLineTool.
pub enum t_870d0affb64d {}

/// Incomplete decompiler type: DLGTEMPLATE.
pub enum t_0a965115e24b {}

/// Incomplete decompiler type: COleDataObject.
pub enum t_9812f6c8da59 {}

/// Incomplete decompiler type: CCalligraphicTool.
pub enum t_9772d8edfec0 {}

/// Incomplete decompiler type: ITypeLib.
pub enum t_33f4a50b1005 {}

/// Incomplete decompiler type: COleObjectFactory.
pub enum t_d09b95f80687 {}

/// Incomplete decompiler type: IDirectXResourceManager.
pub enum t_afa6143b4504 {}

/// Incomplete decompiler type: CException.
pub enum t_c007e168f913 {}

/// Incomplete decompiler type: CGdiplusInit.
pub enum t_963089ed4ce3 {}

/// Incomplete decompiler type: CObject.
pub enum t_c1e4850f82b6 {}

/// Incomplete decompiler type: lconv.
pub enum t_3cc76c58b8d6 {}

/// Incomplete decompiler type: CAutoShapeTool.
pub enum t_034f6e423f84 {}

/// Incomplete decompiler type: AFX_EXCEPTION_LINK.
pub enum t_410e35c258d7 {}

/// Incomplete decompiler type: IPaintAppServices.
pub enum t_feda0bec3ff8 {}

/// Incomplete decompiler type: __POSITION.
pub enum t_800e1261bb01 {}

/// Incomplete decompiler type: CPaintPolygonTool.
pub enum t_b84b80e86fba {}

/// Incomplete decompiler type: COleLinkingDoc.
pub enum t_822925baaa51 {}

/// Incomplete decompiler type: RecoveryData.
pub enum t_8fa0d97ea188 {}

/// Incomplete decompiler type: COleStreamFile.
pub enum t_864cf621e0dc {}

/// Incomplete decompiler type: CSkeletalStrokeTool.
pub enum t_96bc6d8c91d9 {}

/// Incomplete decompiler type: CAttributeStore.
pub enum t_4b238d2fddda {}

/// Incomplete decompiler type: CBezierTool.
pub enum t_b35b08f68261 {}

/// Incomplete decompiler type: COleControlSite.
pub enum t_302de68ab2b7 {}

/// Incomplete decompiler type: AFX_INTERFACEMAP.
pub enum t_96793cec4137 {}

/// Incomplete decompiler type: CPoint.
pub enum t_5d21db35d8d6 {}

/// Incomplete decompiler type: CDocObjectServer.
pub enum t_f77736d1e40b {}

/// Incomplete decompiler type: CWinThread.
pub enum t_d57738207cdb {}

/// Incomplete decompiler type: FloodTool.
pub enum t_958817d0919f {}

/// Incomplete decompiler type: MaskBasedSelectionTool.
pub enum t_9361bd5e555b {}

/// Incomplete decompiler type: CDocTemplate.
pub enum t_bd10236b32bb {}

/// Incomplete decompiler type: NewGenerativeFillTool.
pub enum t_2bfe34522b4e {}

/// Incomplete decompiler type: CPencilTool.
pub enum t_acc1fd82c37b {}

/// Incomplete decompiler type: CView.
pub enum t_0e00949c3f1c {}

/// Incomplete decompiler type: CPtrArray.
pub enum t_ae8ad7c19fe3 {}

/// Incomplete decompiler type: CFrameWnd.
pub enum t_1b7b6e03d5b3 {}

/// Incomplete decompiler type: COleDataSource.
pub enum t_6ab9763eaab3 {}

/// Incomplete decompiler type: D2D_RECT_F.
pub enum t_d99468343608 {}

/// Incomplete decompiler type: IPaintListener.
pub enum t_6a454fe93607 {}

/// Incomplete decompiler type: FreeFormSelectionTool.
pub enum t_086d3dff0cf9 {}

/// Incomplete decompiler type: CMenu.
pub enum t_fc5c1cc9bc5e {}

/// Incomplete decompiler type: EraserTool.
pub enum t_44d1a3dd14e0 {}

/// Incomplete decompiler type: COleServerItem.
pub enum t_8dce37293ad3 {}

/// Incomplete decompiler type: tagCREATESTRUCTW.
pub enum t_d18e942b5916 {}

/// Incomplete decompiler type: tagLOGPALETTE.
pub enum t_3000537fd382 {}

/// Incomplete decompiler type: CCreateContext.
pub enum t_43037acc0d70 {}

/// Incomplete decompiler type: IPaintImageUtilities.
pub enum t_d8f8b19a7e2c {}

/// Incomplete decompiler type: CAttribute.
pub enum t_1b1221a5e85c {}

/// Incomplete decompiler type: _AFX_THREAD_STATE.
pub enum t_3d6dabe3e423 {}

/// Incomplete decompiler type: tagTOOLINFOW.
pub enum t_20dd53585f96 {}

/// Incomplete decompiler type: CRect.
pub enum t_571dd647b3c2 {}

/// Incomplete decompiler type: AFX_MODULE_STATE.
pub enum t_646ec5f8e2c8 {}

/// Incomplete decompiler type: CProcessLocalObject.
pub enum t_40839960747a {}

/// Incomplete decompiler type: CPalette.
pub enum t_bba8457eac48 {}

/// Incomplete decompiler type: COleClientItem.
pub enum t_ae205cbcdfbd {}

/// Incomplete decompiler type: CPaintTool.
pub enum t_2e3425b3a84b {}

/// Incomplete decompiler type: ObjectSelectTool.
pub enum t_d6fb928bf968 {}

/// Incomplete decompiler type: RecoveryHandler.
pub enum t_e9f462756d1e {}

/// Incomplete decompiler type: task<std::shared_ptr<Paint::AI::AIServices::OnlineEdgeHashResult>_>.
pub enum t_dc1f5508eb2f {}

/// Incomplete decompiler type: task<std::shared_ptr<Paint::AI::AIServices::OnlineProvenanceSigningResult>_>.
pub enum t_33a857d08c21 {}

/// Incomplete decompiler type: cancellation_token.
pub enum t_21391c6d9ff5 {}

/// Incomplete decompiler type: task_continuation_context.
pub enum t_e326cf0a7044 {}

/// Incomplete decompiler type: task<std::shared_ptr<Gdiplus::Bitmap>_>.
pub enum t_d15ac2fccbf4 {}

/// Incomplete decompiler type: UMSFreeThreadProxyFactory.
pub enum t_3a50f867af2b {}

/// Incomplete decompiler type: _ExceptionHolder.
pub enum t_2f826d8a0898 {}

/// Incomplete decompiler type: _ContextCallback.
pub enum t_fe0fd484df34 {}

/// Incomplete decompiler type: _TaskEventLogger.
pub enum t_49963138123c {}

/// Incomplete decompiler type: _Threadpool_chore.
pub enum t_f04485dd97db {}

/// Incomplete decompiler type: unique_any_t<wil::details::unique_storage<wil::details::resource_policy<HBITMAP__*___ptr64,int_(__cdecl*)(void*___ptr64),&int___cdecl_DeleteObject(void*___ptr64),wistd::integral_constant<unsigned___int64,0>,HBITMAP__*___ptr64,HBITMAP__*___ptr64,0,std::nullptr_t>_>_>.
pub enum t_999833a83ad0 {}

/// Incomplete decompiler type: unique_any_t<wil::details::unique_storage<wil::details::resource_policy<HPALETTE__*___ptr64,int_(__cdecl*)(void*___ptr64),&int___cdecl_DeleteObject(void*___ptr64),wistd::integral_constant<unsigned___int64,0>,HPALETTE__*___ptr64,HPALETTE__*___ptr64,0,std::nullptr_t>_>_>.
pub enum t_a7f85eef12dc {}

/// Incomplete decompiler type: Bitmap.
pub enum t_0198cc09cb23 {}

/// Incomplete decompiler type: ImageCodecInfo.
pub enum t_e56d69471b87 {}

/// Incomplete decompiler type: Image.
pub enum t_1aa4cb0bcca7 {}

/// Incomplete decompiler type: PropertyItem.
pub enum t_827bc6afc4ca {}

/// Incomplete decompiler type: Rect.
pub enum t_ee6a25464a0c {}

/// Incomplete decompiler type: Color.
pub enum t_6b73191a0a4b {}

/// Incomplete decompiler type: Region.
pub enum t_d3a008ef1335 {}

/// Incomplete decompiler type: com_ptr<IStream>.
pub enum t_0a63d925e5f3 {}

/// Incomplete decompiler type: com_ptr<ID2D1Geometry>.
pub enum t_6c7e695506f3 {}

/// Incomplete decompiler type: hresult.
pub enum t_bfcdb1b37cd2 {}

/// Incomplete decompiler type: com_ptr<IWICImagingFactory>.
pub enum t_6a60cf248b88 {}

/// Incomplete decompiler type: com_ptr<IWICBitmapSource>.
pub enum t_d9c8e109b766 {}

/// Incomplete decompiler type: hstring.
pub enum t_55980f9a178d {}

/// Incomplete decompiler type: event_token.
pub enum t_0aa7840cb874 {}

/// Incomplete decompiler type: com_ptr<ID2D1Bitmap1>.
pub enum t_b402da1e9f15 {}

/// Incomplete decompiler type: delegate<double>.
pub enum t_521660993db1 {}

/// Incomplete decompiler type: delegate<winrt::PaintUI::StoreUpdateStatus>.
pub enum t_00bc3738ba72 {}

/// Incomplete decompiler type: guid.
pub enum t_4b565f53a6d8 {}

/// Incomplete decompiler type: IImageCreatorManager.
pub enum t_5d32e5271a14 {}

/// Incomplete decompiler type: IColoringBookManager.
pub enum t_05630473f256 {}

/// Incomplete decompiler type: IModelAvailabilityManager.
pub enum t_9a52d0a5c2d8 {}

/// Incomplete decompiler type: IServiceOptions.
pub enum t_5a55a71d9304 {}

/// Incomplete decompiler type: IStickersManager.
pub enum t_0563bd78c79a {}

/// Incomplete decompiler type: ICloudCreditsManager.
pub enum t_9c2a9804d7d3 {}

/// Incomplete decompiler type: IPromptModerationService.
pub enum t_fdd2388130b5 {}

/// Incomplete decompiler type: IReimagineManager.
pub enum t_1e232d6dfcca {}

/// Incomplete decompiler type: IRandomAccessStream.
pub enum t_9ac63b5efc50 {}

/// Incomplete decompiler type: HttpClient.
pub enum t_6700c760a283 {}

/// Incomplete decompiler type: IAsyncAction.
pub enum t_62567263df37 {}

/// Incomplete decompiler type: Size.
pub enum t_1af851907331 {}

/// Incomplete decompiler type: Point.
pub enum t_2fc0c524431a {}

/// Incomplete decompiler type: IAsyncOperation<bool>.
pub enum t_47089487316c {}

/// Incomplete decompiler type: IPropertySet.
pub enum t_b68640470f12 {}

/// Incomplete decompiler type: unique_ptr<Gdiplus::Region,std::default_delete<Gdiplus::Region>_>.
pub enum t_084d565a47ac {}

/// Incomplete decompiler type: basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>_>.
pub enum t_52019ede0cdb {}

/// Incomplete decompiler type: vector<Paint::AI::ModerationScore,std::allocator<Paint::AI::ModerationScore>_>.
pub enum t_8c01be5d5ead {}

/// Incomplete decompiler type: ios_base.
pub enum t_91e8acb85ed8 {}

/// Incomplete decompiler type: basic_string_view<wchar_t,std::char_traits<wchar_t>_>.
pub enum t_23ce83bcb9ce {}

/// Incomplete decompiler type: basic_iostream<wchar_t,std::char_traits<wchar_t>_>.
pub enum t_f1a9fabf4022 {}

/// Incomplete decompiler type: function<void___cdecl(void)>.
pub enum t_d1f3327e5041 {}

/// Incomplete decompiler type: _Facet_base.
pub enum t_6accd497825c {}

/// Incomplete decompiler type: optional<bool>.
pub enum t_e032324ef236 {}

/// Incomplete decompiler type: function<class_Concurrency::task<winrt::guid>___cdecl(void)>.
pub enum t_601a9e919ecb {}

/// Incomplete decompiler type: optional<Paint::PlatformHelpers::CodecInfo>.
pub enum t_51f87740b9fc {}

/// Incomplete decompiler type: basic_ostream<wchar_t,std::char_traits<wchar_t>_>.
pub enum t_b5cd9520e20b {}

/// Incomplete decompiler type: function<class_Concurrency::task<winrt::hstring>___cdecl(void)>.
pub enum t_1ae8ab1e637b {}

/// Incomplete decompiler type: unique_ptr<ContentMetadata,std::default_delete<ContentMetadata>_>.
pub enum t_f3eda90f41d5 {}

/// Incomplete decompiler type: unique_ptr<class_Gdiplus::Bitmap,struct_std::default_delete<class_Gdiplus::Bitmap>_>.
pub enum t_44f8f717411e {}

/// Incomplete decompiler type: _Locinfo.
pub enum t_c0a04cd10bed {}

/// Incomplete decompiler type: basic_ostream<wchar_t,struct_std::char_traits<wchar_t>_>.
pub enum t_4ab7de6da030 {}

/// Incomplete decompiler type: basic_streambuf<wchar_t,std::char_traits<wchar_t>_>.
pub enum t_b978e344c1e1 {}

/// Incomplete decompiler type: optional<Paint::AI::AIServices::RegionOfInterest>.
pub enum t_53011625103e {}

/// Incomplete decompiler type: shared_ptr<Paint::PlatformHelpers::TaskOrchestrator>.
pub enum t_8f7add24787a {}

/// Incomplete decompiler type: unique_ptr<BasePaint::Image,std::default_delete<BasePaint::Image>_>.
pub enum t_02530651af7d {}

/// Incomplete decompiler type: vector<Paint::ProjectFile::ImageLayer,std::allocator<Paint::ProjectFile::ImageLayer>_>.
pub enum t_af4135a20171 {}

/// Incomplete decompiler type: unique_ptr<Gdiplus::Graphics,std::default_delete<Gdiplus::Graphics>_>.
pub enum t_59f2097d8b33 {}

/// Incomplete decompiler type: function<void___cdecl(winrt::Windows::Foundation::Diagnostics::LoggingFields_const&___ptr64)>.
pub enum t_0a271b6f7116 {}

/// Incomplete decompiler type: tuple<winrt::com_ptr<ID2D1Bitmap1>,D2D1::Matrix3x2F&___ptr64>.
pub enum t_1e3a3e98405f {}

/// Incomplete decompiler type: basic_streambuf<wchar_t,struct_std::char_traits<wchar_t>_>.
pub enum t_f010dd405dab {}

/// Incomplete decompiler type: locale.
pub enum t_77d9bce4a13d {}

/// Incomplete decompiler type: optional<int>.
pub enum t_2f01edf10e2a {}

/// Incomplete decompiler type: _Lockit.
pub enum t_1059cb443a9b {}

/// Incomplete decompiler type: unique_ptr<CPaintShape,std::default_delete<CPaintShape>_>.
pub enum t_baa3d05d6ec1 {}

/// Incomplete decompiler type: unique_ptr<Paint::AI::ImageObjectEncoding,std::default_delete<Paint::AI::ImageObjectEncoding>_>.
pub enum t_f306cfafe03e {}

/// Incomplete decompiler type: optional<winrt::hstring>.
pub enum t_bc53006a2d2f {}

/// Incomplete decompiler type: unique_ptr<Gdiplus::Bitmap,std::default_delete<Gdiplus::Bitmap>_>.
pub enum t_c68320bb4512 {}

/// Incomplete decompiler type: shared_ptr<CAttribute>.
pub enum t_913394f5901b {}

/// Incomplete decompiler type: unique_ptr<IImageObjectEncoder,std::default_delete<IImageObjectEncoder>_>.
pub enum t_173dc295033e {}

/// Incomplete decompiler type: basic_ios<wchar_t,std::char_traits<wchar_t>_>.
pub enum t_2f7033a87bf7 {}

/// Incomplete decompiler type: _Locimp.
pub enum t_9d904ef95af3 {}

/// Incomplete decompiler type: id.
pub enum t_a56145270ce6 {}

/// Incomplete decompiler type: facet.
pub enum t_5c07ce13958f {}

/// Incomplete decompiler type: path.
pub enum t_a0af9f865bf6 {}

/// Incomplete decompiler type: IProvenanceProvider.
pub enum t_092f8b73aabd {}

/// Incomplete decompiler type: ProvenanceHelper.
pub enum t_e2cec9925dbb {}

/// Incomplete decompiler type: TaskOrchestrator.
pub enum t_c324ac6ad0d7 {}

/// Incomplete decompiler type: PackageInstaller.
pub enum t_8bd0dddcee10 {}

/// Incomplete decompiler type: StoreUpdater.
pub enum t_2d9c0e443461 {}

/// Incomplete decompiler type: CodecInfo.
pub enum t_82193e6209c2 {}

/// Incomplete decompiler type: MagicEraserHelpers.
pub enum t_c22e7e8d9755 {}

/// Incomplete decompiler type: StableDiffusionHelpers.
pub enum t_bf4a57438cb0 {}

/// Incomplete decompiler type: SegmentationHelpers.
pub enum t_54e32828a68e {}

/// Incomplete decompiler type: GeneratedImageResult.
pub enum t_c23b7e56eb84 {}

/// Incomplete decompiler type: ImageObjectEncoding.
pub enum t_9907703a0c5f {}

/// Incomplete decompiler type: StableDiffusionResult.
pub enum t_db683659ff69 {}

/// Incomplete decompiler type: AIFactory.
pub enum t_2dd6065b535d {}

/// Incomplete decompiler type: Background.
pub enum t_ea2b8a878841 {}

/// Incomplete decompiler type: ImageLayer.
pub enum t_91f4247ac1b9 {}

/// Incomplete decompiler type: LayerMetadata.
pub enum t_a5bcf3559f19 {}

/// Incomplete decompiler type: ProjectFileHandler.
pub enum t_32370d1130db {}

/// Incomplete decompiler type: LayerStack.
pub enum t_691b59693f4c {}

/// Incomplete decompiler type: TraceLogger.
pub enum t_778b4967199a {}

/// Incomplete decompiler type: TracePerformanceLogger.
pub enum t_91bb08f86bac {}

/// Incomplete decompiler type: CStringT<wchar_t,StrTraitMFC_DLL<wchar_t,ATL::ChTraitsCRT<wchar_t>_>_>.
pub enum t_ae283e096c21 {}

/// Incomplete decompiler type: CSimpleStringT<wchar_t,1>.
pub enum t_727739213f1e {}

/// Incomplete decompiler type: IFilterChunkValue.
pub enum t_4e28903f8cb8 {}

/// Incomplete decompiler type: CAtlTransactionManager.
pub enum t_76b5785890c9 {}

/// Incomplete decompiler type: CStringT<wchar_t,class_StrTraitMFC_DLL<wchar_t,class_ATL::ChTraitsCRT<wchar_t>_>_>.
pub enum t_ee9183fedfff {}

/// Incomplete decompiler type: IAtlStringMgr.
pub enum t_90e1c7257bc6 {}

/// Incomplete decompiler type: ShaderFactory.
pub enum t_c577a3c77081 {}

/// Incomplete decompiler type: TextureStore.
pub enum t_a873b841019d {}

/// Incomplete decompiler type: PGS_SKELETALSTROKEBRUSH_PARAM.
pub enum t_1be356c6fbf7 {}

/// Incomplete decompiler type: PGS_SKELETALSTROKE_BMPPARAM.
pub enum t_15072ded30c4 {}

/// Incomplete decompiler type: PGSRect.
pub enum t_5196cd129c37 {}

/// Incomplete decompiler type: _PSP.
pub enum t_0ede1bfb8d71 {}

/// Incomplete decompiler type: _ACTIVATION_CONTEXT.
pub enum t_31cee394959e {}

/// Incomplete decompiler type: _TP_CLEANUP_GROUP.
pub enum t_3a31a65ef67d {}

/// Incomplete decompiler type: _TP_CALLBACK_INSTANCE.
pub enum t_f610c92dbd31 {}

/// Incomplete decompiler type: _TP_POOL.
pub enum t_f29900df9ce5 {}

/// Incomplete decompiler type: tagPROPVARIANT.
pub enum t_bf9c49bcca3a {}

pub mod enum_l161 {
    use super::Enum32;
    pub const F_FINDSTREAMINFOSTANDARD: Enum32 = Enum32(0);
    pub const F_FINDSTREAMINFOMAXINFOLEVEL: Enum32 = Enum32(1);
}

pub mod enum_l218 {
    use super::Enum32;
    pub const F_GETFILEEXINFOSTANDARD: Enum32 = Enum32(0);
    pub const F_GETFILEEXMAXINFOLEVEL: Enum32 = Enum32(1);
}

pub mod enum_l225 {
    use super::Enum32;
    pub const F_FILEBASICINFO: Enum32 = Enum32(0);
    pub const F_FILESTANDARDINFO: Enum32 = Enum32(1);
    pub const F_FILENAMEINFO: Enum32 = Enum32(2);
    pub const F_FILERENAMEINFO: Enum32 = Enum32(3);
    pub const F_FILEDISPOSITIONINFO: Enum32 = Enum32(4);
    pub const F_FILEALLOCATIONINFO: Enum32 = Enum32(5);
    pub const F_FILEENDOFFILEINFO: Enum32 = Enum32(6);
    pub const F_FILESTREAMINFO: Enum32 = Enum32(7);
    pub const F_FILECOMPRESSIONINFO: Enum32 = Enum32(8);
    pub const F_FILEATTRIBUTETAGINFO: Enum32 = Enum32(9);
    pub const F_FILEIDBOTHDIRECTORYINFO: Enum32 = Enum32(10);
    pub const F_FILEIDBOTHDIRECTORYRESTARTINFO: Enum32 = Enum32(11);
    pub const F_FILEIOPRIORITYHINTINFO: Enum32 = Enum32(12);
    pub const F_FILEREMOTEPROTOCOLINFO: Enum32 = Enum32(13);
    pub const F_MAXIMUMFILEINFOBYHANDLECLASS: Enum32 = Enum32(14);
}

pub mod enum_l903 {
    use super::Enum32;
}

pub mod enum_l1011 {
    use super::Enum32;
}

pub mod enum_l1049 {
    use super::Enum32;
}

pub mod enum_l1137 {
    use super::Enum32;
}

pub mod enum_l1370 {
    use super::Enum32;
}

pub mod enum_l1423 {
    use super::Enum32;
}

pub mod enum_l1431 {
    use super::Enum32;
}

pub mod enum_l1449 {
    use super::Enum32;
}

pub mod enum_l1452 {
    use super::Enum32;
}

pub mod enum_l1455 {
    use super::Enum32;
}

pub mod enum_l1553 {
    use super::Enum32;
}

pub mod enum_l1611 {
    use super::Enum32;
}

pub mod enum_l1614 {
    use super::Enum32;
}

pub mod enum_l1617 {
    use super::Enum32;
}

pub mod enum_l1620 {
    use super::Enum32;
}

pub mod enum_l1623 {
    use super::Enum32;
}

pub mod enum_l1626 {
    use super::Enum32;
}

pub mod enum_l1629 {
    use super::Enum32;
}

pub mod enum_l1647 {
    use super::Enum32;
}

pub mod enum_l1910 {
    use super::Enum32;
}

pub mod enum_l1913 {
    use super::Enum32;
}

pub mod enum_l1916 {
    use super::Enum32;
}

pub mod enum_l1919 {
    use super::Enum32;
}

pub mod enum_l1922 {
    use super::Enum32;
}

pub mod enum_l1955 {
    use super::Enum32;
}

pub mod enum_l1963 {
    use super::Enum32;
}

pub mod enum_l1986 {
    use super::Enum32;
}

pub mod enum_l1999 {
    use super::Enum32;
}

pub mod enum_l2027 {
    use super::Enum32;
}

pub mod enum_l2035 {
    use super::Enum32;
}

pub mod enum_l2043 {
    use super::Enum32;
}

pub mod enum_l2046 {
    use super::Enum32;
}

pub mod enum_l2049 {
    use super::Enum32;
}

pub mod enum_l2052 {
    use super::Enum32;
}

pub mod enum_l2085 {
    use super::Enum32;
}

pub mod enum_l2088 {
    use super::Enum32;
}

pub mod enum_l2216 {
    use super::Enum32;
    pub const F_APTTYPE_CURRENT: Enum32 = Enum32(4294967295);
    pub const F_APTTYPE_STA: Enum32 = Enum32(0);
    pub const F_APTTYPE_MTA: Enum32 = Enum32(1);
    pub const F_APTTYPE_NA: Enum32 = Enum32(2);
    pub const F_APTTYPE_MAINSTA: Enum32 = Enum32(3);
}

pub mod enum_l2664 {
    use super::Enum32;
    pub const F_APTTYPEQUALIFIER_NONE: Enum32 = Enum32(0);
    pub const F_APTTYPEQUALIFIER_IMPLICIT_MTA: Enum32 = Enum32(1);
    pub const F_APTTYPEQUALIFIER_NA_ON_MTA: Enum32 = Enum32(2);
    pub const F_APTTYPEQUALIFIER_NA_ON_STA: Enum32 = Enum32(3);
    pub const F_APTTYPEQUALIFIER_NA_ON_IMPLICIT_MTA: Enum32 = Enum32(4);
    pub const F_APTTYPEQUALIFIER_NA_ON_MAINSTA: Enum32 = Enum32(5);
}

pub mod enum_l2683 {
    use super::Enum32;
    pub const F_EXCEPTIONCONTINUEEXECUTION: Enum32 = Enum32(0);
    pub const F_EXCEPTIONCONTINUESEARCH: Enum32 = Enum32(1);
    pub const F_EXCEPTIONNESTEDEXCEPTION: Enum32 = Enum32(2);
    pub const F_EXCEPTIONCOLLIDEDUNWIND: Enum32 = Enum32(3);
}

pub mod enum_l3047 {
    use super::Enum32;
    pub const F_TP_CALLBACK_PRIORITY_HIGH: Enum32 = Enum32(0);
    pub const F_TP_CALLBACK_PRIORITY_NORMAL: Enum32 = Enum32(1);
    pub const F_TP_CALLBACK_PRIORITY_LOW: Enum32 = Enum32(2);
    pub const F_TP_CALLBACK_PRIORITY_INVALID: Enum32 = Enum32(3);
}

pub mod enum_l3341 {
    use super::Enum32;
    pub const F_IMAGE_SCN_TYPE_NO_PAD: Enum32 = Enum32(8);
    pub const F_IMAGE_SCN_RESERVED_0001: Enum32 = Enum32(16);
    pub const F_IMAGE_SCN_CNT_CODE: Enum32 = Enum32(32);
    pub const F_IMAGE_SCN_CNT_INITIALIZED_DATA: Enum32 = Enum32(64);
    pub const F_IMAGE_SCN_CNT_UNINITIALIZED_DATA: Enum32 = Enum32(128);
    pub const F_IMAGE_SCN_LNK_OTHER: Enum32 = Enum32(256);
    pub const F_IMAGE_SCN_LNK_INFO: Enum32 = Enum32(512);
    pub const F_IMAGE_SCN_RESERVED_0040: Enum32 = Enum32(1024);
    pub const F_IMAGE_SCN_LNK_REMOVE: Enum32 = Enum32(2048);
    pub const F_IMAGE_SCN_LNK_COMDAT: Enum32 = Enum32(4096);
    pub const F_IMAGE_SCN_GPREL: Enum32 = Enum32(32768);
    pub const F_IMAGE_SCN_MEM_16BIT: Enum32 = Enum32(131072);
    pub const F_IMAGE_SCN_MEM_PURGEABLE: Enum32 = Enum32(131072);
    pub const F_IMAGE_SCN_MEM_LOCKED: Enum32 = Enum32(262144);
    pub const F_IMAGE_SCN_MEM_PRELOAD: Enum32 = Enum32(524288);
    pub const F_IMAGE_SCN_ALIGN_1BYTES: Enum32 = Enum32(1048576);
    pub const F_IMAGE_SCN_ALIGN_2BYTES: Enum32 = Enum32(2097152);
    pub const F_IMAGE_SCN_ALIGN_4BYTES: Enum32 = Enum32(3145728);
    pub const F_IMAGE_SCN_ALIGN_8BYTES: Enum32 = Enum32(4194304);
    pub const F_IMAGE_SCN_ALIGN_16BYTES: Enum32 = Enum32(5242880);
    pub const F_IMAGE_SCN_ALIGN_32BYTES: Enum32 = Enum32(6291456);
    pub const F_IMAGE_SCN_ALIGN_64BYTES: Enum32 = Enum32(7340032);
    pub const F_IMAGE_SCN_ALIGN_128BYTES: Enum32 = Enum32(8388608);
    pub const F_IMAGE_SCN_ALIGN_256BYTES: Enum32 = Enum32(9437184);
    pub const F_IMAGE_SCN_ALIGN_512BYTES: Enum32 = Enum32(10485760);
    pub const F_IMAGE_SCN_ALIGN_1024BYTES: Enum32 = Enum32(11534336);
    pub const F_IMAGE_SCN_ALIGN_2048BYTES: Enum32 = Enum32(12582912);
    pub const F_IMAGE_SCN_ALIGN_4096BYTES: Enum32 = Enum32(13631488);
    pub const F_IMAGE_SCN_ALIGN_8192BYTES: Enum32 = Enum32(14680064);
    pub const F_IMAGE_SCN_LNK_NRELOC_OVFL: Enum32 = Enum32(16777216);
    pub const F_IMAGE_SCN_MEM_DISCARDABLE: Enum32 = Enum32(33554432);
    pub const F_IMAGE_SCN_MEM_NOT_CACHED: Enum32 = Enum32(67108864);
    pub const F_IMAGE_SCN_MEM_NOT_PAGED: Enum32 = Enum32(134217728);
    pub const F_IMAGE_SCN_MEM_SHARED: Enum32 = Enum32(268435456);
    pub const F_IMAGE_SCN_MEM_EXECUTE: Enum32 = Enum32(536870912);
    pub const F_IMAGE_SCN_MEM_READ: Enum32 = Enum32(1073741824);
    pub const F_IMAGE_SCN_MEM_WRITE: Enum32 = Enum32(2147483648);
}

pub mod enum_l3575 {
    use super::Enum32;
    pub const F_IMAGE_GUARD_CF_INSTRUMENTED: Enum32 = Enum32(256);
    pub const F_IMAGE_GUARD_CFW_INSTRUMENTED: Enum32 = Enum32(512);
    pub const F_IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: Enum32 = Enum32(1024);
    pub const F_IMAGE_GUARD_SECURITY_COOKIE_UNUSED: Enum32 = Enum32(2048);
    pub const F_IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: Enum32 = Enum32(4096);
    pub const F_IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: Enum32 = Enum32(8192);
    pub const F_IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: Enum32 = Enum32(16384);
    pub const F_IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: Enum32 = Enum32(32768);
    pub const F_IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: Enum32 = Enum32(65536);
    pub const F_IMAGE_GUARD_RF_INSTRUMENTED: Enum32 = Enum32(131072);
    pub const F_IMAGE_GUARD_RF_ENABLE: Enum32 = Enum32(262144);
    pub const F_IMAGE_GUARD_RF_STRICT: Enum32 = Enum32(524288);
    pub const F_IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK_1: Enum32 = Enum32(268435456);
    pub const F_IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK_2: Enum32 = Enum32(536870912);
    pub const F_IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK_4: Enum32 = Enum32(1073741824);
    pub const F_IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK_8: Enum32 = Enum32(2147483648);
}

pub mod enum_l3764 {
    use super::Enum32;
    pub const F_SE_UNKNOWN_OBJECT_TYPE: Enum32 = Enum32(0);
    pub const F_SE_FILE_OBJECT: Enum32 = Enum32(1);
    pub const F_SE_SERVICE: Enum32 = Enum32(2);
    pub const F_SE_PRINTER: Enum32 = Enum32(3);
    pub const F_SE_REGISTRY_KEY: Enum32 = Enum32(4);
    pub const F_SE_LMSHARE: Enum32 = Enum32(5);
    pub const F_SE_KERNEL_OBJECT: Enum32 = Enum32(6);
    pub const F_SE_WINDOW_OBJECT: Enum32 = Enum32(7);
    pub const F_SE_DS_OBJECT: Enum32 = Enum32(8);
    pub const F_SE_DS_OBJECT_ALL: Enum32 = Enum32(9);
    pub const F_SE_PROVIDER_DEFINED_OBJECT: Enum32 = Enum32(10);
    pub const F_SE_WMIGUID_OBJECT: Enum32 = Enum32(11);
    pub const F_SE_REGISTRY_WOW64_32KEY: Enum32 = Enum32(12);
}

pub mod alias_l1 {
    use super::*;
    pub type A0 = u8;
}

pub mod alias_l3 {
    use super::*;
    pub type A0 = u8;
}

pub mod alias_l4 {
    use super::*;
    pub type A0 = u8;
}

pub mod alias_l5 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l6 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l7 {
    use super::*;
    pub type A0 = Relative32<Unknown>;
}

pub mod alias_l9 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l11 {
    use super::*;
    pub type A0 = i64;
}

pub mod alias_l12 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l13 {
    use super::*;
    pub type A0 = u8;
}

pub mod alias_l14 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l15 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l16 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l17 {
    use super::*;
    pub type A0 = u8;
}

pub mod alias_l18 {
    use super::*;
    pub type A0 = u16;
}

pub mod alias_l19 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l20 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l21 {
    use super::*;
    pub type A0 = u16;
}

pub mod alias_l22 {
    use super::*;
    pub type A0 = u16;
}

pub mod alias_l23 {
    use super::*;
    pub type A0 = i16;
}

pub mod alias_l24 {
    use super::*;
    pub type A0 = u16;
}

/// Incomplete decompiler type: IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryUnion.
pub enum t_0a25ff837faa {}

pub mod alias_l25 {
    use super::*;
    pub type A0 = t_0a25ff837faa;
    pub type A1 = Address<Unknown>;
}

/// Incomplete decompiler type: IMAGE_RESOURCE_DIRECTORY_ENTRY_DirectoryStruct.
pub enum t_33f191c4e08a {}

pub mod alias_l27 {
    use super::*;
    pub type A0 = t_33f191c4e08a;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l39 {
    use super::*;
    pub type A0 = t_a12ab21829aa;
    pub type A1 = Address<t_a12ab21829aa>;
}

pub mod alias_l41 {
    use super::*;
    pub type A0 = t_a12ab21829aa;
}

pub mod alias_l45 {
    use super::*;
    pub type A0 = t_43395d5dc722;
    pub type A1 = Address<t_43395d5dc722>;
}

pub mod alias_l61 {
    use super::*;
    pub type A0 = t_a1b9860cb6ca;
    pub type A1 = Address<t_a1b9860cb6ca>;
}

pub mod alias_l63 {
    use super::*;
    pub type A0 = t_a1b9860cb6ca;
}

pub mod alias_l73 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l77 {
    use super::*;
    pub type A0 = t_16e802e5e765;
    pub type A1 = Address<t_16e802e5e765>;
}

/// Incomplete decompiler type: _s__RTTIClassHierarchyDescriptor.
pub enum t_7e18d64b2a05 {}

pub mod alias_l84 {
    use super::*;
    pub type A0 = t_7e18d64b2a05;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l86 {
    use super::*;
    pub type A0 = t_7e18d64b2a05;
}

pub mod alias_l95 {
    use super::*;
    pub type A0 = t_d3f90483a5cf;
    pub type A1 = Address<t_d3f90483a5cf>;
}

pub mod alias_l97 {
    use super::*;
    pub type A0 = t_a37b3dfd76db;
    pub type A1 = Address<t_a37b3dfd76db>;
}

pub mod alias_l99 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l101 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l103 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l105 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l107 {
    use super::*;
    pub type A0 = u16;
}

pub mod alias_l109 {
    use super::*;
    pub type A0 = t_019e13be7222;
    pub type A1 = Address<t_019e13be7222>;
}

pub mod alias_l134 {
    use super::*;
    pub type A0 = t_9028a010187a;
    pub type A1 = Address<t_9028a010187a>;
}

pub mod alias_l136 {
    use super::*;
    pub type A0 = t_7f6edd60f405;
    pub type A1 = Address<t_7f6edd60f405>;
}

pub mod alias_l138 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l140 {
    use super::*;
    pub type A0 = t_9be79d4e6283;
    pub type A1 = Address<t_9be79d4e6283>;
}

pub mod alias_l142 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l166 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l168 {
    use super::*;
    pub type A0 = t_62c6d3acb4a3;
    pub type A1 = Address<t_62c6d3acb4a3>;
}

pub mod alias_l170 {
    use super::*;
    pub type A0 = Address<t_62c6d3acb4a3>;
}

pub mod alias_l172 {
    use super::*;
    pub type A0 = t_dbd699519b88;
    pub type A1 = Address<t_dbd699519b88>;
}

pub mod alias_l174 {
    use super::*;
    pub type A0 = t_dbd699519b88;
}

pub mod alias_l176 {
    use super::*;
    pub type A0 = i16;
}

pub mod alias_l196 {
    use super::*;
    pub type A0 = t_a9c742fb1d97;
    pub type A1 = Address<t_a9c742fb1d97>;
}

pub mod alias_l198 {
    use super::*;
    pub type A0 = Address<t_a9c742fb1d97>;
}

pub mod alias_l200 {
    use super::*;
    pub type A0 = Address<t_a9c742fb1d97>;
}

pub mod alias_l206 {
    use super::*;
    pub type A0 = t_f2074362ce16;
    pub type A1 = Address<t_f2074362ce16>;
}

pub mod alias_l208 {
    use super::*;
    pub type A0 = Address<t_f2074362ce16>;
}

pub mod alias_l210 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l223 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l243 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l245 {
    use super::*;
    pub type A0 = t_62fc25d82c1b;
    pub type A1 = Address<t_62fc25d82c1b>;
}

pub mod alias_l247 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l249 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l251 {
    use super::*;
    pub type A0 = t_e7ed2387b2bc;
    pub type A1 = Address<t_e7ed2387b2bc>;
}

pub mod alias_l253 {
    use super::*;
    pub type A0 = t_e7ed2387b2bc;
}

pub mod alias_l255 {
    use super::*;
    pub type A0 = Address<t_e7ed2387b2bc>;
}

pub mod alias_l257 {
    use super::*;
    pub type A0 = t_ea23a4932299;
    pub type A1 = Address<t_ea23a4932299>;
}

pub mod alias_l259 {
    use super::*;
    pub type A0 = Address<t_ea23a4932299>;
}

pub mod alias_l261 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l263 {
    use super::*;
    pub type A0 = t_bf1bd547dc12;
    pub type A1 = Address<t_bf1bd547dc12>;
}

pub mod alias_l265 {
    use super::*;
    pub type A0 = t_cbc0908ab753;
    pub type A1 = Address<t_cbc0908ab753>;
}

pub mod alias_l267 {
    use super::*;
    pub type A0 = t_cbc0908ab753;
}

pub mod alias_l269 {
    use super::*;
    pub type A0 = t_5cf4a2ad55dd;
    pub type A1 = Address<t_5cf4a2ad55dd>;
}

pub mod alias_l271 {
    use super::*;
    pub type A0 = t_5cf4a2ad55dd;
}

pub mod alias_l273 {
    use super::*;
    pub type A0 = t_5cf4a2ad55dd;
}

pub mod alias_l275 {
    use super::*;
    pub type A0 = t_275f2ba3754d;
    pub type A1 = Address<t_275f2ba3754d>;
}

pub mod alias_l277 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l279 {
    use super::*;
    pub type A0 = i64;
}

pub mod alias_l281 {
    use super::*;
    pub type A0 = u8;
}

pub mod alias_l396 {
    use super::*;
    pub type A0 = Address<t_d3f90483a5cf>;
}

pub mod alias_l398 {
    use super::*;
    pub type A0 = t_e9a7801c89db;
    pub type A1 = Address<t_e9a7801c89db>;
}

pub mod alias_l400 {
    use super::*;
    pub type A0 = Address<i16>;
}

pub mod alias_l402 {
    use super::*;
    pub type A0 = Address<u8>;
}

pub mod alias_l425 {
    use super::*;
    pub type A0 = Address<t_e9a7801c89db>;
}

pub mod alias_l427 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l429 {
    use super::*;
    pub type A0 = Address<t_9028a010187a>;
}

pub mod alias_l431 {
    use super::*;
    pub type A0 = t_5e6052772e22;
    pub type A1 = Address<t_5e6052772e22>;
}

pub mod alias_l433 {
    use super::*;
    pub type A0 = t_5e6052772e22;
}

pub mod alias_l435 {
    use super::*;
    pub type A0 = Address<t_5e6052772e22>;
}

pub mod alias_l441 {
    use super::*;
    pub type A0 = t_f7cecc1b772e;
    pub type A1 = Address<t_f7cecc1b772e>;
}

pub mod alias_l443 {
    use super::*;
    pub type A0 = t_f7cecc1b772e;
}

pub mod alias_l445 {
    use super::*;
    pub type A0 = Address<t_f7cecc1b772e>;
}

pub mod alias_l451 {
    use super::*;
    pub type A0 = t_d263a3913bc6;
    pub type A1 = Address<t_d263a3913bc6>;
}

pub mod alias_l453 {
    use super::*;
    pub type A0 = Address<t_d263a3913bc6>;
}

pub mod alias_l455 {
    use super::*;
    pub type A0 = Address<t_d263a3913bc6>;
}

pub mod alias_l457 {
    use super::*;
    pub type A0 = t_f7c016150490;
    pub type A1 = Address<t_f7c016150490>;
}

pub mod alias_l459 {
    use super::*;
    pub type A0 = Address<t_f7c016150490>;
}

pub mod alias_l461 {
    use super::*;
    pub type A0 = t_4022dd7ff700;
    pub type A1 = Address<t_4022dd7ff700>;
}

pub mod alias_l463 {
    use super::*;
    pub type A0 = t_4022dd7ff700;
}

pub mod alias_l491 {
    use super::*;
    pub type A0 = t_7a75b1cebcfc;
    pub type A1 = Address<t_7a75b1cebcfc>;
}

pub mod alias_l493 {
    use super::*;
    pub type A0 = t_85e75021d649;
    pub type A1 = Address<t_85e75021d649>;
}

pub mod alias_l495 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l500 {
    use super::*;
    pub type A0 = t_9bbc0df2157e;
}

pub mod alias_l502 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l504 {
    use super::*;
    pub type A0 = i16;
}

pub mod alias_l506 {
    use super::*;
    pub type A0 = Address<i16>;
}

pub mod alias_l523 {
    use super::*;
    pub type A0 = u16;
}

pub mod alias_l525 {
    use super::*;
    pub type A0 = Address<i16>;
}

pub mod alias_l527 {
    use super::*;
    pub type A0 = Address<i16>;
}

pub mod alias_l529 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l531 {
    use super::*;
    pub type A0 = t_07c238eba542;
    pub type A1 = Address<t_07c238eba542>;
}

pub mod alias_l540 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l542 {
    use super::*;
    pub type A0 = t_752b694a7b73;
    pub type A1 = Address<t_752b694a7b73>;
}

pub mod alias_l544 {
    use super::*;
    pub type A0 = t_752b694a7b73;
}

pub mod alias_l553 {
    use super::*;
    pub type A0 = t_085d0cfce539;
    pub type A1 = Address<t_085d0cfce539>;
}

pub mod alias_l561 {
    use super::*;
    pub type A0 = Address<i8>;
}

pub mod alias_l563 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l565 {
    use super::*;
    pub type A0 = t_8dd84e59b4c1;
    pub type A1 = Address<t_8dd84e59b4c1>;
}

pub mod alias_l567 {
    use super::*;
    pub type A0 = t_8dd84e59b4c1;
}

pub mod alias_l578 {
    use super::*;
    pub type A0 = t_7b839c86f896;
    pub type A1 = Address<t_7b839c86f896>;
}

pub mod alias_l580 {
    use super::*;
    pub type A0 = t_819011ce0501;
    pub type A1 = Address<t_819011ce0501>;
}

pub mod alias_l582 {
    use super::*;
    pub type A0 = t_0febd922c66b;
    pub type A1 = Address<t_0febd922c66b>;
}

pub mod alias_l584 {
    use super::*;
    pub type A0 = t_0febd922c66b;
}

pub mod alias_l586 {
    use super::*;
    pub type A0 = t_0cd516fda9fa;
    pub type A1 = Address<t_0cd516fda9fa>;
}

pub mod alias_l588 {
    use super::*;
    pub type A0 = t_0cd516fda9fa;
}

pub mod alias_l615 {
    use super::*;
    pub type A0 = t_237fe8721873;
    pub type A1 = Address<t_237fe8721873>;
}

pub mod alias_l617 {
    use super::*;
    pub type A0 = Address<t_237fe8721873>;
}

pub mod alias_l619 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l621 {
    use super::*;
    pub type A0 = i64;
}

pub mod alias_l623 {
    use super::*;
    pub type A0 = i64;
}

pub mod alias_l625 {
    use super::*;
    pub type A0 = t_c6339fcec9fa;
    pub type A1 = Address<t_c6339fcec9fa>;
}

pub mod alias_l627 {
    use super::*;
    pub type A0 = Address<t_c6339fcec9fa>;
}

pub mod alias_l629 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l631 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l633 {
    use super::*;
    pub type A0 = i64;
}

pub mod alias_l635 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l637 {
    use super::*;
    pub type A0 = t_4c0908fcb2fb;
    pub type A1 = Address<t_4c0908fcb2fb>;
}

pub mod alias_l639 {
    use super::*;
    pub type A0 = Address<t_4c0908fcb2fb>;
}

pub mod alias_l641 {
    use super::*;
    pub type A0 = t_06a2c1deaf59;
    pub type A1 = Address<t_06a2c1deaf59>;
}

pub mod alias_l643 {
    use super::*;
    pub type A0 = Address<t_06a2c1deaf59>;
}

pub mod alias_l645 {
    use super::*;
    pub type A0 = Address<t_06a2c1deaf59>;
}

pub mod alias_l647 {
    use super::*;
    pub type A0 = t_3981d9e73c1e;
    pub type A1 = Address<t_3981d9e73c1e>;
}

pub mod alias_l649 {
    use super::*;
    pub type A0 = Address<t_3981d9e73c1e>;
}

pub mod alias_l651 {
    use super::*;
    pub type A0 = Address<i16>;
}

pub mod alias_l682 {
    use super::*;
    pub type A0 = t_ac6f275477d5;
    pub type A1 = Address<t_ac6f275477d5>;
}

pub mod alias_l684 {
    use super::*;
    pub type A0 = t_ac6f275477d5;
}

pub mod alias_l686 {
    use super::*;
    pub type A0 = t_fd609cf74451;
    pub type A1 = Address<t_fd609cf74451>;
}

pub mod alias_l688 {
    use super::*;
    pub type A0 = t_fd609cf74451;
}

pub mod alias_l704 {
    use super::*;
    pub type A0 = t_9b8d66b76bc0;
    pub type A1 = Address<t_9b8d66b76bc0>;
}

pub mod alias_l706 {
    use super::*;
    pub type A0 = Address<t_9b8d66b76bc0>;
}

pub mod alias_l708 {
    use super::*;
    pub type A0 = t_272e88af6e21;
    pub type A1 = Address<t_272e88af6e21>;
}

pub mod alias_l710 {
    use super::*;
    pub type A0 = t_272e88af6e21;
}

pub mod alias_l726 {
    use super::*;
    pub type A0 = Address<t_ac6f275477d5>;
}

pub mod alias_l728 {
    use super::*;
    pub type A0 = Address<t_7b839c86f896>;
}

pub mod alias_l730 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l732 {
    use super::*;
    pub type A0 = t_4a3399835aa5;
    pub type A1 = Address<t_4a3399835aa5>;
}

pub mod alias_l757 {
    use super::*;
    pub type A0 = Address<u64>;
}

pub mod alias_l759 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l761 {
    use super::*;
    pub type A0 = i64;
}

pub mod alias_l763 {
    use super::*;
    pub type A0 = t_c7c036d7a68e;
    pub type A1 = Address<t_c7c036d7a68e>;
}

pub mod alias_l768 {
    use super::*;
    pub type A0 = t_5a2e07c125b0;
    pub type A1 = Address<t_5a2e07c125b0>;
}

pub mod alias_l773 {
    use super::*;
    pub type A0 = t_12ee6bc4f89c;
    pub type A1 = Address<t_12ee6bc4f89c>;
}

pub mod alias_l778 {
    use super::*;
    pub type A0 = t_8e37953d23da;
    pub type A1 = Address<t_8e37953d23da>;
}

pub mod alias_l783 {
    use super::*;
    pub type A0 = t_0cc46c8a09a9;
    pub type A1 = Address<t_0cc46c8a09a9>;
}

pub mod alias_l788 {
    use super::*;
    pub type A0 = t_d783f65f866b;
    pub type A1 = Address<t_d783f65f866b>;
}

pub mod alias_l793 {
    use super::*;
    pub type A0 = t_ea9f3a76d276;
    pub type A1 = Address<t_ea9f3a76d276>;
}

pub mod alias_l798 {
    use super::*;
    pub type A0 = t_291e456b7c16;
    pub type A1 = Address<t_291e456b7c16>;
}

pub mod alias_l803 {
    use super::*;
    pub type A0 = t_6c3598c1bb33;
    pub type A1 = Address<t_6c3598c1bb33>;
}

pub mod alias_l808 {
    use super::*;
    pub type A0 = t_58d674b3745e;
    pub type A1 = Address<t_58d674b3745e>;
}

pub mod alias_l813 {
    use super::*;
    pub type A0 = t_c84250e745ea;
    pub type A1 = Address<t_c84250e745ea>;
}

pub mod alias_l818 {
    use super::*;
    pub type A0 = t_902a598a35b3;
    pub type A1 = Address<t_902a598a35b3>;
}

pub mod alias_l823 {
    use super::*;
    pub type A0 = t_9d3d102131ad;
    pub type A1 = Address<t_9d3d102131ad>;
}

pub mod alias_l828 {
    use super::*;
    pub type A0 = t_eaa8e2258470;
    pub type A1 = Address<t_eaa8e2258470>;
}

pub mod alias_l833 {
    use super::*;
    pub type A0 = t_49f32ee8809f;
    pub type A1 = Address<t_49f32ee8809f>;
}

pub mod alias_l838 {
    use super::*;
    pub type A0 = t_0e973829986b;
    pub type A1 = Address<t_0e973829986b>;
}

pub mod alias_l843 {
    use super::*;
    pub type A0 = t_a16246e945c4;
    pub type A1 = Address<t_a16246e945c4>;
}

pub mod alias_l848 {
    use super::*;
    pub type A0 = t_ec5129aa08f0;
    pub type A1 = Address<t_ec5129aa08f0>;
}

pub mod alias_l853 {
    use super::*;
    pub type A0 = t_abf1d60f1e9d;
    pub type A1 = Address<t_abf1d60f1e9d>;
}

pub mod alias_l858 {
    use super::*;
    pub type A0 = t_b8c854a3590e;
    pub type A1 = Address<t_b8c854a3590e>;
}

pub mod alias_l863 {
    use super::*;
    pub type A0 = t_c83dc576a2ca;
    pub type A1 = Address<t_c83dc576a2ca>;
}

pub mod alias_l868 {
    use super::*;
    pub type A0 = t_d382ee6afcf4;
    pub type A1 = Address<t_d382ee6afcf4>;
}

pub mod alias_l873 {
    use super::*;
    pub type A0 = t_e8ed33d59614;
    pub type A1 = Address<t_e8ed33d59614>;
}

pub mod alias_l878 {
    use super::*;
    pub type A0 = t_59c98388411f;
    pub type A1 = Address<t_59c98388411f>;
}

pub mod alias_l883 {
    use super::*;
    pub type A0 = t_84421de86d9f;
    pub type A1 = Address<t_84421de86d9f>;
}

pub mod alias_l888 {
    use super::*;
    pub type A0 = t_e56e3528d5cf;
    pub type A1 = Address<t_e56e3528d5cf>;
}

pub mod alias_l893 {
    use super::*;
    pub type A0 = t_cf63636a746a;
    pub type A1 = Address<t_cf63636a746a>;
}

pub mod alias_l898 {
    use super::*;
    pub type A0 = t_8b7a903b2153;
    pub type A1 = Address<t_8b7a903b2153>;
}

pub mod alias_l906 {
    use super::*;
    pub type A0 = t_8a8dc73e6912;
    pub type A1 = Address<t_8a8dc73e6912>;
}

pub mod alias_l911 {
    use super::*;
    pub type A0 = t_c813ac9bdc14;
    pub type A1 = Address<t_c813ac9bdc14>;
}

pub mod alias_l916 {
    use super::*;
    pub type A0 = t_6c5c4deb266a;
    pub type A1 = Address<t_6c5c4deb266a>;
}

pub mod alias_l921 {
    use super::*;
    pub type A0 = t_da756b5fc6b0;
    pub type A1 = Address<t_da756b5fc6b0>;
}

pub mod alias_l926 {
    use super::*;
    pub type A0 = t_793133314b97;
    pub type A1 = Address<t_793133314b97>;
}

pub mod alias_l931 {
    use super::*;
    pub type A0 = t_b39a8ac18b27;
    pub type A1 = Address<t_b39a8ac18b27>;
}

pub mod alias_l936 {
    use super::*;
    pub type A0 = t_5f6270895f0a;
    pub type A1 = Address<t_5f6270895f0a>;
}

pub mod alias_l941 {
    use super::*;
    pub type A0 = t_6f7763f001b6;
    pub type A1 = Address<t_6f7763f001b6>;
}

pub mod alias_l946 {
    use super::*;
    pub type A0 = t_ffa0eee2339a;
    pub type A1 = Address<t_ffa0eee2339a>;
}

pub mod alias_l951 {
    use super::*;
    pub type A0 = t_c4f451916fbc;
    pub type A1 = Address<t_c4f451916fbc>;
}

pub mod alias_l956 {
    use super::*;
    pub type A0 = t_539d6ec608b9;
    pub type A1 = Address<t_539d6ec608b9>;
}

pub mod alias_l961 {
    use super::*;
    pub type A0 = t_e509236be2db;
    pub type A1 = Address<t_e509236be2db>;
}

pub mod alias_l966 {
    use super::*;
    pub type A0 = t_b4cf56875f70;
    pub type A1 = Address<t_b4cf56875f70>;
}

pub mod alias_l971 {
    use super::*;
    pub type A0 = t_901ec624e73c;
    pub type A1 = Address<t_901ec624e73c>;
}

pub mod alias_l976 {
    use super::*;
    pub type A0 = t_f29f4c4e0d9b;
    pub type A1 = Address<t_f29f4c4e0d9b>;
}

pub mod alias_l981 {
    use super::*;
    pub type A0 = t_72baf3a2654f;
    pub type A1 = Address<t_72baf3a2654f>;
}

pub mod alias_l986 {
    use super::*;
    pub type A0 = t_0e05000c513d;
    pub type A1 = Address<t_0e05000c513d>;
}

pub mod alias_l991 {
    use super::*;
    pub type A0 = t_253f094e1e73;
    pub type A1 = Address<t_253f094e1e73>;
}

pub mod alias_l996 {
    use super::*;
    pub type A0 = t_1ca377704dd5;
    pub type A1 = Address<t_1ca377704dd5>;
}

pub mod alias_l1001 {
    use super::*;
    pub type A0 = t_59029f6197c1;
    pub type A1 = Address<t_59029f6197c1>;
}

pub mod alias_l1006 {
    use super::*;
    pub type A0 = t_6a8915ad9c8b;
    pub type A1 = Address<t_6a8915ad9c8b>;
}

pub mod alias_l1014 {
    use super::*;
    pub type A0 = t_adee456ffe46;
    pub type A1 = Address<t_adee456ffe46>;
}

pub mod alias_l1019 {
    use super::*;
    pub type A0 = t_b161a946421e;
    pub type A1 = Address<t_b161a946421e>;
}

pub mod alias_l1024 {
    use super::*;
    pub type A0 = t_cba2bf16a893;
    pub type A1 = Address<t_cba2bf16a893>;
}

pub mod alias_l1029 {
    use super::*;
    pub type A0 = t_a0ea7fcb2cb3;
    pub type A1 = Address<t_a0ea7fcb2cb3>;
}

pub mod alias_l1034 {
    use super::*;
    pub type A0 = t_636542005cc6;
    pub type A1 = Address<t_636542005cc6>;
}

pub mod alias_l1039 {
    use super::*;
    pub type A0 = t_7ee23c085bc3;
    pub type A1 = Address<t_7ee23c085bc3>;
}

pub mod alias_l1044 {
    use super::*;
    pub type A0 = t_7e5ee2efd06b;
    pub type A1 = Address<t_7e5ee2efd06b>;
}

pub mod alias_l1052 {
    use super::*;
    pub type A0 = t_324ea9b736f3;
    pub type A1 = Address<t_324ea9b736f3>;
}

pub mod alias_l1057 {
    use super::*;
    pub type A0 = t_acc5b7b8f0f6;
    pub type A1 = Address<t_acc5b7b8f0f6>;
}

pub mod alias_l1062 {
    use super::*;
    pub type A0 = t_a2a793e11277;
    pub type A1 = Address<t_a2a793e11277>;
}

pub mod alias_l1067 {
    use super::*;
    pub type A0 = t_187b48362a34;
    pub type A1 = Address<t_187b48362a34>;
}

pub mod alias_l1072 {
    use super::*;
    pub type A0 = t_5dfb388a4035;
    pub type A1 = Address<t_5dfb388a4035>;
}

pub mod alias_l1077 {
    use super::*;
    pub type A0 = t_4e1f49a9c8ae;
    pub type A1 = Address<t_4e1f49a9c8ae>;
}

pub mod alias_l1082 {
    use super::*;
    pub type A0 = t_285a6063891d;
    pub type A1 = Address<t_285a6063891d>;
}

pub mod alias_l1087 {
    use super::*;
    pub type A0 = t_ed7d31c5ac72;
    pub type A1 = Address<t_ed7d31c5ac72>;
}

pub mod alias_l1092 {
    use super::*;
    pub type A0 = t_4e6a35ef1c3d;
    pub type A1 = Address<t_4e6a35ef1c3d>;
}

pub mod alias_l1097 {
    use super::*;
    pub type A0 = t_fe5dba535169;
    pub type A1 = Address<t_fe5dba535169>;
}

pub mod alias_l1102 {
    use super::*;
    pub type A0 = t_f0624d661ef9;
    pub type A1 = Address<t_f0624d661ef9>;
}

pub mod alias_l1107 {
    use super::*;
    pub type A0 = t_57e057c985b3;
    pub type A1 = Address<t_57e057c985b3>;
}

pub mod alias_l1112 {
    use super::*;
    pub type A0 = t_4a4ffecd6d74;
    pub type A1 = Address<t_4a4ffecd6d74>;
}

pub mod alias_l1117 {
    use super::*;
    pub type A0 = t_eeb105301392;
    pub type A1 = Address<t_eeb105301392>;
}

pub mod alias_l1122 {
    use super::*;
    pub type A0 = t_6ab2e5ea63f2;
    pub type A1 = Address<t_6ab2e5ea63f2>;
}

pub mod alias_l1127 {
    use super::*;
    pub type A0 = t_ba72d61f4d41;
    pub type A1 = Address<t_ba72d61f4d41>;
}

pub mod alias_l1132 {
    use super::*;
    pub type A0 = t_b5571fed7f64;
    pub type A1 = Address<t_b5571fed7f64>;
}

pub mod alias_l1140 {
    use super::*;
    pub type A0 = t_2bce61ab09c3;
    pub type A1 = Address<t_2bce61ab09c3>;
}

pub mod alias_l1145 {
    use super::*;
    pub type A0 = t_0d2c2444c702;
    pub type A1 = Address<t_0d2c2444c702>;
}

pub mod alias_l1150 {
    use super::*;
    pub type A0 = t_0c6033a2a79b;
    pub type A1 = Address<t_0c6033a2a79b>;
}

pub mod alias_l1155 {
    use super::*;
    pub type A0 = t_870d0affb64d;
    pub type A1 = Address<t_870d0affb64d>;
}

pub mod alias_l1160 {
    use super::*;
    pub type A0 = t_0a965115e24b;
    pub type A1 = Address<t_0a965115e24b>;
}

pub mod alias_l1165 {
    use super::*;
    pub type A0 = t_9812f6c8da59;
    pub type A1 = Address<t_9812f6c8da59>;
}

pub mod alias_l1170 {
    use super::*;
    pub type A0 = t_9772d8edfec0;
    pub type A1 = Address<t_9772d8edfec0>;
}

pub mod alias_l1175 {
    use super::*;
    pub type A0 = t_33f4a50b1005;
    pub type A1 = Address<t_33f4a50b1005>;
}

pub mod alias_l1180 {
    use super::*;
    pub type A0 = t_d09b95f80687;
    pub type A1 = Address<t_d09b95f80687>;
}

pub mod alias_l1185 {
    use super::*;
    pub type A0 = t_afa6143b4504;
    pub type A1 = Address<t_afa6143b4504>;
}

pub mod alias_l1190 {
    use super::*;
    pub type A0 = t_c007e168f913;
    pub type A1 = Address<t_c007e168f913>;
}

pub mod alias_l1195 {
    use super::*;
    pub type A0 = t_963089ed4ce3;
    pub type A1 = Address<t_963089ed4ce3>;
}

pub mod alias_l1200 {
    use super::*;
    pub type A0 = t_c1e4850f82b6;
    pub type A1 = Address<t_c1e4850f82b6>;
}

pub mod alias_l1205 {
    use super::*;
    pub type A0 = t_3cc76c58b8d6;
    pub type A1 = Address<t_3cc76c58b8d6>;
}

pub mod alias_l1210 {
    use super::*;
    pub type A0 = t_034f6e423f84;
    pub type A1 = Address<t_034f6e423f84>;
}

pub mod alias_l1215 {
    use super::*;
    pub type A0 = t_410e35c258d7;
    pub type A1 = Address<t_410e35c258d7>;
}

pub mod alias_l1220 {
    use super::*;
    pub type A0 = t_feda0bec3ff8;
    pub type A1 = Address<t_feda0bec3ff8>;
}

pub mod alias_l1225 {
    use super::*;
    pub type A0 = t_800e1261bb01;
    pub type A1 = Address<t_800e1261bb01>;
}

pub mod alias_l1230 {
    use super::*;
    pub type A0 = t_b84b80e86fba;
    pub type A1 = Address<t_b84b80e86fba>;
}

pub mod alias_l1235 {
    use super::*;
    pub type A0 = t_822925baaa51;
    pub type A1 = Address<t_822925baaa51>;
}

pub mod alias_l1240 {
    use super::*;
    pub type A0 = t_8fa0d97ea188;
    pub type A1 = Address<t_8fa0d97ea188>;
}

pub mod alias_l1245 {
    use super::*;
    pub type A0 = t_864cf621e0dc;
    pub type A1 = Address<t_864cf621e0dc>;
}

pub mod alias_l1250 {
    use super::*;
    pub type A0 = t_96bc6d8c91d9;
    pub type A1 = Address<t_96bc6d8c91d9>;
}

pub mod alias_l1255 {
    use super::*;
    pub type A0 = t_4b238d2fddda;
    pub type A1 = Address<t_4b238d2fddda>;
}

pub mod alias_l1260 {
    use super::*;
    pub type A0 = t_b35b08f68261;
    pub type A1 = Address<t_b35b08f68261>;
}

pub mod alias_l1265 {
    use super::*;
    pub type A0 = t_302de68ab2b7;
    pub type A1 = Address<t_302de68ab2b7>;
}

pub mod alias_l1270 {
    use super::*;
    pub type A0 = t_96793cec4137;
    pub type A1 = Address<t_96793cec4137>;
}

pub mod alias_l1275 {
    use super::*;
    pub type A0 = t_5d21db35d8d6;
    pub type A1 = Address<t_5d21db35d8d6>;
}

pub mod alias_l1280 {
    use super::*;
    pub type A0 = t_f77736d1e40b;
    pub type A1 = Address<t_f77736d1e40b>;
}

pub mod alias_l1285 {
    use super::*;
    pub type A0 = t_d57738207cdb;
    pub type A1 = Address<t_d57738207cdb>;
}

pub mod alias_l1290 {
    use super::*;
    pub type A0 = t_958817d0919f;
    pub type A1 = Address<t_958817d0919f>;
}

pub mod alias_l1295 {
    use super::*;
    pub type A0 = t_9361bd5e555b;
    pub type A1 = Address<t_9361bd5e555b>;
}

pub mod alias_l1300 {
    use super::*;
    pub type A0 = t_bd10236b32bb;
    pub type A1 = Address<t_bd10236b32bb>;
}

pub mod alias_l1305 {
    use super::*;
    pub type A0 = t_2bfe34522b4e;
    pub type A1 = Address<t_2bfe34522b4e>;
}

pub mod alias_l1310 {
    use super::*;
    pub type A0 = t_acc1fd82c37b;
    pub type A1 = Address<t_acc1fd82c37b>;
}

pub mod alias_l1315 {
    use super::*;
    pub type A0 = t_0e00949c3f1c;
    pub type A1 = Address<t_0e00949c3f1c>;
}

pub mod alias_l1320 {
    use super::*;
    pub type A0 = t_ae8ad7c19fe3;
    pub type A1 = Address<t_ae8ad7c19fe3>;
}

pub mod alias_l1325 {
    use super::*;
    pub type A0 = t_1b7b6e03d5b3;
    pub type A1 = Address<t_1b7b6e03d5b3>;
}

pub mod alias_l1330 {
    use super::*;
    pub type A0 = t_6ab9763eaab3;
    pub type A1 = Address<t_6ab9763eaab3>;
}

pub mod alias_l1335 {
    use super::*;
    pub type A0 = t_d99468343608;
    pub type A1 = Address<t_d99468343608>;
}

pub mod alias_l1340 {
    use super::*;
    pub type A0 = t_6a454fe93607;
    pub type A1 = Address<t_6a454fe93607>;
}

pub mod alias_l1345 {
    use super::*;
    pub type A0 = t_086d3dff0cf9;
    pub type A1 = Address<t_086d3dff0cf9>;
}

pub mod alias_l1350 {
    use super::*;
    pub type A0 = t_fc5c1cc9bc5e;
    pub type A1 = Address<t_fc5c1cc9bc5e>;
}

pub mod alias_l1355 {
    use super::*;
    pub type A0 = t_44d1a3dd14e0;
    pub type A1 = Address<t_44d1a3dd14e0>;
}

pub mod alias_l1360 {
    use super::*;
    pub type A0 = t_8dce37293ad3;
    pub type A1 = Address<t_8dce37293ad3>;
}

pub mod alias_l1365 {
    use super::*;
    pub type A0 = t_d18e942b5916;
    pub type A1 = Address<t_d18e942b5916>;
}

pub mod alias_l1373 {
    use super::*;
    pub type A0 = t_3000537fd382;
    pub type A1 = Address<t_3000537fd382>;
}

pub mod alias_l1378 {
    use super::*;
    pub type A0 = t_43037acc0d70;
    pub type A1 = Address<t_43037acc0d70>;
}

pub mod alias_l1383 {
    use super::*;
    pub type A0 = t_d8f8b19a7e2c;
    pub type A1 = Address<t_d8f8b19a7e2c>;
}

pub mod alias_l1388 {
    use super::*;
    pub type A0 = t_1b1221a5e85c;
    pub type A1 = Address<t_1b1221a5e85c>;
}

pub mod alias_l1393 {
    use super::*;
    pub type A0 = t_3d6dabe3e423;
    pub type A1 = Address<t_3d6dabe3e423>;
}

pub mod alias_l1398 {
    use super::*;
    pub type A0 = t_20dd53585f96;
    pub type A1 = Address<t_20dd53585f96>;
}

pub mod alias_l1403 {
    use super::*;
    pub type A0 = t_571dd647b3c2;
    pub type A1 = Address<t_571dd647b3c2>;
}

pub mod alias_l1408 {
    use super::*;
    pub type A0 = t_646ec5f8e2c8;
    pub type A1 = Address<t_646ec5f8e2c8>;
}

pub mod alias_l1413 {
    use super::*;
    pub type A0 = t_40839960747a;
    pub type A1 = Address<t_40839960747a>;
}

pub mod alias_l1418 {
    use super::*;
    pub type A0 = t_bba8457eac48;
    pub type A1 = Address<t_bba8457eac48>;
}

pub mod alias_l1426 {
    use super::*;
    pub type A0 = t_ae205cbcdfbd;
    pub type A1 = Address<t_ae205cbcdfbd>;
}

pub mod alias_l1434 {
    use super::*;
    pub type A0 = t_2e3425b3a84b;
    pub type A1 = Address<t_2e3425b3a84b>;
}

pub mod alias_l1439 {
    use super::*;
    pub type A0 = t_d6fb928bf968;
    pub type A1 = Address<t_d6fb928bf968>;
}

pub mod alias_l1444 {
    use super::*;
    pub type A0 = t_e9f462756d1e;
    pub type A1 = Address<t_e9f462756d1e>;
}

pub mod alias_l1458 {
    use super::*;
    pub type A0 = t_dc1f5508eb2f;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1463 {
    use super::*;
    pub type A0 = t_33a857d08c21;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1468 {
    use super::*;
    pub type A0 = t_21391c6d9ff5;
    pub type A1 = Address<t_21391c6d9ff5>;
}

pub mod alias_l1473 {
    use super::*;
    pub type A0 = t_e326cf0a7044;
    pub type A1 = Address<t_e326cf0a7044>;
}

pub mod alias_l1478 {
    use super::*;
    pub type A0 = t_d15ac2fccbf4;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1483 {
    use super::*;
    pub type A0 = t_3a50f867af2b;
    pub type A1 = Address<t_3a50f867af2b>;
}

pub mod alias_l1488 {
    use super::*;
    pub type A0 = t_2f826d8a0898;
    pub type A1 = Address<t_2f826d8a0898>;
}

pub mod alias_l1493 {
    use super::*;
    pub type A0 = t_fe0fd484df34;
    pub type A1 = Address<t_fe0fd484df34>;
}

pub mod alias_l1498 {
    use super::*;
    pub type A0 = t_49963138123c;
    pub type A1 = Address<t_49963138123c>;
}

pub mod alias_l1503 {
    use super::*;
    pub type A0 = t_f04485dd97db;
    pub type A1 = Address<t_f04485dd97db>;
}

pub mod alias_l1518 {
    use super::*;
    pub type A0 = t_0198cc09cb23;
    pub type A1 = Address<t_0198cc09cb23>;
}

pub mod alias_l1523 {
    use super::*;
    pub type A0 = t_e56d69471b87;
    pub type A1 = Address<t_e56d69471b87>;
}

pub mod alias_l1528 {
    use super::*;
    pub type A0 = t_1aa4cb0bcca7;
    pub type A1 = Address<t_1aa4cb0bcca7>;
}

pub mod alias_l1533 {
    use super::*;
    pub type A0 = t_827bc6afc4ca;
    pub type A1 = Address<t_827bc6afc4ca>;
}

pub mod alias_l1538 {
    use super::*;
    pub type A0 = t_ee6a25464a0c;
    pub type A1 = Address<t_ee6a25464a0c>;
}

pub mod alias_l1543 {
    use super::*;
    pub type A0 = t_6b73191a0a4b;
    pub type A1 = Address<t_6b73191a0a4b>;
}

pub mod alias_l1548 {
    use super::*;
    pub type A0 = t_d3a008ef1335;
    pub type A1 = Address<t_d3a008ef1335>;
}

pub mod alias_l1556 {
    use super::*;
    pub type A0 = t_0a63d925e5f3;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1561 {
    use super::*;
    pub type A0 = t_6c7e695506f3;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1566 {
    use super::*;
    pub type A0 = t_bfcdb1b37cd2;
    pub type A1 = Address<t_bfcdb1b37cd2>;
}

pub mod alias_l1571 {
    use super::*;
    pub type A0 = t_6a60cf248b88;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1576 {
    use super::*;
    pub type A0 = t_d9c8e109b766;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1581 {
    use super::*;
    pub type A0 = t_55980f9a178d;
    pub type A1 = Address<t_55980f9a178d>;
}

pub mod alias_l1586 {
    use super::*;
    pub type A0 = t_0aa7840cb874;
    pub type A1 = Address<t_0aa7840cb874>;
}

pub mod alias_l1591 {
    use super::*;
    pub type A0 = t_b402da1e9f15;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1596 {
    use super::*;
    pub type A0 = t_521660993db1;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1601 {
    use super::*;
    pub type A0 = t_00bc3738ba72;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1606 {
    use super::*;
    pub type A0 = t_4b565f53a6d8;
    pub type A1 = Address<t_4b565f53a6d8>;
}

pub mod alias_l1632 {
    use super::*;
    pub type A0 = t_5d32e5271a14;
    pub type A1 = Address<t_5d32e5271a14>;
}

pub mod alias_l1637 {
    use super::*;
    pub type A0 = t_05630473f256;
    pub type A1 = Address<t_05630473f256>;
}

pub mod alias_l1642 {
    use super::*;
    pub type A0 = t_9a52d0a5c2d8;
    pub type A1 = Address<t_9a52d0a5c2d8>;
}

pub mod alias_l1650 {
    use super::*;
    pub type A0 = t_5a55a71d9304;
    pub type A1 = Address<t_5a55a71d9304>;
}

pub mod alias_l1655 {
    use super::*;
    pub type A0 = t_0563bd78c79a;
    pub type A1 = Address<t_0563bd78c79a>;
}

pub mod alias_l1660 {
    use super::*;
    pub type A0 = t_9c2a9804d7d3;
    pub type A1 = Address<t_9c2a9804d7d3>;
}

pub mod alias_l1665 {
    use super::*;
    pub type A0 = t_fdd2388130b5;
    pub type A1 = Address<t_fdd2388130b5>;
}

pub mod alias_l1670 {
    use super::*;
    pub type A0 = t_1e232d6dfcca;
    pub type A1 = Address<t_1e232d6dfcca>;
}

pub mod alias_l1675 {
    use super::*;
    pub type A0 = t_9ac63b5efc50;
    pub type A1 = Address<t_9ac63b5efc50>;
}

pub mod alias_l1680 {
    use super::*;
    pub type A0 = t_6700c760a283;
    pub type A1 = Address<t_6700c760a283>;
}

pub mod alias_l1685 {
    use super::*;
    pub type A0 = t_62567263df37;
    pub type A1 = Address<t_62567263df37>;
}

pub mod alias_l1690 {
    use super::*;
    pub type A0 = t_1af851907331;
    pub type A1 = Address<t_1af851907331>;
}

pub mod alias_l1695 {
    use super::*;
    pub type A0 = t_2fc0c524431a;
    pub type A1 = Address<t_2fc0c524431a>;
}

pub mod alias_l1700 {
    use super::*;
    pub type A0 = t_47089487316c;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1705 {
    use super::*;
    pub type A0 = t_b68640470f12;
    pub type A1 = Address<t_b68640470f12>;
}

pub mod alias_l1710 {
    use super::*;
    pub type A0 = t_084d565a47ac;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1715 {
    use super::*;
    pub type A0 = t_52019ede0cdb;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1720 {
    use super::*;
    pub type A0 = t_8c01be5d5ead;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1725 {
    use super::*;
    pub type A0 = t_91e8acb85ed8;
    pub type A1 = Address<t_91e8acb85ed8>;
}

pub mod alias_l1730 {
    use super::*;
    pub type A0 = t_23ce83bcb9ce;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1735 {
    use super::*;
    pub type A0 = t_f1a9fabf4022;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1745 {
    use super::*;
    pub type A0 = t_6accd497825c;
    pub type A1 = Address<t_6accd497825c>;
}

pub mod alias_l1750 {
    use super::*;
    pub type A0 = t_e032324ef236;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1760 {
    use super::*;
    pub type A0 = t_51f87740b9fc;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1765 {
    use super::*;
    pub type A0 = t_b5cd9520e20b;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1775 {
    use super::*;
    pub type A0 = t_f3eda90f41d5;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1780 {
    use super::*;
    pub type A0 = t_44f8f717411e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1785 {
    use super::*;
    pub type A0 = t_c0a04cd10bed;
    pub type A1 = Address<t_c0a04cd10bed>;
}

pub mod alias_l1790 {
    use super::*;
    pub type A0 = t_4ab7de6da030;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1795 {
    use super::*;
    pub type A0 = t_b978e344c1e1;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1800 {
    use super::*;
    pub type A0 = t_53011625103e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1805 {
    use super::*;
    pub type A0 = t_8f7add24787a;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1810 {
    use super::*;
    pub type A0 = t_02530651af7d;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1815 {
    use super::*;
    pub type A0 = t_af4135a20171;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1820 {
    use super::*;
    pub type A0 = t_59f2097d8b33;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1830 {
    use super::*;
    pub type A0 = t_1e3a3e98405f;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1835 {
    use super::*;
    pub type A0 = t_f010dd405dab;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1840 {
    use super::*;
    pub type A0 = t_77d9bce4a13d;
    pub type A1 = Address<t_77d9bce4a13d>;
}

pub mod alias_l1845 {
    use super::*;
    pub type A0 = t_2f01edf10e2a;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1850 {
    use super::*;
    pub type A0 = t_1059cb443a9b;
    pub type A1 = Address<t_1059cb443a9b>;
}

pub mod alias_l1855 {
    use super::*;
    pub type A0 = t_baa3d05d6ec1;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1860 {
    use super::*;
    pub type A0 = t_f306cfafe03e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1865 {
    use super::*;
    pub type A0 = t_bc53006a2d2f;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1870 {
    use super::*;
    pub type A0 = t_c68320bb4512;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1875 {
    use super::*;
    pub type A0 = t_913394f5901b;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1880 {
    use super::*;
    pub type A0 = t_173dc295033e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1885 {
    use super::*;
    pub type A0 = t_2f7033a87bf7;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l1890 {
    use super::*;
    pub type A0 = t_9d904ef95af3;
    pub type A1 = Address<t_9d904ef95af3>;
}

pub mod alias_l1895 {
    use super::*;
    pub type A0 = t_a56145270ce6;
    pub type A1 = Address<t_a56145270ce6>;
}

pub mod alias_l1900 {
    use super::*;
    pub type A0 = t_5c07ce13958f;
    pub type A1 = Address<t_5c07ce13958f>;
}

pub mod alias_l1905 {
    use super::*;
    pub type A0 = t_a0af9f865bf6;
    pub type A1 = Address<t_a0af9f865bf6>;
}

pub mod alias_l1925 {
    use super::*;
    pub type A0 = t_092f8b73aabd;
    pub type A1 = Address<t_092f8b73aabd>;
}

pub mod alias_l1930 {
    use super::*;
    pub type A0 = t_e2cec9925dbb;
    pub type A1 = Address<t_e2cec9925dbb>;
}

pub mod alias_l1935 {
    use super::*;
    pub type A0 = t_c324ac6ad0d7;
    pub type A1 = Address<t_c324ac6ad0d7>;
}

pub mod alias_l1940 {
    use super::*;
    pub type A0 = t_8bd0dddcee10;
    pub type A1 = Address<t_8bd0dddcee10>;
}

pub mod alias_l1945 {
    use super::*;
    pub type A0 = t_2d9c0e443461;
    pub type A1 = Address<t_2d9c0e443461>;
}

pub mod alias_l1950 {
    use super::*;
    pub type A0 = t_82193e6209c2;
    pub type A1 = Address<t_82193e6209c2>;
}

pub mod alias_l1958 {
    use super::*;
    pub type A0 = t_c22e7e8d9755;
    pub type A1 = Address<t_c22e7e8d9755>;
}

pub mod alias_l1966 {
    use super::*;
    pub type A0 = t_bf4a57438cb0;
    pub type A1 = Address<t_bf4a57438cb0>;
}

pub mod alias_l1971 {
    use super::*;
    pub type A0 = t_54e32828a68e;
    pub type A1 = Address<t_54e32828a68e>;
}

pub mod alias_l1976 {
    use super::*;
    pub type A0 = t_c23b7e56eb84;
    pub type A1 = Address<t_c23b7e56eb84>;
}

pub mod alias_l1981 {
    use super::*;
    pub type A0 = t_9907703a0c5f;
    pub type A1 = Address<t_9907703a0c5f>;
}

pub mod alias_l1989 {
    use super::*;
    pub type A0 = t_db683659ff69;
    pub type A1 = Address<t_db683659ff69>;
}

pub mod alias_l1994 {
    use super::*;
    pub type A0 = t_2dd6065b535d;
    pub type A1 = Address<t_2dd6065b535d>;
}

pub mod alias_l2002 {
    use super::*;
    pub type A0 = t_ea2b8a878841;
    pub type A1 = Address<t_ea2b8a878841>;
}

pub mod alias_l2007 {
    use super::*;
    pub type A0 = t_91f4247ac1b9;
    pub type A1 = Address<t_91f4247ac1b9>;
}

pub mod alias_l2012 {
    use super::*;
    pub type A0 = t_a5bcf3559f19;
    pub type A1 = Address<t_a5bcf3559f19>;
}

pub mod alias_l2017 {
    use super::*;
    pub type A0 = t_32370d1130db;
    pub type A1 = Address<t_32370d1130db>;
}

pub mod alias_l2022 {
    use super::*;
    pub type A0 = t_691b59693f4c;
    pub type A1 = Address<t_691b59693f4c>;
}

pub mod alias_l2030 {
    use super::*;
    pub type A0 = t_778b4967199a;
    pub type A1 = Address<t_778b4967199a>;
}

pub mod alias_l2038 {
    use super::*;
    pub type A0 = t_91bb08f86bac;
    pub type A1 = Address<t_91bb08f86bac>;
}

pub mod alias_l2055 {
    use super::*;
    pub type A0 = t_ae283e096c21;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2060 {
    use super::*;
    pub type A0 = t_727739213f1e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2065 {
    use super::*;
    pub type A0 = t_4e28903f8cb8;
    pub type A1 = Address<t_4e28903f8cb8>;
}

pub mod alias_l2070 {
    use super::*;
    pub type A0 = t_76b5785890c9;
    pub type A1 = Address<t_76b5785890c9>;
}

pub mod alias_l2075 {
    use super::*;
    pub type A0 = t_ee9183fedfff;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2080 {
    use super::*;
    pub type A0 = t_90e1c7257bc6;
    pub type A1 = Address<t_90e1c7257bc6>;
}

pub mod alias_l2091 {
    use super::*;
    pub type A0 = t_c577a3c77081;
    pub type A1 = Address<t_c577a3c77081>;
}

pub mod alias_l2096 {
    use super::*;
    pub type A0 = t_a873b841019d;
    pub type A1 = Address<t_a873b841019d>;
}

pub mod alias_l2101 {
    use super::*;
    pub type A0 = t_1be356c6fbf7;
    pub type A1 = Address<t_1be356c6fbf7>;
}

pub mod alias_l2106 {
    use super::*;
    pub type A0 = t_15072ded30c4;
    pub type A1 = Address<t_15072ded30c4>;
}

pub mod alias_l2111 {
    use super::*;
    pub type A0 = t_5196cd129c37;
    pub type A1 = Address<t_5196cd129c37>;
}

pub mod alias_l2116 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l2118 {
    use super::*;
    pub type A0 = t_5fd3760ac5ed;
    pub type A1 = Address<t_5fd3760ac5ed>;
}

pub mod alias_l2120 {
    use super::*;
    pub type A0 = t_04d26082a7a2;
    pub type A1 = Address<t_04d26082a7a2>;
}

pub mod alias_l2122 {
    use super::*;
    pub type A0 = Address<t_04d26082a7a2>;
}

pub mod alias_l2124 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l2126 {
    use super::*;
    pub type A0 = t_4e7262b45ab9;
    pub type A1 = Address<t_4e7262b45ab9>;
}

pub mod alias_l2128 {
    use super::*;
    pub type A0 = t_28435f27627d;
    pub type A1 = Address<t_28435f27627d>;
}

pub mod alias_l2130 {
    use super::*;
    pub type A0 = t_5484efe72ed7;
    pub type A1 = Address<t_5484efe72ed7>;
}

pub mod alias_l2132 {
    use super::*;
    pub type A0 = t_fa988a243f90;
    pub type A1 = Address<t_fa988a243f90>;
}

pub mod alias_l2134 {
    use super::*;
    pub type A0 = t_de4125428281;
    pub type A1 = Address<t_de4125428281>;
}

pub mod alias_l2136 {
    use super::*;
    pub type A0 = t_de4125428281;
}

pub mod alias_l2206 {
    use super::*;
    pub type A0 = Address<t_5fd3760ac5ed>;
}

pub mod alias_l2208 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l2210 {
    use super::*;
    pub type A0 = t_ce36b2111292;
    pub type A1 = Address<t_ce36b2111292>;
}

pub mod alias_l2224 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l2226 {
    use super::*;
    pub type A0 = t_482d8d0b333d;
    pub type A1 = Address<t_482d8d0b333d>;
}

pub mod alias_l2228 {
    use super::*;
    pub type A0 = t_d5f8803400c9;
    pub type A1 = Address<t_d5f8803400c9>;
}

pub mod alias_l2230 {
    use super::*;
    pub type A0 = t_fa67fefc92a1;
    pub type A1 = Address<t_fa67fefc92a1>;
}

pub mod alias_l2232 {
    use super::*;
    pub type A0 = t_fa67fefc92a1;
}

pub mod alias_l2234 {
    use super::*;
    pub type A0 = t_30140b67a92e;
    pub type A1 = Address<t_30140b67a92e>;
}

pub mod alias_l2236 {
    use super::*;
    pub type A0 = t_30140b67a92e;
}

pub mod alias_l2238 {
    use super::*;
    pub type A0 = t_30140b67a92e;
}

pub mod alias_l2240 {
    use super::*;
    pub type A0 = t_32a6939c8586;
    pub type A1 = Address<t_32a6939c8586>;
}

pub mod alias_l2242 {
    use super::*;
    pub type A0 = t_2b5c2f17fed6;
    pub type A1 = Address<t_2b5c2f17fed6>;
}

pub mod alias_l2244 {
    use super::*;
    pub type A0 = t_2b5c2f17fed6;
}

pub mod alias_l2246 {
    use super::*;
    pub type A0 = t_bb76df3c134a;
    pub type A1 = Address<t_bb76df3c134a>;
}

pub mod alias_l2248 {
    use super::*;
    pub type A0 = t_39472191c743;
    pub type A1 = Address<t_39472191c743>;
}

pub mod alias_l2250 {
    use super::*;
    pub type A0 = t_a0474f8c0811;
    pub type A1 = Address<t_a0474f8c0811>;
}

pub mod alias_l2252 {
    use super::*;
    pub type A0 = t_9bbc0df2157e;
}

pub mod alias_l2254 {
    use super::*;
    pub type A0 = t_6f44067859e5;
    pub type A1 = Address<t_6f44067859e5>;
}

pub mod alias_l2256 {
    use super::*;
    pub type A0 = t_ec87f6a0d8aa;
    pub type A1 = Address<t_ec87f6a0d8aa>;
}

pub mod alias_l2258 {
    use super::*;
    pub type A0 = t_ec87f6a0d8aa;
}

pub mod alias_l2260 {
    use super::*;
    pub type A0 = t_ac520b3f9a6b;
    pub type A1 = Address<t_ac520b3f9a6b>;
}

pub mod alias_l2262 {
    use super::*;
    pub type A0 = t_c9c365e9e75d;
    pub type A1 = Address<t_c9c365e9e75d>;
}

pub mod alias_l2264 {
    use super::*;
    pub type A0 = t_d3401f1f0509;
    pub type A1 = Address<t_d3401f1f0509>;
}

pub mod alias_l2266 {
    use super::*;
    pub type A0 = Address<t_d3401f1f0509>;
}

pub mod alias_l2268 {
    use super::*;
    pub type A0 = t_b9e6eda47c42;
    pub type A1 = Address<t_b9e6eda47c42>;
}

pub mod alias_l2270 {
    use super::*;
    pub type A0 = Address<t_b9e6eda47c42>;
}

pub mod alias_l2272 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l2274 {
    use super::*;
    pub type A0 = t_633e66262301;
    pub type A1 = Address<t_633e66262301>;
}

pub mod alias_l2276 {
    use super::*;
    pub type A0 = t_89331bec4bba;
    pub type A1 = Address<t_89331bec4bba>;
}

pub mod alias_l2278 {
    use super::*;
    pub type A0 = t_03d4f2b2423d;
    pub type A1 = Address<t_03d4f2b2423d>;
}

pub mod alias_l2280 {
    use super::*;
    pub type A0 = t_627cf47e8ca7;
    pub type A1 = Address<t_627cf47e8ca7>;
}

pub mod alias_l2282 {
    use super::*;
    pub type A0 = t_627cf47e8ca7;
}

pub mod alias_l2284 {
    use super::*;
    pub type A0 = t_730c5ce0c542;
    pub type A1 = Address<t_730c5ce0c542>;
}

pub mod alias_l2286 {
    use super::*;
    pub type A0 = t_730c5ce0c542;
}

pub mod alias_l2288 {
    use super::*;
    pub type A0 = t_ca9b7444fc70;
    pub type A1 = Address<t_ca9b7444fc70>;
}

pub mod alias_l2290 {
    use super::*;
    pub type A0 = t_1801dc4830e5;
    pub type A1 = Address<t_1801dc4830e5>;
}

pub mod alias_l2292 {
    use super::*;
    pub type A0 = t_a485dad48787;
    pub type A1 = Address<t_a485dad48787>;
}

pub mod alias_l2294 {
    use super::*;
    pub type A0 = t_092ddfd804eb;
    pub type A1 = Address<t_092ddfd804eb>;
}

pub mod alias_l2296 {
    use super::*;
    pub type A0 = t_092ddfd804eb;
}

pub mod alias_l2298 {
    use super::*;
    pub type A0 = t_409bfe6e453d;
    pub type A1 = Address<t_409bfe6e453d>;
}

pub mod alias_l2300 {
    use super::*;
    pub type A0 = t_bce96f9772e3;
    pub type A1 = Address<t_bce96f9772e3>;
}

pub mod alias_l2302 {
    use super::*;
    pub type A0 = t_c3fd39d62aff;
    pub type A1 = Address<t_c3fd39d62aff>;
}

pub mod alias_l2304 {
    use super::*;
    pub type A0 = t_ef59add06b29;
    pub type A1 = Address<t_ef59add06b29>;
}

pub mod alias_l2306 {
    use super::*;
    pub type A0 = Address<Address<i16>>;
}

pub mod alias_l2308 {
    use super::*;
    pub type A0 = t_b97acc974f3f;
    pub type A1 = Address<t_b97acc974f3f>;
}

pub mod alias_l2310 {
    use super::*;
    pub type A0 = t_6d69cbb7374d;
    pub type A1 = Address<t_6d69cbb7374d>;
}

pub mod alias_l2312 {
    use super::*;
    pub type A0 = t_17463d5dad35;
    pub type A1 = Address<t_17463d5dad35>;
}

pub mod alias_l2314 {
    use super::*;
    pub type A0 = t_bb8a112e2f00;
    pub type A1 = Address<t_bb8a112e2f00>;
}

pub mod alias_l2316 {
    use super::*;
    pub type A0 = t_8de782ce8af7;
    pub type A1 = Address<t_8de782ce8af7>;
}

pub mod alias_l2318 {
    use super::*;
    pub type A0 = t_f187f44ecbed;
    pub type A1 = Address<t_f187f44ecbed>;
}

pub mod alias_l2594 {
    use super::*;
    pub type A0 = t_525edec8d4e7;
    pub type A1 = Address<t_525edec8d4e7>;
}

pub mod alias_l2596 {
    use super::*;
    pub type A0 = t_4a87cf9300df;
    pub type A1 = Address<t_4a87cf9300df>;
}

pub mod alias_l2598 {
    use super::*;
    pub type A0 = t_decbfceef172;
    pub type A1 = Address<t_decbfceef172>;
}

pub mod alias_l2600 {
    use super::*;
    pub type A0 = t_decbfceef172;
}

pub mod alias_l2623 {
    use super::*;
    pub type A0 = t_82730d4ea9ba;
    pub type A1 = Address<t_82730d4ea9ba>;
}

pub mod alias_l2625 {
    use super::*;
    pub type A0 = t_69e12c459a52;
    pub type A1 = Address<t_69e12c459a52>;
}

pub mod alias_l2627 {
    use super::*;
    pub type A0 = t_d30ceb7d3a3d;
    pub type A1 = Address<t_d30ceb7d3a3d>;
}

pub mod alias_l2629 {
    use super::*;
    pub type A0 = t_7cb6d5f557c0;
    pub type A1 = Address<t_7cb6d5f557c0>;
}

pub mod alias_l2673 {
    use super::*;
    pub type A0 = Address<t_6f44067859e5>;
}

pub mod alias_l2675 {
    use super::*;
    pub type A0 = Address<t_69e12c459a52>;
}

pub mod alias_l2677 {
    use super::*;
    pub type A0 = Address<t_633e66262301>;
}

pub mod alias_l2679 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l2681 {
    use super::*;
    pub type A0 = Address<t_30140b67a92e>;
}

pub mod alias_l2690 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l2692 {
    use super::*;
    pub type A0 = t_9e38acb35fbe;
    pub type A1 = Address<t_9e38acb35fbe>;
}

pub mod alias_l2701 {
    use super::*;
    pub type A0 = t_300f5dc89818;
    pub type A1 = Address<t_300f5dc89818>;
}

pub mod alias_l2703 {
    use super::*;
    pub type A0 = t_a46a405b9465;
    pub type A1 = Address<t_a46a405b9465>;
}

pub mod alias_l2705 {
    use super::*;
    pub type A0 = Address<t_a46a405b9465>;
}

pub mod alias_l2707 {
    use super::*;
    pub type A0 = t_5c940ae036a8;
    pub type A1 = Address<t_5c940ae036a8>;
}

pub mod alias_l2709 {
    use super::*;
    pub type A0 = t_5c940ae036a8;
}

pub mod alias_l2711 {
    use super::*;
    pub type A0 = Address<t_5c940ae036a8>;
}

pub mod alias_l2713 {
    use super::*;
    pub type A0 = Address<t_39472191c743>;
}

pub mod alias_l2715 {
    use super::*;
    pub type A0 = t_0ede1bfb8d71;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2717 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l2755 {
    use super::*;
    pub type A0 = t_d01078937930;
    pub type A1 = Address<t_d01078937930>;
}

pub mod alias_l2757 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l2785 {
    use super::*;
    pub type A0 = Address<t_300f5dc89818>;
}

pub mod alias_l2787 {
    use super::*;
    pub type A0 = Address<t_d01078937930>;
}

pub mod alias_l2789 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l2791 {
    use super::*;
    pub type A0 = u16;
}

pub mod alias_l2793 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l2795 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l2797 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l2799 {
    use super::*;
    pub type A0 = Address<t_9bbc0df2157e>;
}

pub mod alias_l2801 {
    use super::*;
    pub type A0 = t_9bbc0df2157e;
    pub type A1 = Address<t_9bbc0df2157e>;
}

pub mod alias_l2810 {
    use super::*;
    pub type A0 = t_7e49763e4bfe;
    pub type A1 = Address<t_7e49763e4bfe>;
}

pub mod alias_l2812 {
    use super::*;
    pub type A0 = t_7e49763e4bfe;
}

pub mod alias_l2822 {
    use super::*;
    pub type A0 = t_04d26082a7a2;
}

pub mod alias_l2824 {
    use super::*;
    pub type A0 = t_90b4e80c4b85;
    pub type A1 = Address<t_90b4e80c4b85>;
}

pub mod alias_l2826 {
    use super::*;
    pub type A0 = t_90b4e80c4b85;
}

pub mod alias_l2842 {
    use super::*;
    pub type A0 = t_b094c5277644;
    pub type A1 = Address<t_b094c5277644>;
}

pub mod alias_l2844 {
    use super::*;
    pub type A0 = t_91ac713f053f;
    pub type A1 = Address<t_91ac713f053f>;
}

pub mod alias_l2846 {
    use super::*;
    pub type A0 = t_91ac713f053f;
}

pub mod alias_l2860 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l2862 {
    use super::*;
    pub type A0 = t_b094c5277644;
}

pub mod alias_l2864 {
    use super::*;
    pub type A0 = Address<t_b094c5277644>;
}

pub mod alias_l2866 {
    use super::*;
    pub type A0 = t_31cee394959e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2871 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l2873 {
    use super::*;
    pub type A0 = t_15dd9826f96e;
    pub type A1 = Address<t_15dd9826f96e>;
}

pub mod alias_l2880 {
    use super::*;
    pub type A0 = t_3a31a65ef67d;
    pub type A1 = Address<Unknown>;
}

/// Incomplete decompiler type: _struct_317.
pub enum t_68762d4fce91 {}

pub mod alias_l2885 {
    use super::*;
    pub type A0 = t_68762d4fce91;
    pub type A1 = Address<Unknown>;
}

/// Incomplete decompiler type: _struct_316.
pub enum t_6650f3d6fe52 {}

pub mod alias_l2895 {
    use super::*;
    pub type A0 = t_6650f3d6fe52;
    pub type A1 = Address<Unknown>;
}

/// Incomplete decompiler type: _struct_315.
pub enum t_1d67573570b5 {}

pub mod alias_l2906 {
    use super::*;
    pub type A0 = t_1d67573570b5;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2918 {
    use super::*;
    pub type A0 = t_380b96258bfe;
    pub type A1 = Address<t_380b96258bfe>;
}

pub mod alias_l2926 {
    use super::*;
    pub type A0 = Address<t_380b96258bfe>;
}

pub mod alias_l2928 {
    use super::*;
    pub type A0 = t_f610c92dbd31;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l2933 {
    use super::*;
    pub type A0 = t_099cffa171c0;
    pub type A1 = Address<t_099cffa171c0>;
}

pub mod alias_l2935 {
    use super::*;
    pub type A0 = t_099cffa171c0;
}

pub mod alias_l2942 {
    use super::*;
    pub type A0 = t_dfb3f4aafa67;
    pub type A1 = Address<t_dfb3f4aafa67>;
}

pub mod alias_l2944 {
    use super::*;
    pub type A0 = Address<t_cbc0908ab753>;
}

pub mod alias_l2946 {
    use super::*;
    pub type A0 = t_6637dfe39500;
    pub type A1 = Address<t_6637dfe39500>;
}

pub mod alias_l2972 {
    use super::*;
    pub type A0 = t_efa0d9408ae1;
    pub type A1 = Address<t_efa0d9408ae1>;
}

pub mod alias_l2974 {
    use super::*;
    pub type A0 = t_98a0319bda06;
    pub type A1 = Address<t_98a0319bda06>;
}

pub mod alias_l3002 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l3004 {
    use super::*;
    pub type A0 = t_98ebdc684dea;
    pub type A1 = Address<t_98ebdc684dea>;
}

pub mod alias_l3006 {
    use super::*;
    pub type A0 = t_98ebdc684dea;
}

pub mod alias_l3008 {
    use super::*;
    pub type A0 = Address<t_98ebdc684dea>;
}

pub mod alias_l3018 {
    use super::*;
    pub type A0 = i8;
}

pub mod alias_l3020 {
    use super::*;
    pub type A0 = Address<i8>;
}

pub mod alias_l3022 {
    use super::*;
    pub type A0 = u64;
}

pub mod alias_l3024 {
    use super::*;
    pub type A0 = t_f29900df9ce5;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3029 {
    use super::*;
    pub type A0 = Address<i8>;
}

pub mod alias_l3031 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

/// Incomplete decompiler type: _TP_CALLBACK_ENVIRON_V3.
pub enum t_15f3b5475a02 {}

pub mod alias_l3033 {
    use super::*;
    pub type A0 = t_15f3b5475a02;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3035 {
    use super::*;
    pub type A0 = t_15f3b5475a02;
}

pub mod alias_l3037 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3039 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l3041 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3043 {
    use super::*;
    pub type A0 = CodeAddress;
}

/// Incomplete decompiler type: _union_401.
pub enum t_6283f260653f {}

pub mod alias_l3045 {
    use super::*;
    pub type A0 = t_6283f260653f;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3054 {
    use super::*;
    pub type A0 = Enum32;
}

/// Incomplete decompiler type: _struct_402.
pub enum t_4f57b4157ebe {}

pub mod alias_l3056 {
    use super::*;
    pub type A0 = t_4f57b4157ebe;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3082 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3084 {
    use super::*;
    pub type A0 = t_d4858952b580;
    pub type A1 = Address<t_d4858952b580>;
}

pub mod alias_l3100 {
    use super::*;
    pub type A0 = Address<i16>;
}

pub mod alias_l3102 {
    use super::*;
    pub type A0 = t_ee0beb379500;
    pub type A1 = Address<t_ee0beb379500>;
}

pub mod alias_l3104 {
    use super::*;
    pub type A0 = Address<t_ee0beb379500>;
}

pub mod alias_l3106 {
    use super::*;
    pub type A0 = Address<t_ee0beb379500>;
}

pub mod alias_l3112 {
    use super::*;
    pub type A0 = u32;
}

/// Incomplete decompiler type: _SLIST_HEADER.
pub enum t_5be79252e409 {}

pub mod alias_l3114 {
    use super::*;
    pub type A0 = t_5be79252e409;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3123 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3125 {
    use super::*;
    pub type A0 = i16;
}

pub mod alias_l3127 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l3129 {
    use super::*;
    pub type A0 = t_49f218b6b015;
    pub type A1 = Address<t_49f218b6b015>;
}

pub mod alias_l3142 {
    use super::*;
    pub type A0 = t_782ce141addd;
    pub type A1 = Address<t_782ce141addd>;
}

pub mod alias_l3144 {
    use super::*;
    pub type A0 = Address<t_782ce141addd>;
}

pub mod alias_l3156 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3158 {
    use super::*;
    pub type A0 = t_a194e0e83612;
    pub type A1 = Address<t_a194e0e83612>;
}

pub mod alias_l3165 {
    use super::*;
    pub type A0 = Address<t_d4858952b580>;
}

pub mod alias_l3167 {
    use super::*;
    pub type A0 = Address<t_49f218b6b015>;
}

pub mod alias_l3169 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3171 {
    use super::*;
    pub type A0 = Address<t_627cf47e8ca7>;
}

pub mod alias_l3173 {
    use super::*;
    pub type A0 = Address<t_a194e0e83612>;
}

pub mod alias_l3175 {
    use super::*;
    pub type A0 = Address<t_fd609cf74451>;
}

pub mod alias_l3177 {
    use super::*;
    pub type A0 = t_5df0ca05dbf4;
    pub type A1 = Address<t_5df0ca05dbf4>;
}

pub mod alias_l3183 {
    use super::*;
    pub type A0 = t_453047c9bfb3;
    pub type A1 = Address<t_453047c9bfb3>;
}

pub mod alias_l3185 {
    use super::*;
    pub type A0 = Address<t_453047c9bfb3>;
}

pub mod alias_l3191 {
    use super::*;
    pub type A0 = t_a2484b182201;
    pub type A1 = Address<t_a2484b182201>;
}

pub mod alias_l3198 {
    use super::*;
    pub type A0 = t_81ac937c454e;
    pub type A1 = Address<t_81ac937c454e>;
}

pub mod alias_l3204 {
    use super::*;
    pub type A0 = Address<t_4c0908fcb2fb>;
}

pub mod alias_l3206 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3208 {
    use super::*;
    pub type A0 = Address<i32>;
}

pub mod alias_l3210 {
    use super::*;
    pub type A0 = CodeAddress;
}

pub mod alias_l3212 {
    use super::*;
    pub type A0 = Address<t_272e88af6e21>;
}

pub mod alias_l3214 {
    use super::*;
    pub type A0 = Address<i32>;
}

pub mod alias_l3216 {
    use super::*;
    pub type A0 = Address<t_81ac937c454e>;
}

pub mod alias_l3218 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3220 {
    use super::*;
    pub type A0 = Address<t_5df0ca05dbf4>;
}

pub mod alias_l3222 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l3227 {
    use super::*;
    pub type A0 = Address<u32>;
}

pub mod alias_l3229 {
    use super::*;
    pub type A0 = Address<u32>;
}

pub mod alias_l3231 {
    use super::*;
    pub type A0 = Address<t_a2484b182201>;
}

pub mod alias_l3233 {
    use super::*;
    pub type A0 = Address<i32>;
}

pub mod alias_l3235 {
    use super::*;
    pub type A0 = t_69c576e42fd8;
    pub type A1 = Address<t_69c576e42fd8>;
}

pub mod alias_l3237 {
    use super::*;
    pub type A0 = Address<t_69c576e42fd8>;
}

pub mod alias_l3243 {
    use super::*;
    pub type A0 = Address<t_dbd699519b88>;
}

pub mod alias_l3245 {
    use super::*;
    pub type A0 = Address<Address<Unknown>>;
}

pub mod alias_l3247 {
    use super::*;
    pub type A0 = Address<Address<t_5df0ca05dbf4>>;
}

pub mod alias_l3249 {
    use super::*;
    pub type A0 = Address<Unknown>;
}

pub mod alias_l3251 {
    use super::*;
    pub type A0 = t_60529803e0f9;
    pub type A1 = Address<t_60529803e0f9>;
}

/// Incomplete decompiler type: IMAGE_RESOURCE_DIRECTORY_ENTRY_NameStruct.
pub enum t_4467f88a5a08 {}

pub mod alias_l3259 {
    use super::*;
    pub type A0 = t_4467f88a5a08;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3266 {
    use super::*;
    pub type A0 = t_61a649e3740b;
    pub type A1 = Address<t_61a649e3740b>;
}

pub mod alias_l3273 {
    use super::*;
    pub type A0 = t_12a47bfd922d;
    pub type A1 = Address<t_12a47bfd922d>;
}

pub mod alias_l3282 {
    use super::*;
    pub type A0 = t_c5ed2037a9da;
    pub type A1 = Address<t_c5ed2037a9da>;
}

pub mod alias_l3295 {
    use super::*;
    pub type A0 = t_106fb6e7812c;
    pub type A1 = Address<t_106fb6e7812c>;
}

pub mod alias_l3302 {
    use super::*;
    pub type A0 = t_6e1c23316e97;
    pub type A1 = Address<t_6e1c23316e97>;
}

pub mod alias_l3309 {
    use super::*;
    pub type A0 = t_a7b30136199b;
    pub type A1 = Address<t_a7b30136199b>;
}

pub mod alias_l3316 {
    use super::*;
    pub type A0 = t_1a676e47f244;
    pub type A1 = Address<t_1a676e47f244>;
}

pub mod alias_l3323 {
    use super::*;
    pub type A0 = t_48dc04407053;
    pub type A1 = Address<t_48dc04407053>;
}

pub mod alias_l3330 {
    use super::*;
    pub type A0 = t_796044dfeb21;
    pub type A1 = Address<t_796044dfeb21>;
}

pub mod alias_l3337 {
    use super::*;
    pub type A0 = t_d6601bc9eca9;
    pub type A1 = Address<t_d6601bc9eca9>;
}

pub mod alias_l3339 {
    use super::*;
    pub type A0 = t_048cb0a340e5;
    pub type A1 = Address<t_048cb0a340e5>;
}

/// Incomplete decompiler type: IMAGE_NT_HEADERS64.
pub enum t_807803698433 {}

pub mod alias_l3399 {
    use super::*;
    pub type A0 = t_807803698433;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3401 {
    use super::*;
    pub type A0 = t_47fc86ee7976;
    pub type A1 = Address<t_47fc86ee7976>;
}

/// Incomplete decompiler type: IMAGE_OPTIONAL_HEADER64.
pub enum t_4c02c7411dc1 {}

pub mod alias_l3403 {
    use super::*;
    pub type A0 = t_4c02c7411dc1;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3405 {
    use super::*;
    pub type A0 = t_7342e1154646;
    pub type A1 = Address<t_7342e1154646>;
}

pub mod alias_l3461 {
    use super::*;
    pub type A0 = t_c189ba78152a;
    pub type A1 = Address<t_c189ba78152a>;
}

pub mod alias_l3470 {
    use super::*;
    pub type A0 = t_72f67176ebb4;
    pub type A1 = Address<t_72f67176ebb4>;
}

pub mod alias_l3477 {
    use super::*;
    pub type A0 = t_bb03570ab96c;
    pub type A1 = Address<t_bb03570ab96c>;
}

pub mod alias_l3484 {
    use super::*;
    pub type A0 = t_907ea54a4919;
    pub type A1 = Address<t_907ea54a4919>;
}

pub mod alias_l3495 {
    use super::*;
    pub type A0 = t_f74128f01a97;
    pub type A1 = Address<t_f74128f01a97>;
}

pub mod alias_l3502 {
    use super::*;
    pub type A0 = t_8c007ef96404;
    pub type A1 = Address<t_8c007ef96404>;
}

pub mod alias_l3509 {
    use super::*;
    pub type A0 = t_18a17a434ce1;
    pub type A1 = Address<t_18a17a434ce1>;
}

pub mod alias_l3516 {
    use super::*;
    pub type A0 = t_381cd4f10589;
    pub type A1 = Address<t_381cd4f10589>;
}

pub mod alias_l3523 {
    use super::*;
    pub type A0 = t_319d2f5cd9e6;
    pub type A1 = Address<t_319d2f5cd9e6>;
}

pub mod alias_l3530 {
    use super::*;
    pub type A0 = t_4adc45268c21;
    pub type A1 = Address<t_4adc45268c21>;
}

pub mod alias_l3537 {
    use super::*;
    pub type A0 = t_a62146bebd95;
    pub type A1 = Address<t_a62146bebd95>;
}

pub mod alias_l3544 {
    use super::*;
    pub type A0 = t_8567321bec0d;
    pub type A1 = Address<t_8567321bec0d>;
}

pub mod alias_l3551 {
    use super::*;
    pub type A0 = t_5c51d4f905ca;
    pub type A1 = Address<t_5c51d4f905ca>;
}

pub mod alias_l3558 {
    use super::*;
    pub type A0 = t_d05c9f0490ff;
    pub type A1 = Address<t_d05c9f0490ff>;
}

pub mod alias_l3566 {
    use super::*;
    pub type A0 = t_bee8c05912fc;
    pub type A1 = Address<t_bee8c05912fc>;
}

/// Incomplete decompiler type: IMAGE_LOAD_CONFIG_DIRECTORY64.
pub enum t_286b2bf6be2e {}

pub mod alias_l3573 {
    use super::*;
    pub type A0 = t_286b2bf6be2e;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3638 {
    use super::*;
    pub type A0 = t_959541cecaf0;
    pub type A1 = Address<t_959541cecaf0>;
}

pub mod alias_l3651 {
    use super::*;
    pub type A0 = t_18a8ecb04ebb;
    pub type A1 = Address<t_18a8ecb04ebb>;
}

pub mod alias_l3659 {
    use super::*;
    pub type A0 = t_a320da0f5889;
    pub type A1 = Address<t_a320da0f5889>;
}

pub mod alias_l3667 {
    use super::*;
    pub type A0 = t_0a301615ebb4;
    pub type A1 = Address<t_0a301615ebb4>;
}

pub mod alias_l3674 {
    use super::*;
    pub type A0 = t_2a1b0958f3ac;
    pub type A1 = Address<t_2a1b0958f3ac>;
}

pub mod alias_l3681 {
    use super::*;
    pub type A0 = t_35677a4ddd7e;
    pub type A1 = Address<t_35677a4ddd7e>;
}

/// Incomplete decompiler type: IMAGE_RESOURCE_DIRECTORY_ENTRY_NameUnion.
pub enum t_38ebef70f22f {}

pub mod alias_l3689 {
    use super::*;
    pub type A0 = t_38ebef70f22f;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3697 {
    use super::*;
    pub type A0 = t_2311a28c4d26;
    pub type A1 = Address<t_2311a28c4d26>;
}

pub mod alias_l3704 {
    use super::*;
    pub type A0 = t_36cdbcd540ec;
    pub type A1 = Address<t_36cdbcd540ec>;
}

pub mod alias_l3711 {
    use super::*;
    pub type A0 = t_bf3171d9b8d8;
    pub type A1 = Address<t_bf3171d9b8d8>;
}

/// Incomplete decompiler type: IMAGE_RESOURCE_DIRECTORY_ENTRY.
pub enum t_4245c30be2a4 {}

pub mod alias_l3719 {
    use super::*;
    pub type A0 = t_4245c30be2a4;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3726 {
    use super::*;
    pub type A0 = t_14555e1345ce;
    pub type A1 = Address<t_14555e1345ce>;
}

pub mod alias_l3746 {
    use super::*;
    pub type A0 = t_f0a826c4e870;
    pub type A1 = Address<t_f0a826c4e870>;
}

pub mod alias_l3753 {
    use super::*;
    pub type A0 = t_a1cb950cec4a;
    pub type A1 = Address<t_a1cb950cec4a>;
}

pub mod alias_l3780 {
    use super::*;
    pub type A0 = Enum32;
}

pub mod alias_l3782 {
    use super::*;
    pub type A0 = u32;
}

pub mod alias_l3784 {
    use super::*;
    pub type A0 = i32;
}

pub mod alias_l3786 {
    use super::*;
    pub type A0 = t_bf9c49bcca3a;
    pub type A1 = Address<Unknown>;
}

pub mod alias_l3788 {
    use super::*;
    pub type A0 = t_bf9c49bcca3a;
}

/// Source symbol FUN_1400df700. Contents remain caller supplied.
pub type value_G_L3795 = u8;
pub const G_L3795: Global<value_G_L3795> = Global::new("FUN_1400df700", Some(0x1400df700));

/// Source symbol DAT_1401d0510. Contents remain caller supplied.
pub type value_G_L3796 = Address<Unknown>;
pub const G_L3796: Global<value_G_L3796> = Global::new("DAT_1401d0510", Some(0x1401d0510));

/// Source symbol FUN_1400df750. Contents remain caller supplied.
pub type value_G_L3797 = u8;
pub const G_L3797: Global<value_G_L3797> = Global::new("FUN_1400df750", Some(0x1400df750));

/// Source symbol FUN_1400df7c0. Contents remain caller supplied.
pub type value_G_L3798 = u8;
pub const G_L3798: Global<value_G_L3798> = Global::new("FUN_1400df7c0", Some(0x1400df7c0));

/// Source symbol FUN_1400df880. Contents remain caller supplied.
pub type value_G_L3799 = u8;
pub const G_L3799: Global<value_G_L3799> = Global::new("FUN_1400df880", Some(0x1400df880));

/// Source symbol DAT_1401d0468. Contents remain caller supplied.
pub type value_G_L3800 = u32;
pub const G_L3800: Global<value_G_L3800> = Global::new("DAT_1401d0468", Some(0x1401d0468));

/// Source symbol DAT_1401d0470. Contents remain caller supplied.
pub type value_G_L3801 = u8;
pub const G_L3801: Global<value_G_L3801> = Global::new("DAT_1401d0470", Some(0x1401d0470));

/// Source symbol DAT_1401d0480. Contents remain caller supplied.
pub type value_G_L3802 = u8;
pub const G_L3802: Global<value_G_L3802> = Global::new("DAT_1401d0480", Some(0x1401d0480));

/// Source symbol DAT_1401d0484. Contents remain caller supplied.
pub type value_G_L3803 = u8;
pub const G_L3803: Global<value_G_L3803> = Global::new("DAT_1401d0484", Some(0x1401d0484));

/// Source symbol DAT_1401d0c30. Contents remain caller supplied.
pub type value_G_L3804 = Address<u8>;
pub const G_L3804: Global<value_G_L3804> = Global::new("DAT_1401d0c30", Some(0x1401d0c30));

/// Source symbol DAT_1401d0448. Contents remain caller supplied.
pub type value_G_L3805 = u32;
pub const G_L3805: Global<value_G_L3805> = Global::new("DAT_1401d0448", Some(0x1401d0448));

/// Source symbol DAT_1401d0450. Contents remain caller supplied.
pub type value_G_L3806 = u8;
pub const G_L3806: Global<value_G_L3806> = Global::new("DAT_1401d0450", Some(0x1401d0450));

/// Source symbol DAT_1401d0460. Contents remain caller supplied.
pub type value_G_L3807 = u8;
pub const G_L3807: Global<value_G_L3807> = Global::new("DAT_1401d0460", Some(0x1401d0460));

/// Source symbol DAT_1401d0464. Contents remain caller supplied.
pub type value_G_L3808 = u8;
pub const G_L3808: Global<value_G_L3808> = Global::new("DAT_1401d0464", Some(0x1401d0464));

/// Source symbol ThreadLocalStoragePointer. Contents remain caller supplied.
pub type value_G_L3809 = Address<Unknown>;
pub const G_L3809: Global<value_G_L3809> = Global::new("ThreadLocalStoragePointer", None);

/// Source symbol DAT_1401d0c40. Contents remain caller supplied.
pub type value_G_L3810 = Address<u8>;
pub const G_L3810: Global<value_G_L3810> = Global::new("DAT_1401d0c40", Some(0x1401d0c40));

/// Source symbol DAT_1401d0428. Contents remain caller supplied.
pub type value_G_L3811 = u32;
pub const G_L3811: Global<value_G_L3811> = Global::new("DAT_1401d0428", Some(0x1401d0428));

/// Source symbol DAT_1401d0430. Contents remain caller supplied.
pub type value_G_L3812 = u8;
pub const G_L3812: Global<value_G_L3812> = Global::new("DAT_1401d0430", Some(0x1401d0430));

/// Source symbol DAT_1401d0440. Contents remain caller supplied.
pub type value_G_L3813 = u8;
pub const G_L3813: Global<value_G_L3813> = Global::new("DAT_1401d0440", Some(0x1401d0440));

/// Source symbol DAT_1401d0444. Contents remain caller supplied.
pub type value_G_L3814 = u8;
pub const G_L3814: Global<value_G_L3814> = Global::new("DAT_1401d0444", Some(0x1401d0444));

/// Source symbol DAT_1401d0c50. Contents remain caller supplied.
pub type value_G_L3815 = u8;
pub const G_L3815: Global<value_G_L3815> = Global::new("DAT_1401d0c50", Some(0x1401d0c50));

/// Source symbol FUN_1400df890. Contents remain caller supplied.
pub type value_G_L3816 = u8;
pub const G_L3816: Global<value_G_L3816> = Global::new("FUN_1400df890", Some(0x1400df890));

/// Source symbol DAT_1401d0c60. Contents remain caller supplied.
pub type value_G_L3817 = u8;
pub const G_L3817: Global<value_G_L3817> = Global::new("DAT_1401d0c60", Some(0x1401d0c60));

/// Source symbol DAT_1401d0c38. Contents remain caller supplied.
pub type value_G_L3818 = u64;
pub const G_L3818: Global<value_G_L3818> = Global::new("DAT_1401d0c38", Some(0x1401d0c38));

/// Source symbol DAT_1401d0c48. Contents remain caller supplied.
pub type value_G_L3819 = u8;
pub const G_L3819: Global<value_G_L3819> = Global::new("DAT_1401d0c48", Some(0x1401d0c48));

/// Source symbol DAT_1401d0c58. Contents remain caller supplied.
pub type value_G_L3820 = u8;
pub const G_L3820: Global<value_G_L3820> = Global::new("DAT_1401d0c58", Some(0x1401d0c58));

/// Source symbol DAT_1401d0c68. Contents remain caller supplied.
pub type value_G_L3821 = u8;
pub const G_L3821: Global<value_G_L3821> = Global::new("DAT_1401d0c68", Some(0x1401d0c68));

/// Source symbol UNK_1401d0434. Contents remain caller supplied.
pub type value_G_L3822 = u32;
pub const G_L3822: Global<value_G_L3822> = Global::new("UNK_1401d0434", None);

/// Source symbol UNK_1401d0438. Contents remain caller supplied.
pub type value_G_L3823 = u32;
pub const G_L3823: Global<value_G_L3823> = Global::new("UNK_1401d0438", None);

/// Source symbol UNK_1401d043c. Contents remain caller supplied.
pub type value_G_L3824 = u32;
pub const G_L3824: Global<value_G_L3824> = Global::new("UNK_1401d043c", None);

/// Source symbol UNK_1401d0454. Contents remain caller supplied.
pub type value_G_L3825 = u32;
pub const G_L3825: Global<value_G_L3825> = Global::new("UNK_1401d0454", None);

/// Source symbol UNK_1401d0458. Contents remain caller supplied.
pub type value_G_L3826 = u32;
pub const G_L3826: Global<value_G_L3826> = Global::new("UNK_1401d0458", None);

/// Source symbol UNK_1401d045c. Contents remain caller supplied.
pub type value_G_L3827 = u32;
pub const G_L3827: Global<value_G_L3827> = Global::new("UNK_1401d045c", None);

/// Source symbol UNK_1401d0474. Contents remain caller supplied.
pub type value_G_L3828 = u32;
pub const G_L3828: Global<value_G_L3828> = Global::new("UNK_1401d0474", None);

/// Source symbol UNK_1401d0478. Contents remain caller supplied.
pub type value_G_L3829 = u32;
pub const G_L3829: Global<value_G_L3829> = Global::new("UNK_1401d0478", None);

/// Source symbol UNK_1401d047c. Contents remain caller supplied.
pub type value_G_L3830 = u32;
pub const G_L3830: Global<value_G_L3830> = Global::new("UNK_1401d047c", None);

/// Source symbol DAT_1401cfb90. Contents remain caller supplied.
pub type value_G_L3831 = Address<u8>;
pub const G_L3831: Global<value_G_L3831> = Global::new("DAT_1401cfb90", Some(0x1401cfb90));

/// Source symbol DAT_1401cfbf0. Contents remain caller supplied.
pub type value_G_L3832 = Address<u8>;
pub const G_L3832: Global<value_G_L3832> = Global::new("DAT_1401cfbf0", Some(0x1401cfbf0));

/// Source symbol DAT_1401cfc28. Contents remain caller supplied.
pub type value_G_L3833 = Address<u8>;
pub const G_L3833: Global<value_G_L3833> = Global::new("DAT_1401cfc28", Some(0x1401cfc28));

/// Source symbol DebugBreak. Contents remain caller supplied.
pub type value_G_L3834 = u8;
pub const G_L3834: Global<value_G_L3834> = Global::new("DebugBreak", None);

/// Source symbol FUN_14008f670. Contents remain caller supplied.
pub type value_G_L3835 = u8;
pub const G_L3835: Global<value_G_L3835> = Global::new("FUN_14008f670", Some(0x14008f670));

/// Source symbol FUN_14008f6d0. Contents remain caller supplied.
pub type value_G_L3836 = u8;
pub const G_L3836: Global<value_G_L3836> = Global::new("FUN_14008f6d0", Some(0x14008f6d0));

/// Source symbol DAT_1401cfc08. Contents remain caller supplied.
pub type value_G_L3837 = u8;
pub const G_L3837: Global<value_G_L3837> = Global::new("DAT_1401cfc08", Some(0x1401cfc08));

/// Source symbol DAT_1401cfc10. Contents remain caller supplied.
pub type value_G_L3838 = Address<u8>;
pub const G_L3838: Global<value_G_L3838> = Global::new("DAT_1401cfc10", Some(0x1401cfc10));

/// Source symbol DAT_1401cff20. Contents remain caller supplied.
pub type value_G_L3839 = Address<u8>;
pub const G_L3839: Global<value_G_L3839> = Global::new("DAT_1401cff20", Some(0x1401cff20));

/// Source symbol FUN_140090290. Contents remain caller supplied.
pub type value_G_L3840 = u8;
pub const G_L3840: Global<value_G_L3840> = Global::new("FUN_140090290", Some(0x140090290));

/// Source symbol FUN_1400902a0. Contents remain caller supplied.
pub type value_G_L3841 = u8;
pub const G_L3841: Global<value_G_L3841> = Global::new("FUN_1400902a0", Some(0x1400902a0));

/// Source symbol FUN_1400903e0. Contents remain caller supplied.
pub type value_G_L3842 = u8;
pub const G_L3842: Global<value_G_L3842> = Global::new("FUN_1400903e0", Some(0x1400903e0));

/// Source symbol FUN_1400df8c0. Contents remain caller supplied.
pub type value_G_L3843 = u8;
pub const G_L3843: Global<value_G_L3843> = Global::new("FUN_1400df8c0", Some(0x1400df8c0));

/// Source symbol FUN_1400df8e0. Contents remain caller supplied.
pub type value_G_L3844 = u8;
pub const G_L3844: Global<value_G_L3844> = Global::new("FUN_1400df8e0", Some(0x1400df8e0));

/// Source symbol DAT_1401cfba0. Contents remain caller supplied.
pub type value_G_L3845 = Address<u8>;
pub const G_L3845: Global<value_G_L3845> = Global::new("DAT_1401cfba0", Some(0x1401cfba0));

/// Source symbol DAT_1401d0360. Contents remain caller supplied.
pub type value_G_L3846 = Address<Address<u8>>;
pub const G_L3846: Global<value_G_L3846> = Global::new("DAT_1401d0360", Some(0x1401d0360));

/// Source symbol FUN_140091240. Contents remain caller supplied.
pub type value_G_L3847 = u8;
pub const G_L3847: Global<value_G_L3847> = Global::new("FUN_140091240", Some(0x140091240));

/// Source symbol DAT_1401d0378. Contents remain caller supplied.
pub type value_G_L3848 = Address<u64>;
pub const G_L3848: Global<value_G_L3848> = Global::new("DAT_1401d0378", Some(0x1401d0378));

/// Source symbol DAT_140179170. Contents remain caller supplied.
pub type value_G_L3849 = u64;
pub const G_L3849: Global<value_G_L3849> = Global::new("DAT_140179170", Some(0x140179170));

/// Source symbol PTR_s_WilError_03_1401791c8. Contents remain caller supplied.
pub type value_G_L3850 = Address<Unknown>;
pub const G_L3850: Global<value_G_L3850> = Global::new("PTR_s_WilError_03_1401791c8", None);

/// Source symbol DAT_1401d0c80. Contents remain caller supplied.
pub type value_G_L3851 = u64;
pub const G_L3851: Global<value_G_L3851> = Global::new("DAT_1401d0c80", Some(0x1401d0c80));

/// Source symbol DAT_1401d0ca8. Contents remain caller supplied.
pub type value_G_L3852 = u64;
pub const G_L3852: Global<value_G_L3852> = Global::new("DAT_1401d0ca8", Some(0x1401d0ca8));

/// Source symbol FUN_1400df970. Contents remain caller supplied.
pub type value_G_L3853 = u8;
pub const G_L3853: Global<value_G_L3853> = Global::new("FUN_1400df970", Some(0x1400df970));

/// Source symbol DAT_1401d0cb0. Contents remain caller supplied.
pub type value_G_L3854 = u64;
pub const G_L3854: Global<value_G_L3854> = Global::new("DAT_1401d0cb0", Some(0x1401d0cb0));

/// Source symbol DAT_1401d0cc0. Contents remain caller supplied.
pub type value_G_L3855 = u64;
pub const G_L3855: Global<value_G_L3855> = Global::new("DAT_1401d0cc0", Some(0x1401d0cc0));

/// Source symbol DAT_1401d0cd0. Contents remain caller supplied.
pub type value_G_L3856 = u64;
pub const G_L3856: Global<value_G_L3856> = Global::new("DAT_1401d0cd0", Some(0x1401d0cd0));

/// Source symbol DAT_1401d0c70. Contents remain caller supplied.
pub type value_G_L3857 = u8;
pub const G_L3857: Global<value_G_L3857> = Global::new("DAT_1401d0c70", Some(0x1401d0c70));

/// Source symbol DAT_1401d0c78. Contents remain caller supplied.
pub type value_G_L3858 = i64;
pub const G_L3858: Global<value_G_L3858> = Global::new("DAT_1401d0c78", Some(0x1401d0c78));

/// Source symbol DAT_1401d0cb8. Contents remain caller supplied.
pub type value_G_L3859 = u64;
pub const G_L3859: Global<value_G_L3859> = Global::new("DAT_1401d0cb8", Some(0x1401d0cb8));

/// Source symbol DAT_1401d0cc8. Contents remain caller supplied.
pub type value_G_L3860 = u64;
pub const G_L3860: Global<value_G_L3860> = Global::new("DAT_1401d0cc8", Some(0x1401d0cc8));

/// Source symbol DAT_1401cfa60. Contents remain caller supplied.
pub type value_G_L3861 = u8;
pub const G_L3861: Global<value_G_L3861> = Global::new("DAT_1401cfa60", Some(0x1401cfa60));

/// Source symbol FUN_1400dfa40. Contents remain caller supplied.
pub type value_G_L3862 = u8;
pub const G_L3862: Global<value_G_L3862> = Global::new("FUN_1400dfa40", Some(0x1400dfa40));

/// Source symbol FUN_1400dfa50. Contents remain caller supplied.
pub type value_G_L3863 = u8;
pub const G_L3863: Global<value_G_L3863> = Global::new("FUN_1400dfa50", Some(0x1400dfa50));

/// Source symbol DAT_1401cfad1. Contents remain caller supplied.
pub type value_G_L3864 = u8;
pub const G_L3864: Global<value_G_L3864> = Global::new("DAT_1401cfad1", Some(0x1401cfad1));

/// Source symbol DAT_140179040. Contents remain caller supplied.
pub type value_G_L3865 = u64;
pub const G_L3865: Global<value_G_L3865> = Global::new("DAT_140179040", Some(0x140179040));

/// Source symbol DAT_1401051c8. Contents remain caller supplied.
pub type value_G_L3866 = u8;
pub const G_L3866: Global<value_G_L3866> = Global::new("DAT_1401051c8", Some(0x1401051c8));

/// Source symbol DAT_1401051d8. Contents remain caller supplied.
pub type value_G_L3867 = u8;
pub const G_L3867: Global<value_G_L3867> = Global::new("DAT_1401051d8", Some(0x1401051d8));

/// Source symbol DAT_1401051b8. Contents remain caller supplied.
pub type value_G_L3868 = u8;
pub const G_L3868: Global<value_G_L3868> = Global::new("DAT_1401051b8", Some(0x1401051b8));

/// Source symbol DAT_140105248. Contents remain caller supplied.
pub type value_G_L3869 = u64;
pub const G_L3869: Global<value_G_L3869> = Global::new("DAT_140105248", Some(0x140105248));

/// Source symbol DAT_14016a9f0. Contents remain caller supplied.
pub type value_G_L3871 = u8;
pub const G_L3871: Global<value_G_L3871> = Global::new("DAT_14016a9f0", Some(0x14016a9f0));

/// Source symbol DAT_14016aa78. Contents remain caller supplied.
pub type value_G_L3872 = u8;
pub const G_L3872: Global<value_G_L3872> = Global::new("DAT_14016aa78", Some(0x14016aa78));

/// Source symbol DAT_1401cfad8. Contents remain caller supplied.
pub type value_G_L3873 = i64;
pub const G_L3873: Global<value_G_L3873> = Global::new("DAT_1401cfad8", Some(0x1401cfad8));

/// Source symbol DAT_140105330. Contents remain caller supplied.
pub type value_G_L3874 = u8;
pub const G_L3874: Global<value_G_L3874> = Global::new("DAT_140105330", Some(0x140105330));

/// Source symbol DAT_140105290. Contents remain caller supplied.
pub type value_G_L3875 = u8;
pub const G_L3875: Global<value_G_L3875> = Global::new("DAT_140105290", Some(0x140105290));

/// Source symbol DAT_14016a4e0. Contents remain caller supplied.
pub type value_G_L3876 = u8;
pub const G_L3876: Global<value_G_L3876> = Global::new("DAT_14016a4e0", Some(0x14016a4e0));

/// Source symbol DAT_14016a540. Contents remain caller supplied.
pub type value_G_L3877 = u8;
pub const G_L3877: Global<value_G_L3877> = Global::new("DAT_14016a540", Some(0x14016a540));

/// Source symbol DAT_14016a5a0. Contents remain caller supplied.
pub type value_G_L3878 = u8;
pub const G_L3878: Global<value_G_L3878> = Global::new("DAT_14016a5a0", Some(0x14016a5a0));

/// Source symbol DAT_14016a600. Contents remain caller supplied.
pub type value_G_L3879 = u8;
pub const G_L3879: Global<value_G_L3879> = Global::new("DAT_14016a600", Some(0x14016a600));

/// Source symbol DAT_14016a660. Contents remain caller supplied.
pub type value_G_L3880 = u8;
pub const G_L3880: Global<value_G_L3880> = Global::new("DAT_14016a660", Some(0x14016a660));

/// Source symbol DAT_14016a6c0. Contents remain caller supplied.
pub type value_G_L3881 = u8;
pub const G_L3881: Global<value_G_L3881> = Global::new("DAT_14016a6c0", Some(0x14016a6c0));

/// Source symbol DAT_14016a720. Contents remain caller supplied.
pub type value_G_L3882 = u8;
pub const G_L3882: Global<value_G_L3882> = Global::new("DAT_14016a720", Some(0x14016a720));

/// Source symbol DAT_14016a780. Contents remain caller supplied.
pub type value_G_L3883 = u8;
pub const G_L3883: Global<value_G_L3883> = Global::new("DAT_14016a780", Some(0x14016a780));

/// Source symbol DAT_14016a7e0. Contents remain caller supplied.
pub type value_G_L3884 = u8;
pub const G_L3884: Global<value_G_L3884> = Global::new("DAT_14016a7e0", Some(0x14016a7e0));

/// Source symbol DAT_14016a840. Contents remain caller supplied.
pub type value_G_L3885 = u8;
pub const G_L3885: Global<value_G_L3885> = Global::new("DAT_14016a840", Some(0x14016a840));

/// Source symbol DAT_14016a8a0. Contents remain caller supplied.
pub type value_G_L3886 = u8;
pub const G_L3886: Global<value_G_L3886> = Global::new("DAT_14016a8a0", Some(0x14016a8a0));

/// Source symbol DAT_14016a900. Contents remain caller supplied.
pub type value_G_L3887 = u8;
pub const G_L3887: Global<value_G_L3887> = Global::new("DAT_14016a900", Some(0x14016a900));

/// Source symbol DAT_14016a960. Contents remain caller supplied.
pub type value_G_L3888 = u8;
pub const G_L3888: Global<value_G_L3888> = Global::new("DAT_14016a960", Some(0x14016a960));

/// Source symbol DAT_14016aad0. Contents remain caller supplied.
pub type value_G_L3889 = u8;
pub const G_L3889: Global<value_G_L3889> = Global::new("DAT_14016aad0", Some(0x14016aad0));

/// Source symbol DAT_1401cfae8. Contents remain caller supplied.
pub type value_G_L3890 = i32;
pub const G_L3890: Global<value_G_L3890> = Global::new("DAT_1401cfae8", Some(0x1401cfae8));

/// Source symbol DAT_140105538. Contents remain caller supplied.
pub type value_G_L3892 = u8;
pub const G_L3892: Global<value_G_L3892> = Global::new("DAT_140105538", Some(0x140105538));

/// Source symbol DAT_1401d0c74. Contents remain caller supplied.
pub type value_G_L3893 = i32;
pub const G_L3893: Global<value_G_L3893> = Global::new("DAT_1401d0c74", Some(0x1401d0c74));

/// Source symbol DAT_1401d0c78. Contents remain caller supplied.
pub type value_G_L3894 = u64;
pub const G_L3894: Global<value_G_L3894> = Global::new("DAT_1401d0c78", Some(0x1401d0c78));

/// Source symbol DAT_00000008. Contents remain caller supplied.
pub type value_G_L3896 = u8;
pub const G_L3896: Global<value_G_L3896> = Global::new("DAT_00000008", None);

/// Source symbol DAT_00000038. Contents remain caller supplied.
pub type value_G_L3897 = u8;
pub const G_L3897: Global<value_G_L3897> = Global::new("DAT_00000038", None);

/// Source symbol DAT_00000030. Contents remain caller supplied.
pub type value_G_L3898 = u8;
pub const G_L3898: Global<value_G_L3898> = Global::new("DAT_00000030", None);

/// Source symbol DAT_140179148. Contents remain caller supplied.
pub type value_G_L3899 = u32;
pub const G_L3899: Global<value_G_L3899> = Global::new("DAT_140179148", Some(0x140179148));

/// Source symbol DAT_140105548. Contents remain caller supplied.
pub type value_G_L3900 = u8;
pub const G_L3900: Global<value_G_L3900> = Global::new("DAT_140105548", Some(0x140105548));

/// Source symbol DAT_1401057b0. Contents remain caller supplied.
pub type value_G_L3902 = u64;
pub const G_L3902: Global<value_G_L3902> = Global::new("DAT_1401057b0", Some(0x1401057b0));

/// Source symbol DAT_140179110. Contents remain caller supplied.
pub type value_G_L3903 = Address<u32>;
pub const G_L3903: Global<value_G_L3903> = Global::new("DAT_140179110", Some(0x140179110));

/// Source symbol DAT_140179118. Contents remain caller supplied.
pub type value_G_L3904 = u8;
pub const G_L3904: Global<value_G_L3904> = Global::new("DAT_140179118", Some(0x140179118));

/// Source symbol DAT_140179120. Contents remain caller supplied.
pub type value_G_L3905 = u8;
pub const G_L3905: Global<value_G_L3905> = Global::new("DAT_140179120", Some(0x140179120));

/// Source symbol DAT_00000010. Contents remain caller supplied.
pub type value_G_L3906 = u8;
pub const G_L3906: Global<value_G_L3906> = Global::new("DAT_00000010", None);

/// Source symbol DAT_00000018. Contents remain caller supplied.
pub type value_G_L3907 = u8;
pub const G_L3907: Global<value_G_L3907> = Global::new("DAT_00000018", None);

/// Source symbol DAT_140179110. Contents remain caller supplied.
pub type value_G_L3908 = Address<i64>;
pub const G_L3908: Global<value_G_L3908> = Global::new("DAT_140179110", Some(0x140179110));

/// Source symbol DAT_140105500. Contents remain caller supplied.
pub type value_G_L3909 = u8;
pub const G_L3909: Global<value_G_L3909> = Global::new("DAT_140105500", Some(0x140105500));

/// Source symbol PTR_1400e4d48. Contents remain caller supplied.
pub type value_G_L3910 = Address<Unknown>;
pub const G_L3910: Global<value_G_L3910> = Global::new("PTR_1400e4d48", Some(0x1400e4d48));

/// Source symbol DAT_140105420. Contents remain caller supplied.
pub type value_G_L3911 = u64;
pub const G_L3911: Global<value_G_L3911> = Global::new("DAT_140105420", Some(0x140105420));

/// Source symbol DAT_140105490. Contents remain caller supplied.
pub type value_G_L3912 = u64;
pub const G_L3912: Global<value_G_L3912> = Global::new("DAT_140105490", Some(0x140105490));

/// Source symbol DAT_1401054f0. Contents remain caller supplied.
pub type value_G_L3913 = u64;
pub const G_L3913: Global<value_G_L3913> = Global::new("DAT_1401054f0", Some(0x1401054f0));

/// Source symbol DAT_1401055a8. Contents remain caller supplied.
pub type value_G_L3915 = u8;
pub const G_L3915: Global<value_G_L3915> = Global::new("DAT_1401055a8", Some(0x1401055a8));

/// Source symbol DAT_1401cfae0. Contents remain caller supplied.
pub type value_G_L3916 = i64;
pub const G_L3916: Global<value_G_L3916> = Global::new("DAT_1401cfae0", Some(0x1401cfae0));

/// Source symbol DAT_1401cfaf8. Contents remain caller supplied.
pub type value_G_L3917 = u8;
pub const G_L3917: Global<value_G_L3917> = Global::new("DAT_1401cfaf8", Some(0x1401cfaf8));

/// Source symbol DAT_1401cfaf0. Contents remain caller supplied.
pub type value_G_L3918 = u64;
pub const G_L3918: Global<value_G_L3918> = Global::new("DAT_1401cfaf0", Some(0x1401cfaf0));

/// Source symbol DAT_1401055b8. Contents remain caller supplied.
pub type value_G_L3919 = u8;
pub const G_L3919: Global<value_G_L3919> = Global::new("DAT_1401055b8", Some(0x1401055b8));

/// Source symbol DAT_1401cfb00. Contents remain caller supplied.
pub type value_G_L3920 = u8;
pub const G_L3920: Global<value_G_L3920> = Global::new("DAT_1401cfb00", Some(0x1401cfb00));

/// Source symbol DAT_1401cfb40. Contents remain caller supplied.
pub type value_G_L3921 = u8;
pub const G_L3921: Global<value_G_L3921> = Global::new("DAT_1401cfb40", Some(0x1401cfb40));

/// Source symbol DAT_14016ab38. Contents remain caller supplied.
pub type value_G_L3923 = u8;
pub const G_L3923: Global<value_G_L3923> = Global::new("DAT_14016ab38", Some(0x14016ab38));

/// Source symbol DAT_1401cfb30. Contents remain caller supplied.
pub type value_G_L3924 = i64;
pub const G_L3924: Global<value_G_L3924> = Global::new("DAT_1401cfb30", Some(0x1401cfb30));

/// Source symbol DAT_140105708. Contents remain caller supplied.
pub type value_G_L3925 = u16;
pub const G_L3925: Global<value_G_L3925> = Global::new("DAT_140105708", Some(0x140105708));

/// Source symbol DAT_1401cfb18. Contents remain caller supplied.
pub type value_G_L3926 = u8;
pub const G_L3926: Global<value_G_L3926> = Global::new("DAT_1401cfb18", Some(0x1401cfb18));

/// Source symbol DAT_1401cfb10. Contents remain caller supplied.
pub type value_G_L3927 = u64;
pub const G_L3927: Global<value_G_L3927> = Global::new("DAT_1401cfb10", Some(0x1401cfb10));

/// Source symbol DAT_140105778. Contents remain caller supplied.
pub type value_G_L3928 = u8;
pub const G_L3928: Global<value_G_L3928> = Global::new("DAT_140105778", Some(0x140105778));

/// Source symbol DAT_1401cfb20. Contents remain caller supplied.
pub type value_G_L3929 = u8;
pub const G_L3929: Global<value_G_L3929> = Global::new("DAT_1401cfb20", Some(0x1401cfb20));

/// Source symbol FUN_140005ec0. Contents remain caller supplied.
pub type value_G_L3930 = u8;
pub const G_L3930: Global<value_G_L3930> = Global::new("FUN_140005ec0", Some(0x140005ec0));

/// Source symbol DAT_1401cfb10. Contents remain caller supplied.
pub type value_G_L3931 = i64;
pub const G_L3931: Global<value_G_L3931> = Global::new("DAT_1401cfb10", Some(0x1401cfb10));

/// Source symbol RTTI_Type_Descriptor. Contents remain caller supplied.
pub type value_G_L3934 = t_085d0cfce539;
pub const G_L3934: Global<value_G_L3934> = Global::new("RTTI_Type_Descriptor", None);

/// Source symbol DAT_00000040. Contents remain caller supplied.
pub type value_G_L3935 = u8;
pub const G_L3935: Global<value_G_L3935> = Global::new("DAT_00000040", None);

/// Source symbol DAT_00000050. Contents remain caller supplied.
pub type value_G_L3936 = u8;
pub const G_L3936: Global<value_G_L3936> = Global::new("DAT_00000050", None);

/// Source symbol DAT_140105e40. Contents remain caller supplied.
pub type value_G_L3937 = u64;
pub const G_L3937: Global<value_G_L3937> = Global::new("DAT_140105e40", Some(0x140105e40));

/// Source symbol DAT_1401d06e0. Contents remain caller supplied.
pub type value_G_L3938 = Address<t_2e3425b3a84b>;
pub const G_L3938: Global<value_G_L3938> = Global::new("DAT_1401d06e0", Some(0x1401d06e0));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L3939 = i64;
pub const G_L3939: Global<value_G_L3939> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401d06b8. Contents remain caller supplied.
pub type value_G_L3940 = u8;
pub const G_L3940: Global<value_G_L3940> = Global::new("DAT_1401d06b8", Some(0x1401d06b8));

/// Source symbol DAT_140179140. Contents remain caller supplied.
pub type value_G_L3941 = u32;
pub const G_L3941: Global<value_G_L3941> = Global::new("DAT_140179140", Some(0x140179140));

/// Source symbol DAT_140179158. Contents remain caller supplied.
pub type value_G_L3943 = i32;
pub const G_L3943: Global<value_G_L3943> = Global::new("DAT_140179158", Some(0x140179158));

/// Source symbol DAT_140179144. Contents remain caller supplied.
pub type value_G_L3944 = u32;
pub const G_L3944: Global<value_G_L3944> = Global::new("DAT_140179144", Some(0x140179144));

/// Source symbol DAT_1401d0090. Contents remain caller supplied.
pub type value_G_L3945 = u16;
pub const G_L3945: Global<value_G_L3945> = Global::new("DAT_1401d0090", Some(0x1401d0090));

/// Source symbol DAT_1401d0094. Contents remain caller supplied.
pub type value_G_L3946 = u16;
pub const G_L3946: Global<value_G_L3946> = Global::new("DAT_1401d0094", Some(0x1401d0094));

/// Source symbol DAT_1401d0090. Contents remain caller supplied.
pub type value_G_L3947 = i16;
pub const G_L3947: Global<value_G_L3947> = Global::new("DAT_1401d0090", Some(0x1401d0090));

/// Source symbol DAT_1401d0094. Contents remain caller supplied.
pub type value_G_L3948 = i16;
pub const G_L3948: Global<value_G_L3948> = Global::new("DAT_1401d0094", Some(0x1401d0094));

/// Source symbol FUN_14000a160. Contents remain caller supplied.
pub type value_G_L3949 = u8;
pub const G_L3949: Global<value_G_L3949> = Global::new("FUN_14000a160", Some(0x14000a160));

/// Source symbol DAT_140106050. Contents remain caller supplied.
pub type value_G_L3950 = u8;
pub const G_L3950: Global<value_G_L3950> = Global::new("DAT_140106050", Some(0x140106050));

/// Source symbol DAT_00000060. Contents remain caller supplied.
pub type value_G_L3951 = u8;
pub const G_L3951: Global<value_G_L3951> = Global::new("DAT_00000060", None);

/// Source symbol DAT_00000048. Contents remain caller supplied.
pub type value_G_L3952 = u8;
pub const G_L3952: Global<value_G_L3952> = Global::new("DAT_00000048", None);

/// Source symbol DAT_140105fb0. Contents remain caller supplied.
pub type value_G_L3953 = u8;
pub const G_L3953: Global<value_G_L3953> = Global::new("DAT_140105fb0", Some(0x140105fb0));

/// Source symbol FUN_1400b2190. Contents remain caller supplied.
pub type value_G_L3954 = u8;
pub const G_L3954: Global<value_G_L3954> = Global::new("FUN_1400b2190", Some(0x1400b2190));

/// Source symbol FUN_1400b2180. Contents remain caller supplied.
pub type value_G_L3955 = u8;
pub const G_L3955: Global<value_G_L3955> = Global::new("FUN_1400b2180", Some(0x1400b2180));

/// Source symbol DAT_140105f90. Contents remain caller supplied.
pub type value_G_L3956 = u8;
pub const G_L3956: Global<value_G_L3956> = Global::new("DAT_140105f90", Some(0x140105f90));

/// Source symbol DAT_140106070. Contents remain caller supplied.
pub type value_G_L3957 = u64;
pub const G_L3957: Global<value_G_L3957> = Global::new("DAT_140106070", Some(0x140106070));

/// Source symbol FUN_14000a520. Contents remain caller supplied.
pub type value_G_L3958 = u8;
pub const G_L3958: Global<value_G_L3958> = Global::new("FUN_14000a520", Some(0x14000a520));

/// Source symbol FUN_14000a610. Contents remain caller supplied.
pub type value_G_L3959 = u8;
pub const G_L3959: Global<value_G_L3959> = Global::new("FUN_14000a610", Some(0x14000a610));

/// Source symbol DAT_140105f80. Contents remain caller supplied.
pub type value_G_L3960 = u8;
pub const G_L3960: Global<value_G_L3960> = Global::new("DAT_140105f80", Some(0x140105f80));

/// Source symbol FUN_14000a5c0. Contents remain caller supplied.
pub type value_G_L3961 = u8;
pub const G_L3961: Global<value_G_L3961> = Global::new("FUN_14000a5c0", Some(0x14000a5c0));

/// Source symbol FUN_14000b330. Contents remain caller supplied.
pub type value_G_L3962 = u8;
pub const G_L3962: Global<value_G_L3962> = Global::new("FUN_14000b330", Some(0x14000b330));

/// Source symbol FUN_14000b3b0. Contents remain caller supplied.
pub type value_G_L3963 = u8;
pub const G_L3963: Global<value_G_L3963> = Global::new("FUN_14000b3b0", Some(0x14000b3b0));

/// Source symbol DAT_140106dc0. Contents remain caller supplied.
pub type value_G_L3964 = u64;
pub const G_L3964: Global<value_G_L3964> = Global::new("DAT_140106dc0", Some(0x140106dc0));

/// Source symbol DAT_00000028. Contents remain caller supplied.
pub type value_G_L3965 = u8;
pub const G_L3965: Global<value_G_L3965> = Global::new("DAT_00000028", None);

/// Source symbol DAT_140106da0. Contents remain caller supplied.
pub type value_G_L3966 = u8;
pub const G_L3966: Global<value_G_L3966> = Global::new("DAT_140106da0", Some(0x140106da0));

/// Source symbol DAT_140106d28. Contents remain caller supplied.
pub type value_G_L3967 = u8;
pub const G_L3967: Global<value_G_L3967> = Global::new("DAT_140106d28", Some(0x140106d28));

/// Source symbol DAT_1401cfb58. Contents remain caller supplied.
pub type value_G_L3968 = u8;
pub const G_L3968: Global<value_G_L3968> = Global::new("DAT_1401cfb58", Some(0x1401cfb58));

/// Source symbol DAT_1401cfb50. Contents remain caller supplied.
pub type value_G_L3969 = u64;
pub const G_L3969: Global<value_G_L3969> = Global::new("DAT_1401cfb50", Some(0x1401cfb50));

/// Source symbol DAT_140106e60. Contents remain caller supplied.
pub type value_G_L3970 = u8;
pub const G_L3970: Global<value_G_L3970> = Global::new("DAT_140106e60", Some(0x140106e60));

/// Source symbol DAT_1401cfb60. Contents remain caller supplied.
pub type value_G_L3971 = u8;
pub const G_L3971: Global<value_G_L3971> = Global::new("DAT_1401cfb60", Some(0x1401cfb60));

/// Source symbol DAT_1401cfb50. Contents remain caller supplied.
pub type value_G_L3972 = i64;
pub const G_L3972: Global<value_G_L3972> = Global::new("DAT_1401cfb50", Some(0x1401cfb50));

/// Source symbol FUN_14000beb0. Contents remain caller supplied.
pub type value_G_L3973 = u8;
pub const G_L3973: Global<value_G_L3973> = Global::new("FUN_14000beb0", Some(0x14000beb0));

/// Source symbol DAT_1401d06e0. Contents remain caller supplied.
pub type value_G_L3974 = Address<i64>;
pub const G_L3974: Global<value_G_L3974> = Global::new("DAT_1401d06e0", Some(0x1401d06e0));

/// Source symbol DAT_1401d0858. Contents remain caller supplied.
pub type value_G_L3976 = u8;
pub const G_L3976: Global<value_G_L3976> = Global::new("DAT_1401d0858", Some(0x1401d0858));

/// Source symbol DAT_140107040. Contents remain caller supplied.
pub type value_G_L3977 = u8;
pub const G_L3977: Global<value_G_L3977> = Global::new("DAT_140107040", Some(0x140107040));

/// Source symbol DAT_140106fe8. Contents remain caller supplied.
pub type value_G_L3978 = u8;
pub const G_L3978: Global<value_G_L3978> = Global::new("DAT_140106fe8", Some(0x140106fe8));

/// Source symbol DAT_1401072c8. Contents remain caller supplied.
pub type value_G_L3979 = u8;
pub const G_L3979: Global<value_G_L3979> = Global::new("DAT_1401072c8", Some(0x1401072c8));

/// Source symbol DAT_1401063e8. Contents remain caller supplied.
pub type value_G_L3980 = u8;
pub const G_L3980: Global<value_G_L3980> = Global::new("DAT_1401063e8", Some(0x1401063e8));

/// Source symbol DAT_1401073d8. Contents remain caller supplied.
pub type value_G_L3981 = u64;
pub const G_L3981: Global<value_G_L3981> = Global::new("DAT_1401073d8", Some(0x1401073d8));

/// Source symbol DAT_140107430. Contents remain caller supplied.
pub type value_G_L3982 = u64;
pub const G_L3982: Global<value_G_L3982> = Global::new("DAT_140107430", Some(0x140107430));

/// Source symbol DAT_140107ad8. Contents remain caller supplied.
pub type value_G_L3983 = u8;
pub const G_L3983: Global<value_G_L3983> = Global::new("DAT_140107ad8", Some(0x140107ad8));

/// Source symbol DAT_140107ab8. Contents remain caller supplied.
pub type value_G_L3984 = u8;
pub const G_L3984: Global<value_G_L3984> = Global::new("DAT_140107ab8", Some(0x140107ab8));

/// Source symbol DAT_140107ac8. Contents remain caller supplied.
pub type value_G_L3985 = u8;
pub const G_L3985: Global<value_G_L3985> = Global::new("DAT_140107ac8", Some(0x140107ac8));

/// Source symbol DAT_1400e5008. Contents remain caller supplied.
pub type value_G_L3986 = u8;
pub const G_L3986: Global<value_G_L3986> = Global::new("DAT_1400e5008", Some(0x1400e5008));

/// Source symbol DAT_140107a88. Contents remain caller supplied.
pub type value_G_L3987 = u8;
pub const G_L3987: Global<value_G_L3987> = Global::new("DAT_140107a88", Some(0x140107a88));

/// Source symbol DAT_140107a98. Contents remain caller supplied.
pub type value_G_L3988 = u8;
pub const G_L3988: Global<value_G_L3988> = Global::new("DAT_140107a98", Some(0x140107a98));

/// Source symbol DAT_1401cfc30. Contents remain caller supplied.
pub type value_G_L3989 = i64;
pub const G_L3989: Global<value_G_L3989> = Global::new("DAT_1401cfc30", Some(0x1401cfc30));

/// Source symbol FUN_140075f80. Contents remain caller supplied.
pub type value_G_L3990 = u8;
pub const G_L3990: Global<value_G_L3990> = Global::new("FUN_140075f80", Some(0x140075f80));

/// Source symbol DAT_140107aa8. Contents remain caller supplied.
pub type value_G_L3991 = u8;
pub const G_L3991: Global<value_G_L3991> = Global::new("DAT_140107aa8", Some(0x140107aa8));

/// Source symbol DAT_140107a48. Contents remain caller supplied.
pub type value_G_L3992 = u8;
pub const G_L3992: Global<value_G_L3992> = Global::new("DAT_140107a48", Some(0x140107a48));

/// Source symbol FUN_14006e350. Contents remain caller supplied.
pub type value_G_L3993 = u8;
pub const G_L3993: Global<value_G_L3993> = Global::new("FUN_14006e350", Some(0x14006e350));

/// Source symbol DAT_1401079f8. Contents remain caller supplied.
pub type value_G_L3994 = u8;
pub const G_L3994: Global<value_G_L3994> = Global::new("DAT_1401079f8", Some(0x1401079f8));

/// Source symbol DAT_140107ae8. Contents remain caller supplied.
pub type value_G_L3995 = u8;
pub const G_L3995: Global<value_G_L3995> = Global::new("DAT_140107ae8", Some(0x140107ae8));

/// Source symbol DAT_1401cfbf8. Contents remain caller supplied.
pub type value_G_L3996 = i32;
pub const G_L3996: Global<value_G_L3996> = Global::new("DAT_1401cfbf8", Some(0x1401cfbf8));

/// Source symbol DAT_1401cfb90. Contents remain caller supplied.
pub type value_G_L3997 = i64;
pub const G_L3997: Global<value_G_L3997> = Global::new("DAT_1401cfb90", Some(0x1401cfb90));

/// Source symbol DAT_1401cfb98. Contents remain caller supplied.
pub type value_G_L3998 = i64;
pub const G_L3998: Global<value_G_L3998> = Global::new("DAT_1401cfb98", Some(0x1401cfb98));

/// Source symbol DAT_1401cfba0. Contents remain caller supplied.
pub type value_G_L3999 = i64;
pub const G_L3999: Global<value_G_L3999> = Global::new("DAT_1401cfba0", Some(0x1401cfba0));

/// Source symbol DAT_1401cfba8. Contents remain caller supplied.
pub type value_G_L4000 = i64;
pub const G_L4000: Global<value_G_L4000> = Global::new("DAT_1401cfba8", Some(0x1401cfba8));

/// Source symbol DAT_1401cfbb0. Contents remain caller supplied.
pub type value_G_L4001 = i64;
pub const G_L4001: Global<value_G_L4001> = Global::new("DAT_1401cfbb0", Some(0x1401cfbb0));

/// Source symbol DAT_1401cfbb8. Contents remain caller supplied.
pub type value_G_L4002 = i8;
pub const G_L4002: Global<value_G_L4002> = Global::new("DAT_1401cfbb8", Some(0x1401cfbb8));

/// Source symbol DAT_1401cfbc0. Contents remain caller supplied.
pub type value_G_L4003 = i64;
pub const G_L4003: Global<value_G_L4003> = Global::new("DAT_1401cfbc0", Some(0x1401cfbc0));

/// Source symbol DAT_1401cfbe0. Contents remain caller supplied.
pub type value_G_L4004 = i64;
pub const G_L4004: Global<value_G_L4004> = Global::new("DAT_1401cfbe0", Some(0x1401cfbe0));

/// Source symbol DAT_1401cfbd8. Contents remain caller supplied.
pub type value_G_L4005 = i8;
pub const G_L4005: Global<value_G_L4005> = Global::new("DAT_1401cfbd8", Some(0x1401cfbd8));

/// Source symbol DAT_1401cfbe8. Contents remain caller supplied.
pub type value_G_L4006 = i8;
pub const G_L4006: Global<value_G_L4006> = Global::new("DAT_1401cfbe8", Some(0x1401cfbe8));

/// Source symbol DAT_1401cfbf0. Contents remain caller supplied.
pub type value_G_L4007 = i64;
pub const G_L4007: Global<value_G_L4007> = Global::new("DAT_1401cfbf0", Some(0x1401cfbf0));

/// Source symbol DAT_1401cfb70. Contents remain caller supplied.
pub type value_G_L4008 = u8;
pub const G_L4008: Global<value_G_L4008> = Global::new("DAT_1401cfb70", Some(0x1401cfb70));

/// Source symbol DAT_1401cfb74. Contents remain caller supplied.
pub type value_G_L4009 = i32;
pub const G_L4009: Global<value_G_L4009> = Global::new("DAT_1401cfb74", Some(0x1401cfb74));

/// Source symbol DAT_1401cfb78. Contents remain caller supplied.
pub type value_G_L4010 = u8;
pub const G_L4010: Global<value_G_L4010> = Global::new("DAT_1401cfb78", Some(0x1401cfb78));

/// Source symbol DAT_1401cfb7c. Contents remain caller supplied.
pub type value_G_L4011 = i32;
pub const G_L4011: Global<value_G_L4011> = Global::new("DAT_1401cfb7c", Some(0x1401cfb7c));

/// Source symbol DAT_1401cfb80. Contents remain caller supplied.
pub type value_G_L4012 = u8;
pub const G_L4012: Global<value_G_L4012> = Global::new("DAT_1401cfb80", Some(0x1401cfb80));

/// Source symbol DAT_1401cfb84. Contents remain caller supplied.
pub type value_G_L4013 = i32;
pub const G_L4013: Global<value_G_L4013> = Global::new("DAT_1401cfb84", Some(0x1401cfb84));

/// Source symbol DAT_1401cfb88. Contents remain caller supplied.
pub type value_G_L4014 = u8;
pub const G_L4014: Global<value_G_L4014> = Global::new("DAT_1401cfb88", Some(0x1401cfb88));

/// Source symbol DAT_1401cfbd0. Contents remain caller supplied.
pub type value_G_L4015 = u8;
pub const G_L4015: Global<value_G_L4015> = Global::new("DAT_1401cfbd0", Some(0x1401cfbd0));

/// Source symbol DAT_1401cfbc8. Contents remain caller supplied.
pub type value_G_L4016 = i64;
pub const G_L4016: Global<value_G_L4016> = Global::new("DAT_1401cfbc8", Some(0x1401cfbc8));

/// Source symbol PTR_1401073c8. Contents remain caller supplied.
pub type value_G_L4017 = Address<Unknown>;
pub const G_L4017: Global<value_G_L4017> = Global::new("PTR_1401073c8", Some(0x1401073c8));

/// Source symbol DAT_140107e48. Contents remain caller supplied.
pub type value_G_L4018 = u8;
pub const G_L4018: Global<value_G_L4018> = Global::new("DAT_140107e48", Some(0x140107e48));

/// Source symbol DAT_1401cfc10. Contents remain caller supplied.
pub type value_G_L4019 = i64;
pub const G_L4019: Global<value_G_L4019> = Global::new("DAT_1401cfc10", Some(0x1401cfc10));

/// Source symbol DAT_1401cfc28. Contents remain caller supplied.
pub type value_G_L4020 = i64;
pub const G_L4020: Global<value_G_L4020> = Global::new("DAT_1401cfc28", Some(0x1401cfc28));

/// Source symbol DAT_1401cfc18. Contents remain caller supplied.
pub type value_G_L4021 = i64;
pub const G_L4021: Global<value_G_L4021> = Global::new("DAT_1401cfc18", Some(0x1401cfc18));

/// Source symbol DAT_1401cfc20. Contents remain caller supplied.
pub type value_G_L4022 = i64;
pub const G_L4022: Global<value_G_L4022> = Global::new("DAT_1401cfc20", Some(0x1401cfc20));

/// Source symbol DAT_1401d0408. Contents remain caller supplied.
pub type value_G_L4023 = u32;
pub const G_L4023: Global<value_G_L4023> = Global::new("DAT_1401d0408", Some(0x1401d0408));

/// Source symbol DAT_1401d0420. Contents remain caller supplied.
pub type value_G_L4024 = u8;
pub const G_L4024: Global<value_G_L4024> = Global::new("DAT_1401d0420", Some(0x1401d0420));

/// Source symbol DAT_1401d0424. Contents remain caller supplied.
pub type value_G_L4025 = u8;
pub const G_L4025: Global<value_G_L4025> = Global::new("DAT_1401d0424", Some(0x1401d0424));

/// Source symbol DAT_1401d0410. Contents remain caller supplied.
pub type value_G_L4026 = u8;
pub const G_L4026: Global<value_G_L4026> = Global::new("DAT_1401d0410", Some(0x1401d0410));

/// Source symbol UNK_1401d0414. Contents remain caller supplied.
pub type value_G_L4027 = u32;
pub const G_L4027: Global<value_G_L4027> = Global::new("UNK_1401d0414", None);

/// Source symbol UNK_1401d0418. Contents remain caller supplied.
pub type value_G_L4028 = u32;
pub const G_L4028: Global<value_G_L4028> = Global::new("UNK_1401d0418", None);

/// Source symbol UNK_1401d041c. Contents remain caller supplied.
pub type value_G_L4029 = u32;
pub const G_L4029: Global<value_G_L4029> = Global::new("UNK_1401d041c", None);

/// Source symbol DAT_1401d0488. Contents remain caller supplied.
pub type value_G_L4030 = u32;
pub const G_L4030: Global<value_G_L4030> = Global::new("DAT_1401d0488", Some(0x1401d0488));

/// Source symbol DAT_1401d04a0. Contents remain caller supplied.
pub type value_G_L4031 = u8;
pub const G_L4031: Global<value_G_L4031> = Global::new("DAT_1401d04a0", Some(0x1401d04a0));

/// Source symbol DAT_1401d04a4. Contents remain caller supplied.
pub type value_G_L4032 = u8;
pub const G_L4032: Global<value_G_L4032> = Global::new("DAT_1401d04a4", Some(0x1401d04a4));

/// Source symbol DAT_1401d0490. Contents remain caller supplied.
pub type value_G_L4033 = u8;
pub const G_L4033: Global<value_G_L4033> = Global::new("DAT_1401d0490", Some(0x1401d0490));

/// Source symbol UNK_1401d0494. Contents remain caller supplied.
pub type value_G_L4034 = u32;
pub const G_L4034: Global<value_G_L4034> = Global::new("UNK_1401d0494", None);

/// Source symbol UNK_1401d0498. Contents remain caller supplied.
pub type value_G_L4035 = u32;
pub const G_L4035: Global<value_G_L4035> = Global::new("UNK_1401d0498", None);

/// Source symbol UNK_1401d049c. Contents remain caller supplied.
pub type value_G_L4036 = u32;
pub const G_L4036: Global<value_G_L4036> = Global::new("UNK_1401d049c", None);

/// Source symbol DAT_1401d04a8. Contents remain caller supplied.
pub type value_G_L4037 = u32;
pub const G_L4037: Global<value_G_L4037> = Global::new("DAT_1401d04a8", Some(0x1401d04a8));

/// Source symbol DAT_1401d04c0. Contents remain caller supplied.
pub type value_G_L4038 = u8;
pub const G_L4038: Global<value_G_L4038> = Global::new("DAT_1401d04c0", Some(0x1401d04c0));

/// Source symbol DAT_1401d04c4. Contents remain caller supplied.
pub type value_G_L4039 = u8;
pub const G_L4039: Global<value_G_L4039> = Global::new("DAT_1401d04c4", Some(0x1401d04c4));

/// Source symbol DAT_1401d04b0. Contents remain caller supplied.
pub type value_G_L4040 = u8;
pub const G_L4040: Global<value_G_L4040> = Global::new("DAT_1401d04b0", Some(0x1401d04b0));

/// Source symbol UNK_1401d04b4. Contents remain caller supplied.
pub type value_G_L4041 = u32;
pub const G_L4041: Global<value_G_L4041> = Global::new("UNK_1401d04b4", None);

/// Source symbol UNK_1401d04b8. Contents remain caller supplied.
pub type value_G_L4042 = u32;
pub const G_L4042: Global<value_G_L4042> = Global::new("UNK_1401d04b8", None);

/// Source symbol UNK_1401d04bc. Contents remain caller supplied.
pub type value_G_L4043 = u32;
pub const G_L4043: Global<value_G_L4043> = Global::new("UNK_1401d04bc", None);

/// Source symbol DAT_1401d04c8. Contents remain caller supplied.
pub type value_G_L4044 = u32;
pub const G_L4044: Global<value_G_L4044> = Global::new("DAT_1401d04c8", Some(0x1401d04c8));

/// Source symbol DAT_1401d04e0. Contents remain caller supplied.
pub type value_G_L4045 = u8;
pub const G_L4045: Global<value_G_L4045> = Global::new("DAT_1401d04e0", Some(0x1401d04e0));

/// Source symbol DAT_1401d04e4. Contents remain caller supplied.
pub type value_G_L4046 = u8;
pub const G_L4046: Global<value_G_L4046> = Global::new("DAT_1401d04e4", Some(0x1401d04e4));

/// Source symbol DAT_1401d04d0. Contents remain caller supplied.
pub type value_G_L4047 = u8;
pub const G_L4047: Global<value_G_L4047> = Global::new("DAT_1401d04d0", Some(0x1401d04d0));

/// Source symbol UNK_1401d04d4. Contents remain caller supplied.
pub type value_G_L4048 = u32;
pub const G_L4048: Global<value_G_L4048> = Global::new("UNK_1401d04d4", None);

/// Source symbol UNK_1401d04d8. Contents remain caller supplied.
pub type value_G_L4049 = u32;
pub const G_L4049: Global<value_G_L4049> = Global::new("UNK_1401d04d8", None);

/// Source symbol UNK_1401d04dc. Contents remain caller supplied.
pub type value_G_L4050 = u32;
pub const G_L4050: Global<value_G_L4050> = Global::new("UNK_1401d04dc", None);

/// Source symbol DAT_1401d04e8. Contents remain caller supplied.
pub type value_G_L4051 = u32;
pub const G_L4051: Global<value_G_L4051> = Global::new("DAT_1401d04e8", Some(0x1401d04e8));

/// Source symbol DAT_1401d0500. Contents remain caller supplied.
pub type value_G_L4052 = u8;
pub const G_L4052: Global<value_G_L4052> = Global::new("DAT_1401d0500", Some(0x1401d0500));

/// Source symbol DAT_1401d04f0. Contents remain caller supplied.
pub type value_G_L4053 = u8;
pub const G_L4053: Global<value_G_L4053> = Global::new("DAT_1401d04f0", Some(0x1401d04f0));

/// Source symbol UNK_1401d04f4. Contents remain caller supplied.
pub type value_G_L4054 = u32;
pub const G_L4054: Global<value_G_L4054> = Global::new("UNK_1401d04f4", None);

/// Source symbol UNK_1401d04f8. Contents remain caller supplied.
pub type value_G_L4055 = u32;
pub const G_L4055: Global<value_G_L4055> = Global::new("UNK_1401d04f8", None);

/// Source symbol UNK_1401d04fc. Contents remain caller supplied.
pub type value_G_L4056 = u32;
pub const G_L4056: Global<value_G_L4056> = Global::new("UNK_1401d04fc", None);

/// Source symbol DAT_140107f08. Contents remain caller supplied.
pub type value_G_L4058 = u8;
pub const G_L4058: Global<value_G_L4058> = Global::new("DAT_140107f08", Some(0x140107f08));

/// Source symbol DAT_140107fa0. Contents remain caller supplied.
pub type value_G_L4059 = u8;
pub const G_L4059: Global<value_G_L4059> = Global::new("DAT_140107fa0", Some(0x140107fa0));

/// Source symbol DAT_140107fa8. Contents remain caller supplied.
pub type value_G_L4060 = u8;
pub const G_L4060: Global<value_G_L4060> = Global::new("DAT_140107fa8", Some(0x140107fa8));

/// Source symbol DAT_140108008. Contents remain caller supplied.
pub type value_G_L4061 = u8;
pub const G_L4061: Global<value_G_L4061> = Global::new("DAT_140108008", Some(0x140108008));

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4062 = Address<u64>;
pub const G_L4062: Global<value_G_L4062> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol DAT_00000002. Contents remain caller supplied.
pub type value_G_L4063 = u8;
pub const G_L4063: Global<value_G_L4063> = Global::new("DAT_00000002", None);

/// Source symbol DAT_140108298. Contents remain caller supplied.
pub type value_G_L4064 = u64;
pub const G_L4064: Global<value_G_L4064> = Global::new("DAT_140108298", Some(0x140108298));

/// Source symbol FUN_140017a30. Contents remain caller supplied.
pub type value_G_L4065 = u8;
pub const G_L4065: Global<value_G_L4065> = Global::new("FUN_140017a30", Some(0x140017a30));

/// Source symbol DAT_1401d06a0. Contents remain caller supplied.
pub type value_G_L4066 = u8;
pub const G_L4066: Global<value_G_L4066> = Global::new("DAT_1401d06a0", Some(0x1401d06a0));

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4067 = u64;
pub const G_L4067: Global<value_G_L4067> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol DAT_140105578. Contents remain caller supplied.
pub type value_G_L4068 = u64;
pub const G_L4068: Global<value_G_L4068> = Global::new("DAT_140105578", Some(0x140105578));

/// Source symbol FUN_1400b2750. Contents remain caller supplied.
pub type value_G_L4069 = u8;
pub const G_L4069: Global<value_G_L4069> = Global::new("FUN_1400b2750", Some(0x1400b2750));

/// Source symbol FUN_1400b2740. Contents remain caller supplied.
pub type value_G_L4070 = u8;
pub const G_L4070: Global<value_G_L4070> = Global::new("FUN_1400b2740", Some(0x1400b2740));

/// Source symbol DAT_1401d0520. Contents remain caller supplied.
pub type value_G_L4071 = u8;
pub const G_L4071: Global<value_G_L4071> = Global::new("DAT_1401d0520", Some(0x1401d0520));

/// Source symbol DAT_1400e4bf0. Contents remain caller supplied.
pub type value_G_L4072 = u64;
pub const G_L4072: Global<value_G_L4072> = Global::new("DAT_1400e4bf0", Some(0x1400e4bf0));

/// Source symbol DAT_1400e4c10. Contents remain caller supplied.
pub type value_G_L4073 = u64;
pub const G_L4073: Global<value_G_L4073> = Global::new("DAT_1400e4c10", Some(0x1400e4c10));

/// Source symbol DAT_1400e4c60. Contents remain caller supplied.
pub type value_G_L4074 = u64;
pub const G_L4074: Global<value_G_L4074> = Global::new("DAT_1400e4c60", Some(0x1400e4c60));

/// Source symbol DAT_140106428. Contents remain caller supplied.
pub type value_G_L4075 = u64;
pub const G_L4075: Global<value_G_L4075> = Global::new("DAT_140106428", Some(0x140106428));

/// Source symbol PTR_vftable_1401790c8. Contents remain caller supplied.
pub type value_G_L4076 = Address<Unknown>;
pub const G_L4076: Global<value_G_L4076> = Global::new("PTR_vftable_1401790c8", None);

/// Source symbol DAT_1401d0900. Contents remain caller supplied.
pub type value_G_L4077 = u64;
pub const G_L4077: Global<value_G_L4077> = Global::new("DAT_1401d0900", Some(0x1401d0900));

/// Source symbol DAT_1401083e0. Contents remain caller supplied.
pub type value_G_L4078 = u8;
pub const G_L4078: Global<value_G_L4078> = Global::new("DAT_1401083e0", Some(0x1401083e0));

/// Source symbol DAT_140105598. Contents remain caller supplied.
pub type value_G_L4079 = u64;
pub const G_L4079: Global<value_G_L4079> = Global::new("DAT_140105598", Some(0x140105598));

/// Source symbol DAT_1401081f8. Contents remain caller supplied.
pub type value_G_L4080 = u8;
pub const G_L4080: Global<value_G_L4080> = Global::new("DAT_1401081f8", Some(0x1401081f8));

/// Source symbol DAT_14016ab98. Contents remain caller supplied.
pub type value_G_L4081 = u8;
pub const G_L4081: Global<value_G_L4081> = Global::new("DAT_14016ab98", Some(0x14016ab98));

/// Source symbol DAT_1401cfc48. Contents remain caller supplied.
pub type value_G_L4082 = u8;
pub const G_L4082: Global<value_G_L4082> = Global::new("DAT_1401cfc48", Some(0x1401cfc48));

/// Source symbol DAT_1401cfc40. Contents remain caller supplied.
pub type value_G_L4083 = u64;
pub const G_L4083: Global<value_G_L4083> = Global::new("DAT_1401cfc40", Some(0x1401cfc40));

/// Source symbol DAT_140108440. Contents remain caller supplied.
pub type value_G_L4084 = u8;
pub const G_L4084: Global<value_G_L4084> = Global::new("DAT_140108440", Some(0x140108440));

/// Source symbol DAT_1401cfc50. Contents remain caller supplied.
pub type value_G_L4085 = u8;
pub const G_L4085: Global<value_G_L4085> = Global::new("DAT_1401cfc50", Some(0x1401cfc50));

/// Source symbol DAT_1401cfc40. Contents remain caller supplied.
pub type value_G_L4086 = i64;
pub const G_L4086: Global<value_G_L4086> = Global::new("DAT_1401cfc40", Some(0x1401cfc40));

/// Source symbol FUN_1400188e0. Contents remain caller supplied.
pub type value_G_L4087 = u8;
pub const G_L4087: Global<value_G_L4087> = Global::new("FUN_1400188e0", Some(0x1400188e0));

/// Source symbol PTR_vftable_1401790b8. Contents remain caller supplied.
pub type value_G_L4088 = Address<Unknown>;
pub const G_L4088: Global<value_G_L4088> = Global::new("PTR_vftable_1401790b8", None);

/// Source symbol DAT_14016abe0. Contents remain caller supplied.
pub type value_G_L4089 = u8;
pub const G_L4089: Global<value_G_L4089> = Global::new("DAT_14016abe0", Some(0x14016abe0));

/// Source symbol DAT_140108578. Contents remain caller supplied.
pub type value_G_L4090 = u8;
pub const G_L4090: Global<value_G_L4090> = Global::new("DAT_140108578", Some(0x140108578));

/// Source symbol DAT_14010857c. Contents remain caller supplied.
pub type value_G_L4091 = u8;
pub const G_L4091: Global<value_G_L4091> = Global::new("DAT_14010857c", Some(0x14010857c));

/// Source symbol DAT_1401084e0. Contents remain caller supplied.
pub type value_G_L4092 = u8;
pub const G_L4092: Global<value_G_L4092> = Global::new("DAT_1401084e0", Some(0x1401084e0));

/// Source symbol DAT_14016acd0. Contents remain caller supplied.
pub type value_G_L4093 = u8;
pub const G_L4093: Global<value_G_L4093> = Global::new("DAT_14016acd0", Some(0x14016acd0));

/// Source symbol DAT_1401d0b30. Contents remain caller supplied.
pub type value_G_L4094 = u8;
pub const G_L4094: Global<value_G_L4094> = Global::new("DAT_1401d0b30", Some(0x1401d0b30));

/// Source symbol DAT_140179148. Contents remain caller supplied.
pub type value_G_L4095 = i32;
pub const G_L4095: Global<value_G_L4095> = Global::new("DAT_140179148", Some(0x140179148));

/// Source symbol DAT_140179144. Contents remain caller supplied.
pub type value_G_L4096 = i32;
pub const G_L4096: Global<value_G_L4096> = Global::new("DAT_140179144", Some(0x140179144));

/// Source symbol DAT_140179140. Contents remain caller supplied.
pub type value_G_L4097 = i32;
pub const G_L4097: Global<value_G_L4097> = Global::new("DAT_140179140", Some(0x140179140));

/// Source symbol DAT_140179144. Contents remain caller supplied.
pub type value_G_L4098 = u32;
pub const G_L4098: Global<value_G_L4098> = Global::new("DAT_140179144", Some(0x140179144));

/// Source symbol DAT_140179140. Contents remain caller supplied.
pub type value_G_L4099 = u32;
pub const G_L4099: Global<value_G_L4099> = Global::new("DAT_140179140", Some(0x140179140));

/// Source symbol DAT_1401cfca8. Contents remain caller supplied.
pub type value_G_L4100 = u8;
pub const G_L4100: Global<value_G_L4100> = Global::new("DAT_1401cfca8", Some(0x1401cfca8));

/// Source symbol DAT_1401cfca0. Contents remain caller supplied.
pub type value_G_L4101 = i64;
pub const G_L4101: Global<value_G_L4101> = Global::new("DAT_1401cfca0", Some(0x1401cfca0));

/// Source symbol DAT_140179150. Contents remain caller supplied.
pub type value_G_L4102 = u64;
pub const G_L4102: Global<value_G_L4102> = Global::new("DAT_140179150", Some(0x140179150));

/// Source symbol DAT_140179128. Contents remain caller supplied.
pub type value_G_L4103 = Address<u64>;
pub const G_L4103: Global<value_G_L4103> = Global::new("DAT_140179128", Some(0x140179128));

/// Source symbol DAT_140179130. Contents remain caller supplied.
pub type value_G_L4104 = Address<u64>;
pub const G_L4104: Global<value_G_L4104> = Global::new("DAT_140179130", Some(0x140179130));

/// Source symbol DAT_1401cfc88. Contents remain caller supplied.
pub type value_G_L4105 = u8;
pub const G_L4105: Global<value_G_L4105> = Global::new("DAT_1401cfc88", Some(0x1401cfc88));

/// Source symbol DAT_1401cfc80. Contents remain caller supplied.
pub type value_G_L4106 = i64;
pub const G_L4106: Global<value_G_L4106> = Global::new("DAT_1401cfc80", Some(0x1401cfc80));

/// Source symbol DAT_1401cfc68. Contents remain caller supplied.
pub type value_G_L4107 = u8;
pub const G_L4107: Global<value_G_L4107> = Global::new("DAT_1401cfc68", Some(0x1401cfc68));

/// Source symbol DAT_1401cfc60. Contents remain caller supplied.
pub type value_G_L4108 = i64;
pub const G_L4108: Global<value_G_L4108> = Global::new("DAT_1401cfc60", Some(0x1401cfc60));

/// Source symbol DAT_140108998. Contents remain caller supplied.
pub type value_G_L4109 = u8;
pub const G_L4109: Global<value_G_L4109> = Global::new("DAT_140108998", Some(0x140108998));

/// Source symbol FUN_14001fff0. Contents remain caller supplied.
pub type value_G_L4113 = u8;
pub const G_L4113: Global<value_G_L4113> = Global::new("FUN_14001fff0", Some(0x14001fff0));

/// Source symbol FUN_140020800. Contents remain caller supplied.
pub type value_G_L4114 = u8;
pub const G_L4114: Global<value_G_L4114> = Global::new("FUN_140020800", Some(0x140020800));

/// Source symbol DAT_140179128. Contents remain caller supplied.
pub type value_G_L4115 = u64;
pub const G_L4115: Global<value_G_L4115> = Global::new("DAT_140179128", Some(0x140179128));

/// Source symbol DAT_140179130. Contents remain caller supplied.
pub type value_G_L4116 = Address<u32>;
pub const G_L4116: Global<value_G_L4116> = Global::new("DAT_140179130", Some(0x140179130));

/// Source symbol DAT_140179150. Contents remain caller supplied.
pub type value_G_L4117 = Address<i64>;
pub const G_L4117: Global<value_G_L4117> = Global::new("DAT_140179150", Some(0x140179150));

/// Source symbol DAT_140179138. Contents remain caller supplied.
pub type value_G_L4118 = Address<u32>;
pub const G_L4118: Global<value_G_L4118> = Global::new("DAT_140179138", Some(0x140179138));

/// Source symbol DAT_1401093e0. Contents remain caller supplied.
pub type value_G_L4119 = u64;
pub const G_L4119: Global<value_G_L4119> = Global::new("DAT_1401093e0", Some(0x1401093e0));

/// Source symbol DAT_140179110. Contents remain caller supplied.
pub type value_G_L4120 = Address<i32>;
pub const G_L4120: Global<value_G_L4120> = Global::new("DAT_140179110", Some(0x140179110));

/// Source symbol DAT_00000078. Contents remain caller supplied.
pub type value_G_L4121 = u8;
pub const G_L4121: Global<value_G_L4121> = Global::new("DAT_00000078", None);

/// Source symbol DAT_00000070. Contents remain caller supplied.
pub type value_G_L4122 = u8;
pub const G_L4122: Global<value_G_L4122> = Global::new("DAT_00000070", None);

/// Source symbol DAT_1401093f0. Contents remain caller supplied.
pub type value_G_L4123 = u8;
pub const G_L4123: Global<value_G_L4123> = Global::new("DAT_1401093f0", Some(0x1401093f0));

/// Source symbol DAT_00000020. Contents remain caller supplied.
pub type value_G_L4124 = u8;
pub const G_L4124: Global<value_G_L4124> = Global::new("DAT_00000020", None);

/// Source symbol DAT_1401092f0. Contents remain caller supplied.
pub type value_G_L4125 = u8;
pub const G_L4125: Global<value_G_L4125> = Global::new("DAT_1401092f0", Some(0x1401092f0));

/// Source symbol DAT_140179110. Contents remain caller supplied.
pub type value_G_L4126 = u64;
pub const G_L4126: Global<value_G_L4126> = Global::new("DAT_140179110", Some(0x140179110));

/// Source symbol DAT_1401091e8. Contents remain caller supplied.
pub type value_G_L4127 = u8;
pub const G_L4127: Global<value_G_L4127> = Global::new("DAT_1401091e8", Some(0x1401091e8));

/// Source symbol DAT_1401091c8. Contents remain caller supplied.
pub type value_G_L4128 = u8;
pub const G_L4128: Global<value_G_L4128> = Global::new("DAT_1401091c8", Some(0x1401091c8));

/// Source symbol DAT_0000001c. Contents remain caller supplied.
pub type value_G_L4129 = u8;
pub const G_L4129: Global<value_G_L4129> = Global::new("DAT_0000001c", None);

/// Source symbol DAT_140108ba8. Contents remain caller supplied.
pub type value_G_L4130 = u8;
pub const G_L4130: Global<value_G_L4130> = Global::new("DAT_140108ba8", Some(0x140108ba8));

/// Source symbol DAT_140108b50. Contents remain caller supplied.
pub type value_G_L4131 = u8;
pub const G_L4131: Global<value_G_L4131> = Global::new("DAT_140108b50", Some(0x140108b50));

/// Source symbol DAT_1401093a0. Contents remain caller supplied.
pub type value_G_L4132 = u64;
pub const G_L4132: Global<value_G_L4132> = Global::new("DAT_1401093a0", Some(0x1401093a0));

/// Source symbol DAT_1401cfc60. Contents remain caller supplied.
pub type value_G_L4133 = u64;
pub const G_L4133: Global<value_G_L4133> = Global::new("DAT_1401cfc60", Some(0x1401cfc60));

/// Source symbol DAT_1401094a8. Contents remain caller supplied.
pub type value_G_L4134 = u8;
pub const G_L4134: Global<value_G_L4134> = Global::new("DAT_1401094a8", Some(0x1401094a8));

/// Source symbol DAT_1401cfc70. Contents remain caller supplied.
pub type value_G_L4135 = u8;
pub const G_L4135: Global<value_G_L4135> = Global::new("DAT_1401cfc70", Some(0x1401cfc70));

/// Source symbol DAT_140109460. Contents remain caller supplied.
pub type value_G_L4136 = u8;
pub const G_L4136: Global<value_G_L4136> = Global::new("DAT_140109460", Some(0x140109460));

/// Source symbol DAT_1401cfc80. Contents remain caller supplied.
pub type value_G_L4137 = u64;
pub const G_L4137: Global<value_G_L4137> = Global::new("DAT_1401cfc80", Some(0x1401cfc80));

/// Source symbol DAT_140109470. Contents remain caller supplied.
pub type value_G_L4138 = u8;
pub const G_L4138: Global<value_G_L4138> = Global::new("DAT_140109470", Some(0x140109470));

/// Source symbol DAT_1401cfc90. Contents remain caller supplied.
pub type value_G_L4139 = u8;
pub const G_L4139: Global<value_G_L4139> = Global::new("DAT_1401cfc90", Some(0x1401cfc90));

/// Source symbol DAT_1401cfca0. Contents remain caller supplied.
pub type value_G_L4140 = u64;
pub const G_L4140: Global<value_G_L4140> = Global::new("DAT_1401cfca0", Some(0x1401cfca0));

/// Source symbol DAT_1401cfcb0. Contents remain caller supplied.
pub type value_G_L4141 = u8;
pub const G_L4141: Global<value_G_L4141> = Global::new("DAT_1401cfcb0", Some(0x1401cfcb0));

/// Source symbol DAT_140109508. Contents remain caller supplied.
pub type value_G_L4142 = u8;
pub const G_L4142: Global<value_G_L4142> = Global::new("DAT_140109508", Some(0x140109508));

/// Source symbol DAT_140106458. Contents remain caller supplied.
pub type value_G_L4143 = u32;
pub const G_L4143: Global<value_G_L4143> = Global::new("DAT_140106458", Some(0x140106458));

/// Source symbol DAT_1401096b0. Contents remain caller supplied.
pub type value_G_L4144 = u8;
pub const G_L4144: Global<value_G_L4144> = Global::new("DAT_1401096b0", Some(0x1401096b0));

/// Source symbol PTR_s_AiInformation_1401121a0. Contents remain caller supplied.
pub type value_G_L4145 = Address<Unknown>;
pub const G_L4145: Global<value_G_L4145> = Global::new("PTR_s_AiInformation_1401121a0", None);

/// Source symbol DAT_140109690. Contents remain caller supplied.
pub type value_G_L4146 = u8;
pub const G_L4146: Global<value_G_L4146> = Global::new("DAT_140109690", Some(0x140109690));

/// Source symbol DAT_1401cfcd8. Contents remain caller supplied.
pub type value_G_L4147 = u8;
pub const G_L4147: Global<value_G_L4147> = Global::new("DAT_1401cfcd8", Some(0x1401cfcd8));

/// Source symbol DAT_1401cfcd0. Contents remain caller supplied.
pub type value_G_L4148 = i64;
pub const G_L4148: Global<value_G_L4148> = Global::new("DAT_1401cfcd0", Some(0x1401cfcd0));

/// Source symbol DAT_14010c580. Contents remain caller supplied.
pub type value_G_L4149 = u8;
pub const G_L4149: Global<value_G_L4149> = Global::new("DAT_14010c580", Some(0x14010c580));

/// Source symbol FUN_1400b41e0. Contents remain caller supplied.
pub type value_G_L4150 = u8;
pub const G_L4150: Global<value_G_L4150> = Global::new("FUN_1400b41e0", Some(0x1400b41e0));

/// Source symbol FUN_1400b41d0. Contents remain caller supplied.
pub type value_G_L4151 = u8;
pub const G_L4151: Global<value_G_L4151> = Global::new("FUN_1400b41d0", Some(0x1400b41d0));

/// Source symbol IMAGE_DOS_HEADER_140000000. Contents remain caller supplied.
pub type value_G_L4152 = t_4a3399835aa5;
pub const G_L4152: Global<value_G_L4152> =
    Global::new("IMAGE_DOS_HEADER_140000000", Some(0x140000000));

/// Source symbol DAT_14002469c. Contents remain caller supplied.
pub type value_G_L4153 = u8;
pub const G_L4153: Global<value_G_L4153> = Global::new("DAT_14002469c", Some(0x14002469c));

/// Source symbol DAT_14010c7a0. Contents remain caller supplied.
pub type value_G_L4154 = u8;
pub const G_L4154: Global<value_G_L4154> = Global::new("DAT_14010c7a0", Some(0x14010c7a0));

/// Source symbol LAB_140026a00. Contents remain caller supplied.
pub type value_G_L4155 = u8;
pub const G_L4155: Global<value_G_L4155> = Global::new("LAB_140026a00", None);

/// Source symbol DAT_14010d018. Contents remain caller supplied.
pub type value_G_L4156 = u16;
pub const G_L4156: Global<value_G_L4156> = Global::new("DAT_14010d018", Some(0x14010d018));

/// Source symbol DAT_14010ba00. Contents remain caller supplied.
pub type value_G_L4157 = u32;
pub const G_L4157: Global<value_G_L4157> = Global::new("DAT_14010ba00", Some(0x14010ba00));

/// Source symbol DAT_14010bd28. Contents remain caller supplied.
pub type value_G_L4158 = u8;
pub const G_L4158: Global<value_G_L4158> = Global::new("DAT_14010bd28", Some(0x14010bd28));

/// Source symbol DAT_14010d134. Contents remain caller supplied.
pub type value_G_L4159 = u8;
pub const G_L4159: Global<value_G_L4159> = Global::new("DAT_14010d134", Some(0x14010d134));

/// Source symbol DAT_14010d138. Contents remain caller supplied.
pub type value_G_L4160 = u8;
pub const G_L4160: Global<value_G_L4160> = Global::new("DAT_14010d138", Some(0x14010d138));

/// Source symbol DAT_14010d13c. Contents remain caller supplied.
pub type value_G_L4161 = u8;
pub const G_L4161: Global<value_G_L4161> = Global::new("DAT_14010d13c", Some(0x14010d13c));

/// Source symbol DAT_14010d140. Contents remain caller supplied.
pub type value_G_L4162 = u8;
pub const G_L4162: Global<value_G_L4162> = Global::new("DAT_14010d140", Some(0x14010d140));

/// Source symbol DAT_14010d144. Contents remain caller supplied.
pub type value_G_L4163 = u8;
pub const G_L4163: Global<value_G_L4163> = Global::new("DAT_14010d144", Some(0x14010d144));

/// Source symbol UNK_1401097ec. Contents remain caller supplied.
pub type value_G_L4164 = u8;
pub const G_L4164: Global<value_G_L4164> = Global::new("UNK_1401097ec", None);

/// Source symbol DAT_1401097f0. Contents remain caller supplied.
pub type value_G_L4165 = u8;
pub const G_L4165: Global<value_G_L4165> = Global::new("DAT_1401097f0", Some(0x1401097f0));

/// Source symbol UNK_140109926. Contents remain caller supplied.
pub type value_G_L4166 = u8;
pub const G_L4166: Global<value_G_L4166> = Global::new("UNK_140109926", None);

/// Source symbol UNK_1401099cc. Contents remain caller supplied.
pub type value_G_L4167 = u8;
pub const G_L4167: Global<value_G_L4167> = Global::new("UNK_1401099cc", None);

/// Source symbol DAT_1401099d0. Contents remain caller supplied.
pub type value_G_L4168 = u8;
pub const G_L4168: Global<value_G_L4168> = Global::new("DAT_1401099d0", Some(0x1401099d0));

/// Source symbol UNK_14010af3a. Contents remain caller supplied.
pub type value_G_L4169 = u8;
pub const G_L4169: Global<value_G_L4169> = Global::new("UNK_14010af3a", None);

/// Source symbol DAT_1401040f0. Contents remain caller supplied.
pub type value_G_L4170 = u8;
pub const G_L4170: Global<value_G_L4170> = Global::new("DAT_1401040f0", Some(0x1401040f0));

/// Source symbol DAT_140104710. Contents remain caller supplied.
pub type value_G_L4171 = u8;
pub const G_L4171: Global<value_G_L4171> = Global::new("DAT_140104710", Some(0x140104710));

/// Source symbol DAT_1400e8020. Contents remain caller supplied.
pub type value_G_L4172 = u8;
pub const G_L4172: Global<value_G_L4172> = Global::new("DAT_1400e8020", Some(0x1400e8020));

/// Source symbol DAT_1400e8028. Contents remain caller supplied.
pub type value_G_L4173 = u8;
pub const G_L4173: Global<value_G_L4173> = Global::new("DAT_1400e8028", Some(0x1400e8028));

/// Source symbol DAT_1400e9260. Contents remain caller supplied.
pub type value_G_L4174 = u8;
pub const G_L4174: Global<value_G_L4174> = Global::new("DAT_1400e9260", Some(0x1400e9260));

/// Source symbol DAT_1400e9268. Contents remain caller supplied.
pub type value_G_L4175 = u8;
pub const G_L4175: Global<value_G_L4175> = Global::new("DAT_1400e9268", Some(0x1400e9268));

/// Source symbol DAT_14010bd30. Contents remain caller supplied.
pub type value_G_L4176 = u8;
pub const G_L4176: Global<value_G_L4176> = Global::new("DAT_14010bd30", Some(0x14010bd30));

/// Source symbol DAT_14010be70. Contents remain caller supplied.
pub type value_G_L4177 = u8;
pub const G_L4177: Global<value_G_L4177> = Global::new("DAT_14010be70", Some(0x14010be70));

/// Source symbol DAT_14010c360. Contents remain caller supplied.
pub type value_G_L4178 = u8;
pub const G_L4178: Global<value_G_L4178> = Global::new("DAT_14010c360", Some(0x14010c360));

/// Source symbol UNK_1400ea6c0. Contents remain caller supplied.
pub type value_G_L4179 = u8;
pub const G_L4179: Global<value_G_L4179> = Global::new("UNK_1400ea6c0", None);

/// Source symbol UNK_1400f1980. Contents remain caller supplied.
pub type value_G_L4180 = u8;
pub const G_L4180: Global<value_G_L4180> = Global::new("UNK_1400f1980", None);

/// Source symbol DAT_14010bf30. Contents remain caller supplied.
pub type value_G_L4181 = u16;
pub const G_L4181: Global<value_G_L4181> = Global::new("DAT_14010bf30", Some(0x14010bf30));

/// Source symbol DAT_14010bfb0. Contents remain caller supplied.
pub type value_G_L4182 = u8;
pub const G_L4182: Global<value_G_L4182> = Global::new("DAT_14010bfb0", Some(0x14010bfb0));

/// Source symbol DAT_14010c000. Contents remain caller supplied.
pub type value_G_L4183 = u8;
pub const G_L4183: Global<value_G_L4183> = Global::new("DAT_14010c000", Some(0x14010c000));

/// Source symbol DAT_14010c002. Contents remain caller supplied.
pub type value_G_L4184 = u8;
pub const G_L4184: Global<value_G_L4184> = Global::new("DAT_14010c002", Some(0x14010c002));

/// Source symbol DAT_140179040. Contents remain caller supplied.
pub type value_G_L4185 = u64;
pub const G_L4185: Global<value_G_L4185> = Global::new("DAT_140179040", Some(0x140179040));

/// Source symbol DAT_140103f40. Contents remain caller supplied.
pub type value_G_L4186 = u8;
pub const G_L4186: Global<value_G_L4186> = Global::new("DAT_140103f40", Some(0x140103f40));

/// Source symbol DAT_140104040. Contents remain caller supplied.
pub type value_G_L4187 = u32;
pub const G_L4187: Global<value_G_L4187> = Global::new("DAT_140104040", Some(0x140104040));

/// Source symbol DAT_14010c0e0. Contents remain caller supplied.
pub type value_G_L4188 = u8;
pub const G_L4188: Global<value_G_L4188> = Global::new("DAT_14010c0e0", Some(0x14010c0e0));

/// Source symbol DAT_14010c1e0. Contents remain caller supplied.
pub type value_G_L4189 = u8;
pub const G_L4189: Global<value_G_L4189> = Global::new("DAT_14010c1e0", Some(0x14010c1e0));

/// Source symbol DAT_14010c428. Contents remain caller supplied.
pub type value_G_L4190 = u8;
pub const G_L4190: Global<value_G_L4190> = Global::new("DAT_14010c428", Some(0x14010c428));

/// Source symbol DAT_14010c450. Contents remain caller supplied.
pub type value_G_L4191 = u8;
pub const G_L4191: Global<value_G_L4191> = Global::new("DAT_14010c450", Some(0x14010c450));

/// Source symbol DAT_14016ad30. Contents remain caller supplied.
pub type value_G_L4192 = u8;
pub const G_L4192: Global<value_G_L4192> = Global::new("DAT_14016ad30", Some(0x14016ad30));

/// Source symbol DAT_1401cfcc0. Contents remain caller supplied.
pub type value_G_L4193 = i64;
pub const G_L4193: Global<value_G_L4193> = Global::new("DAT_1401cfcc0", Some(0x1401cfcc0));

/// Source symbol PTR_id_1400e0840. Contents remain caller supplied.
pub type value_G_L4194 = Address<u8>;
pub const G_L4194: Global<value_G_L4194> = Global::new("PTR_id_1400e0840", None);

/// Source symbol PTR__Id_cnt_1400e0848. Contents remain caller supplied.
pub type value_G_L4195 = Address<u8>;
pub const G_L4195: Global<value_G_L4195> = Global::new("PTR__Id_cnt_1400e0848", None);

/// Source symbol DAT_14016ad98. Contents remain caller supplied.
pub type value_G_L4196 = u8;
pub const G_L4196: Global<value_G_L4196> = Global::new("DAT_14016ad98", Some(0x14016ad98));

/// Source symbol DAT_1401cfc00. Contents remain caller supplied.
pub type value_G_L4197 = i64;
pub const G_L4197: Global<value_G_L4197> = Global::new("DAT_1401cfc00", Some(0x1401cfc00));

/// Source symbol DAT_1401cfcd0. Contents remain caller supplied.
pub type value_G_L4198 = u64;
pub const G_L4198: Global<value_G_L4198> = Global::new("DAT_1401cfcd0", Some(0x1401cfcd0));

/// Source symbol DAT_14010cb98. Contents remain caller supplied.
pub type value_G_L4199 = u8;
pub const G_L4199: Global<value_G_L4199> = Global::new("DAT_14010cb98", Some(0x14010cb98));

/// Source symbol DAT_1401cfce0. Contents remain caller supplied.
pub type value_G_L4200 = u8;
pub const G_L4200: Global<value_G_L4200> = Global::new("DAT_1401cfce0", Some(0x1401cfce0));

/// Source symbol DAT_1401d0b80. Contents remain caller supplied.
pub type value_G_L4201 = Address<u64>;
pub const G_L4201: Global<value_G_L4201> = Global::new("DAT_1401d0b80", Some(0x1401d0b80));

/// Source symbol DAT_140105fa0. Contents remain caller supplied.
pub type value_G_L4202 = u8;
pub const G_L4202: Global<value_G_L4202> = Global::new("DAT_140105fa0", Some(0x140105fa0));

/// Source symbol FUN_1400b4860. Contents remain caller supplied.
pub type value_G_L4203 = u8;
pub const G_L4203: Global<value_G_L4203> = Global::new("FUN_1400b4860", Some(0x1400b4860));

/// Source symbol FUN_1400b4850. Contents remain caller supplied.
pub type value_G_L4204 = u8;
pub const G_L4204: Global<value_G_L4204> = Global::new("FUN_1400b4850", Some(0x1400b4850));

/// Source symbol FUN_1400b51b0. Contents remain caller supplied.
pub type value_G_L4205 = u8;
pub const G_L4205: Global<value_G_L4205> = Global::new("FUN_1400b51b0", Some(0x1400b51b0));

/// Source symbol FUN_1400b51c0. Contents remain caller supplied.
pub type value_G_L4206 = u8;
pub const G_L4206: Global<value_G_L4206> = Global::new("FUN_1400b51c0", Some(0x1400b51c0));

/// Source symbol FUN_1400b64c0. Contents remain caller supplied.
pub type value_G_L4207 = u8;
pub const G_L4207: Global<value_G_L4207> = Global::new("FUN_1400b64c0", Some(0x1400b64c0));

/// Source symbol DAT_1401d0930. Contents remain caller supplied.
pub type value_G_L4208 = i32;
pub const G_L4208: Global<value_G_L4208> = Global::new("DAT_1401d0930", Some(0x1401d0930));

/// Source symbol DAT_1401d07be. Contents remain caller supplied.
pub type value_G_L4209 = u8;
pub const G_L4209: Global<value_G_L4209> = Global::new("DAT_1401d07be", Some(0x1401d07be));

/// Source symbol FUN_1400b64b0. Contents remain caller supplied.
pub type value_G_L4210 = u8;
pub const G_L4210: Global<value_G_L4210> = Global::new("FUN_1400b64b0", Some(0x1400b64b0));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4211 = Address<u64>;
pub const G_L4211: Global<value_G_L4211> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401d0b80. Contents remain caller supplied.
pub type value_G_L4212 = i64;
pub const G_L4212: Global<value_G_L4212> = Global::new("DAT_1401d0b80", Some(0x1401d0b80));

/// Source symbol FUN_140033700. Contents remain caller supplied.
pub type value_G_L4213 = u8;
pub const G_L4213: Global<value_G_L4213> = Global::new("FUN_140033700", Some(0x140033700));

/// Source symbol FUN_1400b67c0. Contents remain caller supplied.
pub type value_G_L4214 = u8;
pub const G_L4214: Global<value_G_L4214> = Global::new("FUN_1400b67c0", Some(0x1400b67c0));

/// Source symbol FUN_1400b67b0. Contents remain caller supplied.
pub type value_G_L4215 = u8;
pub const G_L4215: Global<value_G_L4215> = Global::new("FUN_1400b67b0", Some(0x1400b67b0));

/// Source symbol DAT_14010d228. Contents remain caller supplied.
pub type value_G_L4216 = u8;
pub const G_L4216: Global<value_G_L4216> = Global::new("DAT_14010d228", Some(0x14010d228));

/// Source symbol DAT_14010d218. Contents remain caller supplied.
pub type value_G_L4217 = u8;
pub const G_L4217: Global<value_G_L4217> = Global::new("DAT_14010d218", Some(0x14010d218));

/// Source symbol DAT_1401cfcf8. Contents remain caller supplied.
pub type value_G_L4218 = u8;
pub const G_L4218: Global<value_G_L4218> = Global::new("DAT_1401cfcf8", Some(0x1401cfcf8));

/// Source symbol DAT_1401cfcf0. Contents remain caller supplied.
pub type value_G_L4219 = u64;
pub const G_L4219: Global<value_G_L4219> = Global::new("DAT_1401cfcf0", Some(0x1401cfcf0));

/// Source symbol DAT_14010d2a8. Contents remain caller supplied.
pub type value_G_L4220 = u8;
pub const G_L4220: Global<value_G_L4220> = Global::new("DAT_14010d2a8", Some(0x14010d2a8));

/// Source symbol DAT_1401cfd00. Contents remain caller supplied.
pub type value_G_L4221 = u8;
pub const G_L4221: Global<value_G_L4221> = Global::new("DAT_1401cfd00", Some(0x1401cfd00));

/// Source symbol DAT_1401d085a. Contents remain caller supplied.
pub type value_G_L4222 = i8;
pub const G_L4222: Global<value_G_L4222> = Global::new("DAT_1401d085a", Some(0x1401d085a));

/// Source symbol DAT_14010d520. Contents remain caller supplied.
pub type value_G_L4223 = u8;
pub const G_L4223: Global<value_G_L4223> = Global::new("DAT_14010d520", Some(0x14010d520));

/// Source symbol DAT_1401d0510. Contents remain caller supplied.
pub type value_G_L4224 = u64;
pub const G_L4224: Global<value_G_L4224> = Global::new("DAT_1401d0510", Some(0x1401d0510));

/// Source symbol DAT_1401d0518. Contents remain caller supplied.
pub type value_G_L4225 = i64;
pub const G_L4225: Global<value_G_L4225> = Global::new("DAT_1401d0518", Some(0x1401d0518));

/// Source symbol FUN_1400340b0. Contents remain caller supplied.
pub type value_G_L4226 = u8;
pub const G_L4226: Global<value_G_L4226> = Global::new("FUN_1400340b0", Some(0x1400340b0));

/// Source symbol DAT_1401d0510. Contents remain caller supplied.
pub type value_G_L4227 = Address<i64>;
pub const G_L4227: Global<value_G_L4227> = Global::new("DAT_1401d0510", Some(0x1401d0510));

/// Source symbol DAT_140106490. Contents remain caller supplied.
pub type value_G_L4228 = u32;
pub const G_L4228: Global<value_G_L4228> = Global::new("DAT_140106490", Some(0x140106490));

/// Source symbol DAT_1401064a0. Contents remain caller supplied.
pub type value_G_L4229 = u8;
pub const G_L4229: Global<value_G_L4229> = Global::new("DAT_1401064a0", Some(0x1401064a0));

/// Source symbol DAT_1401d0518. Contents remain caller supplied.
pub type value_G_L4230 = u64;
pub const G_L4230: Global<value_G_L4230> = Global::new("DAT_1401d0518", Some(0x1401d0518));

/// Source symbol FUN_14003bb10. Contents remain caller supplied.
pub type value_G_L4231 = u8;
pub const G_L4231: Global<value_G_L4231> = Global::new("FUN_14003bb10", Some(0x14003bb10));

/// Source symbol DAT_14010dd70. Contents remain caller supplied.
pub type value_G_L4232 = u8;
pub const G_L4232: Global<value_G_L4232> = Global::new("DAT_14010dd70", Some(0x14010dd70));

/// Source symbol DAT_14010dd80. Contents remain caller supplied.
pub type value_G_L4233 = u8;
pub const G_L4233: Global<value_G_L4233> = Global::new("DAT_14010dd80", Some(0x14010dd80));

/// Source symbol DAT_1401cfd48. Contents remain caller supplied.
pub type value_G_L4234 = u8;
pub const G_L4234: Global<value_G_L4234> = Global::new("DAT_1401cfd48", Some(0x1401cfd48));

/// Source symbol DAT_1401cfd40. Contents remain caller supplied.
pub type value_G_L4235 = u64;
pub const G_L4235: Global<value_G_L4235> = Global::new("DAT_1401cfd40", Some(0x1401cfd40));

/// Source symbol FUN_1400b7910. Contents remain caller supplied.
pub type value_G_L4236 = u8;
pub const G_L4236: Global<value_G_L4236> = Global::new("FUN_1400b7910", Some(0x1400b7910));

/// Source symbol FUN_1400b7900. Contents remain caller supplied.
pub type value_G_L4237 = u8;
pub const G_L4237: Global<value_G_L4237> = Global::new("FUN_1400b7900", Some(0x1400b7900));

/// Source symbol FUN_1400b6f20. Contents remain caller supplied.
pub type value_G_L4238 = u8;
pub const G_L4238: Global<value_G_L4238> = Global::new("FUN_1400b6f20", Some(0x1400b6f20));

/// Source symbol FUN_1400b6f10. Contents remain caller supplied.
pub type value_G_L4239 = u8;
pub const G_L4239: Global<value_G_L4239> = Global::new("FUN_1400b6f10", Some(0x1400b6f10));

/// Source symbol DAT_14010ddd8. Contents remain caller supplied.
pub type value_G_L4240 = u8;
pub const G_L4240: Global<value_G_L4240> = Global::new("DAT_14010ddd8", Some(0x14010ddd8));

/// Source symbol FUN_140039d90. Contents remain caller supplied.
pub type value_G_L4241 = u8;
pub const G_L4241: Global<value_G_L4241> = Global::new("FUN_140039d90", Some(0x140039d90));

/// Source symbol DAT_14016add0. Contents remain caller supplied.
pub type value_G_L4242 = u8;
pub const G_L4242: Global<value_G_L4242> = Global::new("DAT_14016add0", Some(0x14016add0));

/// Source symbol FUN_14003a390. Contents remain caller supplied.
pub type value_G_L4243 = u8;
pub const G_L4243: Global<value_G_L4243> = Global::new("FUN_14003a390", Some(0x14003a390));

/// Source symbol FUN_14003a4b0. Contents remain caller supplied.
pub type value_G_L4244 = u8;
pub const G_L4244: Global<value_G_L4244> = Global::new("FUN_14003a4b0", Some(0x14003a4b0));

/// Source symbol DAT_14016ae90. Contents remain caller supplied.
pub type value_G_L4245 = u8;
pub const G_L4245: Global<value_G_L4245> = Global::new("DAT_14016ae90", Some(0x14016ae90));

/// Source symbol DAT_1401cfd10. Contents remain caller supplied.
pub type value_G_L4246 = u32;
pub const G_L4246: Global<value_G_L4246> = Global::new("DAT_1401cfd10", Some(0x1401cfd10));

/// Source symbol DAT_1401cfd28. Contents remain caller supplied.
pub type value_G_L4247 = Address<u8>;
pub const G_L4247: Global<value_G_L4247> = Global::new("DAT_1401cfd28", Some(0x1401cfd28));

/// Source symbol DAT_1401cfd18. Contents remain caller supplied.
pub type value_G_L4248 = u8;
pub const G_L4248: Global<value_G_L4248> = Global::new("DAT_1401cfd18", Some(0x1401cfd18));

/// Source symbol DAT_1401cfd30. Contents remain caller supplied.
pub type value_G_L4249 = u8;
pub const G_L4249: Global<value_G_L4249> = Global::new("DAT_1401cfd30", Some(0x1401cfd30));

/// Source symbol FUN_1400df760. Contents remain caller supplied.
pub type value_G_L4250 = u8;
pub const G_L4250: Global<value_G_L4250> = Global::new("FUN_1400df760", Some(0x1400df760));

/// Source symbol DAT_1401cfd50. Contents remain caller supplied.
pub type value_G_L4251 = u8;
pub const G_L4251: Global<value_G_L4251> = Global::new("DAT_1401cfd50", Some(0x1401cfd50));

/// Source symbol DAT_14010e2f8. Contents remain caller supplied.
pub type value_G_L4252 = u8;
pub const G_L4252: Global<value_G_L4252> = Global::new("DAT_14010e2f8", Some(0x14010e2f8));

/// Source symbol DAT_1401d03a4. Contents remain caller supplied.
pub type value_G_L4253 = i32;
pub const G_L4253: Global<value_G_L4253> = Global::new("DAT_1401d03a4", Some(0x1401d03a4));

/// Source symbol FUN_140040890. Contents remain caller supplied.
pub type value_G_L4255 = u8;
pub const G_L4255: Global<value_G_L4255> = Global::new("FUN_140040890", Some(0x140040890));

/// Source symbol FUN_140002ca0. Contents remain caller supplied.
pub type value_G_L4256 = u8;
pub const G_L4256: Global<value_G_L4256> = Global::new("FUN_140002ca0", Some(0x140002ca0));

/// Source symbol FUN_1400c0b50. Contents remain caller supplied.
pub type value_G_L4257 = u8;
pub const G_L4257: Global<value_G_L4257> = Global::new("FUN_1400c0b50", Some(0x1400c0b50));

/// Source symbol FUN_1400c0b40. Contents remain caller supplied.
pub type value_G_L4258 = u8;
pub const G_L4258: Global<value_G_L4258> = Global::new("FUN_1400c0b40", Some(0x1400c0b40));

/// Source symbol DAT_14010de08. Contents remain caller supplied.
pub type value_G_L4259 = u64;
pub const G_L4259: Global<value_G_L4259> = Global::new("DAT_14010de08", Some(0x14010de08));

/// Source symbol DAT_1401d0c70. Contents remain caller supplied.
pub type value_G_L4260 = u32;
pub const G_L4260: Global<value_G_L4260> = Global::new("DAT_1401d0c70", Some(0x1401d0c70));

/// Source symbol DAT_140107fc0. Contents remain caller supplied.
pub type value_G_L4261 = u8;
pub const G_L4261: Global<value_G_L4261> = Global::new("DAT_140107fc0", Some(0x140107fc0));

/// Source symbol FUN_140040a10. Contents remain caller supplied.
pub type value_G_L4262 = u8;
pub const G_L4262: Global<value_G_L4262> = Global::new("FUN_140040a10", Some(0x140040a10));

/// Source symbol DAT_14010f0e0. Contents remain caller supplied.
pub type value_G_L4263 = u8;
pub const G_L4263: Global<value_G_L4263> = Global::new("DAT_14010f0e0", Some(0x14010f0e0));

/// Source symbol DAT_14010efe8. Contents remain caller supplied.
pub type value_G_L4264 = u8;
pub const G_L4264: Global<value_G_L4264> = Global::new("DAT_14010efe8", Some(0x14010efe8));

/// Source symbol DAT_1401cfd68. Contents remain caller supplied.
pub type value_G_L4265 = u8;
pub const G_L4265: Global<value_G_L4265> = Global::new("DAT_1401cfd68", Some(0x1401cfd68));

/// Source symbol DAT_1401cfd60. Contents remain caller supplied.
pub type value_G_L4266 = i64;
pub const G_L4266: Global<value_G_L4266> = Global::new("DAT_1401cfd60", Some(0x1401cfd60));

/// Source symbol DAT_1401cfd60. Contents remain caller supplied.
pub type value_G_L4267 = u64;
pub const G_L4267: Global<value_G_L4267> = Global::new("DAT_1401cfd60", Some(0x1401cfd60));

/// Source symbol DAT_14010f4f8. Contents remain caller supplied.
pub type value_G_L4268 = u8;
pub const G_L4268: Global<value_G_L4268> = Global::new("DAT_14010f4f8", Some(0x14010f4f8));

/// Source symbol DAT_1401cfd70. Contents remain caller supplied.
pub type value_G_L4269 = u8;
pub const G_L4269: Global<value_G_L4269> = Global::new("DAT_1401cfd70", Some(0x1401cfd70));

/// Source symbol DAT_1401d0cd8. Contents remain caller supplied.
pub type value_G_L4271 = u8;
pub const G_L4271: Global<value_G_L4271> = Global::new("DAT_1401d0cd8", Some(0x1401d0cd8));

/// Source symbol FUN_1400477f0. Contents remain caller supplied.
pub type value_G_L4272 = u8;
pub const G_L4272: Global<value_G_L4272> = Global::new("FUN_1400477f0", Some(0x1400477f0));

/// Source symbol FUN_140045e20. Contents remain caller supplied.
pub type value_G_L4273 = u8;
pub const G_L4273: Global<value_G_L4273> = Global::new("FUN_140045e20", Some(0x140045e20));

/// Source symbol DAT_1401d0ce0. Contents remain caller supplied.
pub type value_G_L4274 = u32;
pub const G_L4274: Global<value_G_L4274> = Global::new("DAT_1401d0ce0", Some(0x1401d0ce0));

/// Source symbol DAT_1401d0ce8. Contents remain caller supplied.
pub type value_G_L4275 = u8;
pub const G_L4275: Global<value_G_L4275> = Global::new("DAT_1401d0ce8", Some(0x1401d0ce8));

/// Source symbol DAT_1401d0cf0. Contents remain caller supplied.
pub type value_G_L4276 = u64;
pub const G_L4276: Global<value_G_L4276> = Global::new("DAT_1401d0cf0", Some(0x1401d0cf0));

/// Source symbol FUN_1400df7d0. Contents remain caller supplied.
pub type value_G_L4277 = u8;
pub const G_L4277: Global<value_G_L4277> = Global::new("FUN_1400df7d0", Some(0x1400df7d0));

/// Source symbol PTR_GetThisInterfaceMap_140110118. Contents remain caller supplied.
pub type value_G_L4278 = Address<u8>;
pub const G_L4278: Global<value_G_L4278> = Global::new("PTR_GetThisInterfaceMap_140110118", None);

/// Source symbol PTR_s_C_PrintDialogEx_14010fa10. Contents remain caller supplied.
pub type value_G_L4279 = Address<Unknown>;
pub const G_L4279: Global<value_G_L4279> = Global::new("PTR_s_C_PrintDialogEx_14010fa10", None);

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4280 = Address<i32>;
pub const G_L4280: Global<value_G_L4280> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol FUN_1400b7eb0. Contents remain caller supplied.
pub type value_G_L4281 = u8;
pub const G_L4281: Global<value_G_L4281> = Global::new("FUN_1400b7eb0", Some(0x1400b7eb0));

/// Source symbol FUN_1400b7ea0. Contents remain caller supplied.
pub type value_G_L4282 = u8;
pub const G_L4282: Global<value_G_L4282> = Global::new("FUN_1400b7ea0", Some(0x1400b7ea0));

/// Source symbol FUN_1400b80f0. Contents remain caller supplied.
pub type value_G_L4283 = u8;
pub const G_L4283: Global<value_G_L4283> = Global::new("FUN_1400b80f0", Some(0x1400b80f0));

/// Source symbol FUN_1400b80e0. Contents remain caller supplied.
pub type value_G_L4284 = u8;
pub const G_L4284: Global<value_G_L4284> = Global::new("FUN_1400b80e0", Some(0x1400b80e0));

/// Source symbol FUN_1400b8330. Contents remain caller supplied.
pub type value_G_L4285 = u8;
pub const G_L4285: Global<value_G_L4285> = Global::new("FUN_1400b8330", Some(0x1400b8330));

/// Source symbol FUN_1400b8320. Contents remain caller supplied.
pub type value_G_L4286 = u8;
pub const G_L4286: Global<value_G_L4286> = Global::new("FUN_1400b8320", Some(0x1400b8320));

/// Source symbol DAT_1401101d8. Contents remain caller supplied.
pub type value_G_L4287 = u8;
pub const G_L4287: Global<value_G_L4287> = Global::new("DAT_1401101d8", Some(0x1401101d8));

/// Source symbol DAT_140110178. Contents remain caller supplied.
pub type value_G_L4288 = u8;
pub const G_L4288: Global<value_G_L4288> = Global::new("DAT_140110178", Some(0x140110178));

/// Source symbol DAT_1401d0d00. Contents remain caller supplied.
pub type value_G_L4289 = u64;
pub const G_L4289: Global<value_G_L4289> = Global::new("DAT_1401d0d00", Some(0x1401d0d00));

/// Source symbol DAT_1401d0d10. Contents remain caller supplied.
pub type value_G_L4290 = u8;
pub const G_L4290: Global<value_G_L4290> = Global::new("DAT_1401d0d10", Some(0x1401d0d10));

/// Source symbol DAT_1401d0d18. Contents remain caller supplied.
pub type value_G_L4291 = u64;
pub const G_L4291: Global<value_G_L4291> = Global::new("DAT_1401d0d18", Some(0x1401d0d18));

/// Source symbol UNK_1401d0d08. Contents remain caller supplied.
pub type value_G_L4292 = u64;
pub const G_L4292: Global<value_G_L4292> = Global::new("UNK_1401d0d08", None);

/// Source symbol FUN_140049290. Contents remain caller supplied.
pub type value_G_L4293 = u8;
pub const G_L4293: Global<value_G_L4293> = Global::new("FUN_140049290", Some(0x140049290));

/// Source symbol DAT_1401d0cf8. Contents remain caller supplied.
pub type value_G_L4294 = u32;
pub const G_L4294: Global<value_G_L4294> = Global::new("DAT_1401d0cf8", Some(0x1401d0cf8));

/// Source symbol DAT_1401d0d18. Contents remain caller supplied.
pub type value_G_L4295 = u64;
pub const G_L4295: Global<value_G_L4295> = Global::new("DAT_1401d0d18", Some(0x1401d0d18));

/// Source symbol DAT_1401d0d10. Contents remain caller supplied.
pub type value_G_L4296 = u32;
pub const G_L4296: Global<value_G_L4296> = Global::new("DAT_1401d0d10", Some(0x1401d0d10));

/// Source symbol DAT_1401cfd88. Contents remain caller supplied.
pub type value_G_L4297 = u64;
pub const G_L4297: Global<value_G_L4297> = Global::new("DAT_1401cfd88", Some(0x1401cfd88));

/// Source symbol DAT_1401cfd80. Contents remain caller supplied.
pub type value_G_L4298 = i64;
pub const G_L4298: Global<value_G_L4298> = Global::new("DAT_1401cfd80", Some(0x1401cfd80));

/// Source symbol FUN_1400df7f0. Contents remain caller supplied.
pub type value_G_L4299 = u8;
pub const G_L4299: Global<value_G_L4299> = Global::new("FUN_1400df7f0", Some(0x1400df7f0));

/// Source symbol DAT_1401cfda8. Contents remain caller supplied.
pub type value_G_L4300 = u64;
pub const G_L4300: Global<value_G_L4300> = Global::new("DAT_1401cfda8", Some(0x1401cfda8));

/// Source symbol DAT_1401cfda0. Contents remain caller supplied.
pub type value_G_L4301 = i64;
pub const G_L4301: Global<value_G_L4301> = Global::new("DAT_1401cfda0", Some(0x1401cfda0));

/// Source symbol DAT_140110548. Contents remain caller supplied.
pub type value_G_L4302 = u8;
pub const G_L4302: Global<value_G_L4302> = Global::new("DAT_140110548", Some(0x140110548));

/// Source symbol DAT_00000034. Contents remain caller supplied.
pub type value_G_L4303 = u8;
pub const G_L4303: Global<value_G_L4303> = Global::new("DAT_00000034", None);

/// Source symbol DAT_140110590. Contents remain caller supplied.
pub type value_G_L4304 = u8;
pub const G_L4304: Global<value_G_L4304> = Global::new("DAT_140110590", Some(0x140110590));

/// Source symbol DAT_1401cfd80. Contents remain caller supplied.
pub type value_G_L4305 = u64;
pub const G_L4305: Global<value_G_L4305> = Global::new("DAT_1401cfd80", Some(0x1401cfd80));

/// Source symbol DAT_1401105c0. Contents remain caller supplied.
pub type value_G_L4306 = u8;
pub const G_L4306: Global<value_G_L4306> = Global::new("DAT_1401105c0", Some(0x1401105c0));

/// Source symbol DAT_1401cfd90. Contents remain caller supplied.
pub type value_G_L4307 = u8;
pub const G_L4307: Global<value_G_L4307> = Global::new("DAT_1401cfd90", Some(0x1401cfd90));

/// Source symbol DAT_1401cfda0. Contents remain caller supplied.
pub type value_G_L4308 = u64;
pub const G_L4308: Global<value_G_L4308> = Global::new("DAT_1401cfda0", Some(0x1401cfda0));

/// Source symbol DAT_140110630. Contents remain caller supplied.
pub type value_G_L4309 = u8;
pub const G_L4309: Global<value_G_L4309> = Global::new("DAT_140110630", Some(0x140110630));

/// Source symbol DAT_1401cfdb0. Contents remain caller supplied.
pub type value_G_L4310 = u8;
pub const G_L4310: Global<value_G_L4310> = Global::new("DAT_1401cfdb0", Some(0x1401cfdb0));

/// Source symbol FUN_1400b8570. Contents remain caller supplied.
pub type value_G_L4311 = u8;
pub const G_L4311: Global<value_G_L4311> = Global::new("FUN_1400b8570", Some(0x1400b8570));

/// Source symbol FUN_1400b8560. Contents remain caller supplied.
pub type value_G_L4312 = u8;
pub const G_L4312: Global<value_G_L4312> = Global::new("FUN_1400b8560", Some(0x1400b8560));

/// Source symbol DAT_140110990. Contents remain caller supplied.
pub type value_G_L4313 = u8;
pub const G_L4313: Global<value_G_L4313> = Global::new("DAT_140110990", Some(0x140110990));

/// Source symbol DAT_1401108e0. Contents remain caller supplied.
pub type value_G_L4314 = u8;
pub const G_L4314: Global<value_G_L4314> = Global::new("DAT_1401108e0", Some(0x1401108e0));

/// Source symbol DAT_1401d0d20. Contents remain caller supplied.
pub type value_G_L4315 = u32;
pub const G_L4315: Global<value_G_L4315> = Global::new("DAT_1401d0d20", Some(0x1401d0d20));

/// Source symbol DAT_1401d0d30. Contents remain caller supplied.
pub type value_G_L4316 = u8;
pub const G_L4316: Global<value_G_L4316> = Global::new("DAT_1401d0d30", Some(0x1401d0d30));

/// Source symbol DAT_1401d0e18. Contents remain caller supplied.
pub type value_G_L4317 = u8;
pub const G_L4317: Global<value_G_L4317> = Global::new("DAT_1401d0e18", Some(0x1401d0e18));

/// Source symbol DAT_1401d0e20. Contents remain caller supplied.
pub type value_G_L4318 = u8;
pub const G_L4318: Global<value_G_L4318> = Global::new("DAT_1401d0e20", Some(0x1401d0e20));

/// Source symbol FUN_1400df800. Contents remain caller supplied.
pub type value_G_L4320 = u8;
pub const G_L4320: Global<value_G_L4320> = Global::new("FUN_1400df800", Some(0x1400df800));

/// Source symbol PTR_GetThisMessageMap_1401111d8. Contents remain caller supplied.
pub type value_G_L4321 = Address<Unknown>;
pub const G_L4321: Global<value_G_L4321> = Global::new("PTR_GetThisMessageMap_1401111d8", None);

/// Source symbol PTR_s_CPaintPreviewView_140110e08. Contents remain caller supplied.
pub type value_G_L4323 = Address<Unknown>;
pub const G_L4323: Global<value_G_L4323> = Global::new("PTR_s_CPaintPreviewView_140110e08", None);

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4324 = i64;
pub const G_L4324: Global<value_G_L4324> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol PTR_`vftable'_1400e1010. Contents remain caller supplied.
pub type value_G_L4325 = Address<u8>;
pub const G_L4325: Global<value_G_L4325> = Global::new("PTR_`vftable'_1400e1010", None);

/// Source symbol DAT_14016aec0. Contents remain caller supplied.
pub type value_G_L4326 = u8;
pub const G_L4326: Global<value_G_L4326> = Global::new("DAT_14016aec0", Some(0x14016aec0));

/// Source symbol DAT_1401cfe68. Contents remain caller supplied.
pub type value_G_L4327 = u8;
pub const G_L4327: Global<value_G_L4327> = Global::new("DAT_1401cfe68", Some(0x1401cfe68));

/// Source symbol DAT_1401cfe60. Contents remain caller supplied.
pub type value_G_L4328 = u64;
pub const G_L4328: Global<value_G_L4328> = Global::new("DAT_1401cfe60", Some(0x1401cfe60));

/// Source symbol DAT_14010d540. Contents remain caller supplied.
pub type value_G_L4329 = u64;
pub const G_L4329: Global<value_G_L4329> = Global::new("DAT_14010d540", Some(0x14010d540));

/// Source symbol DAT_140111b30. Contents remain caller supplied.
pub type value_G_L4330 = u8;
pub const G_L4330: Global<value_G_L4330> = Global::new("DAT_140111b30", Some(0x140111b30));

/// Source symbol FUN_1400bba00. Contents remain caller supplied.
pub type value_G_L4331 = u8;
pub const G_L4331: Global<value_G_L4331> = Global::new("FUN_1400bba00", Some(0x1400bba00));

/// Source symbol FUN_1400b9360. Contents remain caller supplied.
pub type value_G_L4332 = u8;
pub const G_L4332: Global<value_G_L4332> = Global::new("FUN_1400b9360", Some(0x1400b9360));

/// Source symbol FUN_1400b9340. Contents remain caller supplied.
pub type value_G_L4333 = u8;
pub const G_L4333: Global<value_G_L4333> = Global::new("FUN_1400b9340", Some(0x1400b9340));

/// Source symbol FUN_1400bba10. Contents remain caller supplied.
pub type value_G_L4334 = u8;
pub const G_L4334: Global<value_G_L4334> = Global::new("FUN_1400bba10", Some(0x1400bba10));

/// Source symbol FUN_1400bc4b0. Contents remain caller supplied.
pub type value_G_L4335 = u8;
pub const G_L4335: Global<value_G_L4335> = Global::new("FUN_1400bc4b0", Some(0x1400bc4b0));

/// Source symbol FUN_1400bc4a0. Contents remain caller supplied.
pub type value_G_L4336 = u8;
pub const G_L4336: Global<value_G_L4336> = Global::new("FUN_1400bc4a0", Some(0x1400bc4a0));

/// Source symbol DAT_140111af8. Contents remain caller supplied.
pub type value_G_L4337 = u64;
pub const G_L4337: Global<value_G_L4337> = Global::new("DAT_140111af8", Some(0x140111af8));

/// Source symbol FUN_1400c30a0. Contents remain caller supplied.
pub type value_G_L4338 = u8;
pub const G_L4338: Global<value_G_L4338> = Global::new("FUN_1400c30a0", Some(0x1400c30a0));

/// Source symbol FUN_1400c37a0. Contents remain caller supplied.
pub type value_G_L4339 = u8;
pub const G_L4339: Global<value_G_L4339> = Global::new("FUN_1400c37a0", Some(0x1400c37a0));

/// Source symbol FUN_1400c3cd0. Contents remain caller supplied.
pub type value_G_L4340 = u8;
pub const G_L4340: Global<value_G_L4340> = Global::new("FUN_1400c3cd0", Some(0x1400c3cd0));

/// Source symbol FUN_1400c3cc0. Contents remain caller supplied.
pub type value_G_L4341 = u8;
pub const G_L4341: Global<value_G_L4341> = Global::new("FUN_1400c3cc0", Some(0x1400c3cc0));

/// Source symbol FUN_1400c3090. Contents remain caller supplied.
pub type value_G_L4342 = u8;
pub const G_L4342: Global<value_G_L4342> = Global::new("FUN_1400c3090", Some(0x1400c3090));

/// Source symbol FUN_1400c30b0. Contents remain caller supplied.
pub type value_G_L4343 = u8;
pub const G_L4343: Global<value_G_L4343> = Global::new("FUN_1400c30b0", Some(0x1400c30b0));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4344 = Address<f32>;
pub const G_L4344: Global<value_G_L4344> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401cfdc8. Contents remain caller supplied.
pub type value_G_L4345 = u8;
pub const G_L4345: Global<value_G_L4345> = Global::new("DAT_1401cfdc8", Some(0x1401cfdc8));

/// Source symbol DAT_1401cfdc0. Contents remain caller supplied.
pub type value_G_L4346 = i64;
pub const G_L4346: Global<value_G_L4346> = Global::new("DAT_1401cfdc0", Some(0x1401cfdc0));

/// Source symbol DAT_140111898. Contents remain caller supplied.
pub type value_G_L4347 = u8;
pub const G_L4347: Global<value_G_L4347> = Global::new("DAT_140111898", Some(0x140111898));

/// Source symbol DAT_1401cfe28. Contents remain caller supplied.
pub type value_G_L4348 = u8;
pub const G_L4348: Global<value_G_L4348> = Global::new("DAT_1401cfe28", Some(0x1401cfe28));

/// Source symbol DAT_1401cfe20. Contents remain caller supplied.
pub type value_G_L4349 = u64;
pub const G_L4349: Global<value_G_L4349> = Global::new("DAT_1401cfe20", Some(0x1401cfe20));

/// Source symbol DAT_1401cfe20. Contents remain caller supplied.
pub type value_G_L4350 = i64;
pub const G_L4350: Global<value_G_L4350> = Global::new("DAT_1401cfe20", Some(0x1401cfe20));

/// Source symbol DAT_140111ae8. Contents remain caller supplied.
pub type value_G_L4351 = u8;
pub const G_L4351: Global<value_G_L4351> = Global::new("DAT_140111ae8", Some(0x140111ae8));

/// Source symbol DAT_140179158. Contents remain caller supplied.
pub type value_G_L4352 = u32;
pub const G_L4352: Global<value_G_L4352> = Global::new("DAT_140179158", Some(0x140179158));

/// Source symbol FUN_1400bab90. Contents remain caller supplied.
pub type value_G_L4353 = u8;
pub const G_L4353: Global<value_G_L4353> = Global::new("FUN_1400bab90", Some(0x1400bab90));

/// Source symbol FUN_1400baba0. Contents remain caller supplied.
pub type value_G_L4354 = u8;
pub const G_L4354: Global<value_G_L4354> = Global::new("FUN_1400baba0", Some(0x1400baba0));

/// Source symbol FUN_1400c0dc0. Contents remain caller supplied.
pub type value_G_L4355 = u8;
pub const G_L4355: Global<value_G_L4355> = Global::new("FUN_1400c0dc0", Some(0x1400c0dc0));

/// Source symbol FUN_1400c0db0. Contents remain caller supplied.
pub type value_G_L4356 = u8;
pub const G_L4356: Global<value_G_L4356> = Global::new("FUN_1400c0db0", Some(0x1400c0db0));

/// Source symbol DAT_1401cfe08. Contents remain caller supplied.
pub type value_G_L4357 = u8;
pub const G_L4357: Global<value_G_L4357> = Global::new("DAT_1401cfe08", Some(0x1401cfe08));

/// Source symbol DAT_1401cfe00. Contents remain caller supplied.
pub type value_G_L4358 = u64;
pub const G_L4358: Global<value_G_L4358> = Global::new("DAT_1401cfe00", Some(0x1401cfe00));

/// Source symbol DAT_1401cfde8. Contents remain caller supplied.
pub type value_G_L4359 = u8;
pub const G_L4359: Global<value_G_L4359> = Global::new("DAT_1401cfde8", Some(0x1401cfde8));

/// Source symbol DAT_1401cfde0. Contents remain caller supplied.
pub type value_G_L4360 = i64;
pub const G_L4360: Global<value_G_L4360> = Global::new("DAT_1401cfde0", Some(0x1401cfde0));

/// Source symbol DAT_140111930. Contents remain caller supplied.
pub type value_G_L4361 = u8;
pub const G_L4361: Global<value_G_L4361> = Global::new("DAT_140111930", Some(0x140111930));

/// Source symbol FUN_14005bc50. Contents remain caller supplied.
pub type value_G_L4362 = u8;
pub const G_L4362: Global<value_G_L4362> = Global::new("FUN_14005bc50", Some(0x14005bc50));

/// Source symbol FUN_1400bc8a0. Contents remain caller supplied.
pub type value_G_L4363 = u8;
pub const G_L4363: Global<value_G_L4363> = Global::new("FUN_1400bc8a0", Some(0x1400bc8a0));

/// Source symbol FUN_1400bc890. Contents remain caller supplied.
pub type value_G_L4364 = u8;
pub const G_L4364: Global<value_G_L4364> = Global::new("FUN_1400bc890", Some(0x1400bc890));

/// Source symbol FUN_1400bff10. Contents remain caller supplied.
pub type value_G_L4365 = u8;
pub const G_L4365: Global<value_G_L4365> = Global::new("FUN_1400bff10", Some(0x1400bff10));

/// Source symbol FUN_1400bff00. Contents remain caller supplied.
pub type value_G_L4366 = u8;
pub const G_L4366: Global<value_G_L4366> = Global::new("FUN_1400bff00", Some(0x1400bff00));

/// Source symbol FUN_1400c0260. Contents remain caller supplied.
pub type value_G_L4367 = u8;
pub const G_L4367: Global<value_G_L4367> = Global::new("FUN_1400c0260", Some(0x1400c0260));

/// Source symbol FUN_1400c0250. Contents remain caller supplied.
pub type value_G_L4368 = u8;
pub const G_L4368: Global<value_G_L4368> = Global::new("FUN_1400c0250", Some(0x1400c0250));

/// Source symbol FUN_1400b9b60. Contents remain caller supplied.
pub type value_G_L4369 = u8;
pub const G_L4369: Global<value_G_L4369> = Global::new("FUN_1400b9b60", Some(0x1400b9b60));

/// Source symbol FUN_1400b9350. Contents remain caller supplied.
pub type value_G_L4370 = u8;
pub const G_L4370: Global<value_G_L4370> = Global::new("FUN_1400b9350", Some(0x1400b9350));

/// Source symbol DAT_140111c48. Contents remain caller supplied.
pub type value_G_L4371 = u8;
pub const G_L4371: Global<value_G_L4371> = Global::new("DAT_140111c48", Some(0x140111c48));

/// Source symbol DAT_140111c18. Contents remain caller supplied.
pub type value_G_L4372 = u8;
pub const G_L4372: Global<value_G_L4372> = Global::new("DAT_140111c18", Some(0x140111c18));

/// Source symbol DAT_140111bf8. Contents remain caller supplied.
pub type value_G_L4373 = u8;
pub const G_L4373: Global<value_G_L4373> = Global::new("DAT_140111bf8", Some(0x140111bf8));

/// Source symbol DAT_140111ac8. Contents remain caller supplied.
pub type value_G_L4374 = u8;
pub const G_L4374: Global<value_G_L4374> = Global::new("DAT_140111ac8", Some(0x140111ac8));

/// Source symbol DAT_1401118a8. Contents remain caller supplied.
pub type value_G_L4375 = u8;
pub const G_L4375: Global<value_G_L4375> = Global::new("DAT_1401118a8", Some(0x1401118a8));

/// Source symbol DAT_140111710. Contents remain caller supplied.
pub type value_G_L4376 = u8;
pub const G_L4376: Global<value_G_L4376> = Global::new("DAT_140111710", Some(0x140111710));

/// Source symbol DAT_1401116d8. Contents remain caller supplied.
pub type value_G_L4377 = u8;
pub const G_L4377: Global<value_G_L4377> = Global::new("DAT_1401116d8", Some(0x1401116d8));

/// Source symbol DAT_1401118b8. Contents remain caller supplied.
pub type value_G_L4378 = u8;
pub const G_L4378: Global<value_G_L4378> = Global::new("DAT_1401118b8", Some(0x1401118b8));

/// Source symbol DAT_1401cfdc0. Contents remain caller supplied.
pub type value_G_L4379 = u64;
pub const G_L4379: Global<value_G_L4379> = Global::new("DAT_1401cfdc0", Some(0x1401cfdc0));

/// Source symbol DAT_140111b88. Contents remain caller supplied.
pub type value_G_L4380 = u8;
pub const G_L4380: Global<value_G_L4380> = Global::new("DAT_140111b88", Some(0x140111b88));

/// Source symbol DAT_1401cfdd0. Contents remain caller supplied.
pub type value_G_L4381 = u8;
pub const G_L4381: Global<value_G_L4381> = Global::new("DAT_1401cfdd0", Some(0x1401cfdd0));

/// Source symbol DAT_1401cfde0. Contents remain caller supplied.
pub type value_G_L4382 = u64;
pub const G_L4382: Global<value_G_L4382> = Global::new("DAT_1401cfde0", Some(0x1401cfde0));

/// Source symbol DAT_140112028. Contents remain caller supplied.
pub type value_G_L4383 = u8;
pub const G_L4383: Global<value_G_L4383> = Global::new("DAT_140112028", Some(0x140112028));

/// Source symbol DAT_1401cfdf0. Contents remain caller supplied.
pub type value_G_L4384 = u8;
pub const G_L4384: Global<value_G_L4384> = Global::new("DAT_1401cfdf0", Some(0x1401cfdf0));

/// Source symbol DAT_1401cfe10. Contents remain caller supplied.
pub type value_G_L4385 = u8;
pub const G_L4385: Global<value_G_L4385> = Global::new("DAT_1401cfe10", Some(0x1401cfe10));

/// Source symbol DAT_140112130. Contents remain caller supplied.
pub type value_G_L4386 = u8;
pub const G_L4386: Global<value_G_L4386> = Global::new("DAT_140112130", Some(0x140112130));

/// Source symbol DAT_1401cfe20. Contents remain caller supplied.
pub type value_G_L4387 = u64;
pub const G_L4387: Global<value_G_L4387> = Global::new("DAT_1401cfe20", Some(0x1401cfe20));

/// Source symbol DAT_140112570. Contents remain caller supplied.
pub type value_G_L4388 = u8;
pub const G_L4388: Global<value_G_L4388> = Global::new("DAT_140112570", Some(0x140112570));

/// Source symbol DAT_1401cfe30. Contents remain caller supplied.
pub type value_G_L4389 = u8;
pub const G_L4389: Global<value_G_L4389> = Global::new("DAT_1401cfe30", Some(0x1401cfe30));

/// Source symbol DAT_1401cfe48. Contents remain caller supplied.
pub type value_G_L4390 = u8;
pub const G_L4390: Global<value_G_L4390> = Global::new("DAT_1401cfe48", Some(0x1401cfe48));

/// Source symbol DAT_1401cfe40. Contents remain caller supplied.
pub type value_G_L4391 = u64;
pub const G_L4391: Global<value_G_L4391> = Global::new("DAT_1401cfe40", Some(0x1401cfe40));

/// Source symbol DAT_140112960. Contents remain caller supplied.
pub type value_G_L4392 = u8;
pub const G_L4392: Global<value_G_L4392> = Global::new("DAT_140112960", Some(0x140112960));

/// Source symbol DAT_1401cfe50. Contents remain caller supplied.
pub type value_G_L4393 = u8;
pub const G_L4393: Global<value_G_L4393> = Global::new("DAT_1401cfe50", Some(0x1401cfe50));

/// Source symbol DAT_1401cfe40. Contents remain caller supplied.
pub type value_G_L4394 = i64;
pub const G_L4394: Global<value_G_L4394> = Global::new("DAT_1401cfe40", Some(0x1401cfe40));

/// Source symbol FUN_14005c340. Contents remain caller supplied.
pub type value_G_L4395 = u8;
pub const G_L4395: Global<value_G_L4395> = Global::new("FUN_14005c340", Some(0x14005c340));

/// Source symbol DAT_140112c40. Contents remain caller supplied.
pub type value_G_L4396 = u8;
pub const G_L4396: Global<value_G_L4396> = Global::new("DAT_140112c40", Some(0x140112c40));

/// Source symbol DAT_1401cfe70. Contents remain caller supplied.
pub type value_G_L4397 = u8;
pub const G_L4397: Global<value_G_L4397> = Global::new("DAT_1401cfe70", Some(0x1401cfe70));

/// Source symbol PTR_s_CPBDoc_140115050. Contents remain caller supplied.
pub type value_G_L4399 = Address<Unknown>;
pub const G_L4399: Global<value_G_L4399> = Global::new("PTR_s_CPBDoc_140115050", None);

/// Source symbol PTR_GetThisMessageMap_140115040. Contents remain caller supplied.
pub type value_G_L4400 = Address<Unknown>;
pub const G_L4400: Global<value_G_L4400> = Global::new("PTR_GetThisMessageMap_140115040", None);

/// Source symbol PTR_GetThisInterfaceMap_140114dc8. Contents remain caller supplied.
pub type value_G_L4401 = Address<u8>;
pub const G_L4401: Global<value_G_L4401> = Global::new("PTR_GetThisInterfaceMap_140114dc8", None);

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4402 = Address<Address<u32>>;
pub const G_L4402: Global<value_G_L4402> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol FUN_1400c4060. Contents remain caller supplied.
pub type value_G_L4403 = u8;
pub const G_L4403: Global<value_G_L4403> = Global::new("FUN_1400c4060", Some(0x1400c4060));

/// Source symbol FUN_1400c4050. Contents remain caller supplied.
pub type value_G_L4404 = u8;
pub const G_L4404: Global<value_G_L4404> = Global::new("FUN_1400c4050", Some(0x1400c4050));

/// Source symbol DAT_1401d085b. Contents remain caller supplied.
pub type value_G_L4405 = u8;
pub const G_L4405: Global<value_G_L4405> = Global::new("DAT_1401d085b", Some(0x1401d085b));

/// Source symbol DAT_1401d0934. Contents remain caller supplied.
pub type value_G_L4406 = u16;
pub const G_L4406: Global<value_G_L4406> = Global::new("DAT_1401d0934", Some(0x1401d0934));

/// Source symbol DAT_1401d0c18. Contents remain caller supplied.
pub type value_G_L4407 = Address<i64>;
pub const G_L4407: Global<value_G_L4407> = Global::new("DAT_1401d0c18", Some(0x1401d0c18));

/// Source symbol DAT_1401d0c20. Contents remain caller supplied.
pub type value_G_L4408 = Address<i64>;
pub const G_L4408: Global<value_G_L4408> = Global::new("DAT_1401d0c20", Some(0x1401d0c20));

/// Source symbol DAT_1401d0c10. Contents remain caller supplied.
pub type value_G_L4409 = u8;
pub const G_L4409: Global<value_G_L4409> = Global::new("DAT_1401d0c10", Some(0x1401d0c10));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4410 = Address<Address<u64>>;
pub const G_L4410: Global<value_G_L4410> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1400e4c50. Contents remain caller supplied.
pub type value_G_L4412 = u64;
pub const G_L4412: Global<value_G_L4412> = Global::new("DAT_1400e4c50", Some(0x1400e4c50));

/// Source symbol DAT_140114638. Contents remain caller supplied.
pub type value_G_L4413 = u8;
pub const G_L4413: Global<value_G_L4413> = Global::new("DAT_140114638", Some(0x140114638));

/// Source symbol DAT_140108018. Contents remain caller supplied.
pub type value_G_L4414 = u64;
pub const G_L4414: Global<value_G_L4414> = Global::new("DAT_140108018", Some(0x140108018));

/// Source symbol DAT_1401d0ba8. Contents remain caller supplied.
pub type value_G_L4415 = i64;
pub const G_L4415: Global<value_G_L4415> = Global::new("DAT_1401d0ba8", Some(0x1401d0ba8));

/// Source symbol DAT_1401d0ba0. Contents remain caller supplied.
pub type value_G_L4416 = Address<Address<u8>>;
pub const G_L4416: Global<value_G_L4416> = Global::new("DAT_1401d0ba0", Some(0x1401d0ba0));

/// Source symbol DAT_1401d0ba0. Contents remain caller supplied.
pub type value_G_L4417 = Address<t_8bd0dddcee10>;
pub const G_L4417: Global<value_G_L4417> = Global::new("DAT_1401d0ba0", Some(0x1401d0ba0));

/// Source symbol DAT_1401d07b0. Contents remain caller supplied.
pub type value_G_L4418 = i64;
pub const G_L4418: Global<value_G_L4418> = Global::new("DAT_1401d07b0", Some(0x1401d07b0));

/// Source symbol DAT_1401067f8. Contents remain caller supplied.
pub type value_G_L4419 = u8;
pub const G_L4419: Global<value_G_L4419> = Global::new("DAT_1401067f8", Some(0x1401067f8));

/// Source symbol FUN_140062af0. Contents remain caller supplied.
pub type value_G_L4420 = u8;
pub const G_L4420: Global<value_G_L4420> = Global::new("FUN_140062af0", Some(0x140062af0));

/// Source symbol FUN_140062de0. Contents remain caller supplied.
pub type value_G_L4421 = u8;
pub const G_L4421: Global<value_G_L4421> = Global::new("FUN_140062de0", Some(0x140062de0));

/// Source symbol FUN_140002130. Contents remain caller supplied.
pub type value_G_L4422 = u8;
pub const G_L4422: Global<value_G_L4422> = Global::new("FUN_140002130", Some(0x140002130));

/// Source symbol DAT_1401cfec8. Contents remain caller supplied.
pub type value_G_L4423 = u8;
pub const G_L4423: Global<value_G_L4423> = Global::new("DAT_1401cfec8", Some(0x1401cfec8));

/// Source symbol DAT_1401cfec0. Contents remain caller supplied.
pub type value_G_L4424 = i64;
pub const G_L4424: Global<value_G_L4424> = Global::new("DAT_1401cfec0", Some(0x1401cfec0));

/// Source symbol FUN_1400c4960. Contents remain caller supplied.
pub type value_G_L4425 = u8;
pub const G_L4425: Global<value_G_L4425> = Global::new("FUN_1400c4960", Some(0x1400c4960));

/// Source symbol FUN_1400c4950. Contents remain caller supplied.
pub type value_G_L4426 = u8;
pub const G_L4426: Global<value_G_L4426> = Global::new("FUN_1400c4950", Some(0x1400c4950));

/// Source symbol FUN_140062680. Contents remain caller supplied.
pub type value_G_L4427 = u8;
pub const G_L4427: Global<value_G_L4427> = Global::new("FUN_140062680", Some(0x140062680));

/// Source symbol DAT_1401cfe88. Contents remain caller supplied.
pub type value_G_L4428 = u8;
pub const G_L4428: Global<value_G_L4428> = Global::new("DAT_1401cfe88", Some(0x1401cfe88));

/// Source symbol DAT_1401cfe80. Contents remain caller supplied.
pub type value_G_L4429 = i64;
pub const G_L4429: Global<value_G_L4429> = Global::new("DAT_1401cfe80", Some(0x1401cfe80));

/// Source symbol DAT_1401145c8. Contents remain caller supplied.
pub type value_G_L4430 = u8;
pub const G_L4430: Global<value_G_L4430> = Global::new("DAT_1401145c8", Some(0x1401145c8));

/// Source symbol DAT_1401cfe80. Contents remain caller supplied.
pub type value_G_L4431 = u64;
pub const G_L4431: Global<value_G_L4431> = Global::new("DAT_1401cfe80", Some(0x1401cfe80));

/// Source symbol DAT_140114500. Contents remain caller supplied.
pub type value_G_L4432 = u8;
pub const G_L4432: Global<value_G_L4432> = Global::new("DAT_140114500", Some(0x140114500));

/// Source symbol DAT_1401cfe90. Contents remain caller supplied.
pub type value_G_L4433 = u8;
pub const G_L4433: Global<value_G_L4433> = Global::new("DAT_1401cfe90", Some(0x1401cfe90));

/// Source symbol DAT_1401d0e58. Contents remain caller supplied.
pub type value_G_L4434 = Address<u64>;
pub const G_L4434: Global<value_G_L4434> = Global::new("DAT_1401d0e58", Some(0x1401d0e58));

/// Source symbol DAT_1401d0e80. Contents remain caller supplied.
pub type value_G_L4435 = u64;
pub const G_L4435: Global<value_G_L4435> = Global::new("DAT_1401d0e80", Some(0x1401d0e80));

/// Source symbol DAT_1401d0e68. Contents remain caller supplied.
pub type value_G_L4436 = i64;
pub const G_L4436: Global<value_G_L4436> = Global::new("DAT_1401d0e68", Some(0x1401d0e68));

/// Source symbol DAT_1401d0e60. Contents remain caller supplied.
pub type value_G_L4437 = i64;
pub const G_L4437: Global<value_G_L4437> = Global::new("DAT_1401d0e60", Some(0x1401d0e60));

/// Source symbol DAT_1401d0e88. Contents remain caller supplied.
pub type value_G_L4438 = u64;
pub const G_L4438: Global<value_G_L4438> = Global::new("DAT_1401d0e88", Some(0x1401d0e88));

/// Source symbol DAT_1401d0e50. Contents remain caller supplied.
pub type value_G_L4439 = f32;
pub const G_L4439: Global<value_G_L4439> = Global::new("DAT_1401d0e50", Some(0x1401d0e50));

/// Source symbol DAT_1401d0e58. Contents remain caller supplied.
pub type value_G_L4440 = u64;
pub const G_L4440: Global<value_G_L4440> = Global::new("DAT_1401d0e58", Some(0x1401d0e58));

/// Source symbol DAT_140114558. Contents remain caller supplied.
pub type value_G_L4441 = u8;
pub const G_L4441: Global<value_G_L4441> = Global::new("DAT_140114558", Some(0x140114558));

/// Source symbol DAT_140114440. Contents remain caller supplied.
pub type value_G_L4442 = u8;
pub const G_L4442: Global<value_G_L4442> = Global::new("DAT_140114440", Some(0x140114440));

/// Source symbol DAT_1401140e8. Contents remain caller supplied.
pub type value_G_L4443 = u8;
pub const G_L4443: Global<value_G_L4443> = Global::new("DAT_1401140e8", Some(0x1401140e8));

/// Source symbol DAT_1401140f8. Contents remain caller supplied.
pub type value_G_L4444 = u8;
pub const G_L4444: Global<value_G_L4444> = Global::new("DAT_1401140f8", Some(0x1401140f8));

/// Source symbol DAT_1401d0e88. Contents remain caller supplied.
pub type value_G_L4445 = i64;
pub const G_L4445: Global<value_G_L4445> = Global::new("DAT_1401d0e88", Some(0x1401d0e88));

/// Source symbol DAT_1401d0e68. Contents remain caller supplied.
pub type value_G_L4446 = u64;
pub const G_L4446: Global<value_G_L4446> = Global::new("DAT_1401d0e68", Some(0x1401d0e68));

/// Source symbol DAT_140114060. Contents remain caller supplied.
pub type value_G_L4447 = u8;
pub const G_L4447: Global<value_G_L4447> = Global::new("DAT_140114060", Some(0x140114060));

/// Source symbol DAT_140114050. Contents remain caller supplied.
pub type value_G_L4448 = u8;
pub const G_L4448: Global<value_G_L4448> = Global::new("DAT_140114050", Some(0x140114050));

/// Source symbol DAT_140114040. Contents remain caller supplied.
pub type value_G_L4449 = u8;
pub const G_L4449: Global<value_G_L4449> = Global::new("DAT_140114040", Some(0x140114040));

/// Source symbol DAT_1401cfea8. Contents remain caller supplied.
pub type value_G_L4451 = u8;
pub const G_L4451: Global<value_G_L4451> = Global::new("DAT_1401cfea8", Some(0x1401cfea8));

/// Source symbol DAT_1401cfea0. Contents remain caller supplied.
pub type value_G_L4452 = u64;
pub const G_L4452: Global<value_G_L4452> = Global::new("DAT_1401cfea0", Some(0x1401cfea0));

/// Source symbol DAT_1401146f8. Contents remain caller supplied.
pub type value_G_L4453 = u8;
pub const G_L4453: Global<value_G_L4453> = Global::new("DAT_1401146f8", Some(0x1401146f8));

/// Source symbol DAT_1401cfeb0. Contents remain caller supplied.
pub type value_G_L4454 = u8;
pub const G_L4454: Global<value_G_L4454> = Global::new("DAT_1401cfeb0", Some(0x1401cfeb0));

/// Source symbol DAT_1401cfec0. Contents remain caller supplied.
pub type value_G_L4455 = u64;
pub const G_L4455: Global<value_G_L4455> = Global::new("DAT_1401cfec0", Some(0x1401cfec0));

/// Source symbol DAT_140114768. Contents remain caller supplied.
pub type value_G_L4456 = u8;
pub const G_L4456: Global<value_G_L4456> = Global::new("DAT_140114768", Some(0x140114768));

/// Source symbol DAT_1401cfed0. Contents remain caller supplied.
pub type value_G_L4457 = u8;
pub const G_L4457: Global<value_G_L4457> = Global::new("DAT_1401cfed0", Some(0x1401cfed0));

/// Source symbol DAT_1401cfee8. Contents remain caller supplied.
pub type value_G_L4458 = u8;
pub const G_L4458: Global<value_G_L4458> = Global::new("DAT_1401cfee8", Some(0x1401cfee8));

/// Source symbol DAT_1401cfee0. Contents remain caller supplied.
pub type value_G_L4459 = i64;
pub const G_L4459: Global<value_G_L4459> = Global::new("DAT_1401cfee0", Some(0x1401cfee0));

/// Source symbol FUN_140066aa0. Contents remain caller supplied.
pub type value_G_L4460 = u8;
pub const G_L4460: Global<value_G_L4460> = Global::new("FUN_140066aa0", Some(0x140066aa0));

/// Source symbol DAT_1401cfee0. Contents remain caller supplied.
pub type value_G_L4461 = u64;
pub const G_L4461: Global<value_G_L4461> = Global::new("DAT_1401cfee0", Some(0x1401cfee0));

/// Source symbol DAT_140114848. Contents remain caller supplied.
pub type value_G_L4462 = u8;
pub const G_L4462: Global<value_G_L4462> = Global::new("DAT_140114848", Some(0x140114848));

/// Source symbol DAT_1401cfef0. Contents remain caller supplied.
pub type value_G_L4463 = u8;
pub const G_L4463: Global<value_G_L4463> = Global::new("DAT_1401cfef0", Some(0x1401cfef0));

/// Source symbol FUN_140066d80. Contents remain caller supplied.
pub type value_G_L4464 = u8;
pub const G_L4464: Global<value_G_L4464> = Global::new("FUN_140066d80", Some(0x140066d80));

/// Source symbol FUN_1400c6a30. Contents remain caller supplied.
pub type value_G_L4465 = u8;
pub const G_L4465: Global<value_G_L4465> = Global::new("FUN_1400c6a30", Some(0x1400c6a30));

/// Source symbol FUN_1400c6a20. Contents remain caller supplied.
pub type value_G_L4466 = u8;
pub const G_L4466: Global<value_G_L4466> = Global::new("FUN_1400c6a20", Some(0x1400c6a20));

/// Source symbol PTR_s_CPBFrame_1401158b0. Contents remain caller supplied.
pub type value_G_L4468 = Address<Unknown>;
pub const G_L4468: Global<value_G_L4468> = Global::new("PTR_s_CPBFrame_1401158b0", None);

/// Source symbol PTR_GetThisMessageMap_1401158a0. Contents remain caller supplied.
pub type value_G_L4469 = Address<Unknown>;
pub const G_L4469: Global<value_G_L4469> = Global::new("PTR_GetThisMessageMap_1401158a0", None);

/// Source symbol DAT_1401d08e0. Contents remain caller supplied.
pub type value_G_L4470 = u8;
pub const G_L4470: Global<value_G_L4470> = Global::new("DAT_1401d08e0", Some(0x1401d08e0));

/// Source symbol DAT_1401d08e4. Contents remain caller supplied.
pub type value_G_L4472 = u8;
pub const G_L4472: Global<value_G_L4472> = Global::new("DAT_1401d08e4", Some(0x1401d08e4));

/// Source symbol DAT_1401d08ec. Contents remain caller supplied.
pub type value_G_L4473 = u8;
pub const G_L4473: Global<value_G_L4473> = Global::new("DAT_1401d08ec", Some(0x1401d08ec));

/// Source symbol UNK_1401d08e8. Contents remain caller supplied.
pub type value_G_L4474 = i32;
pub const G_L4474: Global<value_G_L4474> = Global::new("UNK_1401d08e8", None);

/// Source symbol DAT_140109788. Contents remain caller supplied.
pub type value_G_L4475 = u8;
pub const G_L4475: Global<value_G_L4475> = Global::new("DAT_140109788", Some(0x140109788));

/// Source symbol DAT_140110b00. Contents remain caller supplied.
pub type value_G_L4476 = u8;
pub const G_L4476: Global<value_G_L4476> = Global::new("DAT_140110b00", Some(0x140110b00));

/// Source symbol DAT_1401d08c4. Contents remain caller supplied.
pub type value_G_L4477 = u8;
pub const G_L4477: Global<value_G_L4477> = Global::new("DAT_1401d08c4", Some(0x1401d08c4));

/// Source symbol DAT_1401d05f8. Contents remain caller supplied.
pub type value_G_L4478 = u64;
pub const G_L4478: Global<value_G_L4478> = Global::new("DAT_1401d05f8", Some(0x1401d05f8));

/// Source symbol DAT_ffffffff80000001. Contents remain caller supplied.
pub type value_G_L4479 = u8;
pub const G_L4479: Global<value_G_L4479> =
    Global::new("DAT_ffffffff80000001", Some(0xffffffff80000001));

/// Source symbol DAT_1401d085d. Contents remain caller supplied.
pub type value_G_L4480 = u8;
pub const G_L4480: Global<value_G_L4480> = Global::new("DAT_1401d085d", Some(0x1401d085d));

/// Source symbol DAT_1401d0720. Contents remain caller supplied.
pub type value_G_L4481 = Address<i64>;
pub const G_L4481: Global<value_G_L4481> = Global::new("DAT_1401d0720", Some(0x1401d0720));

/// Source symbol DAT_1401d085e. Contents remain caller supplied.
pub type value_G_L4482 = i8;
pub const G_L4482: Global<value_G_L4482> = Global::new("DAT_1401d085e", Some(0x1401d085e));

/// Source symbol DAT_1401d07c6. Contents remain caller supplied.
pub type value_G_L4483 = i8;
pub const G_L4483: Global<value_G_L4483> = Global::new("DAT_1401d07c6", Some(0x1401d07c6));

/// Source symbol DAT_1401d08cc. Contents remain caller supplied.
pub type value_G_L4484 = u32;
pub const G_L4484: Global<value_G_L4484> = Global::new("DAT_1401d08cc", Some(0x1401d08cc));

/// Source symbol DAT_1401d07a8. Contents remain caller supplied.
pub type value_G_L4485 = i64;
pub const G_L4485: Global<value_G_L4485> = Global::new("DAT_1401d07a8", Some(0x1401d07a8));

/// Source symbol DAT_1401d0560. Contents remain caller supplied.
pub type value_G_L4486 = u64;
pub const G_L4486: Global<value_G_L4486> = Global::new("DAT_1401d0560", Some(0x1401d0560));

/// Source symbol FUN_1400c6490. Contents remain caller supplied.
pub type value_G_L4487 = u8;
pub const G_L4487: Global<value_G_L4487> = Global::new("FUN_1400c6490", Some(0x1400c6490));

/// Source symbol FUN_1400c6480. Contents remain caller supplied.
pub type value_G_L4488 = u8;
pub const G_L4488: Global<value_G_L4488> = Global::new("FUN_1400c6480", Some(0x1400c6480));

/// Source symbol DAT_1401d08a8. Contents remain caller supplied.
pub type value_G_L4489 = i32;
pub const G_L4489: Global<value_G_L4489> = Global::new("DAT_1401d08a8", Some(0x1401d08a8));

/// Source symbol DAT_1401d08ac. Contents remain caller supplied.
pub type value_G_L4490 = i32;
pub const G_L4490: Global<value_G_L4490> = Global::new("DAT_1401d08ac", Some(0x1401d08ac));

/// Source symbol DAT_1401d08b0. Contents remain caller supplied.
pub type value_G_L4491 = i32;
pub const G_L4491: Global<value_G_L4491> = Global::new("DAT_1401d08b0", Some(0x1401d08b0));

/// Source symbol DAT_1401d08b4. Contents remain caller supplied.
pub type value_G_L4492 = u32;
pub const G_L4492: Global<value_G_L4492> = Global::new("DAT_1401d08b4", Some(0x1401d08b4));

/// Source symbol DAT_1401d08b8. Contents remain caller supplied.
pub type value_G_L4493 = i32;
pub const G_L4493: Global<value_G_L4493> = Global::new("DAT_1401d08b8", Some(0x1401d08b8));

/// Source symbol FUN_1400863a0. Contents remain caller supplied.
pub type value_G_L4494 = u8;
pub const G_L4494: Global<value_G_L4494> = Global::new("FUN_1400863a0", Some(0x1400863a0));

/// Source symbol DAT_1401d08bc. Contents remain caller supplied.
pub type value_G_L4495 = i32;
pub const G_L4495: Global<value_G_L4495> = Global::new("DAT_1401d08bc", Some(0x1401d08bc));

/// Source symbol FUN_1400884c0. Contents remain caller supplied.
pub type value_G_L4496 = u8;
pub const G_L4496: Global<value_G_L4496> = Global::new("FUN_1400884c0", Some(0x1400884c0));

/// Source symbol DAT_1401d0898. Contents remain caller supplied.
pub type value_G_L4497 = u32;
pub const G_L4497: Global<value_G_L4497> = Global::new("DAT_1401d0898", Some(0x1401d0898));

/// Source symbol DAT_1401d089c. Contents remain caller supplied.
pub type value_G_L4498 = i32;
pub const G_L4498: Global<value_G_L4498> = Global::new("DAT_1401d089c", Some(0x1401d089c));

/// Source symbol DAT_1401d08a0. Contents remain caller supplied.
pub type value_G_L4499 = i32;
pub const G_L4499: Global<value_G_L4499> = Global::new("DAT_1401d08a0", Some(0x1401d08a0));

/// Source symbol DAT_1401d08a4. Contents remain caller supplied.
pub type value_G_L4500 = i32;
pub const G_L4500: Global<value_G_L4500> = Global::new("DAT_1401d08a4", Some(0x1401d08a4));

/// Source symbol DAT_1401cff08. Contents remain caller supplied.
pub type value_G_L4501 = u8;
pub const G_L4501: Global<value_G_L4501> = Global::new("DAT_1401cff08", Some(0x1401cff08));

/// Source symbol DAT_1401cff00. Contents remain caller supplied.
pub type value_G_L4502 = u64;
pub const G_L4502: Global<value_G_L4502> = Global::new("DAT_1401cff00", Some(0x1401cff00));

/// Source symbol PTR_rectDefault_1400e1a28. Contents remain caller supplied.
pub type value_G_L4503 = Address<u8>;
pub const G_L4503: Global<value_G_L4503> = Global::new("PTR_rectDefault_1400e1a28", None);

/// Source symbol DAT_140115220. Contents remain caller supplied.
pub type value_G_L4504 = u8;
pub const G_L4504: Global<value_G_L4504> = Global::new("DAT_140115220", Some(0x140115220));

/// Source symbol DAT_1401151c8. Contents remain caller supplied.
pub type value_G_L4505 = u8;
pub const G_L4505: Global<value_G_L4505> = Global::new("DAT_1401151c8", Some(0x1401151c8));

/// Source symbol DAT_1401151d8. Contents remain caller supplied.
pub type value_G_L4506 = u8;
pub const G_L4506: Global<value_G_L4506> = Global::new("DAT_1401151d8", Some(0x1401151d8));

/// Source symbol DAT_1401151b8. Contents remain caller supplied.
pub type value_G_L4507 = u8;
pub const G_L4507: Global<value_G_L4507> = Global::new("DAT_1401151b8", Some(0x1401151b8));

/// Source symbol DAT_140115290. Contents remain caller supplied.
pub type value_G_L4508 = u8;
pub const G_L4508: Global<value_G_L4508> = Global::new("DAT_140115290", Some(0x140115290));

/// Source symbol DAT_1401cff10. Contents remain caller supplied.
pub type value_G_L4509 = u8;
pub const G_L4509: Global<value_G_L4509> = Global::new("DAT_1401cff10", Some(0x1401cff10));

/// Source symbol DAT_1401cff98. Contents remain caller supplied.
pub type value_G_L4510 = u8;
pub const G_L4510: Global<value_G_L4510> = Global::new("DAT_1401cff98", Some(0x1401cff98));

/// Source symbol DAT_1401cff90. Contents remain caller supplied.
pub type value_G_L4511 = u64;
pub const G_L4511: Global<value_G_L4511> = Global::new("DAT_1401cff90", Some(0x1401cff90));

/// Source symbol DAT_1401cff78. Contents remain caller supplied.
pub type value_G_L4512 = u8;
pub const G_L4512: Global<value_G_L4512> = Global::new("DAT_1401cff78", Some(0x1401cff78));

/// Source symbol DAT_1401cff70. Contents remain caller supplied.
pub type value_G_L4513 = i64;
pub const G_L4513: Global<value_G_L4513> = Global::new("DAT_1401cff70", Some(0x1401cff70));

/// Source symbol DAT_140116c70. Contents remain caller supplied.
pub type value_G_L4514 = u8;
pub const G_L4514: Global<value_G_L4514> = Global::new("DAT_140116c70", Some(0x140116c70));

/// Source symbol FUN_1400c85c0. Contents remain caller supplied.
pub type value_G_L4515 = u8;
pub const G_L4515: Global<value_G_L4515> = Global::new("FUN_1400c85c0", Some(0x1400c85c0));

/// Source symbol FUN_1400c85b0. Contents remain caller supplied.
pub type value_G_L4516 = u8;
pub const G_L4516: Global<value_G_L4516> = Global::new("FUN_1400c85b0", Some(0x1400c85b0));

/// Source symbol FUN_1400c7eb0. Contents remain caller supplied.
pub type value_G_L4517 = u8;
pub const G_L4517: Global<value_G_L4517> = Global::new("FUN_1400c7eb0", Some(0x1400c7eb0));

/// Source symbol FUN_1400c7ea0. Contents remain caller supplied.
pub type value_G_L4518 = u8;
pub const G_L4518: Global<value_G_L4518> = Global::new("FUN_1400c7ea0", Some(0x1400c7ea0));

/// Source symbol PTR_GetThisMessageMap_140117988. Contents remain caller supplied.
pub type value_G_L4519 = Address<Unknown>;
pub const G_L4519: Global<value_G_L4519> = Global::new("PTR_GetThisMessageMap_140117988", None);

/// Source symbol DAT_1401d0698. Contents remain caller supplied.
pub type value_G_L4520 = u8;
pub const G_L4520: Global<value_G_L4520> = Global::new("DAT_1401d0698", Some(0x1401d0698));

/// Source symbol DAT_1401d07a0. Contents remain caller supplied.
pub type value_G_L4521 = i64;
pub const G_L4521: Global<value_G_L4521> = Global::new("DAT_1401d07a0", Some(0x1401d07a0));

/// Source symbol DAT_1401d07b8. Contents remain caller supplied.
pub type value_G_L4522 = u8;
pub const G_L4522: Global<value_G_L4522> = Global::new("DAT_1401d07b8", Some(0x1401d07b8));

/// Source symbol DAT_1401d07c0. Contents remain caller supplied.
pub type value_G_L4523 = u8;
pub const G_L4523: Global<value_G_L4523> = Global::new("DAT_1401d07c0", Some(0x1401d07c0));

/// Source symbol DAT_1401d07c4. Contents remain caller supplied.
pub type value_G_L4524 = u8;
pub const G_L4524: Global<value_G_L4524> = Global::new("DAT_1401d07c4", Some(0x1401d07c4));

/// Source symbol DAT_1401d085c. Contents remain caller supplied.
pub type value_G_L4525 = u8;
pub const G_L4525: Global<value_G_L4525> = Global::new("DAT_1401d085c", Some(0x1401d085c));

/// Source symbol DAT_1401d085e. Contents remain caller supplied.
pub type value_G_L4526 = u8;
pub const G_L4526: Global<value_G_L4526> = Global::new("DAT_1401d085e", Some(0x1401d085e));

/// Source symbol DAT_1401d0880. Contents remain caller supplied.
pub type value_G_L4527 = u8;
pub const G_L4527: Global<value_G_L4527> = Global::new("DAT_1401d0880", Some(0x1401d0880));

/// Source symbol DAT_1401d08a0. Contents remain caller supplied.
pub type value_G_L4528 = u8;
pub const G_L4528: Global<value_G_L4528> = Global::new("DAT_1401d08a0", Some(0x1401d08a0));

/// Source symbol FUN_14006aaa0. Contents remain caller supplied.
pub type value_G_L4529 = u8;
pub const G_L4529: Global<value_G_L4529> = Global::new("FUN_14006aaa0", Some(0x14006aaa0));

/// Source symbol DAT_1401d08d4. Contents remain caller supplied.
pub type value_G_L4530 = u8;
pub const G_L4530: Global<value_G_L4530> = Global::new("DAT_1401d08d4", Some(0x1401d08d4));

/// Source symbol DAT_1401d0908. Contents remain caller supplied.
pub type value_G_L4531 = u8;
pub const G_L4531: Global<value_G_L4531> = Global::new("DAT_1401d0908", Some(0x1401d0908));

/// Source symbol DAT_1401d0918. Contents remain caller supplied.
pub type value_G_L4532 = u8;
pub const G_L4532: Global<value_G_L4532> = Global::new("DAT_1401d0918", Some(0x1401d0918));

/// Source symbol DAT_1401d091c. Contents remain caller supplied.
pub type value_G_L4533 = u8;
pub const G_L4533: Global<value_G_L4533> = Global::new("DAT_1401d091c", Some(0x1401d091c));

/// Source symbol DAT_1401d0930. Contents remain caller supplied.
pub type value_G_L4534 = u32;
pub const G_L4534: Global<value_G_L4534> = Global::new("DAT_1401d0930", Some(0x1401d0930));

/// Source symbol DAT_1401d0b38. Contents remain caller supplied.
pub type value_G_L4535 = u8;
pub const G_L4535: Global<value_G_L4535> = Global::new("DAT_1401d0b38", Some(0x1401d0b38));

/// Source symbol DAT_1401d0b40. Contents remain caller supplied.
pub type value_G_L4536 = u8;
pub const G_L4536: Global<value_G_L4536> = Global::new("DAT_1401d0b40", Some(0x1401d0b40));

/// Source symbol DAT_1401d0b48. Contents remain caller supplied.
pub type value_G_L4537 = Address<u64>;
pub const G_L4537: Global<value_G_L4537> = Global::new("DAT_1401d0b48", Some(0x1401d0b48));

/// Source symbol DAT_1401d0b50. Contents remain caller supplied.
pub type value_G_L4538 = u8;
pub const G_L4538: Global<value_G_L4538> = Global::new("DAT_1401d0b50", Some(0x1401d0b50));

/// Source symbol DAT_1401d0b60. Contents remain caller supplied.
pub type value_G_L4539 = u8;
pub const G_L4539: Global<value_G_L4539> = Global::new("DAT_1401d0b60", Some(0x1401d0b60));

/// Source symbol DAT_1401d0b68. Contents remain caller supplied.
pub type value_G_L4540 = u8;
pub const G_L4540: Global<value_G_L4540> = Global::new("DAT_1401d0b68", Some(0x1401d0b68));

/// Source symbol DAT_1401d0b58. Contents remain caller supplied.
pub type value_G_L4541 = Address<u64>;
pub const G_L4541: Global<value_G_L4541> = Global::new("DAT_1401d0b58", Some(0x1401d0b58));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4542 = u64;
pub const G_L4542: Global<value_G_L4542> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401cff58. Contents remain caller supplied.
pub type value_G_L4543 = u8;
pub const G_L4543: Global<value_G_L4543> = Global::new("DAT_1401cff58", Some(0x1401cff58));

/// Source symbol DAT_1401cff50. Contents remain caller supplied.
pub type value_G_L4544 = i64;
pub const G_L4544: Global<value_G_L4544> = Global::new("DAT_1401cff50", Some(0x1401cff50));

/// Source symbol DAT_1401d0b88. Contents remain caller supplied.
pub type value_G_L4545 = u8;
pub const G_L4545: Global<value_G_L4545> = Global::new("DAT_1401d0b88", Some(0x1401d0b88));

/// Source symbol DAT_1401d0b90. Contents remain caller supplied.
pub type value_G_L4546 = u8;
pub const G_L4546: Global<value_G_L4546> = Global::new("DAT_1401d0b90", Some(0x1401d0b90));

/// Source symbol DAT_1401d0bb8. Contents remain caller supplied.
pub type value_G_L4547 = u8;
pub const G_L4547: Global<value_G_L4547> = Global::new("DAT_1401d0bb8", Some(0x1401d0bb8));

/// Source symbol DAT_1401d0bc0. Contents remain caller supplied.
pub type value_G_L4548 = u8;
pub const G_L4548: Global<value_G_L4548> = Global::new("DAT_1401d0bc0", Some(0x1401d0bc0));

/// Source symbol DAT_1401d0bd0. Contents remain caller supplied.
pub type value_G_L4549 = u8;
pub const G_L4549: Global<value_G_L4549> = Global::new("DAT_1401d0bd0", Some(0x1401d0bd0));

/// Source symbol DAT_1401d0bf0. Contents remain caller supplied.
pub type value_G_L4550 = u8;
pub const G_L4550: Global<value_G_L4550> = Global::new("DAT_1401d0bf0", Some(0x1401d0bf0));

/// Source symbol DAT_1401d0c00. Contents remain caller supplied.
pub type value_G_L4551 = u8;
pub const G_L4551: Global<value_G_L4551> = Global::new("DAT_1401d0c00", Some(0x1401d0c00));

/// Source symbol DAT_1401d0c08. Contents remain caller supplied.
pub type value_G_L4552 = u8;
pub const G_L4552: Global<value_G_L4552> = Global::new("DAT_1401d0c08", Some(0x1401d0c08));

/// Source symbol DAT_1401d0c20. Contents remain caller supplied.
pub type value_G_L4553 = u64;
pub const G_L4553: Global<value_G_L4553> = Global::new("DAT_1401d0c20", Some(0x1401d0c20));

/// Source symbol DAT_1401d0bb0. Contents remain caller supplied.
pub type value_G_L4554 = u8;
pub const G_L4554: Global<value_G_L4554> = Global::new("DAT_1401d0bb0", Some(0x1401d0bb0));

/// Source symbol DAT_1401d08a8. Contents remain caller supplied.
pub type value_G_L4555 = u8;
pub const G_L4555: Global<value_G_L4555> = Global::new("DAT_1401d08a8", Some(0x1401d08a8));

/// Source symbol DAT_1401d08b0. Contents remain caller supplied.
pub type value_G_L4556 = u8;
pub const G_L4556: Global<value_G_L4556> = Global::new("DAT_1401d08b0", Some(0x1401d08b0));

/// Source symbol DAT_1401d08c0. Contents remain caller supplied.
pub type value_G_L4557 = u8;
pub const G_L4557: Global<value_G_L4557> = Global::new("DAT_1401d08c0", Some(0x1401d08c0));

/// Source symbol DAT_1401d0bf8. Contents remain caller supplied.
pub type value_G_L4558 = u8;
pub const G_L4558: Global<value_G_L4558> = Global::new("DAT_1401d0bf8", Some(0x1401d0bf8));

/// Source symbol DAT_1401d08f0. Contents remain caller supplied.
pub type value_G_L4559 = u8;
pub const G_L4559: Global<value_G_L4559> = Global::new("DAT_1401d08f0", Some(0x1401d08f0));

/// Source symbol DAT_1401d07c6. Contents remain caller supplied.
pub type value_G_L4560 = u8;
pub const G_L4560: Global<value_G_L4560> = Global::new("DAT_1401d07c6", Some(0x1401d07c6));

/// Source symbol DAT_1401d08b8. Contents remain caller supplied.
pub type value_G_L4561 = u8;
pub const G_L4561: Global<value_G_L4561> = Global::new("DAT_1401d08b8", Some(0x1401d08b8));

/// Source symbol DAT_1401d08cc. Contents remain caller supplied.
pub type value_G_L4562 = u8;
pub const G_L4562: Global<value_G_L4562> = Global::new("DAT_1401d08cc", Some(0x1401d08cc));

/// Source symbol DAT_1401d0c18. Contents remain caller supplied.
pub type value_G_L4563 = u64;
pub const G_L4563: Global<value_G_L4563> = Global::new("DAT_1401d0c18", Some(0x1401d0c18));

/// Source symbol DAT_1401d08f8. Contents remain caller supplied.
pub type value_G_L4564 = u8;
pub const G_L4564: Global<value_G_L4564> = Global::new("DAT_1401d08f8", Some(0x1401d08f8));

/// Source symbol DAT_1401d07c8. Contents remain caller supplied.
pub type value_G_L4565 = u8;
pub const G_L4565: Global<value_G_L4565> = Global::new("DAT_1401d07c8", Some(0x1401d07c8));

/// Source symbol DAT_1401d0860. Contents remain caller supplied.
pub type value_G_L4566 = u8;
pub const G_L4566: Global<value_G_L4566> = Global::new("DAT_1401d0860", Some(0x1401d0860));

/// Source symbol DAT_1401d0868. Contents remain caller supplied.
pub type value_G_L4567 = u64;
pub const G_L4567: Global<value_G_L4567> = Global::new("DAT_1401d0868", Some(0x1401d0868));

/// Source symbol DAT_1401d0870. Contents remain caller supplied.
pub type value_G_L4568 = u8;
pub const G_L4568: Global<value_G_L4568> = Global::new("DAT_1401d0870", Some(0x1401d0870));

/// Source symbol DAT_1401d0878. Contents remain caller supplied.
pub type value_G_L4569 = u8;
pub const G_L4569: Global<value_G_L4569> = Global::new("DAT_1401d0878", Some(0x1401d0878));

/// Source symbol DAT_1401d0888. Contents remain caller supplied.
pub type value_G_L4570 = u8;
pub const G_L4570: Global<value_G_L4570> = Global::new("DAT_1401d0888", Some(0x1401d0888));

/// Source symbol DAT_1401d0890. Contents remain caller supplied.
pub type value_G_L4571 = u8;
pub const G_L4571: Global<value_G_L4571> = Global::new("DAT_1401d0890", Some(0x1401d0890));

/// Source symbol DAT_1401d0920. Contents remain caller supplied.
pub type value_G_L4572 = u8;
pub const G_L4572: Global<value_G_L4572> = Global::new("DAT_1401d0920", Some(0x1401d0920));

/// Source symbol DAT_1401d0ba0. Contents remain caller supplied.
pub type value_G_L4573 = u64;
pub const G_L4573: Global<value_G_L4573> = Global::new("DAT_1401d0ba0", Some(0x1401d0ba0));

/// Source symbol DAT_1401d0bc8. Contents remain caller supplied.
pub type value_G_L4574 = u8;
pub const G_L4574: Global<value_G_L4574> = Global::new("DAT_1401d0bc8", Some(0x1401d0bc8));

/// Source symbol UNK_1401d08dc. Contents remain caller supplied.
pub type value_G_L4575 = u64;
pub const G_L4575: Global<value_G_L4575> = Global::new("UNK_1401d08dc", None);

/// Source symbol UNK_1401d0910. Contents remain caller supplied.
pub type value_G_L4576 = u64;
pub const G_L4576: Global<value_G_L4576> = Global::new("UNK_1401d0910", None);

/// Source symbol UNK_1401d0b98. Contents remain caller supplied.
pub type value_G_L4577 = u64;
pub const G_L4577: Global<value_G_L4577> = Global::new("UNK_1401d0b98", None);

/// Source symbol FUN_14007cae0. Contents remain caller supplied.
pub type value_G_L4578 = u8;
pub const G_L4578: Global<value_G_L4578> = Global::new("FUN_14007cae0", Some(0x14007cae0));

/// Source symbol DAT_1401cff38. Contents remain caller supplied.
pub type value_G_L4582 = u8;
pub const G_L4582: Global<value_G_L4582> = Global::new("DAT_1401cff38", Some(0x1401cff38));

/// Source symbol DAT_1401cff30. Contents remain caller supplied.
pub type value_G_L4583 = i64;
pub const G_L4583: Global<value_G_L4583> = Global::new("DAT_1401cff30", Some(0x1401cff30));

/// Source symbol DAT_140116be0. Contents remain caller supplied.
pub type value_G_L4584 = u8;
pub const G_L4584: Global<value_G_L4584> = Global::new("DAT_140116be0", Some(0x140116be0));

/// Source symbol DAT_140116c18. Contents remain caller supplied.
pub type value_G_L4585 = u8;
pub const G_L4585: Global<value_G_L4585> = Global::new("DAT_140116c18", Some(0x140116c18));

/// Source symbol FUN_14007c7f0. Contents remain caller supplied.
pub type value_G_L4586 = u8;
pub const G_L4586: Global<value_G_L4586> = Global::new("FUN_14007c7f0", Some(0x14007c7f0));

/// Source symbol DAT_140116ae0. Contents remain caller supplied.
pub type value_G_L4587 = u8;
pub const G_L4587: Global<value_G_L4587> = Global::new("DAT_140116ae0", Some(0x140116ae0));

/// Source symbol DAT_1400e4cc0. Contents remain caller supplied.
pub type value_G_L4588 = u8;
pub const G_L4588: Global<value_G_L4588> = Global::new("DAT_1400e4cc0", Some(0x1400e4cc0));

/// Source symbol DAT_140106808. Contents remain caller supplied.
pub type value_G_L4589 = u8;
pub const G_L4589: Global<value_G_L4589> = Global::new("DAT_140106808", Some(0x140106808));

/// Source symbol PTR_s_CPBView_1401193a0. Contents remain caller supplied.
pub type value_G_L4591 = Address<Unknown>;
pub const G_L4591: Global<value_G_L4591> = Global::new("PTR_s_CPBView_1401193a0", None);

/// Source symbol DAT_1401d0900. Contents remain caller supplied.
pub type value_G_L4592 = Address<i16>;
pub const G_L4592: Global<value_G_L4592> = Global::new("DAT_1401d0900", Some(0x1401d0900));

/// Source symbol DAT_1401d0bd4. Contents remain caller supplied.
pub type value_G_L4593 = i32;
pub const G_L4593: Global<value_G_L4593> = Global::new("DAT_1401d0bd4", Some(0x1401d0bd4));

/// Source symbol DAT_1401cff28. Contents remain caller supplied.
pub type value_G_L4594 = i32;
pub const G_L4594: Global<value_G_L4594> = Global::new("DAT_1401cff28", Some(0x1401cff28));

/// Source symbol DAT_1401d0bf0. Contents remain caller supplied.
pub type value_G_L4595 = i8;
pub const G_L4595: Global<value_G_L4595> = Global::new("DAT_1401d0bf0", Some(0x1401d0bf0));

/// Source symbol DAT_1401d0bd0. Contents remain caller supplied.
pub type value_G_L4596 = i32;
pub const G_L4596: Global<value_G_L4596> = Global::new("DAT_1401d0bd0", Some(0x1401d0bd0));

/// Source symbol DAT_1401d0bd8. Contents remain caller supplied.
pub type value_G_L4597 = u64;
pub const G_L4597: Global<value_G_L4597> = Global::new("DAT_1401d0bd8", Some(0x1401d0bd8));

/// Source symbol DAT_1401d0be0. Contents remain caller supplied.
pub type value_G_L4598 = u64;
pub const G_L4598: Global<value_G_L4598> = Global::new("DAT_1401d0be0", Some(0x1401d0be0));

/// Source symbol DAT_1401d0be8. Contents remain caller supplied.
pub type value_G_L4599 = u64;
pub const G_L4599: Global<value_G_L4599> = Global::new("DAT_1401d0be8", Some(0x1401d0be8));

/// Source symbol DAT_140106820. Contents remain caller supplied.
pub type value_G_L4600 = u32;
pub const G_L4600: Global<value_G_L4600> = Global::new("DAT_140106820", Some(0x140106820));

/// Source symbol PTR_u_FileFormatError_140106828. Contents remain caller supplied.
pub type value_G_L4601 = Address<Unknown>;
pub const G_L4601: Global<value_G_L4601> = Global::new("PTR_u_FileFormatError_140106828", None);

/// Source symbol DAT_140106830. Contents remain caller supplied.
pub type value_G_L4602 = u8;
pub const G_L4602: Global<value_G_L4602> = Global::new("DAT_140106830", Some(0x140106830));

/// Source symbol DAT_140115a30. Contents remain caller supplied.
pub type value_G_L4603 = u8;
pub const G_L4603: Global<value_G_L4603> = Global::new("DAT_140115a30", Some(0x140115a30));

/// Source symbol DAT_1400e4eb0. Contents remain caller supplied.
pub type value_G_L4604 = u8;
pub const G_L4604: Global<value_G_L4604> = Global::new("DAT_1400e4eb0", Some(0x1400e4eb0));

/// Source symbol DAT_1400e4ec0. Contents remain caller supplied.
pub type value_G_L4605 = u8;
pub const G_L4605: Global<value_G_L4605> = Global::new("DAT_1400e4ec0", Some(0x1400e4ec0));

/// Source symbol DAT_1401d05d8. Contents remain caller supplied.
pub type value_G_L4606 = Address<i64>;
pub const G_L4606: Global<value_G_L4606> = Global::new("DAT_1401d05d8", Some(0x1401d05d8));

/// Source symbol DAT_1400e4c00. Contents remain caller supplied.
pub type value_G_L4607 = u64;
pub const G_L4607: Global<value_G_L4607> = Global::new("DAT_1400e4c00", Some(0x1400e4c00));

/// Source symbol DAT_1400e4c80. Contents remain caller supplied.
pub type value_G_L4608 = u64;
pub const G_L4608: Global<value_G_L4608> = Global::new("DAT_1400e4c80", Some(0x1400e4c80));

/// Source symbol DAT_1400e58f8. Contents remain caller supplied.
pub type value_G_L4609 = u8;
pub const G_L4609: Global<value_G_L4609> = Global::new("DAT_1400e58f8", Some(0x1400e58f8));

/// Source symbol DAT_140117168. Contents remain caller supplied.
pub type value_G_L4610 = u8;
pub const G_L4610: Global<value_G_L4610> = Global::new("DAT_140117168", Some(0x140117168));

/// Source symbol DAT_140117178. Contents remain caller supplied.
pub type value_G_L4611 = u8;
pub const G_L4611: Global<value_G_L4611> = Global::new("DAT_140117178", Some(0x140117178));

/// Source symbol DAT_140117188. Contents remain caller supplied.
pub type value_G_L4612 = u8;
pub const G_L4612: Global<value_G_L4612> = Global::new("DAT_140117188", Some(0x140117188));

/// Source symbol DAT_1401d0c30. Contents remain caller supplied.
pub type value_G_L4613 = u64;
pub const G_L4613: Global<value_G_L4613> = Global::new("DAT_1401d0c30", Some(0x1401d0c30));

/// Source symbol DAT_1401d07ba. Contents remain caller supplied.
pub type value_G_L4614 = u8;
pub const G_L4614: Global<value_G_L4614> = Global::new("DAT_1401d07ba", Some(0x1401d07ba));

/// Source symbol DAT_140106898. Contents remain caller supplied.
pub type value_G_L4615 = u8;
pub const G_L4615: Global<value_G_L4615> = Global::new("DAT_140106898", Some(0x140106898));

/// Source symbol DAT_1401068b0. Contents remain caller supplied.
pub type value_G_L4616 = u8;
pub const G_L4616: Global<value_G_L4616> = Global::new("DAT_1401068b0", Some(0x1401068b0));

/// Source symbol FUN_1400bedf0. Contents remain caller supplied.
pub type value_G_L4617 = u8;
pub const G_L4617: Global<value_G_L4617> = Global::new("FUN_1400bedf0", Some(0x1400bedf0));

/// Source symbol FUN_140076320. Contents remain caller supplied.
pub type value_G_L4618 = u8;
pub const G_L4618: Global<value_G_L4618> = Global::new("FUN_140076320", Some(0x140076320));

/// Source symbol FUN_1400bede0. Contents remain caller supplied.
pub type value_G_L4619 = u8;
pub const G_L4619: Global<value_G_L4619> = Global::new("FUN_1400bede0", Some(0x1400bede0));

/// Source symbol FUN_140047a30. Contents remain caller supplied.
pub type value_G_L4620 = u8;
pub const G_L4620: Global<value_G_L4620> = Global::new("FUN_140047a30", Some(0x140047a30));

/// Source symbol FUN_1400c8100. Contents remain caller supplied.
pub type value_G_L4621 = u8;
pub const G_L4621: Global<value_G_L4621> = Global::new("FUN_1400c8100", Some(0x1400c8100));

/// Source symbol FUN_1400c80f0. Contents remain caller supplied.
pub type value_G_L4622 = u8;
pub const G_L4622: Global<value_G_L4622> = Global::new("FUN_1400c80f0", Some(0x1400c80f0));

/// Source symbol DAT_1401d0638. Contents remain caller supplied.
pub type value_G_L4623 = i64;
pub const G_L4623: Global<value_G_L4623> = Global::new("DAT_1401d0638", Some(0x1401d0638));

/// Source symbol DAT_1401d0934. Contents remain caller supplied.
pub type value_G_L4624 = i16;
pub const G_L4624: Global<value_G_L4624> = Global::new("DAT_1401d0934", Some(0x1401d0934));

/// Source symbol DAT_140116e40. Contents remain caller supplied.
pub type value_G_L4625 = u8;
pub const G_L4625: Global<value_G_L4625> = Global::new("DAT_140116e40", Some(0x140116e40));

/// Source symbol FUN_1400b1d90. Contents remain caller supplied.
pub type value_G_L4626 = u8;
pub const G_L4626: Global<value_G_L4626> = Global::new("FUN_1400b1d90", Some(0x1400b1d90));

/// Source symbol FUN_1400b1d80. Contents remain caller supplied.
pub type value_G_L4627 = u8;
pub const G_L4627: Global<value_G_L4627> = Global::new("FUN_1400b1d80", Some(0x1400b1d80));

/// Source symbol DAT_140116e30. Contents remain caller supplied.
pub type value_G_L4628 = u8;
pub const G_L4628: Global<value_G_L4628> = Global::new("DAT_140116e30", Some(0x140116e30));

/// Source symbol DAT_1401168d0. Contents remain caller supplied.
pub type value_G_L4629 = u8;
pub const G_L4629: Global<value_G_L4629> = Global::new("DAT_1401168d0", Some(0x1401168d0));

/// Source symbol DAT_140116810. Contents remain caller supplied.
pub type value_G_L4630 = u8;
pub const G_L4630: Global<value_G_L4630> = Global::new("DAT_140116810", Some(0x140116810));

/// Source symbol FUN_1400c6ff0. Contents remain caller supplied.
pub type value_G_L4631 = u8;
pub const G_L4631: Global<value_G_L4631> = Global::new("FUN_1400c6ff0", Some(0x1400c6ff0));

/// Source symbol FUN_1400c6fd0. Contents remain caller supplied.
pub type value_G_L4632 = u8;
pub const G_L4632: Global<value_G_L4632> = Global::new("FUN_1400c6fd0", Some(0x1400c6fd0));

/// Source symbol FUN_1400c7750. Contents remain caller supplied.
pub type value_G_L4633 = u8;
pub const G_L4633: Global<value_G_L4633> = Global::new("FUN_1400c7750", Some(0x1400c7750));

/// Source symbol FUN_1400c6fe0. Contents remain caller supplied.
pub type value_G_L4634 = u8;
pub const G_L4634: Global<value_G_L4634> = Global::new("FUN_1400c6fe0", Some(0x1400c6fe0));

/// Source symbol DAT_140116430. Contents remain caller supplied.
pub type value_G_L4635 = u8;
pub const G_L4635: Global<value_G_L4635> = Global::new("DAT_140116430", Some(0x140116430));

/// Source symbol DAT_140116420. Contents remain caller supplied.
pub type value_G_L4636 = u8;
pub const G_L4636: Global<value_G_L4636> = Global::new("DAT_140116420", Some(0x140116420));

/// Source symbol DAT_140116410. Contents remain caller supplied.
pub type value_G_L4637 = u8;
pub const G_L4637: Global<value_G_L4637> = Global::new("DAT_140116410", Some(0x140116410));

/// Source symbol DAT_1401cff20. Contents remain caller supplied.
pub type value_G_L4638 = i64;
pub const G_L4638: Global<value_G_L4638> = Global::new("DAT_1401cff20", Some(0x1401cff20));

/// Source symbol DAT_1401cff30. Contents remain caller supplied.
pub type value_G_L4639 = u64;
pub const G_L4639: Global<value_G_L4639> = Global::new("DAT_1401cff30", Some(0x1401cff30));

/// Source symbol DAT_140117418. Contents remain caller supplied.
pub type value_G_L4640 = u8;
pub const G_L4640: Global<value_G_L4640> = Global::new("DAT_140117418", Some(0x140117418));

/// Source symbol DAT_1401cff40. Contents remain caller supplied.
pub type value_G_L4641 = u8;
pub const G_L4641: Global<value_G_L4641> = Global::new("DAT_1401cff40", Some(0x1401cff40));

/// Source symbol DAT_1401cff50. Contents remain caller supplied.
pub type value_G_L4642 = u64;
pub const G_L4642: Global<value_G_L4642> = Global::new("DAT_1401cff50", Some(0x1401cff50));

/// Source symbol DAT_1401cff60. Contents remain caller supplied.
pub type value_G_L4643 = u8;
pub const G_L4643: Global<value_G_L4643> = Global::new("DAT_1401cff60", Some(0x1401cff60));

/// Source symbol DAT_140117510. Contents remain caller supplied.
pub type value_G_L4644 = u8;
pub const G_L4644: Global<value_G_L4644> = Global::new("DAT_140117510", Some(0x140117510));

/// Source symbol DAT_1401cff70. Contents remain caller supplied.
pub type value_G_L4645 = u64;
pub const G_L4645: Global<value_G_L4645> = Global::new("DAT_1401cff70", Some(0x1401cff70));

/// Source symbol DAT_140117d40. Contents remain caller supplied.
pub type value_G_L4646 = u8;
pub const G_L4646: Global<value_G_L4646> = Global::new("DAT_140117d40", Some(0x140117d40));

/// Source symbol DAT_1401cff80. Contents remain caller supplied.
pub type value_G_L4647 = u8;
pub const G_L4647: Global<value_G_L4647> = Global::new("DAT_1401cff80", Some(0x1401cff80));

/// Source symbol DAT_140117d50. Contents remain caller supplied.
pub type value_G_L4648 = u8;
pub const G_L4648: Global<value_G_L4648> = Global::new("DAT_140117d50", Some(0x140117d50));

/// Source symbol DAT_1401cffa0. Contents remain caller supplied.
pub type value_G_L4649 = u8;
pub const G_L4649: Global<value_G_L4649> = Global::new("DAT_1401cffa0", Some(0x1401cffa0));

/// Source symbol PTR_GetThisMessageMap_140119390. Contents remain caller supplied.
pub type value_G_L4651 = Address<Unknown>;
pub const G_L4651: Global<value_G_L4651> = Global::new("PTR_GetThisMessageMap_140119390", None);

/// Source symbol DAT_1401d06e0. Contents remain caller supplied.
pub type value_G_L4652 = i64;
pub const G_L4652: Global<value_G_L4652> = Global::new("DAT_1401d06e0", Some(0x1401d06e0));

/// Source symbol DAT_1401d0654. Contents remain caller supplied.
pub type value_G_L4654 = u32;
pub const G_L4654: Global<value_G_L4654> = Global::new("DAT_1401d0654", Some(0x1401d0654));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4656 = Address<i16>;
pub const G_L4656: Global<value_G_L4656> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401d0cc8. Contents remain caller supplied.
pub type value_G_L4657 = i64;
pub const G_L4657: Global<value_G_L4657> = Global::new("DAT_1401d0cc8", Some(0x1401d0cc8));

/// Source symbol DAT_1401d0cc0. Contents remain caller supplied.
pub type value_G_L4658 = u64;
pub const G_L4658: Global<value_G_L4658> = Global::new("DAT_1401d0cc0", Some(0x1401d0cc0));

/// Source symbol DAT_1401d0cb8. Contents remain caller supplied.
pub type value_G_L4659 = i64;
pub const G_L4659: Global<value_G_L4659> = Global::new("DAT_1401d0cb8", Some(0x1401d0cb8));

/// Source symbol DAT_1401d0cb0. Contents remain caller supplied.
pub type value_G_L4660 = i64;
pub const G_L4660: Global<value_G_L4660> = Global::new("DAT_1401d0cb0", Some(0x1401d0cb0));

/// Source symbol FUN_1400b5ba0. Contents remain caller supplied.
pub type value_G_L4661 = u8;
pub const G_L4661: Global<value_G_L4661> = Global::new("FUN_1400b5ba0", Some(0x1400b5ba0));

/// Source symbol FUN_1400b5bb0. Contents remain caller supplied.
pub type value_G_L4662 = u8;
pub const G_L4662: Global<value_G_L4662> = Global::new("FUN_1400b5bb0", Some(0x1400b5bb0));

/// Source symbol FUN_1400bbf60. Contents remain caller supplied.
pub type value_G_L4663 = u8;
pub const G_L4663: Global<value_G_L4663> = Global::new("FUN_1400bbf60", Some(0x1400bbf60));

/// Source symbol FUN_1400bbf50. Contents remain caller supplied.
pub type value_G_L4664 = u8;
pub const G_L4664: Global<value_G_L4664> = Global::new("FUN_1400bbf50", Some(0x1400bbf50));

/// Source symbol FUN_1400bc200. Contents remain caller supplied.
pub type value_G_L4665 = u8;
pub const G_L4665: Global<value_G_L4665> = Global::new("FUN_1400bc200", Some(0x1400bc200));

/// Source symbol FUN_1400bc1f0. Contents remain caller supplied.
pub type value_G_L4666 = u8;
pub const G_L4666: Global<value_G_L4666> = Global::new("FUN_1400bc1f0", Some(0x1400bc1f0));

/// Source symbol FUN_1400b4ef0. Contents remain caller supplied.
pub type value_G_L4667 = u8;
pub const G_L4667: Global<value_G_L4667> = Global::new("FUN_1400b4ef0", Some(0x1400b4ef0));

/// Source symbol FUN_1400b4ee0. Contents remain caller supplied.
pub type value_G_L4668 = u8;
pub const G_L4668: Global<value_G_L4668> = Global::new("FUN_1400b4ee0", Some(0x1400b4ee0));

/// Source symbol DAT_1401d0ca0. Contents remain caller supplied.
pub type value_G_L4669 = i64;
pub const G_L4669: Global<value_G_L4669> = Global::new("DAT_1401d0ca0", Some(0x1401d0ca0));

/// Source symbol DAT_1401d0058. Contents remain caller supplied.
pub type value_G_L4670 = u8;
pub const G_L4670: Global<value_G_L4670> = Global::new("DAT_1401d0058", Some(0x1401d0058));

/// Source symbol DAT_1401d0050. Contents remain caller supplied.
pub type value_G_L4671 = u64;
pub const G_L4671: Global<value_G_L4671> = Global::new("DAT_1401d0050", Some(0x1401d0050));

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4672 = t_dbd699519b88;
pub const G_L4672: Global<value_G_L4672> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol FUN_140085e80. Contents remain caller supplied.
pub type value_G_L4673 = u8;
pub const G_L4673: Global<value_G_L4673> = Global::new("FUN_140085e80", Some(0x140085e80));

/// Source symbol FUN_140086140. Contents remain caller supplied.
pub type value_G_L4674 = u8;
pub const G_L4674: Global<value_G_L4674> = Global::new("FUN_140086140", Some(0x140086140));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4675 = Address<u16>;
pub const G_L4675: Global<value_G_L4675> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401d0038. Contents remain caller supplied.
pub type value_G_L4676 = u8;
pub const G_L4676: Global<value_G_L4676> = Global::new("DAT_1401d0038", Some(0x1401d0038));

/// Source symbol DAT_1401d0030. Contents remain caller supplied.
pub type value_G_L4677 = i64;
pub const G_L4677: Global<value_G_L4677> = Global::new("DAT_1401d0030", Some(0x1401d0030));

/// Source symbol DAT_1401d0018. Contents remain caller supplied.
pub type value_G_L4678 = u8;
pub const G_L4678: Global<value_G_L4678> = Global::new("DAT_1401d0018", Some(0x1401d0018));

/// Source symbol DAT_1401d0010. Contents remain caller supplied.
pub type value_G_L4679 = i64;
pub const G_L4679: Global<value_G_L4679> = Global::new("DAT_1401d0010", Some(0x1401d0010));

/// Source symbol FUN_1400b5850. Contents remain caller supplied.
pub type value_G_L4680 = u8;
pub const G_L4680: Global<value_G_L4680> = Global::new("FUN_1400b5850", Some(0x1400b5850));

/// Source symbol FUN_1400b5860. Contents remain caller supplied.
pub type value_G_L4681 = u8;
pub const G_L4681: Global<value_G_L4681> = Global::new("FUN_1400b5860", Some(0x1400b5860));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4682 = Address<Unknown>;
pub const G_L4682: Global<value_G_L4682> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_140179148. Contents remain caller supplied.
pub type value_G_L4683 = u32;
pub const G_L4683: Global<value_G_L4683> = Global::new("DAT_140179148", Some(0x140179148));

/// Source symbol FUN_1400b2250. Contents remain caller supplied.
pub type value_G_L4685 = u8;
pub const G_L4685: Global<value_G_L4685> = Global::new("FUN_1400b2250", Some(0x1400b2250));

/// Source symbol FUN_1400b2240. Contents remain caller supplied.
pub type value_G_L4686 = u8;
pub const G_L4686: Global<value_G_L4686> = Global::new("FUN_1400b2240", Some(0x1400b2240));

/// Source symbol DAT_1401d07c2. Contents remain caller supplied.
pub type value_G_L4687 = u8;
pub const G_L4687: Global<value_G_L4687> = Global::new("DAT_1401d07c2", Some(0x1401d07c2));

/// Source symbol FUN_140085a60. Contents remain caller supplied.
pub type value_G_L4688 = u8;
pub const G_L4688: Global<value_G_L4688> = Global::new("FUN_140085a60", Some(0x140085a60));

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4689 = f64;
pub const G_L4689: Global<value_G_L4689> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol FUN_140085ce0. Contents remain caller supplied.
pub type value_G_L4690 = u8;
pub const G_L4690: Global<value_G_L4690> = Global::new("FUN_140085ce0", Some(0x140085ce0));

/// Source symbol DAT_1401cfff8. Contents remain caller supplied.
pub type value_G_L4691 = u8;
pub const G_L4691: Global<value_G_L4691> = Global::new("DAT_1401cfff8", Some(0x1401cfff8));

/// Source symbol DAT_1401cfff0. Contents remain caller supplied.
pub type value_G_L4692 = i64;
pub const G_L4692: Global<value_G_L4692> = Global::new("DAT_1401cfff0", Some(0x1401cfff0));

/// Source symbol DAT_1401cffd8. Contents remain caller supplied.
pub type value_G_L4693 = u8;
pub const G_L4693: Global<value_G_L4693> = Global::new("DAT_1401cffd8", Some(0x1401cffd8));

/// Source symbol DAT_1401cffd0. Contents remain caller supplied.
pub type value_G_L4694 = u64;
pub const G_L4694: Global<value_G_L4694> = Global::new("DAT_1401cffd0", Some(0x1401cffd0));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4695 = t_a2484b182201;
pub const G_L4695: Global<value_G_L4695> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol FUN_140091b50. Contents remain caller supplied.
pub type value_G_L4696 = u8;
pub const G_L4696: Global<value_G_L4696> = Global::new("FUN_140091b50", Some(0x140091b50));

/// Source symbol DAT_140118628. Contents remain caller supplied.
pub type value_G_L4697 = u8;
pub const G_L4697: Global<value_G_L4697> = Global::new("DAT_140118628", Some(0x140118628));

/// Source symbol FUN_1400857e0. Contents remain caller supplied.
pub type value_G_L4698 = u8;
pub const G_L4698: Global<value_G_L4698> = Global::new("FUN_1400857e0", Some(0x1400857e0));

/// Source symbol DAT_1401cffb8. Contents remain caller supplied.
pub type value_G_L4699 = u8;
pub const G_L4699: Global<value_G_L4699> = Global::new("DAT_1401cffb8", Some(0x1401cffb8));

/// Source symbol DAT_1401cffb0. Contents remain caller supplied.
pub type value_G_L4700 = u64;
pub const G_L4700: Global<value_G_L4700> = Global::new("DAT_1401cffb0", Some(0x1401cffb0));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4701 = Address<Address<u32>>;
pub const G_L4701: Global<value_G_L4701> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol FUN_1400c91d0. Contents remain caller supplied.
pub type value_G_L4702 = u8;
pub const G_L4702: Global<value_G_L4702> = Global::new("FUN_1400c91d0", Some(0x1400c91d0));

/// Source symbol FUN_1400c91c0. Contents remain caller supplied.
pub type value_G_L4703 = u8;
pub const G_L4703: Global<value_G_L4703> = Global::new("FUN_1400c91c0", Some(0x1400c91c0));

/// Source symbol FUN_1400c8c30. Contents remain caller supplied.
pub type value_G_L4704 = u8;
pub const G_L4704: Global<value_G_L4704> = Global::new("FUN_1400c8c30", Some(0x1400c8c30));

/// Source symbol FUN_1400c8c20. Contents remain caller supplied.
pub type value_G_L4705 = u8;
pub const G_L4705: Global<value_G_L4705> = Global::new("FUN_1400c8c20", Some(0x1400c8c20));

/// Source symbol FUN_1400ba230. Contents remain caller supplied.
pub type value_G_L4706 = u8;
pub const G_L4706: Global<value_G_L4706> = Global::new("FUN_1400ba230", Some(0x1400ba230));

/// Source symbol FUN_1400ba220. Contents remain caller supplied.
pub type value_G_L4707 = u8;
pub const G_L4707: Global<value_G_L4707> = Global::new("FUN_1400ba220", Some(0x1400ba220));

/// Source symbol DAT_1401cffc0. Contents remain caller supplied.
pub type value_G_L4708 = u8;
pub const G_L4708: Global<value_G_L4708> = Global::new("DAT_1401cffc0", Some(0x1401cffc0));

/// Source symbol DAT_140118880. Contents remain caller supplied.
pub type value_G_L4709 = u8;
pub const G_L4709: Global<value_G_L4709> = Global::new("DAT_140118880", Some(0x140118880));

/// Source symbol DAT_1401cffe0. Contents remain caller supplied.
pub type value_G_L4710 = u8;
pub const G_L4710: Global<value_G_L4710> = Global::new("DAT_1401cffe0", Some(0x1401cffe0));

/// Source symbol DAT_1401188d8. Contents remain caller supplied.
pub type value_G_L4711 = u8;
pub const G_L4711: Global<value_G_L4711> = Global::new("DAT_1401188d8", Some(0x1401188d8));

/// Source symbol DAT_1401cfff0. Contents remain caller supplied.
pub type value_G_L4712 = u64;
pub const G_L4712: Global<value_G_L4712> = Global::new("DAT_1401cfff0", Some(0x1401cfff0));

/// Source symbol DAT_1401d0000. Contents remain caller supplied.
pub type value_G_L4713 = u8;
pub const G_L4713: Global<value_G_L4713> = Global::new("DAT_1401d0000", Some(0x1401d0000));

/// Source symbol DAT_140118920. Contents remain caller supplied.
pub type value_G_L4714 = u8;
pub const G_L4714: Global<value_G_L4714> = Global::new("DAT_140118920", Some(0x140118920));

/// Source symbol DAT_1401d0010. Contents remain caller supplied.
pub type value_G_L4715 = u64;
pub const G_L4715: Global<value_G_L4715> = Global::new("DAT_1401d0010", Some(0x1401d0010));

/// Source symbol DAT_140118930. Contents remain caller supplied.
pub type value_G_L4716 = u8;
pub const G_L4716: Global<value_G_L4716> = Global::new("DAT_140118930", Some(0x140118930));

/// Source symbol DAT_1401d0020. Contents remain caller supplied.
pub type value_G_L4717 = u8;
pub const G_L4717: Global<value_G_L4717> = Global::new("DAT_1401d0020", Some(0x1401d0020));

/// Source symbol DAT_1401d0030. Contents remain caller supplied.
pub type value_G_L4718 = u64;
pub const G_L4718: Global<value_G_L4718> = Global::new("DAT_1401d0030", Some(0x1401d0030));

/// Source symbol DAT_1401d0040. Contents remain caller supplied.
pub type value_G_L4719 = u8;
pub const G_L4719: Global<value_G_L4719> = Global::new("DAT_1401d0040", Some(0x1401d0040));

/// Source symbol DAT_140118980. Contents remain caller supplied.
pub type value_G_L4720 = u8;
pub const G_L4720: Global<value_G_L4720> = Global::new("DAT_140118980", Some(0x140118980));

/// Source symbol DAT_140118990. Contents remain caller supplied.
pub type value_G_L4721 = u8;
pub const G_L4721: Global<value_G_L4721> = Global::new("DAT_140118990", Some(0x140118990));

/// Source symbol DAT_1401d0060. Contents remain caller supplied.
pub type value_G_L4722 = u8;
pub const G_L4722: Global<value_G_L4722> = Global::new("DAT_1401d0060", Some(0x1401d0060));

/// Source symbol DAT_1401d0898. Contents remain caller supplied.
pub type value_G_L4723 = i32;
pub const G_L4723: Global<value_G_L4723> = Global::new("DAT_1401d0898", Some(0x1401d0898));

/// Source symbol DAT_1401d08b4. Contents remain caller supplied.
pub type value_G_L4724 = u32;
pub const G_L4724: Global<value_G_L4724> = Global::new("DAT_1401d08b4", Some(0x1401d08b4));

/// Source symbol DAT_140089628. Contents remain caller supplied.
pub type value_G_L4725 = u8;
pub const G_L4725: Global<value_G_L4725> = Global::new("DAT_140089628", Some(0x140089628));

/// Source symbol DAT_1400e4ee0. Contents remain caller supplied.
pub type value_G_L4726 = u8;
pub const G_L4726: Global<value_G_L4726> = Global::new("DAT_1400e4ee0", Some(0x1400e4ee0));

/// Source symbol DAT_1400e4ef0. Contents remain caller supplied.
pub type value_G_L4727 = u8;
pub const G_L4727: Global<value_G_L4727> = Global::new("DAT_1400e4ef0", Some(0x1400e4ef0));

/// Source symbol DAT_1400e4f00. Contents remain caller supplied.
pub type value_G_L4728 = u8;
pub const G_L4728: Global<value_G_L4728> = Global::new("DAT_1400e4f00", Some(0x1400e4f00));

/// Source symbol DAT_1400e4f10. Contents remain caller supplied.
pub type value_G_L4729 = u8;
pub const G_L4729: Global<value_G_L4729> = Global::new("DAT_1400e4f10", Some(0x1400e4f10));

/// Source symbol DAT_1400e4f20. Contents remain caller supplied.
pub type value_G_L4730 = u8;
pub const G_L4730: Global<value_G_L4730> = Global::new("DAT_1400e4f20", Some(0x1400e4f20));

/// Source symbol DAT_1400e4f30. Contents remain caller supplied.
pub type value_G_L4731 = u8;
pub const G_L4731: Global<value_G_L4731> = Global::new("DAT_1400e4f30", Some(0x1400e4f30));

/// Source symbol DAT_1400e4f40. Contents remain caller supplied.
pub type value_G_L4732 = u8;
pub const G_L4732: Global<value_G_L4732> = Global::new("DAT_1400e4f40", Some(0x1400e4f40));

/// Source symbol DAT_1400e4f50. Contents remain caller supplied.
pub type value_G_L4733 = u8;
pub const G_L4733: Global<value_G_L4733> = Global::new("DAT_1400e4f50", Some(0x1400e4f50));

/// Source symbol DAT_140108978. Contents remain caller supplied.
pub type value_G_L4734 = u8;
pub const G_L4734: Global<value_G_L4734> = Global::new("DAT_140108978", Some(0x140108978));

/// Source symbol DAT_140108988. Contents remain caller supplied.
pub type value_G_L4735 = u8;
pub const G_L4735: Global<value_G_L4735> = Global::new("DAT_140108988", Some(0x140108988));

/// Source symbol DAT_1400e4ed0. Contents remain caller supplied.
pub type value_G_L4736 = u8;
pub const G_L4736: Global<value_G_L4736> = Global::new("DAT_1400e4ed0", Some(0x1400e4ed0));

/// Source symbol PTR_DAT_140105980. Contents remain caller supplied.
pub type value_G_L4737 = Address<Unknown>;
pub const G_L4737: Global<value_G_L4737> = Global::new("PTR_DAT_140105980", None);

/// Source symbol DAT_140119cd0. Contents remain caller supplied.
pub type value_G_L4738 = u8;
pub const G_L4738: Global<value_G_L4738> = Global::new("DAT_140119cd0", Some(0x140119cd0));

/// Source symbol FUN_14008c950. Contents remain caller supplied.
pub type value_G_L4739 = u8;
pub const G_L4739: Global<value_G_L4739> = Global::new("FUN_14008c950", Some(0x14008c950));

/// Source symbol DAT_1401d0078. Contents remain caller supplied.
pub type value_G_L4740 = u8;
pub const G_L4740: Global<value_G_L4740> = Global::new("DAT_1401d0078", Some(0x1401d0078));

/// Source symbol DAT_1401d0070. Contents remain caller supplied.
pub type value_G_L4741 = i64;
pub const G_L4741: Global<value_G_L4741> = Global::new("DAT_1401d0070", Some(0x1401d0070));

/// Source symbol DAT_1401d0070. Contents remain caller supplied.
pub type value_G_L4742 = u64;
pub const G_L4742: Global<value_G_L4742> = Global::new("DAT_1401d0070", Some(0x1401d0070));

/// Source symbol DAT_140119e00. Contents remain caller supplied.
pub type value_G_L4743 = u8;
pub const G_L4743: Global<value_G_L4743> = Global::new("DAT_140119e00", Some(0x140119e00));

/// Source symbol DAT_1401d0080. Contents remain caller supplied.
pub type value_G_L4744 = u8;
pub const G_L4744: Global<value_G_L4744> = Global::new("DAT_1401d0080", Some(0x1401d0080));

/// Source symbol PTR_s_CPBSrvrItem_140119ff8. Contents remain caller supplied.
pub type value_G_L4745 = Address<Unknown>;
pub const G_L4745: Global<value_G_L4745> = Global::new("PTR_s_CPBSrvrItem_140119ff8", None);

/// Source symbol DAT_1401d085b. Contents remain caller supplied.
pub type value_G_L4747 = i8;
pub const G_L4747: Global<value_G_L4747> = Global::new("DAT_1401d085b", Some(0x1401d085b));

/// Source symbol DAT_1401d085c. Contents remain caller supplied.
pub type value_G_L4748 = u8;
pub const G_L4748: Global<value_G_L4748> = Global::new("DAT_1401d085c", Some(0x1401d085c));

/// Source symbol DAT_1401d0400. Contents remain caller supplied.
pub type value_G_L4749 = CodeAddress;
pub const G_L4749: Global<value_G_L4749> = Global::new("DAT_1401d0400", Some(0x1401d0400));

/// Source symbol DAT_1401d03a0. Contents remain caller supplied.
pub type value_G_L4750 = i8;
pub const G_L4750: Global<value_G_L4750> = Global::new("DAT_1401d03a0", Some(0x1401d03a0));

/// Source symbol DAT_1401d03a8. Contents remain caller supplied.
pub type value_G_L4751 = i8;
pub const G_L4751: Global<value_G_L4751> = Global::new("DAT_1401d03a8", Some(0x1401d03a8));

/// Source symbol DAT_1401d03c0. Contents remain caller supplied.
pub type value_G_L4752 = u8;
pub const G_L4752: Global<value_G_L4752> = Global::new("DAT_1401d03c0", Some(0x1401d03c0));

/// Source symbol FUN_140012480. Contents remain caller supplied.
pub type value_G_L4753 = u8;
pub const G_L4753: Global<value_G_L4753> = Global::new("FUN_140012480", Some(0x140012480));

/// Source symbol DAT_1401d0368. Contents remain caller supplied.
pub type value_G_L4754 = Address<u8>;
pub const G_L4754: Global<value_G_L4754> = Global::new("DAT_1401d0368", Some(0x1401d0368));

/// Source symbol DAT_14016af08. Contents remain caller supplied.
pub type value_G_L4755 = u8;
pub const G_L4755: Global<value_G_L4755> = Global::new("DAT_14016af08", Some(0x14016af08));

/// Source symbol DAT_1401d0380. Contents remain caller supplied.
pub type value_G_L4756 = i64;
pub const G_L4756: Global<value_G_L4756> = Global::new("DAT_1401d0380", Some(0x1401d0380));

/// Source symbol DAT_1401d0398. Contents remain caller supplied.
pub type value_G_L4757 = i64;
pub const G_L4757: Global<value_G_L4757> = Global::new("DAT_1401d0398", Some(0x1401d0398));

/// Source symbol DAT_1401d0368. Contents remain caller supplied.
pub type value_G_L4758 = i64;
pub const G_L4758: Global<value_G_L4758> = Global::new("DAT_1401d0368", Some(0x1401d0368));

/// Source symbol DAT_14011a180. Contents remain caller supplied.
pub type value_G_L4759 = u8;
pub const G_L4759: Global<value_G_L4759> = Global::new("DAT_14011a180", Some(0x14011a180));

/// Source symbol DAT_14011a188. Contents remain caller supplied.
pub type value_G_L4760 = u8;
pub const G_L4760: Global<value_G_L4760> = Global::new("DAT_14011a188", Some(0x14011a188));

/// Source symbol DAT_14011a190. Contents remain caller supplied.
pub type value_G_L4761 = u8;
pub const G_L4761: Global<value_G_L4761> = Global::new("DAT_14011a190", Some(0x14011a190));

/// Source symbol DAT_1401d0360. Contents remain caller supplied.
pub type value_G_L4762 = Address<u64>;
pub const G_L4762: Global<value_G_L4762> = Global::new("DAT_1401d0360", Some(0x1401d0360));

/// Source symbol DAT_140179160. Contents remain caller supplied.
pub type value_G_L4763 = i32;
pub const G_L4763: Global<value_G_L4763> = Global::new("DAT_140179160", Some(0x140179160));

/// Source symbol DAT_1401d0378. Contents remain caller supplied.
pub type value_G_L4764 = i64;
pub const G_L4764: Global<value_G_L4764> = Global::new("DAT_1401d0378", Some(0x1401d0378));

/// Source symbol DAT_1401d0370. Contents remain caller supplied.
pub type value_G_L4765 = i64;
pub const G_L4765: Global<value_G_L4765> = Global::new("DAT_1401d0370", Some(0x1401d0370));

/// Source symbol DAT_1401d03ac. Contents remain caller supplied.
pub type value_G_L4766 = u32;
pub const G_L4766: Global<value_G_L4766> = Global::new("DAT_1401d03ac", Some(0x1401d03ac));

/// Source symbol DAT_1401d03b0. Contents remain caller supplied.
pub type value_G_L4767 = i32;
pub const G_L4767: Global<value_G_L4767> = Global::new("DAT_1401d03b0", Some(0x1401d03b0));

/// Source symbol DAT_1401d0390. Contents remain caller supplied.
pub type value_G_L4768 = i8;
pub const G_L4768: Global<value_G_L4768> = Global::new("DAT_1401d0390", Some(0x1401d0390));

/// Source symbol DAT_1401d0388. Contents remain caller supplied.
pub type value_G_L4769 = i64;
pub const G_L4769: Global<value_G_L4769> = Global::new("DAT_1401d0388", Some(0x1401d0388));

/// Source symbol PTR_s_CFullScreenThumbnailView_14011abc0. Contents remain caller supplied.
pub type value_G_L4770 = Address<Unknown>;
pub const G_L4770: Global<value_G_L4770> =
    Global::new("PTR_s_CFullScreenThumbnailView_14011abc0", None);

/// Source symbol PTR_GetThisMessageMap_14011a810. Contents remain caller supplied.
pub type value_G_L4771 = Address<u8>;
pub const G_L4771: Global<value_G_L4771> = Global::new("PTR_GetThisMessageMap_14011a810", None);

/// Source symbol DAT_1401d0350. Contents remain caller supplied.
pub type value_G_L4772 = Address<t_4c0908fcb2fb>;
pub const G_L4772: Global<value_G_L4772> = Global::new("DAT_1401d0350", Some(0x1401d0350));

/// Source symbol DAT_1401d0320. Contents remain caller supplied.
pub type value_G_L4773 = CodeAddress;
pub const G_L4773: Global<value_G_L4773> = Global::new("DAT_1401d0320", Some(0x1401d0320));

/// Source symbol DAT_1401d0328. Contents remain caller supplied.
pub type value_G_L4774 = CodeAddress;
pub const G_L4774: Global<value_G_L4774> = Global::new("DAT_1401d0328", Some(0x1401d0328));

/// Source symbol DAT_1401d0330. Contents remain caller supplied.
pub type value_G_L4775 = CodeAddress;
pub const G_L4775: Global<value_G_L4775> = Global::new("DAT_1401d0330", Some(0x1401d0330));

/// Source symbol DAT_1401d0338. Contents remain caller supplied.
pub type value_G_L4776 = CodeAddress;
pub const G_L4776: Global<value_G_L4776> = Global::new("DAT_1401d0338", Some(0x1401d0338));

/// Source symbol DAT_1401d0340. Contents remain caller supplied.
pub type value_G_L4777 = CodeAddress;
pub const G_L4777: Global<value_G_L4777> = Global::new("DAT_1401d0340", Some(0x1401d0340));

/// Source symbol DAT_1401d0348. Contents remain caller supplied.
pub type value_G_L4778 = u8;
pub const G_L4778: Global<value_G_L4778> = Global::new("DAT_1401d0348", Some(0x1401d0348));

/// Source symbol DAT_1401d01a8. Contents remain caller supplied.
pub type value_G_L4779 = u8;
pub const G_L4779: Global<value_G_L4779> = Global::new("DAT_1401d01a8", Some(0x1401d01a8));

/// Source symbol DAT_1401d01a0. Contents remain caller supplied.
pub type value_G_L4780 = i64;
pub const G_L4780: Global<value_G_L4780> = Global::new("DAT_1401d01a0", Some(0x1401d01a0));

/// Source symbol DAT_1401d01c8. Contents remain caller supplied.
pub type value_G_L4781 = u8;
pub const G_L4781: Global<value_G_L4781> = Global::new("DAT_1401d01c8", Some(0x1401d01c8));

/// Source symbol DAT_1401d01c0. Contents remain caller supplied.
pub type value_G_L4782 = u64;
pub const G_L4782: Global<value_G_L4782> = Global::new("DAT_1401d01c0", Some(0x1401d01c0));

/// Source symbol FUN_140092da0. Contents remain caller supplied.
pub type value_G_L4783 = u8;
pub const G_L4783: Global<value_G_L4783> = Global::new("FUN_140092da0", Some(0x140092da0));

/// Source symbol FUN_140092e10. Contents remain caller supplied.
pub type value_G_L4784 = u8;
pub const G_L4784: Global<value_G_L4784> = Global::new("FUN_140092e10", Some(0x140092e10));

/// Source symbol FUN_140092e80. Contents remain caller supplied.
pub type value_G_L4785 = u8;
pub const G_L4785: Global<value_G_L4785> = Global::new("FUN_140092e80", Some(0x140092e80));

/// Source symbol DAT_1401d00c8. Contents remain caller supplied.
pub type value_G_L4786 = u8;
pub const G_L4786: Global<value_G_L4786> = Global::new("DAT_1401d00c8", Some(0x1401d00c8));

/// Source symbol DAT_1401d00c0. Contents remain caller supplied.
pub type value_G_L4787 = i64;
pub const G_L4787: Global<value_G_L4787> = Global::new("DAT_1401d00c0", Some(0x1401d00c0));

/// Source symbol DAT_14011a2f0. Contents remain caller supplied.
pub type value_G_L4788 = u8;
pub const G_L4788: Global<value_G_L4788> = Global::new("DAT_14011a2f0", Some(0x14011a2f0));

/// Source symbol FUN_140093a80. Contents remain caller supplied.
pub type value_G_L4789 = u8;
pub const G_L4789: Global<value_G_L4789> = Global::new("FUN_140093a80", Some(0x140093a80));

/// Source symbol DAT_14011a2e0. Contents remain caller supplied.
pub type value_G_L4790 = u8;
pub const G_L4790: Global<value_G_L4790> = Global::new("DAT_14011a2e0", Some(0x14011a2e0));

/// Source symbol DAT_14011a418. Contents remain caller supplied.
pub type value_G_L4791 = u8;
pub const G_L4791: Global<value_G_L4791> = Global::new("DAT_14011a418", Some(0x14011a418));

/// Source symbol DAT_14011a298. Contents remain caller supplied.
pub type value_G_L4792 = u8;
pub const G_L4792: Global<value_G_L4792> = Global::new("DAT_14011a298", Some(0x14011a298));

/// Source symbol DAT_14011a2a8. Contents remain caller supplied.
pub type value_G_L4793 = u8;
pub const G_L4793: Global<value_G_L4793> = Global::new("DAT_14011a2a8", Some(0x14011a2a8));

/// Source symbol DAT_14011a238. Contents remain caller supplied.
pub type value_G_L4794 = u8;
pub const G_L4794: Global<value_G_L4794> = Global::new("DAT_14011a238", Some(0x14011a238));

/// Source symbol DAT_1401d00c0. Contents remain caller supplied.
pub type value_G_L4795 = u64;
pub const G_L4795: Global<value_G_L4795> = Global::new("DAT_1401d00c0", Some(0x1401d00c0));

/// Source symbol DAT_14011a300. Contents remain caller supplied.
pub type value_G_L4796 = u8;
pub const G_L4796: Global<value_G_L4796> = Global::new("DAT_14011a300", Some(0x14011a300));

/// Source symbol DAT_1401d00d0. Contents remain caller supplied.
pub type value_G_L4797 = u8;
pub const G_L4797: Global<value_G_L4797> = Global::new("DAT_1401d00d0", Some(0x1401d00d0));

/// Source symbol DAT_1401d01d0. Contents remain caller supplied.
pub type value_G_L4798 = u8;
pub const G_L4798: Global<value_G_L4798> = Global::new("DAT_1401d01d0", Some(0x1401d01d0));

/// Source symbol DAT_1401d01a0. Contents remain caller supplied.
pub type value_G_L4799 = u64;
pub const G_L4799: Global<value_G_L4799> = Global::new("DAT_1401d01a0", Some(0x1401d01a0));

/// Source symbol DAT_14011a428. Contents remain caller supplied.
pub type value_G_L4800 = u8;
pub const G_L4800: Global<value_G_L4800> = Global::new("DAT_14011a428", Some(0x14011a428));

/// Source symbol DAT_1401d01b0. Contents remain caller supplied.
pub type value_G_L4801 = u8;
pub const G_L4801: Global<value_G_L4801> = Global::new("DAT_1401d01b0", Some(0x1401d01b0));

/// Source symbol DAT_1401d00a8. Contents remain caller supplied.
pub type value_G_L4802 = u8;
pub const G_L4802: Global<value_G_L4802> = Global::new("DAT_1401d00a8", Some(0x1401d00a8));

/// Source symbol DAT_1401d00a0. Contents remain caller supplied.
pub type value_G_L4803 = u64;
pub const G_L4803: Global<value_G_L4803> = Global::new("DAT_1401d00a0", Some(0x1401d00a0));

/// Source symbol DAT_14011a6e8. Contents remain caller supplied.
pub type value_G_L4804 = u8;
pub const G_L4804: Global<value_G_L4804> = Global::new("DAT_14011a6e8", Some(0x14011a6e8));

/// Source symbol DAT_1401d00b0. Contents remain caller supplied.
pub type value_G_L4805 = u8;
pub const G_L4805: Global<value_G_L4805> = Global::new("DAT_1401d00b0", Some(0x1401d00b0));

/// Source symbol DAT_1401d0c70. Contents remain caller supplied.
pub type value_G_L4806 = u32;
pub const G_L4806: Global<value_G_L4806> = Global::new("DAT_1401d0c70", Some(0x1401d0c70));

/// Source symbol DAT_1401d0c78. Contents remain caller supplied.
pub type value_G_L4807 = Address<u64>;
pub const G_L4807: Global<value_G_L4807> = Global::new("DAT_1401d0c78", Some(0x1401d0c78));

/// Source symbol DAT_1401d0c70. Contents remain caller supplied.
pub type value_G_L4808 = i32;
pub const G_L4808: Global<value_G_L4808> = Global::new("DAT_1401d0c70", Some(0x1401d0c70));

/// Source symbol DAT_1401d0cd0. Contents remain caller supplied.
pub type value_G_L4809 = u64;
pub const G_L4809: Global<value_G_L4809> = Global::new("DAT_1401d0cd0", Some(0x1401d0cd0));

/// Source symbol DAT_1401d0ca0. Contents remain caller supplied.
pub type value_G_L4810 = u64;
pub const G_L4810: Global<value_G_L4810> = Global::new("DAT_1401d0ca0", Some(0x1401d0ca0));

/// Source symbol DAT_1401d0c98. Contents remain caller supplied.
pub type value_G_L4811 = i64;
pub const G_L4811: Global<value_G_L4811> = Global::new("DAT_1401d0c98", Some(0x1401d0c98));

/// Source symbol DAT_1401d0c90. Contents remain caller supplied.
pub type value_G_L4812 = i64;
pub const G_L4812: Global<value_G_L4812> = Global::new("DAT_1401d0c90", Some(0x1401d0c90));

/// Source symbol DAT_1401d0c88. Contents remain caller supplied.
pub type value_G_L4813 = i64;
pub const G_L4813: Global<value_G_L4813> = Global::new("DAT_1401d0c88", Some(0x1401d0c88));

/// Source symbol DAT_1401d0c98. Contents remain caller supplied.
pub type value_G_L4814 = u64;
pub const G_L4814: Global<value_G_L4814> = Global::new("DAT_1401d0c98", Some(0x1401d0c98));

/// Source symbol FUN_1400ca4f0. Contents remain caller supplied.
pub type value_G_L4815 = u8;
pub const G_L4815: Global<value_G_L4815> = Global::new("FUN_1400ca4f0", Some(0x1400ca4f0));

/// Source symbol FUN_1400ca500. Contents remain caller supplied.
pub type value_G_L4816 = u8;
pub const G_L4816: Global<value_G_L4816> = Global::new("FUN_1400ca500", Some(0x1400ca500));

/// Source symbol FUN_1400523e0. Contents remain caller supplied.
pub type value_G_L4817 = u8;
pub const G_L4817: Global<value_G_L4817> = Global::new("FUN_1400523e0", Some(0x1400523e0));

/// Source symbol FUN_140052370. Contents remain caller supplied.
pub type value_G_L4818 = u8;
pub const G_L4818: Global<value_G_L4818> = Global::new("FUN_140052370", Some(0x140052370));

/// Source symbol FUN_1400524f0. Contents remain caller supplied.
pub type value_G_L4819 = u8;
pub const G_L4819: Global<value_G_L4819> = Global::new("FUN_1400524f0", Some(0x1400524f0));

/// Source symbol FUN_140052460. Contents remain caller supplied.
pub type value_G_L4820 = u8;
pub const G_L4820: Global<value_G_L4820> = Global::new("FUN_140052460", Some(0x140052460));

/// Source symbol DAT_00000069. Contents remain caller supplied.
pub type value_G_L4821 = u8;
pub const G_L4821: Global<value_G_L4821> = Global::new("DAT_00000069", None);

/// Source symbol DAT_000000a4. Contents remain caller supplied.
pub type value_G_L4822 = u8;
pub const G_L4822: Global<value_G_L4822> = Global::new("DAT_000000a4", None);

/// Source symbol FUN_1400cacd0. Contents remain caller supplied.
pub type value_G_L4823 = u8;
pub const G_L4823: Global<value_G_L4823> = Global::new("FUN_1400cacd0", Some(0x1400cacd0));

/// Source symbol FUN_1400cace0. Contents remain caller supplied.
pub type value_G_L4824 = u8;
pub const G_L4824: Global<value_G_L4824> = Global::new("FUN_1400cace0", Some(0x1400cace0));

/// Source symbol DAT_14011ae10. Contents remain caller supplied.
pub type value_G_L4825 = u8;
pub const G_L4825: Global<value_G_L4825> = Global::new("DAT_14011ae10", Some(0x14011ae10));

/// Source symbol DAT_14011add0. Contents remain caller supplied.
pub type value_G_L4826 = u8;
pub const G_L4826: Global<value_G_L4826> = Global::new("DAT_14011add0", Some(0x14011add0));

/// Source symbol DAT_14011b4e8. Contents remain caller supplied.
pub type value_G_L4827 = u8;
pub const G_L4827: Global<value_G_L4827> = Global::new("DAT_14011b4e8", Some(0x14011b4e8));

/// Source symbol DAT_1401d0e28. Contents remain caller supplied.
pub type value_G_L4828 = u32;
pub const G_L4828: Global<value_G_L4828> = Global::new("DAT_1401d0e28", Some(0x1401d0e28));

/// Source symbol DAT_1401d0e30. Contents remain caller supplied.
pub type value_G_L4829 = u8;
pub const G_L4829: Global<value_G_L4829> = Global::new("DAT_1401d0e30", Some(0x1401d0e30));

/// Source symbol DAT_1401d0e40. Contents remain caller supplied.
pub type value_G_L4830 = u8;
pub const G_L4830: Global<value_G_L4830> = Global::new("DAT_1401d0e40", Some(0x1401d0e40));

/// Source symbol UNK_1401d0e38. Contents remain caller supplied.
pub type value_G_L4831 = u64;
pub const G_L4831: Global<value_G_L4831> = Global::new("UNK_1401d0e38", None);

/// Source symbol DAT_14011b508. Contents remain caller supplied.
pub type value_G_L4832 = u8;
pub const G_L4832: Global<value_G_L4832> = Global::new("DAT_14011b508", Some(0x14011b508));

/// Source symbol DAT_14011b490. Contents remain caller supplied.
pub type value_G_L4833 = u8;
pub const G_L4833: Global<value_G_L4833> = Global::new("DAT_14011b490", Some(0x14011b490));

/// Source symbol DAT_14011b760. Contents remain caller supplied.
pub type value_G_L4834 = u8;
pub const G_L4834: Global<value_G_L4834> = Global::new("DAT_14011b760", Some(0x14011b760));

/// Source symbol DAT_00000068. Contents remain caller supplied.
pub type value_G_L4835 = u8;
pub const G_L4835: Global<value_G_L4835> = Global::new("DAT_00000068", None);

/// Source symbol PTR_DAT_14011b7b0. Contents remain caller supplied.
pub type value_G_L4836 = Address<Unknown>;
pub const G_L4836: Global<value_G_L4836> = Global::new("PTR_DAT_14011b7b0", None);

/// Source symbol DAT_00000074. Contents remain caller supplied.
pub type value_G_L4837 = u8;
pub const G_L4837: Global<value_G_L4837> = Global::new("DAT_00000074", None);

/// Source symbol PTR_s_NumPreviewPages_14011b7a0. Contents remain caller supplied.
pub type value_G_L4838 = Address<Unknown>;
pub const G_L4838: Global<value_G_L4838> = Global::new("PTR_s_NumPreviewPages_14011b7a0", None);

/// Source symbol PTR_s_CurrentPage_14011b790. Contents remain caller supplied.
pub type value_G_L4839 = Address<Unknown>;
pub const G_L4839: Global<value_G_L4839> = Global::new("PTR_s_CurrentPage_14011b790", None);

/// Source symbol DAT_14011b780. Contents remain caller supplied.
pub type value_G_L4840 = u8;
pub const G_L4840: Global<value_G_L4840> = Global::new("DAT_14011b780", Some(0x14011b780));

/// Source symbol DAT_14011b6f0. Contents remain caller supplied.
pub type value_G_L4841 = u8;
pub const G_L4841: Global<value_G_L4841> = Global::new("DAT_14011b6f0", Some(0x14011b6f0));

/// Source symbol DAT_14011b918. Contents remain caller supplied.
pub type value_G_L4842 = u8;
pub const G_L4842: Global<value_G_L4842> = Global::new("DAT_14011b918", Some(0x14011b918));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4843 = Address<u8>;
pub const G_L4843: Global<value_G_L4843> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_14011baa8. Contents remain caller supplied.
pub type value_G_L4844 = u8;
pub const G_L4844: Global<value_G_L4844> = Global::new("DAT_14011baa8", Some(0x14011baa8));

/// Source symbol DAT_14011c9d0. Contents remain caller supplied.
pub type value_G_L4846 = u8;
pub const G_L4846: Global<value_G_L4846> = Global::new("DAT_14011c9d0", Some(0x14011c9d0));

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4847 = Address<Address<Address<Address<u64>>>>;
pub const G_L4847: Global<value_G_L4847> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4848 = Address<Address<Address<Address<u64>>>>;
pub const G_L4848: Global<value_G_L4848> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol FUN_1400cb910. Contents remain caller supplied.
pub type value_G_L4849 = u8;
pub const G_L4849: Global<value_G_L4849> = Global::new("FUN_1400cb910", Some(0x1400cb910));

/// Source symbol FUN_1400cb900. Contents remain caller supplied.
pub type value_G_L4850 = u8;
pub const G_L4850: Global<value_G_L4850> = Global::new("FUN_1400cb900", Some(0x1400cb900));

/// Source symbol DAT_140108230. Contents remain caller supplied.
pub type value_G_L4851 = u8;
pub const G_L4851: Global<value_G_L4851> = Global::new("DAT_140108230", Some(0x140108230));

/// Source symbol PTR_140108388. Contents remain caller supplied.
pub type value_G_L4852 = Address<Unknown>;
pub const G_L4852: Global<value_G_L4852> = Global::new("PTR_140108388", Some(0x140108388));

/// Source symbol DAT_1401d0128. Contents remain caller supplied.
pub type value_G_L4853 = u8;
pub const G_L4853: Global<value_G_L4853> = Global::new("DAT_1401d0128", Some(0x1401d0128));

/// Source symbol DAT_1401d0120. Contents remain caller supplied.
pub type value_G_L4854 = u64;
pub const G_L4854: Global<value_G_L4854> = Global::new("DAT_1401d0120", Some(0x1401d0120));

/// Source symbol DAT_1401d0308. Contents remain caller supplied.
pub type value_G_L4855 = u8;
pub const G_L4855: Global<value_G_L4855> = Global::new("DAT_1401d0308", Some(0x1401d0308));

/// Source symbol FUN_1400ad810. Contents remain caller supplied.
pub type value_G_L4856 = u8;
pub const G_L4856: Global<value_G_L4856> = Global::new("FUN_1400ad810", Some(0x1400ad810));

/// Source symbol DAT_1401d0300. Contents remain caller supplied.
pub type value_G_L4857 = u64;
pub const G_L4857: Global<value_G_L4857> = Global::new("DAT_1401d0300", Some(0x1401d0300));

/// Source symbol DAT_1401d0188. Contents remain caller supplied.
pub type value_G_L4858 = u8;
pub const G_L4858: Global<value_G_L4858> = Global::new("DAT_1401d0188", Some(0x1401d0188));

/// Source symbol DAT_1401d0180. Contents remain caller supplied.
pub type value_G_L4859 = i64;
pub const G_L4859: Global<value_G_L4859> = Global::new("DAT_1401d0180", Some(0x1401d0180));

/// Source symbol DAT_1401d06f8. Contents remain caller supplied.
pub type value_G_L4860 = u64;
pub const G_L4860: Global<value_G_L4860> = Global::new("DAT_1401d06f8", Some(0x1401d06f8));

/// Source symbol FUN_1400adcc0. Contents remain caller supplied.
pub type value_G_L4861 = u8;
pub const G_L4861: Global<value_G_L4861> = Global::new("FUN_1400adcc0", Some(0x1400adcc0));

/// Source symbol DAT_1401d0268. Contents remain caller supplied.
pub type value_G_L4862 = u8;
pub const G_L4862: Global<value_G_L4862> = Global::new("DAT_1401d0268", Some(0x1401d0268));

/// Source symbol DAT_1401d0260. Contents remain caller supplied.
pub type value_G_L4863 = u64;
pub const G_L4863: Global<value_G_L4863> = Global::new("DAT_1401d0260", Some(0x1401d0260));

/// Source symbol DAT_1401d0ba0. Contents remain caller supplied.
pub type value_G_L4864 = Address<Address<Address<Address<u64>>>>;
pub const G_L4864: Global<value_G_L4864> = Global::new("DAT_1401d0ba0", Some(0x1401d0ba0));

/// Source symbol DAT_1401d0b80. Contents remain caller supplied.
pub type value_G_L4865 = Address<Address<Address<u8>>>;
pub const G_L4865: Global<value_G_L4865> = Global::new("DAT_1401d0b80", Some(0x1401d0b80));

/// Source symbol DAT_1401d0168. Contents remain caller supplied.
pub type value_G_L4866 = u8;
pub const G_L4866: Global<value_G_L4866> = Global::new("DAT_1401d0168", Some(0x1401d0168));

/// Source symbol DAT_1401d0160. Contents remain caller supplied.
pub type value_G_L4867 = u64;
pub const G_L4867: Global<value_G_L4867> = Global::new("DAT_1401d0160", Some(0x1401d0160));

/// Source symbol DAT_1401d0108. Contents remain caller supplied.
pub type value_G_L4868 = u8;
pub const G_L4868: Global<value_G_L4868> = Global::new("DAT_1401d0108", Some(0x1401d0108));

/// Source symbol DAT_1401d0100. Contents remain caller supplied.
pub type value_G_L4869 = u64;
pub const G_L4869: Global<value_G_L4869> = Global::new("DAT_1401d0100", Some(0x1401d0100));

/// Source symbol FUN_14006e3f0. Contents remain caller supplied.
pub type value_G_L4870 = u8;
pub const G_L4870: Global<value_G_L4870> = Global::new("FUN_14006e3f0", Some(0x14006e3f0));

/// Source symbol DAT_14011c4e0. Contents remain caller supplied.
pub type value_G_L4871 = u8;
pub const G_L4871: Global<value_G_L4871> = Global::new("DAT_14011c4e0", Some(0x14011c4e0));

/// Source symbol DAT_1401d06c8. Contents remain caller supplied.
pub type value_G_L4872 = u8;
pub const G_L4872: Global<value_G_L4872> = Global::new("DAT_1401d06c8", Some(0x1401d06c8));

/// Source symbol FUN_1400a1110. Contents remain caller supplied.
pub type value_G_L4873 = u8;
pub const G_L4873: Global<value_G_L4873> = Global::new("FUN_1400a1110", Some(0x1400a1110));

/// Source symbol FUN_1400a3140. Contents remain caller supplied.
pub type value_G_L4874 = u8;
pub const G_L4874: Global<value_G_L4874> = Global::new("FUN_1400a3140", Some(0x1400a3140));

/// Source symbol FUN_1400acfa0. Contents remain caller supplied.
pub type value_G_L4875 = u8;
pub const G_L4875: Global<value_G_L4875> = Global::new("FUN_1400acfa0", Some(0x1400acfa0));

/// Source symbol FUN_140048210. Contents remain caller supplied.
pub type value_G_L4876 = u8;
pub const G_L4876: Global<value_G_L4876> = Global::new("FUN_140048210", Some(0x140048210));

/// Source symbol FUN_1400cb2c0. Contents remain caller supplied.
pub type value_G_L4878 = u8;
pub const G_L4878: Global<value_G_L4878> = Global::new("FUN_1400cb2c0", Some(0x1400cb2c0));

/// Source symbol FUN_1400cb2b0. Contents remain caller supplied.
pub type value_G_L4879 = u8;
pub const G_L4879: Global<value_G_L4879> = Global::new("FUN_1400cb2b0", Some(0x1400cb2b0));

/// Source symbol FUN_1400cb680. Contents remain caller supplied.
pub type value_G_L4880 = u8;
pub const G_L4880: Global<value_G_L4880> = Global::new("FUN_1400cb680", Some(0x1400cb680));

/// Source symbol FUN_1400cb670. Contents remain caller supplied.
pub type value_G_L4881 = u8;
pub const G_L4881: Global<value_G_L4881> = Global::new("FUN_1400cb670", Some(0x1400cb670));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4882 = Address<Address<Address<Address<Address<i64>>>>>;
pub const G_L4882: Global<value_G_L4882> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401d02a8. Contents remain caller supplied.
pub type value_G_L4883 = u8;
pub const G_L4883: Global<value_G_L4883> = Global::new("DAT_1401d02a8", Some(0x1401d02a8));

/// Source symbol DAT_1401d02a0. Contents remain caller supplied.
pub type value_G_L4884 = i64;
pub const G_L4884: Global<value_G_L4884> = Global::new("DAT_1401d02a0", Some(0x1401d02a0));

/// Source symbol DAT_1401d0288. Contents remain caller supplied.
pub type value_G_L4885 = u8;
pub const G_L4885: Global<value_G_L4885> = Global::new("DAT_1401d0288", Some(0x1401d0288));

/// Source symbol DAT_1401d0280. Contents remain caller supplied.
pub type value_G_L4886 = i64;
pub const G_L4886: Global<value_G_L4886> = Global::new("DAT_1401d0280", Some(0x1401d0280));

/// Source symbol DAT_1401d0b80. Contents remain caller supplied.
pub type value_G_L4887 = Address<Address<Address<Address<Address<i64>>>>>;
pub const G_L4887: Global<value_G_L4887> = Global::new("DAT_1401d0b80", Some(0x1401d0b80));

/// Source symbol DAT_1401d01e8. Contents remain caller supplied.
pub type value_G_L4888 = u8;
pub const G_L4888: Global<value_G_L4888> = Global::new("DAT_1401d01e8", Some(0x1401d01e8));

/// Source symbol DAT_1401d01e0. Contents remain caller supplied.
pub type value_G_L4889 = u64;
pub const G_L4889: Global<value_G_L4889> = Global::new("DAT_1401d01e0", Some(0x1401d01e0));

/// Source symbol FUN_1400a3320. Contents remain caller supplied.
pub type value_G_L4890 = u8;
pub const G_L4890: Global<value_G_L4890> = Global::new("FUN_1400a3320", Some(0x1400a3320));

/// Source symbol FUN_1400a33e0. Contents remain caller supplied.
pub type value_G_L4891 = u8;
pub const G_L4891: Global<value_G_L4891> = Global::new("FUN_1400a33e0", Some(0x1400a33e0));

/// Source symbol DAT_1401d0248. Contents remain caller supplied.
pub type value_G_L4892 = u8;
pub const G_L4892: Global<value_G_L4892> = Global::new("DAT_1401d0248", Some(0x1401d0248));

/// Source symbol DAT_1401d0240. Contents remain caller supplied.
pub type value_G_L4893 = u64;
pub const G_L4893: Global<value_G_L4893> = Global::new("DAT_1401d0240", Some(0x1401d0240));

/// Source symbol DAT_1401d02c8. Contents remain caller supplied.
pub type value_G_L4894 = u8;
pub const G_L4894: Global<value_G_L4894> = Global::new("DAT_1401d02c8", Some(0x1401d02c8));

/// Source symbol DAT_1401d02c0. Contents remain caller supplied.
pub type value_G_L4895 = u64;
pub const G_L4895: Global<value_G_L4895> = Global::new("DAT_1401d02c0", Some(0x1401d02c0));

/// Source symbol DAT_1401d0208. Contents remain caller supplied.
pub type value_G_L4896 = u8;
pub const G_L4896: Global<value_G_L4896> = Global::new("DAT_1401d0208", Some(0x1401d0208));

/// Source symbol DAT_1401d0200. Contents remain caller supplied.
pub type value_G_L4897 = u64;
pub const G_L4897: Global<value_G_L4897> = Global::new("DAT_1401d0200", Some(0x1401d0200));

/// Source symbol DAT_1401d02e8. Contents remain caller supplied.
pub type value_G_L4898 = u8;
pub const G_L4898: Global<value_G_L4898> = Global::new("DAT_1401d02e8", Some(0x1401d02e8));

/// Source symbol DAT_1401d02e0. Contents remain caller supplied.
pub type value_G_L4899 = u64;
pub const G_L4899: Global<value_G_L4899> = Global::new("DAT_1401d02e0", Some(0x1401d02e0));

/// Source symbol DAT_14011c4a0. Contents remain caller supplied.
pub type value_G_L4900 = u8;
pub const G_L4900: Global<value_G_L4900> = Global::new("DAT_14011c4a0", Some(0x14011c4a0));

/// Source symbol DAT_14011c4b0. Contents remain caller supplied.
pub type value_G_L4901 = u8;
pub const G_L4901: Global<value_G_L4901> = Global::new("DAT_14011c4b0", Some(0x14011c4b0));

/// Source symbol DAT_14011c4c0. Contents remain caller supplied.
pub type value_G_L4902 = u8;
pub const G_L4902: Global<value_G_L4902> = Global::new("DAT_14011c4c0", Some(0x14011c4c0));

/// Source symbol DAT_14011c4f0. Contents remain caller supplied.
pub type value_G_L4903 = u8;
pub const G_L4903: Global<value_G_L4903> = Global::new("DAT_14011c4f0", Some(0x14011c4f0));

/// Source symbol DAT_1401d0b70. Contents remain caller supplied.
pub type value_G_L4905 = Address<i64>;
pub const G_L4905: Global<value_G_L4905> = Global::new("DAT_1401d0b70", Some(0x1401d0b70));

/// Source symbol DAT_1401d0228. Contents remain caller supplied.
pub type value_G_L4906 = u8;
pub const G_L4906: Global<value_G_L4906> = Global::new("DAT_1401d0228", Some(0x1401d0228));

/// Source symbol DAT_1401d0220. Contents remain caller supplied.
pub type value_G_L4907 = i64;
pub const G_L4907: Global<value_G_L4907> = Global::new("DAT_1401d0220", Some(0x1401d0220));

/// Source symbol DAT_00000088. Contents remain caller supplied.
pub type value_G_L4908 = u8;
pub const G_L4908: Global<value_G_L4908> = Global::new("DAT_00000088", None);

/// Source symbol DAT_000000b8. Contents remain caller supplied.
pub type value_G_L4909 = u8;
pub const G_L4909: Global<value_G_L4909> = Global::new("DAT_000000b8", None);

/// Source symbol DAT_000003f0. Contents remain caller supplied.
pub type value_G_L4910 = u8;
pub const G_L4910: Global<value_G_L4910> = Global::new("DAT_000003f0", None);

/// Source symbol DAT_1401d00e8. Contents remain caller supplied.
pub type value_G_L4911 = u8;
pub const G_L4911: Global<value_G_L4911> = Global::new("DAT_1401d00e8", Some(0x1401d00e8));

/// Source symbol DAT_1401d00e0. Contents remain caller supplied.
pub type value_G_L4912 = i64;
pub const G_L4912: Global<value_G_L4912> = Global::new("DAT_1401d00e0", Some(0x1401d00e0));

/// Source symbol DAT_14011c888. Contents remain caller supplied.
pub type value_G_L4913 = u8;
pub const G_L4913: Global<value_G_L4913> = Global::new("DAT_14011c888", Some(0x14011c888));

/// Source symbol DAT_14011c898. Contents remain caller supplied.
pub type value_G_L4914 = u8;
pub const G_L4914: Global<value_G_L4914> = Global::new("DAT_14011c898", Some(0x14011c898));

/// Source symbol DAT_1401d08f0. Contents remain caller supplied.
pub type value_G_L4915 = u32;
pub const G_L4915: Global<value_G_L4915> = Global::new("DAT_1401d08f0", Some(0x1401d08f0));

/// Source symbol DAT_1401d08f4. Contents remain caller supplied.
pub type value_G_L4916 = u32;
pub const G_L4916: Global<value_G_L4916> = Global::new("DAT_1401d08f4", Some(0x1401d08f4));

/// Source symbol DAT_1401d08f8. Contents remain caller supplied.
pub type value_G_L4917 = u32;
pub const G_L4917: Global<value_G_L4917> = Global::new("DAT_1401d08f8", Some(0x1401d08f8));

/// Source symbol DAT_1401d08fc. Contents remain caller supplied.
pub type value_G_L4918 = u32;
pub const G_L4918: Global<value_G_L4918> = Global::new("DAT_1401d08fc", Some(0x1401d08fc));

/// Source symbol FUN_140095d50. Contents remain caller supplied.
pub type value_G_L4919 = u8;
pub const G_L4919: Global<value_G_L4919> = Global::new("FUN_140095d50", Some(0x140095d50));

/// Source symbol FUN_1400a32f0. Contents remain caller supplied.
pub type value_G_L4920 = u8;
pub const G_L4920: Global<value_G_L4920> = Global::new("FUN_1400a32f0", Some(0x1400a32f0));

/// Source symbol FUN_1400a38d0. Contents remain caller supplied.
pub type value_G_L4921 = u8;
pub const G_L4921: Global<value_G_L4921> = Global::new("FUN_1400a38d0", Some(0x1400a38d0));

/// Source symbol FUN_1400a3520. Contents remain caller supplied.
pub type value_G_L4922 = u8;
pub const G_L4922: Global<value_G_L4922> = Global::new("FUN_1400a3520", Some(0x1400a3520));

/// Source symbol FUN_1400a3770. Contents remain caller supplied.
pub type value_G_L4923 = u8;
pub const G_L4923: Global<value_G_L4923> = Global::new("FUN_1400a3770", Some(0x1400a3770));

/// Source symbol DAT_000003f4. Contents remain caller supplied.
pub type value_G_L4924 = u8;
pub const G_L4924: Global<value_G_L4924> = Global::new("DAT_000003f4", None);

/// Source symbol DAT_140179144. Contents remain caller supplied.
pub type value_G_L4925 = u32;
pub const G_L4925: Global<value_G_L4925> = Global::new("DAT_140179144", Some(0x140179144));

/// Source symbol DAT_140179140. Contents remain caller supplied.
pub type value_G_L4926 = u32;
pub const G_L4926: Global<value_G_L4926> = Global::new("DAT_140179140", Some(0x140179140));

/// Source symbol PTR_s_ActiveColorSlot_140109518. Contents remain caller supplied.
pub type value_G_L4927 = Address<Unknown>;
pub const G_L4927: Global<value_G_L4927> = Global::new("PTR_s_ActiveColorSlot_140109518", None);

/// Source symbol DAT_1401d085a. Contents remain caller supplied.
pub type value_G_L4928 = u8;
pub const G_L4928: Global<value_G_L4928> = Global::new("DAT_1401d085a", Some(0x1401d085a));

/// Source symbol DAT_14011cb68. Contents remain caller supplied.
pub type value_G_L4929 = u8;
pub const G_L4929: Global<value_G_L4929> = Global::new("DAT_14011cb68", Some(0x14011cb68));

/// Source symbol DAT_14011c808. Contents remain caller supplied.
pub type value_G_L4930 = u8;
pub const G_L4930: Global<value_G_L4930> = Global::new("DAT_14011c808", Some(0x14011c808));

/// Source symbol DAT_14011c420. Contents remain caller supplied.
pub type value_G_L4931 = u8;
pub const G_L4931: Global<value_G_L4931> = Global::new("DAT_14011c420", Some(0x14011c420));

/// Source symbol DAT_1401d0b78. Contents remain caller supplied.
pub type value_G_L4932 = Address<Unknown>;
pub const G_L4932: Global<value_G_L4932> = Global::new("DAT_1401d0b78", Some(0x1401d0b78));

/// Source symbol DAT_14011bf78. Contents remain caller supplied.
pub type value_G_L4933 = u8;
pub const G_L4933: Global<value_G_L4933> = Global::new("DAT_14011bf78", Some(0x14011bf78));

/// Source symbol DAT_14011bf68. Contents remain caller supplied.
pub type value_G_L4934 = u8;
pub const G_L4934: Global<value_G_L4934> = Global::new("DAT_14011bf68", Some(0x14011bf68));

/// Source symbol DAT_14011bf58. Contents remain caller supplied.
pub type value_G_L4935 = u8;
pub const G_L4935: Global<value_G_L4935> = Global::new("DAT_14011bf58", Some(0x14011bf58));

/// Source symbol DAT_14011bf38. Contents remain caller supplied.
pub type value_G_L4936 = u8;
pub const G_L4936: Global<value_G_L4936> = Global::new("DAT_14011bf38", Some(0x14011bf38));

/// Source symbol DAT_14011bf48. Contents remain caller supplied.
pub type value_G_L4937 = u8;
pub const G_L4937: Global<value_G_L4937> = Global::new("DAT_14011bf48", Some(0x14011bf48));

/// Source symbol DAT_14011c4d0. Contents remain caller supplied.
pub type value_G_L4938 = u8;
pub const G_L4938: Global<value_G_L4938> = Global::new("DAT_14011c4d0", Some(0x14011c4d0));

/// Source symbol DAT_14011c500. Contents remain caller supplied.
pub type value_G_L4939 = u8;
pub const G_L4939: Global<value_G_L4939> = Global::new("DAT_14011c500", Some(0x14011c500));

/// Source symbol DAT_14011c510. Contents remain caller supplied.
pub type value_G_L4940 = u8;
pub const G_L4940: Global<value_G_L4940> = Global::new("DAT_14011c510", Some(0x14011c510));

/// Source symbol DAT_14011c570. Contents remain caller supplied.
pub type value_G_L4941 = u8;
pub const G_L4941: Global<value_G_L4941> = Global::new("DAT_14011c570", Some(0x14011c570));

/// Source symbol DAT_1401d00e0. Contents remain caller supplied.
pub type value_G_L4942 = u64;
pub const G_L4942: Global<value_G_L4942> = Global::new("DAT_1401d00e0", Some(0x1401d00e0));

/// Source symbol DAT_14011cbd8. Contents remain caller supplied.
pub type value_G_L4943 = u8;
pub const G_L4943: Global<value_G_L4943> = Global::new("DAT_14011cbd8", Some(0x14011cbd8));

/// Source symbol DAT_1401d00f0. Contents remain caller supplied.
pub type value_G_L4944 = u8;
pub const G_L4944: Global<value_G_L4944> = Global::new("DAT_1401d00f0", Some(0x1401d00f0));

/// Source symbol DAT_1401d0220. Contents remain caller supplied.
pub type value_G_L4945 = u64;
pub const G_L4945: Global<value_G_L4945> = Global::new("DAT_1401d0220", Some(0x1401d0220));

/// Source symbol DAT_14011cc40. Contents remain caller supplied.
pub type value_G_L4946 = u8;
pub const G_L4946: Global<value_G_L4946> = Global::new("DAT_14011cc40", Some(0x14011cc40));

/// Source symbol DAT_1401d0230. Contents remain caller supplied.
pub type value_G_L4947 = u8;
pub const G_L4947: Global<value_G_L4947> = Global::new("DAT_1401d0230", Some(0x1401d0230));

/// Source symbol DAT_14011cca0. Contents remain caller supplied.
pub type value_G_L4948 = u8;
pub const G_L4948: Global<value_G_L4948> = Global::new("DAT_14011cca0", Some(0x14011cca0));

/// Source symbol DAT_1401d02f0. Contents remain caller supplied.
pub type value_G_L4949 = u8;
pub const G_L4949: Global<value_G_L4949> = Global::new("DAT_1401d02f0", Some(0x1401d02f0));

/// Source symbol DAT_14011cce8. Contents remain caller supplied.
pub type value_G_L4950 = u8;
pub const G_L4950: Global<value_G_L4950> = Global::new("DAT_14011cce8", Some(0x14011cce8));

/// Source symbol DAT_1401d0210. Contents remain caller supplied.
pub type value_G_L4951 = u8;
pub const G_L4951: Global<value_G_L4951> = Global::new("DAT_1401d0210", Some(0x1401d0210));

/// Source symbol DAT_14011cd38. Contents remain caller supplied.
pub type value_G_L4952 = u8;
pub const G_L4952: Global<value_G_L4952> = Global::new("DAT_14011cd38", Some(0x14011cd38));

/// Source symbol DAT_1401d02d0. Contents remain caller supplied.
pub type value_G_L4953 = u8;
pub const G_L4953: Global<value_G_L4953> = Global::new("DAT_1401d02d0", Some(0x1401d02d0));

/// Source symbol DAT_14011cd80. Contents remain caller supplied.
pub type value_G_L4954 = u8;
pub const G_L4954: Global<value_G_L4954> = Global::new("DAT_14011cd80", Some(0x14011cd80));

/// Source symbol DAT_1401d0250. Contents remain caller supplied.
pub type value_G_L4955 = u8;
pub const G_L4955: Global<value_G_L4955> = Global::new("DAT_1401d0250", Some(0x1401d0250));

/// Source symbol DAT_14011cdd0. Contents remain caller supplied.
pub type value_G_L4956 = u8;
pub const G_L4956: Global<value_G_L4956> = Global::new("DAT_14011cdd0", Some(0x14011cdd0));

/// Source symbol DAT_1401d01f0. Contents remain caller supplied.
pub type value_G_L4957 = u8;
pub const G_L4957: Global<value_G_L4957> = Global::new("DAT_1401d01f0", Some(0x1401d01f0));

/// Source symbol DAT_1401d0280. Contents remain caller supplied.
pub type value_G_L4958 = u64;
pub const G_L4958: Global<value_G_L4958> = Global::new("DAT_1401d0280", Some(0x1401d0280));

/// Source symbol DAT_14011ce18. Contents remain caller supplied.
pub type value_G_L4959 = u8;
pub const G_L4959: Global<value_G_L4959> = Global::new("DAT_14011ce18", Some(0x14011ce18));

/// Source symbol DAT_1401d0290. Contents remain caller supplied.
pub type value_G_L4960 = u8;
pub const G_L4960: Global<value_G_L4960> = Global::new("DAT_1401d0290", Some(0x1401d0290));

/// Source symbol DAT_1401d02a0. Contents remain caller supplied.
pub type value_G_L4961 = u64;
pub const G_L4961: Global<value_G_L4961> = Global::new("DAT_1401d02a0", Some(0x1401d02a0));

/// Source symbol DAT_14011ce80. Contents remain caller supplied.
pub type value_G_L4962 = u8;
pub const G_L4962: Global<value_G_L4962> = Global::new("DAT_14011ce80", Some(0x14011ce80));

/// Source symbol DAT_1401d02b0. Contents remain caller supplied.
pub type value_G_L4963 = u8;
pub const G_L4963: Global<value_G_L4963> = Global::new("DAT_1401d02b0", Some(0x1401d02b0));

/// Source symbol DAT_1401d0110. Contents remain caller supplied.
pub type value_G_L4964 = u8;
pub const G_L4964: Global<value_G_L4964> = Global::new("DAT_1401d0110", Some(0x1401d0110));

/// Source symbol DAT_14011cef8. Contents remain caller supplied.
pub type value_G_L4965 = u8;
pub const G_L4965: Global<value_G_L4965> = Global::new("DAT_14011cef8", Some(0x14011cef8));

/// Source symbol DAT_14011cf08. Contents remain caller supplied.
pub type value_G_L4966 = u8;
pub const G_L4966: Global<value_G_L4966> = Global::new("DAT_14011cf08", Some(0x14011cf08));

/// Source symbol DAT_1401d0170. Contents remain caller supplied.
pub type value_G_L4967 = u8;
pub const G_L4967: Global<value_G_L4967> = Global::new("DAT_1401d0170", Some(0x1401d0170));

/// Source symbol DAT_1401d0148. Contents remain caller supplied.
pub type value_G_L4968 = u8;
pub const G_L4968: Global<value_G_L4968> = Global::new("DAT_1401d0148", Some(0x1401d0148));

/// Source symbol DAT_1401d0140. Contents remain caller supplied.
pub type value_G_L4969 = u64;
pub const G_L4969: Global<value_G_L4969> = Global::new("DAT_1401d0140", Some(0x1401d0140));

/// Source symbol DAT_1401d0150. Contents remain caller supplied.
pub type value_G_L4970 = u8;
pub const G_L4970: Global<value_G_L4970> = Global::new("DAT_1401d0150", Some(0x1401d0150));

/// Source symbol DAT_14011d028. Contents remain caller supplied.
pub type value_G_L4971 = u8;
pub const G_L4971: Global<value_G_L4971> = Global::new("DAT_14011d028", Some(0x14011d028));

/// Source symbol FUN_1400ad510. Contents remain caller supplied.
pub type value_G_L4972 = u8;
pub const G_L4972: Global<value_G_L4972> = Global::new("FUN_1400ad510", Some(0x1400ad510));

/// Source symbol DAT_1401d0270. Contents remain caller supplied.
pub type value_G_L4973 = u8;
pub const G_L4973: Global<value_G_L4973> = Global::new("DAT_1401d0270", Some(0x1401d0270));

/// Source symbol DAT_14011cf90. Contents remain caller supplied.
pub type value_G_L4974 = u8;
pub const G_L4974: Global<value_G_L4974> = Global::new("DAT_14011cf90", Some(0x14011cf90));

/// Source symbol DAT_1401d0180. Contents remain caller supplied.
pub type value_G_L4975 = u64;
pub const G_L4975: Global<value_G_L4975> = Global::new("DAT_1401d0180", Some(0x1401d0180));

/// Source symbol DAT_14011cfa0. Contents remain caller supplied.
pub type value_G_L4976 = u8;
pub const G_L4976: Global<value_G_L4976> = Global::new("DAT_14011cfa0", Some(0x14011cfa0));

/// Source symbol DAT_1401d0190. Contents remain caller supplied.
pub type value_G_L4977 = u8;
pub const G_L4977: Global<value_G_L4977> = Global::new("DAT_1401d0190", Some(0x1401d0190));

/// Source symbol DAT_1401d0310. Contents remain caller supplied.
pub type value_G_L4978 = u8;
pub const G_L4978: Global<value_G_L4978> = Global::new("DAT_1401d0310", Some(0x1401d0310));

/// Source symbol DAT_14011d018. Contents remain caller supplied.
pub type value_G_L4979 = u8;
pub const G_L4979: Global<value_G_L4979> = Global::new("DAT_14011d018", Some(0x14011d018));

/// Source symbol DAT_14011d038. Contents remain caller supplied.
pub type value_G_L4980 = u8;
pub const G_L4980: Global<value_G_L4980> = Global::new("DAT_14011d038", Some(0x14011d038));

/// Source symbol DAT_1401d0130. Contents remain caller supplied.
pub type value_G_L4981 = u8;
pub const G_L4981: Global<value_G_L4981> = Global::new("DAT_1401d0130", Some(0x1401d0130));

/// Source symbol ImgDelayDescr@14016af40. Contents remain caller supplied.
pub type value_G_L4982 = t_959541cecaf0;
pub const G_L4982: Global<value_G_L4982> =
    Global::new("ImgDelayDescr@14016af40", Some(0x14016af40));

/// Source symbol DAT_1401cf430. Contents remain caller supplied.
pub type value_G_L4983 = Address<u8>;
pub const G_L4983: Global<value_G_L4983> = Global::new("DAT_1401cf430", Some(0x1401cf430));

/// Source symbol DAT_1401cf438. Contents remain caller supplied.
pub type value_G_L4984 = Address<u8>;
pub const G_L4984: Global<value_G_L4984> = Global::new("DAT_1401cf438", Some(0x1401cf438));

/// Source symbol DAT_1401cf440. Contents remain caller supplied.
pub type value_G_L4985 = u64;
pub const G_L4985: Global<value_G_L4985> = Global::new("DAT_1401cf440", Some(0x1401cf440));

/// Source symbol DAT_1401cf428. Contents remain caller supplied.
pub type value_G_L4986 = Address<t_4c0908fcb2fb>;
pub const G_L4986: Global<value_G_L4986> = Global::new("DAT_1401cf428", Some(0x1401cf428));

/// Source symbol DAT_1401cf430. Contents remain caller supplied.
pub type value_G_L4987 = CodeAddress;
pub const G_L4987: Global<value_G_L4987> = Global::new("DAT_1401cf430", Some(0x1401cf430));

/// Source symbol DAT_1401cf438. Contents remain caller supplied.
pub type value_G_L4988 = CodeAddress;
pub const G_L4988: Global<value_G_L4988> = Global::new("DAT_1401cf438", Some(0x1401cf438));

/// Source symbol IMAGE_SECTION_HEADER_140000230. Contents remain caller supplied.
pub type value_G_L4989 = t_d6601bc9eca9;
pub const G_L4989: Global<value_G_L4989> =
    Global::new("IMAGE_SECTION_HEADER_140000230", Some(0x140000230));

/// Source symbol DAT_1401cf450. Contents remain caller supplied.
pub type value_G_L4990 = i32;
pub const G_L4990: Global<value_G_L4990> = Global::new("DAT_1401cf450", Some(0x1401cf450));

/// Source symbol FUN_140002e40. Contents remain caller supplied.
pub type value_G_L4991 = u8;
pub const G_L4991: Global<value_G_L4991> = Global::new("FUN_140002e40", Some(0x140002e40));

/// Source symbol FUN_1400b0ba0. Contents remain caller supplied.
pub type value_G_L4992 = u8;
pub const G_L4992: Global<value_G_L4992> = Global::new("FUN_1400b0ba0", Some(0x1400b0ba0));

/// Source symbol DAT_1401cf470. Contents remain caller supplied.
pub type value_G_L4993 = i32;
pub const G_L4993: Global<value_G_L4993> = Global::new("DAT_1401cf470", Some(0x1401cf470));

/// Source symbol DAT_1400e2a98. Contents remain caller supplied.
pub type value_G_L4994 = u8;
pub const G_L4994: Global<value_G_L4994> = Global::new("DAT_1400e2a98", Some(0x1400e2a98));

/// Source symbol DAT_1400e2b20. Contents remain caller supplied.
pub type value_G_L4995 = u8;
pub const G_L4995: Global<value_G_L4995> = Global::new("DAT_1400e2b20", Some(0x1400e2b20));

/// Source symbol DAT_1400e2b38. Contents remain caller supplied.
pub type value_G_L4996 = u8;
pub const G_L4996: Global<value_G_L4996> = Global::new("DAT_1400e2b38", Some(0x1400e2b38));

/// Source symbol DAT_140179040. Contents remain caller supplied.
pub type value_G_L4997 = u64;
pub const G_L4997: Global<value_G_L4997> = Global::new("DAT_140179040", Some(0x140179040));

/// Source symbol DAT_1401cf460. Contents remain caller supplied.
pub type value_G_L4998 = u8;
pub const G_L4998: Global<value_G_L4998> = Global::new("DAT_1401cf460", Some(0x1401cf460));

/// Source symbol DAT_1401cf468. Contents remain caller supplied.
pub type value_G_L4999 = u8;
pub const G_L4999: Global<value_G_L4999> = Global::new("DAT_1401cf468", Some(0x1401cf468));

/// Source symbol DAT_140179000. Contents remain caller supplied.
pub type value_G_L5000 = i32;
pub const G_L5000: Global<value_G_L5000> = Global::new("DAT_140179000", Some(0x140179000));

/// Source symbol _tls_index. Contents remain caller supplied.
pub type value_G_L5001 = u32;
pub const G_L5001: Global<value_G_L5001> = Global::new("_tls_index", None);

/// Source symbol DAT_140179000. Contents remain caller supplied.
pub type value_G_L5002 = u32;
pub const G_L5002: Global<value_G_L5002> = Global::new("DAT_140179000", Some(0x140179000));

/// Source symbol DAT_1401cf478. Contents remain caller supplied.
pub type value_G_L5003 = Address<Unknown>;
pub const G_L5003: Global<value_G_L5003> = Global::new("DAT_1401cf478", Some(0x1401cf478));

/// Source symbol StackBase. Contents remain caller supplied.
pub type value_G_L5004 = Address<Unknown>;
pub const G_L5004: Global<value_G_L5004> = Global::new("StackBase", None);

/// Source symbol DAT_1401cf480. Contents remain caller supplied.
pub type value_G_L5005 = u8;
pub const G_L5005: Global<value_G_L5005> = Global::new("DAT_1401cf480", Some(0x1401cf480));

/// Source symbol DAT_1401cf481. Contents remain caller supplied.
pub type value_G_L5006 = i8;
pub const G_L5006: Global<value_G_L5006> = Global::new("DAT_1401cf481", Some(0x1401cf481));

/// Source symbol DAT_1401cf488. Contents remain caller supplied.
pub type value_G_L5007 = u64;
pub const G_L5007: Global<value_G_L5007> = Global::new("DAT_1401cf488", Some(0x1401cf488));

/// Source symbol DAT_1401cf498. Contents remain caller supplied.
pub type value_G_L5008 = u8;
pub const G_L5008: Global<value_G_L5008> = Global::new("DAT_1401cf498", Some(0x1401cf498));

/// Source symbol DAT_1401cf4a0. Contents remain caller supplied.
pub type value_G_L5009 = u8;
pub const G_L5009: Global<value_G_L5009> = Global::new("DAT_1401cf4a0", Some(0x1401cf4a0));

/// Source symbol DAT_1401cf4b0. Contents remain caller supplied.
pub type value_G_L5010 = u8;
pub const G_L5010: Global<value_G_L5010> = Global::new("DAT_1401cf4b0", Some(0x1401cf4b0));

/// Source symbol UNK_1401cf490. Contents remain caller supplied.
pub type value_G_L5011 = u64;
pub const G_L5011: Global<value_G_L5011> = Global::new("UNK_1401cf490", None);

/// Source symbol UNK_1401cf4a8. Contents remain caller supplied.
pub type value_G_L5012 = u64;
pub const G_L5012: Global<value_G_L5012> = Global::new("UNK_1401cf4a8", None);

/// Source symbol DAT_140000348. Contents remain caller supplied.
pub type value_G_L5013 = u8;
pub const G_L5013: Global<value_G_L5013> = Global::new("DAT_140000348", Some(0x140000348));

/// Source symbol DAT_1401cf478. Contents remain caller supplied.
pub type value_G_L5014 = u64;
pub const G_L5014: Global<value_G_L5014> = Global::new("DAT_1401cf478", Some(0x1401cf478));

/// Source symbol DAT_1401cf480. Contents remain caller supplied.
pub type value_G_L5015 = i8;
pub const G_L5015: Global<value_G_L5015> = Global::new("DAT_1401cf480", Some(0x1401cf480));

/// Source symbol DAT_1401cf658. Contents remain caller supplied.
pub type value_G_L5016 = u64;
pub const G_L5016: Global<value_G_L5016> = Global::new("DAT_1401cf658", Some(0x1401cf658));

/// Source symbol DAT_1401cf5f8. Contents remain caller supplied.
pub type value_G_L5017 = u8;
pub const G_L5017: Global<value_G_L5017> = Global::new("DAT_1401cf5f8", Some(0x1401cf5f8));

/// Source symbol DAT_1401cf4d0. Contents remain caller supplied.
pub type value_G_L5018 = u8;
pub const G_L5018: Global<value_G_L5018> = Global::new("DAT_1401cf4d0", Some(0x1401cf4d0));

/// Source symbol DAT_1401cf5e0. Contents remain caller supplied.
pub type value_G_L5019 = u8;
pub const G_L5019: Global<value_G_L5019> = Global::new("DAT_1401cf5e0", Some(0x1401cf5e0));

/// Source symbol DAT_1401cf4c0. Contents remain caller supplied.
pub type value_G_L5020 = u8;
pub const G_L5020: Global<value_G_L5020> = Global::new("DAT_1401cf4c0", Some(0x1401cf4c0));

/// Source symbol DAT_1401cf4c4. Contents remain caller supplied.
pub type value_G_L5021 = u8;
pub const G_L5021: Global<value_G_L5021> = Global::new("DAT_1401cf4c4", Some(0x1401cf4c4));

/// Source symbol DAT_1401cf4d8. Contents remain caller supplied.
pub type value_G_L5022 = u8;
pub const G_L5022: Global<value_G_L5022> = Global::new("DAT_1401cf4d8", Some(0x1401cf4d8));

/// Source symbol DAT_140179080. Contents remain caller supplied.
pub type value_G_L5023 = u64;
pub const G_L5023: Global<value_G_L5023> = Global::new("DAT_140179080", Some(0x140179080));

/// Source symbol DAT_1401cf4e0. Contents remain caller supplied.
pub type value_G_L5024 = u64;
pub const G_L5024: Global<value_G_L5024> = Global::new("DAT_1401cf4e0", Some(0x1401cf4e0));

/// Source symbol UNK_1400b077d. Contents remain caller supplied.
pub type value_G_L5025 = u8;
pub const G_L5025: Global<value_G_L5025> = Global::new("UNK_1400b077d", None);

/// Source symbol PTR_DAT_1400e4d60. Contents remain caller supplied.
pub type value_G_L5026 = Address<Unknown>;
pub const G_L5026: Global<value_G_L5026> = Global::new("PTR_DAT_1400e4d60", None);

/// Source symbol DAT_1401cf560. Contents remain caller supplied.
pub type value_G_L5027 = u8;
pub const G_L5027: Global<value_G_L5027> = Global::new("DAT_1401cf560", Some(0x1401cf560));

/// Source symbol DAT_140179080. Contents remain caller supplied.
pub type value_G_L5028 = u64;
pub const G_L5028: Global<value_G_L5028> = Global::new("DAT_140179080", Some(0x140179080));

/// Source symbol DAT_1401cfa30. Contents remain caller supplied.
pub type value_G_L5029 = u8;
pub const G_L5029: Global<value_G_L5029> = Global::new("DAT_1401cfa30", Some(0x1401cfa30));

/// Source symbol DAT_1401cfa40. Contents remain caller supplied.
pub type value_G_L5030 = u8;
pub const G_L5030: Global<value_G_L5030> = Global::new("DAT_1401cfa40", Some(0x1401cfa40));

/// Source symbol DAT_140179088. Contents remain caller supplied.
pub type value_G_L5031 = i32;
pub const G_L5031: Global<value_G_L5031> = Global::new("DAT_140179088", Some(0x140179088));

/// Source symbol DAT_1401d0e98. Contents remain caller supplied.
pub type value_G_L5032 = u8;
pub const G_L5032: Global<value_G_L5032> = Global::new("DAT_1401d0e98", Some(0x1401d0e98));

/// Source symbol DAT_1401d0e90. Contents remain caller supplied.
pub type value_G_L5033 = u8;
pub const G_L5033: Global<value_G_L5033> = Global::new("DAT_1401d0e90", Some(0x1401d0e90));

/// Source symbol DAT_1401cfa48. Contents remain caller supplied.
pub type value_G_L5034 = u8;
pub const G_L5034: Global<value_G_L5034> = Global::new("DAT_1401cfa48", Some(0x1401cfa48));

/// Source symbol FUN_1400b0b00. Contents remain caller supplied.
pub type value_G_L5035 = u8;
pub const G_L5035: Global<value_G_L5035> = Global::new("FUN_1400b0b00", Some(0x1400b0b00));

/// Source symbol DAT_140155078. Contents remain caller supplied.
pub type value_G_L5036 = u64;
pub const G_L5036: Global<value_G_L5036> = Global::new("DAT_140155078", Some(0x140155078));

/// Source symbol DAT_140155088. Contents remain caller supplied.
pub type value_G_L5037 = u64;
pub const G_L5037: Global<value_G_L5037> = Global::new("DAT_140155088", Some(0x140155088));

/// Source symbol DAT_1401790a0. Contents remain caller supplied.
pub type value_G_L5038 = u8;
pub const G_L5038: Global<value_G_L5038> = Global::new("DAT_1401790a0", Some(0x1401790a0));

/// Source symbol DAT_1401790a8. Contents remain caller supplied.
pub type value_G_L5039 = u8;
pub const G_L5039: Global<value_G_L5039> = Global::new("DAT_1401790a8", Some(0x1401790a8));

/// Source symbol DAT_1401cfa54. Contents remain caller supplied.
pub type value_G_L5040 = u32;
pub const G_L5040: Global<value_G_L5040> = Global::new("DAT_1401cfa54", Some(0x1401cfa54));

/// Source symbol DAT_140179090. Contents remain caller supplied.
pub type value_G_L5041 = u64;
pub const G_L5041: Global<value_G_L5041> = Global::new("DAT_140179090", Some(0x140179090));

/// Source symbol DAT_140179098. Contents remain caller supplied.
pub type value_G_L5042 = u8;
pub const G_L5042: Global<value_G_L5042> = Global::new("DAT_140179098", Some(0x140179098));

/// Source symbol DAT_14017909c. Contents remain caller supplied.
pub type value_G_L5043 = u32;
pub const G_L5043: Global<value_G_L5043> = Global::new("DAT_14017909c", Some(0x14017909c));

/// Source symbol DAT_1401cfa50. Contents remain caller supplied.
pub type value_G_L5044 = u8;
pub const G_L5044: Global<value_G_L5044> = Global::new("DAT_1401cfa50", Some(0x1401cfa50));

/// Source symbol DAT_1401790b4. Contents remain caller supplied.
pub type value_G_L5045 = i32;
pub const G_L5045: Global<value_G_L5045> = Global::new("DAT_1401790b4", Some(0x1401790b4));

/// Source symbol DAT_1401d0358. Contents remain caller supplied.
pub type value_G_L5046 = u8;
pub const G_L5046: Global<value_G_L5046> = Global::new("DAT_1401d0358", Some(0x1401d0358));

/// Source symbol DAT_1400e5918. Contents remain caller supplied.
pub type value_G_L5047 = u8;
pub const G_L5047: Global<value_G_L5047> = Global::new("DAT_1400e5918", Some(0x1400e5918));

/// Source symbol DAT_1400e7ed0. Contents remain caller supplied.
pub type value_G_L5048 = u8;
pub const G_L5048: Global<value_G_L5048> = Global::new("DAT_1400e7ed0", Some(0x1400e7ed0));

/// Source symbol DAT_14017909c. Contents remain caller supplied.
pub type value_G_L5049 = u8;
pub const G_L5049: Global<value_G_L5049> = Global::new("DAT_14017909c", Some(0x14017909c));

/// Source symbol DAT_1400e7ef0. Contents remain caller supplied.
pub type value_G_L5050 = u8;
pub const G_L5050: Global<value_G_L5050> = Global::new("DAT_1400e7ef0", Some(0x1400e7ef0));

/// Source symbol DAT_1401cfac8. Contents remain caller supplied.
pub type value_G_L5051 = Address<u64>;
pub const G_L5051: Global<value_G_L5051> = Global::new("DAT_1401cfac8", Some(0x1401cfac8));

/// Source symbol switchdataD_1400b2224. Contents remain caller supplied.
pub type value_G_L5052 = u32;
pub const G_L5052: Global<value_G_L5052> = Global::new("switchdataD_1400b2224", None);

/// Source symbol FUN_1400c2e00. Contents remain caller supplied.
pub type value_G_L5053 = u8;
pub const G_L5053: Global<value_G_L5053> = Global::new("FUN_1400c2e00", Some(0x1400c2e00));

/// Source symbol FUN_1400c2df0. Contents remain caller supplied.
pub type value_G_L5054 = u8;
pub const G_L5054: Global<value_G_L5054> = Global::new("FUN_1400c2df0", Some(0x1400c2df0));

/// Source symbol DAT_14010c760. Contents remain caller supplied.
pub type value_G_L5055 = u64;
pub const G_L5055: Global<value_G_L5055> = Global::new("DAT_14010c760", Some(0x14010c760));

/// Source symbol DAT_14010c790. Contents remain caller supplied.
pub type value_G_L5056 = u8;
pub const G_L5056: Global<value_G_L5056> = Global::new("DAT_14010c790", Some(0x14010c790));

/// Source symbol FUN_1400b3da0. Contents remain caller supplied.
pub type value_G_L5057 = u8;
pub const G_L5057: Global<value_G_L5057> = Global::new("FUN_1400b3da0", Some(0x1400b3da0));

/// Source symbol FUN_1400b3d90. Contents remain caller supplied.
pub type value_G_L5058 = u8;
pub const G_L5058: Global<value_G_L5058> = Global::new("FUN_1400b3d90", Some(0x1400b3d90));

/// Source symbol DAT_14010c780. Contents remain caller supplied.
pub type value_G_L5059 = u8;
pub const G_L5059: Global<value_G_L5059> = Global::new("DAT_14010c780", Some(0x14010c780));

/// Source symbol DAT_1400e4cb0. Contents remain caller supplied.
pub type value_G_L5060 = u8;
pub const G_L5060: Global<value_G_L5060> = Global::new("DAT_1400e4cb0", Some(0x1400e4cb0));

/// Source symbol DAT_1401d06e0. Contents remain caller supplied.
pub type value_G_L5061 = u64;
pub const G_L5061: Global<value_G_L5061> = Global::new("DAT_1401d06e0", Some(0x1401d06e0));

/// Source symbol FUN_1400bb0e0. Contents remain caller supplied.
pub type value_G_L5062 = u8;
pub const G_L5062: Global<value_G_L5062> = Global::new("FUN_1400bb0e0", Some(0x1400bb0e0));

/// Source symbol FUN_1400bb0f0. Contents remain caller supplied.
pub type value_G_L5063 = u8;
pub const G_L5063: Global<value_G_L5063> = Global::new("FUN_1400bb0f0", Some(0x1400bb0f0));

/// Source symbol DAT_1401cfcf0. Contents remain caller supplied.
pub type value_G_L5064 = i64;
pub const G_L5064: Global<value_G_L5064> = Global::new("DAT_1401cfcf0", Some(0x1401cfcf0));

/// Source symbol DAT_14010d288. Contents remain caller supplied.
pub type value_G_L5065 = u8;
pub const G_L5065: Global<value_G_L5065> = Global::new("DAT_14010d288", Some(0x14010d288));

/// Source symbol FUN_140033970. Contents remain caller supplied.
pub type value_G_L5066 = u8;
pub const G_L5066: Global<value_G_L5066> = Global::new("FUN_140033970", Some(0x140033970));

/// Source symbol DAT_14010d298. Contents remain caller supplied.
pub type value_G_L5067 = u8;
pub const G_L5067: Global<value_G_L5067> = Global::new("DAT_14010d298", Some(0x14010d298));

/// Source symbol FUN_1400b5ec0. Contents remain caller supplied.
pub type value_G_L5068 = u8;
pub const G_L5068: Global<value_G_L5068> = Global::new("FUN_1400b5ec0", Some(0x1400b5ec0));

/// Source symbol DAT_1401d07bc. Contents remain caller supplied.
pub type value_G_L5069 = u8;
pub const G_L5069: Global<value_G_L5069> = Global::new("DAT_1401d07bc", Some(0x1401d07bc));

/// Source symbol FUN_1400b5eb0. Contents remain caller supplied.
pub type value_G_L5070 = u8;
pub const G_L5070: Global<value_G_L5070> = Global::new("FUN_1400b5eb0", Some(0x1400b5eb0));

/// Source symbol switchdataD_1400b6914. Contents remain caller supplied.
pub type value_G_L5071 = u32;
pub const G_L5071: Global<value_G_L5071> = Global::new("switchdataD_1400b6914", None);

/// Source symbol DAT_14016ae30. Contents remain caller supplied.
pub type value_G_L5072 = u8;
pub const G_L5072: Global<value_G_L5072> = Global::new("DAT_14016ae30", Some(0x14016ae30));

/// Source symbol FUN_1400b6940. Contents remain caller supplied.
pub type value_G_L5073 = u8;
pub const G_L5073: Global<value_G_L5073> = Global::new("FUN_1400b6940", Some(0x1400b6940));

/// Source symbol FUN_1400b6930. Contents remain caller supplied.
pub type value_G_L5074 = u8;
pub const G_L5074: Global<value_G_L5074> = Global::new("FUN_1400b6930", Some(0x1400b6930));

/// Source symbol DAT_140111ad8. Contents remain caller supplied.
pub type value_G_L5075 = u8;
pub const G_L5075: Global<value_G_L5075> = Global::new("DAT_140111ad8", Some(0x140111ad8));

/// Source symbol FUN_1400c2520. Contents remain caller supplied.
pub type value_G_L5076 = u8;
pub const G_L5076: Global<value_G_L5076> = Global::new("FUN_1400c2520", Some(0x1400c2520));

/// Source symbol FUN_1400c2510. Contents remain caller supplied.
pub type value_G_L5077 = u8;
pub const G_L5077: Global<value_G_L5077> = Global::new("FUN_1400c2510", Some(0x1400c2510));

/// Source symbol DAT_1401d0e48. Contents remain caller supplied.
pub type value_G_L5078 = u32;
pub const G_L5078: Global<value_G_L5078> = Global::new("DAT_1401d0e48", Some(0x1401d0e48));

/// Source symbol DAT_1401d0e50. Contents remain caller supplied.
pub type value_G_L5079 = u8;
pub const G_L5079: Global<value_G_L5079> = Global::new("DAT_1401d0e50", Some(0x1401d0e50));

/// Source symbol DAT_1401d0e60. Contents remain caller supplied.
pub type value_G_L5080 = u64;
pub const G_L5080: Global<value_G_L5080> = Global::new("DAT_1401d0e60", Some(0x1401d0e60));

/// Source symbol DAT_1401d0e70. Contents remain caller supplied.
pub type value_G_L5081 = u8;
pub const G_L5081: Global<value_G_L5081> = Global::new("DAT_1401d0e70", Some(0x1401d0e70));

/// Source symbol DAT_1401d0e80. Contents remain caller supplied.
pub type value_G_L5082 = u64;
pub const G_L5082: Global<value_G_L5082> = Global::new("DAT_1401d0e80", Some(0x1401d0e80));

/// Source symbol DAT_1401d0e58. Contents remain caller supplied.
pub type value_G_L5083 = Address<Unknown>;
pub const G_L5083: Global<value_G_L5083> = Global::new("DAT_1401d0e58", Some(0x1401d0e58));

/// Source symbol DAT_1401d0e88. Contents remain caller supplied.
pub type value_G_L5084 = u64;
pub const G_L5084: Global<value_G_L5084> = Global::new("DAT_1401d0e88", Some(0x1401d0e88));

/// Source symbol FUN_1400df840. Contents remain caller supplied.
pub type value_G_L5085 = u8;
pub const G_L5085: Global<value_G_L5085> = Global::new("FUN_1400df840", Some(0x1400df840));

/// Source symbol UNK_1401d0e54. Contents remain caller supplied.
pub type value_G_L5086 = u32;
pub const G_L5086: Global<value_G_L5086> = Global::new("UNK_1401d0e54", None);

/// Source symbol UNK_1401d0e78. Contents remain caller supplied.
pub type value_G_L5087 = u64;
pub const G_L5087: Global<value_G_L5087> = Global::new("UNK_1401d0e78", None);

/// Source symbol DAT_1401cfea0. Contents remain caller supplied.
pub type value_G_L5088 = i64;
pub const G_L5088: Global<value_G_L5088> = Global::new("DAT_1401cfea0", Some(0x1401cfea0));

/// Source symbol DAT_1401144e0. Contents remain caller supplied.
pub type value_G_L5089 = u8;
pub const G_L5089: Global<value_G_L5089> = Global::new("DAT_1401144e0", Some(0x1401144e0));

/// Source symbol DAT_1401144f0. Contents remain caller supplied.
pub type value_G_L5090 = u8;
pub const G_L5090: Global<value_G_L5090> = Global::new("DAT_1401144f0", Some(0x1401144f0));

/// Source symbol DAT_1401145a8. Contents remain caller supplied.
pub type value_G_L5091 = u8;
pub const G_L5091: Global<value_G_L5091> = Global::new("DAT_1401145a8", Some(0x1401145a8));

/// Source symbol FUN_1400b2c00. Contents remain caller supplied.
pub type value_G_L5092 = u8;
pub const G_L5092: Global<value_G_L5092> = Global::new("FUN_1400b2c00", Some(0x1400b2c00));

/// Source symbol FUN_1400b2bf0. Contents remain caller supplied.
pub type value_G_L5093 = u8;
pub const G_L5093: Global<value_G_L5093> = Global::new("FUN_1400b2bf0", Some(0x1400b2bf0));

/// Source symbol switchdataD_1400c6d08. Contents remain caller supplied.
pub type value_G_L5094 = u32;
pub const G_L5094: Global<value_G_L5094> = Global::new("switchdataD_1400c6d08", None);

/// Source symbol DAT_140116b98. Contents remain caller supplied.
pub type value_G_L5095 = u8;
pub const G_L5095: Global<value_G_L5095> = Global::new("DAT_140116b98", Some(0x140116b98));

/// Source symbol FUN_1400c6d40. Contents remain caller supplied.
pub type value_G_L5096 = u8;
pub const G_L5096: Global<value_G_L5096> = Global::new("FUN_1400c6d40", Some(0x1400c6d40));

/// Source symbol FUN_1400c6d30. Contents remain caller supplied.
pub type value_G_L5097 = u8;
pub const G_L5097: Global<value_G_L5097> = Global::new("FUN_1400c6d30", Some(0x1400c6d30));

/// Source symbol switchdataD_1400c8578. Contents remain caller supplied.
pub type value_G_L5098 = u32;
pub const G_L5098: Global<value_G_L5098> = Global::new("switchdataD_1400c8578", None);

/// Source symbol FUN_1400bf370. Contents remain caller supplied.
pub type value_G_L5099 = u8;
pub const G_L5099: Global<value_G_L5099> = Global::new("FUN_1400bf370", Some(0x1400bf370));

/// Source symbol FUN_1400bf360. Contents remain caller supplied.
pub type value_G_L5100 = u8;
pub const G_L5100: Global<value_G_L5100> = Global::new("FUN_1400bf360", Some(0x1400bf360));

/// Source symbol FUN_1400c1b70. Contents remain caller supplied.
pub type value_G_L5101 = u8;
pub const G_L5101: Global<value_G_L5101> = Global::new("FUN_1400c1b70", Some(0x1400c1b70));

/// Source symbol FUN_1400c1b60. Contents remain caller supplied.
pub type value_G_L5102 = u8;
pub const G_L5102: Global<value_G_L5102> = Global::new("FUN_1400c1b60", Some(0x1400c1b60));

/// Source symbol FUN_1400c3a30. Contents remain caller supplied.
pub type value_G_L5103 = u8;
pub const G_L5103: Global<value_G_L5103> = Global::new("FUN_1400c3a30", Some(0x1400c3a30));

/// Source symbol FUN_1400c3a40. Contents remain caller supplied.
pub type value_G_L5104 = u8;
pub const G_L5104: Global<value_G_L5104> = Global::new("FUN_1400c3a40", Some(0x1400c3a40));

/// Source symbol FUN_1400c9d40. Contents remain caller supplied.
pub type value_G_L5105 = u8;
pub const G_L5105: Global<value_G_L5105> = Global::new("FUN_1400c9d40", Some(0x1400c9d40));

/// Source symbol FUN_1400c9d50. Contents remain caller supplied.
pub type value_G_L5106 = u8;
pub const G_L5106: Global<value_G_L5106> = Global::new("FUN_1400c9d50", Some(0x1400c9d50));

/// Source symbol DAT_14011cbc8. Contents remain caller supplied.
pub type value_G_L5107 = u64;
pub const G_L5107: Global<value_G_L5107> = Global::new("DAT_14011cbc8", Some(0x14011cbc8));

/// Source symbol StackLimit. Contents remain caller supplied.
pub type value_G_L5108 = Address<Unknown>;
pub const G_L5108: Global<value_G_L5108> = Global::new("StackLimit", None);

/// Source symbol DAT_1401cfd20. Contents remain caller supplied.
pub type value_G_L5109 = i64;
pub const G_L5109: Global<value_G_L5109> = Global::new("DAT_1401cfd20", Some(0x1401cfd20));

/// Source symbol DAT_1401d0cf0. Contents remain caller supplied.
pub type value_G_L5110 = i64;
pub const G_L5110: Global<value_G_L5110> = Global::new("DAT_1401d0cf0", Some(0x1401d0cf0));

/// Source symbol FUN_140070330. Contents remain caller supplied.
pub type value_G_L5111 = u8;
pub const G_L5111: Global<value_G_L5111> = Global::new("FUN_140070330", Some(0x140070330));

/// Source symbol DAT_1401791d0. Contents remain caller supplied.
pub type value_G_L5112 = i64;
pub const G_L5112: Global<value_G_L5112> = Global::new("DAT_1401791d0", Some(0x1401791d0));

/// Source symbol DAT_1401791c0. Contents remain caller supplied.
pub type value_G_L5113 = u8;
pub const G_L5113: Global<value_G_L5113> = Global::new("DAT_1401791c0", Some(0x1401791c0));
