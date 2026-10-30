//! Checked translations generated from original bodies by batch 018.
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
fn float_to_integer(value: f64, width: u32, signed: bool) -> Result<u64, AdapterError> {
    let value = value.trunc();
    let lower = if signed {
        -(2.0f64).powi(width as i32 - 1)
    } else {
        0.0
    };
    let upper = (2.0f64).powi(width as i32 - i32::from(signed));
    if !value.is_finite() || value < lower || value >= upper {
        return Err(AdapterError::Arithmetic("floating conversion out of range"));
    }
    Ok(if signed {
        value.trunc() as i64 as u64
    } else {
        value.trunc() as u64
    })
}
/// Translation of `FUN_14000b550` (`mspaint.c:14837`).
pub fn fun_14000b550(
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
    let mut pc: i32 = 5;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = values[1];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                values[3] = {
                    let call_arguments = [memory.read_u64(values[2])?];
                    invoke(
                        "Paint::PlatformHelpers::PackageInstaller::Progress",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 1;
            }
            3 => {
                values[2] = relocate("DAT_00000028");
                pc = 2;
            }
            4 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[2] = (values[0]).wrapping_add(0x18u64);
                pc = 4;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400165d0` (`mspaint.c:24117`).
pub fn fun_1400165d0(
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
    ];
    let mut pc: i32 = 61;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_e0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                return Ok(Some(0x1u64));
            }
            2 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_88"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_88"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    2
                } else {
                    1
                };
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_140017ea0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x0u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = (values[0]).wrapping_add((0x28cu64).wrapping_mul(4));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(4));
                    let stored =
                        memory.read_u64((values[10]).wrapping_add((0x4u64).wrapping_mul(4)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(4));
                    let stored = u64::from((values[4]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = u64::from((values[3]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = u64::from((values[5]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = values[0];
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[10])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 13;
            }
            15 => {
                values[3] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 14;
            }
            16 => {
                values[5] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 15;
            }
            17 => {
                values[10] = {
                    let call_arguments = [];
                    invoke("FUN_140013680", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [
                        values[7],
                        0x1cu64,
                        relocate("STACK_local_48"),
                        relocate("STACK_local_e0"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendPerformanceCompleteEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = relocate("STACK_local_e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = (((values[6]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_90")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored =
                        (((u64::from((memory.read_u64(relocate("STACK_local_90"))?) as u32))
                            & (0xffffffffu64))
                            .wrapping_shl(0x20u64 as u32))
                            | ((values[5]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                values[6] = ({
                    let call_arguments = [values[9]];
                    invoke(
                        "Paint::Provenance::ProvenanceHelper::IngredientCount",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                values[9] = {
                    let call_arguments = [values[0]];
                    invoke("FUN_140017300", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[6]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    24
                } else {
                    0
                };
            }
            26 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x266u64).wrapping_mul(4)),
                            )?)?)
                            .wrapping_add(0x58u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x266u64).wrapping_mul(4)))?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_e0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x44d9u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_98"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    31
                } else {
                    30
                };
            }
            33 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_98")];
                        invoke("FUN_140007410", &call_arguments, memory)
                    }) as u32,
                );
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_e0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            35 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x266u64).wrapping_mul(4)),
                            )?)?)
                            .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x266u64).wrapping_mul(4)))?,
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_e0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x4481u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let call_arguments = [
                        values[0],
                        u64::from(((f64::from_bits(values[2])) as f32).to_bits()),
                        u64::from(((f64::from_bits(values[1])) as f32).to_bits()),
                    ];
                    invoke("FUN_140017a60", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                values[2] = {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke(
                        "Paint::ProjectFile::LayerStack::DpiX",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 41;
            }
            43 => {
                values[1] = {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke(
                        "Paint::ProjectFile::LayerStack::DpiY",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_88")];
                    invoke("FUN_14003fcf0", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                values[11] = 0x0u64;
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add((0x266u64).wrapping_mul(4)))?)
                        as i64)
                        == ((0x0u64) as i64),
                )) != 0
                {
                    45
                } else {
                    44
                };
            }
            47 => {
                values[11] = (memory
                    .read_u64((values[0]).wrapping_add((0x266u64).wrapping_mul(4)))?)
                .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                pc = 46;
            }
            48 => {
                {
                    let call_arguments = [values[8], relocate("STACK_local_88")];
                    invoke(
                        "Paint::ProjectFile::ProjectFileHandler::Load",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 47;
            }
            49 => {
                values[8] = {
                    let call_arguments = [values[0]];
                    invoke("FUN_1400172a0", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_uStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [
                        values[7],
                        relocate("STACK_local_48"),
                        0x1cu64,
                        relocate("STACK_local_88"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendPerformanceStartEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                values[7] = {
                    let call_arguments = [];
                    invoke(
                        "Paint::Telemetry::TraceLogger::GetInstance",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 59;
            }
            61 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_118"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140016ec0` (`mspaint.c:24511`).
pub fn fun_140016ec0(
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
    let mut pc: i32 = 42;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_a8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_a8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            2 => {
                return Ok(Some(0x1u64));
            }
            3 => {
                {
                    let call_arguments = [
                        0x1u64,
                        float_to_integer(
                            (f32::from_bits(
                                (u64::from(
                                    ((memory.read_u64(relocate("STACK_local_78"))?) as f32)
                                        .to_bits(),
                                )) as u32,
                            )) as f64,
                            32,
                            true,
                        )?,
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_78")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as i32 as i64 as u64,
                        values[4],
                    ];
                    invoke(
                        "Paint::Telemetry::TraceMeasureLogger::TraceRestoreRecoveryData",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_b0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_b0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[4] = ({
                    let call_arguments = [relocate("STACK_local_b0")];
                    invoke("FUN_140007410", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = memory.read_u64(relocate("STACK_local_80"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[4]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    1
                };
            }
            9 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(values[1])?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination =
                        (relocate("STACK_local_a8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x4481u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[4]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    0
                };
            }
            15 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[1])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination =
                        (relocate("STACK_local_a8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x44d9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_70"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_70"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_70"))?) != (0x0u64))) != 0
                {
                    20
                } else {
                    19
                };
            }
            22 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140017ea0", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let destination = (values[0]).wrapping_add(0xa30u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [
                        values[0],
                        u64::from(((f64::from_bits(values[3])) as f32).to_bits()),
                        u64::from(((f64::from_bits(values[2])) as f32).to_bits()),
                    ];
                    invoke("FUN_140017a60", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                values[3] = {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke(
                        "Paint::ProjectFile::LayerStack::DpiX",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 24;
            }
            26 => {
                values[2] = {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke(
                        "Paint::ProjectFile::LayerStack::DpiY",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_70")];
                    invoke("FUN_14003fcf0", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                values[6] = 0x0u64;
                pc = 27;
            }
            29 => {
                pc = if (u64::from(((memory.read_u64(values[1])?) as i64) == ((0x0u64) as i64)))
                    != 0
                {
                    28
                } else {
                    27
                };
            }
            30 => {
                values[6] = (memory.read_u64(values[1])?)
                    .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                pc = 29;
            }
            31 => {
                values[1] = (values[0]).wrapping_add(0x998u64);
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_70")];
                    invoke(
                        "Paint::ProjectFile::ProjectFileHandler::Restore",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 31;
            }
            33 => {
                values[5] = {
                    let call_arguments = [];
                    invoke("FUN_1400172a0", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_uStack_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_uStack_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_d8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400170c0` (`mspaint.c:24595`).
pub fn fun_1400170c0(
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
    let mut pc: i32 = 40;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_98")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_98")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            2 => {
                return Ok(Some(0x1u64));
            }
            3 => {
                {
                    let call_arguments = [
                        0x1u64,
                        float_to_integer(
                            (f32::from_bits(
                                (u64::from(
                                    ((memory.read_u64(relocate("STACK_local_68"))?) as f32)
                                        .to_bits(),
                                )) as u32,
                            )) as f64,
                            32,
                            true,
                        )?,
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as i32 as i64 as u64,
                        values[3],
                    ];
                    invoke(
                        "Paint::Telemetry::TraceMeasureLogger::TraceBackupRecoveryData",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_a0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[3] = ({
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("FUN_140007410", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = memory.read_u64(relocate("STACK_local_70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[3]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    1
                };
            }
            9 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(values[2])?,
                        relocate("STACK_local_70"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x4481u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[3]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    0
                };
            }
            15 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[2])?,
                        relocate("STACK_local_68"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x44d9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_60"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_60"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    20
                } else {
                    19
                };
            }
            22 => {
                {
                    let call_arguments = [relocate("STACK_local_50")];
                    invoke("FUN_140017ea0", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [values[4], values[1], relocate("STACK_local_60")];
                    invoke(
                        "Paint::ProjectFile::ProjectFileHandler::Backup",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 22;
            }
            24 => {
                values[4] = {
                    let call_arguments = [values[0]];
                    invoke("FUN_1400172a0", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_60"),
                        ((f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32((values[0]).wrapping_add(0xa58u64))?))
                                    as u32,
                            )) as u32,
                        )) as f64)
                            .to_bits(),
                    ];
                    invoke(
                        "Paint::ProjectFile::LayerStack::DpiY",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 24;
            }
            26 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_60"),
                        ((f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32((values[0]).wrapping_add(0xa54u64))?))
                                    as u32,
                            )) as u32,
                        )) as f64)
                            .to_bits(),
                    ];
                    invoke(
                        "Paint::ProjectFile::LayerStack::DpiX",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_60")];
                    invoke("FUN_14003f8b0", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                values[5] = 0x0u64;
                pc = 28;
            }
            30 => {
                pc = if (u64::from(((memory.read_u64(values[2])?) as i64) == ((0x0u64) as i64)))
                    != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[5] = (memory.read_u64(values[2])?)
                    .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                pc = 30;
            }
            32 => {
                values[2] = (values[0]).wrapping_add(0x998u64);
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_uStack_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_uStack_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140017a60` (`mspaint.c:25085`).
pub fn fun_140017a60(
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
        u64::from((arguments[2]) as u32),
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 25;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_48")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_48")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            2 => {
                return Ok(None);
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from(((values[3]) as i64) != ((0x0u64) as i64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [relocate("STACK_local_28")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[5]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    1
                };
            }
            8 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_28"))?)?)
                                .wrapping_add(0x180u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_28"))?,
                        float_to_integer(
                            f64::from_bits(
                                (((f32::from_bits(
                                    (u64::from(
                                        (u64::from(
                                            memory.read_u32((values[0]).wrapping_add(0xa54u64))?,
                                        )) as u32,
                                    )) as u32,
                                )) as f64)
                                    / (f64::from_bits(0x3f9a027525460aa6u64)))
                                .to_bits(),
                            ),
                            32,
                            true,
                        )?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x1053u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[5]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    0
                };
            }
            13 => {
                values[4] = memory.read_u64(relocate("STACK_local_28"))?;
                pc = 12;
            }
            14 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0xa8u64))?,
                        values[3],
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 13;
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
                    let destination = relocate("STACK_local_40");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_28");
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
                    let destination = relocate("STACK_local_30");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                values[3] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 21;
            }
            23 => {
                {
                    let destination = (values[0]).wrapping_add(0xa58u64);
                    let stored = u64::from((values[2]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = (values[0]).wrapping_add(0xa54u64);
                    let stored = u64::from((values[1]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400349f0` (`mspaint.c:48990`).
pub fn fun_1400349f0(
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
    ];
    let mut pc: i32 = 32;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(4));
                    let stored =
                        (float_to_integer((f32::from_bits((values[4]) as u32)) as f64, 32, true)?)
                            as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                values[4] = {
                    let call_arguments = [u64::from(
                        ((f32::from_bits((values[7]) as u32))
                            - (f32::from_bits((values[6]) as u32)))
                        .to_bits(),
                    )];
                    invoke("ceilf", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored =
                        (float_to_integer((f32::from_bits((values[4]) as u32)) as f64, 32, true)?)
                            as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                values[4] = {
                    let call_arguments = [u64::from(
                        ((f32::from_bits((values[5]) as u32))
                            - (f32::from_bits((values[4]) as u32)))
                        .to_bits(),
                    )];
                    invoke("ceilf", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored =
                        (float_to_integer((f32::from_bits((values[8]) as u32)) as f64, 32, true)?)
                            as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                values[8] = {
                    let call_arguments = [values[6]];
                    invoke("floorf", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination = values[0];
                    let stored =
                        (float_to_integer((f32::from_bits((values[8]) as u32)) as f64, 32, true)?)
                            as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                values[8] = {
                    let call_arguments = [values[4]];
                    invoke("floorf", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                values[7] = u64::from(
                    (({
                        let call_arguments = [];
                        invoke("fmaxf", &call_arguments, memory)
                    }) as f32)
                        .to_bits(),
                );
                pc = 8;
            }
            10 => {
                values[6] = u64::from(
                    (({
                        let call_arguments = [];
                        invoke("fminf", &call_arguments, memory)
                    }) as f32)
                        .to_bits(),
                );
                pc = 9;
            }
            11 => {
                values[5] = u64::from(
                    (({
                        let call_arguments = [];
                        invoke("fmaxf", &call_arguments, memory)
                    }) as f32)
                        .to_bits(),
                );
                pc = 10;
            }
            12 => {
                values[4] = u64::from(
                    (({
                        let call_arguments = [];
                        invoke("fminf", &call_arguments, memory)
                    }) as f32)
                        .to_bits(),
                );
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [];
                    invoke("fmaxf", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [];
                    invoke("fminf", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [];
                    invoke("fmaxf", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [];
                    invoke("fminf", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [];
                    invoke("fmaxf", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [];
                    invoke("fminf", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [];
                    invoke("fmaxf", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [];
                    invoke("fminf", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[3]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    20
                };
            }
            23 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(values[2])?,
                        relocate("STACK_local_88"),
                        0x4u64,
                    ];
                    invoke("GdipTransformMatrixPoints", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_6c");
                    let stored =
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_74"))?)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored =
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_88"))?)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored =
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_80"))?)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_7c");
                    let stored =
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_84"))?)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_74");
                    let stored = u64::from(
                        ((((((u64::from(
                            memory.read_u32((values[1]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64)
                            .wrapping_add(
                                (u64::from(memory.read_u32(
                                    (values[1]).wrapping_add((0x1u64).wrapping_mul(4)),
                                )?)) as i32 as i64 as u64,
                            )) as i32 as i64 as u64) as i64) as f32)
                            .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_84");
                    let stored = u64::from(
                        ((((u64::from(
                            memory.read_u32((values[1]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64) as i64) as f32)
                            .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = u64::from(
                        ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i64)
                            as f32)
                            .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = u64::from(
                        ((((((u64::from(
                            memory.read_u32((values[1]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64)
                            .wrapping_add(
                                (u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64,
                            )) as i32 as i64 as u64) as i64) as f32)
                            .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_a8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14003d1f0` (`mspaint.c:57012`).
pub fn fun_14003d1f0(
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
        (arguments[2]) as i8 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 88;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [values[6]];
                    invoke("FUN_1400c0b50", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[6]).wrapping_add((0x3cu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0x8u64))?,
                        values[7],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                values[7] = (values[6]).wrapping_add((0x10u64).wrapping_mul(2));
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [values[7]];
                    invoke("FUN_14004f150", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[6]).wrapping_add((0x3bu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[6]).wrapping_add((0x3au64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[6]).wrapping_add((0x39u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[6]).wrapping_add((0x38u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[6]).wrapping_add((0x37u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[6]).wrapping_add((0x36u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[6]).wrapping_add((0x35u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[6]).wrapping_add((0x34u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = (values[6]).wrapping_add((0x33u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = (values[6]).wrapping_add((0x32u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[6]).wrapping_add((0x31u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = (values[6]).wrapping_add((0x30u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = (values[6]).wrapping_add((0x2fu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = (values[6]).wrapping_add((0x2eu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[6]).wrapping_add((0x2du64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = (values[6]).wrapping_add((0x2cu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = (values[6]).wrapping_add((0x2bu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = (values[6]).wrapping_add((0x2au64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = (values[6]).wrapping_add((0x29u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[6]).wrapping_add((0x28u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = (values[6]).wrapping_add((0x27u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = (values[6]).wrapping_add((0x26u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = (values[6]).wrapping_add((0x25u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[6]).wrapping_add((0x24u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = (values[6]).wrapping_add((0x23u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = (values[6]).wrapping_add((0x22u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = (values[6]).wrapping_add((0x21u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = (values[6]).wrapping_add((0x20u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = (values[6]).wrapping_add((0x1fu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[6]).wrapping_add((0x1eu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = (values[6]).wrapping_add((0x1du64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (values[6]).wrapping_add((0x1cu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = (values[6]).wrapping_add((0x1bu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (values[6]).wrapping_add((0x1au64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[6]).wrapping_add((0x19u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = (values[6]).wrapping_add((0x18u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = (values[6]).wrapping_add((0x17u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = (values[6]).wrapping_add((0x16u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = (values[6]).wrapping_add((0x15u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = (values[6]).wrapping_add((0x14u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = (values[6]).wrapping_add((0x13u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (values[6]).wrapping_add((0x12u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = (values[6]).wrapping_add((0x11u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[6]).wrapping_add((0x10u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[6]).wrapping_add((0xfu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = (values[6]).wrapping_add((0xeu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = (values[6]).wrapping_add((0xdu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = (values[6]).wrapping_add((0xcu64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = (values[7]).wrapping_add((0x3u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = (values[7]).wrapping_add((0x2u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = (values[7]).wrapping_add((0x0u64).wrapping_mul(2));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 61;
            }
            63 => {
                values[7] = (values[6]).wrapping_add((0x8u64).wrapping_mul(2));
                pc = 62;
            }
            64 => {
                {
                    let destination = (values[6]).wrapping_add((0x45u64).wrapping_mul(2));
                    let stored = u64::from((0x1u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = (values[6]).wrapping_add((0x44u64).wrapping_mul(2));
                    let stored = u64::from((0x2u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = (values[6]).wrapping_add((0x40u64).wrapping_mul(2));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = values[6];
                    let stored = relocate("FUN_1400c0b50");
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = (values[6]).wrapping_add((0x4u64).wrapping_mul(2));
                    let stored = relocate("FUN_1400c0b40");
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                values[6] = {
                    let call_arguments = [0xe0u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 68;
            }
            70 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x28u64))?;
                pc = 69;
            }
            71 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    70
                } else {
                    0
                };
            }
            72 => {
                {
                    let call_arguments = [
                        values[5],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 71;
            }
            73 => {
                values[5] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 72;
            }
            74 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            75 => {
                pc = if (u64::from(
                    (((((values[3]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    74
                } else {
                    71
                };
            }
            76 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    73
                } else {
                    75
                };
            }
            77 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 76;
            }
            78 => {
                {
                    let destination = values[7];
                    let stored = ((((u64::from(memory.read_u32(values[7])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 77;
            }
            79 => {
                values[3] = ((u64::from(memory.read_u32(values[7])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 78;
            }
            80 => {
                values[7] = (memory.read_u64(relocate("STACK_local_38"))?)
                    .wrapping_add((0xcu64).wrapping_mul(2));
                pc = 79;
            }
            81 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 80;
            }
            82 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64))) != 0
                {
                    81
                } else {
                    71
                };
            }
            83 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_140041100", &call_arguments, memory)
                };
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [relocate("STACK_local_38"), relocate("STACK_local_30")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x4u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = relocate("L\"Size\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = (values[0]).wrapping_add(0x30u64);
                    let stored = memory.read_u64(values[1])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32((values[0]).wrapping_add(0x30u64))?))
                                    as u32,
                            )) as u32,
                        )) != (f32::from_bits(
                            (u64::from((u64::from(memory.read_u32(values[1])?)) as u32)) as u32,
                        )),
                    )) != 0)
                        || ((u64::from(
                            (f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32((values[0]).wrapping_add(0x34u64))?))
                                        as u32,
                                )) as u32,
                            )) != (f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32(
                                        (values[1]).wrapping_add((0x1u64).wrapping_mul(4)),
                                    )?)) as u32,
                                )) as u32,
                            )),
                        )) != 0),
                )) != 0
                {
                    87
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140040ed0` (`mspaint.c:60355`).
pub fn fun_140040ed0(
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
    ];
    let mut pc: i32 = 40;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_58"))?) as i64) != ((0x0u64) as i64),
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
                {
                    let call_arguments = [values[5], relocate("STACK_local_50")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            5 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_30"))?)?)
                                .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_30"))?,
                        memory.read_u64(relocate("STACK_local_58"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_50")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x2a8u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_50")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            11 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    9
                };
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
                {
                    let destination = relocate("STACK_local_48");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_50")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x2a6u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[5] = (0x0u64) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[3])?)?,
                        values[3],
                        relocate("DAT_140105e40"),
                        relocate("STACK_local_30"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 14;
            }
            17 => {
                pc = if (u64::from((values[3]) == (0x0u64))) != 0 {
                    15
                } else {
                    16
                };
            }
            18 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[3] = memory.read_u64((values[0]).wrapping_add(0x38u64))?;
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [values[0], relocate("STACK_local_58"), values[1]];
                    invoke("FUN_140040280", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [
                        values[7],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_30"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            24 => {
                pc = if (u64::from(
                    ((((((values[4]) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    23
                } else {
                    20
                };
            }
            25 => {
                pc = if (u64::from(
                    ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    22
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
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 27;
            }
            29 => {
                values[2] = (memory.read_u64(relocate("STACK_local_30"))?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                pc = 28;
            }
            30 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 29;
            }
            31 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64))) != 0
                {
                    30
                } else {
                    20
                };
            }
            32 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_140041100", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let call_arguments = [relocate("STACK_local_30"), relocate("STACK_local_50")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x4u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = relocate("L\"Size\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = (values[0]).wrapping_add(0x30u64);
                    let stored = memory.read_u64(values[6])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32((values[0]).wrapping_add(0x30u64))?))
                                    as u32,
                            )) as u32,
                        )) != (f32::from_bits(
                            (u64::from((u64::from(memory.read_u32(values[6])?)) as u32)) as u32,
                        )),
                    )) != 0)
                        || ((u64::from(
                            (f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32((values[0]).wrapping_add(0x34u64))?))
                                        as u32,
                                )) as u32,
                            )) != (f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32(
                                        (values[6]).wrapping_add((0x1u64).wrapping_mul(4)),
                                    )?)) as u32,
                                )) as u32,
                            )),
                        )) != 0),
                )) != 0
                {
                    36
                } else {
                    20
                };
            }
            38 => {
                values[6] = {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FromNative", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [memory.read_u64(values[1])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140041c90` (`mspaint.c:61177`).
pub fn fun_140041c90(
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
    ];
    let mut pc: i32 = 70;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = values[1];
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = ((values[10]).wrapping_add(0x8u64))
                        .wrapping_add((values[8]).wrapping_mul(0x10u64));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[10]).wrapping_add((values[8]).wrapping_mul(0x10u64));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                pc = 2;
            }
            6 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        ((values[10]).wrapping_add(0x8u64))
                            .wrapping_add((values[8]).wrapping_mul(0x10u64)),
                    )?) != (values[4]),
                )) != 0
                {
                    5
                } else {
                    3
                };
            }
            7 => {
                pc = 2;
            }
            8 => {
                {
                    let destination = (values[10]).wrapping_add((values[8]).wrapping_mul(0x10u64));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                pc = if (u64::from((values[6]) == (values[13]))) != 0 {
                    8
                } else {
                    6
                };
            }
            10 => {
                pc = if (u64::from((values[6]) == (memory.read_u64(values[3])?))) != 0 {
                    4
                } else {
                    9
                };
            }
            11 => {
                values[6] = memory
                    .read_u64((values[10]).wrapping_add((values[8]).wrapping_mul(0x10u64)))?;
                pc = 10;
            }
            12 => {
                values[10] = memory.read_u64((values[0]).wrapping_add((0x6u64).wrapping_mul(4)))?;
                pc = 11;
            }
            13 => {
                values[8] &= memory.read_u64((values[0]).wrapping_add((0xcu64).wrapping_mul(4)))?;
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[13]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = values[4];
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[9]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = values[9];
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(4));
                    let stored = (memory
                        .read_u64((values[0]).wrapping_add((0x4u64).wrapping_mul(4)))?)
                    .wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[4] = memory.read_u64((values[13]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 18;
            }
            20 => {
                values[13] = memory.read_u64(values[13])?;
                pc = 19;
            }
            21 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    20
                };
            }
            22 => {
                values[7] = ({
                    let call_arguments = [
                        (values[9]).wrapping_add((0x2u64).wrapping_mul(8)),
                        (values[13]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 21;
            }
            23 => {
                values[13] =
                    memory.read_u64((values[13]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 22;
            }
            24 => {
                pc = 19;
            }
            25 => {
                pc = if (u64::from((values[13]) == (values[6]))) != 0 {
                    24
                } else {
                    23
                };
            }
            26 => {
                values[13] = values[4];
                pc = 21;
            }
            27 => {
                values[7] = ({
                    let call_arguments = [
                        (values[9]).wrapping_add((0x2u64).wrapping_mul(8)),
                        (values[4]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 26;
            }
            28 => {
                values[6] = memory.read_u64(
                    (memory.read_u64((values[0]).wrapping_add((0x6u64).wrapping_mul(4)))?)
                        .wrapping_add(
                            ((memory
                                .read_u64((values[0]).wrapping_add((0xcu64).wrapping_mul(4)))?)
                                & (values[8]))
                                .wrapping_mul(0x10u64),
                        ),
                )?;
                pc = 27;
            }
            29 => {
                pc = if (u64::from((values[4]) != (values[13]))) != 0 {
                    28
                } else {
                    19
                };
            }
            30 => {
                values[13] = memory.read_u64(values[3])?;
                pc = 29;
            }
            31 => {
                values[4] = memory.read_u64(
                    ((memory.read_u64((values[0]).wrapping_add((0x6u64).wrapping_mul(4)))?)
                        .wrapping_add(0x8u64))
                    .wrapping_add(
                        ((memory.read_u64((values[0]).wrapping_add((0xcu64).wrapping_mul(4)))?)
                            & (values[8]))
                            .wrapping_mul(0x10u64),
                    ),
                )?;
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [values[0], values[12]];
                    invoke("FUN_140044400", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                values[12] = values[11];
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    ((u64::from((values[5]) < (values[11]))) != 0)
                        && ((u64::from(
                            ((u64::from((0x1ffu64) < (values[5]))) != 0)
                                || (({
                                    let _ = {
                                        values[12] = (values[5]).wrapping_mul(0x8u64);
                                        values[12]
                                    };
                                    u64::from(((values[5]).wrapping_mul(0x8u64)) < (values[11]))
                                }) != 0),
                        )) != 0),
                )) != 0
                {
                    33
                } else {
                    32
                };
            }
            35 => {
                values[12] = values[5];
                pc = 34;
            }
            36 => {
                values[11] =
                    (float_to_integer((f32::from_bits((values[14]) as u32)) as f64, 64, true)?)
                        .wrapping_add(values[10]);
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    (0x8u64)
                        < ((float_to_integer(
                            (f32::from_bits((values[14]) as u32)) as f64,
                            64,
                            true,
                        )?)
                        .wrapping_add(values[10])),
                )) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                values[11] = 0x8u64;
                pc = 37;
            }
            39 => {
                values[10] = 0u64.wrapping_sub(0x8000000000000000u64);
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f64::from_bits(0x43dffffffddad230u64))
                            <= ((f32::from_bits((values[14]) as u32)) as f64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[14] = u64::from(
                                    ((f64::from_bits(
                                        (((f32::from_bits((values[14]) as u32)) as f64)
                                            - (f64::from_bits(0x43dffffffddad230u64)))
                                        .to_bits(),
                                    )) as f32)
                                        .to_bits(),
                                );
                                values[14]
                            };
                            u64::from(
                                ((f32::from_bits((values[14]) as u32)) as f64)
                                    < (f64::from_bits(0x43dffffffddad230u64)),
                            )
                        }) != 0),
                )) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                values[10] = 0x0u64;
                pc = 40;
            }
            42 => {
                values[14] = {
                    let call_arguments = [u64::from(
                        ((f32::from_bits(
                            (u64::from(
                                ((((memory.read_u64(
                                    (values[0]).wrapping_add((0x4u64).wrapping_mul(4)),
                                )?)
                                .wrapping_add(0x1u64)) as i64)
                                    as f32)
                                    .to_bits(),
                            )) as u32,
                        )) / (f32::from_bits(
                            (u64::from((u64::from(memory.read_u32(values[0])?)) as u32)) as u32,
                        )))
                        .to_bits(),
                    )];
                    invoke("ceilf", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    (f32::from_bits(
                        (u64::from((u64::from(memory.read_u32(values[0])?)) as u32)) as u32,
                    )) < (f32::from_bits(
                        (u64::from(
                            ((f32::from_bits(
                                (u64::from(
                                    ((((memory.read_u64(
                                        (values[0]).wrapping_add((0x4u64).wrapping_mul(4)),
                                    )?)
                                    .wrapping_add(0x1u64))
                                        as i64) as f32)
                                        .to_bits(),
                                )) as u32,
                            )) / (f32::from_bits(
                                (u64::from(((values[5]) as f32).to_bits())) as u32,
                            )))
                            .to_bits(),
                        )) as u32,
                    )),
                )) != 0
                {
                    42
                } else {
                    19
                };
            }
            44 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0xeu64).wrapping_mul(4)))?;
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[9]).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = (values[9]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                pc = if (u64::from((values[2]) != (0x0u64))) != 0 {
                    47
                } else {
                    46
                };
            }
            49 => {
                {
                    let destination = (values[9]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                values[2] = memory.read_u64(values[2])?;
                pc = 49;
            }
            51 => {
                values[9] = {
                    let call_arguments = [0x28u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [relocate("\"unordered_map/set too long\"")];
                    invoke("std::_Xlength_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xlength_error"));
            }
            53 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add((0x4u64).wrapping_mul(4)))?) as i64)
                        == ((0x666666666666666u64) as i64),
                )) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                pc = if (0x1u64) != 0 { 62 } else { 53 };
            }
            55 => {
                values[7] = ({
                    let call_arguments = [
                        values[2],
                        (values[13]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 54;
            }
            56 => {
                values[13] =
                    memory.read_u64((values[13]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 55;
            }
            57 => {
                pc = 53;
            }
            58 => {
                pc = if (u64::from((values[13]) == (values[4]))) != 0 {
                    57
                } else {
                    56
                };
            }
            59 => {
                return Ok(Some(values[1]));
            }
            60 => {
                {
                    let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = values[1];
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    61
                } else {
                    58
                };
            }
            63 => {
                values[13] = values[9];
                pc = 54;
            }
            64 => {
                values[7] = ({
                    let call_arguments = [
                        values[2],
                        (values[9]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 63;
            }
            65 => {
                values[4] = memory.read_u64(
                    (memory.read_u64((values[0]).wrapping_add((0x6u64).wrapping_mul(4)))?)
                        .wrapping_add(
                            ((values[8])
                                & (memory.read_u64(
                                    (values[0]).wrapping_add((0xcu64).wrapping_mul(4)),
                                )?))
                            .wrapping_mul(0x10u64),
                        ),
                )?;
                pc = 64;
            }
            66 => {
                pc = if (u64::from((values[9]) != (values[13]))) != 0 {
                    65
                } else {
                    53
                };
            }
            67 => {
                values[13] = memory.read_u64(values[3])?;
                pc = 66;
            }
            68 => {
                values[3] = (values[0]).wrapping_add((0x2u64).wrapping_mul(4));
                pc = 67;
            }
            69 => {
                values[9] = memory.read_u64(
                    ((memory.read_u64((values[0]).wrapping_add((0x6u64).wrapping_mul(4)))?)
                        .wrapping_add(0x8u64))
                    .wrapping_add(
                        ((values[8])
                            & (memory
                                .read_u64((values[0]).wrapping_add((0xcu64).wrapping_mul(4)))?))
                        .wrapping_mul(0x10u64),
                    ),
                )?;
                pc = 68;
            }
            70 => {
                values[8] = {
                    let call_arguments = [values[0], values[2]];
                    invoke("FUN_1400445c0", &call_arguments, memory)
                };
                pc = 69;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140051e90` (`mspaint.c:74546`).
pub fn fun_140051e90(
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
                {
                    let call_arguments = [values[1], relocate("STACK_local_20")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                return Ok(Some(
                    ((f32::from_bits(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_28"))?) as f32).to_bits(),
                        )) as u32,
                    )) as f64)
                        .to_bits(),
                ));
            }
            2 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[1]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                values[1] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x50u64))?)?)
                            .wrapping_add(0x120u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x50u64))?,
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 2;
            }
            4 => {
                {
                    let destination =
                        (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0xf82u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140052570` (`mspaint.c:74942`).
pub fn fun_140052570(
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
    ];
    let mut pc: i32 = 49;
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
                    let destination = values[1];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            3 => {
                values[7] = values[10];
                pc = 2;
            }
            4 => {
                pc = if (u64::from((values[9]) == (0x0u64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[7] = (values[9]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_50")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    ((u64::from((values[11]) != (0x0u64))) != 0)
                        && ((u64::from((values[7]) != (0x0u64))) != 0),
                )) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                values[11] = 0x4u64;
                pc = 7;
            }
            9 => {
                values[9] = values[6];
                pc = 8;
            }
            10 => {
                values[7] = values[10];
                pc = 9;
            }
            11 => {
                {
                    let destination = values[6];
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[6]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored = float_to_integer(f64::from_bits(values[12]), 64, false)?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[6]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[6]).wrapping_add((0x5u64).wrapping_mul(8));
                    let stored = u64::from((u64::from(((values[8]) | (0x100u64)) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [(values[6]).wrapping_add((0x3u64).wrapping_mul(8))];
                    invoke("FromNative", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let destination = values[6];
                    let stored = relocate("Paint::XamlTrackerInformation::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[6]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[6]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::produce<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    22
                } else {
                    7
                };
            }
            24 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                values[11] = values[10];
                pc = 24;
            }
            26 => {
                values[9] = values[10];
                pc = 25;
            }
            27 => {
                values[7] = memory.read_u64(relocate("STACK_local_50"))?;
                pc = 26;
            }
            28 => {
                values[6] = {
                    let call_arguments = [0x40u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetTrackerRect", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                values[12] = {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetRotationAngle", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                values[8] = u64::from((u64::from(((values[8]) | (0x2000u64)) as u32)) as u32);
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[5]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u8((values[0]).wrapping_add(0xd9u64))?)) as i8
                                as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    31
                } else {
                    30
                };
            }
            33 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0x21u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 32;
            }
            34 => {
                values[8] = u64::from((0x90u64) as u32);
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                values[8] = u64::from((0x60u64) as u32);
                pc = 35;
            }
            37 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0x10u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let call_arguments = [values[0], values[1], 0x0u64];
                    invoke("FUN_140052790", &call_arguments, memory)
                };
                pc = 0;
            }
            39 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    38
                };
            }
            40 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0xdu64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 39;
            }
            41 => {
                {
                    let call_arguments = [values[0], values[1], 0xffu64];
                    invoke("FUN_140052790", &call_arguments, memory)
                };
                pc = 0;
            }
            42 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    41
                };
            }
            43 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0xfu64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [values[0], values[1], 0x20ffu64];
                    invoke("FUN_140052790", &call_arguments, memory)
                };
                pc = 0;
            }
            45 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    43
                } else {
                    44
                };
            }
            46 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0xeu64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 45;
            }
            47 => {
                pc = if (u64::from(
                    ((u64::from((values[4]) == (0x0u64))) != 0)
                        || (({
                            let _ = {
                                let _ = {
                                    let destination = relocate("STACK_local_50");
                                    let stored = values[1];
                                    memory.write_u64(destination, stored)?;
                                    stored
                                };
                                {
                                    values[5] = ({
                                        let call_arguments = [
                                            memory.read_u64(
                                                (memory.read_u64(values[4])?).wrapping_add(0x68u64),
                                            )?,
                                            values[4],
                                            0x7u64,
                                            values[2],
                                            values[3],
                                            0x0u64,
                                        ];
                                        invoke("INDIRECT_CALL", &call_arguments, memory)
                                    }) as i8 as i64
                                        as u64;
                                    values[5]
                                }
                            };
                            u64::from(
                                (((values[5]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    1
                } else {
                    46
                };
            }
            48 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
                pc = 47;
            }
            49 => {
                values[10] = 0x0u64;
                pc = 48;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140052790` (`mspaint.c:75037`).
pub fn fun_140052790(
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
    ];
    let mut pc: i32 = 29;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let destination = values[1];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                values[8] = 0x0u64;
                pc = 3;
            }
            5 => {
                pc = if (u64::from(((values[7]) as i64) == ((0x0u64) as i64))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[8] = (values[7]).wrapping_add(0x10u64);
                pc = 5;
            }
            7 => {
                values[7] = {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_48"))?,
                        values[6],
                        values[9],
                        values[5],
                        values[10],
                    ];
                    invoke("FUN_1400575e0", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_48"))?) != (0x0u64))) != 0
                {
                    7
                } else {
                    6
                };
            }
            9 => {
                values[7] = 0x0u64;
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = {
                        let call_arguments = [0x40u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                values[6] = {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetTrackerRect", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                values[5] = {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetTrackerRegion", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                values[10] = {
                    let call_arguments = [memory.read_u64((values[0]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetRotationAngle", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                values[9] = u64::from((u64::from(((values[9]) | (0x2000u64)) as u32)) as u32);
                pc = 13;
            }
            15 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[4]) as i32 as i64 as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                ((((u64::from(memory.read_u8((values[0]).wrapping_add(0xd9u64))?))
                                    as i8 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                values[4] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory
                                                .read_u64(memory.read_u64(
                                                    (values[0]).wrapping_add(0x40u64),
                                                )?)?)
                                            .wrapping_add(0x68u64),
                                        )?,
                                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                                        0xdu64,
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
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
                    14
                } else {
                    13
                };
            }
            16 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0x21u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 15;
            }
            17 => {
                values[9] = u64::from((values[3]) as u32);
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    16
                };
            }
            19 => {
                values[9] = u64::from((u64::from(((values[3]) | (0x1000u64)) as u32)) as u32);
                pc = 18;
            }
            20 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0xau64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 19;
            }
            21 => {
                values[3] = u64::from((values[9]) as u32);
                pc = 20;
            }
            22 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    20
                };
            }
            23 => {
                values[3] = u64::from((u64::from(((values[9]) | (0x400u64)) as u32)) as u32);
                pc = 22;
            }
            24 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0x9u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 23;
            }
            25 => {
                values[9] = u64::from((values[2]) as u32);
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    24
                };
            }
            27 => {
                values[9] = u64::from((u64::from(((values[2]) | (0x200u64)) as u32)) as u32);
                pc = 26;
            }
            28 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x40u64))?)?)
                            .wrapping_add(0x68u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        0x8u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 27;
            }
            29 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140053f20` (`mspaint.c:76274`).
pub fn fun_140053f20(
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
    ];
    let mut pc: i32 = 19;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_28")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_28"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_20")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            4 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x168u64),
                        )?,
                        memory.read_u64(values[4])?,
                        (((values[2]) as i64) as f64).to_bits(),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination =
                        (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x70ccu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                values[4] = {
                    let call_arguments = [relocate("STACK_local_28")];
                    invoke("FUN_14004e8c0", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_20")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            11 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    2
                };
            }
            12 => {
                values[3] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x170u64),
                        )?,
                        memory.read_u64(values[4])?,
                        (((values[2]) as i64) as f64).to_bits(),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination =
                        (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x70dcu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                values[4] = {
                    let call_arguments = [relocate("STACK_local_28")];
                    invoke("FUN_14004e8c0", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14000a4a0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a4a0"));
            }
            18 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    16
                };
            }
            19 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    18
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400884c0` (`mspaint.c:118947`).
pub fn fun_1400884c0(
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
        (arguments[1]) as i32 as i64 as u64,
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
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 75;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[16]));
            }
            1 => {
                {
                    let call_arguments = [
                        values[0],
                        (0u64.wrapping_sub(0x15u64)) as i32 as i64 as u64,
                        values[3],
                    ];
                    invoke("SetWindowLongPtrW", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = (values[10]).wrapping_add(0x30u64);
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            3 => {
                {
                    let destination = (values[10]).wrapping_add(0x28u64);
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                values[5] = memory.read_u64((values[3]).wrapping_add((0x2u64).wrapping_mul(4)))?;
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    ((u64::from(((values[8]) as i64) != ((0x0u64) as i64))) != 0)
                        && (({
                            let _ = {
                                values[10] = memory.read_u64((values[8]).wrapping_add(0x58u64))?;
                                values[10]
                            };
                            u64::from(((values[10]) as i64) != ((0x0u64) as i64))
                        }) != 0),
                )) != 0
                {
                    4
                } else {
                    0
                };
            }
            6 => {
                values[16] = 0x1u64;
                pc = 0;
            }
            7 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        values[6],
                        (u64::from(
                            memory.read_u32((values[3]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64,
                        values[19],
                        (u64::from(
                            memory.read_u32((values[3]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64,
                    ];
                    invoke("Rectangle", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let destination = values[3];
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                values[6] = (((((values[6]).wrapping_sub(values[18])) as i32 as i64 as u64)
                    .wrapping_add(values[19])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let destination = (values[3]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = (values[19]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                values[19] = ((((u64::from(memory.read_u32((values[10]).wrapping_add(0x30u64))?))
                    as i32 as i64 as u64)
                    .wrapping_sub((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 10;
            }
            12 => {
                {
                    let call_arguments =
                        [
                            values[9],
                            (((((u64::from(memory.read_u32((values[10]).wrapping_add(0x30u64))?))
                                as i32 as i64 as u64)
                                .wrapping_sub(values[6])) as i32
                                as i64 as u64)
                                .wrapping_sub(values[18])) as i32 as i64
                                as u64,
                            0x0u64,
                        ];
                    invoke("CDC::OffsetClipRgn", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                values[18] = ((u64::from(
                    memory.read_u32((values[3]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 12;
            }
            14 => {
                values[6] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 13;
            }
            15 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (u64::from((u64::from(((values[7]) & (0x1u64)) as u32)) as u32))
                                != (u64::from((0x0u64) as u32)),
                        )) != 0)
                            && ((u64::from(((values[8]) as i64) != ((0x0u64) as i64))) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    values[10] =
                                        memory.read_u64((values[8]).wrapping_add(0x58u64))?;
                                    values[10]
                                };
                                {
                                    values[16] = 0x0u64;
                                    values[16]
                                }
                            };
                            u64::from(((values[10]) as i64) != ((0x0u64) as i64))
                        }) != 0),
                )) != 0
                {
                    14
                } else {
                    0
                };
            }
            16 => {
                values[7] = u64::from(
                    ({
                        let call_arguments = [values[9]];
                        invoke("CDC::GetLayout", &call_arguments, memory)
                    }) as u32,
                );
                pc = 15;
            }
            17 => {
                values[9] = {
                    let call_arguments = [values[2]];
                    invoke("CDC::FromHandle", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                values[16] = 0x1u64;
                pc = 0;
            }
            19 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        values[16],
                    ];
                    invoke("CDC::SelectGdiObject", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                values[16] = memory.read_u64((values[11]).wrapping_add(0x8u64))?;
                pc = 19;
            }
            21 => {
                pc = if (u64::from(((values[11]) as i64) != ((0x0u64) as i64))) != 0 {
                    20
                } else {
                    19
                };
            }
            22 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        values[13],
                    ];
                    invoke("CDC::SelectGdiObject", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                values[13] = memory.read_u64((values[10]).wrapping_add(0x8u64))?;
                pc = 22;
            }
            24 => {
                pc = if (u64::from(((values[10]) as i64) != ((0x0u64) as i64))) != 0 {
                    23
                } else {
                    22
                };
            }
            25 => {
                values[13] = values[16];
                pc = 24;
            }
            26 => {
                {
                    let call_arguments = [
                        values[9],
                        ((values[12])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                        values[19],
                    ];
                    invoke("CDC::LineTo", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [
                        values[9],
                        (relocate("STACK_local_50")) as i32 as i64 as u64,
                        values[18],
                    ];
                    invoke("CDC::MoveTo", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [
                        values[9],
                        ((values[12])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                        ((values[6]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    ];
                    invoke("CDC::LineTo", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [
                        values[9],
                        (relocate("STACK_local_50")) as i32 as i64 as u64,
                        values[18],
                    ];
                    invoke("CDC::MoveTo", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        values[18],
                        values[19],
                        values[12],
                        values[6],
                    ];
                    invoke("Rectangle", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                values[11] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[9])?).wrapping_add(0x58u64))?,
                        values[9],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                values[10] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[9])?).wrapping_add(0x58u64))?,
                        values[9],
                        0x7u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    32
                } else {
                    18
                };
            }
            34 => {
                values[6] = (((values[19]).wrapping_add(float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits((((values[15]) as i64) as f64).to_bits()))
                            * (f64::from_bits(values[21])))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 33;
            }
            35 => {
                values[19] = (((values[19]).wrapping_add(float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits((((values[6]) as i64) as f64).to_bits()))
                            * (f64::from_bits(values[21])))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                values[12] = (((float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits((((values[14]) as i64) as f64).to_bits()))
                            * (f64::from_bits(values[22])))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)
                .wrapping_add(values[18])) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 35;
            }
            37 => {
                values[18] = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                    .wrapping_add(float_to_integer(
                        f64::from_bits(
                            ((f64::from_bits((((values[20]) as i64) as f64).to_bits()))
                                * (f64::from_bits(values[22])))
                            .to_bits(),
                        ),
                        32,
                        true,
                    )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                values[19] = ((u64::from(
                    memory.read_u32((values[3]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                values[18] = (((values[12]).wrapping_sub(float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits((((values[14]) as i64) as f64).to_bits()))
                            * (f64::from_bits(values[22])))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 35;
            }
            40 => {
                values[12] = ((((((u64::from(
                    memory.read_u32((values[4]).wrapping_add((0xcu64).wrapping_mul(4)))?,
                )) as i32 as i64 as u64)
                    .wrapping_sub((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64))
                    as i32 as i64 as u64)
                    .wrapping_sub(float_to_integer(
                        f64::from_bits(
                            ((f64::from_bits((((values[20]) as i64) as f64).to_bits()))
                                * (f64::from_bits(values[22])))
                            .to_bits(),
                        ),
                        32,
                        true,
                    )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                values[19] = ((u64::from(
                    memory.read_u32((values[3]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [
                        values[9],
                        (((((u64::from(
                            memory.read_u32((values[4]).wrapping_add((0xcu64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64)
                            .wrapping_sub(
                                (u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64,
                            )) as i32 as i64 as u64)
                            .wrapping_sub(
                                (u64::from(memory.read_u32(
                                    (values[3]).wrapping_add((0x2u64).wrapping_mul(4)),
                                )?)) as i32 as i64 as u64,
                            )) as i32 as i64 as u64,
                        0x0u64,
                    ];
                    invoke("CDC::OffsetClipRgn", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    (u64::from((u64::from(((values[7]) & (0x1u64)) as u32)) as u32))
                        == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    38
                } else {
                    42
                };
            }
            44 => {
                values[7] = u64::from(
                    ({
                        let call_arguments = [values[9]];
                        invoke("CDC::GetLayout", &call_arguments, memory)
                    }) as u32,
                );
                pc = 43;
            }
            45 => {
                values[9] = {
                    let call_arguments = [values[2]];
                    invoke("CDC::FromHandle", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                values[21] = ((f64::from_bits(
                    ((((((u64::from(
                        memory.read_u32((values[3]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64)
                        .wrapping_sub(
                            (u64::from(
                                memory
                                    .read_u32((values[3]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as i32 as i64 as u64,
                        )) as i32 as i64 as u64) as i64) as f64)
                        .to_bits(),
                )) / (f64::from_bits((((values[19]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 45;
            }
            47 => {
                values[22] = ((f64::from_bits(
                    ((((((u64::from(
                        memory.read_u32((values[3]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64)
                        .wrapping_sub(
                            (u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64,
                        )) as i32 as i64 as u64) as i64) as f64)
                        .to_bits(),
                )) / (f64::from_bits((((values[18]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 46;
            }
            48 => {
                values[6] = (({
                    let numerator =
                        (((values[12]).wrapping_sub(values[15])) as i32 as i64 as u64) as i32;
                    let denominator = (0x2u64) as i32;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[4]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    47
                };
            }
            50 => {
                values[20] = (({
                    let numerator =
                        (((values[17]).wrapping_sub(values[14])) as i32 as i64 as u64) as i32;
                    let denominator = (0x2u64) as i32;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 49;
            }
            51 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    49
                };
            }
            52 => {
                values[20] = (values[6]) as i32 as i64 as u64;
                pc = 51;
            }
            53 => {
                values[12] = (((values[12]).wrapping_sub(
                    ({
                        let numerator = (values[12]) as i32;
                        let denominator = (values[19]) as i32;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    }) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 52;
            }
            54 => {
                values[17] = (((values[17]).wrapping_sub(
                    ({
                        let numerator = (values[17]) as i32;
                        let denominator = (values[18]) as i32;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_rem(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    }) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 53;
            }
            55 => {
                values[17] = (((((values[14])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64)
                    .wrapping_add(values[18])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 54;
            }
            56 => {
                values[12] = (((((values[15])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64)
                    .wrapping_add(values[19])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 55;
            }
            57 => {
                values[14] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((u64::from(
                                    ((u64::from(memory.read_u32(
                                        (values[4]).wrapping_add((0x3u64).wrapping_mul(4)),
                                    )?)) as i32 as i64 as u64)
                                        as u32,
                                )) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(
                                memory
                                    .read_u64((values[4]).wrapping_add((0x6u64).wrapping_mul(4)))?,
                            )))
                            .to_bits(),
                        )) / (f64::from_bits(0x4059000000000000u64)))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 56;
            }
            58 => {
                values[15] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((u64::from(
                                    ((u64::from(memory.read_u32(
                                        (values[4]).wrapping_add((0x3u64).wrapping_mul(4)),
                                    )?)) as i32 as i64 as u64)
                                        as u32,
                                )) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(
                                memory
                                    .read_u64((values[4]).wrapping_add((0x8u64).wrapping_mul(4)))?,
                            )))
                            .to_bits(),
                        )) / (f64::from_bits(0x4059000000000000u64)))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 57;
            }
            59 => {
                values[15] = (values[12]) as i32 as i64 as u64;
                pc = 52;
            }
            60 => {
                values[14] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits((((values[12]) as i64) as f64).to_bits()))
                                * (f64::from_bits(memory.read_u64(
                                    (values[4]).wrapping_add((0x6u64).wrapping_mul(4)),
                                )?)))
                            .to_bits(),
                        )) / (f64::from_bits(
                            memory.read_u64((values[4]).wrapping_add((0x8u64).wrapping_mul(4)))?,
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        < (((values[15]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    60
                } else {
                    52
                };
            }
            62 => {
                values[14] = (values[17]) as i32 as i64 as u64;
                pc = 61;
            }
            63 => {
                values[15] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits((((values[17]) as i64) as f64).to_bits()))
                                * (f64::from_bits(memory.read_u64(
                                    (values[4]).wrapping_add((0x8u64).wrapping_mul(4)),
                                )?)))
                            .to_bits(),
                        )) / (f64::from_bits(
                            memory.read_u64((values[4]).wrapping_add((0x6u64).wrapping_mul(4)))?,
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 62;
            }
            64 => {
                values[12] = (((values[19]).wrapping_mul(
                    (u64::from(
                        memory.read_u32((values[4]).wrapping_add((0x5u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                values[17] = (((values[18]).wrapping_mul(
                    (u64::from(
                        memory.read_u32((values[4]).wrapping_add((0x4u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 64;
            }
            66 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[4]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    58
                } else {
                    65
                };
            }
            67 => {
                values[6] = (0x0u64) as i32 as i64 as u64;
                pc = 66;
            }
            68 => {
                values[19] = ((((((u64::from(memory.read_u32((values[8]).wrapping_add(0x28u64))?))
                    as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[8]).wrapping_add(0x48u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[8]).wrapping_add(0x40u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 67;
            }
            69 => {
                values[18] = ((((((u64::from(memory.read_u32((values[8]).wrapping_add(0x24u64))?))
                    as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[8]).wrapping_add(0x44u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[8]).wrapping_add(0x3cu64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[1]) as i32 as i64 as u64) as i64)
                                == (((0x404u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(((values[8]) as i64) != ((0x0u64) as i64))) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                values[4] = memory.read_u64((values[8]).wrapping_add(0x58u64))?;
                                values[4]
                            };
                            u64::from((values[4]) != (0x0u64))
                        }) != 0),
                )) != 0
                {
                    69
                } else {
                    0
                };
            }
            71 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x403u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    70
                };
            }
            72 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x401u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    71
                };
            }
            73 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x400u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    72
                };
            }
            74 => {
                values[16] = 0x0u64;
                pc = 73;
            }
            75 => {
                values[8] = {
                    let call_arguments =
                        [values[0], (0u64.wrapping_sub(0x15u64)) as i32 as i64 as u64];
                    invoke("GetWindowLongPtrW", &call_arguments, memory)
                };
                pc = 74;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400888e0` (`mspaint.c:119071`).
pub fn fun_1400888e0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
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
        (arguments[3]) as i32 as i64 as u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 33;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[5], 0x0u64, 0x1u64];
                    invoke("InvalidateRect", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[5] = {
                    let call_arguments = [values[1], 0x438u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [values[5], values[4]];
                    invoke("EnableWindow", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                values[5] = {
                    let call_arguments = [values[1], 0x4c5u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[5], values[4]];
                    invoke("EnableWindow", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                values[5] = {
                    let call_arguments = [values[1], 0x4c4u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4c4u64,
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x5u64).wrapping_mul(4)))?,
                            )) as u32,
                        ),
                        0x0u64,
                    ];
                    invoke("SetDlgItemInt", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [values[5], values[4]];
                    invoke("EnableWindow", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                values[5] = {
                    let call_arguments = [values[1], 0x4c3u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [values[5], values[4]];
                    invoke("EnableWindow", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                values[5] = {
                    let call_arguments = [values[1], 0x4c2u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4c2u64,
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x4u64).wrapping_mul(4)))?,
                            )) as u32,
                        ),
                        0x0u64,
                    ];
                    invoke("SetDlgItemInt", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4bfu64,
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                            )) as u32,
                        ),
                    ];
                    invoke("CheckDlgButton", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [
                        values[5],
                        u64::from(
                            (u64::from(
                                (u64::from((values[4]) as u32)) == (u64::from((0x0u64) as u32)),
                            )) as u32,
                        ),
                    ];
                    invoke("EnableWindow", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                values[5] = {
                    let call_arguments = [values[1], 0x4c1u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [
                        values[5],
                        u64::from(
                            (u64::from(
                                (u64::from((values[4]) as u32)) == (u64::from((0x0u64) as u32)),
                            )) as u32,
                        ),
                    ];
                    invoke("EnableWindow", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                values[5] = {
                    let call_arguments = [values[1], 0x4c0u64];
                    invoke("GetDlgItem", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4c0u64,
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                            )) as u32,
                        ),
                        0x0u64,
                    ];
                    invoke("SetDlgItemInt", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    24
                } else {
                    23
                };
            }
            26 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4beu64,
                        u64::from(
                            (u64::from(
                                (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(
                                            (values[0]).wrapping_add((0x2u64).wrapping_mul(4)),
                                        )?)) as u32,
                                    )) as u32,
                                )) == (u64::from((0x0u64) as u32)),
                            )) as u32,
                        ),
                    ];
                    invoke("CheckDlgButton", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4bcu64,
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as u32,
                        ),
                    ];
                    invoke("CheckDlgButton", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [
                        values[1],
                        0x4bbu64,
                        u64::from((u64::from(memory.read_u32(values[0])?)) as u32),
                    ];
                    invoke("CheckDlgButton", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(4));
                    let stored = u64::from(
                        (({
                            let numerator = (((((values[6]).wrapping_sub(float_to_integer(
                                f64::from_bits(
                                    ((f64::from_bits(
                                        ((f64::from_bits(
                                            ((u64::from(
                                                (u64::from(
                                                    memory
                                                        .read_u32((values[0]).wrapping_add(
                                                            (0x3u64).wrapping_mul(4),
                                                        ))?,
                                                ))
                                                    as u32,
                                            )) as f64)
                                                .to_bits(),
                                        )) * (f64::from_bits(memory.read_u64(
                                            (values[0]).wrapping_add((0x8u64).wrapping_mul(4)),
                                        )?)))
                                        .to_bits(),
                                    )) / (f64::from_bits(
                                        (-(f64::from_bits(0x4059000000000000u64))).to_bits(),
                                    )))
                                    .to_bits(),
                                ),
                                32,
                                true,
                            )?)) as i32 as i64
                                as u64)
                                .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                                as i32 as i64 as u64)
                                as i32;
                            let denominator = (values[6]) as i32;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(4));
                    let stored = u64::from(
                        (({
                            let numerator = (((((values[7]).wrapping_sub(float_to_integer(
                                f64::from_bits(
                                    ((f64::from_bits(
                                        ((f64::from_bits(
                                            ((u64::from(
                                                (u64::from(
                                                    memory
                                                        .read_u32((values[0]).wrapping_add(
                                                            (0x3u64).wrapping_mul(4),
                                                        ))?,
                                                ))
                                                    as u32,
                                            )) as f64)
                                                .to_bits(),
                                        )) * (f64::from_bits(memory.read_u64(
                                            (values[0]).wrapping_add((0x6u64).wrapping_mul(4)),
                                        )?)))
                                        .to_bits(),
                                    )) / (f64::from_bits(
                                        (-(f64::from_bits(0x4059000000000000u64))).to_bits(),
                                    )))
                                    .to_bits(),
                                ),
                                32,
                                true,
                            )?)) as i32 as i64
                                as u64)
                                .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                                as i32 as i64 as u64)
                                as i32;
                            let denominator = (values[7]) as i32;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 29;
            }
            31 => {
                values[6] = ((((((u64::from(memory.read_u32((values[2]).wrapping_add(0x28u64))?))
                    as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[2]).wrapping_add(0x48u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[2]).wrapping_add(0x40u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                values[7] = ((((((u64::from(memory.read_u32((values[2]).wrapping_add(0x24u64))?))
                    as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[2]).wrapping_add(0x44u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[2]).wrapping_add(0x3cu64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 31;
            }
            33 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    32
                } else {
                    28
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140088cd0` (`mspaint.c:119232`).
pub fn fun_140088cd0(
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
    ];
    let mut pc: i32 = 96;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let destination = (values[2]).wrapping_add(0x1cu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((((values[10]).wrapping_mul(values[5])) as i32 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let destination = (memory
                        .read_u64((memory.read_u64(values[2])?).wrapping_add(0x130u64))?)
                    .wrapping_add(0x32u64);
                    let stored = ((((values[10]).wrapping_mul(values[5])) as i32 as i64 as u64)
                        as i16 as i64 as u64) as i16 as i64 as u64;
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 2;
            }
            4 => {
                values[5] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 3;
            }
            5 => {
                {
                    let destination = values[3];
                    let stored = (values[9]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 4;
            }
            6 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let destination = (values[0]).wrapping_add(0x38u64);
                    let stored = memory.read_u64(relocate("STACK_local_res10"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from(((values[0]).wrapping_add(0x38u64)) != (0x0u64))) != 0 {
                    7
                } else {
                    6
                };
            }
            9 => {
                {
                    let destination = (values[0]).wrapping_add(0x34u64);
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 8;
            }
            10 => {
                values[7] = (({
                    let numerator =
                        (((values[11]).wrapping_sub(values[12])) as i32 as i64 as u64) as i32;
                    let denominator = (0x2u64) as i32;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    9
                };
            }
            12 => {
                {
                    let destination = (values[0]).wrapping_add(0x30u64);
                    let stored = (values[5]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 11;
            }
            13 => {
                values[5] = (0x0u64) as i32 as i64 as u64;
                pc = 12;
            }
            14 => {
                values[5] = (({
                    let numerator =
                        (((values[5]).wrapping_sub(values[8])) as i32 as i64 as u64) as i32;
                    let denominator = (0x2u64) as i32;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 12;
            }
            15 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    14
                };
            }
            16 => {
                pc = if (u64::from(((values[0]).wrapping_add(0x30u64)) != (0x0u64))) != 0 {
                    15
                } else {
                    8
                };
            }
            17 => {
                {
                    let destination = relocate("STACK_local_res10");
                    let stored = (((values[12]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                        | ((values[8]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                values[11] = (((values[11]).wrapping_mul(values[10])) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 17;
            }
            19 => {
                values[5] = (((values[5]).wrapping_mul(values[9])) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 18;
            }
            20 => {
                values[10] = (({
                    let numerator = (((((values[12])
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64)
                        .wrapping_add(values[11])) as i32
                        as i64 as u64) as i32;
                    let denominator = (values[11]) as i32;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 19;
            }
            21 => {
                values[9] = (({
                    let numerator = (((((values[5])
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64)
                        .wrapping_add(values[8])) as i32 as i64
                        as u64) as i32;
                    let denominator = (values[5]) as i32;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        as u64
                }) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 20;
            }
            22 => {
                values[12] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                (((memory.read_u64(relocate("STACK_local_res20"))?)
                                    & (0xffffffffu64)) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[14])))
                            .to_bits(),
                        )) / (f64::from_bits(0x4059000000000000u64)))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 21;
            }
            23 => {
                values[8] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                (((memory.read_u64(relocate("STACK_local_res20"))?)
                                    & (0xffffffffu64)) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[13])))
                            .to_bits(),
                        )) / (f64::from_bits(0x4059000000000000u64)))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                values[12] = (values[11]) as i32 as i64 as u64;
                pc = 17;
            }
            25 => {
                values[8] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits((((values[11]) as i64) as f64).to_bits()))
                                * (f64::from_bits(values[13])))
                            .to_bits(),
                        )) / (f64::from_bits(values[14])))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 24;
            }
            26 => {
                pc = 16;
            }
            27 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        <= (((values[11]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    25
                };
            }
            28 => {
                values[8] = (values[5]) as i32 as i64 as u64;
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_res10");
                    let stored = (((values[12]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                        | ((values[5]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                values[12] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits((((values[5]) as i64) as f64).to_bits()))
                                * (f64::from_bits(values[14])))
                            .to_bits(),
                        )) / (f64::from_bits(values[13])))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 29;
            }
            31 => {
                values[11] = (((values[11]).wrapping_mul(values[10])) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 30;
            }
            32 => {
                values[5] = (((values[5]).wrapping_mul(values[9])) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 31;
            }
            33 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    23
                } else {
                    32
                };
            }
            34 => {
                values[5] = ((u64::from(memory.read_u32((values[0]).wrapping_add(0x40u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 33;
            }
            35 => {
                values[3] = (values[0]).wrapping_add(0x48u64);
                pc = 34;
            }
            36 => {
                {
                    let destination = (values[0]).wrapping_add(0x44u64);
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                values[11] = (((((values[5]).wrapping_sub(
                    (u64::from(memory.read_u32((values[0]).wrapping_add(0x2cu64))?)) as i32 as i64
                        as u64,
                )) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0x24u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                values[5] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0xau64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                {
                    let destination = (values[0]).wrapping_add(0x40u64);
                    let stored = (((((values[5]).wrapping_sub(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0x28u64))?)) as i32
                            as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_sub(
                            (u64::from(memory.read_u32((values[0]).wrapping_add(0x20u64))?)) as i32
                                as i64 as u64,
                        )) as i32 as i64 as u64) as i32 as i64
                        as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 38;
            }
            40 => {
                values[5] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x8u64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                values[14] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x1cu64))?)) as i32
                            as i64 as u64) as i64) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(values[14])))
                    .to_bits(),
                )) / (f64::from_bits((((values[5]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 40;
            }
            42 => {
                values[13] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x18u64))?)) as i32
                            as i64 as u64) as i64) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(values[13])))
                    .to_bits(),
                )) / (f64::from_bits((((values[5]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = (((u64::from(
                        (u64::from(
                            memory
                                .read_u32((relocate("STACK_local_res20")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("DAT_1401d08b4"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                values[6] =
                    (u64::from((u64::from(memory.read_u8(relocate("DAT_1401d08a8"))?)) as u8))
                        as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                values[4] = ((u64::from(memory.read_u32(relocate("DAT_1401d08ac"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 44;
            }
            46 => {
                values[8] =
                    (u64::from((u64::from(memory.read_u8(relocate("DAT_1401d08b0"))?)) as u8))
                        as i32 as i64 as u64;
                pc = 45;
            }
            47 => {
                values[9] =
                    (u64::from((u64::from(memory.read_u8(relocate("DAT_1401d08b8"))?)) as u8))
                        as i32 as i64 as u64;
                pc = 46;
            }
            48 => {
                values[10] = ((u64::from(memory.read_u32(relocate("DAT_1401d08bc"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 47;
            }
            49 => {
                {
                    let call_arguments = [relocate("STACK_local_res20")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_res20"))?) != (0x0u64)))
                    != 0
                {
                    49
                } else {
                    48
                };
            }
            51 => {
                values[5] = ({
                    let call_arguments = [relocate("STACK_local_res20")];
                    invoke("FUN_140068ea0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b78"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b78"))?) != (0x0u64))) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = memory.read_u64(relocate("DAT_1401d0b78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[0]).wrapping_add(0x24u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = (values[0]).wrapping_add(0x2cu64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                pc = if (u64::from(
                    ((((memory.read_u64(relocate("STACK_local_res20"))?) as i32 as i64 as u64)
                        as i32 as i64 as u64) as i64)
                        <= (((((values[11]).wrapping_add(values[5])) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64),
                )) != 0
                {
                    56
                } else {
                    54
                };
            }
            58 => {
                {
                    let destination = (values[0]).wrapping_add(0x20u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = (values[0]).wrapping_add(0x28u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 58;
            }
            60 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        <= (((((values[8]).wrapping_add(values[6])) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64),
                )) != 0
                {
                    59
                } else {
                    57
                };
            }
            61 => {
                {
                    let destination = (values[0]).wrapping_add(0x2cu64);
                    let stored = (values[5]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 60;
            }
            62 => {
                values[5] = (values[10]) as i32 as i64 as u64;
                pc = 61;
            }
            63 => {
                pc = if (u64::from(
                    (((0x0u64) as i32 as i64 as u64) as i64)
                        < (((values[10]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    62
                } else {
                    61
                };
            }
            64 => {
                values[5] = (0x0u64) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                values[10] = ((((((((memory.read_u64(relocate("STACK_local_res20"))?) as i32 as i64
                    as u64)
                    .wrapping_sub(float_to_integer(
                        f64::from_bits(
                            ((f64::from_bits(
                                ((f64::from_bits(
                                    ((((u64::from(memory.read_u32(relocate("DAT_1401d08a4"))?))
                                        as i32 as i64 as u64)
                                        as i64) as f64)
                                        .to_bits(),
                                )) * (f64::from_bits(values[14])))
                                .to_bits(),
                            )) / (f64::from_bits(
                                (-(f64::from_bits(0x40f86a0000000000u64))).to_bits(),
                            )))
                            .to_bits(),
                        ),
                        32,
                        true,
                    )?)) as i32 as i64 as u64)
                    .wrapping_sub(values[9])) as i32 as i64 as u64)
                    .wrapping_add(values[10])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 64;
            }
            66 => {
                {
                    let destination = (values[0]).wrapping_add(0x28u64);
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 65;
            }
            67 => {
                values[8] = (values[5]) as i32 as i64 as u64;
                pc = 66;
            }
            68 => {
                pc = if (u64::from(
                    (((0x0u64) as i32 as i64 as u64) as i64)
                        < (((values[5]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    67
                } else {
                    66
                };
            }
            69 => {
                values[8] = (0x0u64) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                values[5] = (((((((values[4]).wrapping_sub(float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((u64::from(
                                    (u64::from(memory.read_u8(relocate("DAT_1401d08a0"))?)) as u8,
                                )) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[13])))
                            .to_bits(),
                        )) / (f64::from_bits(
                            (-(f64::from_bits(0x40f86a0000000000u64))).to_bits(),
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)) as i32 as i64 as u64)
                    .wrapping_sub(values[8])) as i32 as i64 as u64)
                    .wrapping_add(values[5])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 69;
            }
            71 => {
                {
                    let destination = (values[0]).wrapping_add(0x24u64);
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                values[11] = (values[12]) as i32 as i64 as u64;
                pc = 71;
            }
            73 => {
                pc = if (u64::from(
                    (((0x0u64) as i32 as i64 as u64) as i64)
                        < (((values[12]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    72
                } else {
                    71
                };
            }
            74 => {
                values[11] = (0x0u64) as i32 as i64 as u64;
                pc = 73;
            }
            75 => {
                values[12] = ((((0u64.wrapping_sub(float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((((u64::from(memory.read_u32(relocate("DAT_1401d089c"))?)) as i32
                                    as i64 as u64) as i64) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[14])))
                            .to_bits(),
                        )) / (f64::from_bits(
                            (-(f64::from_bits(0x40f86a0000000000u64))).to_bits(),
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)) as i32 as i64 as u64)
                    .wrapping_sub(values[10])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 74;
            }
            76 => {
                {
                    let destination = (values[0]).wrapping_add(0x20u64);
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                values[6] = (values[11]) as i32 as i64 as u64;
                pc = 76;
            }
            78 => {
                pc = if (u64::from(
                    (((0x0u64) as i32 as i64 as u64) as i64)
                        < (((values[11]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    77
                } else {
                    76
                };
            }
            79 => {
                values[6] = (0x0u64) as i32 as i64 as u64;
                pc = 78;
            }
            80 => {
                values[7] = (0x0u64) as i32 as i64 as u64;
                pc = 79;
            }
            81 => {
                values[11] = ((((0u64.wrapping_sub(float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((((u64::from(memory.read_u32(relocate("DAT_1401d0898"))?)) as i32
                                    as i64 as u64) as i64) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[13])))
                            .to_bits(),
                        )) / (f64::from_bits(
                            (-(f64::from_bits(0x40f86a0000000000u64))).to_bits(),
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?)) as i32 as i64 as u64)
                    .wrapping_sub(values[5])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 80;
            }
            82 => {
                values[14] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((((memory.read_u64(relocate("STACK_local_res20"))?) as i32 as i64 as u64)
                            as i64) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(0x408f400000000000u64)))
                    .to_bits(),
                )) / (f64::from_bits((((values[7]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 81;
            }
            83 => {
                values[13] = ((f64::from_bits(
                    ((f64::from_bits((((values[4]) as i64) as f64).to_bits()))
                        * (f64::from_bits(0x408f400000000000u64)))
                    .to_bits(),
                )) / (f64::from_bits((((values[6]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 82;
            }
            84 => {
                values[10] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x71u64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 83;
            }
            85 => {
                values[5] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x70u64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 84;
            }
            86 => {
                values[9] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x6fu64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 85;
            }
            87 => {
                values[8] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x6eu64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 86;
            }
            88 => {
                values[7] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x6u64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 87;
            }
            89 => {
                values[6] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x4u64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = (((u64::from(
                        (u64::from(
                            memory
                                .read_u32((relocate("STACK_local_res20")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((values[5]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                values[5] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0xau64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 90;
            }
            92 => {
                values[4] = ({
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add(0x10u64))?, 0x8u64];
                    invoke("GetDeviceCaps", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 91;
            }
            93 => {
                return Ok(None);
            }
            94 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[1]).wrapping_add(0x8u64))?) as i64)
                        == ((0x0u64) as i64),
                )) != 0
                {
                    93
                } else {
                    92
                };
            }
            95 => {
                return Ok(None);
            }
            96 => {
                pc = if (u64::from(((values[1]) as i64) == ((0x0u64) as i64))) != 0 {
                    95
                } else {
                    94
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400966e0` (`mspaint.c:129937`).
pub fn fun_1400966e0(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64, 0u64];
    let mut pc: i32 = 25;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let call_arguments = [(memory.read_u64(
                        (memory.read_u64((values[4]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                            .wrapping_add(0x40u64),
                    )?)
                    .wrapping_add(0x1e0u64)];
                    invoke("FUN_1400a2fa0", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((memory.read_u64(
                        (memory.read_u64((values[4]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                            .wrapping_add(0x40u64),
                    )?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                values[4] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[2]).wrapping_add(0xccu64);
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(
                                ((u64::from((memory.read_u64(relocate("STACK_local_48"))?) as u32)
                                    as u32)
                                    .wrapping_shr(0x3u64 as u32)
                                    as u64) as u32,
                            )) as u8,
                        )) & (0x1u64)) as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[2]).wrapping_add(0xd0u64);
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_iStack_2c"))?))
                        as i32 as i64 as u64)
                        & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | (((u64::from(memory.read_u32(relocate("STACK_local_30"))?)) as i32 as i64
                            as u64)
                            & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0xb0u64))?,
                        values[3],
                        relocate("STACK_local_30"),
                        relocate("STACK_local_50"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    7
                } else {
                    0
                };
            }
            9 => {
                values[3] = memory.read_u64((values[2]).wrapping_add(0x40u64))?;
                pc = 8;
            }
            10 => {
                values[2] = memory.read_u64(values[3])?;
                pc = 9;
            }
            11 => {
                values[3] = relocate("DAT_00000030");
                pc = 10;
            }
            12 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    11
                } else {
                    10
                };
            }
            13 => {
                values[3] = (values[0]).wrapping_add(0x10u64);
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_iStack_2c");
                    let stored = ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_58")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (relocate("STACK_local_58")).wrapping_add(0x4u64);
                    let stored = float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                (((values[1]).wrapping_shr(0x20u64 as u32)) as f32).to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        false,
                    )?;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((memory.read_u64(relocate("STACK_local_58"))?) as f32).to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (relocate("STACK_local_58")).wrapping_add(0x0u64);
                    let stored = float_to_integer(
                        (f32::from_bits((u64::from(((values[1]) as f32).to_bits())) as u32)) as f64,
                        32,
                        false,
                    )?;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = relocate("STACK_local_28");
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_uStack_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x3f800000u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0xffffffffffffffffu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("DAT_1401d07c6");
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140097710` (`mspaint.c:131005`).
pub fn fun_140097710(
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
    ];
    let mut pc: i32 = 102;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x18u64),
                        values[7],
                        memory.read_u64((values[0]).wrapping_add(0x30u64))?,
                        values[11],
                    ];
                    invoke("FUN_140006a00", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = relocate("STACK_local_50");
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = relocate("STACK_local_60");
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = relocate("STACK_local_70");
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_64");
                    let stored = u64::from(
                        (u64::from(
                            memory.read_u32((values[2]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = u64::from(
                        (u64::from(
                            memory.read_u32((values[2]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_uStack_58");
                    let stored =
                        memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = memory.read_u64(values[10])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                values[10] = {
                    let call_arguments = [relocate("STACK_local_b0")];
                    invoke("ToD2D", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    12
                } else {
                    11
                };
            }
            14 => {
                {
                    let destination = relocate("STACK_uStack_44");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[9]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[9]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_uStack_4c");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[9]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[9])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 16;
            }
            18 => {
                values[9] = {
                    let call_arguments = [
                        relocate("STACK_local_b0"),
                        (memory.read_u64(relocate("STACK_local_90"))?) & (0xffffffffu64),
                    ];
                    invoke("ToD2D", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_c8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            20 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_88"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_88"))?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x85au64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_90"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_c8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            27 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    25
                };
            }
            28 => {
                values[4] = memory.read_u64(relocate("STACK_local_88"))?;
                pc = 27;
            }
            29 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x38u64))?)?)
                            .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x38u64))?,
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = (relocate("STACK_local_c8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x4493u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[8])?).wrapping_add(0x8u64))?,
                        values[8],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    37
                };
            }
            40 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 39;
            }
            41 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                values[6] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 41;
            }
            43 => {
                values[3] = (values[8]).wrapping_add(0xcu64);
                pc = 42;
            }
            44 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[8])?)?, values[8]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    45
                } else {
                    37
                };
            }
            47 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 46;
            }
            48 => {
                {
                    let destination = values[4];
                    let stored = ((((memory.read_u64(values[4])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 47;
            }
            49 => {
                values[5] = memory.read_u64(values[4])?;
                pc = 48;
            }
            50 => {
                values[4] = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 49;
            }
            51 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 50;
            }
            52 => {
                pc = if (u64::from((values[8]) != (0x0u64))) != 0 {
                    51
                } else {
                    37
                };
            }
            53 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 52;
            }
            54 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_98"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    53
                } else {
                    52
                };
            }
            55 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x2a8u64))?,
                        values[4],
                        memory.read_u64(values[10])?,
                        relocate("STACK_local_c8"),
                        0x3f800000u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_e8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x1u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                values[10] = {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_b0"))?];
                    invoke("BasePaint::Image::D2DBitmap", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                values[8] = memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 60;
            }
            62 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = memory.read_u64(values[10])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 62;
            }
            64 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 63;
            }
            65 => {
                values[3] = (memory
                    .read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                .wrapping_add(0x8u64);
                pc = 64;
            }
            66 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 65;
            }
            67 => {
                pc = if (u64::from(
                    (memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    66
                } else {
                    62
                };
            }
            68 => {
                values[10] = relocate("DAT_00000040");
                pc = 67;
            }
            69 => {
                pc = if (u64::from(((memory.read_u64(values[8])?) as i64) == ((0x0u64) as i64)))
                    != 0
                {
                    68
                } else {
                    67
                };
            }
            70 => {
                values[10] = (memory.read_u64(values[8])?).wrapping_add(0x30u64);
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                values[4] = memory.read_u64(values[7])?;
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_74");
                    let stored = u64::from(
                        ((f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[2]).wrapping_add((0x3u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        )) + (f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_7c"))?)) as u32,
                            )) as u32,
                        )))
                        .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = u64::from(
                        ((f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[2]).wrapping_add((0x2u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        )) + (f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_80"))?)) as u32,
                            )) as u32,
                        )))
                        .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored =
                        memory.read_u64((values[2]).wrapping_add((0x2u64).wrapping_mul(4)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = u64::from((u64::from(memory.read_u32(values[2])?)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_7c");
                    let stored = u64::from(
                        (u64::from(
                            memory.read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 76;
            }
            78 => {
                values[8] = {
                    let call_arguments = [
                        relocate("STACK_local_90"),
                        relocate("STACK_local_88"),
                        0x0u64,
                    ];
                    invoke("FUN_1400072f0", &call_arguments, memory)
                };
                pc = 77;
            }
            79 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_b0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            80 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    79
                } else {
                    78
                };
            }
            81 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x30u64))?,
                        values[4],
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_b0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x4481u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[11]).wrapping_add((0x9u64).wrapping_mul(8)))?)
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    85
                } else {
                    33
                };
            }
            87 => {
                values[11] = 0x0u64;
                pc = 86;
            }
            88 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    87
                } else {
                    86
                };
            }
            89 => {
                values[11] = (values[4]).wrapping_add(
                    ((0u64.wrapping_sub(0x2u64)) as i32 as i64 as u64).wrapping_mul(8),
                );
                pc = 88;
            }
            90 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x38u64))?;
                pc = 89;
            }
            91 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 90;
            }
            92 => {
                values[7] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_90"))?)?)
                            .wrapping_add(0x18u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 91;
            }
            93 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_b0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            94 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    93
                } else {
                    92
                };
            }
            95 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[7])?)?,
                        values[7],
                        relocate("DAT_14011b4e8"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x140105690u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_b0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((0x84bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    98
                } else {
                    92
                };
            }
            100 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                values[7] = memory.read_u64(values[1])?;
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_108"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140098760` (`mspaint.c:131895`).
pub fn fun_140098760(
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
    let mut pc: i32 = 30;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_18"))?)?)
                                .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_18"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[5] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[2] = (memory.read_u64(relocate("STACK_local_18"))?).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_18"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_18"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[3];
                    let stored = ((((memory.read_u64(values[3])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64(values[3])?;
                pc = 11;
            }
            13 => {
                values[3] = (memory.read_u64(relocate("STACK_local_18"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_18"))?) != (0x0u64))) != 0
                {
                    14
                } else {
                    0
                };
            }
            16 => {
                {
                    let call_arguments = [values[1]];
                    invoke("FromNative", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [memory.read_u64(values[6])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                values[6] = {
                    let call_arguments = [values[0], relocate("STACK_local_20")];
                    invoke("FUN_140099480", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let destination = (values[1]).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_20")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            20 => {
                {
                    let destination = (values[1]).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from(
                                        ((((u64::from(
                                            memory.read_u32((values[0]).wrapping_add(0x74u64))?,
                                        )) as i32 as i64
                                            as u64)
                                            as i64)
                                            as f32)
                                            .to_bits(),
                                    )) as u32,
                                )) * (f32::from_bits(
                                    (u64::from(
                                        ((memory.read_u64(relocate("STACK_local_20"))?) as f32)
                                            .to_bits(),
                                    )) as u32,
                                )))
                                .to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_38")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            22 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    20
                };
            }
            23 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x60u64))?,
                        values[3],
                        relocate("STACK_local_20"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x53a7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                pc = if (u64::from((values[3]) == (0x0u64))) != 0 {
                    18
                } else {
                    27
                };
            }
            29 => {
                values[3] = memory.read_u64((values[0]).wrapping_add(0x68u64))?;
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_58"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140099920` (`mspaint.c:132730`).
pub fn fun_140099920(
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
                {
                    let call_arguments = [values[4], values[3]];
                    invoke("FUN_140099560", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[4] = 0x50u64;
                pc = 1;
            }
            3 => {
                values[3] = 0x68u64;
                pc = 2;
            }
            4 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    3
                } else {
                    1
                };
            }
            5 => {
                values[4] = (values[0]).wrapping_add(0x40u64);
                pc = 4;
            }
            6 => {
                values[3] = (values[0]).wrapping_add(0x58u64);
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [values[3], relocate("PTR_DAT_14011b7b0")];
                    invoke("FUN_140099c90", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                values[3] = 0x18u64;
                pc = 7;
            }
            9 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    8
                } else {
                    7
                };
            }
            10 => {
                values[3] = (values[0]).wrapping_add(0x8u64);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[2];
                    let stored = values[1];
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    (f32::from_bits(
                        (u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32,
                    )) != (f32::from_bits((values[1]) as u32)),
                )) != 0
                {
                    11
                } else {
                    0
                };
            }
            13 => {
                values[2] = relocate("DAT_00000078");
                pc = 12;
            }
            14 => {
                pc = if (u64::from(((values[0]) as i64) == ((0x0u64) as i64))) != 0 {
                    13
                } else {
                    12
                };
            }
            15 => {
                values[2] = (values[0]).wrapping_add(0x68u64);
                pc = 14;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140099e50` (`mspaint.c:133061`).
pub fn fun_140099e50(
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
    ];
    let mut pc: i32 = 65;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x18u64),
                        values[7],
                        memory.read_u64((values[0]).wrapping_add(0x30u64))?,
                        values[11],
                    ];
                    invoke("FUN_140006a00", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = relocate("STACK_local_50");
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = relocate("STACK_local_60");
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = relocate("STACK_local_70");
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_64");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_54");
                    let stored = u64::from(
                        ((((u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x4u64).wrapping_mul(1)))?,
                        )) as i32 as i64 as u64) as i64) as f32)
                            .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = u64::from(
                        ((((u64::from(
                            memory.read_u32((values[10]).wrapping_add((0x0u64).wrapping_mul(1)))?,
                        )) as i32 as i64 as u64) as i64) as f32)
                            .to_bits(),
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?)
                                .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_78"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                values[6] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 15;
            }
            17 => {
                values[4] = (memory.read_u64(relocate("STACK_local_78"))?).wrapping_add(0xcu64);
                pc = 16;
            }
            18 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_78"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    11
                };
            }
            21 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            22 => {
                {
                    let destination = values[3];
                    let stored = ((((memory.read_u64(values[3])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                values[5] = memory.read_u64(values[3])?;
                pc = 22;
            }
            24 => {
                values[3] = (memory.read_u64(relocate("STACK_local_78"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 23;
            }
            25 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 24;
            }
            26 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    25
                } else {
                    11
                };
            }
            27 => {
                values[10] = {
                    let call_arguments = [memory.read_u64(values[9])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                values[9] = {
                    let call_arguments = [values[0], relocate("STACK_local_80")];
                    invoke("FUN_14009a0b0", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                values[11] = 0x0u64;
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x38u64))?) as i64)
                        == ((0x0u64) as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[11] = (memory.read_u64((values[0]).wrapping_add(0x38u64))?)
                    .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [relocate("STACK_local_80")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    32
                } else {
                    31
                };
            }
            34 => {
                {
                    let destination = relocate("STACK_uStack_44");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[8]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[8]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_uStack_4c");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[8]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[8])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 36;
            }
            38 => {
                values[8] = {
                    let call_arguments = [
                        relocate("STACK_local_a0"),
                        (memory.read_u64(relocate("STACK_local_88"))?) & (0xffffffffu64),
                    ];
                    invoke("ToD2D", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_b8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            40 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_80"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_80"))?,
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x85au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_88"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_b8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            47 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    45
                };
            }
            48 => {
                values[3] = memory.read_u64(relocate("STACK_local_80"))?;
                pc = 47;
            }
            49 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x38u64))?)?)
                            .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x38u64))?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x4493u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 53;
            }
            55 => {
                values[7] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_88"))?)?)
                            .wrapping_add(0x18u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_a0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            57 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    56
                } else {
                    55
                };
            }
            58 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[9])?)?,
                        values[9],
                        relocate("DAT_14011b4e8"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination =
                        (relocate("STACK_local_a0")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 60;
            }
            62 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    61
                } else {
                    55
                };
            }
            63 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                values[9] = memory.read_u64(values[1])?;
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_f8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400a6030` (`mspaint.c:142453`).
pub fn fun_1400a6030(
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
    ];
    let mut pc: i32 = 36;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x8u64))?,
                        values[5],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[7] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[3] = (values[5]).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_30"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_30"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 11;
            }
            13 => {
                values[2] = (memory.read_u64(relocate("STACK_plStack_30"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_30"))?) != (0x0u64)))
                    != 0
                {
                    14
                } else {
                    0
                };
            }
            16 => {
                values[5] = memory.read_u64(relocate("STACK_plStack_30"))?;
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_38"))?,
                        relocate("STACK_local_60"),
                    ];
                    invoke("CAttribute::SetValue", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = memory.read_u64(values[8])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[8] = values[9];
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (f64::from_bits(values[1]))
                        <= (f64::from_bits(memory.read_u64(relocate("STACK_local_40"))?)),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[8] = relocate("STACK_local_40");
                pc = 20;
            }
            22 => {
                values[9] = relocate("STACK_local_68");
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (f64::from_bits(memory.read_u64(relocate("STACK_local_48"))?))
                        <= (f64::from_bits(values[1])),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[9] = relocate("STACK_local_48");
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = relocate("CAttributeValue<double,5>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = u64::from((0x5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                pc = if (values[6]) != 0 { 26 } else { 16 };
            }
            28 => {
                values[6] = u64::from(
                    ({
                        let call_arguments = [
                            relocate("DAT_1401d0b30"),
                            values[7],
                            relocate("STACK_local_48"),
                            relocate("STACK_local_40"),
                        ];
                        invoke("CAttributeStore::GetRange", &call_arguments, memory)
                    }) != 0,
                );
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0b30"),
                        (relocate("STACK_local_38")) as i32 as i64 as u64,
                    ];
                    invoke("CAttributeStore::Get", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_plStack_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    0
                };
            }
            33 => {
                values[7] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                            .wrapping_add(0xe0u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64))) != 0
                {
                    34
                } else {
                    0
                };
            }
            36 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_88"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400a62c0` (`mspaint.c:142588`).
pub fn fun_1400a62c0(
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
    ];
    let mut pc: i32 = 36;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x8u64))?,
                        values[5],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[7] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[3] = (values[5]).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_30"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_30"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((values[4]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 11;
            }
            13 => {
                values[2] = (memory.read_u64(relocate("STACK_plStack_30"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_30"))?) != (0x0u64)))
                    != 0
                {
                    14
                } else {
                    0
                };
            }
            16 => {
                values[5] = memory.read_u64(relocate("STACK_plStack_30"))?;
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_38"))?,
                        relocate("STACK_local_60"),
                    ];
                    invoke("CAttribute::SetValue", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = memory.read_u64(values[8])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[8] = values[9];
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (f64::from_bits(values[1]))
                        <= (f64::from_bits(memory.read_u64(relocate("STACK_local_40"))?)),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[8] = relocate("STACK_local_40");
                pc = 20;
            }
            22 => {
                values[9] = relocate("STACK_local_68");
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (f64::from_bits(memory.read_u64(relocate("STACK_local_48"))?))
                        <= (f64::from_bits(values[1])),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[9] = relocate("STACK_local_48");
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = relocate("CAttributeValue<double,5>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = u64::from((0x5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                pc = if (values[6]) != 0 { 26 } else { 16 };
            }
            28 => {
                values[6] = u64::from(
                    ({
                        let call_arguments = [
                            relocate("DAT_1401d0b30"),
                            values[7],
                            relocate("STACK_local_48"),
                            relocate("STACK_local_40"),
                        ];
                        invoke("CAttributeStore::GetRange", &call_arguments, memory)
                    }) != 0,
                );
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0b30"),
                        (relocate("STACK_local_38")) as i32 as i64 as u64,
                    ];
                    invoke("CAttributeStore::Get", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_plStack_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    0
                };
            }
            33 => {
                values[7] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                            .wrapping_add(0xd8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64))) != 0
                {
                    34
                } else {
                    0
                };
            }
            36 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_88"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400a6590` (`mspaint.c:142736`).
pub fn fun_1400a6590(
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
        (arguments[1]) as i32 as i64 as u64,
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
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                        values[4],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = values[8];
                    let stored = ((((u64::from(memory.read_u32(values[8])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[6] = ((u64::from(memory.read_u32(values[8])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[8] = (values[4]).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_18"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_18"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[3] = memory.read_u64(values[2])?;
                pc = 11;
            }
            13 => {
                values[2] = (memory.read_u64(relocate("STACK_plStack_18"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_18"))?) != (0x0u64)))
                    != 0
                {
                    14
                } else {
                    0
                };
            }
            16 => {
                values[4] = memory.read_u64(relocate("STACK_plStack_18"))?;
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_20"))?,
                        relocate("STACK_local_48"),
                    ];
                    invoke("CAttribute::SetValue", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = ((u64::from(memory.read_u32(values[7])?)) as i32 as i64 as u64)
                        as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                values[7] = values[8];
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        <= (((float_to_integer(
                            f64::from_bits(memory.read_u64(relocate("STACK_local_30"))?),
                            32,
                            true,
                        )?) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[7] = relocate("STACK_local_58");
                pc = 20;
            }
            22 => {
                values[8] = (relocate("STACK_local_58")).wrapping_add((0x2u64).wrapping_mul(4));
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (((float_to_integer(
                        f64::from_bits(memory.read_u64(relocate("STACK_local_28"))?),
                        32,
                        true,
                    )?) as i32 as i64 as u64) as i64)
                        <= (((values[1]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[8] = (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(4));
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("CAttributeValue<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = u64::from((0x3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = (float_to_integer(
                        f64::from_bits(memory.read_u64(relocate("STACK_local_28"))?),
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = (float_to_integer(
                        f64::from_bits(memory.read_u64(relocate("STACK_local_30"))?),
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                pc = if (values[5]) != 0 { 28 } else { 16 };
            }
            30 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [
                            relocate("DAT_1401d0b30"),
                            values[6],
                            relocate("STACK_local_28"),
                            relocate("STACK_local_30"),
                        ];
                        invoke("CAttributeStore::GetRange", &call_arguments, memory)
                    }) != 0,
                );
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0b30"),
                        (relocate("STACK_local_20")) as i32 as i64 as u64,
                    ];
                    invoke("CAttributeStore::Get", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_plStack_18");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                                .wrapping_add(0x108u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64))) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    0
                };
            }
            37 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                                .wrapping_add(0xd0u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x0u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = (values[1]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64))) != 0
                {
                    38
                } else {
                    0
                };
            }
            40 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400a6700` (`mspaint.c:142815`).
pub fn fun_1400a6700(
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
        (arguments[1]) as i32 as i64 as u64,
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
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                        values[4],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 2;
            }
            4 => {
                {
                    let destination = values[8];
                    let stored = ((((u64::from(memory.read_u32(values[8])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[6] = ((u64::from(memory.read_u32(values[8])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[8] = (values[4]).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_18"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_18"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((values[3]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    0
                };
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[3] = memory.read_u64(values[2])?;
                pc = 11;
            }
            13 => {
                values[2] = (memory.read_u64(relocate("STACK_plStack_18"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_18"))?) != (0x0u64)))
                    != 0
                {
                    14
                } else {
                    0
                };
            }
            16 => {
                values[4] = memory.read_u64(relocate("STACK_plStack_18"))?;
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_20"))?,
                        relocate("STACK_local_48"),
                    ];
                    invoke("CAttribute::SetValue", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = ((u64::from(memory.read_u32(values[7])?)) as i32 as i64 as u64)
                        as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                values[7] = values[8];
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        <= (((float_to_integer(
                            f64::from_bits(memory.read_u64(relocate("STACK_local_30"))?),
                            32,
                            true,
                        )?) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[7] = relocate("STACK_local_58");
                pc = 20;
            }
            22 => {
                values[8] = (relocate("STACK_local_58")).wrapping_add((0x2u64).wrapping_mul(4));
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (((float_to_integer(
                        f64::from_bits(memory.read_u64(relocate("STACK_local_28"))?),
                        32,
                        true,
                    )?) as i32 as i64 as u64) as i64)
                        <= (((values[1]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[8] = (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(4));
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("CAttributeValue<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = u64::from((0x3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = (float_to_integer(
                        f64::from_bits(memory.read_u64(relocate("STACK_local_28"))?),
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = (float_to_integer(
                        f64::from_bits(memory.read_u64(relocate("STACK_local_30"))?),
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                pc = if (values[5]) != 0 { 28 } else { 16 };
            }
            30 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [
                            relocate("DAT_1401d0b30"),
                            values[6],
                            relocate("STACK_local_28"),
                            relocate("STACK_local_30"),
                        ];
                        invoke("CAttributeStore::GetRange", &call_arguments, memory)
                    }) != 0,
                );
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0b30"),
                        (relocate("STACK_local_20")) as i32 as i64 as u64,
                    ];
                    invoke("CAttributeStore::Get", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_plStack_18");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    0
                };
            }
            35 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                                .wrapping_add(0xd0u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64))) != 0
                {
                    35
                } else {
                    0
                };
            }
            37 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                                .wrapping_add(0x110u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d06e0"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = (values[1]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64))) != 0
                {
                    38
                } else {
                    0
                };
            }
            40 => {
                {
                    let destination = relocate("STACK_local_10");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400c9d50` (`mspaint.c:174860`).
pub fn fun_1400c9d50(
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
    ];
    let mut pc: i32 = 197;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x260u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x92u64))?)) as i16
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
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140008c50", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = 5;
            }
            7 => {
                pc = 5;
            }
            8 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x13u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_90"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x14u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_98"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    14
                } else {
                    13
                };
            }
            16 => {
                pc = if (u64::from(((values[4]) as i64) == ((0x1u64) as i64))) != 0 {
                    21
                } else {
                    15
                };
            }
            17 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 16;
            }
            18 => {
                {
                    let destination = values[12];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[4] = memory.read_u64(values[12])?;
                pc = 18;
            }
            20 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = values[12];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            23 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 22;
            }
            24 => {
                {
                    let destination = values[12];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                values[4] = memory.read_u64(values[12])?;
                pc = 24;
            }
            26 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 25;
            }
            27 => {
                values[12] =
                    memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                pc = 26;
            }
            28 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    27
                } else {
                    15
                };
            }
            29 => {
                pc = 5;
            }
            30 => {
                return Ok(None);
            }
            31 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                values[9] = ({
                    let call_arguments = [];
                    invoke("FUN_14000c570", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 31;
            }
            33 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = (values[0]).wrapping_add((0x26u64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x13u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_90"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x14u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_98"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [
                        values[4],
                        memory.read_u64((values[0]).wrapping_add((0x2au64).wrapping_mul(8)))?,
                    ];
                    invoke("FUN_140095700", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                {
                    let destination = (values[0]).wrapping_add((0x22u64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (values[0]).wrapping_add(0xecu64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[0]).wrapping_add(0x124u64))?))
                                as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[0]).wrapping_add((0x1du64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = relocate("FUN_140052460");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = (values[4]).wrapping_add(0xa1u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 47;
            }
            49 => {
                values[4] = memory.read_u64((values[0]).wrapping_add((0x4au64).wrapping_mul(8)))?;
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    50
                } else {
                    49
                };
            }
            52 => {
                pc = if (u64::from(((values[4]) as i64) == ((0x1u64) as i64))) != 0 {
                    57
                } else {
                    51
                };
            }
            53 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 52;
            }
            54 => {
                {
                    let destination = values[12];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                values[4] = memory.read_u64(values[12])?;
                pc = 54;
            }
            56 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = values[12];
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            59 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 58;
            }
            60 => {
                {
                    let destination = values[12];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                values[4] = memory.read_u64(values[12])?;
                pc = 60;
            }
            62 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 61;
            }
            63 => {
                values[12] =
                    memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                pc = 62;
            }
            64 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    63
                } else {
                    51
                };
            }
            65 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x17u64).wrapping_mul(8))];
                    invoke("FUN_14000c240", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                pc = 5;
            }
            67 => {
                pc = 35;
            }
            68 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x13u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_90"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    68
                } else {
                    67
                };
            }
            70 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x14u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 70;
            }
            72 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_98"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    71
                } else {
                    70
                };
            }
            73 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = (values[4]).wrapping_add(0xa1u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x25u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 74;
            }
            76 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x25u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    75
                } else {
                    74
                };
            }
            77 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[4]).wrapping_add(0x30u64))?,
                        (values[0]).wrapping_add((0x25u64).wrapping_mul(8)),
                        0x25au64,
                    ];
                    invoke("FUN_1400512b0", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_88"))?)) as i8 as i64 as u64)
                        as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    77
                } else {
                    73
                };
            }
            79 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = ((u64::from(
                        memory.read_u8((values[0]).wrapping_add((0x37u64).wrapping_mul(8)))?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x38u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            81 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    80
                } else {
                    79
                };
            }
            82 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x2fu64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        (values[0]).wrapping_add((0x37u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = (values[0]).wrapping_add((0x3au64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[0]).wrapping_add((0x39u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Input.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = (values[0]).wrapping_add((0x38u64).wrapping_mul(8));
                    let stored = u64::from((0x11ebu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = (values[0]).wrapping_add((0x37u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 86;
            }
            88 => {
                pc = 41;
            }
            89 => {
                pc = 65;
            }
            90 => {
                return Ok(None);
            }
            91 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    90
                } else {
                    89
                };
            }
            92 => {
                values[9] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add((0x17u64).wrapping_mul(8)),
                        values[0],
                    ];
                    invoke("FUN_14000c540", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 91;
            }
            93 => {
                {
                    let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(8));
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = (values[0]).wrapping_add((0x1au64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = (values[0]).wrapping_add((0x18u64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let call_arguments = [relocate("STACK_local_d0"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            99 => {
                {
                    let call_arguments = [relocate("STACK_local_d0")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                {
                    let destination = (values[0]).wrapping_add((0x49u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = (values[0]).wrapping_add((0x48u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = (values[0]).wrapping_add((0x47u64).wrapping_mul(8));
                    let stored = u64::from((0x1628u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 101;
            }
            103 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    102
                } else {
                    97
                };
            }
            104 => {
                values[11] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[12])?).wrapping_add(0xc0u64))?,
                        values[12],
                        (values[0]).wrapping_add((0x16u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 103;
            }
            105 => {
                values[12] =
                    (memory.read_u64((values[4]).wrapping_add(0x30u64))?).wrapping_add(0x8u64);
                pc = 104;
            }
            106 => {
                pc = if (u64::from(
                    ((u64::from(!((values[7]) != 0))) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u8((values[4]).wrapping_add(0xa1u64))?)) as i8
                                as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    105
                } else {
                    88
                };
            }
            107 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 106;
            }
            108 => {
                pc = if (u64::from(
                    ((values[8]) != 0)
                        && ((u64::from(
                            (memory
                                .read_u64((values[0]).wrapping_add((0x15u64).wrapping_mul(8)))?)
                                != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    107
                } else {
                    106
                };
            }
            109 => {
                values[7] = 0u64;
                pc = 108;
            }
            110 => {
                pc = 128;
            }
            111 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_88"))?)) as i8 as i64 as u64)
                        as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    110
                } else {
                    109
                };
            }
            112 => {
                values[7] = 1u64;
                pc = 111;
            }
            113 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = ((u64::from(
                        memory.read_u8((values[0]).wrapping_add((0x43u64).wrapping_mul(8)))?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x44u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            115 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    114
                } else {
                    113
                };
            }
            116 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[12])?).wrapping_add(0x90u64))?,
                        values[12],
                        (values[0]).wrapping_add((0x43u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 115;
            }
            117 => {
                {
                    let destination = (values[0]).wrapping_add((0x46u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = (values[0]).wrapping_add((0x45u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = (values[0]).wrapping_add((0x44u64).wrapping_mul(8));
                    let stored = u64::from((0x76bbu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = (values[0]).wrapping_add((0x43u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = u64::from((0x7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 120;
            }
            122 => {
                values[8] = 1u64;
                pc = 121;
            }
            123 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[12])?).wrapping_add(0x8u64))?,
                        values[12],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 122;
            }
            124 => {
                pc = if (u64::from((values[12]) != (0x0u64))) != 0 {
                    123
                } else {
                    122
                };
            }
            125 => {
                {
                    let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(8));
                    let stored = values[12];
                    memory.write_u64(destination, stored)?;
                }
                pc = 124;
            }
            126 => {
                values[12] = memory.read_u64((values[4]).wrapping_add(0x78u64))?;
                pc = 125;
            }
            127 => {
                values[7] = 1u64;
                pc = 108;
            }
            128 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 127;
            }
            129 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_87"))?)) as i8 as i64 as u64)
                        as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    126
                } else {
                    128
                };
            }
            130 => {
                values[7] = 0u64;
                pc = 129;
            }
            131 => {
                {
                    let destination = relocate("STACK_local_87");
                    let stored = ((u64::from(
                        memory.read_u8((values[0]).wrapping_add((0x3fu64).wrapping_mul(8)))?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 130;
            }
            132 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x40u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            133 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    132
                } else {
                    131
                };
            }
            134 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[5])?)?).wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        (values[0]).wrapping_add((0x3fu64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 133;
            }
            135 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = memory.read_u64(values[5])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 134;
            }
            136 => {
                {
                    let destination = (values[0]).wrapping_add((0x42u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let destination = (values[0]).wrapping_add((0x41u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Input.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let destination = (values[0]).wrapping_add((0x40u64).wrapping_mul(8));
                    let stored = u64::from((0x11ebu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let destination = (values[0]).wrapping_add((0x3fu64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 138;
            }
            140 => {
                pc = if (u64::from(
                    (f64::from_bits(0x0u64))
                        < ((f32::from_bits(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_80"))?)) as u32,
                            )) as u32,
                        )) as f64),
                )) != 0
                {
                    139
                } else {
                    87
                };
            }
            141 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x3bu64).wrapping_mul(8)))?,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x3cu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            143 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    142
                } else {
                    141
                };
            }
            144 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[5])?)?).wrapping_add(0xb0u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        (values[0]).wrapping_add((0x3bu64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = memory.read_u64(values[5])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                {
                    let destination = (values[0]).wrapping_add((0x3eu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 145;
            }
            147 => {
                {
                    let destination = (values[0]).wrapping_add((0x3du64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Input.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = (values[0]).wrapping_add((0x3cu64).wrapping_mul(8));
                    let stored = u64::from((0x12f9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = (values[0]).wrapping_add((0x3bu64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 148;
            }
            150 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x25au64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    149
                } else {
                    87
                };
            }
            151 => {
                values[10] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64((values[4]).wrapping_add(0x30u64))?).wrapping_add(0x40u64),
                    )?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 150;
            }
            152 => {
                {
                    let destination = (values[0]).wrapping_add((0x4au64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 151;
            }
            153 => {
                values[4] = memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                pc = 152;
            }
            154 => {
                {
                    let destination = (memory
                        .read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?)
                    .wrapping_add(0xa1u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 73;
            }
            155 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_84"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    153
                } else {
                    154
                };
            }
            156 => {
                {
                    let destination = relocate("STACK_local_84");
                    let stored = ((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0x33u64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 155;
            }
            157 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x34u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            158 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    157
                } else {
                    156
                };
            }
            159 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[12])?)?).wrapping_add(0x40u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_90"))?,
                        (values[0]).wrapping_add((0x33u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 158;
            }
            160 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(values[12])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 159;
            }
            161 => {
                {
                    let destination = (values[0]).wrapping_add((0x36u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 160;
            }
            162 => {
                {
                    let destination = (values[0]).wrapping_add((0x35u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Input.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 161;
            }
            163 => {
                {
                    let destination = (values[0]).wrapping_add((0x34u64).wrapping_mul(8));
                    let stored = u64::from((0x116du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 162;
            }
            164 => {
                {
                    let destination = (values[0]).wrapping_add((0x33u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 163;
            }
            165 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = u64::from((0x6u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 164;
            }
            166 => {
                {
                    let destination = (values[0]).wrapping_add((0x14u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[5])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 165;
            }
            167 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x30u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            168 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    167
                } else {
                    166
                };
            }
            169 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[12])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_90"))?,
                        values[5],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 168;
            }
            170 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(values[12])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 169;
            }
            171 => {
                {
                    let destination = (values[0]).wrapping_add((0x32u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                {
                    let destination = (values[0]).wrapping_add((0x31u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Input.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 171;
            }
            173 => {
                {
                    let destination = (values[0]).wrapping_add((0x30u64).wrapping_mul(8));
                    let stored = u64::from((0x11a3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 172;
            }
            174 => {
                {
                    let destination = values[5];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                values[5] = (values[0]).wrapping_add((0x2fu64).wrapping_mul(8));
                pc = 174;
            }
            176 => {
                {
                    let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[12])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x2cu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            178 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    176
                };
            }
            179 => {
                values[10] = ({
                    let call_arguments =
                        [
                            memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x38u64))?,
                            values[5],
                            values[12],
                            values[2],
                            values[3],
                            u64::from(
                                (u64::from(memory.read_u16(
                                    (values[0]).wrapping_add((0x12u64).wrapping_mul(8)),
                                )?)) as u16,
                            ),
                            values[0],
                        ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 178;
            }
            180 => {
                {
                    let destination = (values[0]).wrapping_add((0x2eu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 179;
            }
            181 => {
                {
                    let destination = (values[0]).wrapping_add((0x2du64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 180;
            }
            182 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = u64::from((0x5329u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 181;
            }
            183 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?;
                pc = 182;
            }
            184 => {
                {
                    let destination = values[12];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                values[12] = (values[0]).wrapping_add((0x28u64).wrapping_mul(8));
                pc = 184;
            }
            186 => {
                {
                    let destination = (values[0]).wrapping_add((0x2au64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x11u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 185;
            }
            187 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 186;
            }
            188 => {
                return Ok(None);
            }
            189 => {
                {
                    let call_arguments = [values[6]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 188;
            }
            190 => {
                values[6] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 189;
            }
            191 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    190
                };
            }
            192 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    191
                };
            }
            193 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    192
                };
            }
            194 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    65
                } else {
                    193
                };
            }
            195 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    66
                } else {
                    194
                };
            }
            196 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    187
                } else {
                    195
                };
            }
            197 => {
                values[13] = (u64::from(
                    (u64::from(
                        memory.read_u16((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?,
                    )) as u16,
                )) as i32 as i64 as u64;
                pc = 196;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L176423` (`mspaint.c:176423`).
/// Original symbol: `ceilf`.
pub fn named_l176423(
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
    let mut values = [u64::from((arguments[0]) as u32), 0u64];
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                values[1] = {
                    let call_arguments = [values[0]];
                    invoke("ceilf", &call_arguments, memory)
                };
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L176436` (`mspaint.c:176436`).
/// Original symbol: `floor`.
pub fn named_l176436(
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
                return Ok(Some(values[1]));
            }
            1 => {
                values[1] = {
                    let call_arguments = [values[0]];
                    invoke("floor", &call_arguments, memory)
                };
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L176449` (`mspaint.c:176449`).
/// Original symbol: `floorf`.
pub fn named_l176449(
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
    let mut values = [u64::from((arguments[0]) as u32), 0u64];
    let mut pc: i32 = 1;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                values[1] = {
                    let call_arguments = [values[0]];
                    invoke("floorf", &call_arguments, memory)
                };
                pc = 0;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `NAMED_L176462` (`mspaint.c:176462`).
/// Original symbol: `fmod`.
pub fn named_l176462(
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
                    invoke("fmod", &call_arguments, memory)
                };
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
    ("FUN_14000b550", 2, fun_14000b550),
    ("FUN_1400165d0", 1, fun_1400165d0),
    ("FUN_140016ec0", 1, fun_140016ec0),
    ("FUN_1400170c0", 2, fun_1400170c0),
    ("FUN_140017a60", 3, fun_140017a60),
    ("FUN_1400349f0", 3, fun_1400349f0),
    ("FUN_14003d1f0", 3, fun_14003d1f0),
    ("FUN_140040ed0", 2, fun_140040ed0),
    ("FUN_140041c90", 3, fun_140041c90),
    ("FUN_140051e90", 1, fun_140051e90),
    ("FUN_140052570", 4, fun_140052570),
    ("FUN_140052790", 3, fun_140052790),
    ("FUN_140053f20", 3, fun_140053f20),
    ("FUN_1400884c0", 4, fun_1400884c0),
    ("FUN_1400888e0", 4, fun_1400888e0),
    ("FUN_140088cd0", 3, fun_140088cd0),
    ("FUN_1400966e0", 2, fun_1400966e0),
    ("FUN_140097710", 3, fun_140097710),
    ("FUN_140098760", 2, fun_140098760),
    ("FUN_140099920", 2, fun_140099920),
    ("FUN_140099e50", 3, fun_140099e50),
    ("FUN_1400a6030", 2, fun_1400a6030),
    ("FUN_1400a62c0", 2, fun_1400a62c0),
    ("FUN_1400a6590", 2, fun_1400a6590),
    ("FUN_1400a6700", 2, fun_1400a6700),
    ("FUN_1400c9d50", 4, fun_1400c9d50),
    ("NAMED_L176423", 1, named_l176423),
    ("NAMED_L176436", 1, named_l176436),
    ("NAMED_L176449", 1, named_l176449),
    ("NAMED_L176462", 2, named_l176462),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_1400165d0",
        &[
            ("STACK_auStack_118", -280),
            ("STACK_local_f8", -248),
            ("STACK_local_e8", -232),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_a8", -168),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_uStack_80", -128),
            ("STACK_local_78", -120),
            ("STACK_uStack_70", -112),
            ("STACK_local_68", -104),
            ("STACK_uStack_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140016ec0",
        &[
            ("STACK_auStack_d8", -216),
            ("STACK_local_b0", -176),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_uStack_68", -104),
            ("STACK_local_60", -96),
            ("STACK_uStack_58", -88),
            ("STACK_local_50", -80),
            ("STACK_uStack_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_1400170c0",
        &[
            ("STACK_auStack_c8", -200),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_uStack_58", -88),
            ("STACK_local_50", -80),
            ("STACK_uStack_48", -72),
            ("STACK_local_40", -64),
            ("STACK_uStack_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140017a60",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_1400349f0",
        &[
            ("STACK_auStack_a8", -168),
            ("STACK_local_88", -136),
            ("STACK_local_84", -132),
            ("STACK_local_80", -128),
            ("STACK_local_7c", -124),
            ("STACK_local_78", -120),
            ("STACK_local_74", -116),
            ("STACK_local_70", -112),
            ("STACK_local_6c", -108),
            ("STACK_local_68", -104),
        ],
    ),
    (
        "FUN_14003d1f0",
        &[
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140040ed0",
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
        "FUN_140051e90",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140052570",
        &[("STACK_local_50", -80), ("STACK_local_48", -72)],
    ),
    (
        "FUN_140052790",
        &[("STACK_local_58", -88), ("STACK_local_48", -72)],
    ),
    (
        "FUN_140053f20",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    ("FUN_1400884c0", &[("STACK_local_50", -80)]),
    (
        "FUN_140088cd0",
        &[("STACK_local_res10", 16), ("STACK_local_res20", 32)],
    ),
    (
        "FUN_1400966e0",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_uStack_38", -56),
            ("STACK_local_30", -48),
            ("STACK_iStack_2c", -44),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140097710",
        &[
            ("STACK_auStack_108", -264),
            ("STACK_local_e8", -232),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_c0", -192),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_7c", -124),
            ("STACK_local_78", -120),
            ("STACK_local_74", -116),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_64", -100),
            ("STACK_local_60", -96),
            ("STACK_uStack_58", -88),
            ("STACK_local_50", -80),
            ("STACK_uStack_4c", -76),
            ("STACK_uStack_48", -72),
            ("STACK_uStack_44", -68),
            ("STACK_local_40", -64),
        ],
    ),
    (
        "FUN_140098760",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140099e50",
        &[
            ("STACK_auStack_f8", -248),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_c0", -192),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_64", -100),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_54", -84),
            ("STACK_local_50", -80),
            ("STACK_uStack_4c", -76),
            ("STACK_uStack_48", -72),
            ("STACK_uStack_44", -68),
            ("STACK_local_40", -64),
        ],
    ),
    (
        "FUN_1400a6030",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_plStack_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400a62c0",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_plStack_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400a6590",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_plStack_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400a6700",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_plStack_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400c9d50",
        &[
            ("STACK_local_d0", -208),
            ("STACK_local_b8", -184),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_87", -135),
            ("STACK_local_84", -132),
            ("STACK_local_80", -128),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_1400165d0",
        &[
            ("STACK_auStack_118", 32),
            ("STACK_local_f8", 8),
            ("STACK_local_e8", 8),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 1),
            ("STACK_local_a8", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_uStack_80", 8),
            ("STACK_local_78", 8),
            ("STACK_uStack_70", 8),
            ("STACK_local_68", 8),
            ("STACK_uStack_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 16),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140016ec0",
        &[
            ("STACK_auStack_d8", 40),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_uStack_68", 8),
            ("STACK_local_60", 8),
            ("STACK_uStack_58", 8),
            ("STACK_local_50", 8),
            ("STACK_uStack_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_1400170c0",
        &[
            ("STACK_auStack_c8", 40),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_uStack_58", 8),
            ("STACK_local_50", 8),
            ("STACK_uStack_48", 8),
            ("STACK_local_40", 8),
            ("STACK_uStack_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140017a60",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_1400349f0",
        &[
            ("STACK_auStack_a8", 32),
            ("STACK_local_88", 4),
            ("STACK_local_84", 4),
            ("STACK_local_80", 4),
            ("STACK_local_7c", 4),
            ("STACK_local_78", 4),
            ("STACK_local_74", 4),
            ("STACK_local_70", 4),
            ("STACK_local_6c", 4),
            ("STACK_local_68", 8),
        ],
    ),
    (
        "FUN_14003d1f0",
        &[
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140040ed0",
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
        "FUN_140051e90",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140052570",
        &[("STACK_local_50", 8), ("STACK_local_48", 8)],
    ),
    (
        "FUN_140052790",
        &[("STACK_local_58", 16), ("STACK_local_48", 8)],
    ),
    (
        "FUN_140053f20",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    ("FUN_1400884c0", &[("STACK_local_50", 40)]),
    (
        "FUN_140088cd0",
        &[("STACK_local_res10", 8), ("STACK_local_res20", 8)],
    ),
    (
        "FUN_1400966e0",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_uStack_38", 8),
            ("STACK_local_30", 4),
            ("STACK_iStack_2c", 4),
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140097710",
        &[
            ("STACK_auStack_108", 32),
            ("STACK_local_e8", 8),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 1),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 4),
            ("STACK_local_7c", 4),
            ("STACK_local_78", 4),
            ("STACK_local_74", 4),
            ("STACK_local_70", 8),
            ("STACK_local_68", 4),
            ("STACK_local_64", 4),
            ("STACK_local_60", 8),
            ("STACK_uStack_58", 8),
            ("STACK_local_50", 4),
            ("STACK_uStack_4c", 4),
            ("STACK_uStack_48", 4),
            ("STACK_uStack_44", 4),
            ("STACK_local_40", 8),
        ],
    ),
    (
        "FUN_140098760",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140099e50",
        &[
            ("STACK_auStack_f8", 32),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 1),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 4),
            ("STACK_local_64", 4),
            ("STACK_local_60", 8),
            ("STACK_local_58", 4),
            ("STACK_local_54", 4),
            ("STACK_local_50", 4),
            ("STACK_uStack_4c", 4),
            ("STACK_uStack_48", 4),
            ("STACK_uStack_44", 4),
            ("STACK_local_40", 8),
        ],
    ),
    (
        "FUN_1400a6030",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 4),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_plStack_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400a62c0",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 4),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_plStack_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400a6590",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 16),
            ("STACK_local_48", 8),
            ("STACK_local_40", 4),
            ("STACK_local_38", 4),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_plStack_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400a6700",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 16),
            ("STACK_local_48", 8),
            ("STACK_local_40", 4),
            ("STACK_local_38", 4),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_plStack_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400c9d50",
        &[
            ("STACK_local_d0", 24),
            ("STACK_local_b8", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 1),
            ("STACK_local_87", 1),
            ("STACK_local_84", 4),
            ("STACK_local_80", 4),
        ],
    ),
];
