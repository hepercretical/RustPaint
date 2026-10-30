//! Checked translations generated from original bodies by batch 020.
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
/// Translation of `FUN_140001290` (`mspaint.c:5238`).
pub fn fun_140001290(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 3;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let destination = relocate("DAT_1401cff20");
                    let stored = relocate("FUN_1400903e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_1401cfc10");
                    let stored = relocate("FUN_1400902a0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("_DAT_1401cfc08");
                    let stored = relocate("FUN_140090290");
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14000c380` (`mspaint.c:15661`).
pub fn fun_14000c380(
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
    ];
    let mut pc: i32 = 40;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_14000cf80", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    1
                };
            }
            4 => {
                {
                    let call_arguments = [
                        values[7],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_80"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            7 => {
                pc = if (u64::from(
                    (((((values[4]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    3
                };
            }
            8 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    7
                };
            }
            9 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 8;
            }
            10 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                values[4] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 10;
            }
            12 => {
                values[1] = (memory.read_u64(relocate("STACK_local_80"))?).wrapping_add(0x18u64);
                pc = 11;
            }
            13 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 12;
            }
            14 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_80"))?) != (0x0u64))) != 0
                {
                    13
                } else {
                    3
                };
            }
            15 => {
                values[3] = ({
                    let call_arguments = [
                        values[6],
                        relocate("STACK_local_80"),
                        relocate("STACK_local_60"),
                    ];
                    invoke("FUN_14003edb0", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [relocate("STACK_local_80"), relocate("STACK_local_78")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0xdu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("L\"Rotate layers\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = relocate("STACK_local_60");
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("STACK_local_60");
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                values[6] = {
                    let call_arguments = [];
                    invoke("FUN_14000c050", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [0x0u64, values[9]];
                    invoke(
                        "Paint::Telemetry::TraceOperationLogger::SendRotateOperation",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 23;
            }
            25 => {
                {
                    let call_arguments =
                        [memory.read_u64((values[5]).wrapping_add((0x8u64).wrapping_mul(1)))?];
                    invoke("CCmdTarget::BeginWaitCursor", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                values[5] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [values[2], values[9]];
                    invoke("CPaintTool::OnRotate", &call_arguments, memory)
                };
                pc = 0;
            }
            28 => {
                {
                    let call_arguments = [values[8], values[9]];
                    invoke(
                        "Paint::Telemetry::TraceOperationLogger::SendRotateOperation",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 27;
            }
            29 => {
                values[8] = (0x1u64) as i32 as i64 as u64;
                pc = 28;
            }
            30 => {
                values[8] = (0x2u64) as i32 as i64 as u64;
                pc = 28;
            }
            31 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14000a4a0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a4a0"));
            }
            32 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    28
                };
            }
            33 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    32
                };
            }
            34 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    33
                };
            }
            35 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x60u64))?,
                        values[2],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[3]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[3] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(values[2])?).wrapping_add(0x68u64),
                                        )?,
                                        values[2],
                                        0x3u64,
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[3]
                            };
                            u64::from(
                                ((u64::from(
                                    (((values[3]) as i32 as i64 as u64) as i64)
                                        == (((0x0u64) as i32 as i64 as u64) as i64),
                                )) != 0)
                                    || (({
                                        let _ = {
                                            values[10] = {
                                                let call_arguments =
                                                    [values[9], 0x4056800000000000u64];
                                                invoke("fmod", &call_arguments, memory)
                                            };
                                            values[10]
                                        };
                                        u64::from(
                                            (f64::from_bits(values[10]))
                                                != (f64::from_bits(0x0u64)),
                                        )
                                    }) != 0),
                            )
                        }) != 0),
                )) != 0
                {
                    26
                } else {
                    35
                };
            }
            37 => {
                values[8] = (0x3u64) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                                .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x21u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 37;
            }
            39 => {
                values[2] = memory.read_u64(relocate("DAT_1401d06e0"))?;
                pc = 38;
            }
            40 => {
                values[9] = {
                    let call_arguments = [values[0]];
                    invoke("NormalizeAngle", &call_arguments, memory)
                };
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400122c0` (`mspaint.c:20637`).
pub fn fun_1400122c0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [u64::from((arguments[0]) as u32), 0u64];
    let mut pc: i32 = 5;
    loop {
        match pc {
            0 => {
                return Ok(Some(
                    (((values[1]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64,
                ));
            }
            1 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_1401cfb74");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfb74"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("_DAT_1401cfb70");
                    let stored = u64::from((values[0]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[1] = ((u64::from(memory.read_u32(relocate("DAT_1401cfb74"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 4;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400122e0` (`mspaint.c:20654`).
pub fn fun_1400122e0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [u64::from((arguments[0]) as u32), 0u64];
    let mut pc: i32 = 5;
    loop {
        match pc {
            0 => {
                return Ok(Some(
                    (((values[1]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64,
                ));
            }
            1 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_1401cfb7c");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfb7c"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("_DAT_1401cfb78");
                    let stored = u64::from((values[0]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[1] = ((u64::from(memory.read_u32(relocate("DAT_1401cfb7c"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 4;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140012460` (`mspaint.c:20756`).
pub fn fun_140012460(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [u64::from((arguments[0]) as u32), 0u64];
    let mut pc: i32 = 5;
    loop {
        match pc {
            0 => {
                return Ok(Some(
                    (((values[1]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64,
                ));
            }
            1 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("DAT_1401cfb84");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfb84"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("_DAT_1401cfb80");
                    let stored = u64::from((values[0]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[1] = ((u64::from(memory.read_u32(relocate("DAT_1401cfb84"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 4;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140012480` (`mspaint.c:20773`).
pub fn fun_140012480(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let values = [u64::from((arguments[0]) as u32)];
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x1u64));
            }
            1 => {
                {
                    let destination = relocate("_DAT_1401cfb88");
                    let stored = u64::from((values[0]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140015d70` (`mspaint.c:23693`).
pub fn fun_140015d70(
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
        u64::from((arguments[2]) as u32),
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
    let mut pc: i32 = 89;
    loop {
        match pc {
            0 => {
                return Ok(Some(u64::from((values[4]) as u8)));
            }
            1 => {
                return Ok(Some(u64::from((values[4]) as u8)));
            }
            2 => {
                {
                    let call_arguments = [values[7]];
                    invoke("GlobalFree", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_e8"))?)) as i8 as i64 as u64)
                        as i32 as i64 as u64) as i64)
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
                    let call_arguments = [values[13]];
                    invoke("GlobalUnlock", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from((values[13]) != (0x0u64))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("CClientDC::~CClientDC", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [values[3]];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (values[14]) != 0 { 7 } else { 6 };
            }
            9 => {
                values[4] = u64::from((0x0u64) as u8);
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x3f4u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_d8"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_d8"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 8;
            }
            14 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_d8"))?) != (0x0u64))) != 0
                {
                    13
                } else {
                    8
                };
            }
            15 => {
                values[4] = u64::from(
                    ({
                        let call_arguments = [
                            memory.read_u64(relocate("STACK_local_e0"))?,
                            relocate("STACK_local_d8"),
                            0x2u64,
                            relocate("STACK_local_50"),
                        ];
                        invoke("FUN_140017480", &call_arguments, memory)
                    }) as u8,
                );
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x2ef31ef800007b9du64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x11d30728b96b3ca9u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                pc = 17;
            }
            19 => {
                {
                    let destination = (values[9]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    15
                };
            }
            21 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        relocate("STACK_local_50"),
                    ];
                    invoke("GdipGetImageRawFormat", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 20;
            }
            22 => {
                pc = if (u64::from((values[9]) == (0x0u64))) != 0 {
                    17
                } else {
                    21
                };
            }
            23 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                values[9] = {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_68"))?, 0x0u64];
                    invoke("FUN_14001a000", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_68"))?) == (0x0u64))) != 0
                {
                    12
                } else {
                    26
                };
            }
            28 => {
                values[14] = u64::from(
                    (u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64))) != 0,
                );
                pc = 27;
            }
            29 => {
                values[3] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_68"),
                        values[8],
                        memory.read_u64(relocate("STACK_uStack_88"))?,
                    ];
                    invoke("FUN_140045bb0", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [relocate("STACK_local_90"), 0x0u64];
                    invoke("CClientDC::CClientDC", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_uStack_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_uStack_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    50
                } else {
                    37
                };
            }
            39 => {
                values[11] = (values[11]).wrapping_add(values[12]);
                pc = 38;
            }
            40 => {
                values[10] =
                    u64::from((u64::from(((values[10]).wrapping_sub(values[12])) as u32)) as u32);
                pc = 39;
            }
            41 => {
                pc = 5;
            }
            42 => {
                values[4] = u64::from((0x0u64) as u8);
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x3e9u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    (u64::from((values[5]) as u32)) != (u64::from((values[12]) as u32)),
                )) != 0
                {
                    45
                } else {
                    40
                };
            }
            47 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [
                            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x80u64))?,
                            values[1],
                            values[11],
                            values[12],
                        ];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 46;
            }
            48 => {
                values[12] = u64::from((0x4000u64) as u32);
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    (u64::from((0x4000u64) as u32)) < (u64::from((values[10]) as u32)),
                )) != 0
                {
                    48
                } else {
                    47
                };
            }
            50 => {
                values[12] = u64::from((values[10]) as u32);
                pc = 49;
            }
            51 => {
                values[13] = 0x0u64;
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                pc = if (u64::from((values[8]) == (0x0u64))) != 0 {
                    52
                } else {
                    50
                };
            }
            54 => {
                values[13] = values[7];
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                values[11] = values[8];
                pc = 56;
            }
            58 => {
                values[8] = {
                    let call_arguments = [values[7]];
                    invoke("GlobalLock", &call_arguments, memory)
                };
                pc = 57;
            }
            59 => {
                pc = 3;
            }
            60 => {
                values[4] = u64::from((0x0u64) as u8);
                pc = 59;
            }
            61 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 60;
            }
            62 => {
                pc = if (u64::from((values[7]) == (0x0u64))) != 0 {
                    61
                } else {
                    58
                };
            }
            63 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = (u64::from((values[7]) != (0x0u64))) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 63;
            }
            65 => {
                values[7] = {
                    let call_arguments = [0x40u64, values[10]];
                    invoke("GlobalAlloc", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                return Ok(Some(u64::from((0x0u64) as u8)));
            }
            67 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    66
                } else {
                    65
                };
            }
            68 => {
                values[10] =
                    u64::from((u64::from(((values[10]).wrapping_sub(0xeu64)) as u32)) as u32);
                pc = 67;
            }
            69 => {
                return Ok(Some(u64::from((0x0u64) as u8)));
            }
            70 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x3f4u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_d0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 71;
            }
            73 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((values[6]) as i32 as i64 as u64) as i64)
                                    != (((0xeu64) as i32 as i64 as u64) as i64),
                            )) != 0)
                                || ((u64::from(
                                    ((((memory.read_u64(relocate("STACK_local_60"))?) as i16 as i64
                                        as u64) as i32 as i64
                                        as u64) as i64)
                                        != (((0x4d42u64) as i32 as i64 as u64) as i64),
                                )) != 0),
                        )) != 0)
                            || ((u64::from(
                                (u64::from(
                                    (u64::from(
                                        ((u64::from(
                                            (u64::from(
                                                memory.read_u32(relocate("STACK_local_56"))?,
                                            )) as u32,
                                        ))
                                        .wrapping_sub(0x1u64))
                                            as u32,
                                    )) as u32,
                                )) < (u64::from((0xdu64) as u32)),
                            )) != 0),
                    )) != 0)
                        || ((u64::from(
                            (u64::from((values[10]) as u32))
                                <= (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("STACK_local_56"))?))
                                            as u32,
                                    )) as u32,
                                )),
                        )) != 0),
                )) != 0
                {
                    72
                } else {
                    68
                };
            }
            74 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x80u64))?,
                        values[1],
                        relocate("STACK_local_60"),
                        0xeu64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 73;
            }
            75 => {
                pc = if (values[14]) != 0 { 74 } else { 67 };
            }
            76 => {
                values[10] =
                    u64::from((u64::from(((values[10]).wrapping_sub(0x4u64)) as u32)) as u32);
                pc = 75;
            }
            77 => {
                pc = 72;
            }
            78 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[6]) as i32 as i64 as u64) as i64)
                            != (((0x4u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            (u64::from((values[10]) as u32))
                                < (u64::from(
                                    (u64::from(
                                        (memory.read_u64(relocate("STACK_local_68"))?) as u32,
                                    )) as u32,
                                )),
                        )) != 0),
                )) != 0
                {
                    77
                } else {
                    76
                };
            }
            79 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x80u64))?,
                        values[1],
                        relocate("STACK_local_68"),
                        0x4u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 78;
            }
            80 => {
                pc = if (u64::from((u64::from((values[2]) as u32)) == (u64::from((0x3u64) as u32))))
                    != 0
                {
                    79
                } else {
                    75
                };
            }
            81 => {
                return Ok(Some(u64::from((0x1u64) as u8)));
            }
            82 => {
                {
                    let destination = (values[0]).wrapping_add(0xa31u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 81;
            }
            83 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    82
                } else {
                    80
                };
            }
            84 => {
                values[10] = u64::from(
                    ({
                        let call_arguments = [
                            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x78u64))?,
                            values[1],
                        ];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                values[14] = 1u64;
                pc = 85;
            }
            87 => {
                values[14] = 0u64;
                pc = 85;
            }
            88 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((u64::from(((values[2]) & (0xfffffffcu64)) as u32)) as u32))
                            == (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            (u64::from((values[2]) as u32)) != (u64::from((0x1u64) as u32)),
                        )) != 0),
                )) != 0
                {
                    86
                } else {
                    87
                };
            }
            89 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_118"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14001cd30` (`mspaint.c:29717`).
pub fn fun_14001cd30(
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
        u64::from((arguments[1]) as u32),
        u64::from((arguments[2]) as u32),
        arguments[3],
        arguments[4],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 37;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = values[4];
                    let stored = u64::from((values[9]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                pc = if (0x1u64) != 0 { 14 } else { 1 };
            }
            3 => {
                values[11] = (values[11]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 2;
            }
            4 => {
                pc = 1;
            }
            5 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[10] =
                    u64::from((u64::from(((values[10]).wrapping_sub(0x1u64)) as u32)) as u32);
                pc = 5;
            }
            7 => {
                values[3] = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 6;
            }
            8 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let destination = values[3];
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                values[5] = memory.read_u64(values[11])?;
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [values[3]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(((memory.read_u64(values[3])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                pc = if (u64::from((values[3]) != (values[11]))) != 0 {
                    13
                } else {
                    7
                };
            }
            15 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    2
                } else {
                    1
                };
            }
            16 => {
                values[10] = u64::from((values[9]) as u32);
                pc = 15;
            }
            17 => {
                values[11] = (memory.read_u64((values[8]).wrapping_add(0x70u64))?)
                    .wrapping_add((values[1]).wrapping_mul(0x8u64));
                pc = 16;
            }
            18 => {
                values[9] = u64::from((values[2]) as u32);
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    (u64::from((values[2]) as u32)) < (u64::from((values[9]) as u32)),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                values[8] = values[7];
                pc = 19;
            }
            21 => {
                values[9] = u64::from(
                    (u64::from(
                        (((((memory.read_u64(relocate("_DAT_00000078"))?)
                            .wrapping_sub(memory.read_u64(relocate("_DAT_00000070"))?))
                        .wrapping_shr(0x3u64 as u32)) as i32 as i64
                            as u64)
                            .wrapping_sub(values[1])) as u32,
                    )) as u32,
                );
                pc = 20;
            }
            22 => {
                values[9] = u64::from((values[2]) as u32);
                pc = 17;
            }
            23 => {
                pc = if (u64::from(
                    (u64::from((values[2]) as u32)) < (u64::from((values[9]) as u32)),
                )) != 0
                {
                    22
                } else {
                    17
                };
            }
            24 => {
                values[9] = u64::from(
                    (u64::from(
                        (((((memory.read_u64((values[8]).wrapping_add(0x78u64))?)
                            .wrapping_sub(memory.read_u64((values[8]).wrapping_add(0x70u64))?)
                            as i64)
                            .wrapping_shr(0x3u64 as u32) as u64) as i32
                            as i64 as u64)
                            .wrapping_sub(values[1])) as u32,
                    )) as u32,
                );
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    ((values[8]) as i64)
                        == (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    24
                };
            }
            26 => {
                pc = if (u64::from(
                    (u64::from((values[1]) as u32))
                        < (u64::from(
                            (u64::from(
                                (((values[6]).wrapping_sub(
                                    memory.read_u64((values[12]).wrapping_add(0x70u64))?,
                                ) as i64)
                                    .wrapping_shr(0x3u64 as u32)
                                    as u64) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    25
                } else {
                    1
                };
            }
            27 => {
                values[12] = values[8];
                pc = 26;
            }
            28 => {
                values[6] = memory.read_u64((values[8]).wrapping_add(0x78u64))?;
                pc = 27;
            }
            29 => {
                pc = if (u64::from(
                    ((values[8]) as i64)
                        != (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    26
                };
            }
            30 => {
                values[12] = values[7];
                pc = 29;
            }
            31 => {
                values[6] = memory.read_u64(relocate("_DAT_00000078"))?;
                pc = 30;
            }
            32 => {
                values[8] = values[7];
                pc = 31;
            }
            33 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    32
                } else {
                    31
                };
            }
            34 => {
                values[8] =
                    (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
                pc = 33;
            }
            35 => {
                values[9] = u64::from((0x0u64) as u32);
                pc = 34;
            }
            36 => {
                values[7] = 0x0u64;
                pc = 35;
            }
            37 => {
                {
                    let call_arguments =
                        [values[3], 0x0u64, (values[2]).wrapping_shl(0x3u64 as u32)];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 36;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14001ce50` (`mspaint.c:29785`).
pub fn fun_14001ce50(
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
    ];
    let mut pc: i32 = 22;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = values[3];
                    let stored = u64::from(
                        (u64::from(
                            (u64::from((values[5]) as u32))
                                < (u64::from(
                                    (u64::from(
                                        (((values[6]).wrapping_sub(
                                            memory.read_u64((values[8]).wrapping_add(0x70u64))?,
                                        ) as i64)
                                            .wrapping_shr(0x3u64 as u32)
                                            as u64) as u32,
                                    )) as u32,
                                )),
                        )) != 0,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 0;
            }
            2 => {
                values[6] = memory.read_u64(relocate("_DAT_00000078"))?;
                pc = 1;
            }
            3 => {
                {
                    let destination = values[2];
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                values[5] = u64::from(
                    (u64::from(
                        (((values[6]).wrapping_sub(memory.read_u64(relocate("_DAT_00000070"))?))
                            .wrapping_shr(0x3u64 as u32)) as u32,
                    )) as u32,
                );
                pc = 3;
            }
            5 => {
                values[8] = values[7];
                pc = 1;
            }
            6 => {
                values[6] = memory.read_u64((values[7]).wrapping_add(0x78u64))?;
                pc = 5;
            }
            7 => {
                {
                    let destination = values[2];
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                values[5] = u64::from(
                    (u64::from(
                        (((values[6])
                            .wrapping_sub(memory.read_u64((values[7]).wrapping_add(0x70u64))?)
                            as i64)
                            .wrapping_shr(0x3u64 as u32) as u64) as u32,
                    )) as u32,
                );
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((values[7]) as i64)
                        == (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    8
                };
            }
            10 => {
                pc = if (u64::from(
                    ((u64::from(((values[6]) as i64) != ((values[9]) as i64))) != 0)
                        && (({
                            let _ = {
                                values[4] = ({
                                    let call_arguments = [relocate("STACK_local_res10"), values[6]];
                                    invoke("FUN_140001a20", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[4]
                            };
                            u64::from(
                                (((values[4]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    11
                } else {
                    9
                };
            }
            11 => {
                values[6] = (values[6]).wrapping_add(0x8u64);
                pc = 10;
            }
            12 => {
                {
                    let destination =
                        (relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            13 => {
                values[6] = memory.read_u64((values[6]).wrapping_add(0x70u64))?;
                pc = 12;
            }
            14 => {
                values[9] = memory.read_u64((values[7]).wrapping_add(0x78u64))?;
                pc = 13;
            }
            15 => {
                values[6] = values[7];
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    ((values[7]) as i64)
                        != (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    15
                } else {
                    13
                };
            }
            17 => {
                values[9] = memory.read_u64(relocate("_DAT_00000078"))?;
                pc = 16;
            }
            18 => {
                values[6] = values[8];
                pc = 17;
            }
            19 => {
                values[7] = values[8];
                pc = 18;
            }
            20 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[7] =
                    (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
                pc = 20;
            }
            22 => {
                values[8] = 0x0u64;
                pc = 21;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14001d0d0` (`mspaint.c:29956`).
pub fn fun_14001d0d0(
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
        u64::from((arguments[1]) as u32),
        u64::from((arguments[2]) as u32),
        arguments[3],
        arguments[4],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 37;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = values[4];
                    let stored = u64::from((values[9]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                pc = if (0x1u64) != 0 { 14 } else { 1 };
            }
            3 => {
                values[11] = (values[11]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 2;
            }
            4 => {
                pc = 1;
            }
            5 => {
                pc = if (u64::from(
                    (u64::from((values[10]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[10] =
                    u64::from((u64::from(((values[10]).wrapping_sub(0x1u64)) as u32)) as u32);
                pc = 5;
            }
            7 => {
                values[3] = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 6;
            }
            8 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let destination = values[3];
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                values[5] = memory.read_u64(values[11])?;
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [values[3]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(((memory.read_u64(values[3])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                pc = if (u64::from((values[3]) != (values[11]))) != 0 {
                    13
                } else {
                    7
                };
            }
            15 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    2
                } else {
                    1
                };
            }
            16 => {
                values[10] = u64::from((values[9]) as u32);
                pc = 15;
            }
            17 => {
                values[11] = (memory.read_u64((values[8]).wrapping_add(0x70u64))?)
                    .wrapping_add((values[1]).wrapping_mul(0x8u64));
                pc = 16;
            }
            18 => {
                values[9] = u64::from((values[2]) as u32);
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    (u64::from((values[2]) as u32)) < (u64::from((values[9]) as u32)),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                values[8] = values[7];
                pc = 19;
            }
            21 => {
                values[9] = u64::from(
                    (u64::from(
                        (((((memory.read_u64(relocate("_DAT_00000078"))?)
                            .wrapping_sub(memory.read_u64(relocate("_DAT_00000070"))?))
                        .wrapping_shr(0x3u64 as u32)) as i32 as i64
                            as u64)
                            .wrapping_sub(values[1])) as u32,
                    )) as u32,
                );
                pc = 20;
            }
            22 => {
                values[9] = u64::from((values[2]) as u32);
                pc = 17;
            }
            23 => {
                pc = if (u64::from(
                    (u64::from((values[2]) as u32)) < (u64::from((values[9]) as u32)),
                )) != 0
                {
                    22
                } else {
                    17
                };
            }
            24 => {
                values[9] = u64::from(
                    (u64::from(
                        (((((memory.read_u64((values[8]).wrapping_add(0x78u64))?)
                            .wrapping_sub(memory.read_u64((values[8]).wrapping_add(0x70u64))?)
                            as i64)
                            .wrapping_shr(0x3u64 as u32) as u64) as i32
                            as i64 as u64)
                            .wrapping_sub(values[1])) as u32,
                    )) as u32,
                );
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    ((values[8]) as i64)
                        == (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    24
                };
            }
            26 => {
                pc = if (u64::from(
                    (u64::from((values[1]) as u32))
                        < (u64::from(
                            (u64::from(
                                (((values[6]).wrapping_sub(
                                    memory.read_u64((values[12]).wrapping_add(0x70u64))?,
                                ) as i64)
                                    .wrapping_shr(0x3u64 as u32)
                                    as u64) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    25
                } else {
                    1
                };
            }
            27 => {
                values[12] = values[8];
                pc = 26;
            }
            28 => {
                values[6] = memory.read_u64((values[8]).wrapping_add(0x78u64))?;
                pc = 27;
            }
            29 => {
                pc = if (u64::from(
                    ((values[8]) as i64)
                        != (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    26
                };
            }
            30 => {
                values[12] = values[7];
                pc = 29;
            }
            31 => {
                values[6] = memory.read_u64(relocate("_DAT_00000078"))?;
                pc = 30;
            }
            32 => {
                values[8] = values[7];
                pc = 31;
            }
            33 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    32
                } else {
                    31
                };
            }
            34 => {
                values[8] =
                    (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
                pc = 33;
            }
            35 => {
                values[9] = u64::from((0x0u64) as u32);
                pc = 34;
            }
            36 => {
                values[7] = 0x0u64;
                pc = 35;
            }
            37 => {
                {
                    let call_arguments =
                        [values[3], 0x0u64, (values[2]).wrapping_shl(0x3u64 as u32)];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 36;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14001d3a0` (`mspaint.c:30136`).
pub fn fun_14001d3a0(
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
    ];
    let mut pc: i32 = 22;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = values[3];
                    let stored = u64::from(
                        (u64::from(
                            (u64::from((values[5]) as u32))
                                < (u64::from(
                                    (u64::from(
                                        (((values[6]).wrapping_sub(
                                            memory.read_u64((values[8]).wrapping_add(0x70u64))?,
                                        ) as i64)
                                            .wrapping_shr(0x3u64 as u32)
                                            as u64) as u32,
                                    )) as u32,
                                )),
                        )) != 0,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 0;
            }
            2 => {
                values[6] = memory.read_u64(relocate("_DAT_00000078"))?;
                pc = 1;
            }
            3 => {
                {
                    let destination = values[2];
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                values[5] = u64::from(
                    (u64::from(
                        (((values[6]).wrapping_sub(memory.read_u64(relocate("_DAT_00000070"))?))
                            .wrapping_shr(0x3u64 as u32)) as u32,
                    )) as u32,
                );
                pc = 3;
            }
            5 => {
                values[8] = values[7];
                pc = 1;
            }
            6 => {
                values[6] = memory.read_u64((values[7]).wrapping_add(0x78u64))?;
                pc = 5;
            }
            7 => {
                {
                    let destination = values[2];
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                values[5] = u64::from(
                    (u64::from(
                        (((values[6])
                            .wrapping_sub(memory.read_u64((values[7]).wrapping_add(0x70u64))?)
                            as i64)
                            .wrapping_shr(0x3u64 as u32) as u64) as u32,
                    )) as u32,
                );
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((values[7]) as i64)
                        == (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    8
                };
            }
            10 => {
                pc = if (u64::from(
                    ((u64::from(((values[6]) as i64) != ((values[9]) as i64))) != 0)
                        && (({
                            let _ = {
                                values[4] = ({
                                    let call_arguments = [relocate("STACK_local_res10"), values[6]];
                                    invoke("FUN_140001a20", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[4]
                            };
                            u64::from(
                                (((values[4]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    11
                } else {
                    9
                };
            }
            11 => {
                values[6] = (values[6]).wrapping_add(0x8u64);
                pc = 10;
            }
            12 => {
                {
                    let destination =
                        (relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            13 => {
                values[6] = memory.read_u64((values[6]).wrapping_add(0x70u64))?;
                pc = 12;
            }
            14 => {
                values[9] = memory.read_u64((values[7]).wrapping_add(0x78u64))?;
                pc = 13;
            }
            15 => {
                values[6] = values[7];
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    ((values[7]) as i64)
                        != (((0u64.wrapping_sub(0x50u64)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    15
                } else {
                    13
                };
            }
            17 => {
                values[9] = memory.read_u64(relocate("_DAT_00000078"))?;
                pc = 16;
            }
            18 => {
                values[6] = values[8];
                pc = 17;
            }
            19 => {
                values[7] = values[8];
                pc = 18;
            }
            20 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[7] =
                    (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
                pc = 20;
            }
            22 => {
                values[8] = 0x0u64;
                pc = 21;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140023ac0` (`mspaint.c:35168`).
pub fn fun_140023ac0(
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
        u64::from((arguments[1]) as u32),
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 15;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                return Ok(Some(0x1u64));
            }
            2 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 1;
            }
            3 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    2
                } else {
                    0
                };
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add(0x20u64);
                    let stored = u64::from((values[1]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[0]).wrapping_add(0x18u64);
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[3]];
                    invoke("SetLastError", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [values[2]];
                    invoke("GlobalFree", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                values[3] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("GetLastError", &call_arguments, memory)
                    }) as u32,
                );
                pc = 7;
            }
            9 => {
                pc = if (u64::from((values[2]) != (0x0u64))) != 0 {
                    8
                } else {
                    5
                };
            }
            10 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
                pc = 9;
            }
            11 => {
                values[4] = {
                    let call_arguments = [0x2u64, values[1]];
                    invoke("GlobalAlloc", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                values[4] = {
                    let call_arguments = [values[4], values[1], 0x0u64];
                    invoke("GlobalReAlloc", &call_arguments, memory)
                };
                pc = 10;
            }
            13 => {
                {
                    let destination = (values[0]).wrapping_add(0x18u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    11
                } else {
                    13
                };
            }
            15 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
                pc = 14;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140032df0` (`mspaint.c:47299`).
pub fn fun_140032df0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 47;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[0]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_238")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((values[1]) != (0x0u64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            6 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(
                                memory.read_u64(
                                    (relocate("STACK_local_238"))
                                        .wrapping_add((0x0u64).wrapping_mul(8)),
                                )?,
                            )?)
                            .wrapping_add(0x118u64),
                        )?,
                        memory.read_u64(
                            (relocate("STACK_local_238")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination =
                        (relocate("STACK_local_250")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0xf71u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            12 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                values[1] = memory.read_u64(
                    (relocate("STACK_local_238")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 12;
            }
            14 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0xa8u64))?,
                        values[0],
                        relocate("STACK_local_238"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination =
                        (relocate("STACK_local_250")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination =
                        (relocate("STACK_local_238")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b70"))?)?)
                                .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d0b70"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                values[0] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_258"))?];
                    invoke("FUN_1400b64c0", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_local_258"))?).wrapping_add(0x11u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_local_258"))?).wrapping_add(0x14u64);
                    let stored = u64::from((0x10002u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_258"))?;
                    let stored = relocate("FUN_1400b64c0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_258"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("FUN_1400b64b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = {
                        let call_arguments = [0x230u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [values[5]];
                    invoke("FUN_140032cf0", &call_arguments, memory)
                };
                pc = 22;
            }
            30 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0930"))?)) as u32))
                            as u32,
                    )) == (u64::from((0x1u64) as u32)),
                )) != 0
                {
                    28
                } else {
                    29
                };
            }
            31 => {
                {
                    let call_arguments = [relocate("STACK_local_238")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_238")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    31
                } else {
                    30
                };
            }
            33 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d06a0"),
                        relocate("STACK_local_238"),
                        values[3],
                    ];
                    invoke("FUN_1400512b0", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                values[3] = ({
                    let call_arguments = [memory.read_u64(relocate("DAT_1401d06e0"))?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    34
                } else {
                    0
                };
            }
            36 => {
                values[2] = ({
                    let call_arguments = [0xe125u64];
                    invoke("FUN_140007c00", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 35;
            }
            37 => {
                values[5] = 0x1u64;
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    36
                };
            }
            39 => {
                values[2] = ({
                    let call_arguments = [relocate("DAT_1401d0520"), relocate("STACK_local_228")];
                    invoke("FUN_140075d90", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64)
                            < (((values[3]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                let destination = relocate("_DAT_1401d07be");
                                let stored = u64::from(
                                    (((u64::from(
                                        (u64::from(memory.read_u16(relocate("_DAT_1401d07be"))?))
                                            as u16,
                                    ))
                                    .wrapping_add(0x1u64))
                                        as i32 as i64 as u64)
                                        as u16,
                                );
                                memory.write_u16(destination, stored as u16)?;
                                stored
                            };
                            u64::from(
                                ((((u64::from(
                                    memory.read_u16(
                                        (relocate("STACK_local_228"))
                                            .wrapping_add((0x0u64).wrapping_mul(2)),
                                    )?,
                                )) as i16 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    39
                } else {
                    36
                };
            }
            41 => {
                values[5] = values[4];
                pc = 40;
            }
            42 => {
                values[3] = ({
                    let call_arguments = [relocate("STACK_local_228")];
                    invoke("FUN_140010390", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [relocate("STACK_local_228"), 0x0u64, 0x1fau64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0930"))?)) as u32))
                            as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    43
                } else {
                    36
                };
            }
            45 => {
                values[5] = 0x0u64;
                pc = 44;
            }
            46 => {
                values[4] = 0x0u64;
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_278"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140047ae0` (`mspaint.c:66263`).
pub fn fun_140047ae0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 7;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d0ce8")));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ce0")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("FUN_1400df7d0")];
                    invoke("atexit", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("DAT_1401d0cf0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("_DAT_1401d0ce8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0ce0"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    4
                } else {
                    0
                };
            }
            6 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ce0")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
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
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0ce0"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    6
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14004cbe0` (`mspaint.c:70482`).
pub fn fun_14004cbe0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 11;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d0d30")));
            }
            1 => {
                return Ok(Some(relocate("DAT_1401d0d30")));
            }
            2 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d20")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [relocate("FUN_1400df800")];
                    invoke("atexit", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("_DAT_1401d0e20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("_DAT_1401d0e18");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("_DAT_1401d0d30");
                    let stored = relocate("CNarratorBridge::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d30")];
                    invoke("CWnd::CWnd", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d30"), 0x0u64, 0xf8u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0d20"))?)) as u32))
                            as u32,
                    )) == (u64::from(((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32)),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d20")];
                    invoke("FUN_1400b03b8", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
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
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_1401d0d20"))?)) as u32))
                            as u32,
                    )),
                )) != 0
                {
                    10
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400548c0` (`mspaint.c:76741`).
pub fn fun_1400548c0(
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
    ];
    let mut pc: i32 = 67;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [values[10], 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 0;
            }
            3 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    2
                } else {
                    0
                };
            }
            4 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_30"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_30"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 3;
            }
            6 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64))) != 0
                {
                    5
                } else {
                    3
                };
            }
            7 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 7;
            }
            9 => {
                return Ok(Some(values[1]));
            }
            10 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_30"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_30"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                return Ok(Some(values[1]));
            }
            12 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) == (0x0u64))) != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                {
                    let call_arguments = [values[5], 0x58u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_38"))?];
                    invoke("BasePaint::Image::~Image", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    14
                } else {
                    12
                };
            }
            16 => {
                {
                    let destination = values[1];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                values[5] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[12])?).wrapping_add(0x8u64))?,
                        values[12],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 19;
            }
            21 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                values[3] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 21;
            }
            23 => {
                values[2] = (values[12]).wrapping_add(0xcu64);
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_40"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_40"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    17
                };
            }
            27 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 26;
            }
            28 => {
                {
                    let destination = values[11];
                    let stored = ((((memory.read_u64(values[11])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                values[4] = memory.read_u64(values[11])?;
                pc = 28;
            }
            30 => {
                values[11] = (memory.read_u64(relocate("STACK_local_40"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 29;
            }
            31 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 30;
            }
            32 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_40"))?) != (0x0u64))) != 0
                {
                    31
                } else {
                    17
                };
            }
            33 => {
                values[12] = memory.read_u64(relocate("STACK_local_40"))?;
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_48")];
                    invoke("FUN_140017840", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = values[12];
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                values[12] = values[11];
                pc = 36;
            }
            38 => {
                {
                    let destination = (values[11]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_48"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = values[11];
                    let stored = relocate("std::_Ref_count_resource<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[11]).wrapping_add(0xcu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = (values[11]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (values[11]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = values[11];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    43
                } else {
                    36
                };
            }
            45 => {
                values[11] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    45
                } else {
                    8
                };
            }
            47 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_38"),
                        (values[0]).wrapping_add(0x18u64),
                    ];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) == (0x0u64))) != 0
                {
                    4
                } else {
                    49
                };
            }
            51 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        u64::from(
                            ((relocate("STACK_local_30")).wrapping_shr(
                                ((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64 as u32,
                            )) as u8,
                        ),
                    ];
                    invoke("SelectionTool::SelectedContent", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                values[10] = {
                    let call_arguments = [values[9]];
                    invoke("FUN_140014e60", &call_arguments, memory)
                };
                pc = 53;
            }
            55 => {
                {
                    let call_arguments = [values[9], 0x0u64, 0xa60u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    56
                } else {
                    53
                };
            }
            58 => {
                values[10] = values[12];
                pc = 57;
            }
            59 => {
                values[9] = {
                    let call_arguments = [0xa60u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                pc = if (values[7]) != 0 { 1 } else { 59 };
            }
            61 => {
                values[7] = u64::from(
                    ({
                        let call_arguments = [values[8]];
                        invoke("PGS::PGSRect::IsRectEmpty", &call_arguments, memory)
                    }) != 0,
                );
                pc = 60;
            }
            62 => {
                values[8] = {
                    let call_arguments = [];
                    invoke("PGS::PGSRect::Empty", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                values[8] = {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetTrackerRect", &call_arguments, memory)
                };
                pc = 61;
            }
            64 => {
                pc = if (u64::from(
                    ((u64::from(
                        (memory.read_u64((values[0]).wrapping_add(0x40u64))?) == (0x0u64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[6] = ({
                                    let call_arguments = [memory.read_u64(
                                        (memory.read_u64(
                                            memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                                        )?)
                                        .wrapping_add(0x68u64),
                                    )?];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[6]
                            };
                            u64::from(
                                (((values[6]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    62
                } else {
                    63
                };
            }
            65 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                values[12] = 0x0u64;
                pc = 65;
            }
            67 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14005dea0` (`mspaint.c:85491`).
pub fn fun_14005dea0(
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
    let mut pc: i32 = 13;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                return Ok(Some(0x1u64));
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
                        memory.read_u64(memory.read_u64(values[2])?)?,
                        values[2],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(((values[2]) as i64) != ((0x0u64) as i64))) != 0 {
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
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                values[2] = memory.read_u64(relocate("DAT_1401d0c78"))?;
                pc = 9;
            }
            11 => {
                values[3] = ({
                    let call_arguments = [values[0], values[1]];
                    invoke("COleServerDoc::CanCloseFrame", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    0
                };
            }
            13 => {
                values[3] = ({
                    let call_arguments = [];
                    invoke("FUN_140062050", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 12;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400678c0` (`mspaint.c:93590`).
pub fn fun_1400678c0(
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
    let mut pc: i32 = 19;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[4]];
                    invoke("RegCloseKey", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("DAT_1401d0b30"), values[4]];
                    invoke(
                        "CAttributeStore::SaveProfileSettings",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    2
                } else {
                    0
                };
            }
            4 => {
                values[4] = {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x20006u64];
                    invoke("FUN_14008b8e0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [memory.read_u64(
                        (relocate("STACK_local_238")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?];
                    invoke("RegCloseKey", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (relocate("STACK_local_238")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                        relocate("L\"WindowPlacement\""),
                        0x0u64,
                        0x3u64,
                        relocate("DAT_1401d08c4"),
                        0x2cu64,
                    ];
                    invoke("RegSetValueExW", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    4
                };
            }
            8 => {
                values[2] = ({
                    let call_arguments = [
                        relocate("DAT_ffffffff80000001"),
                        relocate("STACK_local_228"),
                        0x0u64,
                        0x0u64,
                        0x0u64,
                        0x2001fu64,
                        0x0u64,
                        relocate("STACK_local_238"),
                        0x0u64,
                    ];
                    invoke("RegCreateKeyExW", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    4
                };
            }
            10 => {
                values[1] = ({
                    let call_arguments = [relocate("STACK_local_228"), 0x104u64];
                    invoke("FUN_14008c9d0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [relocate("STACK_local_228"), 0x0u64, 0x208u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [values[3]];
                    invoke("FUN_14004cd30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                values[3] = {
                    let call_arguments = [];
                    invoke("FUN_14004cbe0", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [values[0], relocate("DAT_1401d08c4")];
                    invoke("CWnd::GetWindowPlacement", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("_DAT_1401d08c4");
                    let stored = u64::from((0x2cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1e0u64).wrapping_mul(1))];
                    invoke("FUN_1400a4350", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140067a30", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [values[0]];
                    invoke("CFrameWnd::OnDestroy", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_288"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14006e350` (`mspaint.c:98642`).
pub fn fun_14006e350(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    let mut pc: i32 = 11;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let destination = relocate("DAT_1401d0934");
                    let stored = (0x0u64) as i16 as i64 as u64;
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("_DAT_1401d0c00");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("DAT_1401d0930");
                    let stored = u64::from((0x5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    0
                };
            }
            5 => {
                values[1] = ({
                    let call_arguments = [];
                    invoke("FUN_140010a00", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 4;
            }
            6 => {
                return Ok(None);
            }
            7 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), relocate("STACK_local_228")];
                    invoke("FUN_14006f820", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (((values[0]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    5
                };
            }
            9 => {
                values[0] = ({
                    let call_arguments = [relocate("STACK_local_228")];
                    invoke("PathFileExistsW", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_228"),
                        memory.read_u64(relocate("DAT_1401d0900"))?,
                    ];
                    invoke("AfxFullPath", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_248"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140082d40` (`mspaint.c:114573`).
pub fn fun_140082d40(
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
    let mut values = [arguments[0], 0u64, 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 31;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0x8u64))?,
                        values[6],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 2;
            }
            4 => {
                return Ok(None);
            }
            5 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[2] = ({
                    let call_arguments = [];
                    invoke("FUN_140091b50", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 5;
            }
            7 => {
                values[2] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0x390u64))?,
                        values[6],
                        (values[1]).wrapping_add(0x48u64),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 5;
            }
            8 => {
                pc = if (u64::from(
                    (memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0x390u64))?)
                        == (relocate("FUN_140091b50")),
                )) != 0
                {
                    6
                } else {
                    7
                };
            }
            9 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    8
                } else {
                    3
                };
            }
            10 => {
                values[6] = values[5];
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[5]).wrapping_add((0x218u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[5]).wrapping_add((0x210u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[5]).wrapping_add((0x208u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[5]).wrapping_add((0x200u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[5]).wrapping_add((0x1f8u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[5]).wrapping_add((0x1f0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [(values[5]).wrapping_add((0x1e8u64).wrapping_mul(1))];
                    invoke("FUN_140093f80", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[5]).wrapping_add((0x1e0u64).wrapping_mul(1));
                    let stored = (values[4]).wrapping_add((0x1e0u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = (values[5]).wrapping_add((0x1d8u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = values[5];
                    let stored = relocate("CFullScreenThumbnailView::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [values[5]];
                    invoke("CFrameWnd::CFrameWnd", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                {
                    let call_arguments = [values[5], 0x0u64, 0x220u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    22
                } else {
                    9
                };
            }
            24 => {
                values[5] = {
                    let call_arguments = [0x220u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                values[4] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0xf8u64))?,
                        values[3],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    25
                } else {
                    24
                };
            }
            27 => {
                values[4] = values[6];
                pc = 26;
            }
            28 => {
                values[6] = 0x0u64;
                pc = 27;
            }
            29 => {
                values[3] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                values[1] = memory.read_u64((values[0]).wrapping_add(0xe8u64))?;
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [];
                    invoke("TraceFullScreen", &call_arguments, memory)
                };
                pc = 30;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b03b8` (`mspaint.c:154808`).
pub fn fun_1400b03b8(
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
    let values = [arguments[0]];
    let mut pc: i32 = 10;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                pc = if (0x1u64) != 0 { 9 } else { 0 };
            }
            2 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401cf460"),
                        relocate("DAT_1401cf468"),
                        0xffffffffu64,
                        0x0u64,
                    ];
                    invoke("SleepConditionVariableSRW", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = 7;
            }
            4 => {
                {
                    let destination = (memory.read_u64(
                        (memory.read_u64(relocate("ThreadLocalStoragePointer"))?).wrapping_add(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("_tls_index"))?)) as u32,
                            ))
                            .wrapping_mul(0x8u64),
                        ),
                    )?)
                    .wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(relocate("DAT_140179000"))?)) as u32))
                            as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(values[0])?)) as i32 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        != ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    2
                };
            }
            6 => {
                return Ok(None);
            }
            7 => {
                {
                    let call_arguments = [relocate("DAT_1401cf468")];
                    invoke("ReleaseSRWLockExclusive", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let destination = values[0];
                    let stored =
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(values[0])?)) as i32 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    5
                };
            }
            10 => {
                {
                    let call_arguments = [relocate("DAT_1401cf468")];
                    invoke("AcquireSRWLockExclusive", &call_arguments, memory)
                };
                pc = 9;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b0900` (`mspaint.c:155238`).
pub fn fun_1400b0900(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let destination = relocate("_DAT_1401cfa48");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b4860` (`mspaint.c:159621`).
pub fn fun_1400b4860(
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
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 210;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x110u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x7eu64))?)) as i16
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140008c50", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = 3;
            }
            5 => {
                pc = 3;
            }
            6 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_118"))?, 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    7
                } else {
                    5
                };
            }
            9 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    9
                } else {
                    8
                };
            }
            11 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                pc = if (u64::from(((values[5]) as i64) == ((0x1u64) as i64))) != 0 {
                    17
                } else {
                    11
                };
            }
            13 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 12;
            }
            14 => {
                {
                    let destination = values[6];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[5] = memory.read_u64(values[6])?;
                pc = 14;
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            19 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 18;
            }
            20 => {
                {
                    let destination = values[6];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                values[5] = memory.read_u64(values[6])?;
                pc = 20;
            }
            22 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 21;
            }
            23 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x13u64).wrapping_mul(8)))?;
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x13u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    23
                } else {
                    11
                };
            }
            25 => {
                pc = 3;
            }
            26 => {
                return Ok(None);
            }
            27 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    25
                };
            }
            28 => {
                values[11] = ({
                    let call_arguments = [];
                    invoke("FUN_14000c570", &call_arguments, memory)
                }) as i8 as i64 as u64;
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
                {
                    let destination = values[1];
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = (values[0]).wrapping_add((0x1du64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_118"))?, 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    33
                } else {
                    31
                };
            }
            35 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0x8u64))?,
                        values[7],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 36;
            }
            38 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                values[4] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 38;
            }
            40 => {
                values[2] = (values[7]).wrapping_add(0xcu64);
                pc = 39;
            }
            41 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[7])?)?, values[7]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    42
                } else {
                    34
                };
            }
            44 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 44;
            }
            46 => {
                {
                    let destination = values[6];
                    let stored = ((((memory.read_u64(values[6])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 45;
            }
            47 => {
                values[5] = memory.read_u64(values[6])?;
                pc = 46;
            }
            48 => {
                values[6] = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 47;
            }
            49 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 48;
            }
            50 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x18u64).wrapping_mul(8)))?;
                pc = 49;
            }
            51 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x18u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    50
                } else {
                    34
                };
            }
            52 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_118"))?, 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 31;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    54
                } else {
                    31
                };
            }
            56 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x10u64).wrapping_mul(8))];
                    invoke("FUN_1400328c0", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0x8u64))?,
                        values[7],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    57
                } else {
                    56
                };
            }
            59 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 58;
            }
            60 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 59;
            }
            61 => {
                values[4] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 60;
            }
            62 => {
                values[2] = (values[7]).wrapping_add(0xcu64);
                pc = 61;
            }
            63 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 62;
            }
            64 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[7])?)?, values[7]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    64
                } else {
                    56
                };
            }
            66 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 66;
            }
            68 => {
                {
                    let destination = values[6];
                    let stored = ((((memory.read_u64(values[6])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 67;
            }
            69 => {
                values[5] = memory.read_u64(values[6])?;
                pc = 68;
            }
            70 => {
                values[6] = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 69;
            }
            71 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 70;
            }
            72 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x18u64).wrapping_mul(8)))?;
                pc = 71;
            }
            73 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x18u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    72
                } else {
                    56
                };
            }
            74 => {
                {
                    let call_arguments = [values[8], 0x58u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_88"))?];
                    invoke("BasePaint::Image::~Image", &call_arguments, memory)
                };
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                values[8] = memory.read_u64(values[3])?;
                pc = 76;
            }
            78 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    77
                } else {
                    73
                };
            }
            79 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    81
                } else {
                    79
                };
            }
            83 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 82;
            }
            84 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 83;
            }
            85 => {
                values[4] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 84;
            }
            86 => {
                values[2] = (memory.read_u64(relocate("STACK_local_f8"))?).wrapping_add(0xcu64);
                pc = 85;
            }
            87 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?,
                    )?)?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    90
                } else {
                    79
                };
            }
            92 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 91;
            }
            93 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 92;
            }
            94 => {
                values[4] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 93;
            }
            95 => {
                values[2] = (memory.read_u64(relocate("STACK_local_f8"))?).wrapping_add(0x8u64);
                pc = 94;
            }
            96 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    97
                } else {
                    79
                };
            }
            99 => {
                {
                    let call_arguments = [
                        values[8],
                        (values[0]).wrapping_add((0x1bu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140017840", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                values[17] = values[16];
                pc = 102;
            }
            104 => {
                {
                    let destination = (values[16]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = values[16];
                    let stored = relocate("std::_Ref_count_resource<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[16]).wrapping_add(0xcu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 105;
            }
            107 => {
                {
                    let destination = (values[16]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 106;
            }
            108 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    107
                } else {
                    102
                };
            }
            109 => {
                values[16] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                values[9] = memory.read_u64(values[3])?;
                pc = 110;
            }
            112 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                values[8] = memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                pc = 113;
            }
            115 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) == (0x0u64))) != 0
                {
                    52
                } else {
                    114
                };
            }
            116 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x1au64).wrapping_mul(8)))?,
                        )?)?,
                        memory.read_u64(relocate("STACK_local_e8"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1au64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1au64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    118
                } else {
                    116
                };
            }
            120 => {
                {
                    let call_arguments = [values[3], relocate("DAT_1401d06b8")];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 119;
            }
            121 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_d8"))?];
                    invoke("BasePaint::Image::Bitmap", &call_arguments, memory)
                };
                pc = 120;
            }
            122 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 122;
            }
            124 => {
                values[3] = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
                pc = 123;
            }
            125 => {
                {
                    let call_arguments = [values[5], 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 124;
            }
            126 => {
                pc = if (u64::from(((values[5]) as i64) != ((0x0u64) as i64))) != 0 {
                    125
                } else {
                    124
                };
            }
            127 => {
                {
                    let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                pc = 127;
            }
            129 => {
                values[16] = {
                    let call_arguments = [values[15]];
                    invoke("FUN_140014e60", &call_arguments, memory)
                };
                pc = 128;
            }
            130 => {
                {
                    let call_arguments = [values[15], 0x0u64, 0xa60u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 129;
            }
            131 => {
                pc = if (u64::from((values[15]) != (0x0u64))) != 0 {
                    130
                } else {
                    128
                };
            }
            132 => {
                values[16] = values[17];
                pc = 131;
            }
            133 => {
                {
                    let destination = (values[0]).wrapping_add((0x21u64).wrapping_mul(8));
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 132;
            }
            134 => {
                values[15] = {
                    let call_arguments = [0xa60u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 133;
            }
            135 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?)
                            .wrapping_add(0x298u64),
                    )?];
                    invoke("FUN_140015600", &call_arguments, memory)
                };
                pc = 134;
            }
            136 => {
                {
                    let destination = (values[0]).wrapping_add((0x18u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 137;
            }
            139 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    138
                } else {
                    137
                };
            }
            140 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            141 => {
                pc = if (u64::from(((values[5]) as i64) == ((0x1u64) as i64))) != 0 {
                    146
                } else {
                    140
                };
            }
            142 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 141;
            }
            143 => {
                {
                    let destination = values[6];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 142;
            }
            144 => {
                values[5] = memory.read_u64(values[6])?;
                pc = 143;
            }
            145 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 144;
            }
            146 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 141;
            }
            148 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 147;
            }
            149 => {
                {
                    let destination = values[6];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                values[5] = memory.read_u64(values[6])?;
                pc = 149;
            }
            151 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 150;
            }
            152 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x13u64).wrapping_mul(8)))?;
                pc = 151;
            }
            153 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x13u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    152
                } else {
                    140
                };
            }
            154 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x13u64).wrapping_mul(8))];
                    invoke("FUN_14000c240", &call_arguments, memory)
                };
                pc = 153;
            }
            155 => {
                values[17] = 0x0u64;
                pc = 154;
            }
            156 => {
                pc = 3;
            }
            157 => {
                pc = 31;
            }
            158 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_118"))?, 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 157;
            }
            159 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = memory.read_u64(values[6])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 158;
            }
            160 => {
                pc = if (u64::from(((memory.read_u64(values[6])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    159
                } else {
                    157
                };
            }
            161 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_118"))?, 0x1u64];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 157;
            }
            162 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = memory.read_u64(values[6])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 161;
            }
            163 => {
                pc = if (u64::from(((memory.read_u64(values[6])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    162
                } else {
                    157
                };
            }
            164 => {
                pc = 155;
            }
            165 => {
                return Ok(None);
            }
            166 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    165
                } else {
                    164
                };
            }
            167 => {
                values[11] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add((0x13u64).wrapping_mul(8)),
                        values[0],
                    ];
                    invoke("FUN_14000c540", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 166;
            }
            168 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let destination = (values[0]).wrapping_add((0x16u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 168;
            }
            170 => {
                {
                    let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 169;
            }
            171 => {
                {
                    let destination = (values[0]).wrapping_add((0x14u64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x12u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                {
                    let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 171;
            }
            173 => {
                {
                    let call_arguments = [relocate("STACK_local_c0"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            174 => {
                {
                    let call_arguments = [relocate("STACK_local_c0")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 173;
            }
            175 => {
                {
                    let destination = (values[0]).wrapping_add((0x20u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 174;
            }
            176 => {
                {
                    let destination = (values[0]).wrapping_add((0x1fu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let destination = (values[0]).wrapping_add((0x1eu64).wrapping_mul(8));
                    let stored = u64::from((0x1628u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 176;
            }
            178 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    172
                };
            }
            179 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d06a0"),
                        (values[0]).wrapping_add((0x12u64).wrapping_mul(8)),
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x0u64,
                    ];
                    invoke("FUN_140051600", &call_arguments, memory)
                };
                pc = 178;
            }
            180 => {
                pc = if (u64::from(((values[5]) as i64) != ((0x0u64) as i64))) != 0 {
                    179
                } else {
                    163
                };
            }
            181 => {
                {
                    let destination = (values[0]).wrapping_add((0x11u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 180;
            }
            182 => {
                values[5] =
                    memory.read_u64((values[14]).wrapping_add((0xe8u64).wrapping_mul(1)))?;
                pc = 181;
            }
            183 => {
                pc = if (u64::from((values[14]) == (0x0u64))) != 0 {
                    160
                } else {
                    182
                };
            }
            184 => {
                values[14] = {
                    let call_arguments = [values[13]];
                    invoke("CFrameWnd::GetActiveView", &call_arguments, memory)
                };
                pc = 183;
            }
            185 => {
                values[13] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[12])?).wrapping_add(0xf8u64))?,
                        values[12],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 184;
            }
            186 => {
                pc = if (u64::from((values[12]) != (0x0u64))) != 0 {
                    185
                } else {
                    184
                };
            }
            187 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 186;
            }
            188 => {
                pc = 56;
            }
            189 => {
                pc = if (u64::from(((memory.read_u64(values[6])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    188
                } else {
                    187
                };
            }
            190 => {
                {
                    let call_arguments = [relocate("DAT_1401d06a0"), values[6]];
                    invoke("FUN_1400548c0", &call_arguments, memory)
                };
                pc = 189;
            }
            191 => {
                {
                    let destination = values[6];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 190;
            }
            192 => {
                values[6] = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
                pc = 191;
            }
            193 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    192
                } else {
                    157
                };
            }
            194 => {
                values[11] = ({
                    let call_arguments = [0xe122u64];
                    invoke("FUN_140007c00", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 193;
            }
            195 => {
                values[13] = 0x0u64;
                pc = 194;
            }
            196 => {
                return Ok(None);
            }
            197 => {
                {
                    let call_arguments = [values[10]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 196;
            }
            198 => {
                values[10] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 197;
            }
            199 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    198
                };
            }
            200 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    199
                };
            }
            201 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    24
                } else {
                    200
                };
            }
            202 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    155
                } else {
                    201
                };
            }
            203 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    156
                } else {
                    202
                };
            }
            204 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    195
                } else {
                    203
                };
            }
            205 => {
                values[18] =
                    (u64::from((u64::from(memory.read_u16(relocate("STACK_local_110"))?)) as u16))
                        as i32 as i64 as u64;
                pc = 204;
            }
            206 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 205;
            }
            207 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 206;
            }
            208 => {
                {
                    let destination = relocate("STACK_local_110");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u16(values[1])?)) as u16)) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 207;
            }
            209 => {
                values[1] = (values[0]).wrapping_add(0x7cu64);
                pc = 208;
            }
            210 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_138"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 209;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b51c0` (`mspaint.c:160066`).
pub fn fun_1400b51c0(
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
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 193;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x150u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x7eu64))?)) as i16
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140008c50", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = 3;
            }
            5 => {
                pc = 3;
            }
            6 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_100"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                pc = if (u64::from(((values[5]) as i64) == ((0x1u64) as i64))) != 0 {
                    14
                } else {
                    8
                };
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[13];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                values[5] = memory.read_u64(values[13])?;
                pc = 11;
            }
            13 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                {
                    let destination = values[13];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                values[5] = memory.read_u64(values[13])?;
                pc = 17;
            }
            19 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 18;
            }
            20 => {
                values[13] =
                    memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?;
                pc = 19;
            }
            21 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    20
                } else {
                    8
                };
            }
            22 => {
                pc = 3;
            }
            23 => {
                return Ok(None);
            }
            24 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    23
                } else {
                    22
                };
            }
            25 => {
                values[9] = ({
                    let call_arguments = [];
                    invoke("FUN_14000c570", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 24;
            }
            26 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_f0"))?;
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = (values[0]).wrapping_add((0x1du64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x16u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_a0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    32
                } else {
                    31
                };
            }
            34 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[6])?)?, values[6]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    35
                } else {
                    34
                };
            }
            37 => {
                {
                    let call_arguments = [values[15]];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                pc = if (u64::from((values[15]) != (0x0u64))) != 0 {
                    37
                } else {
                    36
                };
            }
            39 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x16u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 28;
            }
            41 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_a0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    40
                } else {
                    28
                };
            }
            42 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    43
                } else {
                    42
                };
            }
            45 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[6])?)?, values[6]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    46
                } else {
                    45
                };
            }
            48 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140009e00", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x19u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x19u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    49
                } else {
                    47
                };
            }
            51 => {
                {
                    let call_arguments = [values[7]];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_90"))?];
                    invoke("BasePaint::Image::~Image", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(values[4])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                values[7] = memory.read_u64(values[4])?;
                pc = 53;
            }
            55 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    54
                } else {
                    50
                };
            }
            56 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(values[4])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x19u64).wrapping_mul(8))];
                    invoke("FUN_1400328c0", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    59
                } else {
                    57
                };
            }
            61 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 60;
            }
            62 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 61;
            }
            63 => {
                values[10] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 62;
            }
            64 => {
                values[2] = (memory.read_u64(relocate("STACK_local_108"))?).wrapping_add(0xcu64);
                pc = 63;
            }
            65 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?,
                    )?)?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    68
                } else {
                    57
                };
            }
            70 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 69;
            }
            71 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                values[10] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 71;
            }
            73 => {
                values[2] = (memory.read_u64(relocate("STACK_local_108"))?).wrapping_add(0x8u64);
                pc = 72;
            }
            74 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1cu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    75
                } else {
                    57
                };
            }
            77 => {
                {
                    let call_arguments = [values[15]];
                    invoke("FUN_140017840", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_120"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                values[17] = values[16];
                pc = 80;
            }
            82 => {
                {
                    let destination = (values[16]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = memory.read_u64(relocate("STACK_local_120"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = values[16];
                    let stored = relocate("std::_Ref_count_resource<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = (values[16]).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[16]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 84;
            }
            86 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    85
                } else {
                    80
                };
            }
            87 => {
                values[16] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(relocate("STACK_local_120"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = memory.read_u64(values[4])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 90;
            }
            92 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) == (0x0u64))) != 0
                {
                    39
                } else {
                    91
                };
            }
            93 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(values[4])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let call_arguments = [values[4], relocate("DAT_1401d06b8")];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 93;
            }
            95 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                values[4] = (values[0]).wrapping_add((0x1au64).wrapping_mul(8));
                pc = 95;
            }
            97 => {
                {
                    let destination = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                values[15] = {
                    let call_arguments = [values[14]];
                    invoke("FUN_140014e60", &call_arguments, memory)
                };
                pc = 97;
            }
            99 => {
                {
                    let call_arguments = [values[14], 0x0u64, 0xa60u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    99
                } else {
                    97
                };
            }
            101 => {
                values[15] = values[17];
                pc = 100;
            }
            102 => {
                {
                    let destination = (values[0]).wrapping_add((0x29u64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                values[14] = {
                    let call_arguments = [0xa60u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 102;
            }
            104 => {
                {
                    let destination = (values[0]).wrapping_add((0x18u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = values[13];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            106 => {
                values[6] = memory.read_u64(values[13])?;
                pc = 105;
            }
            107 => {
                values[13] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[15])?).wrapping_add(0x18u64))?
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 106;
            }
            108 => {
                {
                    let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(8));
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                values[15] = memory.read_u64(relocate("STACK_local_98"))?;
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x25u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            112 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    111
                } else {
                    110
                };
            }
            113 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(memory.read_u64(values[3])?)?)?,
                        memory.read_u64(relocate("STACK_local_a0"))?,
                        relocate("DAT_140107fc0"),
                        (values[0]).wrapping_add((0x28u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = (values[0]).wrapping_add((0x26u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination = (values[0]).wrapping_add((0x25u64).wrapping_mul(8));
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = (values[0]).wrapping_add((0x28u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                pc = if (u64::from(((memory.read_u64(values[3])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    118
                } else {
                    108
                };
            }
            120 => {
                values[15] = values[17];
                pc = 119;
            }
            121 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x15u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 120;
            }
            122 => {
                pc = if (u64::from((values[13]) != (0x0u64))) != 0 {
                    121
                } else {
                    120
                };
            }
            123 => {
                {
                    let destination = (values[0]).wrapping_add((0x16u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x22u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            125 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    124
                } else {
                    123
                };
            }
            126 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[13])?).wrapping_add(0x98u64))?,
                        values[13],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 125;
            }
            127 => {
                {
                    let destination = (values[0]).wrapping_add((0x24u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                {
                    let destination = (values[0]).wrapping_add((0x23u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 127;
            }
            129 => {
                {
                    let destination = (values[0]).wrapping_add((0x22u64).wrapping_mul(8));
                    let stored = u64::from((0x455bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 128;
            }
            130 => {
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 129;
            }
            131 => {
                values[3] = (values[0]).wrapping_add((0x21u64).wrapping_mul(8));
                pc = 130;
            }
            132 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[13])?).wrapping_add(0x8u64))?,
                        values[13],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 131;
            }
            133 => {
                pc = if (u64::from((values[13]) != (0x0u64))) != 0 {
                    132
                } else {
                    131
                };
            }
            134 => {
                {
                    let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(8));
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                values[13] = memory.read_u64(
                    (memory.read_u64((values[15]).wrapping_add((0x298u64).wrapping_mul(1)))?)
                        .wrapping_add(0x998u64),
                )?;
                pc = 134;
            }
            136 => {
                values[15] =
                    memory.read_u64((values[12]).wrapping_add((0xe8u64).wrapping_mul(1)))?;
                pc = 135;
            }
            137 => {
                pc = if (u64::from((values[12]) != (0x0u64))) != 0 {
                    136
                } else {
                    135
                };
            }
            138 => {
                values[15] = values[17];
                pc = 137;
            }
            139 => {
                values[12] = {
                    let call_arguments = [values[15]];
                    invoke("CFrameWnd::GetActiveView", &call_arguments, memory)
                };
                pc = 138;
            }
            140 => {
                values[15] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[11])?).wrapping_add(0xf8u64))?,
                        values[11],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 139;
            }
            141 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    140
                } else {
                    139
                };
            }
            142 => {
                values[15] = values[17];
                pc = 141;
            }
            143 => {
                values[11] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 142;
            }
            144 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 143;
            }
            145 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_100"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    144
                } else {
                    143
                };
            }
            146 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 145;
            }
            147 => {
                pc = if (u64::from(((values[5]) as i64) == ((0x1u64) as i64))) != 0 {
                    152
                } else {
                    146
                };
            }
            148 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 147;
            }
            149 => {
                {
                    let destination = values[13];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                values[5] = memory.read_u64(values[13])?;
                pc = 149;
            }
            151 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 150;
            }
            152 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 151;
            }
            153 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 147;
            }
            154 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 153;
            }
            155 => {
                {
                    let destination = values[13];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 154;
            }
            156 => {
                values[5] = memory.read_u64(values[13])?;
                pc = 155;
            }
            157 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 156;
            }
            158 => {
                values[13] =
                    memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?;
                pc = 157;
            }
            159 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    158
                } else {
                    146
                };
            }
            160 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x11u64).wrapping_mul(8))];
                    invoke("FUN_14000c240", &call_arguments, memory)
                };
                pc = 159;
            }
            161 => {
                values[17] = 0x0u64;
                pc = 160;
            }
            162 => {
                pc = 3;
            }
            163 => {
                pc = 161;
            }
            164 => {
                return Ok(None);
            }
            165 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    164
                } else {
                    163
                };
            }
            166 => {
                values[9] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add((0x11u64).wrapping_mul(8)),
                        values[0],
                    ];
                    invoke("FUN_14000c540", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 165;
            }
            167 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 166;
            }
            168 => {
                {
                    let destination = (values[0]).wrapping_add((0x14u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                {
                    let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 169;
            }
            171 => {
                {
                    let destination = (values[0]).wrapping_add((0x11u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                {
                    let call_arguments = [relocate("STACK_local_d8"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            173 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 172;
            }
            174 => {
                {
                    let destination = (values[0]).wrapping_add((0x20u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                {
                    let destination = (values[0]).wrapping_add((0x1fu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 174;
            }
            176 => {
                {
                    let destination = (values[0]).wrapping_add((0x1eu64).wrapping_mul(8));
                    let stored = u64::from((0x1628u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 175;
            }
            177 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    176
                } else {
                    171
                };
            }
            178 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d06a0"),
                        (values[0]).wrapping_add((0x10u64).wrapping_mul(8)),
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x0u64,
                    ];
                    invoke("FUN_140051600", &call_arguments, memory)
                };
                pc = 177;
            }
            179 => {
                return Ok(None);
            }
            180 => {
                {
                    let call_arguments = [values[8]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 179;
            }
            181 => {
                values[8] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 180;
            }
            182 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    181
                };
            }
            183 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    182
                };
            }
            184 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    183
                };
            }
            185 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    161
                } else {
                    184
                };
            }
            186 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    162
                } else {
                    185
                };
            }
            187 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    178
                } else {
                    186
                };
            }
            188 => {
                values[18] =
                    (u64::from((u64::from(memory.read_u16(relocate("STACK_local_128"))?)) as u16))
                        as i32 as i64 as u64;
                pc = 187;
            }
            189 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 188;
            }
            190 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 189;
            }
            191 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u16(values[1])?)) as u16)) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 190;
            }
            192 => {
                values[1] = (values[0]).wrapping_add(0x7cu64);
                pc = 191;
            }
            193 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_148"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 192;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b64c0` (`mspaint.c:160938`).
pub fn fun_1400b64c0(
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
    ];
    let mut pc: i32 = 109;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x230u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x16u64))?)) as i16
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                pc = 2;
            }
            4 => {
                pc = 2;
            }
            5 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x18u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                pc = if (u64::from(((values[8]) as i64) == ((0x1u64) as i64))) != 0 {
                    12
                } else {
                    6
                };
            }
            8 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 7;
            }
            9 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[8] = memory.read_u64(values[4])?;
                pc = 9;
            }
            11 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 7;
            }
            14 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[8] = memory.read_u64(values[4])?;
                pc = 14;
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x20u64))?;
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x20u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    17
                } else {
                    6
                };
            }
            19 => {
                pc = 2;
            }
            20 => {
                {
                    let call_arguments = [0x0u64];
                    invoke("FUN_140032cf0", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("_DAT_1401d07bc");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d07bc"))?)) as u16,
                        ))
                        .wrapping_add(0x1u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 20;
            }
            22 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    19
                };
            }
            23 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x18u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    23
                } else {
                    22
                };
            }
            25 => {
                pc = if (u64::from(((values[8]) as i64) == ((0x1u64) as i64))) != 0 {
                    30
                } else {
                    24
                };
            }
            26 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 25;
            }
            27 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                values[8] = memory.read_u64(values[4])?;
                pc = 27;
            }
            29 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 25;
            }
            32 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                values[8] = memory.read_u64(values[4])?;
                pc = 32;
            }
            34 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 33;
            }
            35 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x20u64))?;
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x20u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    35
                } else {
                    24
                };
            }
            37 => {
                values[6] = ({
                    let call_arguments = [(values[0]).wrapping_add(0x20u64)];
                    invoke("FUN_140008b20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 36;
            }
            38 => {
                pc = 2;
            }
            39 => {
                pc = 37;
            }
            40 => {
                return Ok(None);
            }
            41 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    39
                };
            }
            42 => {
                values[6] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add(0x20u64),
                        values[0],
                        values[2],
                        values[3],
                        values[10],
                    ];
                    invoke("thunk_FUN_140009ba0", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 41;
            }
            43 => {
                {
                    let destination = (values[0]).wrapping_add(0x14u64);
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (values[0]).wrapping_add(0x38u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[0]).wrapping_add(0x30u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = (values[0]).wrapping_add(0x28u64);
                    let stored = (values[0]).wrapping_add(0x18u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = (values[0]).wrapping_add(0x20u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400b5ec0", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                values[10] = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
                pc = 48;
            }
            50 => {
                {
                    let destination =
                        (memory.read_u64((values[0]).wrapping_add(0x40u64))?).wrapping_add(0x80u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    51
                } else {
                    50
                };
            }
            53 => {
                {
                    let destination = (values[0]).wrapping_add(0x18u64);
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[8]).wrapping_add(0x78u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[8]).wrapping_add(0x10u64);
                    let stored = relocate("std::coroutine_traits<>::promise_type::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = (values[8]).wrapping_add(0x74u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = (values[8]).wrapping_add(0x70u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = (values[8]).wrapping_add(0x68u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = (values[8]).wrapping_add(0x60u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = (values[8]).wrapping_add(0x58u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let call_arguments = [(values[8]).wrapping_add(0x48u64)];
                    invoke("__ExceptionPtrCreate", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                {
                    let destination = (values[8]).wrapping_add(0x50u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (values[8]).wrapping_add(0x48u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = (values[8]).wrapping_add(0x10u64);
                    let stored = relocate("winrt::impl::promise_base<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = (values[8]).wrapping_add(0x40u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = (values[8]).wrapping_add(0x38u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = (values[8]).wrapping_add(0x30u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = (values[8]).wrapping_add(0x18u64);
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 69;
            }
            71 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 70;
            }
            72 => {
                {
                    let destination = (values[8]).wrapping_add(0x28u64);
                    let stored = relocate("winrt::impl::produce<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = values[4];
                    let stored = relocate("winrt::impl::produce<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                values[4] = (values[8]).wrapping_add(0x20u64);
                pc = 73;
            }
            75 => {
                values[8] = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
                pc = 74;
            }
            76 => {
                {
                    let destination = (values[8]).wrapping_add(0x78u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = (values[8]).wrapping_add(0x70u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = (values[8]).wrapping_add(0x68u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let destination = (values[8]).wrapping_add(0x60u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = (values[8]).wrapping_add(0x58u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = (values[8]).wrapping_add(0x50u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = (values[8]).wrapping_add(0x48u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = (values[8]).wrapping_add(0x40u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = (values[8]).wrapping_add(0x38u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[8]).wrapping_add(0x30u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = (values[8]).wrapping_add(0x28u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = (values[8]).wrapping_add(0x20u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (values[8]).wrapping_add(0x18u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = (values[8]).wrapping_add(0x10u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                values[8] = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
                pc = 89;
            }
            91 => {
                {
                    let destination =
                        (memory.read_u64((values[0]).wrapping_add(0x40u64))?).wrapping_add(0x90u64);
                    let stored = u64::from((0x10002u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination =
                        (memory.read_u64((values[0]).wrapping_add(0x40u64))?).wrapping_add(0x88u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
                    let stored = relocate("FUN_1400b5ec0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = (values[9]).wrapping_add(0x8u64);
                    let stored = relocate("FUN_1400b5eb0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = (values[0]).wrapping_add(0x40u64);
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                values[9] = {
                    let call_arguments = [0x1c0u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 95;
            }
            97 => {
                values[10] = 0x0u64;
                pc = 96;
            }
            98 => {
                values[10] = memory.read_u64((values[8]).wrapping_add(0x40u64))?;
                pc = 96;
            }
            99 => {
                pc = if (u64::from(
                    ((u64::from((values[7]) == (0x0u64))) != 0)
                        || (({
                            let _ = {
                                values[8] = {
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(values[7])?).wrapping_add(0xf8u64),
                                        )?,
                                        values[7],
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                };
                                values[8]
                            };
                            u64::from(((values[8]) as i64) == ((0x0u64) as i64))
                        }) != 0),
                )) != 0
                {
                    97
                } else {
                    98
                };
            }
            100 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 99;
            }
            101 => {
                return Ok(None);
            }
            102 => {
                {
                    let call_arguments = [values[5]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 101;
            }
            103 => {
                values[5] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 102;
            }
            104 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    103
                };
            }
            105 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    104
                };
            }
            106 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    105
                };
            }
            107 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    106
                };
            }
            108 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    100
                } else {
                    107
                };
            }
            109 => {
                values[11] = (u64::from(
                    (u64::from(memory.read_u16((values[0]).wrapping_add(0x14u64))?)) as u16,
                )) as i32 as i64 as u64;
                pc = 108;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400d5e9f` (`mspaint.c:180516`).
pub fn fun_1400d5e9f(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    let mut pc: i32 = 8;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x19u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [(values[1]).wrapping_add(0x90u64)];
                    invoke("FUN_140002130", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [
                        values[2],
                        (values[1]).wrapping_add(0x90u64),
                        (values[1]).wrapping_add(0xa8u64),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendFailureEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [
                        (values[1]).wrapping_add(0x90u64),
                        (values[1]).wrapping_add(0xd8u64),
                    ];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[1]).wrapping_add(0xe0u64);
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[1]).wrapping_add(0xd8u64);
                    let stored = relocate("L\"CopyWICMetadata\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = (values[1]).wrapping_add(0xa8u64);
                    let stored = u64::from((0x8007000eu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                values[2] = {
                    let call_arguments = [];
                    invoke(
                        "Paint::Telemetry::TraceLogger::GetInstance",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 7;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400df800` (`mspaint.c:187078`).
pub fn fun_1400df800(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut pc: i32 = 3;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d30")];
                    invoke("CWnd::~CWnd", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d30")];
                    invoke("FUN_14004cd30", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("_DAT_1401d0d30");
                    let stored = relocate("CNarratorBridge::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400df970` (`mspaint.c:187157`).
pub fn fun_1400df970(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    let mut pc: i32 = 19;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                return Ok(None);
            }
            2 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("DAT_1401d0c78"))?)?,
                        )?,
                        memory.read_u64(relocate("DAT_1401d0c78"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0c78"))?) != (0x0u64))) != 0
                {
                    2
                } else {
                    0
                };
            }
            4 => {
                {
                    let call_arguments = [values[1], 0x10u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("DAT_1401d0c80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                values[1] = memory.read_u64(relocate("DAT_1401d0c80"))?;
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [relocate("DAT_1401d0c80")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [values[1], 0x10u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("DAT_1401d0ca8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[1] = memory.read_u64(relocate("DAT_1401d0ca8"))?;
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ca8")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("DAT_1401d0cd0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ca8")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [relocate("DAT_1401d0c80")];
                    invoke("FUN_140094a60", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[0])?)?,
                        values[0],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                pc = if (u64::from(((values[0]) as i64) != ((0x0u64) as i64))) != 0 {
                    15
                } else {
                    14
                };
            }
            17 => {
                {
                    let destination = relocate("DAT_1401d0c78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("_DAT_1401d0c70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[0] = memory.read_u64(relocate("DAT_1401d0c78"))?;
                pc = 18;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L154789` (`mspaint.c:154789`).
/// Original symbol: `_Init_thread_footer`.
pub fn named_l154789(
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
    let mut values = [arguments[0], 0u64];
    let mut pc: i32 = 7;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401cf460")];
                    invoke("WakeAllConditionVariable", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("DAT_1401cf468")];
                    invoke("ReleaseSRWLockExclusive", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let destination = (memory.read_u64(
                        (memory.read_u64(relocate("ThreadLocalStoragePointer"))?)
                            .wrapping_add((values[1]).wrapping_mul(0x8u64)),
                    )?)
                    .wrapping_add(0x4u64);
                    let stored = (u64::from(
                        (u64::from(memory.read_u32(relocate("DAT_140179000"))?)) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = values[0];
                    let stored = (u64::from(
                        (u64::from(memory.read_u32(relocate("DAT_140179000"))?)) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("DAT_140179000");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("DAT_140179000"))?)) as u32,
                            ))
                            .wrapping_add(0x1u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                values[1] = u64::from((u64::from(memory.read_u32(relocate("_tls_index"))?)) as u32);
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [relocate("DAT_1401cf468")];
                    invoke("AcquireSRWLockExclusive", &call_arguments, memory)
                };
                pc = 6;
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
    ("FUN_140001290", 0, fun_140001290),
    ("FUN_14000c380", 1, fun_14000c380),
    ("FUN_1400122c0", 1, fun_1400122c0),
    ("FUN_1400122e0", 1, fun_1400122e0),
    ("FUN_140012460", 1, fun_140012460),
    ("FUN_140012480", 1, fun_140012480),
    ("FUN_140015d70", 3, fun_140015d70),
    ("FUN_14001cd30", 5, fun_14001cd30),
    ("FUN_14001ce50", 4, fun_14001ce50),
    ("FUN_14001d0d0", 5, fun_14001d0d0),
    ("FUN_14001d3a0", 4, fun_14001d3a0),
    ("FUN_140023ac0", 2, fun_140023ac0),
    ("FUN_140032df0", 0, fun_140032df0),
    ("FUN_140047ae0", 0, fun_140047ae0),
    ("FUN_14004cbe0", 0, fun_14004cbe0),
    ("FUN_1400548c0", 2, fun_1400548c0),
    ("FUN_14005dea0", 2, fun_14005dea0),
    ("FUN_1400678c0", 1, fun_1400678c0),
    ("FUN_14006e350", 0, fun_14006e350),
    ("FUN_140082d40", 1, fun_140082d40),
    ("FUN_1400b03b8", 1, fun_1400b03b8),
    ("FUN_1400b0900", 0, fun_1400b0900),
    ("FUN_1400b4860", 1, fun_1400b4860),
    ("FUN_1400b51c0", 1, fun_1400b51c0),
    ("FUN_1400b64c0", 4, fun_1400b64c0),
    ("FUN_1400d5e9f", 2, fun_1400d5e9f),
    ("FUN_1400df800", 0, fun_1400df800),
    ("FUN_1400df970", 0, fun_1400df970),
    ("NAMED_L154789", 1, named_l154789),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_14000c380",
        &[
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140015d70",
        &[
            ("STACK_auStack_118", -280),
            ("STACK_local_f8", -248),
            ("STACK_local_e8", -232),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_uStack_88", -136),
            ("STACK_local_80", -128),
            ("STACK_uStack_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_56", -86),
            ("STACK_local_50", -80),
            ("STACK_uStack_48", -72),
            ("STACK_local_40", -64),
        ],
    ),
    ("FUN_14001ce50", &[("STACK_local_res10", 16)]),
    ("FUN_14001d3a0", &[("STACK_local_res10", 16)]),
    (
        "FUN_140032df0",
        &[
            ("STACK_auStack_278", -632),
            ("STACK_local_258", -600),
            ("STACK_local_250", -592),
            ("STACK_local_248", -584),
            ("STACK_local_240", -576),
            ("STACK_local_238", -568),
            ("STACK_local_228", -552),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400548c0",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400678c0",
        &[
            ("STACK_auStackY_288", -648),
            ("STACK_local_238", -568),
            ("STACK_local_228", -552),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14006e350",
        &[
            ("STACK_auStack_248", -584),
            ("STACK_local_228", -552),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400b4860",
        &[
            ("STACK_auStack_138", -312),
            ("STACK_local_118", -280),
            ("STACK_local_110", -272),
            ("STACK_local_108", -264),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
            ("STACK_local_e8", -232),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c0", -192),
            ("STACK_local_a8", -168),
            ("STACK_local_88", -136),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_1400b51c0",
        &[
            ("STACK_auStack_148", -328),
            ("STACK_local_128", -296),
            ("STACK_local_120", -288),
            ("STACK_local_118", -280),
            ("STACK_local_108", -264),
            ("STACK_local_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_c0", -192),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_30", -48),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_14000c380",
        &[
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140015d70",
        &[
            ("STACK_auStack_118", 32),
            ("STACK_local_f8", 8),
            ("STACK_local_e8", 1),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 8),
            ("STACK_local_b8", 1),
            ("STACK_local_b0", 24),
            ("STACK_local_98", 1),
            ("STACK_local_90", 8),
            ("STACK_uStack_88", 8),
            ("STACK_local_80", 8),
            ("STACK_uStack_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_56", 4),
            ("STACK_local_50", 8),
            ("STACK_uStack_48", 8),
            ("STACK_local_40", 8),
        ],
    ),
    ("FUN_14001ce50", &[("STACK_local_res10", 24)]),
    ("FUN_14001d3a0", &[("STACK_local_res10", 24)]),
    (
        "FUN_140032df0",
        &[
            ("STACK_auStack_278", 32),
            ("STACK_local_258", 8),
            ("STACK_local_250", 8),
            ("STACK_local_248", 8),
            ("STACK_local_240", 8),
            ("STACK_local_238", 16),
            ("STACK_local_228", 528),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400548c0",
        &[
            ("STACK_auStack_78", 40),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400678c0",
        &[
            ("STACK_auStackY_288", 32),
            ("STACK_local_238", 16),
            ("STACK_local_228", 528),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14006e350",
        &[
            ("STACK_auStack_248", 32),
            ("STACK_local_228", 528),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400b4860",
        &[
            ("STACK_auStack_138", 32),
            ("STACK_local_118", 8),
            ("STACK_local_110", 2),
            ("STACK_local_108", 8),
            ("STACK_local_f8", 8),
            ("STACK_local_f0", 8),
            ("STACK_local_e8", 8),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c0", 24),
            ("STACK_local_a8", 8),
            ("STACK_local_88", 8),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_1400b51c0",
        &[
            ("STACK_auStack_148", 32),
            ("STACK_local_128", 2),
            ("STACK_local_120", 8),
            ("STACK_local_118", 8),
            ("STACK_local_108", 8),
            ("STACK_local_100", 8),
            ("STACK_local_f8", 8),
            ("STACK_local_f0", 8),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 24),
            ("STACK_local_c0", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_30", 8),
        ],
    ),
];
