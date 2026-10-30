//! Checked translations generated from original bodies by batch 024.
//! Named definitions use source-line identities; see the manifest for original signatures.
//! Resolve `STACK_local_*` within one caller-owned frame using `FRAME_OFFSETS`.
//! Slot widths are listed in `SCRATCH_LAYOUTS`; preserve adjacency and aliases.
//! `INDIRECT_CALL` receives the target word followed by explicit source arguments.
//! `Halted` ends a recovered non-returning path after the runtime callback.
use super::call_support::MemoryFault;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    Memory(MemoryFault),
    Arguments(MissingArguments),
    Halted(&'static str),
    Arithmetic(&'static str),
}
impl From<MemoryFault> for AdapterError {
    fn from(fault: MemoryFault) -> Self {
        Self::Memory(fault)
    }
}
use super::call_support::{MemoryImage, MissingArguments, RecordCall, RelocateCall};
/// Translation of `FUN_140001310` (`mspaint.c:5281`).
pub fn fun_140001310(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    let mut pc: i32 = 21;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("FUN_1400df970")];
                    invoke("atexit", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_1401d0cd0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ca8")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [relocate("DAT_1401d0c80")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[0])?)?,
                        values[0],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(((values[0]) as i64) != ((0x0u64) as i64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let destination = relocate("DAT_1401d0c78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("_DAT_1401d0c70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("DAT_1401d0cd0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[0] = memory.read_u64(relocate("DAT_1401d0c78"))?;
                pc = 9;
            }
            11 => {
                {
                    let destination = memory.read_u64(relocate("DAT_1401d0ca8"))?;
                    let stored = relocate("DAT_1401d0ca8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (memory.read_u64(relocate("DAT_1401d0ca8"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("DAT_1401d0ca8");
                    let stored = {
                        let call_arguments = [0x10u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("DAT_1401d0cc8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("DAT_1401d0cc0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("DAT_1401d0cb8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("DAT_1401d0cb0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("DAT_1401d0ca8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = memory.read_u64(relocate("DAT_1401d0c80"))?;
                    let stored = relocate("DAT_1401d0c80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = (memory.read_u64(relocate("DAT_1401d0c80"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("DAT_1401d0c80");
                    let stored = {
                        let call_arguments = [0x10u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140004c10` (`mspaint.c:8958`).
pub fn fun_140004c10(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], arguments[1], 0u64, 0u64];
    let mut pc: i32 = 30;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[2]));
            }
            1 => {
                values[2] = 0x0u64;
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                values[2] = 0x8007000eu64;
                pc = 2;
            }
            4 => {
                {
                    let destination = values[1];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x8u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((values[0]) != (0x0u64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [(values[2]).wrapping_add((0x1u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                pc = if (u64::from(
                    (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        != (values[0]),
                )) != 0
                {
                    9
                } else {
                    4
                };
            }
            11 => {
                values[3] = values[2];
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[2]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64))) != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(
                                (memory.read_u64(relocate("STACK_local_30"))?)
                                    .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?)
                            .wrapping_add((0x0u64).wrapping_mul(1)),
                        )?,
                        memory.read_u64(relocate("STACK_local_30"))?,
                        relocate("DAT_1401054f0"),
                        relocate("STACK_local_38"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) == (0x0u64))) != 0
                {
                    16
                } else {
                    20
                };
            }
            22 => {
                {
                    let call_arguments = [0x0u64, relocate("STACK_local_30")];
                    invoke("CoCreateFreeThreadedMarshaler", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = values[2];
                    let stored=relocate("`int___cdecl_winrt::impl::make_marshaler(winrt::impl::abi<>::type*___ptr64,void*___ptr64*___ptr64)'\r\n              ::__l2::marshaler::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                pc = if (u64::from((values[2]) != (0x0u64))) != 0 {
                    25
                } else {
                    4
                };
            }
            27 => {
                values[3] = 0x0u64;
                pc = 26;
            }
            28 => {
                values[2] = {
                    let call_arguments = [0x20u64, relocate("PTR_1400e4d48")];
                    invoke("FUN_1400b0234", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_30"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400086f0` (`mspaint.c:12090`).
pub fn fun_1400086f0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 3 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 3,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 118;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("ReleaseStgMedium", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) != (0x0u64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                pc = 1;
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("CDC::~CDC", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                pc = 1;
            }
            9 => {
                {
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("CDC::~CDC", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_e8")];
                    invoke("FUN_14000ab00", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = relocate("CBitmap::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [(values[16]).wrapping_shr(0)];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 11;
            }
            14 => {
                pc = if (u64::from(((values[16]).wrapping_shr(0)) != (0x0u64))) != 0 {
                    13
                } else {
                    11
                };
            }
            15 => {
                {
                    let call_arguments = [values[0], relocate("DAT_1401d06b8")];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("CDC::DeleteDC", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments =
                        [values[5], (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64];
                    invoke("RestoreDC", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [(values[4]).wrapping_shr(0)];
                    invoke("GlobalUnlock", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [
                        values[5],
                        memory.read_u64((values[13]).wrapping_add((0x4u64).wrapping_mul(4)))?,
                    ];
                    invoke("PlayMetaFile", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [
                        values[5],
                        (((u64::from(memory.read_u32(
                            (relocate("STACK_local_58")).wrapping_add((0x8u64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64)
                            .wrapping_sub(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_58"))
                                            .wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?,
                                )) as i32 as i64 as u64,
                            )) as i32 as i64 as u64,
                        (((u64::from(memory.read_u32(
                            (relocate("STACK_local_58")).wrapping_add((0xcu64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64)
                            .wrapping_sub(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_58"))
                                            .wrapping_add((0x4u64).wrapping_mul(1)),
                                    )?,
                                )) as i32 as i64 as u64,
                            )) as i32 as i64 as u64,
                        0x0u64,
                    ];
                    invoke("SetViewportExtEx", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u32(values[13])?)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            == (((0x7u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            ((((u64::from(memory.read_u32(values[13])?)) as i32 as i64 as u64)
                                as i32 as i64 as u64) as i64)
                                == (((0x8u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    20
                } else {
                    19
                };
            }
            22 => {
                {
                    let call_arguments = [
                        values[5],
                        (u64::from(memory.read_u32(values[13])?)) as i32 as i64 as u64,
                    ];
                    invoke("SetMapMode", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [values[14]];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_58"), values[14]];
                    invoke("FillRect", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    24
                } else {
                    22
                };
            }
            26 => {
                values[14] = {
                    let call_arguments = [values[15]];
                    invoke("CreateSolidBrush", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                values[15] = u64::from(
                    (u64::from((u64::from(memory.read_u32(relocate("DAT_140179144"))?)) as u32))
                        as u32,
                );
                pc = 26;
            }
            28 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32))
                            as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    27
                } else {
                    26
                };
            }
            29 => {
                values[15] = u64::from(
                    (u64::from((u64::from(memory.read_u32(relocate("DAT_140179140"))?)) as u32))
                        as u32,
                );
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [values[5]];
                    invoke("SaveDC", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                pc = if (u64::from((values[13]) != (0x0u64))) != 0 {
                    30
                } else {
                    16
                };
            }
            32 => {
                values[13] = {
                    let call_arguments = [memory.read_u64(
                        ((relocate("STACK_local_d8")).wrapping_add((0x8u64).wrapping_mul(1)))
                            .wrapping_add((0x0u64).wrapping_mul(1)),
                    )?];
                    invoke("GlobalLock", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = (values[3]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                values[4] = memory.read_u64(
                    (relocate("STACK_local_d8")).wrapping_add((0x8u64).wrapping_mul(1)),
                )?;
                pc = 36;
            }
            38 => {
                values[5] = memory.read_u64(relocate("STACK_pHStack_98"))?;
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [relocate("STACK_local_a0"), values[12], 0x0u64];
                    invoke("CDC::SelectPalette", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                values[12] = {
                    let call_arguments = [(values[16]).wrapping_shr(0)];
                    invoke("CGdiObject::FromHandle", &call_arguments, memory)
                };
                pc = 39;
            }
            41 => {
                pc = if (u64::from(((values[16]).wrapping_shr(0)) != (0x0u64))) != 0 {
                    40
                } else {
                    38
                };
            }
            42 => {
                {
                    let stored = 0x0u64;
                    values[16] = stored;
                }
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [relocate("STACK_local_c0")];
                    invoke("ReleaseStgMedium", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let destination =
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) != (0x0u64),
                )) != 0
                {
                    45
                } else {
                    44
                };
            }
            47 => {
                {
                    let call_arguments = [relocate("STACK_local_c0")];
                    invoke("ReleaseStgMedium", &call_arguments, memory)
                };
                pc = 41;
            }
            48 => {
                {
                    let destination =
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = memory.read_u64(
                        ((relocate("STACK_local_c0")).wrapping_add((0x8u64).wrapping_mul(1)))
                            .wrapping_add((0x0u64).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                pc = 44;
            }
            50 => {
                pc = if (u64::from(
                    ((values[17]) != 0)
                        || ((u64::from(
                            (memory.read_u64(
                                ((relocate("STACK_local_c0"))
                                    .wrapping_add((0x8u64).wrapping_mul(1)))
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?) == (0x0u64),
                        )) != 0),
                )) != 0
                {
                    49
                } else {
                    48
                };
            }
            51 => {
                values[16] = memory.read_u64(
                    (relocate("STACK_local_c0")).wrapping_add((0x8u64).wrapping_mul(1)),
                )?;
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) != (0x0u64),
                )) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                values[17] = u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_c0"))
                                            .wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?,
                                )) as u32,
                            )) as u32,
                        )) != (u64::from((0x10u64) as u32)),
                    )) != 0,
                );
                pc = 53;
            }
            55 => {
                pc = 46;
            }
            56 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    55
                } else {
                    54
                };
            }
            57 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(
                                (memory.read_u64(
                                    (relocate("STACK_local_60"))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?)
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?)
                            .wrapping_add((0x18u64).wrapping_mul(1)),
                        )?,
                        memory.read_u64(
                            (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = u64::from((0x10u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_6c");
                    let stored = u64::from((0xffffffffu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination =
                        (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(2));
                    let stored = u64::from((0x9u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 61;
            }
            63 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    62
                };
            }
            64 => {
                values[6] = ({
                    let call_arguments =
                        [(relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1))];
                    invoke("OleGetClipboard", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                {
                    let destination =
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination =
                        (relocate("STACK_local_c0")).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = ((relocate("STACK_local_c0"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = (relocate("STACK_local_c0")).wrapping_add(0x4u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 67;
            }
            69 => {
                {
                    let destination =
                        (relocate("STACK_local_c0")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_pHStack_98"))?,
                        memory.read_u64(relocate("STACK_local_e0"))?,
                    ];
                    invoke("CDC::SelectGdiObject", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    70
                };
            }
            72 => {
                values[8] = ({
                    let call_arguments = [relocate("STACK_local_e8"), values[11]];
                    invoke("CGdiObject::Attach", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 71;
            }
            73 => {
                values[11] = {
                    let call_arguments = [
                        memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        (u64::from(memory.read_u32(relocate("STACK_local_f4"))?)) as i32 as i64
                            as u64,
                        (u64::from(memory.read_u32(relocate("STACK_local_f8"))?)) as i32 as i64
                            as u64,
                    ];
                    invoke("CreateCompatibleBitmap", &call_arguments, memory)
                };
                pc = 72;
            }
            74 => {
                values[7] = ((u64::from(memory.read_u32(relocate("STACK_local_f8"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 73;
            }
            75 => {
                values[3] = ((u64::from(memory.read_u32(relocate("STACK_local_f4"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 74;
            }
            76 => {
                values[9] = {
                    let call_arguments = [values[1]];
                    invoke("CDC::FromHandle", &call_arguments, memory)
                };
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = relocate("CBitmap::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    78
                } else {
                    7
                };
            }
            80 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            ((relocate("STACK_local_d8")).wrapping_add((0x8u64).wrapping_mul(1)))
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                        )?,
                        memory.read_u64(relocate("STACK_pHStack_98"))?,
                        relocate("STACK_local_f4"),
                    ];
                    invoke("FUN_140008440", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = relocate("STACK_local_60");
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_110");
                    let stored = relocate("STACK_local_f0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination =
                        (relocate("STACK_local_f0")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination =
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (memory.read_u64(
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = relocate("STACK_local_f4");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    87
                } else {
                    7
                };
            }
            89 => {
                values[7] = ({
                    let call_arguments = [relocate("STACK_local_a0"), values[10]];
                    invoke("CDC::Attach", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 88;
            }
            90 => {
                values[10] = {
                    let call_arguments = [(values[16]).wrapping_shr(0)];
                    invoke("CreateCompatibleDC", &call_arguments, memory)
                };
                pc = 89;
            }
            91 => {
                values[16] = memory.read_u64(
                    ((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))
                        .wrapping_add((0x0u64).wrapping_mul(1)),
                )?;
                pc = 90;
            }
            92 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    91
                } else {
                    90
                };
            }
            93 => {
                {
                    let stored = 0x0u64;
                    values[16] = stored;
                }
                pc = 92;
            }
            94 => {
                values[9] = {
                    let call_arguments = [values[1]];
                    invoke("CDC::FromHandle", &call_arguments, memory)
                };
                pc = 93;
            }
            95 => {
                {
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("CDC::CDC", &call_arguments, memory)
                };
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_uStack_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_pHStack_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                pc = if (u64::from(
                    ((values[17]) != 0)
                        && ((u64::from(
                            (memory.read_u64(
                                ((relocate("STACK_local_d8"))
                                    .wrapping_add((0x8u64).wrapping_mul(1)))
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    99
                } else {
                    2
                };
            }
            101 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 100;
            }
            102 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) != (0x0u64),
                )) != 0
                {
                    101
                } else {
                    100
                };
            }
            103 => {
                values[17] = u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_d8"))
                                            .wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?,
                                )) as u32,
                            )) as u32,
                        )) == (u64::from((0x20u64) as u32)),
                    )) != 0,
                );
                pc = 102;
            }
            104 => {
                pc = 4;
            }
            105 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    104
                } else {
                    103
                };
            }
            106 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(
                                (memory.read_u64(
                                    (relocate("STACK_local_60"))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?)
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?)
                            .wrapping_add((0x18u64).wrapping_mul(1)),
                        )?,
                        memory.read_u64(
                            (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?,
                        relocate("STACK_local_c0"),
                        relocate("STACK_local_d8"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = u64::from((0x20u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 106;
            }
            108 => {
                {
                    let destination =
                        (relocate("STACK_local_c0")).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = 0xffffffff00000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let destination = ((relocate("STACK_local_c0"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let destination = ((relocate("STACK_local_c0"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x3u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 109;
            }
            111 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    110
                };
            }
            112 => {
                values[6] = ({
                    let call_arguments =
                        [(relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1))];
                    invoke("OleGetClipboard", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 111;
            }
            113 => {
                {
                    let destination =
                        (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let destination =
                        (relocate("STACK_local_d8")).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = ((relocate("STACK_local_d8"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = (relocate("STACK_local_d8")).wrapping_add(0x4u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination =
                        (relocate("STACK_local_d8")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_138"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400113f0` (`mspaint.c:19734`).
pub fn fun_1400113f0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 32;
    loop {
        match pc {
            0 => {
                return Ok(Some((values[1]) as i32 as i64 as u64));
            }
            1 => {
                values[1] =
                    ((0u64.wrapping_sub(0x7ff8fff2u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x10u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x8u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                return Ok(Some((values[2]) as i32 as i64 as u64));
            }
            6 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x10u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments=[relocate("CONTEXT_unaff_retaddr"),0x62bu64,relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.Windows.ImplementationLibrary.1.0.250325.1\\\\include\\\\wil\\\\winrt.h\""),values[2]];
                    invoke("FUN_140012290", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    4
                };
            }
            9 => {
                values[2] = ({
                    let call_arguments = [];
                    invoke("FUN_140011c50", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [
                        (values[3]).wrapping_add((0x7u64).wrapping_mul(8)),
                        values[4],
                    ];
                    invoke("FUN_140011d10", &call_arguments, memory)
                };
                pc = 4;
            }
            11 => {
                {
                    let call_arguments = [];
                    invoke("GetLastError", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    9
                } else {
                    11
                };
            }
            13 => {
                values[4] = {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x1f0003u64];
                    invoke("CreateEventExW", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[3]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[3]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored=relocate("`long___cdecl_wil::details::WaitForCompletion<>(ABI::Windows::Foundation::IAsyncOperation<>*___ptr64,tagCOWAIT_FLAGS,unsigned_long,bool*___ptr64)'\r\n                ::__l2::CompletionDelegate::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = values[3];
                    let stored=relocate("`long___cdecl_wil::details::WaitForCompletion<>(ABI::Windows::Foundation::IAsyncOperation<>*___ptr64,tagCOWAIT_FLAGS,unsigned_long,bool*___ptr64)'\r\n              ::__l2::CompletionDelegate::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401cfc30"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("DAT_1401cfc30"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("Microsoft::WRL::RuntimeClass<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = values[3];
                    let stored = relocate("Microsoft::WRL::RuntimeClass<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = (values[3]).wrapping_add(0x2cu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [(values[3]).wrapping_add((0x1u64).wrapping_mul(8))];
                    invoke("FUN_140011b10", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination = values[3];
                    let stored = relocate(
                        "ABI::Windows::Foundation::IAsyncOperationCompletedHandler<>::vftable",
                    );
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[3]) == (0x0u64))) != 0 {
                    1
                } else {
                    24
                };
            }
            26 => {
                values[3] = {
                    let call_arguments = [0x40u64];
                    invoke("FUN_1400b0234", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x10u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    29
                } else {
                    27
                };
            }
            31 => {
                values[1] = (0x0u64) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                values[3] = memory.read_u64(values[0])?;
                pc = 31;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140011570` (`mspaint.c:19797`).
pub fn fun_140011570(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 32;
    loop {
        match pc {
            0 => {
                return Ok(Some((values[1]) as i32 as i64 as u64));
            }
            1 => {
                values[1] =
                    ((0u64.wrapping_sub(0x7ff8fff2u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x10u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x8u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                return Ok(Some((values[2]) as i32 as i64 as u64));
            }
            6 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x10u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments=[relocate("CONTEXT_unaff_retaddr"),0x62bu64,relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.Windows.ImplementationLibrary.1.0.250325.1\\\\include\\\\wil\\\\winrt.h\""),values[2]];
                    invoke("FUN_140012290", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    4
                };
            }
            9 => {
                values[2] = ({
                    let call_arguments = [];
                    invoke("FUN_140011c50", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [
                        (values[3]).wrapping_add((0x7u64).wrapping_mul(8)),
                        values[4],
                    ];
                    invoke("FUN_140011d10", &call_arguments, memory)
                };
                pc = 4;
            }
            11 => {
                {
                    let call_arguments = [];
                    invoke("GetLastError", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    9
                } else {
                    11
                };
            }
            13 => {
                values[4] = {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x1f0003u64];
                    invoke("CreateEventExW", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[3]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[3]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored=relocate("`long___cdecl_wil::details::WaitForCompletion<>(ABI::Windows::Foundation::IAsyncOperation<>*___ptr64,tagCOWAIT_FLAGS,unsigned_long,bool*___ptr64)'\r\n                ::__l2::CompletionDelegate::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = values[3];
                    let stored=relocate("`long___cdecl_wil::details::WaitForCompletion<>(ABI::Windows::Foundation::IAsyncOperation<>*___ptr64,tagCOWAIT_FLAGS,unsigned_long,bool*___ptr64)'\r\n              ::__l2::CompletionDelegate::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401cfc30"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("DAT_1401cfc30"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("Microsoft::WRL::RuntimeClass<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = values[3];
                    let stored = relocate("Microsoft::WRL::RuntimeClass<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = (values[3]).wrapping_add(0x2cu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [(values[3]).wrapping_add((0x1u64).wrapping_mul(8))];
                    invoke("FUN_140011b10", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination = values[3];
                    let stored = relocate(
                        "ABI::Windows::Foundation::IAsyncOperationCompletedHandler<>::vftable",
                    );
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[3]) == (0x0u64))) != 0 {
                    1
                } else {
                    24
                };
            }
            26 => {
                values[3] = {
                    let call_arguments = [0x40u64];
                    invoke("FUN_1400b0234", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x10u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    29
                } else {
                    27
                };
            }
            31 => {
                values[1] = (0x0u64) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                values[3] = memory.read_u64(values[0])?;
                pc = 31;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140012300` (`mspaint.c:20672`).
pub fn fun_140012300(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 17;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [relocate("STACK_local_14c8")];
                    invoke("FUN_140012ba0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140012ba0"));
            }
            1 => {
                return Ok(None);
            }
            2 => {
                pc = if (u64::from(
                    (((((u64::from(
                        (u64::from(memory.read_u8(relocate("STACK_local_14c4"))?)) as u8,
                    )) & (0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140012830", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("STACK_local_1528");
                    let stored = memory.read_u64(relocate("STACK_in_stack_00000028"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_1520");
                    let stored = memory.read_u64(relocate("STACK_in_stack_00000030"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_1518");
                    let stored = u64::from((0x2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_1510");
                    let stored = memory.read_u64(relocate("STACK_in_stack_00000038"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_1508");
                    let stored = memory.read_u64(relocate("STACK_in_stack_00000040"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_1500");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_14f0");
                    let stored = 0x800u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_14f8");
                    let stored = relocate("STACK_local_1028");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_14e0");
                    let stored = 0x400u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_14e8");
                    let stored = relocate("STACK_local_1428");
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_14d8");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_in_stack_00000050"))?))
                                as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_14d0");
                    let stored = relocate("STACK_local_14c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_1548"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_uStack_18");
                    let stored = 0x14001230du64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400134c0` (`mspaint.c:21511`).
pub fn fun_1400134c0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 10;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d0410")));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d0408")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("uRam00000001401d041c");
                    let stored = u64::from((0x2ef31ef8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("uRam00000001401d0418");
                    let stored = u64::from((0x7b9du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("uRam00000001401d0414");
                    let stored = u64::from((0x11d30728u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("_DAT_1401d0410");
                    let stored = u64::from((0xb96b3cabu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("DAT_1401d0424");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("_DAT_1401d0420");
                    let stored = u64::from((0x21808u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0408"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    7
                } else {
                    0
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("DAT_1401d0408")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (u64::from(
                        ((u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    memory.read_u64(relocate("ThreadLocalStoragePointer"))?,
                                )?)
                                .wrapping_add(0x4u64),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    )) < (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0408"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    9
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140013530` (`mspaint.c:21533`).
pub fn fun_140013530(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 10;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d0490")));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d0488")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("uRam00000001401d049c");
                    let stored = u64::from((0x2ef31ef8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("uRam00000001401d0498");
                    let stored = u64::from((0x7b9du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("uRam00000001401d0494");
                    let stored = u64::from((0x11d30728u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("_DAT_1401d0490");
                    let stored = u64::from((0xb96b3caeu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("DAT_1401d04a4");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("_DAT_1401d04a0");
                    let stored = u64::from((0x21808u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0488"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    7
                } else {
                    0
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("DAT_1401d0488")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (u64::from(
                        ((u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    memory.read_u64(relocate("ThreadLocalStoragePointer"))?,
                                )?)
                                .wrapping_add(0x4u64),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    )) < (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0488"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    9
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400135a0` (`mspaint.c:21555`).
pub fn fun_1400135a0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 10;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d04b0")));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d04a8")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("uRam00000001401d04bc");
                    let stored = u64::from((0x2ef31ef8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("uRam00000001401d04b8");
                    let stored = u64::from((0x7b9du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("uRam00000001401d04b4");
                    let stored = u64::from((0x11d30728u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("_DAT_1401d04b0");
                    let stored = u64::from((0xb96b3cb0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("DAT_1401d04c4");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("_DAT_1401d04c0");
                    let stored = u64::from((0x30803u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d04a8"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    7
                } else {
                    0
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("DAT_1401d04a8")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (u64::from(
                        ((u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    memory.read_u64(relocate("ThreadLocalStoragePointer"))?,
                                )?)
                                .wrapping_add(0x4u64),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    )) < (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d04a8"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    9
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140013610` (`mspaint.c:21577`).
pub fn fun_140013610(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 10;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d04d0")));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d04c8")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("uRam00000001401d04dc");
                    let stored = u64::from((0x2ef31ef8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("uRam00000001401d04d8");
                    let stored = u64::from((0x7b9du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("uRam00000001401d04d4");
                    let stored = u64::from((0x11d30728u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("_DAT_1401d04d0");
                    let stored = u64::from((0xb96b3cafu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("DAT_1401d04e4");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("_DAT_1401d04e0");
                    let stored = u64::from((0x26200au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d04c8"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    7
                } else {
                    0
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("DAT_1401d04c8")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (u64::from(
                        ((u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    memory.read_u64(relocate("ThreadLocalStoragePointer"))?,
                                )?)
                                .wrapping_add(0x4u64),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    )) < (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d04c8"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    9
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140013680` (`mspaint.c:21599`).
pub fn fun_140013680(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 10;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d04f0")));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d04e8")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("uRam00000001401d04fc");
                    let stored = u64::from((0x759ffab0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("uRam00000001401d04f8");
                    let stored = u64::from((0xc96acba9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("uRam00000001401d04f4");
                    let stored = u64::from((0x44149b51u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("_DAT_1401d04f0");
                    let stored = u64::from((0x3654cecau64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination =
                        (relocate("DAT_1401d0500")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination =
                        (relocate("DAT_1401d0500")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x26200au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d04e8"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    7
                } else {
                    0
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("DAT_1401d04e8")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (u64::from(
                        ((u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    memory.read_u64(relocate("ThreadLocalStoragePointer"))?,
                                )?)
                                .wrapping_add(0x4u64),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    )) < (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d04e8"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    9
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400139a0` (`mspaint.c:21745`).
pub fn fun_1400139a0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 111;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_1400147f0", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(4));
                    let stored = u64::from((values[8]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                values[8] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x4u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(4));
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = u64::from((values[4]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = u64::from((values[8]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = values[0];
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[10])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                values[5] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 7;
            }
            9 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 8;
            }
            10 => {
                values[8] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 9;
            }
            11 => {
                values[10] = {
                    let call_arguments = [];
                    invoke("FUN_140013610", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(4));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(4));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((relocate("STACK_local_48"))
                                        .wrapping_add((0x8u64).wrapping_mul(1)))
                                    .wrapping_add((0x4u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            14 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((relocate("STACK_local_48"))
                                        .wrapping_add((0x8u64).wrapping_mul(1)))
                                    .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_48")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = values[0];
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(
                                (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(1)),
                            )?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(4));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 16;
            }
            18 => {
                values[8] = u64::from(
                    ({
                        let call_arguments = [values[14], 0x0u64, 0xau64];
                        invoke("wcstol", &call_arguments, memory)
                    }) as u32,
                );
                pc = 17;
            }
            19 => {
                values[14] = memory.read_u64(values[14])?;
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64(
                            (memory.read_u64(relocate("STACK_local_98"))?)
                                .wrapping_add((0x1cu64).wrapping_mul(2)),
                        )?),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[14] = (memory.read_u64(relocate("STACK_local_98"))?)
                    .wrapping_add((0x10u64).wrapping_mul(2));
                pc = 20;
            }
            22 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    21
                };
            }
            23 => {
                values[7] = ({
                    let call_arguments = [values[11], relocate("STACK_local_48")];
                    invoke("CLSIDFromString", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                values[11] = memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?;
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64(
                            (memory.read_u64(relocate("STACK_local_98"))?)
                                .wrapping_add((0xcu64).wrapping_mul(2)),
                        )?),
                )) != 0
                {
                    24
                } else {
                    23
                };
            }
            26 => {
                values[11] = memory.read_u64(relocate("STACK_local_98"))?;
                pc = 25;
            }
            27 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x7u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x5u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x3u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = ((relocate("STACK_local_48"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(4));
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[10]).wrapping_add((0x4u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            39 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(4));
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = u64::from((values[4]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = u64::from((values[8]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = values[0];
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[10])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            43 => {
                values[5] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 42;
            }
            44 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 43;
            }
            45 => {
                values[8] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 44;
            }
            46 => {
                values[10] = {
                    let call_arguments = [];
                    invoke("FUN_140013610", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                {
                    let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(4));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 46;
            }
            48 => {
                pc = if (u64::from(
                    (((memory.read_u64(relocate("STACK_lStack_90"))?)
                        .wrapping_sub(memory.read_u64(relocate("STACK_local_98"))?))
                        & (0xffffffffffffffe0u64))
                        == (0x40u64),
                )) != 0
                {
                    37
                } else {
                    47
                };
            }
            49 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_98"),
                        relocate("STACK_local_f8"),
                        relocate("STACK_local_128"),
                    ];
                    invoke("FUN_140013dd0", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_uStack_a0");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_uStack_120");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_uStack_dc");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_uStack_60")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_uStack_e0");
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_uStack_60"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_uStack_e4");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_local_68"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_uStack_ec");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (relocate("STACK_uStack_120")).wrapping_add(0x4u64),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_uStack_f0");
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_uStack_120"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = (relocate("STACK_local_d8")).wrapping_add(0xcu64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_uStack_a0")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = (relocate("STACK_local_d8")).wrapping_add(0x0u64);
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (relocate("STACK_local_d8")).wrapping_add(0x8u64);
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_uStack_a0"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = relocate("STACK_uStack_110");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = (relocate("STACK_uStack_a0")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((relocate("STACK_local_80")).wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = (relocate("STACK_uStack_a0")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            ((relocate("STACK_local_80")).wrapping_shr(
                                (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 67;
            }
            69 => {
                {
                    let destination = (relocate("STACK_uStack_120")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(((values[12]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = (relocate("STACK_uStack_120")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            ((values[12]).wrapping_shr(
                                (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(((values[13]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x0u64);
                    let stored = u64::from((u64::from((values[13]) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(((values[9]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x0u64);
                    let stored = u64::from((u64::from((values[9]) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_ppuStack_100");
                    let stored =
                        memory.read_u64((relocate("STACK_local_d8")).wrapping_add(0x8u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        memory.read_u64((relocate("STACK_local_d8")).wrapping_add(0x0u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                values[6] = memory.read_u64(relocate("STACK_local_108"))?;
                pc = 77;
            }
            79 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_108"))?) & (0xffffffffffffff00u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                values[9] = (values[9]).wrapping_add(0x2u64);
                pc = 79;
            }
            81 => {
                values[13] = (values[9]).wrapping_add(0x4u64);
                pc = 80;
            }
            82 => {
                pc = if (u64::from(
                    ((u64::from(((values[9]) as i64) != ((values[3]) as i64))) != 0)
                        && (({
                            let _ = {
                                values[13] = (values[9]).wrapping_add(0x2u64);
                                values[13]
                            };
                            u64::from(
                                ((memory.read_u64(relocate("STACK_lStack_70"))?) as i64)
                                    == ((0x0u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    81
                } else {
                    79
                };
            }
            83 => {
                values[13] = values[9];
                pc = 82;
            }
            84 => {
                values[9] = {
                    let call_arguments = [values[12], values[3], relocate("DAT_140107f08"), 0x1u64];
                    invoke("thunk_FUN_1400b1350", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_uStack_120");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                values[3] = ((memory
                    .read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                .wrapping_mul(0x2u64))
                .wrapping_add(values[12]);
                pc = 87;
            }
            89 => {
                values[12] = memory.read_u64(values[1])?;
                pc = 88;
            }
            90 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[1]).wrapping_add((0x3u64).wrapping_mul(8)))?),
                )) != 0
                {
                    89
                } else {
                    88
                };
            }
            91 => {
                values[12] = values[1];
                pc = 90;
            }
            92 => {
                {
                    let destination = (relocate("STACK_local_d8")).wrapping_add(0x0u64);
                    let stored = memory.read_u64(relocate("STACK_local_108"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = (relocate("STACK_local_d8")).wrapping_add(0x8u64);
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                values[2] = ((memory
                    .read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                .wrapping_mul(0x2u64))
                .wrapping_add(values[12]);
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_108"))?) & (0xffffffffffffff00u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                values[12] = memory.read_u64(values[1])?;
                pc = 95;
            }
            97 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[1]).wrapping_add((0x3u64).wrapping_mul(8)))?),
                )) != 0
                {
                    96
                } else {
                    95
                };
            }
            98 => {
                values[12] = values[1];
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_lStack_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("STACK_lStack_70");
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("DAT_140107f08");
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = relocate("STACK_local_49");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let destination = relocate("STACK_local_4b");
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("STACK_local_4f");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 106;
            }
            108 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_local_57");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_148"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400163a0` (`mspaint.c:24027`).
pub fn fun_1400163a0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 3 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 3,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        (arguments[2]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 40;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[7]));
            }
            1 => {
                values[7] = {
                    let call_arguments = [values[0]];
                    invoke("FUN_1400165d0", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                return Ok(Some(values[4]));
            }
            3 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[4] = u64::from(
                    ({
                        let call_arguments = [
                            values[0],
                            relocate("STACK_local_38"),
                            values[2],
                            relocate("STACK_local_30"),
                        ];
                        invoke("FUN_140017480", &call_arguments, memory)
                    }) as u8,
                );
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[8], values[1]];
                    invoke(
                        "Paint::Provenance::ProvenanceHelper::InsertFileIngredientIfManifestExists",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 5;
            }
            7 => {
                values[8] = {
                    let call_arguments = [values[0]];
                    invoke("FUN_140017300", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0)
                        && ((u64::from(
                            ((u64::from(
                                (({
                                    let _ = {
                                        values[5] = ({
                                            let call_arguments = [
                                                relocate("STACK_local_30"),
                                                relocate("DAT_140106428"),
                                                0x10u64,
                                            ];
                                            invoke("memcmp", &call_arguments, memory)
                                        })
                                            as i32
                                            as i64
                                            as u64;
                                        values[5]
                                    };
                                    u64::from(
                                        ((u64::from(
                                            (((values[5]) as i32 as i64 as u64) as i64)
                                                == (((0x0u64) as i32 as i64 as u64) as i64),
                                        )) != 0)
                                            || (({
                                                let _ = {
                                                    values[5] = ({
                                                        let call_arguments = [
                                                            relocate("STACK_local_30"),
                                                            relocate("DAT_1400e4c10"),
                                                            0x10u64,
                                                        ];
                                                        invoke("memcmp", &call_arguments, memory)
                                                    })
                                                        as i32
                                                        as i64
                                                        as u64;
                                                    values[5]
                                                };
                                                u64::from(
                                                    (((values[5]) as i32 as i64 as u64) as i64)
                                                        == (((0x0u64) as i32 as i64 as u64) as i64),
                                                )
                                            }) != 0),
                                    )
                                }) != 0)
                                    || (({
                                        let _ = {
                                            values[5] = ({
                                                let call_arguments = [
                                                    relocate("STACK_local_30"),
                                                    relocate("DAT_1400e4bf0"),
                                                    0x10u64,
                                                ];
                                                invoke("memcmp", &call_arguments, memory)
                                            })
                                                as i32
                                                as i64
                                                as u64;
                                            values[5]
                                        };
                                        u64::from(
                                            (((values[5]) as i32 as i64 as u64) as i64)
                                                == (((0x0u64) as i32 as i64 as u64) as i64),
                                        )
                                    }) != 0),
                            )) != 0)
                                || (({
                                    let _ = {
                                        values[5] = ({
                                            let call_arguments = [
                                                relocate("STACK_local_30"),
                                                relocate("DAT_1400e4c60"),
                                                0x10u64,
                                            ];
                                            invoke("memcmp", &call_arguments, memory)
                                        })
                                            as i32
                                            as i64
                                            as u64;
                                        values[5]
                                    };
                                    u64::from(
                                        (((values[5]) as i32 as i64 as u64) as i64)
                                            == (((0x0u64) as i32 as i64 as u64) as i64),
                                    )
                                }) != 0),
                        )) != 0),
                )) != 0
                {
                    7
                } else {
                    5
                };
            }
            9 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_38"),
                        values[10],
                        (values[0]).wrapping_add(0x20u64),
                        (values[0]).wrapping_add(0x28u64),
                        (values[0]).wrapping_add(0x30u64),
                        (((values[0]) as i32 as i64 as u64).wrapping_add(0x38u64)) as i32 as i64
                            as u64,
                        (((u64::from(
                            (u64::from(
                                memory.read_u32(relocate("STACK_in_stack_ffffffffffffff6c"))?,
                            )) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl((0x20u64) as u32))
                            | ((0x64u64) & (0xffffffffu64)),
                    ];
                    invoke("GdiplusHelpers::LoadFileImage", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                values[10] = memory.read_u64(values[1])?;
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[1]).wrapping_add((0x18u64).wrapping_mul(1)))?),
                )) != 0
                {
                    10
                } else {
                    9
                };
            }
            12 => {
                values[10] = values[1];
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_uStack_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                pc = if (u64::from(!((values[3]) != 0))) != 0 {
                    15
                } else {
                    1
                };
            }
            17 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                pc = if (values[11]) != 0 { 17 } else { 16 };
            }
            19 => {
                values[3] = 0u64;
                pc = 18;
            }
            20 => {
                pc = 18;
            }
            21 => {
                values[3] = 1u64;
                pc = 20;
            }
            22 => {
                pc = if (values[3]) != 0 { 21 } else { 19 };
            }
            23 => {
                values[3] = u64::from(
                    ({
                        let call_arguments =
                            [relocate("STACK_local_78"), relocate("STACK_local_68")];
                        invoke(
                            "Paint::PlatformHelpers::EqualIgnoreCase",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_uStack_70");
                    let stored =
                        memory.read_u64((values[6]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_uStack_5c");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_uStack_70")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_uStack_70"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_uStack_64");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_78")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_local_78"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 28;
            }
            30 => {
                values[9] = memory.read_u64(values[6])?;
                pc = 29;
            }
            31 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[6]).wrapping_add((0x3u64).wrapping_mul(8)))?),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                values[9] = values[6];
                pc = 31;
            }
            33 => {
                values[11] = 1u64;
                pc = 32;
            }
            34 => {
                values[6] = {
                    let call_arguments = [values[1], relocate("STACK_local_58")];
                    invoke("FUN_140019ef0", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_uStack_70");
                    let stored = 0x6u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x4u64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x0u64);
                    let stored = u64::from((0x401080f8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    19
                };
            }
            39 => {
                values[11] = 0u64;
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14001aa10` (`mspaint.c:27558`).
pub fn fun_14001aa10(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 3 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 3,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        (arguments[1]) as i32 as i64 as u64,
        (arguments[2]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 51;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14001ac00", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_140179140");
                    let stored = u64::from((values[4]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("DAT_140179144");
                    let stored = u64::from((values[7]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [values[0], relocate("STACK_local_28")];
                    invoke("FUN_14001bb70", &call_arguments, memory)
                };
                pc = 0;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x13u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = relocate("L\"IsBackgroundVisible\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14001bb70", &call_arguments, memory)
                };
                pc = 0;
            }
            8 => {
                {
                    let destination = relocate("DAT_140179148");
                    let stored = u64::from((values[2]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0xau64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = relocate("L\"Background\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400944c0", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_38")];
                    invoke("FUN_140094110", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_38"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[4]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_38"))?;
                    let stored = relocate("BackgroundColorChange::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    17
                } else {
                    15
                };
            }
            19 => {
                values[4] =
                    (u64::from((u64::from(memory.read_u32(relocate("DAT_140179148"))?)) as u32))
                        as i32 as i64 as u64;
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = {
                        let call_arguments = [0x10u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("DAT_1401d0c74"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    11
                };
            }
            22 => {
                values[5] = memory.read_u64(relocate("DAT_1401d0c78"))?;
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [values[8], 0x0u64, values[6]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                values[8] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            26 => {
                pc = if (u64::from(
                    (((((values[4]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    22
                };
            }
            27 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    24
                } else {
                    26
                };
            }
            28 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 27;
            }
            29 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 28;
            }
            30 => {
                values[4] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 29;
            }
            31 => {
                values[3] = (memory.read_u64(relocate("STACK_local_38"))?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                pc = 30;
            }
            32 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 31;
            }
            33 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    32
                } else {
                    22
                };
            }
            34 => {
                values[6] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140094340", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = ((u64::from(
                        ((memory.read_word(relocate("STACK_local_30"), 5)?) & 0xffffffffffu64)
                            as u32,
                    )) & 0xffffffffffu64)
                        & 0xffffffffffu64;
                    memory.write_word(destination, 5, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ca8")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 35;
            }
            38 => {
                {
                    let destination = relocate("DAT_1401d0c74");
                    let stored = (0x1u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    (u64::from((0x1u64) as u32))
                        < (u64::from(
                            (u64::from(
                                (((u64::from(memory.read_u32(relocate("DAT_1401d0c74"))?)) as i32
                                    as i64 as u64)
                                    .wrapping_sub(0x3u64)) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    38
                } else {
                    35
                };
            }
            40 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("DAT_1401d0c74"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    36
                } else {
                    39
                };
            }
            41 => {
                {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x17u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = relocate("L\"Change background color\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = ((u64::from(
                        ((memory.read_word(relocate("STACK_local_30"), 5)?) & 0xffffffffffu64)
                            as u32,
                    )) & 0xffffffffffu64)
                        & 0xffffffffffu64;
                    memory.write_word(destination, 5, stored)?;
                }
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    (u64::from((values[2]) as u32))
                        == (u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("DAT_140179148"))?)) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    6
                } else {
                    44
                };
            }
            46 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14000a4a0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a4a0"));
            }
            47 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    45
                };
            }
            48 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[1]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                let _ = {
                                    values[4] = (u64::from(
                                        (u64::from(memory.read_u32(relocate("DAT_140179140"))?))
                                            as u32,
                                    )) as i32 as i64
                                        as u64;
                                    values[4]
                                };
                                {
                                    values[7] = (values[2]) as i32 as i64 as u64;
                                    values[7]
                                }
                            };
                            u64::from(
                                (((values[1]) as i32 as i64 as u64) as i64)
                                    == (((0x1u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    3
                } else {
                    47
                };
            }
            49 => {
                values[7] =
                    (u64::from((u64::from(memory.read_u32(relocate("DAT_140179144"))?)) as u32))
                        as i32 as i64 as u64;
                pc = 48;
            }
            50 => {
                values[4] = (values[2]) as i32 as i64 as u64;
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_58"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140028d90` (`mspaint.c:39305`).
pub fn fun_140028d90(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 4 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 4,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 60;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let call_arguments = [
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_88"))?)) as u32),
                        values[1],
                        relocate("STACK_local_88"),
                    ];
                    invoke("FUN_140028fa0", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("STACK_uStack_7c");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[2]).wrapping_add(0xcu64))?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored = u64::from(
                        (u64::from(
                            (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                                as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("STACK_uStack_84");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[2]).wrapping_add(0x4u64))?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored =
                        u64::from((u64::from((memory.read_u64(values[2])?) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[0], values[1], values[10], values[3]];
                    invoke("FUN_14002ba80", &call_arguments, memory)
                };
                pc = 0;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = relocate("STACK_local_88");
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_uStack_7c");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[2]).wrapping_add(0xcu64))?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored = u64::from(
                        (u64::from(
                            (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                                as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_uStack_84");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[2]).wrapping_add(0x4u64))?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored =
                        u64::from((u64::from((memory.read_u64(values[2])?) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                values[14] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 12;
            }
            14 => {
                values[10] = (values[11]) as i32 as i64 as u64;
                pc = 13;
            }
            15 => {
                pc = if (0x1u64) != 0 { 42 } else { 14 };
            }
            16 => {
                pc = 14;
            }
            17 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[4]) as i32 as i64 as u64) as i64)
                                < (((values[12]) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                                    as u64) as i64)
                                    < (((values[4]) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        || (({
                            let _ = {
                                let _ = {
                                    let call_arguments = [relocate("STACK_local_68")];
                                    invoke("FUN_14002c190", &call_arguments, memory)
                                };
                                {
                                    values[11] = (values[12]) as i32 as i64 as u64;
                                    values[11]
                                }
                            };
                            u64::from(
                                ((memory.read_u64(relocate("STACK_local_68"))?) as i64)
                                    == ((memory.read_u64(relocate("STACK_local_60"))?) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    16
                } else {
                    15
                };
            }
            18 => {
                values[12] =
                    (u64::from(((values[11]).wrapping_add(values[5])) as u32)) as i32 as i64 as u64;
                pc = 17;
            }
            19 => {
                pc = 14;
            }
            20 => {
                pc = 12;
            }
            21 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    19
                };
            }
            22 => {
                pc = if (u64::from(
                    ((((u64::from(((0x7fffffffu64).wrapping_sub(values[5])) as u32)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((values[11]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    18
                };
            }
            23 => {
                values[5] = u64::from((0x1u64) as u32);
                pc = 22;
            }
            24 => {
                pc = if (u64::from((values[8]) != (relocate("DAT_14010bd28")))) != 0 {
                    28
                } else {
                    23
                };
            }
            25 => {
                values[8] = (values[8]).wrapping_add((0x1u64).wrapping_mul(4));
                pc = 24;
            }
            26 => {
                values[5] = u64::from((u64::from(((values[5]) ^ (0x3u64)) as u32)) as u32);
                pc = 25;
            }
            27 => {
                pc = 22;
            }
            28 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_50"))?)) as u32,
                        )) as u32,
                    )) < (u64::from(
                        (u64::from((u64::from(memory.read_u32(values[8])?)) as u32)) as u32,
                    )),
                )) != 0
                {
                    27
                } else {
                    26
                };
            }
            29 => {
                values[5] = u64::from((0x1u64) as u32);
                pc = 28;
            }
            30 => {
                values[5] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((values[7]).wrapping_add(
                                    ((0u64.wrapping_sub(0x50042e80u64)) as i32 as i64 as u64)
                                        .wrapping_mul(4),
                                ) as i64)
                                    .wrapping_shr((0x2u64) as u32)
                                    as u64) as u32,
                            )) & (0x1u64)) as u32,
                        ))
                        .wrapping_add(0x1u64)) as u32,
                    )) as u32,
                );
                pc = 22;
            }
            31 => {
                pc = if (u64::from(((0x0u64) as i64) < ((values[13]) as i64))) != 0 {
                    36
                } else {
                    30
                };
            }
            32 => {
                values[9] = values[13];
                pc = 31;
            }
            33 => {
                values[13] = (values[9]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_sub(values[13]),
                );
                pc = 32;
            }
            34 => {
                values[7] = ((values[7]).wrapping_add((values[13]).wrapping_mul(4)))
                    .wrapping_add((0x1u64).wrapping_mul(4));
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[7]).wrapping_add((values[13]).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        )) as u32,
                    )) <= (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_50"))?)) as u32,
                        )) as u32,
                    )),
                )) != 0
                {
                    34
                } else {
                    32
                };
            }
            36 => {
                values[13] = (values[9]).wrapping_shr((0x1u64) as u32);
                pc = 35;
            }
            37 => {
                values[9] = 0xcau64;
                pc = 36;
            }
            38 => {
                values[7] = relocate("DAT_14010ba00");
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_50"))?)) as u32,
                        )) as u32,
                    )) < (u64::from((0x25fdu64) as u32)),
                )) != 0
                {
                    29
                } else {
                    38
                };
            }
            40 => {
                pc = 14;
            }
            41 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[11]) as i32 as i64 as u64) as i64)
                            == (((values[4]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                < (((values[4]) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    40
                } else {
                    39
                };
            }
            42 => {
                values[8] = relocate("DAT_14010ba00");
                pc = 41;
            }
            43 => {
                values[11] = (0x0u64) as i32 as i64 as u64;
                pc = 42;
            }
            44 => {
                values[10] = (0x7fffffffu64) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_68"))?) as i64)
                        != ((memory.read_u64(relocate("STACK_local_60"))?) as i64),
                )) != 0
                {
                    44
                } else {
                    14
                };
            }
            46 => {
                values[11] = (0x0u64) as i32 as i64 as u64;
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("STACK_local_4c");
                    let stored = ((u64::from(
                        ((memory.read_word(relocate("STACK_local_4c"), 5)?) & 0xffffffffffu64)
                            as u32,
                    )) & 0xffffffffffu64)
                        & 0xffffffffffu64;
                    memory.write_word(destination, 5, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = memory.read_u64(values[6])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                values[6] = {
                    let call_arguments = [
                        relocate("STACK_local_88"),
                        memory.read_u64(relocate("STACK_local_68"))?,
                        values[14],
                        relocate("STACK_local_50"),
                    ];
                    invoke("FUN_14002c790", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_68"))?) as i64)
                        != ((values[14]) as i64),
                )) != 0
                {
                    49
                } else {
                    47
                };
            }
            51 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_44");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_4c");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                values[14] = (memory.read_u64(relocate("STACK_local_68"))?).wrapping_add(
                    (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        .wrapping_mul(0x2u64),
                );
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[4]) as i32 as i64 as u64) as i64)
                            < (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                                as i64 as u64) as i64)
                                < (((0x1u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    5
                } else {
                    57
                };
            }
            59 => {
                values[4] = ((u64::from(
                    memory.read_u32((values[3]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002c850` (`mspaint.c:42346`).
pub fn fun_14002c850(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        (arguments[4]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 72;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add(values[9]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[1], values[10], values[9]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[9]) as i64),
                )) != 0
                {
                    2
                } else {
                    5
                };
            }
            7 => {
                values[9] = (relocate("STACK_local_18")).wrapping_sub(values[10]);
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    7
                };
            }
            9 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[8]) & (0x1fu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = values[6];
                pc = 9;
            }
            11 => {
                values[6] = u64::from(
                    (((u64::from((values[3]) as u16) as i32).wrapping_shr((0x5u64) as u32) as u64)
                        as i32 as i64 as u64) as u16,
                );
                pc = 10;
            }
            12 => {
                values[8] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 11;
            }
            13 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 12;
            }
            14 => {
                pc = 7;
            }
            15 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    14
                };
            }
            16 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[8]) & (0xfu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 15;
            }
            17 => {
                values[3] = values[6];
                pc = 16;
            }
            18 => {
                values[6] = u64::from(
                    (((u64::from((values[3]) as u16) as i32).wrapping_shr((0x4u64) as u32) as u64)
                        as i32 as i64 as u64) as u16,
                );
                pc = 17;
            }
            19 => {
                values[8] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 18;
            }
            20 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 19;
            }
            21 => {
                pc = 7;
            }
            22 => {
                pc = if (u64::from(
                    ((((values[3]) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    21
                };
            }
            23 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                (({
                                    let numerator = (values[7]) as i64;
                                    let denominator = (values[4]) as i64;
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )? as u64
                                }) as i32 as i64 as u64)
                                    .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 22;
            }
            24 => {
                values[3] = ({
                    let numerator = (values[7]) as i64;
                    let denominator = (values[4]) as i64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) & (0xffffffffu64);
                pc = 23;
            }
            25 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 24;
            }
            26 => {
                values[7] = (values[3]) & (0xffffu64);
                pc = 25;
            }
            27 => {
                pc = 7;
            }
            28 => {
                pc = if (u64::from(
                    ((((values[7]) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    32
                } else {
                    27
                };
            }
            29 => {
                values[3] = values[7];
                pc = 28;
            }
            30 => {
                {
                    let destination = values[10];
                    let stored = ((((((values[3]) as i8 as i64 as u64).wrapping_add(
                        (((values[7]) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 29;
            }
            31 => {
                values[7] = {
                    let numerator = values[3];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 30;
            }
            32 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 31;
            }
            33 => {
                pc = 7;
            }
            34 => {
                pc = if (u64::from(
                    ((((values[3]) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    33
                };
            }
            35 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[5]) & (0x7u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 34;
            }
            36 => {
                values[3] = (values[3]).wrapping_shr((0x3u64) as u32);
                pc = 35;
            }
            37 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 36;
            }
            38 => {
                values[5] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 37;
            }
            39 => {
                pc = 7;
            }
            40 => {
                pc = if (u64::from(
                    ((((values[3]) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    39
                };
            }
            41 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[5]) & (0x3u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 40;
            }
            42 => {
                values[3] = (values[3]).wrapping_shr((0x2u64) as u32);
                pc = 41;
            }
            43 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 42;
            }
            44 => {
                values[5] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 43;
            }
            45 => {
                pc = 7;
            }
            46 => {
                pc = if (u64::from(
                    ((((values[3]) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    45
                };
            }
            47 => {
                {
                    let destination = values[10];
                    let stored = (((({
                        let numerator = (values[7]) as i64;
                        let denominator = (values[4]) as i64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    }) as i8 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 46;
            }
            48 => {
                values[3] = ({
                    let numerator = (values[7]) as i64;
                    let denominator = (values[4]) as i64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) & (0xffffffffu64);
                pc = 47;
            }
            49 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 48;
            }
            50 => {
                values[7] = (values[3]) & (0xffffu64);
                pc = 49;
            }
            51 => {
                pc = 7;
            }
            52 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    57
                } else {
                    51
                };
            }
            53 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[5]) & (0x1u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 52;
            }
            54 => {
                values[3] = values[6];
                pc = 53;
            }
            55 => {
                values[6] = u64::from(
                    (((u64::from((values[3]) as u16) as i32).wrapping_shr((0x1u64) as u32) as u64)
                        as i32 as i64 as u64) as u16,
                );
                pc = 54;
            }
            56 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 55;
            }
            57 => {
                values[5] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 56;
            }
            58 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x20u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    26
                };
            }
            59 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x10u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0xau64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    32
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    60
                };
            }
            62 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    61
                };
            }
            63 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x9u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    62
                };
            }
            64 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x7u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    63
                };
            }
            65 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x6u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    64
                };
            }
            66 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    65
                };
            }
            67 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    66
                };
            }
            68 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    57
                } else {
                    67
                };
            }
            69 => {
                values[11] = (values[4]) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                values[10] = relocate("STACK_local_18");
                pc = 69;
            }
            71 => {
                values[3] &= 0xffffu64;
                pc = 70;
            }
            72 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_48"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002ca70` (`mspaint.c:42453`).
pub fn fun_14002ca70(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        (arguments[4]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 72;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add(values[10]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[1], values[11], values[10]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[10]) as i64),
                )) != 0
                {
                    2
                } else {
                    5
                };
            }
            7 => {
                values[10] = (relocate("STACK_local_10")).wrapping_sub(values[11]);
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    ((((values[3]) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    7
                };
            }
            9 => {
                {
                    let destination = values[11];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[9]) & (0x1fu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = (values[3]).wrapping_shr((0x5u64) as u32);
                pc = 9;
            }
            11 => {
                values[9] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 10;
            }
            12 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 11;
            }
            13 => {
                pc = 7;
            }
            14 => {
                pc = if (u64::from(
                    ((((values[3]) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    13
                };
            }
            15 => {
                {
                    let destination = values[11];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[9]) & (0xfu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 14;
            }
            16 => {
                values[3] = (values[3]).wrapping_shr((0x4u64) as u32);
                pc = 15;
            }
            17 => {
                values[9] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 16;
            }
            18 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 17;
            }
            19 => {
                pc = 7;
            }
            20 => {
                pc = if (u64::from(
                    ((((values[3]) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    24
                } else {
                    19
                };
            }
            21 => {
                {
                    let destination = values[11];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                (({
                                    let numerator = (values[8]) as i64;
                                    let denominator = (values[4]) as i64;
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )? as u64
                                }) as i32 as i64 as u64)
                                    .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 20;
            }
            22 => {
                values[3] = ({
                    let numerator = (values[8]) as i64;
                    let denominator = (values[4]) as i64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) & (0xffffffffu64);
                pc = 21;
            }
            23 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 22;
            }
            24 => {
                values[8] = (values[3]) & (0xffu64);
                pc = 23;
            }
            25 => {
                pc = 7;
            }
            26 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    25
                };
            }
            27 => {
                values[3] = {
                    let numerator = values[3];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 26;
            }
            28 => {
                {
                    let destination = values[11];
                    let stored = ((((((values[3]) as i8 as i64 as u64).wrapping_add(
                        ((values[5]).wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 27;
            }
            29 => {
                values[5] = (({
                    let numerator = values[3];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                }) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 28;
            }
            30 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 29;
            }
            31 => {
                pc = 7;
            }
            32 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    31
                };
            }
            33 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        (((((values[6]) & (0x7u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                values[3] = values[7];
                pc = 33;
            }
            35 => {
                values[7] = u64::from(
                    (((values[6] as i32).wrapping_shr((0x3u64) as u32) as u64) as i32 as i64 as u64)
                        as u8,
                );
                pc = 34;
            }
            36 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 35;
            }
            37 => {
                values[6] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 36;
            }
            38 => {
                pc = 7;
            }
            39 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    38
                };
            }
            40 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        (((((values[6]) & (0x3u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 39;
            }
            41 => {
                values[3] = values[7];
                pc = 40;
            }
            42 => {
                values[7] = u64::from(
                    (((values[6] as i32).wrapping_shr((0x2u64) as u32) as u64) as i32 as i64 as u64)
                        as u8,
                );
                pc = 41;
            }
            43 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 42;
            }
            44 => {
                values[6] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 43;
            }
            45 => {
                pc = 7;
            }
            46 => {
                pc = if (u64::from(
                    ((((values[3]) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    45
                };
            }
            47 => {
                {
                    let destination = values[11];
                    let stored = (((({
                        let numerator = (values[8]) as i64;
                        let denominator = (values[4]) as i64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    }) as i8 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 46;
            }
            48 => {
                values[3] = ({
                    let numerator = (values[8]) as i64;
                    let denominator = (values[4]) as i64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) & (0xffffffffu64);
                pc = 47;
            }
            49 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 48;
            }
            50 => {
                values[8] = (values[3]) & (0xffu64);
                pc = 49;
            }
            51 => {
                pc = 7;
            }
            52 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    57
                } else {
                    51
                };
            }
            53 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        (((((values[6]) & (0x1u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 52;
            }
            54 => {
                values[3] = values[7];
                pc = 53;
            }
            55 => {
                values[7] = u64::from(
                    (((values[6] as i32).wrapping_shr((0x1u64) as u32) as u64) as i32 as i64 as u64)
                        as u8,
                );
                pc = 54;
            }
            56 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 55;
            }
            57 => {
                values[6] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 56;
            }
            58 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x20u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    24
                };
            }
            59 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x10u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0xau64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    60
                };
            }
            62 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    61
                };
            }
            63 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x9u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    62
                };
            }
            64 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x7u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    63
                };
            }
            65 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x6u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    64
                };
            }
            66 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    65
                };
            }
            67 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    66
                };
            }
            68 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    57
                } else {
                    67
                };
            }
            69 => {
                values[12] = (values[4]) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                values[11] = relocate("STACK_local_10");
                pc = 69;
            }
            71 => {
                values[3] &= 0xffu64;
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_38"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002cc90` (`mspaint.c:42561`).
pub fn fun_14002cc90(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        (arguments[4]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 78;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = (values[8]).wrapping_add((values[9]).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[8], values[11], values[9]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[8])) as i64) < ((values[9]) as i64),
                )) != 0
                {
                    2
                } else {
                    5
                };
            }
            7 => {
                values[9] = (relocate("STACK_local_18")).wrapping_sub(values[11]);
                pc = 6;
            }
            8 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    12
                } else {
                    7
                };
            }
            9 => {
                {
                    let destination = values[11];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[5]) & (0x1fu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = (values[3]).wrapping_shr((0x5u64) as u32);
                pc = 9;
            }
            11 => {
                values[5] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 10;
            }
            12 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 11;
            }
            13 => {
                pc = 7;
            }
            14 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    18
                } else {
                    13
                };
            }
            15 => {
                {
                    let destination = values[11];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[5]) & (0xfu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 14;
            }
            16 => {
                values[3] = (values[3]).wrapping_shr((0x4u64) as u32);
                pc = 15;
            }
            17 => {
                values[5] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 16;
            }
            18 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 17;
            }
            19 => {
                pc = 7;
            }
            20 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    24
                } else {
                    19
                };
            }
            21 => {
                values[3] = values[6];
                pc = 20;
            }
            22 => {
                {
                    let destination = values[11];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                ({
                                    let numerator = values[3];
                                    let denominator = values[7];
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )?
                                })
                                .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 21;
            }
            23 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 22;
            }
            24 => {
                values[6] = {
                    let numerator = values[3];
                    let denominator = values[7];
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 23;
            }
            25 => {
                pc = 7;
            }
            26 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    30
                } else {
                    25
                };
            }
            27 => {
                values[3] = values[7];
                pc = 26;
            }
            28 => {
                {
                    let destination = values[11];
                    let stored = ((((((values[3]) as i8 as i64 as u64).wrapping_add(
                        (((values[7]) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 27;
            }
            29 => {
                values[7] = {
                    let numerator = values[3];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 28;
            }
            30 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 29;
            }
            31 => {
                pc = 7;
            }
            32 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    36
                } else {
                    31
                };
            }
            33 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        (((((values[10]) & (0x7u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                values[3] = (values[3]).wrapping_shr((0x3u64) as u32);
                pc = 33;
            }
            35 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 34;
            }
            36 => {
                values[10] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 35;
            }
            37 => {
                pc = 7;
            }
            38 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    42
                } else {
                    37
                };
            }
            39 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        (((((values[10]) & (0x3u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 38;
            }
            40 => {
                values[3] = (values[3]).wrapping_shr((0x2u64) as u32);
                pc = 39;
            }
            41 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 40;
            }
            42 => {
                values[10] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 41;
            }
            43 => {
                pc = 7;
            }
            44 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    48
                } else {
                    43
                };
            }
            45 => {
                values[3] = values[6];
                pc = 44;
            }
            46 => {
                {
                    let destination = values[11];
                    let stored = (((({
                        let numerator = values[3];
                        let denominator = values[7];
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) as i8 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 45;
            }
            47 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 46;
            }
            48 => {
                values[6] = {
                    let numerator = values[3];
                    let denominator = values[7];
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 47;
            }
            49 => {
                pc = 7;
            }
            50 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    54
                } else {
                    49
                };
            }
            51 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        (((((values[10]) & (0x1u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 50;
            }
            52 => {
                values[3] = (values[3]).wrapping_shr((0x1u64) as u32);
                pc = 51;
            }
            53 => {
                values[11] =
                    (values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 52;
            }
            54 => {
                values[10] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x20u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    24
                };
            }
            56 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x10u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    55
                };
            }
            57 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0xau64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    56
                };
            }
            58 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    36
                } else {
                    57
                };
            }
            59 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    42
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x9u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x7u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    60
                };
            }
            62 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x6u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    61
                };
            }
            63 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    62
                };
            }
            64 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    63
                };
            }
            65 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    54
                } else {
                    64
                };
            }
            66 => {
                values[12] = (values[4]) as i32 as i64 as u64;
                pc = 65;
            }
            67 => {
                values[11] = relocate("STACK_local_18");
                pc = 66;
            }
            68 => {
                values[7] = values[4];
                pc = 67;
            }
            69 => {
                values[3] = 0u64.wrapping_sub(values[3]);
                pc = 68;
            }
            70 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x2du64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 69;
            }
            71 => {
                values[8] = (values[1]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 70;
            }
            72 => {
                return Ok(Some(values[0]));
            }
            73 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                pc = if (u64::from((values[1]) == (values[2]))) != 0 {
                    74
                } else {
                    71
                };
            }
            76 => {
                pc = if (u64::from(((values[3]) as i64) < ((0x0u64) as i64))) != 0 {
                    75
                } else {
                    68
                };
            }
            77 => {
                values[8] = values[1];
                pc = 76;
            }
            78 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002cef0` (`mspaint.c:42677`).
pub fn fun_14002cef0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        u64::from((arguments[3]) as u32),
        u64::from((arguments[4]) as u32),
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 76;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = (values[8]).wrapping_add((values[9]).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[8], values[10], values[9]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[8])) as i64) < ((values[9]) as i64),
                )) != 0
                {
                    2
                } else {
                    5
                };
            }
            7 => {
                values[9] = (relocate("STACK_local_18")).wrapping_sub(values[10]);
                pc = 6;
            }
            8 => {
                pc = if (u64::from((u64::from((values[6]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    12
                } else {
                    7
                };
            }
            9 => {
                values[3] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x5u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 8;
            }
            10 => {
                values[6] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x5u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 9;
            }
            11 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[3]) & (0x1fu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 10;
            }
            12 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 11;
            }
            13 => {
                pc = 7;
            }
            14 => {
                pc = if (u64::from((u64::from((values[6]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    18
                } else {
                    13
                };
            }
            15 => {
                values[3] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x4u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 14;
            }
            16 => {
                values[6] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x4u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 15;
            }
            17 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[3]) & (0xfu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 16;
            }
            18 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 17;
            }
            19 => {
                pc = 7;
            }
            20 => {
                pc = if (u64::from((u64::from((values[3]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    24
                } else {
                    19
                };
            }
            21 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                ({
                                    let numerator = values[7];
                                    let denominator = values[4];
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )?
                                })
                                .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 20;
            }
            22 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 21;
            }
            23 => {
                values[3] = u64::from(
                    (u64::from(
                        ({
                            let numerator = (values[3]) as u32;
                            let denominator = (values[4]) as u32;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }) as u32,
                    )) as u32,
                );
                pc = 22;
            }
            24 => {
                values[7] = values[3];
                pc = 23;
            }
            25 => {
                pc = 7;
            }
            26 => {
                pc = if (u64::from((u64::from((values[3]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    29
                } else {
                    25
                };
            }
            27 => {
                values[3] = u64::from(
                    (u64::from(
                        ({
                            let numerator = values[3];
                            let denominator = 0xau64;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        }) as u32,
                    )) as u32,
                );
                pc = 26;
            }
            28 => {
                {
                    let destination = values[10];
                    let stored = ((((((values[3]) as i8 as i64 as u64).wrapping_add(
                        ((({
                            let numerator = values[3];
                            let denominator = 0xau64;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        }) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 27;
            }
            29 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 28;
            }
            30 => {
                pc = 7;
            }
            31 => {
                pc = if (u64::from((u64::from((values[6]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    35
                } else {
                    30
                };
            }
            32 => {
                values[3] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x3u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 31;
            }
            33 => {
                values[6] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x3u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 32;
            }
            34 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((u64::from((values[3]) as u8)) & (0x7u64)) as i32 as i64 as u64)
                            .wrapping_add(0x30u64)) as i32 as i64 as u64)
                            as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 33;
            }
            35 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 34;
            }
            36 => {
                pc = 7;
            }
            37 => {
                pc = if (u64::from((u64::from((values[6]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    41
                } else {
                    36
                };
            }
            38 => {
                values[3] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x2u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 37;
            }
            39 => {
                values[6] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x2u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 38;
            }
            40 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((u64::from((values[3]) as u8)) & (0x3u64)) as i32 as i64 as u64)
                            .wrapping_add(0x30u64)) as i32 as i64 as u64)
                            as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 39;
            }
            41 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 40;
            }
            42 => {
                pc = 7;
            }
            43 => {
                pc = if (u64::from((u64::from((values[6]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    47
                } else {
                    42
                };
            }
            44 => {
                values[3] = u64::from((values[6]) as u32);
                pc = 43;
            }
            45 => {
                {
                    let destination = values[10];
                    let stored = ((((u64::from(
                        ({
                            let numerator = (values[3]) as u32;
                            let denominator = (values[4]) as u32;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_rem(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }) as u32,
                    )) as i8 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 44;
            }
            46 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 45;
            }
            47 => {
                values[6] = u64::from(
                    (u64::from(
                        ({
                            let numerator = (values[3]) as u32;
                            let denominator = (values[4]) as u32;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }) as u32,
                    )) as u32,
                );
                pc = 46;
            }
            48 => {
                pc = 7;
            }
            49 => {
                pc = if (u64::from((u64::from((values[3]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    53
                } else {
                    48
                };
            }
            50 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[5]) & (0x1u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 49;
            }
            51 => {
                values[3] = u64::from(
                    (u64::from(((values[3] as u32).wrapping_shr((0x1u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 50;
            }
            52 => {
                values[5] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 51;
            }
            53 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 52;
            }
            54 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x20u64) as u32)),
                )) != 0
                {
                    12
                } else {
                    24
                };
            }
            55 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x10u64) as u32)),
                )) != 0
                {
                    18
                } else {
                    54
                };
            }
            56 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0xau64) as u32)),
                )) != 0
                {
                    29
                } else {
                    55
                };
            }
            57 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x8u64) as u32)),
                )) != 0
                {
                    35
                } else {
                    56
                };
            }
            58 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x4u64) as u32)),
                )) != 0
                {
                    41
                } else {
                    57
                };
            }
            59 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x9u64) as u32)),
                )) != 0
                {
                    47
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x7u64) as u32)),
                )) != 0
                {
                    47
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x6u64) as u32)),
                )) != 0
                {
                    47
                } else {
                    60
                };
            }
            62 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x5u64) as u32)),
                )) != 0
                {
                    47
                } else {
                    61
                };
            }
            63 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x3u64) as u32)),
                )) != 0
                {
                    47
                } else {
                    62
                };
            }
            64 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x2u64) as u32)),
                )) != 0
                {
                    53
                } else {
                    63
                };
            }
            65 => {
                values[11] = u64::from((values[4]) as u32);
                pc = 64;
            }
            66 => {
                values[10] = relocate("STACK_local_18");
                pc = 65;
            }
            67 => {
                values[3] = u64::from((u64::from((0u64.wrapping_sub(values[3])) as u32)) as u32);
                pc = 66;
            }
            68 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x2du64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 67;
            }
            69 => {
                values[8] = (values[1]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 68;
            }
            70 => {
                return Ok(Some(values[0]));
            }
            71 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                pc = if (u64::from((values[1]) == (values[2]))) != 0 {
                    72
                } else {
                    69
                };
            }
            74 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    73
                } else {
                    66
                };
            }
            75 => {
                values[8] = values[1];
                pc = 74;
            }
            76 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_58"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002d120` (`mspaint.c:42789`).
pub fn fun_14002d120(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        (arguments[4]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 69;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add(values[7]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[1], values[9], values[7]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[7]) as i64),
                )) != 0
                {
                    2
                } else {
                    5
                };
            }
            7 => {
                values[7] = (relocate("STACK_local_18")).wrapping_sub(values[9]);
                pc = 6;
            }
            8 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    12
                } else {
                    7
                };
            }
            9 => {
                {
                    let destination = values[9];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[5]) & (0x1fu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = (values[3]).wrapping_shr((0x5u64) as u32);
                pc = 9;
            }
            11 => {
                values[5] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 10;
            }
            12 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 11;
            }
            13 => {
                pc = 7;
            }
            14 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    18
                } else {
                    13
                };
            }
            15 => {
                {
                    let destination = values[9];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[5]) & (0xfu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 14;
            }
            16 => {
                values[3] = (values[3]).wrapping_shr((0x4u64) as u32);
                pc = 15;
            }
            17 => {
                values[5] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 16;
            }
            18 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 17;
            }
            19 => {
                pc = 7;
            }
            20 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    24
                } else {
                    19
                };
            }
            21 => {
                values[3] = values[6];
                pc = 20;
            }
            22 => {
                {
                    let destination = values[9];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                ({
                                    let numerator = values[3];
                                    let denominator = values[10];
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )?
                                })
                                .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 21;
            }
            23 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 22;
            }
            24 => {
                values[6] = {
                    let numerator = values[3];
                    let denominator = values[10];
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 23;
            }
            25 => {
                pc = 7;
            }
            26 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    30
                } else {
                    25
                };
            }
            27 => {
                values[3] = values[10];
                pc = 26;
            }
            28 => {
                {
                    let destination = values[9];
                    let stored = ((((((values[3]) as i8 as i64 as u64).wrapping_add(
                        (((values[10]) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 27;
            }
            29 => {
                values[10] = {
                    let numerator = values[3];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 28;
            }
            30 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 29;
            }
            31 => {
                pc = 7;
            }
            32 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    36
                } else {
                    31
                };
            }
            33 => {
                {
                    let destination = values[9];
                    let stored = u64::from(
                        (((((values[8]) & (0x7u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                values[3] = (values[3]).wrapping_shr((0x3u64) as u32);
                pc = 33;
            }
            35 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 34;
            }
            36 => {
                values[8] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 35;
            }
            37 => {
                pc = 7;
            }
            38 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    42
                } else {
                    37
                };
            }
            39 => {
                {
                    let destination = values[9];
                    let stored = u64::from(
                        (((((values[8]) & (0x3u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 38;
            }
            40 => {
                values[3] = (values[3]).wrapping_shr((0x2u64) as u32);
                pc = 39;
            }
            41 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 40;
            }
            42 => {
                values[8] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 41;
            }
            43 => {
                pc = 7;
            }
            44 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    48
                } else {
                    43
                };
            }
            45 => {
                values[3] = values[6];
                pc = 44;
            }
            46 => {
                {
                    let destination = values[9];
                    let stored = (((({
                        let numerator = values[3];
                        let denominator = values[10];
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) as i8 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 45;
            }
            47 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 46;
            }
            48 => {
                values[6] = {
                    let numerator = values[3];
                    let denominator = values[10];
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 47;
            }
            49 => {
                pc = 7;
            }
            50 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    54
                } else {
                    49
                };
            }
            51 => {
                {
                    let destination = values[9];
                    let stored = u64::from(
                        (((((values[8]) & (0x1u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 50;
            }
            52 => {
                values[3] = (values[3]).wrapping_shr((0x1u64) as u32);
                pc = 51;
            }
            53 => {
                values[9] =
                    (values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 52;
            }
            54 => {
                values[8] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x20u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    24
                };
            }
            56 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x10u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    55
                };
            }
            57 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0xau64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    56
                };
            }
            58 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    36
                } else {
                    57
                };
            }
            59 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    42
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x9u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x7u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    60
                };
            }
            62 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x6u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    61
                };
            }
            63 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    62
                };
            }
            64 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    63
                };
            }
            65 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    54
                } else {
                    64
                };
            }
            66 => {
                values[11] = (values[4]) as i32 as i64 as u64;
                pc = 65;
            }
            67 => {
                values[9] = relocate("STACK_local_18");
                pc = 66;
            }
            68 => {
                values[10] = values[4];
                pc = 67;
            }
            69 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002ec80` (`mspaint.c:44098`).
pub fn fun_14002ec80(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        u64::from((arguments[4]) as u32),
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 164;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                values[14] = (values[14]).wrapping_add((values[17]).wrapping_mul(1));
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [values[14], 0x30u64, values[17]];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                values[14] = (values[1]).wrapping_add((0x2u64).wrapping_mul(1));
                pc = 4;
            }
            6 => {
                {
                    let destination = values[14];
                    let stored = (0x2eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                pc = 2;
            }
            8 => {
                pc = if (u64::from((u64::from((values[4]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    7
                } else {
                    6
                };
            }
            9 => {
                {
                    let destination = values[1];
                    let stored = (0x30u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                values[14] = (values[1]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 9;
            }
            11 => {
                pc = 17;
            }
            12 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64)
                        < (((u64::from(
                            ((((u64::from(
                                (u64::from((values[4]) as u32)) != (u64::from((0x0u64) as u32)),
                            ))
                            .wrapping_add(0x1u64)) as i32 as i64
                                as u64)
                                .wrapping_add(values[4])) as u32,
                        )) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                pc = 4;
            }
            14 => {
                pc = if (u64::from(
                    ((((values[17]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    3
                };
            }
            15 => {
                return Ok(Some(values[0]));
            }
            16 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[14])) as i64) < ((values[17]) as i64),
                )) != 0
                {
                    17
                } else {
                    14
                };
            }
            19 => {
                pc = 2;
            }
            20 => {
                pc = 17;
            }
            21 => {
                return Ok(Some(values[0]));
            }
            22 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = values[0];
                    let stored = (values[14]).wrapping_add((0x1u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = values[14];
                    let stored = (0x30u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[14]) != (values[2]))) != 0 {
                    24
                } else {
                    20
                };
            }
            26 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (0x2eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = (values[8]).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x30u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 26;
            }
            28 => {
                pc = if (u64::from((values[8]) != (values[2]))) != 0 {
                    27
                } else {
                    25
                };
            }
            29 => {
                {
                    let destination = values[12];
                    let stored = (0x31u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 28;
            }
            30 => {
                pc = if ({
                    let _ = {
                        values[8] = values[9];
                        values[8]
                    };
                    u64::from((values[12]) != (values[1]))
                }) != 0
                {
                    44
                } else {
                    29
                };
            }
            31 => {
                values[9] = values[8];
                pc = 30;
            }
            32 => {
                values[16] = (0x1u64) as i8 as i64 as u64;
                pc = 31;
            }
            33 => {
                {
                    let destination = values[12];
                    let stored = (0x30u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                pc = 2;
            }
            35 => {
                return Ok(Some(values[0]));
            }
            36 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = values[0];
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = values[12];
                    let stored = (((values[5]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i8
                        as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[16]) as i32 as i64 as u64) as i64)
                            == (((0x1u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            (((((values[5]) & (0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    38
                } else {
                    34
                };
            }
            40 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        != (((0x39u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    39
                } else {
                    33
                };
            }
            41 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        != (((0x2eu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    30
                };
            }
            42 => {
                values[9] = values[12];
                pc = 41;
            }
            43 => {
                values[12] = (values[12]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                );
                pc = 42;
            }
            44 => {
                values[5] = u64::from(
                    ((u64::from(memory.read_u8((values[12]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                    ))?)) as i8 as i64 as u64) as u8,
                );
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    19
                };
            }
            46 => {
                values[9] = values[2];
                pc = 45;
            }
            47 => {
                values[12] = values[14];
                pc = 46;
            }
            48 => {
                values[14] = (values[14]).wrapping_add((values[21]).wrapping_mul(1));
                pc = 47;
            }
            49 => {
                {
                    let call_arguments = [values[21], values[17], values[14]];
                    invoke("FUN_14002f250", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                pc = 17;
            }
            51 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[14])) as i64) < ((values[21]) as i64),
                )) != 0
                {
                    50
                } else {
                    49
                };
            }
            52 => {
                pc = if (u64::from(
                    (u64::from((values[21]) as u32)) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    51
                } else {
                    47
                };
            }
            53 => {
                values[16] =
                    (u64::from((u64::from((0x5u64) as u32)) < (u64::from((values[10]) as u32))))
                        as i8 as i64 as u64;
                pc = 52;
            }
            54 => {
                pc = 52;
            }
            55 => {
                values[16] = (0x2u64) as i8 as i64 as u64;
                pc = 54;
            }
            56 => {
                values[16] = (0x1u64) as i8 as i64 as u64;
                pc = 54;
            }
            57 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32 as i64
                            as u64) as i32 as i64 as u64) as i64)
                            < (((0x1u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            ((u64::from(
                                ((((u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32
                                    as i64 as u64) as i32 as i64
                                    as u64) as i64)
                                    < (((0x3cu64) as i32 as i64 as u64) as i64),
                            )) != 0)
                                && ((u64::from(
                                    ((memory.read_u64(relocate("STACK_local_70"))?)
                                        & (u64::from(
                                            ((((0x1u64 as i32).wrapping_shl(
                                                (((u64::from(
                                                    ((u64::from(
                                                        memory
                                                            .read_u32(relocate("STACK_local_78"))?,
                                                    ))
                                                        as i32
                                                        as i64
                                                        as u64)
                                                        as u8,
                                                )) & (0x3fu64))
                                                    as i32
                                                    as i64
                                                    as u64)
                                                    as u32,
                                            ) as u64)
                                                as i32
                                                as i64
                                                as u64)
                                                .wrapping_sub(0x1u64))
                                                as u32,
                                        )))
                                        == (0x0u64),
                                )) != 0),
                        )) != 0),
                )) != 0
                {
                    55
                } else {
                    56
                };
            }
            58 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = (u64::from(
                        ((u64::from((!(values[4])) as u32)).wrapping_sub(
                            (u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32 as i64
                                as u64,
                        )) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) == (u64::from((0x5u64) as u32)),
                )) != 0
                {
                    58
                } else {
                    53
                };
            }
            60 => {
                pc = if (u64::from((values[18]) != (0x0u64))) != 0 {
                    64
                } else {
                    59
                };
            }
            61 => {
                values[20] = values[17];
                pc = 60;
            }
            62 => {
                values[18] = (values[18]).wrapping_sub(0x1u64);
                pc = 61;
            }
            63 => {
                values[10] = u64::from(
                    ((((values[20]) as i32 as i64 as u64).wrapping_add(
                        (((values[17]) as i32 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64) as u32,
                );
                pc = 62;
            }
            64 => {
                values[17] = {
                    let numerator = values[20];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 63;
            }
            65 => {
                values[20] = values[17];
                pc = 64;
            }
            66 => {
                values[18] = u64::from(((0x9u64).wrapping_sub(values[21])) as u32);
                pc = 65;
            }
            67 => {
                pc = if (u64::from(
                    (u64::from((u64::from(((0x9u64).wrapping_sub(values[21])) as u32)) as u32))
                        != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    66
                } else {
                    53
                };
            }
            68 => {
                values[10] = u64::from((0x0u64) as u32);
                pc = 67;
            }
            69 => {
                values[21] = u64::from(
                    (u64::from(
                        ((values[4]).wrapping_add(u64::from(
                            ((values[13])
                                .wrapping_mul((0u64.wrapping_sub(0x9u64)) as i32 as i64 as u64))
                                as u32,
                        ))) as u32,
                    )) as u32,
                );
                pc = 68;
            }
            70 => {
                pc = if (0x1u64) != 0 { 89 } else { 69 };
            }
            71 => {
                return Ok(Some(values[0]));
            }
            72 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = values[0];
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                pc = if (u64::from(
                    (u64::from((values[21]) as u32)) <= (u64::from((values[13]) as u32)),
                )) != 0
                {
                    73
                } else {
                    70
                };
            }
            75 => {
                values[22] =
                    u64::from((u64::from(((values[22]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 74;
            }
            76 => {
                values[13] =
                    u64::from((u64::from(((values[13]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 75;
            }
            77 => {
                values[24] =
                    (values[24]).wrapping_add((0u64.wrapping_sub(0x9u64)) as i32 as i64 as u64);
                pc = 76;
            }
            78 => {
                values[14] = (values[14]).wrapping_add((0x9u64).wrapping_mul(1));
                pc = 77;
            }
            79 => {
                values[17] = values[10];
                pc = 78;
            }
            80 => {
                {
                    let call_arguments = [values[11], values[14]];
                    invoke("FUN_140030d70", &call_arguments, memory)
                };
                pc = 79;
            }
            81 => {
                pc = 17;
            }
            82 => {
                pc = if (u64::from(((values[24]) as i64) < ((0x9u64) as i64))) != 0 {
                    81
                } else {
                    80
                };
            }
            83 => {
                pc = 69;
            }
            84 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ({
                                let numerator = (values[4]) as u32;
                                let denominator = (0x9u64) as u32;
                                if denominator == 0 {
                                    return Err(AdapterError::Arithmetic("division by zero"));
                                }
                                numerator
                                    .checked_div(denominator)
                                    .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                    as u64
                            }) as u32,
                        )) as u32,
                    )) <= (u64::from((values[13]) as u32)),
                )) != 0
                {
                    83
                } else {
                    82
                };
            }
            85 => {
                values[17] = values[11];
                pc = 84;
            }
            86 => {
                values[11] = u64::from(
                    ({
                        let call_arguments = [
                            (memory.read_u64(relocate("STACK_local_70"))?)
                                .wrapping_shl((0x8u64) as u32),
                            (relocate("UNK_1400f1980"))
                                .wrapping_add(((values[22]).wrapping_mul(0x18u64)).wrapping_mul(8)),
                            ((((((0x8u64).wrapping_sub((values[17]) as i32 as i64 as u64)) as i32
                                as i64 as u64)
                                .wrapping_mul(0x10u64)) as i32 as i64
                                as u64)
                                .wrapping_sub(
                                    (u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32
                                        as i64 as u64,
                                )) as i32 as i64 as u64,
                        ];
                        invoke("FUN_14002f170", &call_arguments, memory)
                    }) as u32,
                );
                pc = 85;
            }
            87 => {
                pc = 18;
            }
            88 => {
                values[17] = u64::from(
                    ((values[4]).wrapping_add(u64::from(
                        ((values[13])
                            .wrapping_mul((0u64.wrapping_sub(0x9u64)) as i32 as i64 as u64))
                            as u32,
                    ))) as u32,
                );
                pc = 87;
            }
            89 => {
                pc = if (u64::from(
                    (u64::from((values[6]) as u32)) <= (u64::from((values[22]) as u32)),
                )) != 0
                {
                    88
                } else {
                    86
                };
            }
            90 => {
                values[6] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u16(
                                (relocate("DAT_14010c002")).wrapping_add(
                                    (((values[17]) as i32 as i64 as u64).wrapping_mul(0x2u64))
                                        .wrapping_mul(1),
                                ),
                            )?,
                        )) as u16,
                    )) as u16,
                );
                pc = 70;
            }
            91 => {
                values[24] = (values[2]).wrapping_sub(values[14]);
                pc = 90;
            }
            92 => {
                values[22] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                (u64::from(
                                    (u64::from(memory.read_u16(
                                        (relocate("DAT_14010c000")).wrapping_add(
                                            ((values[20]).wrapping_mul(0x2u64)).wrapping_mul(1),
                                        ),
                                    )?)) as u16,
                                )) as u32,
                            ))
                            .wrapping_sub(u64::from((values[5]) as u32)))
                                as u32,
                        ))
                        .wrapping_add(values[13])) as u32,
                    )) as u32,
                );
                pc = 91;
            }
            93 => {
                pc = if (u64::from(
                    (u64::from((values[13]) as u32)) < (u64::from((values[21]) as u32)),
                )) != 0
                {
                    92
                } else {
                    19
                };
            }
            94 => {
                values[13] = u64::from((u64::from((values[5]) as u32)) as u32);
                pc = 93;
            }
            95 => {
                values[14] = (values[14]).wrapping_add((values[18]).wrapping_mul(1));
                pc = 94;
            }
            96 => {
                values[17] = values[10];
                pc = 95;
            }
            97 => {
                {
                    let call_arguments = [values[14], 0x30u64, values[18]];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 96;
            }
            98 => {
                pc = if (u64::from(
                    (u64::from((values[13]) as u32)) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    97
                } else {
                    95
                };
            }
            99 => {
                pc = 17;
            }
            100 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[14])) as i64) < ((values[18]) as i64),
                )) != 0
                {
                    99
                } else {
                    98
                };
            }
            101 => {
                values[18] = values[13];
                pc = 100;
            }
            102 => {
                values[13] = u64::from(
                    (u64::from(
                        ((u64::from((values[5]) as u32)).wrapping_add(u64::from(
                            ((u64::from((values[5]) as u32)).wrapping_mul(0x8u64)) as u32,
                        ))) as u32,
                    )) as u32,
                );
                pc = 101;
            }
            103 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    102
                } else {
                    93
                };
            }
            104 => {
                values[13] = u64::from((0x0u64) as u32);
                pc = 103;
            }
            105 => {
                values[17] = values[20];
                pc = 104;
            }
            106 => {
                pc = if (u64::from(
                    (u64::from((values[5]) as u32)) < (u64::from((values[21]) as u32)),
                )) != 0
                {
                    105
                } else {
                    18
                };
            }
            107 => {
                values[21] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ({
                                let numerator = (values[4]) as u32;
                                let denominator = (0x9u64) as u32;
                                if denominator == 0 {
                                    return Err(AdapterError::Arithmetic("division by zero"));
                                }
                                numerator
                                    .checked_div(denominator)
                                    .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                    as u64
                            }) as u32,
                        ))
                        .wrapping_add(0x1u64)) as u32,
                    )) as u32,
                );
                pc = 106;
            }
            108 => {
                values[5] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("DAT_14010bfb0")).wrapping_add((values[20]).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 107;
            }
            109 => {
                values[20] = (values[10]) as i32 as i64 as u64;
                pc = 108;
            }
            110 => {
                values[10] = u64::from(
                    ((0u64.wrapping_sub(
                        (((u64::from(
                            (((u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32
                                as i64 as u64)
                                .wrapping_add(u64::from(
                                    (((((u64::from(memory.read_u32(relocate("STACK_local_78"))?))
                                        as i32 as i64 as u64
                                        as i32)
                                        .wrapping_shr((0x1fu64) as u32)
                                        as u64) as i32 as i64
                                        as u64)
                                        & (0xfu64)) as u32,
                                ))) as u32,
                        )) as i32 as i64 as u64 as i32)
                            .wrapping_shr((0x4u64) as u32) as u64) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as u32,
                );
                pc = 109;
            }
            111 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    110
                } else {
                    18
                };
            }
            112 => {
                values[14] = (values[14]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 111;
            }
            113 => {
                {
                    let destination = values[14];
                    let stored = (0x2eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 112;
            }
            114 => {
                pc = 17;
            }
            115 => {
                pc = if (u64::from((values[14]) == (values[2]))) != 0 {
                    114
                } else {
                    113
                };
            }
            116 => {
                pc = if (u64::from((u64::from((values[4]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    115
                } else {
                    111
                };
            }
            117 => {
                values[14] = (values[14]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 116;
            }
            118 => {
                {
                    let destination = values[14];
                    let stored = (0x30u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 117;
            }
            119 => {
                pc = 17;
            }
            120 => {
                pc = if (u64::from((values[14]) == (values[2]))) != 0 {
                    119
                } else {
                    118
                };
            }
            121 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored =
                        ((0u64.wrapping_sub(0x432u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 120;
            }
            122 => {
                pc = 120;
            }
            123 => {
                pc = if (u64::from(!((values[7]) != 0))) != 0 {
                    122
                } else {
                    116
                };
            }
            124 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[15]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    141
                } else {
                    123
                };
            }
            125 => {
                values[15] = (((values[15])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 124;
            }
            126 => {
                values[23] = (((values[23])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 125;
            }
            127 => {
                values[14] = (values[14]).wrapping_add((0x9u64).wrapping_mul(1));
                pc = 126;
            }
            128 => {
                {
                    let call_arguments = [values[10], values[14]];
                    invoke("FUN_140030d70", &call_arguments, memory)
                };
                pc = 127;
            }
            129 => {
                pc = 17;
            }
            130 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[14])) as i64) < ((0x9u64) as i64),
                )) != 0
                {
                    129
                } else {
                    128
                };
            }
            131 => {
                values[7] = 1u64;
                pc = 126;
            }
            132 => {
                values[14] = (values[14]).wrapping_add((values[18]).wrapping_mul(1));
                pc = 131;
            }
            133 => {
                {
                    let call_arguments = [values[18], (values[20]) & (0xffffffffu64), values[14]];
                    invoke("FUN_140030c60", &call_arguments, memory)
                };
                pc = 132;
            }
            134 => {
                pc = 17;
            }
            135 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[14])) as i64) < ((values[18]) as i64),
                )) != 0
                {
                    134
                } else {
                    133
                };
            }
            136 => {
                values[18] = values[10];
                pc = 135;
            }
            137 => {
                values[10] = u64::from(
                    ({
                        let call_arguments = [values[20]];
                        invoke("FUN_140030bf0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 136;
            }
            138 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    137
                } else {
                    126
                };
            }
            139 => {
                pc = if (values[7]) != 0 { 130 } else { 138 };
            }
            140 => {
                values[20] = values[10];
                pc = 139;
            }
            141 => {
                values[10] = u64::from(
                    ({
                        let call_arguments = [
                            (memory.read_u64(relocate("STACK_local_70"))?)
                                .wrapping_shl((0x8u64) as u32),
                            (relocate("UNK_1400ea6c0"))
                                .wrapping_add(((values[23]).wrapping_mul(0x18u64)).wrapping_mul(8)),
                            ((((values[19]).wrapping_sub(
                                (u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32
                                    as i64 as u64,
                            )) as i32 as i64 as u64)
                                .wrapping_add(0x8u64)) as i32 as i64
                                as u64,
                        ];
                        invoke("FUN_14002f170", &call_arguments, memory)
                    }) as u32,
                );
                pc = 140;
            }
            142 => {
                values[23] = (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                (u64::from(
                                    memory.read_u16(
                                        (relocate("DAT_14010bf30"))
                                            .wrapping_add((values[20]).wrapping_mul(2)),
                                    )?,
                                )) as u16,
                            )) as u16,
                        )) as u32,
                    ))
                    .wrapping_add(values[15])) as u32,
                )) as i32 as i64 as u64;
                pc = 141;
            }
            143 => {
                values[19] = (0x78u64) as i32 as i64 as u64;
                pc = 142;
            }
            144 => {
                values[15] = (0x1u64) as i32 as i64 as u64;
                pc = 143;
            }
            145 => {
                values[20] = 0x0u64;
                pc = 144;
            }
            146 => {
                pc = 120;
            }
            147 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    146
                } else {
                    142
                };
            }
            148 => {
                values[15] = (u64::from(
                    ((u64::from(
                        ({
                            let numerator = (u64::from(
                                ((u64::from(
                                    ((u64::from(((values[10]).wrapping_mul(0x134410u64)) as u32)
                                        as u32)
                                        .wrapping_shr((0x12u64) as u32)
                                        as u64) as u32,
                                ))
                                .wrapping_add(0x19u64)) as u32,
                            )) as u32;
                            let denominator = (0x9u64) as u32;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }) as u32,
                    ))
                    .wrapping_sub(0x1u64)) as u32,
                )) as i32 as i64 as u64;
                pc = 147;
            }
            149 => {
                values[19] = (u64::from(
                    ((u64::from(((values[10]).wrapping_mul(0x10u64)) as u32)).wrapping_add(0x78u64))
                        as u32,
                )) as i32 as i64 as u64;
                pc = 148;
            }
            150 => {
                values[20] = values[10];
                pc = 149;
            }
            151 => {
                values[10] = u64::from(
                    (u64::from(
                        ((u64::from(((values[10]).wrapping_sub(0x424u64)) as u32) as u32)
                            .wrapping_shr((0x4u64) as u32) as u64) as u32,
                    )) as u32,
                );
                pc = 150;
            }
            152 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    145
                } else {
                    151
                };
            }
            153 => {
                pc = 120;
            }
            154 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_78"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < ((((0u64.wrapping_sub(0x34u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    153
                } else {
                    152
                };
            }
            155 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_70"))?) | (0x10000000000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 154;
            }
            156 => {
                values[7] = 0u64;
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = (u64::from(((values[10]).wrapping_sub(0x433u64)) as u32)) as i32
                        as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 156;
            }
            158 => {
                values[10] = u64::from(
                    (u64::from(((values[10] as u32).wrapping_shr((0x14u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 157;
            }
            159 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((values[10] as u32).wrapping_shr((0x14u64) as u32) as u64) as u32,
                        )) as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    121
                } else {
                    158
                };
            }
            160 => {
                values[14] = values[1];
                pc = 159;
            }
            161 => {
                values[10] = u64::from(
                    (u64::from(((values[3]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 160;
            }
            162 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = (values[3]) & (0xfffffffffffffu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 161;
            }
            163 => {
                pc = if (u64::from((values[3]) == (0x0u64))) != 0 {
                    12
                } else {
                    162
                };
            }
            164 => {
                values[17] = values[4];
                pc = 163;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140030210` (`mspaint.c:45091`).
pub fn fun_140030210(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        u64::from((arguments[4]) as u32),
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 71;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add(values[6]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[1], values[10], values[6]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[6]) as i64),
                )) != 0
                {
                    2
                } else {
                    5
                };
            }
            7 => {
                values[6] = (relocate("STACK_local_18")).wrapping_sub(values[10]);
                pc = 6;
            }
            8 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    13
                } else {
                    7
                };
            }
            9 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[8]) & (0x1fu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = values[9];
                pc = 9;
            }
            11 => {
                values[9] = u64::from(
                    (u64::from(((values[8] as u32).wrapping_shr((0x5u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 10;
            }
            12 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 11;
            }
            13 => {
                values[8] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 12;
            }
            14 => {
                pc = 7;
            }
            15 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    20
                } else {
                    14
                };
            }
            16 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(memory.read_u8(
                        (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                            (u64::from(((values[8]) & (0xfu64)) as u32)).wrapping_mul(1),
                        ),
                    )?)) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 15;
            }
            17 => {
                values[3] = values[9];
                pc = 16;
            }
            18 => {
                values[9] = u64::from(
                    (u64::from(((values[8] as u32).wrapping_shr((0x4u64) as u32) as u64) as u32))
                        as u32,
                );
                pc = 17;
            }
            19 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 18;
            }
            20 => {
                values[8] = u64::from((u64::from((values[3]) as u32)) as u32);
                pc = 19;
            }
            21 => {
                pc = 7;
            }
            22 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    21
                };
            }
            23 => {
                {
                    let destination = values[10];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                ({
                                    let numerator = values[5];
                                    let denominator = values[4];
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )?
                                })
                                .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 22;
            }
            24 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 23;
            }
            25 => {
                values[3] = {
                    let numerator = values[5];
                    let denominator = values[4];
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 24;
            }
            26 => {
                values[5] = (values[3]) & (0xffffffffu64);
                pc = 25;
            }
            27 => {
                pc = 7;
            }
            28 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    32
                } else {
                    27
                };
            }
            29 => {
                values[3] = values[5];
                pc = 28;
            }
            30 => {
                {
                    let destination = values[10];
                    let stored = ((((((values[3]) as i8 as i64 as u64).wrapping_add(
                        (((values[5]) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 29;
            }
            31 => {
                values[5] = {
                    let numerator = (values[3]) & (0xffffffffu64);
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 30;
            }
            32 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 31;
            }
            33 => {
                pc = 7;
            }
            34 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    33
                };
            }
            35 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[7]) & (0x7u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 34;
            }
            36 => {
                values[3] = ((values[3]).wrapping_shr((0x3u64) as u32)) & (0x1fffffffu64);
                pc = 35;
            }
            37 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 36;
            }
            38 => {
                values[7] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 37;
            }
            39 => {
                pc = 7;
            }
            40 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    39
                };
            }
            41 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[7]) & (0x3u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 40;
            }
            42 => {
                values[3] = ((values[3]).wrapping_shr((0x2u64) as u32)) & (0x3fffffffu64);
                pc = 41;
            }
            43 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 42;
            }
            44 => {
                values[7] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 43;
            }
            45 => {
                pc = 7;
            }
            46 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    45
                };
            }
            47 => {
                {
                    let destination = values[10];
                    let stored = (((({
                        let numerator = values[5];
                        let denominator = values[4];
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) as i8 as i64 as u64)
                        .wrapping_add(0x30u64)) as i32 as i64
                        as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 46;
            }
            48 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 47;
            }
            49 => {
                values[3] = {
                    let numerator = values[5];
                    let denominator = values[4];
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 48;
            }
            50 => {
                values[5] = (values[3]) & (0xffffffffu64);
                pc = 49;
            }
            51 => {
                pc = 7;
            }
            52 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    57
                } else {
                    51
                };
            }
            53 => {
                {
                    let destination = values[10];
                    let stored = u64::from(
                        (((((values[7]) & (0x1u64)) as i32 as i64 as u64).wrapping_add(0x30u64))
                            as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 52;
            }
            54 => {
                values[3] = values[8];
                pc = 53;
            }
            55 => {
                values[8] = u64::from(
                    (u64::from(
                        ((u64::from((values[3]) as u32) as u32).wrapping_shr((0x1u64) as u32)
                            as u64) as u32,
                    )) as u32,
                );
                pc = 54;
            }
            56 => {
                values[10] =
                    (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 55;
            }
            57 => {
                values[7] = u64::from((u64::from((values[3]) as u8)) as u8);
                pc = 56;
            }
            58 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x20u64) as u32)),
                )) != 0
                {
                    13
                } else {
                    26
                };
            }
            59 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x10u64) as u32)),
                )) != 0
                {
                    20
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0xau64) as u32)),
                )) != 0
                {
                    32
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x8u64) as u32)),
                )) != 0
                {
                    38
                } else {
                    60
                };
            }
            62 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x4u64) as u32)),
                )) != 0
                {
                    44
                } else {
                    61
                };
            }
            63 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x9u64) as u32)),
                )) != 0
                {
                    50
                } else {
                    62
                };
            }
            64 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x7u64) as u32)),
                )) != 0
                {
                    50
                } else {
                    63
                };
            }
            65 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x6u64) as u32)),
                )) != 0
                {
                    50
                } else {
                    64
                };
            }
            66 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x5u64) as u32)),
                )) != 0
                {
                    50
                } else {
                    65
                };
            }
            67 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x3u64) as u32)),
                )) != 0
                {
                    50
                } else {
                    66
                };
            }
            68 => {
                pc = if (u64::from(
                    (u64::from((values[11]) as u32)) == (u64::from((0x2u64) as u32)),
                )) != 0
                {
                    57
                } else {
                    67
                };
            }
            69 => {
                values[11] = u64::from((values[4]) as u32);
                pc = 68;
            }
            70 => {
                values[10] = relocate("STACK_local_18");
                pc = 69;
            }
            71 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_58"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140030420` (`mspaint.c:45193`).
pub fn fun_140030420(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 5 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 5,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        u64::from((arguments[3]) as u32),
        (arguments[4]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 71;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                return Ok(Some(values[0]));
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add((0x4u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[1], relocate("\"0p+0\""), 0x4u64];
                    invoke("builtin_strncpy", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    ((0x3u64) as i64) < (((values[2]).wrapping_sub(values[1])) as i64),
                )) != 0
                {
                    6
                } else {
                    2
                };
            }
            8 => {
                return Ok(Some(values[0]));
            }
            9 => {
                {
                    let call_arguments = [
                        values[0],
                        (values[9]).wrapping_add((0x2u64).wrapping_mul(1)),
                        values[2],
                        values[5],
                        0xau64,
                    ];
                    invoke("FUN_140030210", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                values[5] = (values[12]) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(values[12])) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    9
                };
            }
            12 => {
                values[5] =
                    ((0u64.wrapping_sub(values[12])) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[9]).wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (values[6]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 12;
            }
            14 => {
                values[6] = (0x2bu64) as i8 as i64 as u64;
                pc = 13;
            }
            15 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[12]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    14
                } else {
                    13
                };
            }
            16 => {
                values[6] = (0x2du64) as i8 as i64 as u64;
                pc = 15;
            }
            17 => {
                {
                    let destination = values[9];
                    let stored = (0x70u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    ((0x1u64) as i64) < (((values[2]).wrapping_sub(values[9])) as i64),
                )) != 0
                {
                    17
                } else {
                    2
                };
            }
            19 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    25
                } else {
                    18
                };
            }
            20 => {
                values[8] = u64::from(
                    (u64::from(
                        ((values[8])
                            & (u64::from(
                                ((((0x1u64 as i32).wrapping_shl(
                                    (((values[7]) & (0x1fu64)) as i32 as i64 as u64) as u32,
                                ) as u64) as i32 as i64 as u64)
                                    .wrapping_sub(0x1u64)) as u32,
                            ))) as u32,
                    )) as u32,
                );
                pc = 19;
            }
            21 => {
                values[9] = (values[9]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 20;
            }
            22 => {
                {
                    let destination = values[9];
                    let stored = ((u64::from(
                        memory.read_u8(
                            (relocate("\"0123456789abcdefghijklmnopqrstuvwxyzinf\"")).wrapping_add(
                                (u64::from(
                                    ((values[8] as u32).wrapping_shr(
                                        (((values[7]) & (0x3fu64)) as i32 as i64 as u64) as u32,
                                    ) as u64) as u32,
                                ))
                                .wrapping_mul(1),
                            ),
                        )?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 21;
            }
            23 => {
                pc = 2;
            }
            24 => {
                pc = if (u64::from((values[9]) == (values[2]))) != 0 {
                    23
                } else {
                    22
                };
            }
            25 => {
                values[7] =
                    u64::from((((values[7]).wrapping_sub(0x4u64)) as i32 as i64 as u64) as u8);
                pc = 24;
            }
            26 => {
                values[8] = u64::from(
                    (u64::from(
                        ((u64::from(((values[8]) & (0x7fffffu64)) as u32)).wrapping_mul(0x2u64))
                            as u32,
                    )) as u32,
                );
                pc = 25;
            }
            27 => {
                values[7] = u64::from((0x18u64) as u8);
                pc = 26;
            }
            28 => {
                values[9] = (values[1]).wrapping_add((0x2u64).wrapping_mul(1));
                pc = 27;
            }
            29 => {
                {
                    let destination = values[9];
                    let stored = (0x2eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 28;
            }
            30 => {
                pc = 2;
            }
            31 => {
                pc = if (u64::from((values[9]) == (values[2]))) != 0 {
                    30
                } else {
                    29
                };
            }
            32 => {
                pc = if (u64::from(
                    (u64::from((u64::from(((values[8]) & (0x7fffffu64)) as u32)) as u32))
                        != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    31
                } else {
                    18
                };
            }
            33 => {
                {
                    let destination = values[1];
                    let stored = (((u64::from(
                        (u64::from(
                            (u64::from(
                                ((values[8] as u32).wrapping_shr((0x17u64) as u32) as u64) as u32,
                            )) as u32,
                        )) != (u64::from((0x0u64) as u32)),
                    ))
                    .wrapping_add(0x30u64)) as i32 as i64 as u64)
                        as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                values[9] = (values[1]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 33;
            }
            35 => {
                pc = if (u64::from((values[1]) != (values[2]))) != 0 {
                    34
                } else {
                    2
                };
            }
            36 => {
                values[12] = (u64::from(
                    ((u64::from(((values[8] as u32).wrapping_shr((0x17u64) as u32) as u64) as u32))
                        .wrapping_sub(0x7fu64)) as u32,
                )) as i32 as i64 as u64;
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((values[8] as u32).wrapping_shr((0x17u64) as u32) as u64) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                values[12] =
                    ((0u64.wrapping_sub(0x7eu64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    7
                } else {
                    38
                };
            }
            40 => {
                return Ok(Some(values[0]));
            }
            41 => {
                {
                    let call_arguments = [values[0], values[1], values[2], values[11], values[4]];
                    invoke("FUN_140031400", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    41
                } else {
                    39
                };
            }
            43 => {
                return Ok(Some(values[0]));
            }
            44 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add((values[10]).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [values[1], values[9], values[10]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                return Ok(Some(values[0]));
            }
            48 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[10]) as i64),
                )) != 0
                {
                    49
                } else {
                    46
                };
            }
            51 => {
                values[10] = 0x3u64;
                pc = 50;
            }
            52 => {
                values[9] = relocate("\"inf\"");
                pc = 51;
            }
            53 => {
                values[10] = 0x8u64;
                pc = 50;
            }
            54 => {
                values[9] = relocate("\"nan(ind)\"");
                pc = 53;
            }
            55 => {
                values[10] = 0x9u64;
                pc = 50;
            }
            56 => {
                values[9] = relocate("\"nan(snan)\"");
                pc = 55;
            }
            57 => {
                values[10] = 0x3u64;
                pc = 50;
            }
            58 => {
                values[9] = relocate("\"nan\"");
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((values[8] as u32).wrapping_shr((0x16u64) as u32) as u64) as u32,
                        )) as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    56
                } else {
                    58
                };
            }
            60 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                            < (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (u64::from((values[8]) as u32)) == (u64::from((0x400000u64) as u32)),
                        )) != 0),
                )) != 0
                {
                    54
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    52
                } else {
                    60
                };
            }
            62 => {
                values[8] = u64::from((u64::from(((values[8]) & (0x7fffffu64)) as u32)) as u32);
                pc = 61;
            }
            63 => {
                pc = if (u64::from(
                    (u64::from((u64::from(((values[8]) & (0x7f800000u64)) as u32)) as u32))
                        == (u64::from((0x7f800000u64) as u32)),
                )) != 0
                {
                    62
                } else {
                    42
                };
            }
            64 => {
                values[8] = u64::from((u64::from(((values[3]) & (0x7fffffffu64)) as u32)) as u32);
                pc = 63;
            }
            65 => {
                values[1] = (values[1]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 64;
            }
            66 => {
                {
                    let destination = values[1];
                    let stored = (0x2du64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 65;
            }
            67 => {
                pc = 2;
            }
            68 => {
                pc = if (u64::from((values[1]) == (values[2]))) != 0 {
                    67
                } else {
                    66
                };
            }
            69 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    68
                } else {
                    63
                };
            }
            70 => {
                values[8] = u64::from((values[3]) as u32);
                pc = 69;
            }
            71 => {
                values[11] = relocate("CONTEXT_in_R9");
                pc = 70;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400324c0` (`mspaint.c:46794`).
pub fn fun_1400324c0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 3 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 3,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        u64::from((arguments[1]) as u32),
        arguments[2],
        0u64,
        0u64,
    ];
    let mut pc: i32 = 5;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[3]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[3] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [values[0], (values[4]) & (0xffffffffu64), values[2]];
                    invoke("FUN_140032490", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_18"),
                        u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_in_stack_00000038"))?))
                                as u32,
                        ),
                    ];
                    invoke("FUN_1400131e0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                values[4] = values[1];
                pc = 4;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14003e7b0` (`mspaint.c:58223`).
pub fn fun_14003e7b0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 77;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_60"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [0x1u64];
                    invoke("FUN_140033a00", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400944c0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[0], relocate("STACK_local_60")];
                    invoke("FUN_140040540", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[0], values[9]];
                    invoke("FUN_140041070", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x38u64),
                        values[9],
                        relocate("STACK_local_60"),
                    ];
                    invoke("FUN_14003f420", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_140004100", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(((values[5]) as i64) != ((0x0u64) as i64))) != 0 {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_40"))?,
                        relocate("STACK_local_60"),
                    ];
                    invoke("FUN_1400357a0", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                values[5] = memory.read_u64(relocate("STACK_local_40"))?;
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                pc = if (values[10]) != 0 { 12 } else { 11 };
            }
            14 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (memory.read_u64(relocate("STACK_local_40"))?)
                        .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[10] = u64::from(
                    (u64::from((memory.read_u64(relocate("STACK_local_40"))?) == (0x0u64))) != 0,
                );
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            17 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    16
                } else {
                    15
                };
            }
            18 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[3])?)?,
                        values[3],
                        relocate("DAT_14010de08"),
                        relocate("STACK_local_40"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_58")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x84bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    21
                } else {
                    11
                };
            }
            23 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                values[3] = memory.read_u64(values[1])?;
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_140004100", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                pc = if (u64::from(((values[5]) as i64) != ((0x0u64) as i64))) != 0 {
                    25
                } else {
                    24
                };
            }
            27 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400412e0", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = (values[9]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                values[5] = memory.read_u64(relocate("STACK_local_40"))?;
                pc = 28;
            }
            30 => {
                pc = if (u64::from(!((values[10]) != 0))) != 0 {
                    45
                } else {
                    29
                };
            }
            31 => {
                values[5] = values[8];
                pc = 30;
            }
            32 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 31;
            }
            33 => {
                values[8] = values[5];
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[0]).wrapping_add(0x8u64);
                    let stored = (values[5]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                pc = if (values[10]) != 0 { 34 } else { 32 };
            }
            36 => {
                values[10] =
                    u64::from((u64::from(((values[5]) as i64) == ((values[8]) as i64))) != 0);
                pc = 35;
            }
            37 => {
                values[8] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
                pc = 36;
            }
            38 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 37;
            }
            39 => {
                pc = 29;
            }
            40 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 39;
            }
            41 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                values[2] = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 41;
            }
            43 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 42;
            }
            44 => {
                pc = if (u64::from(((values[5]) as i64) < ((0x0u64) as i64))) != 0 {
                    43
                } else {
                    38
                };
            }
            45 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                values[5] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
                pc = 45;
            }
            47 => {
                pc = if (u64::from(((values[0]) as i64) != ((0x0u64) as i64))) != 0 {
                    46
                } else {
                    29
                };
            }
            48 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                values[9] = ((((u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as i32 as i64 as u64)
                    .wrapping_add(0x1u64)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [values[7], 0x0u64, values[4]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            53 => {
                pc = if (u64::from(
                    (((((values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    52
                } else {
                    49
                };
            }
            54 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    51
                } else {
                    53
                };
            }
            55 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 54;
            }
            56 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                values[9] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 56;
            }
            58 => {
                values[2] = (values[4]).wrapping_add(0x18u64);
                pc = 57;
            }
            59 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 58;
            }
            60 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    59
                } else {
                    49
                };
            }
            61 => {
                values[4] = (((u64::from(
                    (u64::from(memory.read_u32(relocate("STACK_uStack_64"))?)) as u32,
                )) & (0xffffffffu64))
                    .wrapping_shl((0x20u64) as u32))
                    | (((u64::from(memory.read_u32(relocate("STACK_local_68"))?)) as i32 as i64
                        as u64)
                        & (0xffffffffu64));
                pc = 60;
            }
            62 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140094340", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = ((u64::from(
                        ((memory.read_word(relocate("STACK_local_40"), 5)?) & 0xffffffffffu64)
                            as u32,
                    )) & 0xffffffffffu64)
                        & 0xffffffffffu64;
                    memory.write_word(destination, 5, stored)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ca8")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 62;
            }
            65 => {
                {
                    let destination = relocate("DAT_1401d0c74");
                    let stored = (0x1u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 64;
            }
            66 => {
                pc = if (u64::from(
                    (u64::from((0x1u64) as u32))
                        < (u64::from(
                            (u64::from(
                                (((u64::from(memory.read_u32(relocate("DAT_1401d0c74"))?)) as i32
                                    as i64 as u64)
                                    .wrapping_sub(0x3u64)) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    65
                } else {
                    62
                };
            }
            67 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("DAT_1401d0c74"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    66
                };
            }
            68 => {
                {
                    let call_arguments = [relocate("STACK_local_68"), relocate("STACK_local_58")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = relocate("L\"Duplicate layer\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = ((u64::from(
                        ((memory.read_word(relocate("STACK_local_40"), 5)?) & 0xffffffffffu64)
                            as u32,
                    )) & 0xffffffffffu64)
                        & 0xffffffffffu64;
                    memory.write_word(destination, 5, stored)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 71;
            }
            73 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_40"))?) != (0x0u64))) != 0
                {
                    72
                } else {
                    71
                };
            }
            74 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x28u64))?,
                        relocate("STACK_local_40"),
                        memory.read_u64(
                            (memory.read_u64((values[0]).wrapping_add(0x28u64))?)
                                .wrapping_add(0x40u64),
                        )?,
                        0x0u64,
                    ];
                    invoke("FUN_140051600", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x38u64),
                        values[1],
                        relocate("STACK_local_38"),
                    ];
                    invoke("FUN_14003d0f0", &call_arguments, memory)
                };
                pc = 74;
            }
            76 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_88"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14004d580` (`mspaint.c:71033`).
pub fn fun_14004d580(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let values = [arguments[0], arguments[1]];
    let mut pc: i32 = 5;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x40u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(((values[1]) & (0x1u64)) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let destination = values[0];
                    let stored = relocate("_vftable__exref");
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x3u64).wrapping_mul(8))];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x4u64).wrapping_mul(8))];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 4;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400511d0` (`mspaint.c:73826`).
pub fn fun_1400511d0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], arguments[1], 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 22;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    4
                } else {
                    0
                };
            }
            2 => {
                values[4] = values[5];
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [values[4], 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                values[5] = memory.read_u64(values[4])?;
                pc = 3;
            }
            5 => {
                pc = if ({
                    let _ = {
                        let _ = {
                            values[6] = values[3];
                            values[6]
                        };
                        {
                            values[4] = memory.read_u64(relocate("STACK_local_28"))?;
                            values[4]
                        }
                    };
                    u64::from((values[6]) != (values[2]))
                }) != 0
                {
                    15
                } else {
                    1
                };
            }
            6 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_20"))?;
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = (values[4]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[5] = memory.read_u64((values[6]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 9;
            }
            11 => {
                {
                    let destination = values[6];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64(values[6])?;
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[0]).wrapping_add(0x90u64);
                    let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                pc = if (u64::from(
                    (memory.read_u64((values[6]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                        == (values[1]),
                )) != 0
                {
                    13
                } else {
                    5
                };
            }
            15 => {
                values[3] = memory.read_u64(values[6])?;
                pc = 14;
            }
            16 => {
                values[3] = memory.read_u64(values[2])?;
                pc = 5;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = relocate("STACK_local_28");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                values[2] = memory.read_u64(memory.read_u64(relocate("STACK_local_30"))?)?;
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (values[0]).wrapping_add(0x88u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x10u64))?,
                        values[1],
                        (values[0]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_58"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14005a2a0` (`mspaint.c:82251`).
pub fn fun_14005a2a0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 54;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[11]));
            }
            1 => {
                values[11] = values[12];
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [values[10], values[11]];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            4 => {
                pc = if (u64::from(
                    (0x1fu64)
                        < ((values[5]).wrapping_add(
                            ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                .wrapping_sub(values[10]),
                        )),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[10] = memory.read_u64((values[5]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                ))?;
                pc = 4;
            }
            6 => {
                values[11] = (values[11]).wrapping_add(0x27u64);
                pc = 5;
            }
            7 => {
                pc = if (u64::from((0xfffu64) < (values[11]))) != 0 {
                    6
                } else {
                    2
                };
            }
            8 => {
                values[10] = values[5];
                pc = 7;
            }
            9 => {
                values[11] = ((values[10]).wrapping_sub(values[5])) & (0xfffffffffffffff0u64);
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14005a690", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    10
                } else {
                    1
                };
            }
            12 => {
                pc = if (u64::from((values[4]) != (values[6]))) != 0 {
                    19
                } else {
                    11
                };
            }
            13 => {
                let _ = {
                    values[4] = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
                    values[4]
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x8u64))?,
                        values[3],
                        0x1u64,
                        u64::from(!((values[13]) != 0)),
                        u64::from(!((values[13]) != 0)),
                        values[9],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                values[9] = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 14;
            }
            16 => {
                values[9] = (memory.read_u64(values[0])?).wrapping_add(0x70u64);
                pc = 14;
            }
            17 => {
                pc = if (values[13]) != 0 { 15 } else { 16 };
            }
            18 => {
                values[13] = u64::from((u64::from(((values[1]) as i64) == ((0x0u64) as i64))) != 0);
                pc = 17;
            }
            19 => {
                values[3] = memory.read_u64(values[4])?;
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            21 => {
                values[4] = values[5];
                pc = 20;
            }
            22 => {
                values[1] = memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x70u64))?;
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [(values[1]).wrapping_add(0x18u64)];
                    invoke("_Mtx_unlock", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                values[5] = memory.read_u64(relocate("STACK_local_48"))?;
                pc = 23;
            }
            25 => {
                values[6] = memory.read_u64(relocate("STACK_plStack_40"))?;
                pc = 24;
            }
            26 => {
                values[10] = 0x0u64;
                pc = 25;
            }
            27 => {
                values[12] = u64::from((0x1u64) as u8);
                pc = 26;
            }
            28 => {
                values[10] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 25;
            }
            29 => {
                values[12] = u64::from((0x1u64) as u8);
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored =
                        memory.read_u64((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_plStack_40");
                    let stored =
                        memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                pc = if (u64::from((relocate("STACK_local_48")) == (values[2]))) != 0 {
                    27
                } else {
                    35
                };
            }
            37 => {
                values[2] = memory.read_u64(values[0])?;
                pc = 36;
            }
            38 => {
                {
                    let destination = (memory.read_u64(values[0])?).wrapping_add(0x81u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8((memory.read_u64(values[0])?).wrapping_add(0x81u64))?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    25
                };
            }
            40 => {
                {
                    let call_arguments = [0x6u64];
                    invoke("std::_Throw_Cpp_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Throw_Cpp_error"));
            }
            41 => {
                {
                    let destination = (values[1]).wrapping_add(0x64u64);
                    let stored = u64::from((0x7ffffffeu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32((values[1]).wrapping_add(0x64u64))?)) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x7fffffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    41
                } else {
                    39
                };
            }
            43 => {
                {
                    let call_arguments = [0x5u64];
                    invoke("std::_Throw_Cpp_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Throw_Cpp_error"));
            }
            44 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    43
                } else {
                    42
                };
            }
            45 => {
                values[7] = ({
                    let call_arguments = [(values[1]).wrapping_add(0x18u64)];
                    invoke("_Mtx_lock", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 44;
            }
            46 => {
                values[12] = u64::from((0x0u64) as u8);
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                values[10] = 0x0u64;
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_plStack_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                values[11] = (values[8]) & (0xffffffffffffff00u64);
                pc = 0;
            }
            52 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[1]).wrapping_add(0x81u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    51
                };
            }
            53 => {
                values[1] = memory.read_u64(values[0])?;
                pc = 52;
            }
            54 => {
                values[8] = relocate("CONTEXT_in_RAX");
                pc = 53;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140075530` (`mspaint.c:103342`).
pub fn fun_140075530(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 53;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x288u64))?];
                    invoke("RecoveryHandler::CleanUp", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add(0x288u64))?) != (0x0u64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_30"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_50")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            6 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                values[5] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(values[7])?)?).wrapping_add(0x38u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination =
                        (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x2c60u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                values[7] = {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_14005c3d0", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    (values[8]) != (memory.read_u64((values[0]).wrapping_add(0x200u64))?),
                )) != 0
                {
                    33
                } else {
                    11
                };
            }
            13 => {
                values[9] = values[8];
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [values[6], 0x30u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                values[6] = {
                    let call_arguments = [(values[0]).wrapping_add(0x200u64), values[9]];
                    invoke("FUN_1400344f0", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    15
                };
            }
            17 => {
                values[2] = memory.read_u64(values[2])?;
                pc = 16;
            }
            18 => {
                values[8] = values[2];
                pc = 17;
            }
            19 => {
                values[1] = ((u64::from(
                    memory.read_u8((memory.read_u64(values[2])?).wrapping_add(0x19u64))?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 18;
            }
            20 => {
                values[2] = memory.read_u64(values[8])?;
                pc = 16;
            }
            21 => {
                values[1] = ((u64::from(
                    memory.read_u8((memory.read_u64(values[8])?).wrapping_add(0x19u64))?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 20;
            }
            22 => {
                pc = if ({
                    let _ = {
                        values[8] = values[4];
                        values[8]
                    };
                    u64::from(
                        ((u64::from(
                            (((values[1]) as i32 as i64 as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                (values[2])
                                    == (memory.read_u64(
                                        (values[8]).wrapping_add((0x2u64).wrapping_mul(8)),
                                    )?),
                            )) != 0),
                    )
                }) != 0
                {
                    25
                } else {
                    15
                };
            }
            23 => {
                values[2] = values[8];
                pc = 22;
            }
            24 => {
                values[4] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 23;
            }
            25 => {
                values[1] = ((u64::from(
                    memory.read_u8(
                        (memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x19u64),
                    )?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 24;
            }
            26 => {
                values[2] = values[9];
                pc = 22;
            }
            27 => {
                values[4] = memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 26;
            }
            28 => {
                values[1] = ((u64::from(
                    memory.read_u8(
                        (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x19u64),
                    )?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 27;
            }
            29 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[8]).wrapping_add(0x19u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    28
                };
            }
            30 => {
                values[8] = memory.read_u64((values[9]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[7])?)?,
                        values[7],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    31
                } else {
                    30
                };
            }
            33 => {
                values[7] = memory.read_u64((values[9]).wrapping_add((0x5u64).wrapping_mul(8)))?;
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    (values[9]) != (memory.read_u64((values[0]).wrapping_add(0x200u64))?),
                )) != 0
                {
                    33
                } else {
                    11
                };
            }
            35 => {
                values[9] =
                    memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x200u64))?)?;
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [(values[0]).wrapping_add(0x1e0u64)];
                    invoke("FUN_14008ec90", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x1a8u64),
                        memory.read_u64((values[0]).wrapping_add(0x668u64))?,
                    ];
                    invoke("FUN_1400034e0", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                {
                    let destination = values[9];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [values[9]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                pc = if (u64::from(((memory.read_u64(values[9])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                pc = if (u64::from((values[9]) != (relocate("STACK_local_38")))) != 0 {
                    40
                } else {
                    37
                };
            }
            42 => {
                values[9] = (values[0]).wrapping_add(0x650u64);
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((memory.read_u64(relocate("STACK_local_30"))?) as i64)
                            != ((0x0u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                let call_arguments = [
                                    memory.read_u64(relocate("STACK_local_30"))?,
                                    memory.read_u64((values[0]).wrapping_add(0x678u64))?,
                                ];
                                invoke("FUN_140018690", &call_arguments, memory)
                            };
                            u64::from(((values[3]) as i64) != ((0x0u64) as i64))
                        }) != 0),
                )) != 0
                {
                    44
                } else {
                    43
                };
            }
            46 => {
                values[3] = memory.read_u64(relocate("STACK_local_30"))?;
                pc = 45;
            }
            47 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_58"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_58"))?,
                        relocate("DAT_140105598"),
                        relocate("STACK_local_30"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) != (0x0u64))) != 0
                {
                    48
                } else {
                    42
                };
            }
            50 => {
                {
                    let destination = (values[0]).wrapping_add(0x670u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = memory.read_u64((values[0]).wrapping_add(0x670u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140075740", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14007c4b0` (`mspaint.c:109544`).
pub fn fun_14007c4b0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 28;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [
                        values[0],
                        0x5c6u64,
                        relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\pbrush.cpp\""),
                    ];
                    invoke("FUN_140012f00", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140012f00"));
            }
            1 => {
                return Ok(Some(values[4]));
            }
            2 => {
                {
                    let call_arguments = [];
                    invoke("thunk_FUN_140011f80", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    0
                };
            }
            4 => {
                {
                    let destination = relocate("STACK_local_1070");
                    let stored = memory.read_u64(relocate("STACK_in_stack_00000030"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_1068");
                    let stored = relocate("STACK_local_1048");
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_1060");
                    let stored = relocate("STACK_local_1018");
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_1040");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_1048");
                    let stored = (((values[1]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                        | ((u64::from((memory.read_u64(relocate("STACK_local_1048"))?) as u32))
                            & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                values[1] = u64::from(
                    ({
                        let call_arguments = [0x8007023eu64];
                        invoke("FUN_140012f40", &call_arguments, memory)
                    }) as u32,
                );
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_1048");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_1048")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x8007023eu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                values[4] = 0x8007023eu64;
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    6
                };
            }
            13 => {
                values[5] = (u64::from(
                    ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) as i8 as i64 as u64;
                pc = 12;
            }
            14 => {
                values[4] = (memory.read_u64(relocate("STACK_local_1048"))?) & (0xffffffffu64);
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_1040");
                    let stored = u64::from(
                        (u64::from(
                            (memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                                as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_1048");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                values[3] = {
                    let call_arguments = [
                        memory.read_u64(relocate("DAT_1401cff20"))?,
                        relocate("STACK_local_1038"),
                        (relocate("STACK_local_1018")).wrapping_add((values[2]).wrapping_mul(2)),
                        (0x800u64).wrapping_sub(values[2]),
                        relocate("STACK_local_1028"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("DAT_1401cff20"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    17
                } else {
                    13
                };
            }
            19 => {
                {
                    let destination = relocate("STACK_local_1040");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_1048");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                values[4] = 0x0u64;
                pc = 20;
            }
            22 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16(
                        (relocate("STACK_local_1018")).wrapping_add((values[2]).wrapping_mul(2)),
                    )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    23
                } else {
                    21
                };
            }
            23 => {
                values[2] = (values[2]).wrapping_add(0x1u64);
                pc = 22;
            }
            24 => {
                values[2] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 23;
            }
            25 => {
                {
                    let destination =
                        (relocate("STACK_local_1028")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let call_arguments = [relocate("STACK_local_1018"), 0x0u64, 0x1000u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_1098"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = 0x14007c4c5u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14007d400` (`mspaint.c:110353`).
pub fn fun_14007d400(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], arguments[1], 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 17;
    loop {
        match pc {
            0 => {
                return Ok(Some((values[4]) as i32 as i64 as u64));
            }
            1 => {
                values[4] = (values[2]) as i32 as i64 as u64;
                pc = 0;
            }
            2 => {
                {
                    let destination = (values[1]).wrapping_add((0x40u64).wrapping_mul(1));
                    let stored = memory.read_u64(
                        (relocate("STACK_local_68")).wrapping_add((0x40u64).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    0
                };
            }
            4 => {
                values[4] = (0x0u64) as i32 as i64 as u64;
                pc = 3;
            }
            5 => {
                values[3] = ({
                    let call_arguments = [relocate("STACK_local_68")];
                    invoke("AfxRegisterClass", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 4;
            }
            6 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x30u64).wrapping_mul(1));
                    let stored = {
                        let call_arguments = [0x5u64];
                        invoke("GetStockObject", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x40u64).wrapping_mul(1));
                    let stored = relocate("u_MSPaintView_1401790f8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_68"))
                                            .wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?,
                                )) as u32,
                            )) & (0xfffffffcu64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[5]).wrapping_add((0x10u64).wrapping_mul(1)))?,
                        memory.read_u64((values[1]).wrapping_add((0x40u64).wrapping_mul(1)))?,
                        relocate("STACK_local_68"),
                    ];
                    invoke("GetClassInfoW", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                values[5] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    0
                };
            }
            12 => {
                values[4] = (values[2]) as i32 as i64 as u64;
                pc = 11;
            }
            13 => {
                values[2] = ({
                    let call_arguments = [values[0], values[1]];
                    invoke("CView::PreCreateWindow", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[1]).wrapping_add((0x30u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                ((u64::from(
                                    (u64::from(memory.read_u32(
                                        (values[1]).wrapping_add((0x30u64).wrapping_mul(1)),
                                    )?)) as u32,
                                )) & (0xef7fffffu64)) as u32,
                            )) | (0xa000000u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[1]).wrapping_add((0x48u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(
                                    (values[1]).wrapping_add((0x48u64).wrapping_mul(1)),
                                )?)) as u32,
                            )) | (0x100000u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[1]).wrapping_add((0x48u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(
                                    (values[1]).wrapping_add((0x48u64).wrapping_mul(1)),
                                )?)) as u32,
                            )) & (0xfffffdffu64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_88"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14008ec90` (`mspaint.c:123431`).
pub fn fun_14008ec90(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 63;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                pc = if (u64::from((values[7]) != (memory.read_u64(values[0])?))) != 0 {
                    59
                } else {
                    0
                };
            }
            2 => {
                values[8] = values[7];
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_38"))?, 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    7
                } else {
                    3
                };
            }
            5 => {
                values[4] = values[3];
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[4], 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                values[3] = memory.read_u64(values[4])?;
                pc = 6;
            }
            8 => {
                values[4] = memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?;
                pc = 4;
            }
            9 => {
                {
                    let destination = memory.read_u64(
                        (memory.read_u64(relocate("STACK_local_38"))?)
                            .wrapping_add((0x1u64).wrapping_mul(8)),
                    )?;
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [values[6], 0x38u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [(values[6]).wrapping_add(0x28u64)];
                    invoke("FUN_140006890", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                values[6] = {
                    let call_arguments = [values[0], values[8]];
                    invoke("FUN_1400344f0", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    16
                } else {
                    12
                };
            }
            14 => {
                values[2] = memory.read_u64(values[2])?;
                pc = 13;
            }
            15 => {
                values[7] = values[2];
                pc = 14;
            }
            16 => {
                values[1] = ((u64::from(
                    memory.read_u8((memory.read_u64(values[2])?).wrapping_add(0x19u64))?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 15;
            }
            17 => {
                values[2] = memory.read_u64(values[7])?;
                pc = 13;
            }
            18 => {
                values[1] = ((u64::from(
                    memory.read_u8((memory.read_u64(values[7])?).wrapping_add(0x19u64))?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 17;
            }
            19 => {
                pc = if ({
                    let _ = {
                        values[7] = values[5];
                        values[7]
                    };
                    u64::from(
                        ((u64::from(
                            (((values[1]) as i32 as i64 as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                (values[2])
                                    == (memory.read_u64(
                                        (values[7]).wrapping_add((0x2u64).wrapping_mul(8)),
                                    )?),
                            )) != 0),
                    )
                }) != 0
                {
                    22
                } else {
                    12
                };
            }
            20 => {
                values[2] = values[7];
                pc = 19;
            }
            21 => {
                values[5] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 20;
            }
            22 => {
                values[1] = ((u64::from(
                    memory.read_u8(
                        (memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x19u64),
                    )?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 21;
            }
            23 => {
                values[2] = values[8];
                pc = 19;
            }
            24 => {
                values[5] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 23;
            }
            25 => {
                values[1] = ((u64::from(
                    memory.read_u8(
                        (memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x19u64),
                    )?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[7]).wrapping_add(0x19u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    25
                };
            }
            27 => {
                values[7] = memory.read_u64((values[8]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 26;
            }
            28 => {
                pc = if (u64::from((values[2]) != (memory.read_u64(relocate("STACK_local_38"))?)))
                    != 0
                {
                    40
                } else {
                    27
                };
            }
            29 => {
                values[7] = values[2];
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [values[7], 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_lStack_30");
                    let stored = (memory.read_u64(relocate("STACK_lStack_30"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = (memory.read_u64(values[7])?).wrapping_add(0x8u64);
                    let stored =
                        memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination =
                        memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                values[2] = memory.read_u64(values[7])?;
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [values[6], 0x58u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [memory.read_u64((values[6]).wrapping_add(0x10u64))?];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    (memory.read_u64((values[6]).wrapping_add(0x10u64))?) != (0x0u64),
                )) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                {
                    let call_arguments = [memory.read_u64((values[6]).wrapping_add(0x18u64))?];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    (memory.read_u64((values[6]).wrapping_add(0x18u64))?) != (0x0u64),
                )) != 0
                {
                    38
                } else {
                    37
                };
            }
            40 => {
                values[6] = memory.read_u64((values[7]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 39;
            }
            41 => {
                pc = if (u64::from((values[7]) != (memory.read_u64(relocate("STACK_local_38"))?)))
                    != 0
                {
                    40
                } else {
                    27
                };
            }
            42 => {
                values[7] = memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?;
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_lStack_30");
                    let stored = memory.read_u64(relocate("STACK_local_60"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_38"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_38"))?;
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_58"))?;
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            47 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_lStack_50"))?).wrapping_add(0x8u64);
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_38"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_58"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_38"))?;
                    let stored = memory.read_u64(relocate("STACK_lStack_50"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_60"))?) as i64) == ((0x0u64) as i64),
                )) != 0
                {
                    45
                } else {
                    49
                };
            }
            51 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = {
                        let call_arguments = [0x18u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = memory.read_u64(relocate("STACK_local_68"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_68"),
                        memory.read_u64(
                            memory.read_u64((values[8]).wrapping_add((0x5u64).wrapping_mul(8)))?,
                        )?,
                    ];
                    invoke("FUN_14008eea0", &call_arguments, memory)
                };
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_lStack_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("STACK_local_38");
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_lStack_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                pc = if (u64::from((values[8]) != (memory.read_u64(values[0])?))) != 0 {
                    60
                } else {
                    0
                };
            }
            62 => {
                values[8] = memory.read_u64(memory.read_u64(values[0])?)?;
                pc = 61;
            }
            63 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_88"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400aa8a0` (`mspaint.c:146775`).
pub fn fun_1400aa8a0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 54;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[11]));
            }
            1 => {
                values[11] = values[12];
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [values[10], values[11]];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            4 => {
                pc = if (u64::from(
                    (0x1fu64)
                        < ((values[5]).wrapping_add(
                            ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                .wrapping_sub(values[10]),
                        )),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[10] = memory.read_u64((values[5]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                ))?;
                pc = 4;
            }
            6 => {
                values[11] = (values[11]).wrapping_add(0x27u64);
                pc = 5;
            }
            7 => {
                pc = if (u64::from((0xfffu64) < (values[11]))) != 0 {
                    6
                } else {
                    2
                };
            }
            8 => {
                values[10] = values[5];
                pc = 7;
            }
            9 => {
                values[11] = ((values[10]).wrapping_sub(values[5])) & (0xfffffffffffffff0u64);
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14005a690", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    10
                } else {
                    1
                };
            }
            12 => {
                pc = if (u64::from((values[4]) != (values[6]))) != 0 {
                    19
                } else {
                    11
                };
            }
            13 => {
                let _ = {
                    values[4] = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
                    values[4]
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x8u64))?,
                        values[3],
                        0x1u64,
                        u64::from(!((values[13]) != 0)),
                        u64::from(!((values[13]) != 0)),
                        values[9],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                values[9] = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 14;
            }
            16 => {
                values[9] = (memory.read_u64(values[0])?).wrapping_add(0x78u64);
                pc = 14;
            }
            17 => {
                pc = if (values[13]) != 0 { 15 } else { 16 };
            }
            18 => {
                values[13] = u64::from((u64::from(((values[1]) as i64) == ((0x0u64) as i64))) != 0);
                pc = 17;
            }
            19 => {
                values[3] = memory.read_u64(values[4])?;
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            21 => {
                values[4] = values[5];
                pc = 20;
            }
            22 => {
                values[1] = memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x78u64))?;
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [(values[1]).wrapping_add(0x18u64)];
                    invoke("_Mtx_unlock", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                values[5] = memory.read_u64(relocate("STACK_local_48"))?;
                pc = 23;
            }
            25 => {
                values[6] = memory.read_u64(relocate("STACK_plStack_40"))?;
                pc = 24;
            }
            26 => {
                values[10] = 0x0u64;
                pc = 25;
            }
            27 => {
                values[12] = u64::from((0x1u64) as u8);
                pc = 26;
            }
            28 => {
                values[10] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 25;
            }
            29 => {
                values[12] = u64::from((0x1u64) as u8);
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored =
                        memory.read_u64((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_plStack_40");
                    let stored =
                        memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                pc = if (u64::from((relocate("STACK_local_48")) == (values[2]))) != 0 {
                    27
                } else {
                    35
                };
            }
            37 => {
                values[2] = memory.read_u64(values[0])?;
                pc = 36;
            }
            38 => {
                {
                    let destination = (memory.read_u64(values[0])?).wrapping_add(0x89u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8((memory.read_u64(values[0])?).wrapping_add(0x89u64))?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    25
                };
            }
            40 => {
                {
                    let call_arguments = [0x6u64];
                    invoke("std::_Throw_Cpp_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Throw_Cpp_error"));
            }
            41 => {
                {
                    let destination = (values[1]).wrapping_add(0x64u64);
                    let stored = u64::from((0x7ffffffeu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32((values[1]).wrapping_add(0x64u64))?)) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x7fffffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    41
                } else {
                    39
                };
            }
            43 => {
                {
                    let call_arguments = [0x5u64];
                    invoke("std::_Throw_Cpp_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Throw_Cpp_error"));
            }
            44 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    43
                } else {
                    42
                };
            }
            45 => {
                values[7] = ({
                    let call_arguments = [(values[1]).wrapping_add(0x18u64)];
                    invoke("_Mtx_lock", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 44;
            }
            46 => {
                values[12] = u64::from((0x0u64) as u8);
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                values[10] = 0x0u64;
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_plStack_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                values[11] = (values[8]) & (0xffffffffffffff00u64);
                pc = 0;
            }
            52 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[1]).wrapping_add(0x89u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    51
                };
            }
            53 => {
                values[1] = memory.read_u64(values[0])?;
                pc = 52;
            }
            54 => {
                values[8] = relocate("CONTEXT_in_RAX");
                pc = 53;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400ae624` (`mspaint.c:150304`).
pub fn fun_1400ae624(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], arguments[1], 0u64, 0u64];
    let mut pc: i32 = 9;
    loop {
        match pc {
            0 => {
                return Ok(Some(
                    ((relocate("IMAGE_DOS_HEADER_140000000"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add(
                        (u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[2]).wrapping_add((0xcu64).wrapping_mul(1)),
                                )?)) as u32,
                            )) as u32,
                        ))
                        .wrapping_mul(1),
                    ),
                ));
            }
            1 => {
                {
                    let destination = values[1];
                    let stored = ((u64::from(
                        memory.read_u32((values[2]).wrapping_add((0x24u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[0];
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[2]).wrapping_add((0x8u64).wrapping_mul(1)))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((0x1dd000u64) as u32))
                            < (u64::from(
                                (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(
                                            (values[2]).wrapping_add((0xcu64).wrapping_mul(1)),
                                        )?)) as u32,
                                    )) as u32,
                                )) as u32,
                            )),
                    )) != 0)
                        || ((u64::from(
                            (u64::from(
                                (u64::from(
                                    ((u64::from(
                                        (u64::from(memory.read_u32(
                                            (values[2]).wrapping_add((0xcu64).wrapping_mul(1)),
                                        )?)) as u32,
                                    ))
                                    .wrapping_add(u64::from(
                                        (u64::from(
                                            memory.read_u32(
                                                ((values[2])
                                                    .wrapping_add((0x8u64).wrapping_mul(1)))
                                                .wrapping_add((0x0u64).wrapping_mul(1)),
                                            )?,
                                        )) as u32,
                                    ))) as u32,
                                )) as u32,
                            )) < (u64::from((0x1dd001u64) as u32)),
                        )) != 0),
                )) != 0
                {
                    7
                } else {
                    2
                };
            }
            4 => {
                return Ok(Some(0x0u64));
            }
            5 => {
                pc = if (u64::from((u64::from((0x6u64) as u32)) < (u64::from((values[3]) as u32))))
                    != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[2] = (values[2]).wrapping_add((0x1u64).wrapping_mul(40));
                pc = 5;
            }
            7 => {
                values[3] =
                    u64::from((u64::from(((values[3]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 6;
            }
            8 => {
                values[2] = relocate("IMAGE_SECTION_HEADER_140000230");
                pc = 3;
            }
            9 => {
                values[3] = u64::from((0x0u64) as u32);
                pc = 8;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400ae870` (`mspaint.c:150428`).
pub fn fun_1400ae870(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 52;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[8]));
            }
            1 => {
                values[8] = 0x0u64;
                pc = 0;
            }
            2 => {
                {
                    let call_arguments =
                        [0xc06d0057u64, 0x0u64, 0x1u64, relocate("STACK_local_res8")];
                    invoke("RaiseException", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400ae7b8", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("STACK_local_res8");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400ae7b8", &call_arguments, memory)
                };
                pc = 0;
            }
            6 => {
                {
                    let call_arguments = [values[1], 0x8u64, relocate("STACK_local_88")];
                    invoke("FUN_1400ae344", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                values[8] = memory.read_u64(relocate("STACK_local_40"))?;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400ae430", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let call_arguments =
                        [0xc06d007fu64, 0x0u64, 0x1u64, relocate("STACK_local_res8")];
                    invoke("RaiseException", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400ae7b8", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_res8");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = u64::from(
                        ({
                            let call_arguments = [];
                            invoke("GetLastError", &call_arguments, memory)
                        }) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from(
                                ((u64::from(
                                    (u64::from(
                                        (u64::from(
                                            (u64::from(memory.read_u32(
                                                (values[0]).wrapping_add((0x5u64).wrapping_mul(4)),
                                            )?)) as u32,
                                        )) as u32,
                                    )) == (u64::from((0x0u64) as u32)),
                                )) != 0)
                                    || ((u64::from(
                                        (u64::from(
                                            (u64::from(
                                                (u64::from(
                                                    memory
                                                        .read_u32((values[0]).wrapping_add(
                                                            (0x7u64).wrapping_mul(4),
                                                        ))?,
                                                ))
                                                    as u32,
                                            )) as u32,
                                        )) == (u64::from((0x0u64) as u32)),
                                    )) != 0),
                            )) != 0)
                                || (({
                                    let _ = {
                                        values[7] = (u64::from(
                                            memory.read_u32(
                                                ((values[6])
                                                    .wrapping_add((0xfu64).wrapping_mul(4)))
                                                .wrapping_add((0x0u64).wrapping_mul(1)),
                                            )?,
                                        ))
                                            as i32
                                            as i64
                                            as u64;
                                        values[7]
                                    };
                                    u64::from(
                                        ((((u64::from(
                                            memory.read_u32(
                                                ((values[6])
                                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                                .wrapping_add(values[7]),
                                            )?,
                                        )) as i32 as i64
                                            as u64) as i32
                                            as i64 as u64)
                                            as i64)
                                            != (((0x4550u64) as i32 as i64 as u64) as i64),
                                    )
                                }) != 0),
                        )) != 0)
                            || ((u64::from(
                                ((u64::from(
                                    ((u64::from(
                                        ((((u64::from(
                                            memory.read_u32(
                                                (((values[6])
                                                    .wrapping_add((0x2u64).wrapping_mul(4)))
                                                .wrapping_add((0x0u64).wrapping_mul(1)))
                                                .wrapping_add(values[7]),
                                            )?,
                                        )) as i32 as i64
                                            as u64) as i32
                                            as i64 as u64)
                                            as i64)
                                            != ((((memory.read_u64(relocate("STACK_local_res8"))?)
                                                as i32
                                                as i64
                                                as u64)
                                                as i32
                                                as i64
                                                as u64)
                                                as i64),
                                    )) != 0)
                                        || ((u64::from(
                                            (values[6])
                                                != (memory.read_u64(
                                                    (((values[6])
                                                        .wrapping_add((0xcu64).wrapping_mul(4)))
                                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                                    .wrapping_add(values[7]),
                                                )?),
                                        )) != 0),
                                )) != 0)
                                    || (({
                                        let _ = {
                                            values[8] = memory.read_u64(
                                                (((relocate("IMAGE_DOS_HEADER_140000000"))
                                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                                .wrapping_add(
                                                    ((values[9]).wrapping_mul(0x8u64))
                                                        .wrapping_mul(1),
                                                ))
                                                .wrapping_add((values[4]).wrapping_mul(1)),
                                            )?;
                                            values[8]
                                        };
                                        u64::from(
                                            (memory.read_u64(
                                                (((relocate("IMAGE_DOS_HEADER_140000000"))
                                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                                .wrapping_add(
                                                    ((values[9]).wrapping_mul(0x8u64))
                                                        .wrapping_mul(1),
                                                ))
                                                .wrapping_add((values[4]).wrapping_mul(1)),
                                            )?) == (0x0u64),
                                        )
                                    }) != 0),
                            )) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                values[8] = {
                                    let call_arguments =
                                        [values[6], memory.read_u64(relocate("STACK_pcStack_50"))?];
                                    invoke("GetProcAddress", &call_arguments, memory)
                                };
                                values[8]
                            };
                            u64::from((values[8]) == (0x0u64))
                        }) != 0),
                )) != 0
                {
                    14
                } else {
                    8
                };
            }
            16 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [values[6]];
                    invoke("FreeLibrary", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                pc = if (u64::from((values[2]) == (values[6]))) != 0 {
                    17
                } else {
                    16
                };
            }
            19 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 18;
            }
            20 => {
                {
                    let destination = values[10];
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                values[2] = memory.read_u64(values[10])?;
                pc = 20;
            }
            22 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 21;
            }
            23 => {
                return Ok(Some(memory.read_u64(relocate("STACK_local_40"))?));
            }
            24 => {
                {
                    let call_arguments =
                        [0xc06d007eu64, 0x0u64, 0x1u64, relocate("STACK_local_res8")];
                    invoke("RaiseException", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400ae7b8", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_res8");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = u64::from(
                        ({
                            let call_arguments = [];
                            invoke("GetLastError", &call_arguments, memory)
                        }) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                pc = if (u64::from((values[6]) == (0x0u64))) != 0 {
                    27
                } else {
                    22
                };
            }
            29 => {
                values[6] = {
                    let call_arguments =
                        [memory.read_u64(relocate("STACK_local_60"))?, 0x0u64, 0x0u64];
                    invoke("LoadLibraryExA", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                pc = if (u64::from((values[6]) == (0x0u64))) != 0 {
                    29
                } else {
                    16
                };
            }
            31 => {
                {
                    let destination = relocate("STACK_pcStack_50");
                    let stored = u64::from(
                        (u64::from(
                            memory.read_u16(
                                (((relocate("IMAGE_DOS_HEADER_140000000"))
                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                .wrapping_add(((values[9]).wrapping_mul(0x8u64)).wrapping_mul(1)))
                                .wrapping_add((values[3]).wrapping_mul(1)),
                            )?,
                        )) as u16,
                    );
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_pcStack_50");
                    let stored = (((relocate("IMAGE_DOS_HEADER_140000000"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    (((relocate("IMAGE_DOS_HEADER_140000000"))
                                        .wrapping_add((0x0u64).wrapping_mul(1)))
                                    .wrapping_add(
                                        ((values[9]).wrapping_mul(0x8u64)).wrapping_mul(1),
                                    ))
                                    .wrapping_add((values[3]).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        ))
                        .wrapping_mul(1),
                    ))
                    .wrapping_add((0x2u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            33 => {
                pc = if (u64::from((u64::from((values[5]) as u32)) == (u64::from((0x1u64) as u32))))
                    != 0
                {
                    31
                } else {
                    32
                };
            }
            34 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (values[5]) ^ (0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                values[5] = u64::from(
                    ((0u64.wrapping_sub(
                        ((memory.read_u64(
                            (((relocate("IMAGE_DOS_HEADER_140000000"))
                                .wrapping_add((0x0u64).wrapping_mul(1)))
                            .wrapping_add(((values[9]).wrapping_mul(0x8u64)).wrapping_mul(1)))
                            .wrapping_add((values[3]).wrapping_mul(1)),
                        )? as i64)
                            .wrapping_shr((0x3fu64) as u32) as u64) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as u32,
                );
                pc = 34;
            }
            36 => {
                values[9] = (((values[1]).wrapping_sub(
                    ((relocate("IMAGE_DOS_HEADER_140000000"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                            )) as u32,
                        ))
                        .wrapping_mul(1),
                    ),
                ) as i64)
                    .wrapping_shr((0x3u64) as u32) as u64)
                    & (0xffffffffu64);
                pc = 35;
            }
            37 => {
                values[6] = memory.read_u64(values[10])?;
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from((u64::from(memory.read_u32(values[0])?)) as u32))
                                & (0x1u64)) as u32,
                        )) as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    4
                } else {
                    37
                };
            }
            39 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_pcStack_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination =
                        (relocate("STACK_local_78")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x48u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("STACK_local_res8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_res8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x7u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x5u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 47;
            }
            49 => {
                values[3] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x4u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 48;
            }
            50 => {
                values[10] = ((relocate("IMAGE_DOS_HEADER_140000000"))
                    .wrapping_add((0x0u64).wrapping_mul(1)))
                .wrapping_add(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    ))
                    .wrapping_mul(1),
                );
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = ((relocate("IMAGE_DOS_HEADER_140000000"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as u32,
                        ))
                        .wrapping_mul(1),
                    );
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400ae430", &call_arguments, memory)
                };
                pc = 51;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b003c` (`mspaint.c:154606`).
pub fn fun_1400b003c(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 35;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[4]];
                    invoke("exit", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("exit"));
            }
            1 => {
                {
                    let call_arguments = [0x7u64];
                    invoke("FUN_1400b090c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [0x7u64];
                    invoke("FUN_1400b090c", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = 0;
            }
            4 => {
                return Ok(Some((values[4]) as i32 as i64 as u64));
            }
            5 => {
                {
                    let call_arguments = [0x1u64, 0x0u64];
                    invoke("__scrt_uninitialize_crt", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [];
                    invoke("_cexit", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from(!((values[0]) != 0))) != 0 {
                    6
                } else {
                    5
                };
            }
            8 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    3
                };
            }
            9 => {
                values[1] = ({
                    let call_arguments = [];
                    invoke("FUN_1400b0a9c", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 8;
            }
            10 => {
                values[4] = ({
                    let call_arguments = [
                        relocate("IMAGE_DOS_HEADER_140000000"),
                        0x0u64,
                        values[6],
                        u64::from((values[3]) as u32),
                    ];
                    invoke("AfxWinMain", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                values[6] = {
                    let call_arguments = [];
                    invoke("_get_wide_winmain_command_line", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                values[3] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("__scrt_get_show_window_mode", &call_arguments, memory)
                    }) as u16,
                );
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [memory.read_u64(values[5])?];
                    invoke(
                        "_register_thread_local_exe_atexit_callback",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 12;
            }
            14 => {
                pc = if (u64::from(
                    ((u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64))) != 0)
                        && (({
                            let _ = {
                                values[1] = ({
                                    let call_arguments = [values[5]];
                                    invoke("FUN_1400b0538", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[1]
                            };
                            u64::from(
                                (((values[1]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                values[5] = {
                    let call_arguments = [];
                    invoke("FUN_1400b08f8", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [memory.read_u64(values[5])?, 0x0u64];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                pc = if (u64::from(
                    ((u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64))) != 0)
                        && (({
                            let _ = {
                                values[1] = ({
                                    let call_arguments = [values[5]];
                                    invoke("FUN_1400b0538", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[1]
                            };
                            u64::from(
                                (((values[1]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    16
                } else {
                    15
                };
            }
            18 => {
                values[5] = {
                    let call_arguments = [];
                    invoke("FUN_1400b08f0", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [values[2]];
                    invoke("__scrt_release_startup_lock", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("DAT_1401cf470");
                    let stored = (0x2u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [relocate("DAT_1400e2a98")];
                    invoke("_initterm", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                return Ok(Some((0xffu64) as i32 as i64 as u64));
            }
            23 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[4] = ({
                    let call_arguments = [relocate("DAT_1400e2b20"), relocate("DAT_1400e2b38")];
                    invoke("_initterm_e", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("DAT_1401cf470");
                    let stored = (0x1u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                values[0] = 1u64;
                pc = 19;
            }
            27 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("DAT_1401cf470"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    26
                };
            }
            28 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("DAT_1401cf470"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    1
                };
            }
            29 => {
                values[4] = (((((((((values[7]).wrapping_shr((0x8u64) as u32)) << 8) as i64 >> 8)
                    as u64)
                    & (0xffffffffffffffu64))
                    .wrapping_shl((0x8u64) as u32))
                    | ((values[2]) & (0xffu64))) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 28;
            }
            30 => {
                values[2] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("__scrt_acquire_startup_lock", &call_arguments, memory)
                    }) as u8,
                );
                pc = 29;
            }
            31 => {
                values[0] = 0u64;
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    31
                };
            }
            33 => {
                values[1] = ({
                    let call_arguments = [0x1u64];
                    invoke("FUN_1400b0470", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 32;
            }
            34 => {
                values[4] = ((values[7]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 33;
            }
            35 => {
                values[7] = relocate("CONTEXT_unaff_RBX");
                pc = 34;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b07f8` (`mspaint.c:155139`).
pub fn fun_1400b07f8(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    let mut pc: i32 = 14;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let destination = relocate("DAT_140179080");
                    let stored = !(memory.read_u64(relocate("DAT_140179040"))?);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_140179040");
                    let stored = 0x2b992ddfa233u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("DAT_140179040"))?) == (0x2b992ddfa232u64),
                )) != 0
                {
                    2
                } else {
                    1
                };
            }
            4 => {
                {
                    let destination = relocate("DAT_140179040");
                    let stored = (((((u64::from(
                        (u64::from(
                            memory.read_u32(
                                ((relocate("STACK_local_res10"))
                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?,
                        )) as u32,
                    ))
                    .wrapping_shl((0x20u64) as u32))
                        ^ (((((u64::from(
                            memory.read_u32(
                                ((relocate("STACK_local_res10"))
                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64)
                            & (0xffffffffu64))
                            .wrapping_shl((0x20u64) as u32))
                            | ((u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        ((relocate("STACK_local_res10"))
                                            .wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?,
                                )) as u32,
                            )) & (0xffffffffu64))))
                        ^ (memory.read_u64(
                            (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?))
                        ^ (relocate("STACK_local_18")))
                        & (0xffffffffffffu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [relocate("STACK_local_res10")];
                    invoke("QueryPerformanceCounter", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) ^ (values[0]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                values[0] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("GetCurrentProcessId", &call_arguments, memory)
                    }) as u32,
                );
                pc = 6;
            }
            8 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) ^ (values[0]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                values[0] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("GetCurrentThreadId", &call_arguments, memory)
                    }) as u32,
                );
                pc = 8;
            }
            10 => {
                {
                    let destination =
                        (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_res8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [relocate("STACK_local_res8")];
                    invoke("GetSystemTimeAsFileTime", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let destination =
                        (relocate("STACK_local_res8")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination =
                        (relocate("STACK_local_res8")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("DAT_140179040"))?) == (0x2b992ddfa232u64),
                )) != 0
                {
                    13
                } else {
                    1
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b090c` (`mspaint.c:155247`).
pub fn fun_1400b090c(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [
        u64::from((arguments[0]) as u32),
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 38;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [0x3u64];
                    invoke("FUN_1400b0900", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0a46u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[3]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (((values[2]) as i32 as i64 as u64) as i64)
                                != (((0x1u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    2
                } else {
                    0
                };
            }
            4 => {
                values[3] = ({
                    let call_arguments = [(values[5]).wrapping_add((0x40u64).wrapping_mul(1))];
                    invoke("UnhandledExceptionFilter", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0a35u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [0x0u64];
                    invoke("SetUnhandledExceptionFilter", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0a2au64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = (values[5]).wrapping_add((0x48u64).wrapping_mul(1));
                    let stored = relocate("STACK_local_4d8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[5]).wrapping_add((0x40u64).wrapping_mul(1));
                    let stored = (values[5]).wrapping_add((0x50u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[2] = ({
                    let call_arguments = [];
                    invoke("IsDebuggerPresent", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0a0du64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[5]).wrapping_add((0x54u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[5]).wrapping_add((0x50u64).wrapping_mul(1));
                    let stored = u64::from((0x40000015u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[5]).wrapping_add((0x60u64).wrapping_mul(1));
                    let stored = relocate("CONTEXT_unaff_retaddr");
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [
                        (values[5]).wrapping_add((0x50u64).wrapping_mul(1)),
                        0x0u64,
                        0x98u64,
                    ];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b09ebu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_440");
                    let stored = relocate("stack0x00000008");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_res10"))?,
                        memory.read_u64(relocate("STACK_local_3e0"))?,
                        values[4],
                        memory.read_u64((values[5]).wrapping_add((0x20u64).wrapping_mul(1)))?,
                        memory.read_u64((values[5]).wrapping_add((0x28u64).wrapping_mul(1)))?,
                        memory.read_u64((values[5]).wrapping_add((0x30u64).wrapping_mul(1)))?,
                        memory.read_u64((values[5]).wrapping_add((0x38u64).wrapping_mul(1)))?,
                    ];
                    invoke("RtlVirtualUnwind", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b09b9u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = (values[5]).wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = relocate("STACK_local_4d8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[5]).wrapping_add((0x28u64).wrapping_mul(1));
                    let stored = relocate("STACK_local_res20");
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = (values[5]).wrapping_add((0x30u64).wrapping_mul(1));
                    let stored = relocate("STACK_local_res18");
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = (values[5]).wrapping_add((0x38u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    23
                } else {
                    17
                };
            }
            25 => {
                values[4] = {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_3e0"))?,
                        relocate("STACK_local_res10"),
                        0x0u64,
                    ];
                    invoke("RtlLookupFunctionEntry", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0975u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [relocate("STACK_local_4d8")];
                    invoke("RtlCaptureContext", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b095bu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [relocate("STACK_local_4d8"), 0x0u64, 0x4d0u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0951u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [0x3u64];
                    invoke("FUN_1400b0900", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                {
                    let destination = (values[5]).wrapping_add(
                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64).wrapping_mul(1),
                    );
                    let stored = 0x1400b0940u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                values[5] = relocate("STACK_auStack_5c0");
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [values[1], values[0]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                values[1] = {
                    let call_arguments = [0x29u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    32
                };
            }
            37 => {
                values[2] = ({
                    let call_arguments = [0x17u64];
                    invoke("IsProcessorFeaturePresent", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                values[5] = relocate("STACK_auStack_5c8");
                pc = 37;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b0a9c` (`mspaint.c:155343`).
pub fn fun_1400b0a9c(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64];
    let mut pc: i32 = 4;
    loop {
        match pc {
            0 => {
                return Ok(Some(u64::from((values[2]) != 0)));
            }
            1 => {
                values[2] = 0u64;
                pc = 0;
            }
            2 => {
                values[2] = u64::from(
                    (u64::from(
                        ((((u64::from(
                            memory.read_u32(
                                (((values[0]).wrapping_add((0x3eu64).wrapping_mul(4)))
                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                .wrapping_add(values[1]),
                            )?,
                        )) as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0,
                );
                pc = 0;
            }
            3 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from((values[0]) == (0x0u64))) != 0)
                                || ((u64::from(
                                    (((((u64::from(memory.read_u32(
                                        (values[0]).wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?)) as i32 as i64
                                        as u64) as i16 as i64
                                        as u64) as i32 as i64
                                        as u64) as i64)
                                        != (((0x5a4du64) as i32 as i64 as u64) as i64),
                                )) != 0),
                        )) != 0)
                            || (({
                                let _ = {
                                    values[1] = (u64::from(
                                        memory.read_u32(
                                            ((values[0]).wrapping_add((0xfu64).wrapping_mul(4)))
                                                .wrapping_add((0x0u64).wrapping_mul(1)),
                                        )?,
                                    )) as i32 as i64
                                        as u64;
                                    values[1]
                                };
                                u64::from(
                                    ((((u64::from(
                                        memory.read_u32(
                                            ((values[0]).wrapping_add((0x0u64).wrapping_mul(1)))
                                                .wrapping_add(values[1]),
                                        )?,
                                    )) as i32 as i64 as u64)
                                        as i32 as i64 as u64)
                                        as i64)
                                        != (((0x4550u64) as i32 as i64 as u64) as i64),
                                )
                            }) != 0),
                    )) != 0)
                        || ((u64::from(
                            ((u64::from(
                                ((((u64::from(
                                    memory.read_u16(
                                        (((values[0]).wrapping_add((0x6u64).wrapping_mul(4)))
                                            .wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add(values[1]),
                                    )?,
                                )) as i16 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    != (((0x20bu64) as i32 as i64 as u64) as i64),
                            )) != 0)
                                || ((u64::from(
                                    (u64::from(
                                        (u64::from(
                                            (u64::from(
                                                memory.read_u32(
                                                    (((values[0])
                                                        .wrapping_add((0x21u64).wrapping_mul(4)))
                                                    .wrapping_add((0x0u64).wrapping_mul(1)))
                                                    .wrapping_add(values[1]),
                                                )?,
                                            )) as u32,
                                        )) as u32,
                                    )) < (u64::from((0xfu64) as u32)),
                                )) != 0),
                        )) != 0),
                )) != 0
                {
                    1
                } else {
                    2
                };
            }
            4 => {
                values[0] = {
                    let call_arguments = [0x0u64];
                    invoke("GetModuleHandleW", &call_arguments, memory)
                };
                pc = 3;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L155017` (`mspaint.c:155017`).
/// Original symbol: `_onexit`.
pub fn named_l155017(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64, 0u64];
    let mut pc: i32 = 6;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[2]));
            }
            1 => {
                values[2] = values[0];
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                values[2] = 0x0u64;
                pc = 2;
            }
            4 => {
                values[1] = ({
                    let call_arguments = [];
                    invoke("_crt_atexit", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 3;
            }
            5 => {
                values[1] = ({
                    let call_arguments = [relocate("DAT_1401cf488")];
                    invoke("_register_onexit_function", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 3;
            }
            6 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("DAT_1401cf488"))?)
                        == ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64),
                )) != 0
                {
                    4
                } else {
                    5
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L155043` (`mspaint.c:155043`).
/// Original symbol: `atexit`.
pub fn named_l155043(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64];
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(Some(
                    (((u64::from((values[1]) != (0x0u64))).wrapping_sub(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64,
                ));
            }
            1 => {
                values[1] = {
                    let call_arguments = [values[0]];
                    invoke("_onexit", &call_arguments, memory)
                };
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L155059` (`mspaint.c:155059`).
/// Original symbol: `__raise_securityfailure`.
pub fn named_l155059(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64];
    let mut pc: i32 = 4;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[1], 0xc0000409u64];
                    invoke("TerminateProcess", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[1] = {
                    let call_arguments = [];
                    invoke("GetCurrentProcess", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [values[0]];
                    invoke("UnhandledExceptionFilter", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [0x0u64];
                    invoke("SetUnhandledExceptionFilter", &call_arguments, memory)
                };
                pc = 3;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L157707` (`mspaint.c:157707`).
/// Original symbol: `InterlockedPushEntrySList`.
pub fn named_l157707(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], arguments[1], 0u64];
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[2]));
            }
            1 => {
                values[2] = {
                    let call_arguments = [values[0], values[1]];
                    invoke("InterlockedPushEntrySList", &call_arguments, memory)
                };
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L157808` (`mspaint.c:157808`).
/// Original symbol: `TrySubmitThreadpoolCallback`.
pub fn named_l157808(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 3 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 3,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(Some((values[3]) as i32 as i64 as u64));
            }
            1 => {
                values[3] = ({
                    let call_arguments = [values[0], values[1], values[2]];
                    invoke("TrySubmitThreadpoolCallback", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

pub type Handler = fn(
    &[u64],
    &mut MemoryImage,
    &mut RelocateCall<'_>,
    &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError>;
pub const HANDLERS: &[(&str, usize, Handler)] = &[
    ("FUN_140001310", 0, fun_140001310),
    ("FUN_140004c10", 2, fun_140004c10),
    ("FUN_1400086f0", 3, fun_1400086f0),
    ("FUN_1400113f0", 1, fun_1400113f0),
    ("FUN_140011570", 1, fun_140011570),
    ("FUN_140012300", 0, fun_140012300),
    ("FUN_1400134c0", 0, fun_1400134c0),
    ("FUN_140013530", 0, fun_140013530),
    ("FUN_1400135a0", 0, fun_1400135a0),
    ("FUN_140013610", 0, fun_140013610),
    ("FUN_140013680", 0, fun_140013680),
    ("FUN_1400139a0", 2, fun_1400139a0),
    ("FUN_1400163a0", 3, fun_1400163a0),
    ("FUN_14001aa10", 3, fun_14001aa10),
    ("FUN_140028d90", 4, fun_140028d90),
    ("FUN_14002c850", 5, fun_14002c850),
    ("FUN_14002ca70", 5, fun_14002ca70),
    ("FUN_14002cc90", 5, fun_14002cc90),
    ("FUN_14002cef0", 5, fun_14002cef0),
    ("FUN_14002d120", 5, fun_14002d120),
    ("FUN_14002ec80", 5, fun_14002ec80),
    ("FUN_140030210", 5, fun_140030210),
    ("FUN_140030420", 5, fun_140030420),
    ("FUN_1400324c0", 3, fun_1400324c0),
    ("FUN_14003e7b0", 2, fun_14003e7b0),
    ("FUN_14004d580", 2, fun_14004d580),
    ("FUN_1400511d0", 2, fun_1400511d0),
    ("FUN_14005a2a0", 1, fun_14005a2a0),
    ("FUN_140075530", 1, fun_140075530),
    ("FUN_14007c4b0", 1, fun_14007c4b0),
    ("FUN_14007d400", 2, fun_14007d400),
    ("FUN_14008ec90", 1, fun_14008ec90),
    ("FUN_1400aa8a0", 1, fun_1400aa8a0),
    ("FUN_1400ae624", 2, fun_1400ae624),
    ("FUN_1400ae870", 2, fun_1400ae870),
    ("FUN_1400b003c", 0, fun_1400b003c),
    ("FUN_1400b07f8", 0, fun_1400b07f8),
    ("FUN_1400b090c", 1, fun_1400b090c),
    ("FUN_1400b0a9c", 0, fun_1400b0a9c),
    ("NAMED_L155017", 1, named_l155017),
    ("NAMED_L155043", 1, named_l155043),
    ("NAMED_L155059", 1, named_l155059),
    ("NAMED_L157707", 2, named_l157707),
    ("NAMED_L157808", 3, named_l157808),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_140004c10",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400086f0",
        &[
            ("STACK_auStack_138", -312),
            ("STACK_local_118", -280),
            ("STACK_local_110", -272),
            ("STACK_local_108", -264),
            ("STACK_local_f8", -248),
            ("STACK_local_f4", -244),
            ("STACK_local_f0", -240),
            ("STACK_local_e8", -232),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_c0", -192),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_pHStack_98", -152),
            ("STACK_local_90", -144),
            ("STACK_uStack_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_6c", -108),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_48", -72),
        ],
    ),
    (
        "FUN_140012300",
        &[
            ("STACK_in_stack_00000028", 40),
            ("STACK_in_stack_00000030", 48),
            ("STACK_in_stack_00000038", 56),
            ("STACK_in_stack_00000040", 64),
            ("STACK_in_stack_00000050", 80),
            ("STACK_auStack_1548", -5448),
            ("STACK_local_1528", -5416),
            ("STACK_local_1520", -5408),
            ("STACK_local_1518", -5400),
            ("STACK_local_1510", -5392),
            ("STACK_local_1508", -5384),
            ("STACK_local_1500", -5376),
            ("STACK_local_14f8", -5368),
            ("STACK_local_14f0", -5360),
            ("STACK_local_14e8", -5352),
            ("STACK_local_14e0", -5344),
            ("STACK_local_14d8", -5336),
            ("STACK_local_14d0", -5328),
            ("STACK_local_14c8", -5320),
            ("STACK_local_14c4", -5316),
            ("STACK_local_1428", -5160),
            ("STACK_local_1028", -4136),
            ("STACK_local_28", -40),
            ("STACK_uStack_18", -24),
        ],
    ),
    (
        "FUN_1400139a0",
        &[
            ("STACK_auStack_148", -328),
            ("STACK_local_128", -296),
            ("STACK_uStack_120", -288),
            ("STACK_local_118", -280),
            ("STACK_uStack_110", -272),
            ("STACK_local_108", -264),
            ("STACK_ppuStack_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_uStack_f0", -240),
            ("STACK_uStack_ec", -236),
            ("STACK_local_e8", -232),
            ("STACK_uStack_e4", -228),
            ("STACK_uStack_e0", -224),
            ("STACK_uStack_dc", -220),
            ("STACK_local_d8", -216),
            ("STACK_uStack_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_lStack_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_lStack_70", -112),
            ("STACK_local_68", -104),
            ("STACK_uStack_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_57", -87),
            ("STACK_local_4f", -79),
            ("STACK_local_4b", -75),
            ("STACK_local_49", -73),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_1400163a0",
        &[
            ("STACK_in_stack_ffffffffffffff6c", -148),
            ("STACK_auStackY_c8", -200),
            ("STACK_local_78", -120),
            ("STACK_uStack_70", -112),
            ("STACK_local_68", -104),
            ("STACK_uStack_64", -100),
            ("STACK_uStack_60", -96),
            ("STACK_uStack_5c", -92),
            ("STACK_local_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_uStack_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_14001aa10",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_uStack_2b", -43),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140028d90",
        &[
            ("STACK_auStack_b8", -184),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_uStack_84", -132),
            ("STACK_uStack_80", -128),
            ("STACK_uStack_7c", -124),
            ("STACK_local_78", -120),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_4c", -76),
            ("STACK_uStack_47", -71),
            ("STACK_local_44", -68),
            ("STACK_local_40", -64),
        ],
    ),
    (
        "FUN_14002c850",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_19", -25),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14002ca70",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_11", -17),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14002cc90",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_19", -25),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14002cef0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_19", -25),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14002d120",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_19", -25),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14002ec80",
        &[("STACK_local_78", -120), ("STACK_local_70", -112)],
    ),
    (
        "FUN_140030210",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_19", -25),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400324c0",
        &[("STACK_in_stack_00000038", 56), ("STACK_local_18", -24)],
    ),
    (
        "FUN_14003e7b0",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_uStack_64", -100),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_uStack_3b", -59),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
        ],
    ),
    (
        "FUN_1400511d0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14005a2a0",
        &[
            ("STACK_local_48", -72),
            ("STACK_plStack_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140075530",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14007c4b0",
        &[
            ("STACK_in_stack_00000030", 48),
            ("STACK_auStack_1098", -4248),
            ("STACK_local_1070", -4208),
            ("STACK_local_1068", -4200),
            ("STACK_local_1060", -4192),
            ("STACK_local_1048", -4168),
            ("STACK_local_1040", -4160),
            ("STACK_local_1038", -4152),
            ("STACK_local_1028", -4136),
            ("STACK_local_1018", -4120),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007d400",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14008ec90",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_lStack_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_lStack_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400aa8a0",
        &[
            ("STACK_local_48", -72),
            ("STACK_plStack_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_1400ae870",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_pcStack_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_1400b07f8",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_res10", 16),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400b090c",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_res18", 24),
            ("STACK_local_res20", 32),
            ("STACK_auStack_5c8", -1480),
            ("STACK_auStack_5c0", -1472),
            ("STACK_local_4d8", -1240),
            ("STACK_local_440", -1088),
            ("STACK_local_3e0", -992),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_140004c10",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400086f0",
        &[
            ("STACK_auStack_138", 32),
            ("STACK_local_118", 8),
            ("STACK_local_110", 8),
            ("STACK_local_108", 8),
            ("STACK_local_f8", 4),
            ("STACK_local_f4", 4),
            ("STACK_local_f0", 8),
            ("STACK_local_e8", 8),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 24),
            ("STACK_local_c0", 24),
            ("STACK_local_a8", 4),
            ("STACK_local_a0", 8),
            ("STACK_pHStack_98", 8),
            ("STACK_local_90", 8),
            ("STACK_uStack_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 4),
            ("STACK_local_6c", 4),
            ("STACK_local_68", 4),
            ("STACK_local_60", 8),
            ("STACK_local_58", 16),
            ("STACK_local_48", 8),
        ],
    ),
    (
        "FUN_140012300",
        &[
            ("STACK_in_stack_00000028", 8),
            ("STACK_in_stack_00000030", 8),
            ("STACK_in_stack_00000038", 8),
            ("STACK_in_stack_00000040", 8),
            ("STACK_in_stack_00000050", 4),
            ("STACK_auStack_1548", 32),
            ("STACK_local_1528", 8),
            ("STACK_local_1520", 8),
            ("STACK_local_1518", 4),
            ("STACK_local_1510", 8),
            ("STACK_local_1508", 8),
            ("STACK_local_1500", 1),
            ("STACK_local_14f8", 8),
            ("STACK_local_14f0", 8),
            ("STACK_local_14e8", 8),
            ("STACK_local_14e0", 8),
            ("STACK_local_14d8", 4),
            ("STACK_local_14d0", 8),
            ("STACK_local_14c8", 4),
            ("STACK_local_14c4", 1),
            ("STACK_local_1428", 1024),
            ("STACK_local_1028", 4096),
            ("STACK_local_28", 8),
            ("STACK_uStack_18", 8),
        ],
    ),
    (
        "FUN_1400139a0",
        &[
            ("STACK_auStack_148", 32),
            ("STACK_local_128", 8),
            ("STACK_uStack_120", 8),
            ("STACK_local_118", 8),
            ("STACK_uStack_110", 8),
            ("STACK_local_108", 8),
            ("STACK_ppuStack_100", 8),
            ("STACK_local_f8", 8),
            ("STACK_uStack_f0", 4),
            ("STACK_uStack_ec", 4),
            ("STACK_local_e8", 4),
            ("STACK_uStack_e4", 4),
            ("STACK_uStack_e0", 4),
            ("STACK_uStack_dc", 4),
            ("STACK_local_d8", 56),
            ("STACK_uStack_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_lStack_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_lStack_70", 8),
            ("STACK_local_68", 8),
            ("STACK_uStack_60", 8),
            ("STACK_local_58", 1),
            ("STACK_local_57", 8),
            ("STACK_local_4f", 4),
            ("STACK_local_4b", 2),
            ("STACK_local_49", 1),
            ("STACK_local_48", 16),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_1400163a0",
        &[
            ("STACK_in_stack_ffffffffffffff6c", 4),
            ("STACK_auStackY_c8", 32),
            ("STACK_local_78", 8),
            ("STACK_uStack_70", 8),
            ("STACK_local_68", 4),
            ("STACK_uStack_64", 4),
            ("STACK_uStack_60", 4),
            ("STACK_uStack_5c", 4),
            ("STACK_local_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_uStack_28", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_14001aa10",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 5),
            ("STACK_uStack_2b", 3),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140028d90",
        &[
            ("STACK_auStack_b8", 40),
            ("STACK_local_90", 8),
            ("STACK_local_88", 4),
            ("STACK_uStack_84", 4),
            ("STACK_uStack_80", 4),
            ("STACK_uStack_7c", 4),
            ("STACK_local_78", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 4),
            ("STACK_local_4c", 5),
            ("STACK_uStack_47", 3),
            ("STACK_local_44", 4),
            ("STACK_local_40", 8),
        ],
    ),
    (
        "FUN_14002c850",
        &[
            ("STACK_auStack_48", 47),
            ("STACK_local_19", 1),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_14002ca70",
        &[
            ("STACK_auStack_38", 39),
            ("STACK_local_11", 1),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14002cc90",
        &[
            ("STACK_auStack_78", 95),
            ("STACK_local_19", 1),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_14002cef0",
        &[
            ("STACK_auStack_58", 63),
            ("STACK_local_19", 1),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_14002d120",
        &[
            ("STACK_auStack_78", 95),
            ("STACK_local_19", 1),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_14002ec80",
        &[("STACK_local_78", 4), ("STACK_local_70", 8)],
    ),
    (
        "FUN_140030210",
        &[
            ("STACK_auStack_58", 63),
            ("STACK_local_19", 1),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_1400324c0",
        &[("STACK_in_stack_00000038", 4), ("STACK_local_18", 16)],
    ),
    (
        "FUN_14003e7b0",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 4),
            ("STACK_uStack_64", 4),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 5),
            ("STACK_uStack_3b", 3),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
        ],
    ),
    (
        "FUN_1400511d0",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14005a2a0",
        &[
            ("STACK_local_48", 8),
            ("STACK_plStack_40", 8),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140075530",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14007c4b0",
        &[
            ("STACK_in_stack_00000030", 8),
            ("STACK_auStack_1098", 40),
            ("STACK_local_1070", 8),
            ("STACK_local_1068", 8),
            ("STACK_local_1060", 8),
            ("STACK_local_1048", 8),
            ("STACK_local_1040", 4),
            ("STACK_local_1038", 16),
            ("STACK_local_1028", 16),
            ("STACK_local_1018", 4096),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007d400",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 72),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14008ec90",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_lStack_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_lStack_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400aa8a0",
        &[
            ("STACK_local_48", 8),
            ("STACK_plStack_40", 8),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_1400ae870",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_pcStack_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 4),
        ],
    ),
    (
        "FUN_1400b07f8",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_res10", 8),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_1400b090c",
        &[
            ("STACK_local_res10", 8),
            ("STACK_local_res18", 8),
            ("STACK_local_res20", 8),
            ("STACK_auStack_5c8", 8),
            ("STACK_auStack_5c0", 232),
            ("STACK_local_4d8", 152),
            ("STACK_local_440", 8),
            ("STACK_local_3e0", 8),
        ],
    ),
];
