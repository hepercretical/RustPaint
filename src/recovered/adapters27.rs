//! Checked translations generated from original bodies by batch 027.
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
/// Translation of `FUN_140023b70` (`mspaint.c:35207`).
pub fn fun_140023b70(
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
        0u64,
    ];
    let mut pc: i32 = 187;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[9])?).wrapping_add(0x8u64))?,
                        values[9],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
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
                values[11] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[3] = (values[9]).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[9])?)?, values[9]];
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
                    let destination = values[6];
                    let stored = ((((memory.read_u64(values[6])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64(values[6])?;
                pc = 11;
            }
            13 => {
                values[6] = (values[9]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    14
                } else {
                    0
                };
            }
            16 => {
                {
                    let call_arguments = [values[13], 0x0u64, values[14]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                values[13] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            19 => {
                pc = if (u64::from(
                    (((((values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    15
                };
            }
            20 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    19
                };
            }
            21 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            22 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                values[11] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 22;
            }
            24 => {
                values[5] = (values[14]).wrapping_add((0x3u64).wrapping_mul(8));
                pc = 23;
            }
            25 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 24;
            }
            26 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    25
                } else {
                    15
                };
            }
            27 => {
                {
                    let call_arguments = [relocate("STACK_local_80")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [values[8]];
                    invoke("FUN_1400236d0", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_ppuStack_a0");
                    let stored =
                        memory.read_u64((values[12]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = memory.read_u64(values[12])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[12]).wrapping_add((0x3u64).wrapping_mul(8)))?),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = values[12];
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                values[12] = {
                    let call_arguments = [
                        relocate("STACK_local_80"),
                        relocate("STACK_local_a8"),
                        relocate("STACK_local_48"),
                    ];
                    invoke("FUN_140024910", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = relocate("STACK_local_c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_ppuStack_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = relocate("DAT_140105708");
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_ppuStack_a0");
                    let stored = u64::from(
                        (u64::from(memory.read_u32(
                            (memory.read_u64(relocate("STACK_local_88"))?).wrapping_add(0x4u64),
                        )?)) as u32,
                    );
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = memory.read_u64(
                        (memory.read_u64(relocate("STACK_local_88"))?)
                            .wrapping_add((0x2u64).wrapping_mul(8)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) == (0x0u64))) != 0
                {
                    37
                } else {
                    39
                };
            }
            41 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = 0x1000000000000000u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_60"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                values[14] = memory.read_u64(relocate("STACK_local_88"))?;
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(relocate("STACK_local_48"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        relocate("STACK_local_48"),
                        relocate("STACK_local_c8"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 46;
            }
            48 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(relocate("STACK_local_48"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            54 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_c8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            58 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    57
                } else {
                    56
                };
            }
            59 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_80"))?,
                        relocate("STACK_local_48"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (relocate("STACK_local_c8")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((relocate("STACK_local_80")).wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    52
                } else {
                    64
                };
            }
            66 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 65;
            }
            67 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("L\"TransferFromDeviceStatus\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_74");
                    let stored = u64::from((0x18u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(relocate("STACK_local_48"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            75 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 76;
            }
            78 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_c8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            79 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    78
                } else {
                    77
                };
            }
            80 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_80"))?,
                        relocate("STACK_local_48"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = (relocate("STACK_local_c8")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((relocate("STACK_local_80")).wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                pc = 52;
            }
            87 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    86
                } else {
                    85
                };
            }
            88 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 87;
            }
            89 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 89;
            }
            91 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("L\"ProcessingDataStatus\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = relocate("STACK_local_74");
                    let stored = u64::from((0x14u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(relocate("STACK_local_c8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            97 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_c8"),
                        relocate("STACK_local_48"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 98;
            }
            100 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(relocate("STACK_local_48"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            102 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 103;
            }
            105 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_a8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            106 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    105
                } else {
                    104
                };
            }
            107 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_80"))?,
                        relocate("STACK_local_48"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 106;
            }
            108 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("STACK_ppuStack_a0");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_a8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    100
                } else {
                    111
                };
            }
            113 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("STACK_local_80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("L\"TransferToClientStatus\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = relocate("STACK_local_74");
                    let stored = u64::from((0x16u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 120;
            }
            122 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    121
                } else {
                    43
                };
            }
            123 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    95
                } else {
                    122
                };
            }
            124 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    73
                } else {
                    123
                };
            }
            125 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 124;
            }
            126 => {
                values[4] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 125;
            }
            127 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 126;
            }
            128 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    127
                } else {
                    126
                };
            }
            129 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 128;
            }
            130 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    129
                } else {
                    128
                };
            }
            131 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_c8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            132 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    131
                } else {
                    130
                };
            }
            133 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0x38u64))?,
                        values[6],
                        values[14],
                        relocate("STACK_local_38"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 132;
            }
            134 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Dispatching.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 134;
            }
            136 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x3eu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                values[6] = memory.read_u64(values[8])?;
                pc = 136;
            }
            138 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let call_arguments = [relocate("STACK_ppuStack_a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 138;
            }
            140 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_ppuStack_a0"))?) != (0x0u64)))
                    != 0
                {
                    139
                } else {
                    138
                };
            }
            141 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            142 => {
                values[14] = memory.read_u64(relocate("STACK_local_c8"))?;
                pc = 141;
            }
            143 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_c8"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 142;
            }
            144 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 144;
            }
            146 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 145;
            }
            147 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_c8"))?)
                        .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_ppuStack_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_c8"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_a8")).wrapping_add(0x0u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 148;
            }
            150 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_c8"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 149;
            }
            151 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_c8"))?) != (0x0u64))) != 0
                {
                    150
                } else {
                    141
                };
            }
            152 => {
                values[7] = memory.read_u64(relocate("STACK_ppuStack_a0"))?;
                pc = 151;
            }
            153 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = relocate("STACK_local_a8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 153;
            }
            155 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(values[5])?).wrapping_add((0x2u64).wrapping_mul(4)),
                        )?,
                        values[5],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 154;
            }
            156 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    155
                } else {
                    154
                };
            }
            157 => {
                {
                    let destination = relocate("STACK_ppuStack_a0");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_a8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[2]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 157;
            }
            159 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(values[5])?).wrapping_add((0x2u64).wrapping_mul(4)),
                        )?,
                        values[5],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 158;
            }
            160 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    159
                } else {
                    158
                };
            }
            161 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 160;
            }
            162 => {
                values[5] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_58"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8)),
                )?;
                pc = 161;
            }
            163 => {
                pc = 15;
            }
            164 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) == (0x0u64))) != 0
                {
                    163
                } else {
                    162
                };
            }
            165 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = u64::from((values[2]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 164;
            }
            166 => {
                values[8] = memory.read_u64(relocate("STACK_local_58"))?;
                pc = 165;
            }
            167 => {
                values[9] = memory.read_u64(relocate("STACK_plStack_50"))?;
                pc = 166;
            }
            168 => {
                {
                    let destination = relocate("STACK_plStack_50");
                    let stored = memory.read_u64((values[0]).wrapping_add(0x30u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = memory.read_u64((values[0]).wrapping_add(0x28u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                pc = if (u64::from(!((values[15]) != 0))) != 0 {
                    180
                } else {
                    169
                };
            }
            171 => {
                values[11] = (values[10]) as i32 as i64 as u64;
                pc = 170;
            }
            172 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 171;
            }
            173 => {
                values[10] = (values[11]) as i32 as i64 as u64;
                pc = 172;
            }
            174 => {
                {
                    let destination = (values[4]).wrapping_add(0x8u64);
                    let stored = (((values[11]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32
                        as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 173;
            }
            175 => {
                pc = if (values[15]) != 0 { 174 } else { 172 };
            }
            176 => {
                values[15] = u64::from(
                    (u64::from(
                        (((values[11]) as i32 as i64 as u64) as i64)
                            == (((values[10]) as i32 as i64 as u64) as i64),
                    )) != 0,
                );
                pc = 175;
            }
            177 => {
                values[10] = ((u64::from(memory.read_u32((values[4]).wrapping_add(0x8u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 176;
            }
            178 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 177;
            }
            179 => {
                pc = 167;
            }
            180 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    179
                } else {
                    178
                };
            }
            181 => {
                values[11] = ((u64::from(memory.read_u32((values[4]).wrapping_add(0x8u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 180;
            }
            182 => {
                pc = if (u64::from(((values[4]) as i64) != ((0x0u64) as i64))) != 0 {
                    181
                } else {
                    167
                };
            }
            183 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x30u64))?;
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("STACK_plStack_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 184;
            }
            186 => {
                values[14] = 0x0u64;
                pc = 185;
            }
            187 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_e8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 186;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400275a0` (`mspaint.c:38122`).
pub fn fun_1400275a0(
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
                return Ok(Some(memory.read_u64(relocate("STACK_local_6d8"))?));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_5e8")];
                    invoke("FUN_14000f600", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_6d8"))?,
                        values[1],
                        (u64::from(memory.read_u32(relocate("STACK_local_5f4"))?)) as i32 as i64
                            as u64,
                        values[3],
                    ];
                    invoke("FUN_14002b700", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_6a8"),
                        memory.read_u64(relocate("STACK_local_6d8"))?,
                        values[1],
                    ];
                    invoke("FUN_140027c30", &call_arguments, memory)
                };
                pc = 1;
            }
            4 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f5"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    3
                };
            }
            5 => {
                {
                    let destination = relocate("STACK_local_678");
                    let stored = relocate("STACK_local_628");
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_688");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_638");
                    let stored = relocate("STACK_local_5f0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_640");
                    let stored = relocate("STACK_local_620");
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_648");
                    let stored = relocate("STACK_local_5f7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_650");
                    let stored = relocate("STACK_local_618");
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_658");
                    let stored = relocate("STACK_local_600");
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_660");
                    let stored = relocate("STACK_local_5ec");
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_668");
                    let stored = relocate("STACK_local_5e8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_670");
                    let stored = relocate("STACK_local_608");
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_680");
                    let stored = relocate("STACK_PARAM_param_5");
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_690");
                    let stored = relocate("STACK_local_5f4");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_698");
                    let stored = relocate("STACK_local_5f5");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_6a0");
                    let stored = relocate("STACK_local_5f6");
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_6a8");
                    let stored = relocate("STACK_local_5f8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_5f5");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_5f5");
                    let stored = (0x1u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 19;
            }
            22 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xdu64))?)) as i8
                                as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            || ((u64::from(
                                ((((u64::from(memory.read_u8((values[3]).wrapping_add(0x9u64))?))
                                    as i8 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        || ((values[26]) != 0),
                )) != 0
                {
                    20
                } else {
                    21
                };
            }
            23 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(values[13])) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (values[10]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(values[18])?)) as i8 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        != (((values[24]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    24
                };
            }
            26 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = values[18];
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                values[18] = (values[18]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                );
                pc = 26;
            }
            28 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[12]) as i32 as i64 as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xbu64))?))
                                    as i8 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xcu64))?)) as i8
                                as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    27
                } else {
                    24
                };
            }
            29 => {
                pc = 24;
            }
            30 => {
                pc = if ({
                    let _ = {
                        values[10] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?))
                            as i32 as i64 as u64) as i32 as i64
                            as u64;
                        values[10]
                    };
                    u64::from((values[17]) < (memory.read_u64(relocate("STACK_local_618"))?))
                }) != 0
                {
                    36
                } else {
                    29
                };
            }
            31 => {
                let _ = {
                    values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[17]
                };
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (values[13]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                values[13] =
                    (((values[13]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 32;
            }
            34 => {
                pc = 29;
            }
            35 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        != (((0x2eu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    34
                } else {
                    31
                };
            }
            36 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        == (((0x30u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    35
                };
            }
            37 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (values[18]) == (memory.read_u64(relocate("STACK_local_618"))?),
                        )) != 0)
                            && ((u64::from(
                                ((f32::from_bits(
                                    (u64::from(((values[2]) & (0x7fffffffu64)) as u32)) as u32,
                                )) as f64)
                                    < (f64::from_bits(0x3ff0000000000000u64)),
                            )) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    let destination = relocate("STACK_local_5f0");
                                    let stored = (values[13]) as i32 as i64 as u64;
                                    memory.write_u32(destination, stored as u32)?;
                                    stored
                                };
                                {
                                    values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                                    values[17]
                                }
                            };
                            u64::from(
                                ((f32::from_bits((values[2]) as u32)) as f64)
                                    != (f64::from_bits(0x0u64)),
                            )
                        }) != 0),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            38 => {
                values[10] = (values[13]) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                values[13] = (((((values[12]).wrapping_sub(values[20])) as i32 as i64 as u64)
                    .wrapping_add(values[11])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 38;
            }
            40 => {
                values[20] = (((values[20])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                pc = if (u64::from(
                    (((values[19]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    39
                };
            }
            42 => {
                values[20] = ((((values[18]) as i32 as i64 as u64).wrapping_sub(
                    (memory.read_u64(relocate("STACK_local_628"))?) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xbu64))?)) as i8
                            as i64 as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((u64::from(
                                ((((u64::from(memory.read_u8((values[3]).wrapping_add(0x8u64))?))
                                    as i8 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    == (((0x67u64) as i32 as i64 as u64) as i64),
                            )) != 0)
                                || ((u64::from(
                                    ((((u64::from(
                                        memory.read_u8((values[3]).wrapping_add(0x8u64))?,
                                    )) as i8 as i64 as u64)
                                        as i32 as i64 as u64)
                                        as i64)
                                        == (((0x47u64) as i32 as i64 as u64) as i64),
                                )) != 0),
                        )) != 0),
                )) != 0
                {
                    42
                } else {
                    29
                };
            }
            44 => {
                values[10] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                values[13] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    (((values[25]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    45
                } else {
                    28
                };
            }
            47 => {
                pc = 24;
            }
            48 => {
                pc = if (u64::from(
                    (((values[25]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                pc = if (u64::from(
                    (((values[25]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    28
                };
            }
            50 => {
                values[10] = (values[12]) as i32 as i64 as u64;
                pc = 49;
            }
            51 => {
                values[13] = (values[12]) as i32 as i64 as u64;
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_5ec");
                    let stored = (values[20]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                values[19] = ((u64::from(memory.read_u8(relocate("STACK_local_5f7"))?)) as i8 as i64
                    as u64) as i8 as i64 as u64;
                pc = 52;
            }
            54 => {
                values[18] = memory.read_u64(relocate("STACK_local_620"))?;
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(values[20])) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                pc = if (u64::from((values[22]) < (values[21]))) != 0 {
                    61
                } else {
                    55
                };
            }
            57 => {
                values[22] = (u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64;
                pc = 56;
            }
            58 => {
                values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    ((values[17]).wrapping_add((0x1u64).wrapping_mul(1))) != (values[18]),
                )) != 0
                {
                    58
                } else {
                    57
                };
            }
            60 => {
                values[20] =
                    (((values[20]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                values[21] = (values[21]).wrapping_sub((values[22]) as i8 as i64 as u64);
                pc = 60;
            }
            62 => {
                values[18] = (values[17])
                    .wrapping_add((memory.read_u64(relocate("STACK_local_5d8"))?).wrapping_mul(1));
                pc = 61;
            }
            63 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((memory.read_u64(relocate("STACK_local_5d8"))?) as i64)
                            != ((0x0u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[22] =
                                    (u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64;
                                values[22]
                            };
                            u64::from((values[22]) < (values[21]))
                        }) != 0),
                )) != 0
                {
                    62
                } else {
                    55
                };
            }
            64 => {
                values[21] = (memory.read_u64(relocate("STACK_local_608"))?)
                    .wrapping_sub(memory.read_u64(relocate("STACK_local_628"))?);
                pc = 63;
            }
            65 => {
                values[17] = memory.read_u64(relocate("STACK_local_5e8"))?;
                pc = 64;
            }
            66 => {
                pc = if (u64::from((0xfu64) < (memory.read_u64(relocate("STACK_uStack_5d0"))?)))
                    != 0
                {
                    65
                } else {
                    64
                };
            }
            67 => {
                values[17] = relocate("STACK_local_5e8");
                pc = 66;
            }
            68 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[16])?)?,
                        values[16],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("STACK_uStack_6e0"))?) != (0x0u64)))
                        != 0)
                        && (({
                            let _ = {
                                values[16] = {
                                    let call_arguments = [memory.read_u64(
                                        (memory.read_u64(
                                            memory.read_u64(relocate("STACK_uStack_6e0"))?,
                                        )?)
                                        .wrapping_add(0x10u64),
                                    )?];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                };
                                values[16]
                            };
                            u64::from((values[16]) != (0x0u64))
                        }) != 0),
                )) != 0
                {
                    68
                } else {
                    67
                };
            }
            70 => {
                {
                    let call_arguments = [relocate("STACK_local_6d0")];
                    invoke("FUN_14000f600", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_6d0");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_6d0"))?)) as u32,
                            )) & (0xffffff00u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_uStack_6b8");
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_6c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = relocate("STACK_uStack_5d0");
                    let stored = memory.read_u64(relocate("STACK_uStack_6b8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_5d8");
                    let stored = memory.read_u64(relocate("STACK_local_6c0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_uStack_5e0");
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_6c4"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_uStack_6c8"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_5e8");
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_6cc"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_6d0"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 77;
            }
            79 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            80 => {
                pc = if (u64::from(
                    ((u64::from(
                        (0xfffu64)
                            < ((memory.read_u64(relocate("STACK_uStack_5d0"))?)
                                .wrapping_add(0x1u64)),
                    )) != 0)
                        && ((u64::from(
                            (0x1fu64)
                                < ((memory.read_u64(relocate("STACK_local_5e8"))?).wrapping_add(
                                    ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                        .wrapping_sub(
                                            memory.read_u64(
                                                (memory.read_u64(relocate("STACK_local_5e8"))?)
                                                    .wrapping_add(
                                                        (0u64.wrapping_sub(0x8u64)) as i32 as i64
                                                            as u64,
                                                    ),
                                            )?,
                                        ),
                                )),
                        )) != 0),
                )) != 0
                {
                    79
                } else {
                    78
                };
            }
            81 => {
                pc = if (u64::from((0xfu64) < (memory.read_u64(relocate("STACK_uStack_5d0"))?)))
                    != 0
                {
                    80
                } else {
                    77
                };
            }
            82 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[15])?).wrapping_add(0x28u64))?,
                        values[15],
                        relocate("STACK_local_6d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 81;
            }
            83 => {
                values[15] = {
                    let call_arguments = [relocate("STACK_local_6e8")];
                    invoke("FUN_140031bf0", &call_arguments, memory)
                };
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_uStack_6e0");
                    let stored = {
                        let call_arguments = [0x1u64];
                        invoke("std::locale::_Init", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_uStack_6e0"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 83;
            }
            86 => {
                {
                    let destination = relocate("STACK_uStack_6e0");
                    let stored = memory.read_u64(
                        (memory.read_u64(relocate("STACK_PARAM_param_5"))?).wrapping_add(0x8u64),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_PARAM_param_5"))?) as i64)
                        == ((0x0u64) as i64),
                )) != 0
                {
                    84
                } else {
                    86
                };
            }
            88 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xcu64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    87
                } else {
                    51
                };
            }
            89 => {
                {
                    let destination = relocate("STACK_local_5f7");
                    let stored = (0x1u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 88;
            }
            90 => {
                values[19] = (0x1u64) as i8 as i64 as u64;
                pc = 89;
            }
            91 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 90;
            }
            92 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xbu64))?)) as i8
                            as i64 as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (values[8]) == (memory.read_u64(relocate("STACK_local_618"))?),
                        )) != 0),
                )) != 0
                {
                    91
                } else {
                    88
                };
            }
            93 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = values[18];
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                pc = if (u64::from((values[18]) < (values[8]))) != 0 {
                    93
                } else {
                    92
                };
            }
            95 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                pc = if (u64::from((values[17]) < (memory.read_u64(relocate("STACK_local_618"))?)))
                    != 0
                {
                    104
                } else {
                    95
                };
            }
            97 => {
                let _ = {
                    values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[17]
                };
                pc = 96;
            }
            98 => {
                values[8] = values[23];
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_local_600");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                values[18] = values[17];
                pc = 100;
            }
            102 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x2eu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    values[23] = values[8];
                                    values[23]
                                };
                                {
                                    values[9] = memory.read_u64(relocate("STACK_local_600"))?;
                                    values[9]
                                }
                            };
                            u64::from(
                                ((((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64)
                                    as i32 as i64 as u64) as i64)
                                    == (((values[24]) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    101
                } else {
                    99
                };
            }
            103 => {
                values[9] = values[17];
                pc = 102;
            }
            104 => {
                values[23] = values[17];
                pc = 103;
            }
            105 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xbu64))?)) as i8
                            as i64 as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            ((((u64::from(memory.read_u8((values[3]).wrapping_add(0xcu64))?)) as i8
                                as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    96
                } else {
                    51
                };
            }
            106 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = memory.read_u64(relocate("STACK_local_618"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 105;
            }
            107 => {
                values[8] = memory.read_u64(relocate("STACK_local_618"))?;
                pc = 106;
            }
            108 => {
                values[18] = memory.read_u64(relocate("STACK_local_618"))?;
                pc = 107;
            }
            109 => {
                values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                pc = 108;
            }
            110 => {
                pc = 24;
            }
            111 => {
                pc = if (values[26]) != 0 { 110 } else { 109 };
            }
            112 => {
                values[10] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 111;
            }
            113 => {
                {
                    let destination = relocate("STACK_local_600");
                    let stored = memory.read_u64(relocate("STACK_local_618"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = memory.read_u64(relocate("STACK_local_618"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = memory.read_u64(relocate("STACK_local_618"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                values[13] = (0x0u64) as i32 as i64 as u64;
                pc = 115;
            }
            117 => {
                {
                    let destination = relocate("STACK_local_5e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_uStack_5d0");
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("STACK_local_5d8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = relocate("STACK_uStack_5e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_local_5ec");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let destination = relocate("STACK_local_5f7");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 122;
            }
            124 => {
                values[26] = u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                ((float_to_integer(
                                    (f32::from_bits((values[2]) as u32)) as f64,
                                    32,
                                    false,
                                )?) & (0x7f800000u64)) as u32,
                            )) as u32,
                        )) == (u64::from((0x7f800000u64) as u32)),
                    )) != 0,
                );
                pc = 123;
            }
            125 => {
                values[24] = (((values[24])
                    .wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 124;
            }
            126 => {
                pc = if (u64::from((values[17]) != (memory.read_u64(relocate("STACK_local_618"))?)))
                    != 0
                {
                    130
                } else {
                    125
                };
            }
            127 => {
                let _ = {
                    values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[17]
                };
                pc = 126;
            }
            128 => {
                {
                    let destination = values[17];
                    let stored = (((values[5])
                        .wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 127;
            }
            129 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((0x60u64) as i32 as i64 as u64) as i64)
                            < (((values[5]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (((values[5]) as i32 as i64 as u64) as i64)
                                < (((0x7bu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    128
                } else {
                    127
                };
            }
            130 => {
                values[5] = ((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i8
                    as i64 as u64;
                pc = 129;
            }
            131 => {
                pc = if (values[7]) != 0 { 126 } else { 124 };
            }
            132 => {
                values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                pc = 131;
            }
            133 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 132;
            }
            134 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    133
                } else {
                    132
                };
            }
            135 => {
                {
                    let destination = relocate("STACK_local_628");
                    let stored = relocate("STACK_acStack_5c7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 132;
            }
            136 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f6"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    134
                } else {
                    135
                };
            }
            137 => {
                values[19] = (0x0u64) as i8 as i64 as u64;
                pc = 136;
            }
            138 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((memory.read_u64(relocate("STACK_local_618"))?) as i32 as i64
                        as u64)
                        .wrapping_sub(
                            (memory.read_u64(relocate("STACK_local_628"))?) as i32 as i64 as u64,
                        )) as i32 as i64 as u64) as i32 as i64
                        as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let destination = relocate("STACK_local_628");
                    let stored = relocate("STACK_local_5c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let destination = relocate("STACK_local_618");
                    let stored = (memory.read_u64(relocate("STACK_local_618"))?)
                        .wrapping_add((0x3u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_618"))?)
                        .wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = (0x6eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_618"))?)
                        .wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (0x61u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 141;
            }
            143 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_618"))?)
                        .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x6eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = relocate("STACK_local_618");
                    let stored = relocate("STACK_local_5c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 143;
            }
            145 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    144
                } else {
                    143
                };
            }
            146 => {
                {
                    let destination = relocate("STACK_local_618");
                    let stored = relocate("STACK_acStack_5c7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_local_618");
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_6e4"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_6e8"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            148 => {
                {
                    let destination = relocate("STACK_uStack_60c");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[14]).wrapping_add((0x3u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = relocate("STACK_uStack_610");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[14]).wrapping_add((0x2u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 148;
            }
            150 => {
                {
                    let destination = relocate("STACK_uStack_6e4");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[14]).wrapping_add((0x1u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 149;
            }
            151 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[14])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 150;
            }
            152 => {
                values[14] = {
                    let call_arguments = [];
                    invoke("FUN_140031510", &call_arguments, memory)
                };
                pc = 151;
            }
            153 => {
                pc = 157;
            }
            154 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140030420", &call_arguments, memory)
                };
                pc = 153;
            }
            155 => {
                pc = if (u64::from(!((values[26]) != 0))) != 0 {
                    154
                } else {
                    152
                };
            }
            156 => {
                {
                    let destination = relocate("STACK_uStack_60c");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (relocate("STACK_uStack_6e0")).wrapping_add(0x4u64),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 147;
            }
            157 => {
                {
                    let destination = relocate("STACK_uStack_610");
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_uStack_6e0"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_6e8"),
                        relocate("STACK_local_5c8"),
                        relocate("STACK_local_5f"),
                        values[2],
                    ];
                    invoke("FUN_140030050", &call_arguments, memory)
                };
                pc = 157;
            }
            159 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    155
                } else {
                    158
                };
            }
            160 => {
                pc = if ({
                    let call_arguments = [values[2]];
                    invoke("NAN", &call_arguments, memory)
                }) != 0
                {
                    146
                } else {
                    159
                };
            }
            161 => {
                {
                    let destination = relocate("STACK_local_5f6");
                    let stored = (u64::from(
                        (((values[13]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 160;
            }
            162 => {
                values[13] = ({
                    let call_arguments = [values[2]];
                    invoke("_fdsign", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 161;
            }
            163 => {
                {
                    let destination = relocate("STACK_local_6d8");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 162;
            }
            164 => {
                values[12] = (values[13]) as i32 as i64 as u64;
                pc = 163;
            }
            165 => {
                values[11] = (0x432u64) as i32 as i64 as u64;
                pc = 164;
            }
            166 => {
                pc = if (u64::from(
                    (((0x432u64) as i32 as i64 as u64) as i64)
                        < (((values[11]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    165
                } else {
                    163
                };
            }
            167 => {
                values[12] = (0x0u64) as i32 as i64 as u64;
                pc = 166;
            }
            168 => {
                values[13] = (((values[11])
                    .wrapping_add((0u64.wrapping_sub(0x432u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 167;
            }
            169 => {
                values[11] = (0x6u64) as i32 as i64 as u64;
                pc = 168;
            }
            170 => {
                values[11] = (0x1u64) as i32 as i64 as u64;
                pc = 168;
            }
            171 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    170
                } else {
                    168
                };
            }
            172 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    169
                } else {
                    171
                };
            }
            173 => {
                values[7] = u64::from((values[6]) != 0);
                pc = 172;
            }
            174 => {
                pc = 168;
            }
            175 => {
                values[25] = (0x2u64) as i32 as i64 as u64;
                pc = 174;
            }
            176 => {
                values[11] = (0x6u64) as i32 as i64 as u64;
                pc = 175;
            }
            177 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    176
                } else {
                    175
                };
            }
            178 => {
                values[7] = u64::from((values[6]) != 0);
                pc = 177;
            }
            179 => {
                pc = 168;
            }
            180 => {
                values[25] = (0x1u64) as i32 as i64 as u64;
                pc = 179;
            }
            181 => {
                values[11] = (0x6u64) as i32 as i64 as u64;
                pc = 180;
            }
            182 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    181
                } else {
                    180
                };
            }
            183 => {
                values[7] = u64::from((values[6]) != 0);
                pc = 182;
            }
            184 => {
                pc = 168;
            }
            185 => {
                values[26] = u64::from((values[6]) != 0);
                pc = 184;
            }
            186 => {
                pc = 168;
            }
            187 => {
                values[24] = (0x70u64) as i8 as i64 as u64;
                pc = 186;
            }
            188 => {
                values[25] = (0x4u64) as i32 as i64 as u64;
                pc = 187;
            }
            189 => {
                values[7] = u64::from((values[6]) != 0);
                pc = 188;
            }
            190 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x67u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    172
                } else {
                    185
                };
            }
            191 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x47u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    173
                } else {
                    190
                };
            }
            192 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x66u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    191
                };
            }
            193 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x46u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    178
                } else {
                    192
                };
            }
            194 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x65u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    182
                } else {
                    193
                };
            }
            195 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x45u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    183
                } else {
                    194
                };
            }
            196 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x61u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    188
                } else {
                    195
                };
            }
            197 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x41u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    189
                } else {
                    196
                };
            }
            198 => {
                values[27] = (u64::from(
                    (u64::from(memory.read_u8((values[3]).wrapping_add(0x8u64))?)) as u8,
                )) as i32 as i64 as u64;
                pc = 197;
            }
            199 => {
                values[6] = 1u64;
                pc = 198;
            }
            200 => {
                values[26] = 0u64;
                pc = 199;
            }
            201 => {
                values[11] = ((u64::from(memory.read_u32((values[3]).wrapping_add(0x4u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 200;
            }
            202 => {
                values[24] = (0x65u64) as i8 as i64 as u64;
                pc = 201;
            }
            203 => {
                values[25] = (0x3u64) as i32 as i64 as u64;
                pc = 202;
            }
            204 => {
                values[7] = 0u64;
                pc = 203;
            }
            205 => {
                {
                    let destination = relocate("STACK_local_5f8");
                    let stored = (0x2u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 204;
            }
            206 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    205
                } else {
                    204
                };
            }
            207 => {
                {
                    let destination = relocate("STACK_local_5f8");
                    let stored = ((u64::from(memory.read_u8((values[3]).wrapping_add(0xau64))?))
                        as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 206;
            }
            208 => {
                values[20] = (0x0u64) as i32 as i64 as u64;
                pc = 207;
            }
            209 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_728"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 208;
            }
            210 => {
                {
                    let destination = relocate("STACK_PARAM_param_5");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 209;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140045890` (`mspaint.c:64591`).
pub fn fun_140045890(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 6 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 6,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        u64::from((arguments[2]) as u16),
        arguments[3],
        (arguments[4]) as i32 as i64 as u64,
        (arguments[5]) as i32 as i64 as u64,
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
    let mut pc: i32 = 101;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [values[12]];
                    invoke("GlobalFree", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = 6;
            }
            4 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[13] = values[14];
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[13]];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 2;
            }
            7 => {
                pc = 1;
            }
            8 => {
                {
                    let call_arguments = [values[14]];
                    invoke("DeleteDC", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [values[13]];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [values[14], values[15]];
                    invoke("SelectObject", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from((values[15]) != (0x0u64))) != 0 {
                    10
                } else {
                    9
                };
            }
            12 => {
                {
                    let call_arguments = [values[12]];
                    invoke("GlobalFree", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                return Ok(Some(values[0]));
            }
            14 => {
                {
                    let destination = values[0];
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [values[14]];
                    invoke("DeleteDC", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [values[13]];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [values[14], values[15]];
                    invoke("SelectObject", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                pc = if (u64::from((values[15]) != (0x0u64))) != 0 {
                    17
                } else {
                    16
                };
            }
            19 => {
                {
                    let destination = ((values[16]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x14u64).wrapping_mul(1));
                    let stored = u64::from((values[6]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    12
                };
            }
            21 => {
                values[12] = values[16];
                pc = 20;
            }
            22 => {
                values[11] = ({
                    let call_arguments = [
                        values[14],
                        values[1],
                        0x0u64,
                        u64::from(
                            (u64::from(
                                (u64::from(
                                    (u64::from(memory.read_u16(
                                        (relocate("STACK_local_78")).wrapping_add(0x8u64),
                                    )?)) as u16,
                                )) as u16,
                            )) as u32,
                        ),
                        ((((((values[16]).wrapping_add((0x28u64).wrapping_mul(1)))
                            .wrapping_add(
                                ((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64).wrapping_mul(4),
                            ))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add(u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[16]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )))
                        .wrapping_add(values[9]),
                        values[16],
                        0x0u64,
                    ];
                    invoke("GetDIBits", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 21;
            }
            23 => {
                values[6] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32(
                                ((values[16]).wrapping_add((0x0u64).wrapping_mul(1)))
                                    .wrapping_add((0x14u64).wrapping_mul(1)),
                            )?,
                        )) as u32,
                    )) as u32,
                );
                pc = 22;
            }
            24 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [(values[16]).wrapping_add((0x0u64).wrapping_mul(1))];
                        invoke("PaletteSize", &call_arguments, memory)
                    }) as u16,
                );
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [values[12]];
                    invoke("GlobalFree", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                {
                    let call_arguments =
                        [values[16], values[12], (values[9]).wrapping_add(0x28u64)];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    26
                } else {
                    12
                };
            }
            28 => {
                values[16] = {
                    let call_arguments = [0x40u64, values[18]];
                    invoke("GlobalAlloc", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let destination = values[3];
                    let stored = u64::from((values[18]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 28;
            }
            30 => {
                values[18] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((values[10]).wrapping_add(u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_68")).wrapping_add(0x4u64),
                                    )?,
                                )) as u32,
                            ))) as u32,
                        ))
                        .wrapping_add(u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_78")).wrapping_add(0x0u64))?,
                            )) as u32,
                        ))) as u32,
                    )) as u32,
                );
                pc = 29;
            }
            31 => {
                values[10] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("PaletteSize", &call_arguments, memory)
                    }) as u16,
                );
                pc = 30;
            }
            32 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                ((u64::from(
                                    ((u64::from(
                                        ((u64::from(
                                            ((u64::from(
                                                (u64::from(
                                                    (u64::from(
                                                        (u64::from(
                                                            memory.read_u16(
                                                                (relocate("STACK_local_78"))
                                                                    .wrapping_add(0xeu64),
                                                            )?,
                                                        ))
                                                            as u16,
                                                    ))
                                                        as u16,
                                                ))
                                                    as u32,
                                            ))
                                            .wrapping_mul(u64::from(
                                                (u64::from(
                                                    memory.read_u32(
                                                        (relocate("STACK_local_78"))
                                                            .wrapping_add(0x4u64),
                                                    )?,
                                                ))
                                                    as u32,
                                            ))) as u32,
                                        ))
                                        .wrapping_add(0x1fu64))
                                            as u32,
                                    ) as u32)
                                        .wrapping_shr((0x3u64) as u32)
                                        as u64) as u32,
                                )) & (0x1ffffffcu64)) as u32,
                            ))
                            .wrapping_mul(u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_78")).wrapping_add(0x8u64),
                                    )?,
                                )) as u32,
                            ))) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    )) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    32
                } else {
                    31
                };
            }
            34 => {
                {
                    let destination = (relocate("STACK_local_58")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x24u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = (relocate("STACK_local_58")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x20u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_LStack_5c");
                    let stored = ((u64::from(
                        memory.read_u32(
                            ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                .wrapping_add((0x1cu64).wrapping_mul(1)),
                        )?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_LStack_60");
                    let stored = ((u64::from(
                        memory.read_u32(
                            ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                .wrapping_add((0x18u64).wrapping_mul(1)),
                        )?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x14u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x10u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0xeu64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0xeu64).wrapping_mul(1)),
                                )?,
                            )) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0xcu64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0xcu64).wrapping_mul(1)),
                                )?,
                            )) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x8u64);
                    let stored = u64::from(
                        ((u64::from(
                            memory.read_u32(
                                ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                    .wrapping_add((0x8u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        ((u64::from(
                            memory.read_u32(
                                ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                    .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [
                        values[14],
                        values[1],
                        0x0u64,
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_78")).wrapping_add(0x8u64))?,
                            )) as u32,
                        ),
                        0x0u64,
                        values[12],
                        0x0u64,
                    ];
                    invoke("GetDIBits", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                values[15] = {
                    let call_arguments = [values[14], values[13]];
                    invoke("SelectObject", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    46
                } else {
                    6
                };
            }
            48 => {
                pc = if (u64::from((values[13]) == (0x0u64))) != 0 {
                    5
                } else {
                    47
                };
            }
            49 => {
                values[14] = {
                    let call_arguments = [0x0u64];
                    invoke("CreateCompatibleDC", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                values[13] = {
                    let call_arguments =
                        [
                            0x2u64,
                            0x2u64,
                            u64::from(
                                (u64::from((memory.read_u64(relocate("STACK_local_88"))?) as u16))
                                    as u32,
                            ),
                            u64::from(
                                (u64::from(
                                    (u64::from(memory.read_u16(
                                        (relocate("STACK_local_88")).wrapping_add(0x2u64),
                                    )?)) as u16,
                                )) as u32,
                            ),
                            0x0u64,
                        ];
                    invoke("CreateBitmap", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x8u64);
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x0u64);
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x24u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_58")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_local_58"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x1cu64).wrapping_mul(1));
                    let stored = ((u64::from(memory.read_u32(relocate("STACK_LStack_5c"))?)) as i32
                        as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = ((u64::from(memory.read_u32(relocate("STACK_LStack_60"))?)) as i32
                        as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x14u64).wrapping_mul(1));
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
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_local_68"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0xeu64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u16((relocate("STACK_local_78")).wrapping_add(0xeu64))?,
                            )) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u16((relocate("STACK_local_78")).wrapping_add(0xcu64))?,
                            )) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_78")).wrapping_add(0x8u64))?,
                        )) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_78")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = ((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))
                        .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_78")).wrapping_add(0x0u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 62;
            }
            64 => {
                return Ok(Some(values[0]));
            }
            65 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                pc = if (u64::from((values[12]) == (0x0u64))) != 0 {
                    65
                } else {
                    63
                };
            }
            67 => {
                values[8] = memory.read_u64((relocate("STACK_local_78")).wrapping_add(0x8u64))?;
                pc = 66;
            }
            68 => {
                values[7] = memory.read_u64((relocate("STACK_local_78")).wrapping_add(0x0u64))?;
                pc = 67;
            }
            69 => {
                values[12] = {
                    let call_arguments = [0x40u64, (values[9]).wrapping_add(0x28u64)];
                    invoke("GlobalAlloc", &call_arguments, memory)
                };
                pc = 68;
            }
            70 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("PaletteSize", &call_arguments, memory)
                    }) as u16,
                );
                pc = 69;
            }
            71 => {
                {
                    let destination = (relocate("STACK_local_58")).wrapping_add(0x4u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = (relocate("STACK_local_58")).wrapping_add(0x0u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x4u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x0u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0xcu64);
                    let stored = u64::from((0x1u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x0u64);
                    let stored = u64::from((0x28u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_LStack_5c");
                    let stored = (values[5]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0xeu64);
                    let stored = u64::from((values[17]) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let destination = relocate("STACK_LStack_60");
                    let stored = (values[4]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x8u64);
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_uStack_90"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0xcu64);
                    let stored = u64::from(
                        (u64::from(((values[8]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = (relocate("STACK_local_78")).wrapping_add(0x0u64);
                    let stored = ((((u64::from(
                        (memory.read_u64((relocate("STACK_local_78")).wrapping_add(0x8u64))?)
                            as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_98")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) & (0xffffffffu64)))
                        .wrapping_shl((0x20u64) as u32);
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                values[17] = u64::from((0x4u64) as u16);
                pc = 82;
            }
            84 => {
                values[17] = u64::from((0x8u64) as u16);
                pc = 82;
            }
            85 => {
                values[17] = u64::from((0x18u64) as u16);
                pc = 82;
            }
            86 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x19u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    85
                } else {
                    82
                };
            }
            87 => {
                values[17] = u64::from((0x20u64) as u16);
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x9u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    84
                } else {
                    87
                };
            }
            89 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    83
                } else {
                    88
                };
            }
            90 => {
                pc = if (u64::from(
                    (((0x1u64) as i32 as i64 as u64) as i64)
                        < (((values[2]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    89
                } else {
                    82
                };
            }
            91 => {
                values[17] = u64::from((0x1u64) as u16);
                pc = 90;
            }
            92 => {
                values[2] = u64::from(
                    (((u64::from(
                        (u64::from(
                            memory.read_u16((relocate("STACK_local_88")).wrapping_add(0x2u64))?,
                        )) as u16,
                    ))
                    .wrapping_mul(u64::from(
                        (memory.read_u64(relocate("STACK_local_88"))?) as u16,
                    ))) as i32 as i64 as u64) as u16,
                );
                pc = 91;
            }
            93 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    92
                } else {
                    91
                };
            }
            94 => {
                values[8] = memory.read_u64((relocate("STACK_local_78")).wrapping_add(0x8u64))?;
                pc = 93;
            }
            95 => {
                values[7] = memory.read_u64((relocate("STACK_local_78")).wrapping_add(0x0u64))?;
                pc = 94;
            }
            96 => {
                {
                    let call_arguments = [values[1], 0x20u64, relocate("STACK_local_98")];
                    invoke("GetObjectW", &call_arguments, memory)
                };
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_uStack_90");
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
                    let destination = relocate("STACK_local_50");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_e8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14005eb50` (`mspaint.c:86158`).
pub fn fun_14005eb50(
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
    let mut pc: i32 = 513;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x1u64));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("STACK_local_198")];
                    invoke("FUN_14000cf80", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_b8"))?,
                        relocate("STACK_local_58"),
                        values[14],
                        values[15],
                        0x1u64,
                    ];
                    invoke("Paint::Telemetry::TracePerformanceLogger::SendImageSavePerformanceComplete",&call_arguments,memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0xd8u64))?,
                        values[0],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("STACK_local_140"))?) == (0x0u64))) != 0)
                        && ((u64::from(
                            (((values[16]) as i32 as i64 as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x1b0u64))?,
                        values[0],
                        0x2u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0xc0u64))?,
                        values[0],
                        memory.read_u64(relocate("STACK_local_90"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_168"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    5
                };
            }
            9 => {
                values[16] = ((u64::from(memory.read_u32(relocate("STACK_local_168"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_160")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [
                        u64::from(
                            (relocate("CONTEXT_IGNORED_FUN_14005eb50_extraout_XMM0_Da")) as u32,
                        ),
                        values[0],
                    ];
                    invoke("FUN_1400177a0", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                values[12] = ({
                    let call_arguments = [values[9], relocate("STACK_local_160"), 0x1u64];
                    invoke("FUN_1400163a0", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_160"),
                        memory.read_u64(relocate("STACK_local_90"))?,
                        values[20],
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_148");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_uStack_158");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory.read_u16(
                                (memory.read_u64(relocate("STACK_local_90"))?)
                                    .wrapping_add((values[20]).wrapping_mul(2)),
                            )?,
                        )) as u16,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    18
                };
            }
            20 => {
                values[20] = (values[20]).wrapping_add(0x1u64);
                pc = 19;
            }
            21 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?;
                pc = 20;
            }
            22 => {
                pc = if (values[13]) != 0 { 21 } else { 9 };
            }
            23 => {
                {
                    let call_arguments = [relocate("STACK_local_c0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((u64::from(((values[25]) & (0x4u64)) as u32)) as u32))
                            != (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && ((u64::from((values[24]) != (0x0u64))) != 0),
                )) != 0
                {
                    23
                } else {
                    22
                };
            }
            25 => {
                {
                    let call_arguments = [relocate("STACK_local_d0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((u64::from(((values[25]) & (0x8u64)) as u32)) as u32))
                            != (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && (({
                            let _ = {
                                values[25] = u64::from(
                                    (u64::from(((values[25]) & (0xfffffff7u64)) as u32)) as u32,
                                );
                                values[25]
                            };
                            u64::from((memory.read_u64(relocate("STACK_local_d0"))?) != (0x0u64))
                        }) != 0),
                )) != 0
                {
                    25
                } else {
                    24
                };
            }
            27 => {
                values[13] = 0u64;
                pc = 26;
            }
            28 => {
                pc = 26;
            }
            29 => {
                values[13] = 1u64;
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[16]) as i32 as i64 as u64) as i64)
                            == (((0x1u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(
                                memory.read_u8(
                                    (memory.read_u64(
                                        (values[0]).wrapping_add((0x53u64).wrapping_mul(8)),
                                    )?)
                                    .wrapping_add(0x14u64),
                                )?,
                            )) as i8 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    29
                } else {
                    27
                };
            }
            31 => {
                values[25] = u64::from((values[17]) as u32);
                pc = 30;
            }
            32 => {
                values[16] = ({
                    let call_arguments = [relocate("STACK_local_d0")];
                    invoke("FUN_140007410", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[25]) & (0xffffffffu64)))
                        | (0xcu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                values[17] = u64::from((0xcu64) as u32);
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_188")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            36 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(values[24])?).wrapping_add((0xcu64).wrapping_mul(4)),
                        )?,
                        values[24],
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_188")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x4481u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[25]) & (0xffffffffu64)))
                        | (0x4u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(values[24])?).wrapping_add((0x2u64).wrapping_mul(4)),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                pc = if (u64::from((values[24]) != (0x0u64))) != 0 {
                    43
                } else {
                    42
                };
            }
            45 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = values[24];
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                values[24] = memory.read_u64(
                    (memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?)
                        .wrapping_add(0x998u64),
                )?;
                pc = 45;
            }
            47 => {
                pc = if (u64::from(
                    ((values[13]) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x1d4u64))?))
                                as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    46
                } else {
                    27
                };
            }
            48 => {
                values[24] = memory.read_u64(relocate("STACK_local_c0"))?;
                pc = 47;
            }
            49 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    49
                } else {
                    48
                };
            }
            51 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_188")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            52 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    51
                } else {
                    50
                };
            }
            53 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(values[10])?).wrapping_add((0x30u64).wrapping_mul(8)),
                        )?,
                        values[10],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_188")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x515u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b70"))?)?)
                                .wrapping_add((0x1u64).wrapping_mul(8)),
                        )?,
                        memory.read_u64(relocate("DAT_1401d0b70"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    57
                } else {
                    56
                };
            }
            59 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    59
                } else {
                    48
                };
            }
            61 => {
                values[10] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 60;
            }
            62 => {
                {
                    let call_arguments = [values[17]];
                    invoke("TraceFileSaveAs", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                {
                    let call_arguments = [values[17]];
                    invoke("TraceFileSave", &call_arguments, memory)
                };
                pc = 61;
            }
            64 => {
                pc = if (values[14]) != 0 { 62 } else { 63 };
            }
            65 => {
                values[17] = u64::from(
                    ({
                        let call_arguments = [values[0]];
                        invoke("FUN_14005dbb0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 64;
            }
            66 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    65
                } else {
                    61
                };
            }
            67 => {
                return Ok(Some(0x0u64));
            }
            68 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let call_arguments = [relocate("STACK_local_198")];
                    invoke("FUN_14000cf80", &call_arguments, memory)
                };
                pc = 68;
            }
            70 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_b8"))?,
                        relocate("STACK_local_58"),
                        values[14],
                        values[15],
                        0x0u64,
                    ];
                    invoke("Paint::Telemetry::TracePerformanceLogger::SendImageSavePerformanceComplete",&call_arguments,memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [];
                    invoke("AfxTryCleanup", &call_arguments, memory)
                };
                pc = 70;
            }
            72 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_90"))?, 0x0u64];
                    invoke("CFile::Remove", &call_arguments, memory)
                };
                pc = 71;
            }
            73 => {
                {
                    let call_arguments = [relocate("STACK_local_b0")];
                    invoke(
                        "AFX_EXCEPTION_LINK::AFX_EXCEPTION_LINK",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 72;
            }
            74 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_140"))?) == (0x0u64)))
                    != 0
                {
                    73
                } else {
                    70
                };
            }
            75 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    74
                } else {
                    66
                };
            }
            76 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x110u64))?
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 75;
            }
            77 => {
                {
                    let destination =
                        (relocate("STACK_local_198")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((values[15]) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 76;
            }
            78 => {
                values[15] = 0u64;
                pc = 77;
            }
            79 => {
                values[15] = 1u64;
                pc = 77;
            }
            80 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(
                            memory.read_u8(
                                (memory.read_u64(
                                    (values[0]).wrapping_add((0x53u64).wrapping_mul(8)),
                                )?)
                                .wrapping_add(0xa28u64),
                            )?,
                        )) as i8 as i64 as u64) as i32 as i64 as u64)
                            as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[15] = u64::from(
                                    ({
                                        let call_arguments = [(memory.read_u64(
                                            (values[0]).wrapping_add((0x53u64).wrapping_mul(8)),
                                        )?)
                                        .wrapping_add(0x9c8u64)];
                                        invoke(
                                            "Paint::Provenance::ProvenanceHelper::HasAIContent",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[15]
                            };
                            u64::from(!((values[15]) != 0))
                        }) != 0),
                )) != 0
                {
                    78
                } else {
                    79
                };
            }
            81 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_c8"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                values[14] = 0u64;
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x9u64).wrapping_mul(8)))?,
                        memory.read_u64(relocate("STACK_local_90"))?,
                        0x2u64,
                        0x80u64,
                    ];
                    invoke("DuplicateEncryptionInfoFile", &call_arguments, memory)
                };
                pc = 80;
            }
            84 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((values[18]) as u32)) != (u64::from((0xffffffffu64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            (u64::from(
                                (u64::from(
                                    ((u64::from(
                                        ((values[18] as u32).wrapping_shr((0xeu64) as u32) as u64)
                                            as u32,
                                    )) & (0x1u64)) as u32,
                                )) as u32,
                            )) != (u64::from((0x0u64) as u32)),
                        )) != 0),
                )) != 0
                {
                    83
                } else {
                    80
                };
            }
            85 => {
                values[18] = u64::from(
                    ({
                        let call_arguments =
                            [memory
                                .read_u64((values[0]).wrapping_add((0x9u64).wrapping_mul(8)))?];
                        invoke("GetFileAttributesW", &call_arguments, memory)
                    }) as u32,
                );
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                values[14] = 1u64;
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    ((values[14]) != 0)
                        || (({
                            let _ = {
                                values[16] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (values[0]).wrapping_add((0x9u64).wrapping_mul(8)),
                                        )?,
                                        memory.read_u64(relocate("STACK_local_90"))?,
                                    ];
                                    invoke("lstrcmpiW", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[16]
                            };
                            u64::from(
                                (((values[16]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    82
                } else {
                    87
                };
            }
            89 => {
                values[14] = u64::from(
                    ({
                        let call_arguments = [(values[0]).wrapping_add((0x9u64).wrapping_mul(8))];
                        invoke(
                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 88;
            }
            90 => {
                values[13] = 0u64;
                pc = 89;
            }
            91 => {
                values[13] = 1u64;
                pc = 89;
            }
            92 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[16]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[16] = ({
                                    let call_arguments =
                                        [values[21], relocate("DAT_1400e4c50"), 0x10u64];
                                    invoke("memcmp", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[16]
                            };
                            u64::from(
                                ((u64::from(
                                    (((values[16]) as i32 as i64 as u64) as i64)
                                        != (((0x0u64) as i32 as i64 as u64) as i64),
                                )) != 0)
                                    || ((u64::from(
                                        ((((u64::from(
                                            memory.read_u32((values[21]).wrapping_add(0x10u64))?,
                                        )) as i32 as i64
                                            as u64) as i32
                                            as i64 as u64)
                                            as i64)
                                            == ((((memory.read_u64(relocate("STACK_local_60"))?)
                                                as i32
                                                as i64
                                                as u64)
                                                as i32
                                                as i64
                                                as u64)
                                                as i64),
                                    )) != 0),
                            )
                        }) != 0),
                )) != 0
                {
                    90
                } else {
                    91
                };
            }
            93 => {
                values[16] = ({
                    let call_arguments = [values[21], relocate("STACK_local_70"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 92;
            }
            94 => {
                values[21] =
                    memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?;
                pc = 93;
            }
            95 => {
                {
                    let call_arguments =
                        [memory.read_u64((values[19]).wrapping_add((0x8u64).wrapping_mul(1)))?];
                    invoke("CCmdTarget::BeginWaitCursor", &call_arguments, memory)
                };
                pc = 94;
            }
            96 => {
                values[19] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 95;
            }
            97 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_b8"))?,
                        relocate("STACK_local_58"),
                    ];
                    invoke(
                        "Paint::Telemetry::TracePerformanceLogger::SendImageSavePerformanceStart",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = {
                        let call_arguments = [];
                        invoke(
                            "Paint::Telemetry::TracePerformanceLogger::GetInstance",
                            &call_arguments,
                            memory,
                        )
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let destination = (values[8]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_uStack_80"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = values[8];
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_84"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_88"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                values[8] = memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?;
                pc = 101;
            }
            103 => {
                pc = 68;
            }
            104 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    103
                } else {
                    102
                };
            }
            105 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[11]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 104;
            }
            106 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 105;
            }
            107 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            108 => {
                pc = if (u64::from(
                    (((((values[16]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    107
                } else {
                    104
                };
            }
            109 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    106
                } else {
                    108
                };
            }
            110 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 109;
            }
            111 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 110;
            }
            112 => {
                values[16] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 111;
            }
            113 => {
                values[4] = (values[11]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 112;
            }
            114 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 113;
            }
            115 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    114
                } else {
                    104
                };
            }
            116 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[24]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 115;
            }
            117 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 116;
            }
            118 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            119 => {
                pc = if (u64::from(
                    (((((values[16]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    118
                } else {
                    115
                };
            }
            120 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    117
                } else {
                    119
                };
            }
            121 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 120;
            }
            122 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 121;
            }
            123 => {
                values[16] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 122;
            }
            124 => {
                values[5] = (memory.read_u64(relocate("STACK_local_c0"))?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                pc = 123;
            }
            125 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 124;
            }
            126 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_c0"))?) != (0x0u64))) != 0
                {
                    125
                } else {
                    115
                };
            }
            127 => {
                values[24] = memory.read_u64(relocate("STACK_local_c0"))?;
                pc = 126;
            }
            128 => {
                values[12] = ({
                    let call_arguments = [relocate("STACK_local_c0")];
                    invoke("FUN_140045100", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 127;
            }
            129 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_c0"),
                        relocate("STACK_local_190"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 128;
            }
            130 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 129;
            }
            131 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 130;
            }
            132 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 131;
            }
            133 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 128;
            }
            134 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 134;
            }
            136 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_188")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            137 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    136
                } else {
                    135
                };
            }
            138 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_138"))?,
                        relocate("STACK_local_c0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 137;
            }
            139 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_188")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 141;
            }
            143 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    132
                } else {
                    142
                };
            }
            144 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("STACK_local_138");
                    memory.write_u64(destination, stored)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = relocate("STACK_local_130");
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = relocate("L\"DoYouWantToContinue\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 149;
            }
            151 => {
                {
                    let destination = relocate("STACK_local_12c");
                    let stored = u64::from((0x13u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 150;
            }
            152 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 151;
            }
            153 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x80000000u64)) as u32)) as u32);
                pc = 152;
            }
            154 => {
                values[9] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 153;
            }
            155 => {
                values[11] = memory.read_u64(relocate("STACK_local_d0"))?;
                pc = 154;
            }
            156 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_d0"),
                        relocate("STACK_local_190"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 157;
            }
            159 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 158;
            }
            160 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 155;
            }
            161 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 160;
            }
            162 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 161;
            }
            163 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_188")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            164 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    163
                } else {
                    162
                };
            }
            165 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_160"))?,
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 164;
            }
            166 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 165;
            }
            167 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 166;
            }
            168 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_188")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    159
                } else {
                    169
                };
            }
            171 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 170;
            }
            172 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 171;
            }
            173 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 172;
            }
            174 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("STACK_local_160");
                    memory.write_u64(destination, stored)?;
                }
                pc = 174;
            }
            176 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = relocate("STACK_uStack_158");
                    memory.write_u64(destination, stored)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let destination = relocate("STACK_local_148");
                    let stored = relocate("L\"ColorSaveWarning\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 176;
            }
            178 => {
                {
                    let destination = relocate("STACK_uStack_158");
                    let stored = 0x1000000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 177;
            }
            179 => {
                pc = if (u64::from(
                    ((u64::from((u64::from((values[17]) as u32)) < (u64::from((0x18u64) as u32))))
                        != 0)
                        && ((u64::from(
                            (u64::from((values[17]) as u32))
                                < (u64::from(
                                    (u64::from(
                                        (((((memory.read_u64(relocate("STACK_local_60"))?) as i32
                                            as i64
                                            as u64
                                            as i32)
                                            .wrapping_shr((0x8u64) as u32)
                                            as u64) as i32
                                            as i64
                                            as u64)
                                            & (0xffu64))
                                            as u32,
                                    )) as u32,
                                )),
                        )) != 0),
                )) != 0
                {
                    178
                } else {
                    102
                };
            }
            180 => {
                values[17] = u64::from(
                    ((((((memory.read_u64(relocate("STACK_local_78"))?) as i32 as i64 as u64 as i32)
                        .wrapping_shr((0x8u64) as u32) as u64) as i32 as i64
                        as u64)
                        & (0xffu64)) as i32 as i64 as u64) as u32,
                );
                pc = 179;
            }
            181 => {
                values[9] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 180;
            }
            182 => {
                values[25] = u64::from((values[17]) as u32);
                pc = 181;
            }
            183 => {
                pc = 68;
            }
            184 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    183
                } else {
                    182
                };
            }
            185 => {
                values[12] = ({
                    let call_arguments = [
                        u64::from(
                            (u64::from(
                                memory.read_u8(
                                    (relocate("STACK_local_198"))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u8,
                        ),
                        relocate("STACK_local_90"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("FUN_14005d180", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 184;
            }
            186 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored =
                        memory.read_u64((values[23]).wrapping_add((0x4u64).wrapping_mul(4)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 185;
            }
            187 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored =
                        memory.read_u64((values[23]).wrapping_add((0x2u64).wrapping_mul(4)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 186;
            }
            188 => {
                {
                    let destination = relocate("STACK_uStack_84");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[23]).wrapping_add((0x1u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 187;
            }
            189 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[23])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 188;
            }
            190 => {
                values[23] = {
                    let call_arguments = [];
                    invoke("FUN_140013680", &call_arguments, memory)
                };
                pc = 189;
            }
            191 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14000a4a0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a4a0"));
            }
            192 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    191
                } else {
                    182
                };
            }
            193 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    190
                } else {
                    192
                };
            }
            194 => {
                values[17] = u64::from((u64::from(((values[25]) | (0x20000u64)) as u32)) as u32);
                pc = 193;
            }
            195 => {
                pc = 68;
            }
            196 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    195
                } else {
                    194
                };
            }
            197 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[11]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 196;
            }
            198 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 197;
            }
            199 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            200 => {
                pc = if (u64::from(
                    (((((values[6]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    199
                } else {
                    196
                };
            }
            201 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    198
                } else {
                    200
                };
            }
            202 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 201;
            }
            203 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 202;
            }
            204 => {
                values[6] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 203;
            }
            205 => {
                values[4] = (memory.read_u64(relocate("STACK_local_d0"))?)
                    .wrapping_add((0x18u64).wrapping_mul(1));
                pc = 204;
            }
            206 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 205;
            }
            207 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_d0"))?) != (0x0u64))) != 0
                {
                    206
                } else {
                    196
                };
            }
            208 => {
                values[11] = memory.read_u64(relocate("STACK_local_d0"))?;
                pc = 207;
            }
            209 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[24]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 208;
            }
            210 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 209;
            }
            211 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            212 => {
                pc = if (u64::from(
                    (((((values[6]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    211
                } else {
                    208
                };
            }
            213 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    210
                } else {
                    212
                };
            }
            214 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 213;
            }
            215 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 214;
            }
            216 => {
                values[6] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 215;
            }
            217 => {
                values[5] = (memory.read_u64(relocate("STACK_local_c0"))?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                pc = 216;
            }
            218 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 217;
            }
            219 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_c0"))?) != (0x0u64))) != 0
                {
                    218
                } else {
                    208
                };
            }
            220 => {
                values[24] = memory.read_u64(relocate("STACK_local_c0"))?;
                pc = 219;
            }
            221 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[11]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 220;
            }
            222 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 221;
            }
            223 => {
                pc = 298;
            }
            224 => {
                pc = if (u64::from(
                    (((((values[6]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    223
                } else {
                    220
                };
            }
            225 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    222
                } else {
                    224
                };
            }
            226 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 225;
            }
            227 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 226;
            }
            228 => {
                values[6] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 227;
            }
            229 => {
                values[4] = (values[11]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 228;
            }
            230 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 229;
            }
            231 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    230
                } else {
                    220
                };
            }
            232 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[4]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 231;
            }
            233 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 232;
            }
            234 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            235 => {
                pc = if (u64::from(
                    (((((values[6]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    234
                } else {
                    231
                };
            }
            236 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    233
                } else {
                    235
                };
            }
            237 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 236;
            }
            238 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 237;
            }
            239 => {
                values[6] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 238;
            }
            240 => {
                values[3] = (memory.read_u64(relocate("STACK_local_c8"))?).wrapping_add(0x18u64);
                pc = 239;
            }
            241 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 240;
            }
            242 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_c8"))?) != (0x0u64))) != 0
                {
                    241
                } else {
                    231
                };
            }
            243 => {
                values[4] = memory.read_u64(relocate("STACK_local_c8"))?;
                pc = 242;
            }
            244 => {
                values[16] = ({
                    let call_arguments = [
                        relocate("STACK_local_c8"),
                        relocate("STACK_local_b8"),
                        relocate("STACK_local_c0"),
                        relocate("STACK_local_d0"),
                    ];
                    invoke("FUN_140045460", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 243;
            }
            245 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x42000u64)) as u32)) as u32);
                pc = 244;
            }
            246 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_c8"),
                        relocate("STACK_local_190"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 245;
            }
            247 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 246;
            }
            248 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 247;
            }
            249 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 248;
            }
            250 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 244;
            }
            251 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 250;
            }
            252 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 251;
            }
            253 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x1c2000u64)) as u32)) as u32);
                pc = 252;
            }
            254 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_188")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            255 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    254
                } else {
                    253
                };
            }
            256 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_160"))?,
                        relocate("STACK_local_c8"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 255;
            }
            257 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 256;
            }
            258 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 257;
            }
            259 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_188")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 258;
            }
            260 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 259;
            }
            261 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    249
                } else {
                    260
                };
            }
            262 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 261;
            }
            263 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 262;
            }
            264 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 263;
            }
            265 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 264;
            }
            266 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("STACK_local_160");
                    memory.write_u64(destination, stored)?;
                }
                pc = 265;
            }
            267 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = relocate("STACK_uStack_158");
                    memory.write_u64(destination, stored)?;
                }
                pc = 266;
            }
            268 => {
                {
                    let destination = relocate("STACK_local_148");
                    let stored = relocate("L\"TitleFlattenLayers\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 267;
            }
            269 => {
                {
                    let destination = relocate("STACK_uStack_158");
                    let stored = 0x1200000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 268;
            }
            270 => {
                values[9] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 269;
            }
            271 => {
                values[11] = memory.read_u64(relocate("STACK_local_b8"))?;
                pc = 270;
            }
            272 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x2004000u64)) as u32)) as u32);
                pc = 271;
            }
            273 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_b8"),
                        relocate("STACK_local_c8"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 272;
            }
            274 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 273;
            }
            275 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 274;
            }
            276 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 275;
            }
            277 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 271;
            }
            278 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 277;
            }
            279 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 278;
            }
            280 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x201c000u64)) as u32)) as u32);
                pc = 279;
            }
            281 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_b0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            282 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    281
                } else {
                    280
                };
            }
            283 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_138"))?,
                        relocate("STACK_local_b8"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 282;
            }
            284 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 283;
            }
            285 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 284;
            }
            286 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_b0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 285;
            }
            287 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 286;
            }
            288 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    276
                } else {
                    287
                };
            }
            289 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 288;
            }
            290 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 289;
            }
            291 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 290;
            }
            292 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 291;
            }
            293 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = relocate("STACK_local_138");
                    memory.write_u64(destination, stored)?;
                }
                pc = 292;
            }
            294 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = relocate("STACK_local_130");
                    memory.write_u64(destination, stored)?;
                }
                pc = 293;
            }
            295 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = relocate("u_MessageFlattenLayers_140113938");
                    memory.write_u64(destination, stored)?;
                }
                pc = 294;
            }
            296 => {
                {
                    let destination = relocate("STACK_local_12c");
                    let stored = u64::from((0x14u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 295;
            }
            297 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 296;
            }
            298 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            299 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("u_MessageFlattenLayers_140113938"))
                            .wrapping_add((0x14u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    298
                } else {
                    297
                };
            }
            300 => {
                values[9] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 299;
            }
            301 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x4200000u64)) as u32)) as u32);
                pc = 300;
            }
            302 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_c0"),
                        relocate("STACK_local_b8"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 301;
            }
            303 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 302;
            }
            304 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 303;
            }
            305 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 304;
            }
            306 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 300;
            }
            307 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 306;
            }
            308 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 307;
            }
            309 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x1c200000u64)) as u32)) as u32);
                pc = 308;
            }
            310 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_b0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            311 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    310
                } else {
                    309
                };
            }
            312 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_f8"))?,
                        relocate("STACK_local_c0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 311;
            }
            313 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 312;
            }
            314 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 313;
            }
            315 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_b0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 314;
            }
            316 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 315;
            }
            317 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    305
                } else {
                    316
                };
            }
            318 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 317;
            }
            319 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 318;
            }
            320 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 319;
            }
            321 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 320;
            }
            322 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = relocate("STACK_local_f8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 321;
            }
            323 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_f0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 322;
            }
            324 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = relocate("L\"SaveAsProject\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 323;
            }
            325 => {
                {
                    let destination = relocate("STACK_local_ec");
                    let stored = u64::from((0xdu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 324;
            }
            326 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 325;
            }
            327 => {
                values[9] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 326;
            }
            328 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x401000u64)) as u32)) as u32);
                pc = 327;
            }
            329 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_d0"),
                        relocate("STACK_local_c0"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 328;
            }
            330 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 329;
            }
            331 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 330;
            }
            332 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 331;
            }
            333 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 327;
            }
            334 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 333;
            }
            335 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 334;
            }
            336 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x1c01000u64)) as u32)) as u32);
                pc = 335;
            }
            337 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_b0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            338 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    337
                } else {
                    336
                };
            }
            339 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_118"))?,
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 338;
            }
            340 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 339;
            }
            341 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 340;
            }
            342 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_b0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 341;
            }
            343 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 342;
            }
            344 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    332
                } else {
                    343
                };
            }
            345 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 344;
            }
            346 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 345;
            }
            347 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 346;
            }
            348 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 347;
            }
            349 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = relocate("STACK_local_118");
                    memory.write_u64(destination, stored)?;
                }
                pc = 348;
            }
            350 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = relocate("STACK_local_110");
                    memory.write_u64(destination, stored)?;
                }
                pc = 349;
            }
            351 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = relocate("L\"ContinueLabel\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 350;
            }
            352 => {
                {
                    let destination = relocate("STACK_local_10c");
                    let stored = u64::from((0xdu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 351;
            }
            353 => {
                {
                    let destination = relocate("STACK_local_110");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 352;
            }
            354 => {
                pc = if (values[13]) != 0 { 353 } else { 182 };
            }
            355 => {
                values[17] = u64::from((u64::from(((values[25]) | (0x1000u64)) as u32)) as u32);
                pc = 354;
            }
            356 => {
                values[9] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 355;
            }
            357 => {
                {
                    let call_arguments = [relocate("STACK_local_c0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 356;
            }
            358 => {
                pc = if (u64::from((values[24]) != (0x0u64))) != 0 {
                    357
                } else {
                    356
                };
            }
            359 => {
                {
                    let call_arguments = [relocate("STACK_local_b8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 358;
            }
            360 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_b8"))?) != (0x0u64))) != 0
                {
                    359
                } else {
                    358
                };
            }
            361 => {
                values[13] = 0u64;
                pc = 360;
            }
            362 => {
                values[13] = 1u64;
                pc = 360;
            }
            363 => {
                pc = if (u64::from(
                    ((u64::from((u64::from((values[17]) as u32)) < (u64::from((0x2u64) as u32))))
                        != 0)
                        || ((u64::from(
                            (((u64::from(
                                (u64::from(
                                    memory.read_u8(
                                        (relocate("STACK_local_78")).wrapping_add(0x4u64),
                                    )?,
                                )) as u8,
                            )) as i32 as i64 as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    361
                } else {
                    362
                };
            }
            364 => {
                values[17] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_b8")];
                        invoke("FUN_140007410", &call_arguments, memory)
                    }) as u32,
                );
                pc = 363;
            }
            365 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[25]) & (0xffffffffu64)))
                        | (0x1003u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 364;
            }
            366 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = memory.read_u64(relocate("STACK_local_d0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 365;
            }
            367 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_b0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            368 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    367
                } else {
                    366
                };
            }
            369 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(values[24])?).wrapping_add((0xcu64).wrapping_mul(4)),
                        )?,
                        values[24],
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 368;
            }
            370 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 369;
            }
            371 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 370;
            }
            372 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_b0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x4481u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 371;
            }
            373 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 372;
            }
            374 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_c8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[25]) & (0xffffffffu64)))
                        | (0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 373;
            }
            375 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(values[24])?).wrapping_add((0x2u64).wrapping_mul(4)),
                        )?,
                        values[24],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 374;
            }
            376 => {
                pc = if (u64::from((values[24]) != (0x0u64))) != 0 {
                    375
                } else {
                    374
                };
            }
            377 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = values[24];
                    memory.write_u64(destination, stored)?;
                }
                pc = 376;
            }
            378 => {
                values[24] = memory.read_u64(
                    (memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?)
                        .wrapping_add(0x998u64),
                )?;
                pc = 377;
            }
            379 => {
                pc = 68;
            }
            380 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    379
                } else {
                    378
                };
            }
            381 => {
                values[12] = ({
                    let call_arguments = [
                        u64::from(
                            (u64::from(
                                memory.read_u8(
                                    (relocate("STACK_local_198"))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u8,
                        ),
                        relocate("STACK_local_90"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("FUN_14005d180", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 380;
            }
            382 => {
                {
                    let destination =
                        (relocate("STACK_local_198")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            (((values[2]) as i32 as i64 as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 381;
            }
            383 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = memory.read_u64(relocate("STACK_local_60"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 382;
            }
            384 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32(
                                ((relocate("STACK_local_70"))
                                    .wrapping_add((0x8u64).wrapping_mul(1)))
                                .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((relocate("STACK_local_70"))
                                        .wrapping_add((0x8u64).wrapping_mul(1)))
                                    .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 383;
            }
            385 => {
                {
                    let destination = (relocate("STACK_uStack_84")).wrapping_add(0x2u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u16(
                                (relocate("STACK_local_70")).wrapping_add((0x6u64).wrapping_mul(1)),
                            )?)) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 384;
            }
            386 => {
                {
                    let destination = (relocate("STACK_uStack_84")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u16(
                                (relocate("STACK_local_70")).wrapping_add((0x4u64).wrapping_mul(1)),
                            )?)) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 385;
            }
            387 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(
                                (relocate("STACK_local_70")).wrapping_add((0x0u64).wrapping_mul(1)),
                            )?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 386;
            }
            388 => {
                {
                    let call_arguments = [relocate("STACK_local_c8")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 387;
            }
            389 => {
                {
                    let call_arguments = [relocate("STACK_local_90"), relocate("STACK_local_c8")];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 388;
            }
            390 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[16]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[13] = u64::from(
                                    ({
                                        let call_arguments = [relocate("STACK_local_c8")];
                                        invoke(
                                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[13]
                            };
                            u64::from(!((values[13]) != 0))
                        }) != 0),
                )) != 0
                {
                    389
                } else {
                    388
                };
            }
            391 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0xd8u64))?,
                        values[7],
                        relocate("STACK_local_c8"),
                        0x4u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 390;
            }
            392 => {
                {
                    let call_arguments = [relocate("STACK_local_c8")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 391;
            }
            393 => {
                {
                    let call_arguments = [relocate("STACK_local_90"), values[16]];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::ReleaseBuffer",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 392;
            }
            394 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        != ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    393
                } else {
                    392
                };
            }
            395 => {
                values[16] = ({
                    let call_arguments = [relocate("STACK_local_90"), relocate("L\"#%;/\\\\\"")];
                    invoke("ATL::CStringT<>::FindOneOf", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 394;
            }
            396 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_90"),
                        (values[0]).wrapping_add((0x8u64).wrapping_mul(8)),
                    ];
                    invoke("ATL::CStringT<>::operator=", &call_arguments, memory)
                };
                pc = 395;
            }
            397 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[2]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[13] = u64::from(
                                    ({
                                        let call_arguments = [relocate("STACK_local_90")];
                                        invoke(
                                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[13]
                            };
                            values[13]
                        }) != 0),
                )) != 0
                {
                    396
                } else {
                    387
                };
            }
            398 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_90"),
                        (values[0]).wrapping_add((0x9u64).wrapping_mul(8)),
                    ];
                    invoke("ATL::CStringT<>::operator=", &call_arguments, memory)
                };
                pc = 397;
            }
            399 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0xau64).wrapping_mul(8)))?;
                pc = 398;
            }
            400 => {
                pc = if (u64::from(
                    ((u64::from(!((values[13]) != 0))) != 0)
                        || (({
                            let _ = {
                                values[13] = u64::from(
                                    ({
                                        let call_arguments = [relocate("STACK_local_90")];
                                        invoke(
                                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[13]
                            };
                            values[13]
                        }) != 0),
                )) != 0
                {
                    399
                } else {
                    98
                };
            }
            401 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_90"),
                        memory.read_u64(relocate("STACK_local_140"))?,
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 400;
            }
            402 => {
                values[20] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 401;
            }
            403 => {
                return Ok(Some(0x0u64));
            }
            404 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    403
                } else {
                    402
                };
            }
            405 => {
                values[13] = 0u64;
                pc = 404;
            }
            406 => {
                values[2] = ((u64::from(memory.read_u32(relocate("STACK_local_168"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 405;
            }
            407 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[11]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 406;
            }
            408 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 407;
            }
            409 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            410 => {
                pc = if (u64::from(
                    (((((values[16]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    409
                } else {
                    406
                };
            }
            411 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    408
                } else {
                    410
                };
            }
            412 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 411;
            }
            413 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 412;
            }
            414 => {
                values[16] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 413;
            }
            415 => {
                values[4] = (values[11]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 414;
            }
            416 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 415;
            }
            417 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    416
                } else {
                    406
                };
            }
            418 => {
                {
                    let call_arguments = [values[22], 0x0u64, values[4]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 417;
            }
            419 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 418;
            }
            420 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            421 => {
                pc = if (u64::from(
                    (((((values[16]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    420
                } else {
                    417
                };
            }
            422 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    419
                } else {
                    421
                };
            }
            423 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 422;
            }
            424 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 423;
            }
            425 => {
                values[16] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 424;
            }
            426 => {
                values[3] = (memory.read_u64(relocate("STACK_local_b8"))?)
                    .wrapping_add((0x18u64).wrapping_mul(1));
                pc = 425;
            }
            427 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 426;
            }
            428 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_b8"))?) != (0x0u64))) != 0
                {
                    427
                } else {
                    417
                };
            }
            429 => {
                values[4] = memory.read_u64(relocate("STACK_local_b8"))?;
                pc = 428;
            }
            430 => {
                values[12] = ({
                    let call_arguments = [relocate("STACK_local_b8")];
                    invoke("FUN_140045100", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 429;
            }
            431 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x100u64)) as u32)) as u32);
                pc = 430;
            }
            432 => {
                values[25] = u64::from((u64::from(((values[25]) | (0x210u64)) as u32)) as u32);
                pc = 431;
            }
            433 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_b8"),
                        relocate("STACK_local_d0"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 432;
            }
            434 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[20];
                    memory.write_u64(destination, stored)?;
                }
                pc = 433;
            }
            435 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 434;
            }
            436 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 435;
            }
            437 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 431;
            }
            438 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 437;
            }
            439 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 438;
            }
            440 => {
                values[25] = u64::from((u64::from(((values[25]) | (0xe10u64)) as u32)) as u32);
                pc = 439;
            }
            441 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = memory.read_u64(relocate("STACK_local_d0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 440;
            }
            442 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            443 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    442
                } else {
                    441
                };
            }
            444 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_b0"))?,
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 443;
            }
            445 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 444;
            }
            446 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 445;
            }
            447 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 446;
            }
            448 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 447;
            }
            449 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    436
                } else {
                    448
                };
            }
            450 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 449;
            }
            451 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 450;
            }
            452 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 451;
            }
            453 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 452;
            }
            454 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = relocate("STACK_local_b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 453;
            }
            455 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("STACK_local_a8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 454;
            }
            456 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = relocate("L\"DoYouWantToContinue\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 455;
            }
            457 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x1300000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 456;
            }
            458 => {
                values[20] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 457;
            }
            459 => {
                values[11] = memory.read_u64(relocate("STACK_local_c8"))?;
                pc = 458;
            }
            460 => {
                values[25] = u64::from((0x20u64) as u32);
                pc = 459;
            }
            461 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_c8"),
                        relocate("STACK_local_d0"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 460;
            }
            462 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[20];
                    memory.write_u64(destination, stored)?;
                }
                pc = 461;
            }
            463 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 462;
            }
            464 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 463;
            }
            465 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 459;
            }
            466 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 465;
            }
            467 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 466;
            }
            468 => {
                values[25] = u64::from((0xe0u64) as u32);
                pc = 467;
            }
            469 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = memory.read_u64(relocate("STACK_local_d0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 468;
            }
            470 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            471 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    470
                } else {
                    469
                };
            }
            472 => {
                values[16] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_188"))?,
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 471;
            }
            473 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 472;
            }
            474 => {
                {
                    let destination = relocate("STACK_uStack_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 473;
            }
            475 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 474;
            }
            476 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 475;
            }
            477 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    464
                } else {
                    476
                };
            }
            478 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 477;
            }
            479 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 478;
            }
            480 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 479;
            }
            481 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 480;
            }
            482 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = relocate("STACK_local_188");
                    memory.write_u64(destination, stored)?;
                }
                pc = 481;
            }
            483 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = relocate("STACK_local_180");
                    memory.write_u64(destination, stored)?;
                }
                pc = 482;
            }
            484 => {
                {
                    let destination = relocate("STACK_local_170");
                    let stored = relocate("L\"ProvenanceUnsupportedFormatConfirmationMessage\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 483;
            }
            485 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored = 0x2e00000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 484;
            }
            486 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    485
                } else {
                    402
                };
            }
            487 => {
                values[20] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 486;
            }
            488 => {
                values[12] = ({
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?,
                        relocate("STACK_local_70"),
                    ];
                    invoke("FUN_140015560", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 487;
            }
            489 => {
                values[13] = 1u64;
                pc = 488;
            }
            490 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[16]) as i32 as i64 as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && (({
                                let _ = {
                                    values[16] = ({
                                        let call_arguments = [
                                            relocate("STACK_local_70"),
                                            relocate("DAT_1400e4c50"),
                                            0x10u64,
                                        ];
                                        invoke("memcmp", &call_arguments, memory)
                                    }) as i32
                                        as i64
                                        as u64;
                                    values[16]
                                };
                                u64::from(
                                    ((u64::from(
                                        (((values[16]) as i32 as i64 as u64) as i64)
                                            != (((0x0u64) as i32 as i64 as u64) as i64),
                                    )) != 0)
                                        || ((u64::from(
                                            ((((memory.read_u64(relocate("STACK_local_60"))?) as i32
                                                as i64
                                                as u64)
                                                as i32
                                                as i64
                                                as u64)
                                                as i64)
                                                == ((((u64::from(memory.read_u32(
                                                    (values[21]).wrapping_add(0x10u64),
                                                )?))
                                                    as i32
                                                    as i64
                                                    as u64)
                                                    as i32
                                                    as i64
                                                    as u64)
                                                    as i64),
                                        )) != 0),
                                )
                            }) != 0),
                    )) != 0)
                        || (({
                            let _ = {
                                values[16] = ({
                                    let call_arguments = [relocate("STACK_local_70"), 0x0u64];
                                    invoke("GetClsidOfEncoder", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[16]
                            };
                            u64::from(
                                (((values[16]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    489
                } else {
                    488
                };
            }
            491 => {
                values[16] = ({
                    let call_arguments = [relocate("STACK_local_70"), values[21], 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 490;
            }
            492 => {
                values[21] = {
                    let call_arguments = [];
                    invoke("FUN_140013680", &call_arguments, memory)
                };
                pc = 491;
            }
            493 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored =
                        memory.read_u64((values[23]).wrapping_add((0x4u64).wrapping_mul(4)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 492;
            }
            494 => {
                {
                    let destination = ((relocate("STACK_local_70"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x4u64).wrapping_mul(1));
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[23]).wrapping_add((0x3u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 493;
            }
            495 => {
                {
                    let destination = ((relocate("STACK_local_70"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[23]).wrapping_add((0x2u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 494;
            }
            496 => {
                {
                    let destination = (relocate("STACK_local_70")).wrapping_add(0x4u64);
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[23]).wrapping_add((0x1u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 495;
            }
            497 => {
                {
                    let destination =
                        (relocate("STACK_local_70")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[23])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 496;
            }
            498 => {
                values[23] =
                    memory.read_u64((values[0]).wrapping_add((0x53u64).wrapping_mul(8)))?;
                pc = 497;
            }
            499 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[20]).wrapping_add(0x178u64))?,
                        values[20],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 498;
            }
            500 => {
                pc = if (u64::from(
                    ((u64::from(((values[20]) as i64) != ((0x0u64) as i64))) != 0)
                        && ((u64::from(
                            (memory.read_u64((values[20]).wrapping_add(0x178u64))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    499
                } else {
                    498
                };
            }
            501 => {
                values[20] = {
                    let call_arguments = [
                        memory.read_u64((values[20]).wrapping_add(0x40u64))?,
                        0x0u64,
                        relocate("CWnd::RTTI_Type_Descriptor"),
                        relocate("CPBFrame::RTTI_Type_Descriptor"),
                    ];
                    invoke("__RTDynamicCast", &call_arguments, memory)
                };
                pc = 500;
            }
            502 => {
                values[20] = {
                    let call_arguments = [
                        memory.read_u64((values[19]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        0x0u64,
                        relocate("CWinApp::RTTI_Type_Descriptor"),
                        relocate("CPBApp::RTTI_Type_Descriptor"),
                    ];
                    invoke("__RTDynamicCast", &call_arguments, memory)
                };
                pc = 501;
            }
            503 => {
                values[19] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 502;
            }
            504 => {
                {
                    let call_arguments = [relocate("STACK_local_d0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 503;
            }
            505 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_d0"))?) != (0x0u64))) != 0
                {
                    504
                } else {
                    503
                };
            }
            506 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d06a0"),
                        relocate("STACK_local_d0"),
                        values[16],
                    ];
                    invoke("FUN_1400512b0", &call_arguments, memory)
                };
                pc = 505;
            }
            507 => {
                values[16] = ({
                    let call_arguments = [memory.read_u64(relocate("DAT_1401d06e0"))?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 506;
            }
            508 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 507;
            }
            509 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = (values[2]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 508;
            }
            510 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_c8"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 509;
            }
            511 => {
                values[13] = 0u64;
                pc = 510;
            }
            512 => {
                values[25] = u64::from((0x0u64) as u32);
                pc = 511;
            }
            513 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_1c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 512;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14006aaf0` (`mspaint.c:96121`).
pub fn fun_14006aaf0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 185;
    loop {
        match pc {
            0 => {
                return Ok(Some(relocate("DAT_1401d0520")));
            }
            1 => {
                {
                    let destination = relocate("DAT_1401d07c6");
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("_DAT_1401d08f8");
                    let stored = 0x7d000000096u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("_DAT_1401d08f0");
                    let stored = 0x500000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400794f0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from(((values[7]) as i64) != ((0x0u64) as i64))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                {
                    let destination = relocate("DAT_1401d07a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                values[7] = memory.read_u64(relocate("DAT_1401d07a8"))?;
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [values[7], 0x40u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [values[7]];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [(values[7]).wrapping_add(0x20u64)];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from(((values[7]) as i64) != ((0x0u64) as i64))) != 0 {
                    10
                } else {
                    7
                };
            }
            12 => {
                {
                    let destination = relocate("DAT_1401d07a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                values[7] = memory.read_u64(relocate("DAT_1401d07a0"))?;
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[7])?)?,
                        values[7],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                pc = if (u64::from(((values[7]) as i64) != ((0x0u64) as i64))) != 0 {
                    14
                } else {
                    13
                };
            }
            16 => {
                {
                    let destination = relocate("DAT_1401d07b0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("DAT_1401d0bf8");
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("_DAT_1401d08c0");
                    let stored = u64::from((0x2u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("_DAT_1401d08b8");
                    let stored = 0x100000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("_DAT_1401d08b0");
                    let stored = 0x6400000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("_DAT_1401d08a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                values[7] = memory.read_u64(relocate("DAT_1401d07b0"))?;
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0898"),
                        0x4f6u64,
                        0x4f6u64,
                        0x4f6u64,
                        0x4f6u64,
                    ];
                    invoke("SetRect", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("_DAT_1401d0bb0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("DAT_1401d0858");
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("DAT_1401d0c20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("DAT_1401d0c18");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("_DAT_1401d0c10");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("_DAT_1401d0c08");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("_DAT_1401d0c00");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("DAT_1401d0bf0");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("_DAT_1401d0bd0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let call_arguments = [relocate("DAT_1401d0bc8")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("_DAT_1401d0bc0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("_DAT_1401d0bb8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [relocate("DAT_1401d0ba0"), relocate("STACK_local_78")];
                    invoke("FUN_1400765a0", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("FUN_14006aaa0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("uRam00000001401d0b98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("_DAT_1401d0b90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("_DAT_1401d0b88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("DAT_1401d0b80");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    41
                } else {
                    40
                };
            }
            43 => {
                {
                    let destination = relocate("DAT_1401d0b80");
                    let stored = (values[4]).wrapping_add((0x1u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                values[4] = values[5];
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[5]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = values[5];
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = (values[5]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 48;
            }
            50 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 49;
            }
            51 => {
                {
                    let destination = values[5];
                    let stored = relocate("winrt::impl::producers_base<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (values[5]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::produce<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = (values[5]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[5]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    54
                } else {
                    43
                };
            }
            56 => {
                values[4] = values[8];
                pc = 55;
            }
            57 => {
                values[5] = {
                    let call_arguments = [0x20u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14007c880", &call_arguments, memory)
                };
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("_DAT_1401cff58");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("FUN_14007cae0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 60;
            }
            62 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 61;
            }
            63 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 57;
            }
            64 => {
                {
                    let destination = relocate("_DAT_1401cff58");
                    let stored = (memory.read_u64(relocate("_DAT_1401cff58"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 64;
            }
            66 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14007ca00", &call_arguments, memory)
                };
                pc = 65;
            }
            67 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cff50"))?) == (0x0u64))) != 0
                {
                    62
                } else {
                    66
                };
            }
            68 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("_DAT_1401cff58");
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = relocate("DAT_1401cff58");
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("DAT_1401d0b70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                pc = if (u64::from((values[4]) != (values[5]))) != 0 {
                    77
                } else {
                    72
                };
            }
            74 => {
                let _ = {
                    values[4] = (values[4]).wrapping_add((0x1u64).wrapping_mul(8));
                    values[4]
                };
                pc = 73;
            }
            75 => {
                values[7] = memory.read_u64(relocate("_DAT_1401cff58"))?;
                pc = 74;
            }
            76 => {
                {
                    let destination = values[4];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("_DAT_1401cff58");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                values[7] = memory.read_u64(relocate("_DAT_1401cff58"))?;
                pc = 73;
            }
            79 => {
                {
                    let destination = relocate("DAT_1401d0b48");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("DAT_1401d0b58");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("_DAT_1401d0b50");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                values[5] = (values[4]).wrapping_add((0x10u64).wrapping_mul(8));
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [values[5]];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            85 => {
                pc = if (u64::from(
                    ((u64::from((0xfffu64) < ((values[7]).wrapping_mul(0x8u64)))) != 0)
                        && (({
                            let _ = {
                                values[5] = memory.read_u64(
                                    (memory.read_u64(relocate("DAT_1401d0b48"))?).wrapping_add(
                                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                            .wrapping_mul(8),
                                    ),
                                )?;
                                values[5]
                            };
                            u64::from(
                                (0x1fu64)
                                    < ((memory.read_u64(relocate("DAT_1401d0b48"))?).wrapping_add(
                                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                            .wrapping_sub(values[5]),
                                    )),
                            )
                        }) != 0),
                )) != 0
                {
                    84
                } else {
                    83
                };
            }
            86 => {
                values[5] = memory.read_u64(relocate("DAT_1401d0b48"))?;
                pc = 85;
            }
            87 => {
                pc = if (u64::from(((values[7]) as i64) != ((0x0u64) as i64))) != 0 {
                    86
                } else {
                    82
                };
            }
            88 => {
                values[7] = ((memory.read_u64(relocate("DAT_1401d0b58"))?)
                    .wrapping_sub(memory.read_u64(relocate("DAT_1401d0b48"))?)
                    as i64)
                    .wrapping_shr((0x3u64) as u32) as u64;
                pc = 87;
            }
            89 => {
                values[4] = {
                    let call_arguments = [0x80u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("_DAT_1401d0b38");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination = relocate("_DAT_1401d0b30");
                    let stored = u64::from((0x3f800000u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination = relocate("_DAT_1401d0b68");
                    let stored = 0x8u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("_DAT_1401d0b60");
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = relocate("DAT_1401d0b58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("_DAT_1401d0b50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("DAT_1401d0b48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = (values[3]).wrapping_add(0x8u64);
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = values[3];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                values[3] = {
                    let call_arguments = [0x88u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("_DAT_1401d0b40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("_DAT_1401d0b38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("_DAT_1401d0b30");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = relocate("DAT_1401d0b30");
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let call_arguments = [relocate("DAT_1401d0934"), 0x0u64, 0x1fau64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 103;
            }
            105 => {
                {
                    let destination = relocate("DAT_1401d0930");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let call_arguments = [relocate("DAT_1401d0920")];
                    invoke("CGdiplusInit::CGdiplusInit", &call_arguments, memory)
                };
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("DAT_1401d091c");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 106;
            }
            108 => {
                {
                    let destination = relocate("_DAT_1401d0918");
                    let stored = u64::from((0x26200au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("uRam00000001401d0910");
                    let stored = 0x64bd525195314dbbu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("_DAT_1401d0908");
                    let stored = 0x4651cf499407b3fcu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let call_arguments = [relocate("DAT_1401d0900")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 110;
            }
            112 => {
                {
                    let destination = relocate("_DAT_1401d08ec");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = relocate("_DAT_1401d08e4");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("uRam00000001401d08dc");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("_DAT_1401d08d4");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("_DAT_1401d08cc");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination = relocate("_DAT_1401d08c4");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("_DAT_1401d08a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("_DAT_1401d0898");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let call_arguments = [relocate("DAT_1401d0890")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 119;
            }
            121 => {
                {
                    let call_arguments = [relocate("DAT_1401d0888")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 120;
            }
            122 => {
                {
                    let destination = relocate("DAT_1401d0880");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let call_arguments = [relocate("DAT_1401d0878")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [relocate("DAT_1401d0870")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 123;
            }
            125 => {
                {
                    let call_arguments = [relocate("DAT_1401d0868")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 124;
            }
            126 => {
                {
                    let call_arguments = [relocate("DAT_1401d0860")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 125;
            }
            127 => {
                {
                    let destination = relocate("DAT_1401d085e");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 126;
            }
            128 => {
                {
                    let destination = relocate("_DAT_1401d085c");
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 127;
            }
            129 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 128;
            }
            130 => {
                {
                    let call_arguments = [relocate("DAT_1401d07c8")];
                    invoke(
                        "COleTemplateServer::COleTemplateServer",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 129;
            }
            131 => {
                {
                    let destination = relocate("_DAT_1401d07c4");
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 130;
            }
            132 => {
                {
                    let destination = relocate("_DAT_1401d07c0");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 131;
            }
            133 => {
                {
                    let destination = relocate("_DAT_1401d07b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 132;
            }
            134 => {
                {
                    let destination = relocate("DAT_1401d07b0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                {
                    let destination = relocate("DAT_1401d07a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 134;
            }
            136 => {
                {
                    let destination = relocate("DAT_1401d07a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let call_arguments = [relocate("DAT_1401d06a0")];
                    invoke("FUN_14006b110", &call_arguments, memory)
                };
                pc = 136;
            }
            138 => {
                {
                    let destination = relocate("_DAT_1401d0698");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let destination = relocate("_DAT_1401d0520");
                    let stored = relocate("CPBApp::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let call_arguments = [values[2], 0x0u64, values[5]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 139;
            }
            141 => {
                values[2] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 140;
            }
            142 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            143 => {
                pc = if (u64::from(
                    (((((values[1]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    142
                } else {
                    139
                };
            }
            144 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    141
                } else {
                    143
                };
            }
            145 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 144;
            }
            146 => {
                {
                    let destination = values[0];
                    let stored = ((((u64::from(memory.read_u32(values[0])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 145;
            }
            147 => {
                values[1] = ((u64::from(memory.read_u32(values[0])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 146;
            }
            148 => {
                values[0] = (values[5]).wrapping_add((0x3u64).wrapping_mul(8));
                pc = 147;
            }
            149 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 148;
            }
            150 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    149
                } else {
                    139
                };
            }
            151 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), values[6]];
                    invoke("CWinApp::CWinApp", &call_arguments, memory)
                };
                pc = 150;
            }
            152 => {
                values[5] = values[8];
                pc = 151;
            }
            153 => {
                values[6] = relocate("L\"\"");
                pc = 152;
            }
            154 => {
                values[5] = memory.read_u64(relocate("STACK_local_20"))?;
                pc = 151;
            }
            155 => {
                values[6] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_20"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8)),
                )?;
                pc = 154;
            }
            156 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64))) != 0
                {
                    153
                } else {
                    155
                };
            }
            157 => {
                values[4] = memory.read_u64(relocate("STACK_local_20"))?;
                pc = 156;
            }
            158 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_20"),
                        relocate("STACK_local_78"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 157;
            }
            159 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 158;
            }
            160 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 159;
            }
            161 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 160;
            }
            162 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 157;
            }
            163 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 162;
            }
            164 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 163;
            }
            165 => {
                {
                    let call_arguments = [values[1], relocate("STACK_local_38")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            166 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    165
                } else {
                    164
                };
            }
            167 => {
                values[1] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_58"))?,
                        relocate("STACK_local_20"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 166;
            }
            168 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 169;
            }
            171 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    161
                } else {
                    171
                };
            }
            173 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 172;
            }
            174 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 174;
            }
            176 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("STACK_local_58");
                    memory.write_u64(destination, stored)?;
                }
                pc = 176;
            }
            178 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = relocate("STACK_local_50");
                    memory.write_u64(destination, stored)?;
                }
                pc = 177;
            }
            179 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = relocate("L\"AppTitle\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 178;
            }
            180 => {
                {
                    let destination = relocate("STACK_local_4c");
                    let stored = u64::from((0x8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 179;
            }
            181 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 180;
            }
            182 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("DAT_1401d0520");
                    memory.write_u64(destination, stored)?;
                }
                pc = 181;
            }
            183 => {
                values[8] = 0x0u64;
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_a8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                values[7] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 184;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14007e010` (`mspaint.c:110958`).
pub fn fun_14007e010(
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
    let mut pc: i32 = 309;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[25]));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[27])?).wrapping_add(0x8u64))?,
                        values[27],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    (((values[28]) as i32 as i64 as u64) as i64)
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
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                values[28] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 4;
            }
            6 => {
                values[1] = (values[27]).wrapping_add(0xcu64);
                pc = 5;
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(values[27])?)?, values[27]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((((values[26]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
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
                    let destination = values[5];
                    let stored = ((((memory.read_u64(values[5])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[26] = memory.read_u64(values[5])?;
                pc = 11;
            }
            13 => {
                values[5] = (values[27]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((values[27]) != (0x0u64))) != 0 {
                    14
                } else {
                    0
                };
            }
            16 => {
                {
                    let call_arguments = [values[20], 0x0u64, values[8]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                values[20] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            19 => {
                pc = if (u64::from(
                    (((((values[28]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    15
                };
            }
            20 => {
                pc = if (u64::from(
                    (((values[28]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    19
                };
            }
            21 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            22 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                values[28] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 22;
            }
            24 => {
                values[1] = (values[8]).wrapping_add(0x18u64);
                pc = 23;
            }
            25 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 24;
            }
            26 => {
                pc = if (u64::from((values[8]) != (0x0u64))) != 0 {
                    25
                } else {
                    15
                };
            }
            27 => {
                values[8] = (((u64::from(
                    (u64::from(memory.read_u32(relocate("STACK_uStack_264"))?)) as u32,
                )) & (0xffffffffu64))
                    .wrapping_shl((0x20u64) as u32))
                    | ((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_local_268"))?)) as u32,
                    )) & (0xffffffffu64));
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [values[20], 0x0u64, values[11]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                values[20] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            31 => {
                pc = if (u64::from(
                    (((((values[28]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    27
                };
            }
            32 => {
                pc = if (u64::from(
                    (((values[28]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    31
                };
            }
            33 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 32;
            }
            34 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                values[28] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 34;
            }
            36 => {
                values[3] = (memory.read_u64(relocate("STACK_local_270"))?)
                    .wrapping_add((0xcu64).wrapping_mul(2));
                pc = 35;
            }
            37 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 36;
            }
            38 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_270"))?) != (0x0u64)))
                    != 0
                {
                    37
                } else {
                    27
                };
            }
            39 => {
                values[11] = memory.read_u64(relocate("STACK_local_270"))?;
                pc = 38;
            }
            40 => {
                {
                    let call_arguments = [relocate("STACK_local_270")];
                    invoke("FUN_140044770", &call_arguments, memory)
                };
                pc = 39;
            }
            41 => {
                {
                    let call_arguments = [relocate("STACK_local_270"), relocate("STACK_local_2d8")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored = 0x17u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = relocate("L\"CannotCompleteOperation\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [relocate("STACK_local_268"), relocate("STACK_local_2d8")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = relocate("L\"BitmapSizeError\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                values[25] = 0x1u64;
                pc = 15;
            }
            48 => {
                {
                    let call_arguments = [relocate("STACK_local_270")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [values[20], 0x0u64, values[7]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                values[20] = {
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
                    (((((values[28]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
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
                    (((values[28]) as i32 as i64 as u64) as i64)
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
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                values[28] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 56;
            }
            58 => {
                values[4] = (values[7]).wrapping_add((0xcu64).wrapping_mul(2));
                pc = 57;
            }
            59 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 58;
            }
            60 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    59
                } else {
                    49
                };
            }
            61 => {
                {
                    let call_arguments = [values[23]];
                    invoke("operator_delete__", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0xb8u64))?,
                        values[5],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                {
                    let call_arguments = [
                        0x2400u64,
                        values[24],
                        0x0u64,
                        0x0u64,
                        values[23],
                        u64::from(((values[28]).wrapping_add(0x105u64)) as u32),
                        relocate("STACK_local_260"),
                    ];
                    invoke("FormatMessageW", &call_arguments, memory)
                };
                pc = 62;
            }
            64 => {
                values[24] = relocate("DAT_140105708");
                pc = 63;
            }
            65 => {
                values[24] = memory.read_u64((values[7]).wrapping_add((0x8u64).wrapping_mul(2)))?;
                pc = 63;
            }
            66 => {
                pc = if (values[12]) != 0 { 64 } else { 65 };
            }
            67 => {
                {
                    let destination = relocate("STACK_local_260");
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_264"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_268"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                values[23] = {
                    let call_arguments = [values[22]];
                    invoke("operator_new__", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                values[22] =
                    (u64::from((u64::from(((values[28]).wrapping_add(0x105u64)) as u32)) as u32))
                        .wrapping_mul(0x2u64);
                pc = 68;
            }
            70 => {
                values[28] = ((u64::from(
                    memory.read_u32(
                        (memory.read_u64(relocate("STACK_local_278"))?)
                            .wrapping_add((0x2u64).wrapping_mul(2)),
                    )?,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 69;
            }
            71 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_278"))?) != (0x0u64)))
                    != 0
                {
                    70
                } else {
                    69
                };
            }
            72 => {
                values[12] = u64::from(
                    (u64::from((memory.read_u64(relocate("STACK_local_278"))?) == (0x0u64))) != 0,
                );
                pc = 71;
            }
            73 => {
                values[7] = memory.read_u64(relocate("STACK_local_278"))?;
                pc = 72;
            }
            74 => {
                {
                    let call_arguments = [0x0u64, relocate("STACK_local_278")];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 75;
            }
            77 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 76;
            }
            78 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 73;
            }
            79 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 79;
            }
            81 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_290")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            82 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    81
                } else {
                    80
                };
            }
            83 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_2b0"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_local_280");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_288");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination =
                        (relocate("STACK_local_290")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = relocate("STACK_local_278");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    77
                } else {
                    87
                };
            }
            89 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 90;
            }
            92 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("STACK_local_2b0");
                    let stored = relocate("STACK_local_2a8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = relocate("L\"RecoveredStatus\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_2a4");
                    let stored = u64::from((0xfu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_local_2a8");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 95;
            }
            97 => {
                values[25] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 96;
            }
            98 => {
                {
                    let call_arguments = [relocate("STACK_local_268"), 0xf003u64];
                    invoke("ATL::CStringT<>::LoadStringW", &call_arguments, memory)
                };
                pc = 97;
            }
            99 => {
                {
                    let call_arguments = [relocate("STACK_local_268"), relocate("STACK_local_248")];
                    invoke("ATL::CStringT<>::operator=", &call_arguments, memory)
                };
                pc = 97;
            }
            100 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_270"))?,
                        relocate("STACK_local_248"),
                        0x104u64,
                    ];
                    invoke("GetFileTitleW", &call_arguments, memory)
                };
                pc = 99;
            }
            101 => {
                pc = if (values[12]) != 0 { 98 } else { 100 };
            }
            102 => {
                values[12] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_270")];
                        invoke(
                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 101;
            }
            103 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 102;
            }
            104 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_270"),
                        (memory.read_u64(relocate("DAT_1401d07b0"))?).wrapping_add(0x18u64),
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 103;
            }
            105 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((memory.read_u64(relocate("DAT_1401d07b0"))?) as i64) != ((0x0u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u8(
                                (memory.read_u64(relocate("DAT_1401d07b0"))?).wrapping_add(0x10u64),
                            )?)) as i8 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    104
                } else {
                    47
                };
            }
            106 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 105;
            }
            107 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    106
                } else {
                    105
                };
            }
            108 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 107;
            }
            109 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 108;
            }
            110 => {
                values[14] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 109;
            }
            111 => {
                values[1] = (values[17]).wrapping_add(0xcu64);
                pc = 110;
            }
            112 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 111;
            }
            113 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(values[17])?)?, values[17]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 112;
            }
            114 => {
                pc = if (u64::from(
                    ((((values[25]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    113
                } else {
                    105
                };
            }
            115 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 114;
            }
            116 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 115;
            }
            117 => {
                values[25] = memory.read_u64(values[2])?;
                pc = 116;
            }
            118 => {
                values[2] = (values[17]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 117;
            }
            119 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 118;
            }
            120 => {
                pc = if (u64::from((values[17]) != (0x0u64))) != 0 {
                    119
                } else {
                    105
                };
            }
            121 => {
                {
                    let destination = (values[5]).wrapping_add((0x56u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                values[17] =
                    memory.read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?;
                pc = 121;
            }
            123 => {
                {
                    let destination = (values[5]).wrapping_add((0x55u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 123;
            }
            125 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    124
                } else {
                    123
                };
            }
            126 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 125;
            }
            127 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 126;
            }
            128 => {
                values[14] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 127;
            }
            129 => {
                values[1] = (values[17]).wrapping_add(0xcu64);
                pc = 128;
            }
            130 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 129;
            }
            131 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(values[17])?)?, values[17]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 130;
            }
            132 => {
                pc = if (u64::from(
                    ((((values[25]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    131
                } else {
                    123
                };
            }
            133 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 132;
            }
            134 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 133;
            }
            135 => {
                values[25] = memory.read_u64(values[2])?;
                pc = 134;
            }
            136 => {
                values[2] = (values[17]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 135;
            }
            137 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 136;
            }
            138 => {
                pc = if (u64::from((values[17]) != (0x0u64))) != 0 {
                    137
                } else {
                    123
                };
            }
            139 => {
                {
                    let destination = (values[5]).wrapping_add((0x54u64).wrapping_mul(8));
                    let stored = values[27];
                    memory.write_u64(destination, stored)?;
                }
                pc = 138;
            }
            140 => {
                values[17] =
                    memory.read_u64((values[5]).wrapping_add((0x54u64).wrapping_mul(8)))?;
                pc = 139;
            }
            141 => {
                {
                    let destination = (values[5]).wrapping_add((0x53u64).wrapping_mul(8));
                    let stored = values[26];
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            142 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 141;
            }
            143 => {
                {
                    let destination = (values[27]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = ((((memory
                        .read_u64((values[27]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 142;
            }
            144 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 143;
            }
            145 => {
                pc = if (u64::from((values[27]) != (0x0u64))) != 0 {
                    144
                } else {
                    141
                };
            }
            146 => {
                {
                    let call_arguments = [relocate("STACK_local_278")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 145;
            }
            147 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    146
                } else {
                    145
                };
            }
            148 => {
                {
                    let call_arguments = [relocate("STACK_local_270")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 147;
            }
            149 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    148
                } else {
                    147
                };
            }
            150 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_2d8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            151 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    150
                } else {
                    149
                };
            }
            152 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_270"))?)?)
                                .wrapping_add(0x138u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_270"))?,
                        0x142c80000u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 151;
            }
            153 => {
                {
                    let destination = relocate("STACK_local_2c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 153;
            }
            155 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2d8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0xfb5u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 154;
            }
            156 => {
                {
                    let destination = relocate("STACK_uStack_264");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_268");
                    let stored = u64::from((0x42c80000u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_2d8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            159 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    158
                } else {
                    157
                };
            }
            160 => {
                values[11] = memory.read_u64(relocate("STACK_local_270"))?;
                pc = 159;
            }
            161 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0xa8u64))?,
                        values[7],
                        relocate("STACK_local_270"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 160;
            }
            162 => {
                {
                    let destination = relocate("STACK_local_2c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 161;
            }
            163 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 162;
            }
            164 => {
                {
                    let destination = (relocate("STACK_local_2d8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 163;
            }
            165 => {
                {
                    let destination = relocate("STACK_local_270");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 164;
            }
            166 => {
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
                pc = 165;
            }
            167 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    166
                } else {
                    165
                };
            }
            168 => {
                {
                    let destination = relocate("STACK_local_278");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 167;
            }
            169 => {
                values[7] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 168;
            }
            170 => {
                {
                    let call_arguments = [
                        (values[25]).wrapping_add(0x1e0u64),
                        relocate("STACK_local_270"),
                    ];
                    invoke("FUN_1400a1130", &call_arguments, memory)
                };
                pc = 169;
            }
            171 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_270"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 170;
            }
            172 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_270"))?) != (0x0u64)))
                    != 0
                {
                    171
                } else {
                    170
                };
            }
            173 => {
                {
                    let destination = relocate("STACK_local_270");
                    let stored = memory.read_u64((values[26]).wrapping_add(0x998u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 172;
            }
            174 => {
                values[25] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[21])?).wrapping_add(0xf8u64))?,
                        values[21],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 173;
            }
            175 => {
                pc = if (u64::from((values[21]) != (0x0u64))) != 0 {
                    174
                } else {
                    173
                };
            }
            176 => {
                values[21] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 175;
            }
            177 => {
                pc = if (values[9]) != 0 { 46 } else { 176 };
            }
            178 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_258"))?)?)
                                .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_258"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 177;
            }
            179 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    178
                } else {
                    177
                };
            }
            180 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 179;
            }
            181 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 180;
            }
            182 => {
                values[14] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 181;
            }
            183 => {
                values[1] = (memory.read_u64(relocate("STACK_local_258"))?).wrapping_add(0xcu64);
                pc = 182;
            }
            184 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 183;
            }
            185 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_258"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_258"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 184;
            }
            186 => {
                pc = if (u64::from(
                    ((((values[10]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    185
                } else {
                    177
                };
            }
            187 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 186;
            }
            188 => {
                {
                    let destination = values[17];
                    let stored = ((((memory.read_u64(values[17])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 187;
            }
            189 => {
                values[10] = memory.read_u64(values[17])?;
                pc = 188;
            }
            190 => {
                values[17] = (memory.read_u64(relocate("STACK_local_258"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 189;
            }
            191 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 190;
            }
            192 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_258"))?) != (0x0u64)))
                    != 0
                {
                    191
                } else {
                    177
                };
            }
            193 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2d0"))?)?)
                                .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2d0"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 192;
            }
            194 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    193
                } else {
                    192
                };
            }
            195 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 194;
            }
            196 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 195;
            }
            197 => {
                values[14] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 196;
            }
            198 => {
                values[1] = (memory.read_u64(relocate("STACK_local_2d0"))?).wrapping_add(0xcu64);
                pc = 197;
            }
            199 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 198;
            }
            200 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_2d0"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_2d0"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 199;
            }
            201 => {
                pc = if (u64::from(
                    ((((values[10]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    200
                } else {
                    192
                };
            }
            202 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 201;
            }
            203 => {
                {
                    let destination = values[17];
                    let stored = ((((memory.read_u64(values[17])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 202;
            }
            204 => {
                values[10] = memory.read_u64(values[17])?;
                pc = 203;
            }
            205 => {
                values[17] = (memory.read_u64(relocate("STACK_local_2d0"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 204;
            }
            206 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 205;
            }
            207 => {
                pc = if (u64::from(
                    ((values[12]) != 0)
                        && ((u64::from(
                            (memory.read_u64(relocate("STACK_local_2d0"))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    206
                } else {
                    192
                };
            }
            208 => {
                values[9] = 1u64;
                pc = 207;
            }
            209 => {
                pc = 208;
            }
            210 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[19]).wrapping_add((0x4u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    209
                } else {
                    207
                };
            }
            211 => {
                values[9] = 0u64;
                pc = 210;
            }
            212 => {
                values[19] = {
                    let call_arguments = [memory.read_u64(values[18])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 211;
            }
            213 => {
                values[12] = 1u64;
                pc = 212;
            }
            214 => {
                values[18] = {
                    let call_arguments = [values[26], relocate("STACK_local_2d8")];
                    invoke("FUN_140015600", &call_arguments, memory)
                };
                pc = 213;
            }
            215 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[19]).wrapping_add((0x0u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    208
                } else {
                    214
                };
            }
            216 => {
                values[19] = {
                    let call_arguments = [memory.read_u64(values[18])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 215;
            }
            217 => {
                values[12] = 0u64;
                pc = 216;
            }
            218 => {
                values[18] = {
                    let call_arguments = [values[26], relocate("STACK_local_260")];
                    invoke("FUN_140015600", &call_arguments, memory)
                };
                pc = 217;
            }
            219 => {
                pc = 15;
            }
            220 => {
                pc = if (values[12]) != 0 { 219 } else { 218 };
            }
            221 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 220;
            }
            222 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    221
                } else {
                    220
                };
            }
            223 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 222;
            }
            224 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 223;
            }
            225 => {
                values[14] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 224;
            }
            226 => {
                values[1] = (values[17]).wrapping_add(0xcu64);
                pc = 225;
            }
            227 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 226;
            }
            228 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_2d0"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_2d0"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 227;
            }
            229 => {
                pc = if (u64::from(
                    ((((values[10]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    228
                } else {
                    220
                };
            }
            230 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 229;
            }
            231 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 230;
            }
            232 => {
                values[10] = memory.read_u64(values[2])?;
                pc = 231;
            }
            233 => {
                values[2] = (memory.read_u64(relocate("STACK_local_2d0"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 232;
            }
            234 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 233;
            }
            235 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2d0"))?) != (0x0u64)))
                    != 0
                {
                    234
                } else {
                    220
                };
            }
            236 => {
                values[17] = memory.read_u64(relocate("STACK_local_2d0"))?;
                pc = 235;
            }
            237 => {
                values[12] = 1u64;
                pc = 236;
            }
            238 => {
                values[12] = 0u64;
                pc = 236;
            }
            239 => {
                pc = if (u64::from(
                    ((u64::from(((memory.read_u64(values[17])?) as i64) == ((0x0u64) as i64)))
                        != 0)
                        && (({
                            let _ = {
                                values[13] = ({
                                    let call_arguments = [values[26], 0x0u64, 0x0u64];
                                    invoke("FUN_140015700", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[13]
                            };
                            u64::from(
                                (((values[13]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    237
                } else {
                    238
                };
            }
            240 => {
                values[17] = {
                    let call_arguments = [values[26], relocate("STACK_local_2d8")];
                    invoke("FUN_140015600", &call_arguments, memory)
                };
                pc = 239;
            }
            241 => {
                {
                    let call_arguments = [relocate("STACK_local_278")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 240;
            }
            242 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 241;
            }
            243 => {
                {
                    let destination = relocate("STACK_plStack_2b8");
                    let stored = values[27];
                    memory.write_u64(destination, stored)?;
                }
                pc = 242;
            }
            244 => {
                {
                    let destination = relocate("STACK_local_2c0");
                    let stored = values[26];
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            245 => {
                values[27] =
                    memory.read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?;
                pc = 244;
            }
            246 => {
                values[26] =
                    memory.read_u64((values[5]).wrapping_add((0x55u64).wrapping_mul(8)))?;
                pc = 245;
            }
            247 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 246;
            }
            248 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 247;
            }
            249 => {
                values[1] = (memory
                    .read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?)
                .wrapping_add(0x8u64);
                pc = 248;
            }
            250 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 249;
            }
            251 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    250
                } else {
                    246
                };
            }
            252 => {
                pc = 15;
            }
            253 => {
                values[27] = memory.read_u64(relocate("STACK_plStack_2b8"))?;
                pc = 252;
            }
            254 => {
                {
                    let call_arguments = [relocate("STACK_local_278")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 253;
            }
            255 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 254;
            }
            256 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    255
                } else {
                    251
                };
            }
            257 => {
                values[14] = ({
                    let call_arguments = [values[5], 0x1u64];
                    invoke("FUN_140061c50", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 256;
            }
            258 => {
                {
                    let call_arguments = [(values[5]).wrapping_add((0x57u64).wrapping_mul(8))];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::Empty",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 257;
            }
            259 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0xb8u64))?,
                        values[5],
                        memory.read_u64(relocate("STACK_local_278"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 258;
            }
            260 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0xc0u64))?,
                        values[5],
                        memory.read_u64(relocate("STACK_local_278"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 259;
            }
            261 => {
                {
                    let call_arguments = [relocate("STACK_local_270")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 259;
            }
            262 => {
                {
                    let call_arguments = [
                        values[6],
                        values[5],
                        memory.read_u64(relocate("STACK_local_270"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 261;
            }
            263 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_270"),
                        memory.read_u64(relocate("STACK_local_278"))?,
                        values[15],
                        values[7],
                        values[14],
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::Concatenate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 262;
            }
            264 => {
                values[15] = ({
                    let call_arguments = [relocate("STACK_local_278")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 263;
            }
            265 => {
                values[7] = (((u64::from(
                    (u64::from(memory.read_u32(relocate("STACK_uStack_264"))?)) as u32,
                )) & (0xffffffffu64))
                    .wrapping_shl((0x20u64) as u32))
                    | ((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_local_268"))?)) as u32,
                    )) & (0xffffffffu64));
                pc = 264;
            }
            266 => {
                values[14] = ({
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 265;
            }
            267 => {
                {
                    let call_arguments = [relocate("STACK_local_270"), values[16]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 266;
            }
            268 => {
                values[16] = {
                    let call_arguments = [relocate("STACK_local_278")];
                    invoke("ATL::CStringT<>::GetManager", &call_arguments, memory)
                };
                pc = 267;
            }
            269 => {
                values[6] = memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0xc0u64))?;
                pc = 268;
            }
            270 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[14]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[12] = u64::from(
                                    ({
                                        let call_arguments = [relocate("STACK_local_268")];
                                        invoke(
                                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[12]
                            };
                            values[12]
                        }) != 0),
                )) != 0
                {
                    260
                } else {
                    269
                };
            }
            271 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64(
                                    (values[5]).wrapping_add((0xau64).wrapping_mul(8)),
                                )?)?)
                            .wrapping_add(0xd8u64),
                        )?,
                        memory.read_u64((values[5]).wrapping_add((0xau64).wrapping_mul(8)))?,
                        relocate("STACK_local_268"),
                        0x4u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 270;
            }
            272 => {
                {
                    let call_arguments = [relocate("STACK_local_278"), 0xf003u64];
                    invoke("ATL::CStringT<>::LoadStringW", &call_arguments, memory)
                };
                pc = 271;
            }
            273 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[14]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[12] = u64::from(
                                    ({
                                        let call_arguments = [relocate("STACK_local_278")];
                                        invoke(
                                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[12]
                            };
                            values[12]
                        }) != 0),
                )) != 0
                {
                    272
                } else {
                    271
                };
            }
            274 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64(
                                    (values[5]).wrapping_add((0xau64).wrapping_mul(8)),
                                )?)?)
                            .wrapping_add(0xd8u64),
                        )?,
                        memory.read_u64((values[5]).wrapping_add((0xau64).wrapping_mul(8)))?,
                        relocate("STACK_local_278"),
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 273;
            }
            275 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 274;
            }
            276 => {
                {
                    let call_arguments = [relocate("STACK_local_278")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 275;
            }
            277 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[27])?).wrapping_add(0x8u64))?,
                        values[27],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 276;
            }
            278 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    277
                } else {
                    276
                };
            }
            279 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 278;
            }
            280 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 279;
            }
            281 => {
                values[14] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 280;
            }
            282 => {
                values[1] = (values[27]).wrapping_add(0xcu64);
                pc = 281;
            }
            283 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 282;
            }
            284 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(values[27])?)?, values[27]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 283;
            }
            285 => {
                pc = if (u64::from(
                    ((((values[26]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    284
                } else {
                    276
                };
            }
            286 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 285;
            }
            287 => {
                {
                    let destination = values[17];
                    let stored = ((((memory.read_u64(values[17])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 286;
            }
            288 => {
                values[26] = memory.read_u64(values[17])?;
                pc = 287;
            }
            289 => {
                values[17] = (values[27]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 288;
            }
            290 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 289;
            }
            291 => {
                pc = if (u64::from((values[27]) != (0x0u64))) != 0 {
                    290
                } else {
                    276
                };
            }
            292 => {
                {
                    let destination = relocate("STACK_plStack_2b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 291;
            }
            293 => {
                {
                    let destination = relocate("STACK_local_2c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 292;
            }
            294 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((((memory
                                .read_u64((values[5]).wrapping_add((0x58u64).wrapping_mul(8)))?)
                                as i8 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                ((((u64::from(memory.read_u32((values[26]).wrapping_add(0x18u64))?))
                                    as i32 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(
                            ((((memory
                                .read_u64((values[5]).wrapping_add((0x5bu64).wrapping_mul(8)))?)
                                as i8 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    293
                } else {
                    240
                };
            }
            295 => {
                {
                    let destination = relocate("STACK_plStack_2b8");
                    let stored = values[27];
                    memory.write_u64(destination, stored)?;
                }
                pc = 294;
            }
            296 => {
                {
                    let destination = relocate("STACK_local_2c0");
                    let stored = values[26];
                    memory.write_u64(destination, stored)?;
                }
                pc = 295;
            }
            297 => {
                values[27] =
                    memory.read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?;
                pc = 296;
            }
            298 => {
                values[26] =
                    memory.read_u64((values[5]).wrapping_add((0x55u64).wrapping_mul(8)))?;
                pc = 297;
            }
            299 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 298;
            }
            300 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 299;
            }
            301 => {
                values[1] = (memory
                    .read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?)
                .wrapping_add(0x8u64);
                pc = 300;
            }
            302 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 301;
            }
            303 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[5]).wrapping_add((0x56u64).wrapping_mul(8)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    302
                } else {
                    298
                };
            }
            304 => {
                return Ok(Some(0x0u64));
            }
            305 => {
                pc = if (u64::from(
                    ((u64::from((values[5]) == (0x0u64))) != 0)
                        || ((u64::from(
                            ((memory
                                .read_u64((values[5]).wrapping_add((0x55u64).wrapping_mul(8)))?)
                                as i64)
                                == ((0x0u64) as i64),
                        )) != 0),
                )) != 0
                {
                    304
                } else {
                    303
                };
            }
            306 => {
                values[5] = memory.read_u64((values[0]).wrapping_add(0xe8u64))?;
                pc = 305;
            }
            307 => {
                values[28] = (0x0u64) as i32 as i64 as u64;
                pc = 306;
            }
            308 => {
                values[25] = 0x0u64;
                pc = 307;
            }
            309 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_328"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 308;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14008b2c0` (`mspaint.c:120682`).
pub fn fun_14008b2c0(
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
    let mut pc: i32 = 224;
    loop {
        match pc {
            0 => {
                pc = 45;
            }
            1 => {
                pc = 15;
            }
            2 => {
                pc = if (u64::from(
                    (u64::from((0x63u64) as u32)) < (u64::from((values[20]) as u32)),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                values[17] = (values[17]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 2;
            }
            4 => {
                values[20] =
                    u64::from((u64::from(((values[20]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 3;
            }
            5 => {
                return Ok(Some(values[11]));
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_b8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [values[21]];
                    invoke("DeleteFileW", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                values[21] = memory.read_u64(
                    (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 7;
            }
            9 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_a0"))?))) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                values[21] = relocate("STACK_local_b8");
                pc = 9;
            }
            11 => {
                values[11] = u64::from((0x0u64) as u8);
                pc = 10;
            }
            12 => {
                pc = 10;
            }
            13 => {
                pc = 153;
            }
            14 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                values[11] = u64::from((0x0u64) as u8);
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_d8"))?) != (0x0u64))) != 0
                {
                    16
                } else {
                    15
                };
            }
            18 => {
                values[27] = 0u64;
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                values[26] = ((u64::from(memory.read_u8(relocate("STACK_local_198"))?)) as i8 as i64
                    as u64) as i8 as i64 as u64;
                pc = 19;
            }
            21 => {
                values[3] = memory.read_u64(relocate("STACK_local_180"))?;
                pc = 20;
            }
            22 => {
                values[27] = 1u64;
                pc = 21;
            }
            23 => {
                values[12] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_d8"))?)?)
                            .wrapping_add(0x38u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                pc = if (values[10]) != 0 { 23 } else { 17 };
            }
            25 => {
                values[26] = ((u64::from(memory.read_u8(relocate("STACK_local_198"))?)) as i8 as i64
                    as u64) as i8 as i64 as u64;
                pc = 24;
            }
            26 => {
                values[3] = memory.read_u64(relocate("STACK_local_180"))?;
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    (u64::from((values[20]) as u32)) < (u64::from((0x64u64) as u32)),
                )) != 0
                {
                    35
                } else {
                    26
                };
            }
            28 => {
                values[25] = (values[25]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 27;
            }
            29 => {
                values[17] = (values[17]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 28;
            }
            30 => {
                values[15] = values[20];
                pc = 29;
            }
            31 => {
                values[20] = u64::from(
                    ((((values[15]) as i32 as i64 as u64).wrapping_add(0x1u64)) as i32 as i64
                        as u64) as u32,
                );
                pc = 30;
            }
            32 => {
                values[2] = memory.read_u64(relocate("STACK_local_190"))?;
                pc = 31;
            }
            33 => {
                values[1] = memory.read_u64(relocate("STACK_local_188"))?;
                pc = 32;
            }
            34 => {
                values[10] = 1u64;
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[17])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[12] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(
                                                memory.read_u64(relocate("STACK_local_d8"))?,
                                            )?)
                                            .wrapping_add(0x30u64),
                                        )?,
                                        memory.read_u64(relocate("STACK_local_d8"))?,
                                        memory.read_u64(values[25])?,
                                        ((values[7]).wrapping_add((0x8u64).wrapping_mul(1)))
                                            .wrapping_add(
                                                ((values[15]).wrapping_mul(0x18u64))
                                                    .wrapping_mul(1),
                                            ),
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[12]
                            };
                            u64::from(
                                ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                                    as u64) as i64)
                                    < (((values[12]) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                values[17] = (values[7]).wrapping_add((0x8u64).wrapping_mul(1));
                pc = 35;
            }
            37 => {
                values[7] = (values[1]).wrapping_add((0x3u64).wrapping_mul(16));
                pc = 36;
            }
            38 => {
                values[25] = relocate("PTR_DAT_140105980");
                pc = 37;
            }
            39 => {
                values[15] = 0x0u64;
                pc = 38;
            }
            40 => {
                values[10] = 0u64;
                pc = 39;
            }
            41 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[14]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    17
                };
            }
            42 => {
                values[14] = ({
                    let call_arguments = [values[21], 0x1u64, relocate("STACK_local_d8")];
                    invoke("CreatePropertyStoreOnFile", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                pc = 4;
            }
            45 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16(values[17])?)) as i16 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    43
                };
            }
            46 => {
                values[20] = u64::from((0x0u64) as u32);
                pc = 45;
            }
            47 => {
                values[27] = 1u64;
                pc = 46;
            }
            48 => {
                values[17] = ((values[1]).wrapping_add((0x3u64).wrapping_mul(16)))
                    .wrapping_add((0x8u64).wrapping_mul(1));
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[12]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[6] = ((u64::from(
                                    (u64::from(
                                        memory.read_u32(
                                            ((values[1]).wrapping_add((0x3u64).wrapping_mul(16)))
                                                .wrapping_add((0x0u64).wrapping_mul(1)),
                                        )?,
                                    )) as u32,
                                )) | ((u64::from(
                                    (u64::from(
                                        memory.read_u16(
                                            ((values[1]).wrapping_add((0x3u64).wrapping_mul(16)))
                                                .wrapping_add((0x4u64).wrapping_mul(1)),
                                        )?,
                                    )) as u16,
                                ))
                                .wrapping_shl((0x20u64) as u32)))
                                    | ((u64::from(
                                        (u64::from(
                                            memory.read_u16(
                                                ((values[1])
                                                    .wrapping_add((0x3u64).wrapping_mul(16)))
                                                .wrapping_add((0x6u64).wrapping_mul(1)),
                                            )?,
                                        )) as u16,
                                    ))
                                    .wrapping_shl((0x30u64) as u32));
                                values[6]
                            };
                            u64::from(
                                ((u64::from(((values[6]) as i64) == ((0x0u64) as i64))) != 0)
                                    || (({
                                        let _ = {
                                            values[9] = ({
                                                let call_arguments = [values[21]];
                                                invoke("FUN_140089980", &call_arguments, memory)
                                            })
                                                as i8
                                                as i64
                                                as u64;
                                            values[9]
                                        };
                                        u64::from(
                                            (((values[9]) as i32 as i64 as u64) as i64)
                                                != (((0x0u64) as i32 as i64 as u64) as i64),
                                        )
                                    }) != 0),
                            )
                        }) != 0),
                )) != 0
                {
                    48
                } else {
                    11
                };
            }
            50 => {
                values[12] = ({
                    let call_arguments = [values[1], relocate("DAT_1400e4c10"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 49;
            }
            51 => {
                pc = 6;
            }
            52 => {
                {
                    let call_arguments = [relocate("DAT_1401d0900"), values[2]];
                    invoke("ATL::CStringT<>::operator=", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    52
                } else {
                    10
                };
            }
            54 => {
                values[11] = u64::from(
                    ({
                        let call_arguments = [values[21], values[2], values[3], values[26]];
                        invoke("FUN_140013240", &call_arguments, memory)
                    }) as u8,
                );
                pc = 53;
            }
            55 => {
                values[21] = memory.read_u64(
                    (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 54;
            }
            56 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_a0"))?))) != 0
                {
                    55
                } else {
                    54
                };
            }
            57 => {
                values[21] = relocate("STACK_local_b8");
                pc = 56;
            }
            58 => {
                pc = 10;
            }
            59 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    58
                } else {
                    57
                };
            }
            60 => {
                values[11] = u64::from((0x0u64) as u8);
                pc = 59;
            }
            61 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_b8"),
                        values[23],
                        memory.read_u64(relocate("STACK_local_68"))?,
                    ];
                    invoke("FUN_140014910", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                values[23] = memory.read_u64(
                    (relocate("STACK_local_78")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 62;
            }
            64 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_60"))?))) != 0
                {
                    63
                } else {
                    62
                };
            }
            65 => {
                values[23] = relocate("STACK_local_78");
                pc = 64;
            }
            66 => {
                pc = if (u64::from(
                    (((values[27]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    65
                } else {
                    61
                };
            }
            67 => {
                {
                    let call_arguments = [values[19], 0x0u64, values[8]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                values[19] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            70 => {
                pc = if (u64::from(
                    (((((values[12]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    69
                } else {
                    66
                };
            }
            71 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    68
                } else {
                    70
                };
            }
            72 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 71;
            }
            73 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                values[12] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 73;
            }
            75 => {
                values[4] = (values[8]).wrapping_add((0xcu64).wrapping_mul(2));
                pc = 74;
            }
            76 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 75;
            }
            77 => {
                pc = if (u64::from((values[8]) != (0x0u64))) != 0 {
                    76
                } else {
                    66
                };
            }
            78 => {
                {
                    let call_arguments = [relocate("STACK_local_f8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 77;
            }
            79 => {
                {
                    let call_arguments = [relocate("STACK_local_118")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 78;
            }
            80 => {
                {
                    let call_arguments = [relocate("STACK_local_138")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 79;
            }
            81 => {
                {
                    let call_arguments = [relocate("STACK_local_158")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 80;
            }
            82 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 81;
            }
            83 => {
                values[27] = u64::from(
                    ({
                        let call_arguments = [
                            values[18],
                            relocate("STACK_local_d8"),
                            relocate("STACK_local_158"),
                            relocate("STACK_local_f8"),
                        ];
                        invoke(
                            "Paint::Provenance::ProvenanceHelper::AuthoringFinalizeOutputToFile",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_d8"),
                        values[23],
                        memory.read_u64(relocate("STACK_local_128"))?,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x7u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x5u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x3u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination =
                        (relocate("STACK_auStack_d0")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                values[23] = memory.read_u64(
                    (relocate("STACK_local_138")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 95;
            }
            97 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_120"))?))) != 0
                {
                    96
                } else {
                    95
                };
            }
            98 => {
                values[23] = relocate("STACK_local_138");
                pc = 97;
            }
            99 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_158"),
                        values[23],
                        memory.read_u64(relocate("STACK_local_108"))?,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("STACK_local_148");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_uStack_150");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                values[23] = memory.read_u64(
                    (relocate("STACK_local_118")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 103;
            }
            105 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_100"))?))) != 0
                {
                    104
                } else {
                    103
                };
            }
            106 => {
                values[23] = relocate("STACK_local_118");
                pc = 105;
            }
            107 => {
                values[18] = {
                    let call_arguments = [values[1]];
                    invoke("FUN_140017300", &call_arguments, memory)
                };
                pc = 106;
            }
            108 => {
                {
                    let call_arguments = [relocate("STACK_local_138"), relocate("STACK_local_b8")];
                    invoke("FUN_1400199f0", &call_arguments, memory)
                };
                pc = 107;
            }
            109 => {
                {
                    let call_arguments = [relocate("STACK_local_118"), relocate("STACK_local_78")];
                    invoke("FUN_1400199f0", &call_arguments, memory)
                };
                pc = 108;
            }
            110 => {
                {
                    let call_arguments = [relocate("STACK_local_f8")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 109;
            }
            111 => {
                pc =
                    if (u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(
                                (values[22]).wrapping_add((values[24]).wrapping_mul(2)),
                            )?)) as u16,
                        )) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0
                    {
                        112
                    } else {
                        110
                    };
            }
            112 => {
                values[24] = (values[24]).wrapping_add(0x1u64);
                pc = 111;
            }
            113 => {
                values[24] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_uStack_e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("STACK_uStack_f0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                values[22] = relocate("DAT_140105708");
                pc = 117;
            }
            119 => {
                values[22] = 0x0u64;
                pc = 117;
            }
            120 => {
                values[22] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_190"))?)
                        .wrapping_add((0x8u64).wrapping_mul(2)),
                )?;
                pc = 117;
            }
            121 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_190"))?) == (0x0u64)))
                    != 0
                {
                    119
                } else {
                    120
                };
            }
            122 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_190"))?) == (0x0u64)))
                    != 0
                {
                    118
                } else {
                    121
                };
            }
            123 => {
                values[8] = memory.read_u64(relocate("STACK_local_190"))?;
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 123;
            }
            125 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_d8"))?) != (0x0u64))) != 0
                {
                    124
                } else {
                    123
                };
            }
            126 => {
                {
                    let call_arguments = [relocate("STACK_local_d8"), relocate("STACK_local_190")];
                    invoke("FUN_14004bcf0", &call_arguments, memory)
                };
                pc = 125;
            }
            127 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b78"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 126;
            }
            128 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b78"))?) != (0x0u64))) != 0
                {
                    127
                } else {
                    126
                };
            }
            129 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = memory.read_u64(relocate("DAT_1401d0b78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 128;
            }
            130 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_a8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    129
                } else {
                    66
                };
            }
            131 => {
                values[27] = u64::from(
                    (u64::from(
                        ((memory.read_u64(relocate("STACK_local_a8"))?) as i64)
                            != ((0x0u64) as i64),
                    )) != 0,
                );
                pc = 130;
            }
            132 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 131;
            }
            133 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke(
                        "Paint::PlatformHelpers::MakeUniqueTempFilePath",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 132;
            }
            134 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), values[2]];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 133;
            }
            135 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 134;
            }
            136 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x7u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x5u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x3u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 141;
            }
            143 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = ((relocate("STACK_local_98"))
                        .wrapping_add((0x8u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 143;
            }
            145 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 144;
            }
            146 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 145;
            }
            147 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 146;
            }
            148 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory
                                .read_u16((values[2]).wrapping_add((values[24]).wrapping_mul(2)))?,
                        )) as u16,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    149
                } else {
                    147
                };
            }
            149 => {
                values[24] = (values[24]).wrapping_add(0x1u64);
                pc = 148;
            }
            150 => {
                values[24] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 149;
            }
            151 => {
                pc = if (values[10]) != 0 { 150 } else { 60 };
            }
            152 => {
                values[10] = u64::from(
                    ({
                        let call_arguments = [((values[1])
                            .wrapping_add((0x9cu64).wrapping_mul(16)))
                        .wrapping_add((0x8u64).wrapping_mul(1))];
                        invoke(
                            "Paint::Provenance::ProvenanceHelper::HasAIContent",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 151;
            }
            153 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory.read_u8(
                                (((values[1]).wrapping_add((0xa2u64).wrapping_mul(16)))
                                    .wrapping_add((0x8u64).wrapping_mul(1)))
                                .wrapping_add((0x0u64).wrapping_mul(1)),
                            )?,
                        )) as u8,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    152
                } else {
                    57
                };
            }
            154 => {
                values[27] = 1u64;
                pc = 153;
            }
            155 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[26]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[9] = ({
                                    let call_arguments = [values[3], values[21], values[1]];
                                    invoke("FUN_14008a5e0", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[9]
                            };
                            u64::from(
                                (((values[9]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    50
                } else {
                    154
                };
            }
            156 => {
                {
                    let destination = relocate("STACK_local_198");
                    let stored = (values[26]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 155;
            }
            157 => {
                values[21] = memory.read_u64(
                    (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 156;
            }
            158 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_a0"))?))) != 0
                {
                    157
                } else {
                    156
                };
            }
            159 => {
                values[21] = relocate("STACK_local_b8");
                pc = 158;
            }
            160 => {
                values[26] = (0x0u64) as i8 as i64 as u64;
                pc = 159;
            }
            161 => {
                values[26] = (0x1u64) as i8 as i64 as u64;
                pc = 159;
            }
            162 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from((values[3]) == (0x0u64))) != 0)
                            || ((u64::from(
                                (((u64::from((u64::from(memory.read_u16(values[3])?)) as u16))
                                    as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        || (({
                            let _ = {
                                values[13] = u64::from(
                                    ({
                                        let call_arguments = [values[3]];
                                        invoke("GetFileAttributesW", &call_arguments, memory)
                                    }) as u32,
                                );
                                values[13]
                            };
                            u64::from(
                                (u64::from((values[13]) as u32))
                                    == (u64::from((0xffffffffu64) as u32)),
                            )
                        }) != 0),
                )) != 0
                {
                    160
                } else {
                    161
                };
            }
            163 => {
                pc = 6;
            }
            164 => {
                pc = if (u64::from(!((values[27]) != 0))) != 0 {
                    163
                } else {
                    162
                };
            }
            165 => {
                values[11] = u64::from((0x0u64) as u8);
                pc = 164;
            }
            166 => {
                values[27] = 0u64;
                pc = 165;
            }
            167 => {
                {
                    let call_arguments = [values[16], 0x28u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 165;
            }
            168 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    167
                } else {
                    165
                };
            }
            169 => {
                values[27] = u64::from(
                    (u64::from(
                        (((values[12]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0,
                );
                pc = 168;
            }
            170 => {
                values[12] = (0x0u64) as i32 as i64 as u64;
                pc = 169;
            }
            171 => {
                {
                    let destination = (values[0]).wrapping_add(0x10u64);
                    let stored = (values[12]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 169;
            }
            172 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    170
                } else {
                    171
                };
            }
            173 => {
                values[12] = ({
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x8u64))?,
                        values[21],
                        relocate("STACK_local_d8"),
                        values[16],
                    ];
                    invoke("GdipSaveImageToFile", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 172;
            }
            174 => {
                {
                    let destination = (values[16]).wrapping_add((0x8u64).wrapping_mul(4));
                    let stored = relocate("DAT_140119cd0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                {
                    let destination = (values[16]).wrapping_add((0x7u64).wrapping_mul(4));
                    let stored = u64::from((0x4u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 174;
            }
            176 => {
                {
                    let destination = (values[16]).wrapping_add((0x6u64).wrapping_mul(4));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let destination = (values[16]).wrapping_add((0x4u64).wrapping_mul(4));
                    let stored = 0xebe70551b35ddd9cu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 176;
            }
            178 => {
                {
                    let destination = (values[16]).wrapping_add((0x2u64).wrapping_mul(4));
                    let stored = 0x452dfa4a1d5be4b5u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 177;
            }
            179 => {
                {
                    let destination = values[16];
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 178;
            }
            180 => {
                values[16] = 0x0u64;
                pc = 179;
            }
            181 => {
                {
                    let destination = (values[16]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 179;
            }
            182 => {
                pc = if (u64::from((values[16]) == (0x0u64))) != 0 {
                    180
                } else {
                    181
                };
            }
            183 => {
                values[16] = {
                    let call_arguments = [0x28u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 182;
            }
            184 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[12]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[12] = ({
                                    let call_arguments = [
                                        relocate("STACK_local_d8"),
                                        relocate("STACK_local_98"),
                                        0x10u64,
                                    ];
                                    invoke("memcmp", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[12]
                            };
                            u64::from(
                                (((values[12]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    183
                } else {
                    173
                };
            }
            185 => {
                values[12] = ({
                    let call_arguments = [relocate("DAT_1400e4c10"), relocate("STACK_local_98")];
                    invoke("GetClsidOfEncoder", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 184;
            }
            186 => {
                values[16] = 0x0u64;
                pc = 185;
            }
            187 => {
                pc = if (u64::from(
                    (u64::from((values[20]) as u32))
                        < (u64::from(
                            (u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        ((values[1]).wrapping_add((0x2u64).wrapping_mul(16)))
                                            .wrapping_add((0x8u64).wrapping_mul(1)),
                                    )?,
                                )) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    192
                } else {
                    186
                };
            }
            188 => {
                values[15] = values[20];
                pc = 187;
            }
            189 => {
                values[20] = u64::from(
                    ((((values[15]) as i32 as i64 as u64).wrapping_add(0x1u64)) as i32 as i64
                        as u64) as u32,
                );
                pc = 188;
            }
            190 => {
                {
                    let destination = (values[0]).wrapping_add(0x10u64);
                    let stored = (values[12]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 189;
            }
            191 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    190
                } else {
                    189
                };
            }
            192 => {
                values[12] = ({
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x8u64))?,
                        (memory.read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(16)))?)
                            .wrapping_add((values[15]).wrapping_mul(0x18u64)),
                    ];
                    invoke("GdipSetPropertyItem", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 191;
            }
            193 => {
                pc = if (u64::from(
                    ((u64::from(((values[24]) as i64) != ((0x0u64) as i64))) != 0)
                        && ((u64::from(
                            ((((u64::from(
                                memory.read_u32(
                                    ((values[1]).wrapping_add((0x2u64).wrapping_mul(16)))
                                        .wrapping_add((0x8u64).wrapping_mul(1)),
                                )?,
                            )) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    192
                } else {
                    186
                };
            }
            194 => {
                {
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(
                                    ((values[1]).wrapping_add((0x2u64).wrapping_mul(16)))
                                        .wrapping_add((0x6u64).wrapping_mul(1)),
                                )?,
                            )) as u16,
                        )) as u16,
                    );
                    values[24] =
                        (values[24] & 0xffffffffffffu64) | ((stored << 48) & 0xffff000000000000u64);
                }
                pc = 193;
            }
            195 => {
                {
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(
                                    ((values[1]).wrapping_add((0x2u64).wrapping_mul(16)))
                                        .wrapping_add((0x4u64).wrapping_mul(1)),
                                )?,
                            )) as u16,
                        )) as u16,
                    );
                    values[24] =
                        (values[24] & 0xffff0000ffffffffu64) | ((stored << 32) & 0xffff00000000u64);
                }
                pc = 194;
            }
            196 => {
                {
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[1]).wrapping_add((0x2u64).wrapping_mul(16)))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        )) as u32,
                    );
                    values[24] = (values[24] & 0xffffffff00000000u64) | (stored & 0xffffffffu64);
                }
                pc = 195;
            }
            197 => {
                {
                    let destination = (values[0]).wrapping_add(0x10u64);
                    let stored = (values[12]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 196;
            }
            198 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    197
                } else {
                    196
                };
            }
            199 => {
                values[12] = ({
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x8u64))?,
                        values[5],
                        u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[1]).wrapping_add((0xa5u64).wrapping_mul(16)))
                                        .wrapping_add((0x8u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        ),
                    ];
                    invoke("GdipBitmapSetResolution", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 198;
            }
            200 => {
                {
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(
                                    ((values[1]).wrapping_add((0xa5u64).wrapping_mul(16)))
                                        .wrapping_add((0x6u64).wrapping_mul(1)),
                                )?,
                            )) as u16,
                        )) as u16,
                    );
                    values[5] =
                        (values[5] & 0xffffffff0000ffffu64) | ((stored << 16) & 0xffff0000u64);
                }
                pc = 199;
            }
            201 => {
                {
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(
                                    ((values[1]).wrapping_add((0xa5u64).wrapping_mul(16)))
                                        .wrapping_add((0x4u64).wrapping_mul(1)),
                                )?,
                            )) as u16,
                        )) as u16,
                    );
                    values[5] = (values[5] & 0xffffffffffff0000u64) | (stored & 0xffffu64);
                }
                pc = 200;
            }
            202 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    166
                } else {
                    201
                };
            }
            203 => {
                values[12] = ({
                    let call_arguments = [values[1], relocate("STACK_local_d8")];
                    invoke("GetClsidOfEncoder", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 202;
            }
            204 => {
                values[21] = memory.read_u64(
                    (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 203;
            }
            205 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_a0"))?))) != 0
                {
                    204
                } else {
                    203
                };
            }
            206 => {
                values[21] = relocate("STACK_local_b8");
                pc = 205;
            }
            207 => {
                return Ok(Some((values[15]) & (0xffffffffffffff00u64)));
            }
            208 => {
                values[15] = {
                    let call_arguments = [relocate("STACK_local_b8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 207;
            }
            209 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_a8"))?) as i64) == ((0x0u64) as i64),
                )) != 0
                {
                    208
                } else {
                    206
                };
            }
            210 => {
                {
                    let call_arguments = [relocate("STACK_local_178")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 209;
            }
            211 => {
                {
                    let call_arguments = [relocate("STACK_local_b8")];
                    invoke(
                        "Paint::PlatformHelpers::MakeUniqueTempFilePath",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 210;
            }
            212 => {
                {
                    let call_arguments = [relocate("STACK_local_178"), values[2]];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 211;
            }
            213 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 212;
            }
            214 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 213;
            }
            215 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 214;
            }
            216 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 215;
            }
            217 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 216;
            }
            218 => {
                {
                    let destination = relocate("STACK_uStack_170");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 217;
            }
            219 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 218;
            }
            220 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory
                                .read_u16((values[2]).wrapping_add((values[24]).wrapping_mul(2)))?,
                        )) as u16,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    221
                } else {
                    219
                };
            }
            221 => {
                values[24] = (values[24]).wrapping_add(0x1u64);
                pc = 220;
            }
            222 => {
                values[24] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 221;
            }
            223 => {
                values[15] = 0x0u64;
                pc = 222;
            }
            224 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_1b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 223;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140098890` (`mspaint.c:131957`).
pub fn fun_140098890(
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
        (arguments[3]) as i32 as i64 as u64,
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
    let mut pc: i32 = 326;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[11])?)?,
                        values[11],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((values[17]) != (0x0u64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_110")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            6 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x20u64))?,
                        values[17],
                        0x0u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\XamlCanvasPrintPreviewImageSourceAdapter.cpp\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination =
                        (relocate("STACK_local_110")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0xaau64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_78"))?];
                    invoke("GdipDeleteGraphics", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_78"))?,
                        values[16],
                        float_to_integer(
                            (f32::from_bits(
                                (u64::from(
                                    ((f32::from_bits((values[22]) as u32))
                                        + (f32::from_bits((values[19]) as u32)))
                                    .to_bits(),
                                )) as u32,
                            )) as f64,
                            32,
                            true,
                        )?,
                        float_to_integer((f32::from_bits((values[20]) as u32)) as f64, 32, true)?,
                    ];
                    invoke("GdipDrawImageRectRectI", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = (float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from((((values[10]) as i64) as f32).to_bits())) as u32,
                                )) - (f32::from_bits((values[21]) as u32)))
                                .to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = (float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((memory.read_u64(relocate("STACK_local_120"))?) as f32).to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_170");
                    let stored = ((u64::from(memory.read_u32(relocate("STACK_local_bc"))?)) as i32
                        as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = u64::from((0x2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_148");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                values[16] =
                    memory.read_u64((values[11]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 19;
            }
            21 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    20
                } else {
                    19
                };
            }
            22 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored =
                        ((((u64::from(memory.read_u32(relocate("STACK_local_b8"))?)) as i32 as i64
                            as u64)
                            .wrapping_sub(float_to_integer(
                                (f32::from_bits(
                                    (u64::from(
                                        ((memory.read_u64(relocate("STACK_local_120"))?) as f32)
                                            .to_bits(),
                                    )) as u32,
                                )) as f64,
                                32,
                                true,
                            )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = (((values[18]).wrapping_sub(
                        (u64::from(memory.read_u32(relocate("STACK_local_bc"))?)) as i32 as i64
                            as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored = (float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from(
                                        ((((((u64::from(
                                            memory.read_u32(relocate("STACK_local_c0"))?,
                                        )) as i32
                                            as i64
                                            as u64)
                                            .wrapping_sub(
                                                (u64::from(
                                                    memory.read_u32(relocate("STACK_local_118"))?,
                                                ))
                                                    as i32
                                                    as i64
                                                    as u64,
                                            )) as i32
                                            as i64
                                            as u64)
                                            as i64)
                                            as f32)
                                            .to_bits(),
                                    )) as u32,
                                )) - ((u64::from(
                                    ((u64::from(
                                        (u64::from(memory.read_u32(
                                            (relocate("STACK_local_138")).wrapping_add(0x4u64),
                                        )?)) as u32,
                                    ))
                                    .wrapping_add(u64::from(
                                        (u64::from(memory.read_u32(
                                            (relocate("STACK_local_138")).wrapping_add(0x4u64),
                                        )?)) as u32,
                                    ))) as u32,
                                )) as f32))
                                    .to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_110")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            26 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    24
                };
            }
            27 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x48u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_138"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                values[16] = 0x0u64;
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination =
                        (relocate("STACK_local_110")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5371u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                values[21] =
                    u64::from(
                        ((f32::from_bits(
                            (u64::from(
                                ((u64::from(
                                    (u64::from(memory.read_u32(
                                        (relocate("STACK_local_f8")).wrapping_add(0x0u64),
                                    )?)) as u32,
                                )) as f32)
                                    .to_bits(),
                            )) as u32,
                        )) + (f32::from_bits(
                            (u64::from(
                                ((u64::from(
                                    (u64::from(memory.read_u32(
                                        (relocate("STACK_local_f8")).wrapping_add(0x0u64),
                                    )?)) as u32,
                                )) as f32)
                                    .to_bits(),
                            )) as u32,
                        )))
                        .to_bits(),
                    );
                pc = 33;
            }
            35 => {
                values[10] = ((((u64::from(memory.read_u32(relocate("STACK_local_e0"))?)) as i32
                    as i64 as u64)
                    .wrapping_sub(values[10])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_110")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            37 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x48u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_f8"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination =
                        (relocate("STACK_local_110")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5371u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x8u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x0u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                values[20] = u64::from(
                    ((f32::from_bits(
                        (u64::from(
                            ((((u64::from(memory.read_u32(relocate("STACK_local_118"))?)) as i32
                                as i64 as u64) as i64) as f32)
                                .to_bits(),
                        )) as u32,
                    )) + (f32::from_bits(
                        (u64::from(
                            ((u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_b0")).wrapping_add(0x4u64),
                                    )?,
                                )) as u32,
                            )) as f32)
                                .to_bits(),
                        )) as u32,
                    )))
                    .to_bits(),
                );
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_110")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            46 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    45
                } else {
                    44
                };
            }
            47 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x48u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_b0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination =
                        (relocate("STACK_local_110")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5371u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = (relocate("STACK_local_b0")).wrapping_add(0x8u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (relocate("STACK_local_b0")).wrapping_add(0x0u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                values[19] = u64::from(
                    ((f32::from_bits(
                        (u64::from(
                            ((((u64::from(memory.read_u32(relocate("STACK_local_c4"))?)) as i32
                                as i64 as u64) as i64) as f32)
                                .to_bits(),
                        )) as u32,
                    )) + (f32::from_bits(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_a0"))?) as f32).to_bits(),
                        )) as u32,
                    )))
                    .to_bits(),
                );
                pc = 52;
            }
            54 => {
                values[22] = u64::from(
                    ((f32::from_bits(
                        (u64::from(
                            (((({
                                let numerator =
                                    ((u64::from(memory.read_u32(relocate("STACK_local_c8"))?))
                                        as i32 as i64 as u64)
                                        as i32;
                                let denominator = ((u64::from(
                                    memory.read_u32((values[0]).wrapping_add(0x74u64))?,
                                )) as i32 as i64
                                    as u64)
                                    as i32;
                                if denominator == 0 {
                                    return Err(AdapterError::Arithmetic("division by zero"));
                                }
                                numerator
                                    .checked_rem(denominator)
                                    .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                    as u64
                            }) as i32 as i64 as u64) as i64) as f32)
                                .to_bits(),
                        )) as u32,
                    )) * ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_d0")).wrapping_add(0x0u64))?,
                        )) as u32,
                    )) as f32))
                        .to_bits(),
                );
                pc = 53;
            }
            55 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_110")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            56 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    55
                } else {
                    54
                };
            }
            57 => {
                values[10] = ((u64::from(memory.read_u32(relocate("STACK_local_c4"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 56;
            }
            58 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x60u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_d0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination =
                        (relocate("STACK_local_110")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x53a7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_110")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            64 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    62
                };
            }
            65 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x48u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_a0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination =
                        (relocate("STACK_local_110")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5371u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("STACK_uStack_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_uStack_88");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_uStack_88")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[10]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                values[10] = (values[7]) as i32 as i64 as u64;
                pc = 71;
            }
            73 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    72
                } else {
                    71
                };
            }
            74 => {
                values[7] = ({
                    let call_arguments =
                        [
                            values[16],
                            u64::from(
                                ((f32::from_bits(
                                    (u64::from(
                                        (u64::from(memory.read_u32(
                                            memory.read_u64(relocate("STACK_local_d0"))?,
                                        )?)) as u32,
                                    )) as u32,
                                )) / (f32::from_bits(
                                    (u64::from((u64::from(memory.read_u32(values[15])?)) as u32))
                                        as u32,
                                )))
                                .to_bits(),
                            ),
                            u64::from(
                                ((f32::from_bits((values[19]) as u32))
                                    / (f32::from_bits((values[22]) as u32)))
                                .to_bits(),
                            ),
                            0x0u64,
                        ];
                    invoke("GdipScaleWorldTransform", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 73;
            }
            75 => {
                values[15] = {
                    let call_arguments = [values[0], relocate("STACK_local_a0")];
                    invoke("FUN_140098760", &call_arguments, memory)
                };
                pc = 74;
            }
            76 => {
                values[22] = u64::from(
                    (u64::from(memory.read_u32((values[14]).wrapping_add(0x4u64))?)) as u32,
                );
                pc = 75;
            }
            77 => {
                values[19] = u64::from(
                    (u64::from(
                        memory.read_u32(
                            (memory.read_u64(relocate("STACK_local_d0"))?)
                                .wrapping_add((0x1u64).wrapping_mul(4)),
                        )?,
                    )) as u32,
                );
                pc = 76;
            }
            78 => {
                values[14] = {
                    let call_arguments = [values[0], relocate("STACK_local_b0")];
                    invoke("FUN_140098760", &call_arguments, memory)
                };
                pc = 77;
            }
            79 => {
                {
                    let destination = (relocate("STACK_uStack_88")).wrapping_add(0x0u64);
                    let stored = u64::from((values[10]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(relocate("STACK_local_78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                values[16] = memory.read_u64(relocate("STACK_local_78"))?;
                pc = 80;
            }
            82 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_70"))?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("GdipCreateFromHDC", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_uStack_88");
                    let stored = 0x0u64;
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
                {
                    let call_arguments = [values[10], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            87 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    86
                } else {
                    85
                };
            }
            88 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x18u64))?,
                        values[17],
                        0x0u64,
                        relocate("STACK_local_70"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 87;
            }
            89 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x14011b650u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_138")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x92u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            95 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    94
                } else {
                    93
                };
            }
            96 => {
                values[17] = memory.read_u64(relocate("STACK_local_78"))?;
                pc = 95;
            }
            97 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[6])?)?,
                        values[6],
                        relocate("DAT_14011b760"),
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105690u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_138")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x84bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    101
                } else {
                    93
                };
            }
            103 => {
                values[6] = memory.read_u64(values[13])?;
                pc = 102;
            }
            104 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 103;
            }
            105 => {
                values[13] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?)
                            .wrapping_add(0x18u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 104;
            }
            106 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            107 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    106
                } else {
                    105
                };
            }
            108 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[6])?)?,
                        values[6],
                        relocate("DAT_14011b4e8"),
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105690u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_138")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x84bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    111
                } else {
                    105
                };
            }
            113 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                values[6] = memory.read_u64(
                    memory.read_u64((relocate("STACK_local_b0")).wrapping_add(0x0u64))?,
                )?;
                pc = 113;
            }
            115 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 114;
            }
            116 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    115
                } else {
                    114
                };
            }
            117 => {
                {
                    let call_arguments = [relocate("STACK_local_d8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 116;
            }
            118 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_d8"))?) != (0x0u64))) != 0
                {
                    117
                } else {
                    116
                };
            }
            119 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = values[13];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 119;
            }
            121 => {
                values[11] = memory.read_u64(values[13])?;
                pc = 120;
            }
            122 => {
                values[13] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(values[11])?)?).wrapping_add(0x18u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 121;
            }
            123 => {
                values[11] = {
                    let call_arguments = [relocate("STACK_local_78"), relocate("STACK_local_d8")];
                    invoke("FUN_1400606c0", &call_arguments, memory)
                };
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_138")];
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
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64((values[0]).wrapping_add(0x30u64))?)?)
                            .wrapping_add(0x98u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add(0x30u64))?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 125;
            }
            127 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 127;
            }
            129 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_138")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x455bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 128;
            }
            130 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 129;
            }
            131 => {
                values[17] = 0x0u64;
                pc = 130;
            }
            132 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64)
                            < (((values[10]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                < (((values[18]) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    131
                } else {
                    0
                };
            }
            133 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 132;
            }
            134 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    133
                } else {
                    132
                };
            }
            135 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 134;
            }
            136 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 135;
            }
            137 => {
                values[7] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 136;
            }
            138 => {
                values[5] = (values[17]).wrapping_add(0xcu64);
                pc = 137;
            }
            139 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 138;
            }
            140 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_130"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_130"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 139;
            }
            141 => {
                pc = if (u64::from(
                    ((((values[14]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    140
                } else {
                    132
                };
            }
            142 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 141;
            }
            143 => {
                {
                    let destination = values[13];
                    let stored = ((((memory.read_u64(values[13])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 142;
            }
            144 => {
                values[14] = memory.read_u64(values[13])?;
                pc = 143;
            }
            145 => {
                values[13] = (memory.read_u64(relocate("STACK_plStack_130"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 144;
            }
            146 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 145;
            }
            147 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_130"))?) != (0x0u64)))
                    != 0
                {
                    146
                } else {
                    132
                };
            }
            148 => {
                values[17] = memory.read_u64(relocate("STACK_plStack_130"))?;
                pc = 147;
            }
            149 => {
                values[18] = ({
                    let call_arguments = [
                        ((values[8]).wrapping_sub(values[18])) as i32 as i64 as u64,
                        (u64::from(
                            memory.read_u32((values[12]).wrapping_add((0x4u64).wrapping_mul(1)))?,
                        )) as i32 as i64 as u64,
                        float_to_integer((f32::from_bits((values[19]) as u32)) as f64, 32, true)?,
                    ];
                    invoke("MulDiv", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 148;
            }
            150 => {
                values[12] = {
                    let call_arguments = [memory.read_u64(values[11])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 149;
            }
            151 => {
                values[11] = {
                    let call_arguments = [values[0], relocate("STACK_local_138")];
                    invoke("FUN_140099480", &call_arguments, memory)
                };
                pc = 150;
            }
            152 => {
                values[19] = u64::from(
                    ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_78")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as f32)
                        .to_bits(),
                );
                pc = 151;
            }
            153 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            154 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    153
                } else {
                    152
                };
            }
            155 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 154;
            }
            156 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 155;
            }
            157 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x8u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5395u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 157;
            }
            159 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 158;
            }
            160 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 159;
            }
            161 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    160
                } else {
                    159
                };
            }
            162 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 161;
            }
            163 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 162;
            }
            164 => {
                values[7] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 163;
            }
            165 => {
                values[5] = (values[17]).wrapping_add(0xcu64);
                pc = 164;
            }
            166 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 165;
            }
            167 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_130"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_130"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 166;
            }
            168 => {
                pc = if (u64::from(
                    ((((values[14]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    167
                } else {
                    159
                };
            }
            169 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 168;
            }
            170 => {
                {
                    let destination = values[13];
                    let stored = ((((memory.read_u64(values[13])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 169;
            }
            171 => {
                values[14] = memory.read_u64(values[13])?;
                pc = 170;
            }
            172 => {
                values[13] = (memory.read_u64(relocate("STACK_plStack_130"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 171;
            }
            173 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 172;
            }
            174 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_130"))?) != (0x0u64)))
                    != 0
                {
                    173
                } else {
                    159
                };
            }
            175 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = (values[10]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 174;
            }
            176 => {
                values[17] = memory.read_u64(relocate("STACK_plStack_130"))?;
                pc = 175;
            }
            177 => {
                values[10] = ({
                    let call_arguments = [
                        (((u64::from(memory.read_u32(relocate("STACK_local_e0"))?)) as i32 as i64
                            as u64)
                            .wrapping_sub(values[7])) as i32 as i64 as u64,
                        (u64::from(
                            memory.read_u32((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))?,
                        )) as i32 as i64 as u64,
                        float_to_integer((f32::from_bits((values[19]) as u32)) as f64, 32, true)?,
                    ];
                    invoke("MulDiv", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 176;
            }
            178 => {
                values[12] = {
                    let call_arguments = [memory.read_u64(values[11])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 177;
            }
            179 => {
                values[11] = {
                    let call_arguments = [values[0], relocate("STACK_local_138")];
                    invoke("FUN_140099480", &call_arguments, memory)
                };
                pc = 178;
            }
            180 => {
                values[19] =
                    u64::from(((memory.read_u64(relocate("STACK_local_78"))?) as f32).to_bits());
                pc = 179;
            }
            181 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            182 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    181
                } else {
                    180
                };
            }
            183 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x8u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 184;
            }
            186 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5395u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 185;
            }
            187 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 186;
            }
            188 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 187;
            }
            189 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    188
                } else {
                    187
                };
            }
            190 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 189;
            }
            191 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 190;
            }
            192 => {
                values[10] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 191;
            }
            193 => {
                values[5] = (values[17]).wrapping_add(0xcu64);
                pc = 192;
            }
            194 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 193;
            }
            195 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_130"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_130"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 194;
            }
            196 => {
                pc = if (u64::from(
                    ((((values[14]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    195
                } else {
                    187
                };
            }
            197 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 196;
            }
            198 => {
                {
                    let destination = values[13];
                    let stored = ((((memory.read_u64(values[13])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 197;
            }
            199 => {
                values[14] = memory.read_u64(values[13])?;
                pc = 198;
            }
            200 => {
                values[13] = (memory.read_u64(relocate("STACK_plStack_130"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 199;
            }
            201 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 200;
            }
            202 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_130"))?) != (0x0u64)))
                    != 0
                {
                    201
                } else {
                    187
                };
            }
            203 => {
                values[17] = memory.read_u64(relocate("STACK_plStack_130"))?;
                pc = 202;
            }
            204 => {
                {
                    let destination = relocate("STACK_local_bc");
                    let stored = ({
                        let call_arguments =
                            [
                                (((u64::from(memory.read_u32(relocate("STACK_local_118"))?)) as i32
                                    as i64 as u64)
                                    .wrapping_sub(values[18]))
                                    as i32 as i64 as u64,
                                (u64::from(memory.read_u32(
                                    (values[12]).wrapping_add((0x4u64).wrapping_mul(1)),
                                )?)) as i32 as i64 as u64,
                                float_to_integer(
                                    (f32::from_bits((values[19]) as u32)) as f64,
                                    32,
                                    true,
                                )?,
                            ];
                        invoke("MulDiv", &call_arguments, memory)
                    }) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 203;
            }
            205 => {
                values[12] = {
                    let call_arguments = [memory.read_u64(values[11])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 204;
            }
            206 => {
                values[11] = {
                    let call_arguments = [values[0], relocate("STACK_local_138")];
                    invoke("FUN_140099480", &call_arguments, memory)
                };
                pc = 205;
            }
            207 => {
                values[19] = u64::from(
                    ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_78")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as f32)
                        .to_bits(),
                );
                pc = 206;
            }
            208 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            209 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    208
                } else {
                    207
                };
            }
            210 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 209;
            }
            211 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 210;
            }
            212 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x8u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 211;
            }
            213 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5395u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 212;
            }
            214 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 213;
            }
            215 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[17])?).wrapping_add(0x8u64))?,
                        values[17],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 214;
            }
            216 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    215
                } else {
                    214
                };
            }
            217 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 216;
            }
            218 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 217;
            }
            219 => {
                values[10] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 218;
            }
            220 => {
                values[5] = (values[17]).wrapping_add(0xcu64);
                pc = 219;
            }
            221 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 220;
            }
            222 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_130"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_130"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 221;
            }
            223 => {
                pc = if (u64::from(
                    ((((values[14]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    222
                } else {
                    214
                };
            }
            224 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 223;
            }
            225 => {
                {
                    let destination = values[13];
                    let stored = ((((memory.read_u64(values[13])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 224;
            }
            226 => {
                values[14] = memory.read_u64(values[13])?;
                pc = 225;
            }
            227 => {
                values[13] = (memory.read_u64(relocate("STACK_plStack_130"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 226;
            }
            228 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 227;
            }
            229 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_130"))?) != (0x0u64)))
                    != 0
                {
                    228
                } else {
                    214
                };
            }
            230 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_120")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[10]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 229;
            }
            231 => {
                values[17] = memory.read_u64(relocate("STACK_plStack_130"))?;
                pc = 230;
            }
            232 => {
                values[10] = ({
                    let call_arguments = [
                        ((values[10]).wrapping_sub(values[7])) as i32 as i64 as u64,
                        (u64::from(
                            memory.read_u32((values[12]).wrapping_add((0x0u64).wrapping_mul(1)))?,
                        )) as i32 as i64 as u64,
                        float_to_integer((f32::from_bits((values[19]) as u32)) as f64, 32, true)?,
                    ];
                    invoke("MulDiv", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 231;
            }
            233 => {
                values[12] = {
                    let call_arguments = [memory.read_u64(values[11])?];
                    invoke("BasePaint::Image::Size", &call_arguments, memory)
                };
                pc = 232;
            }
            234 => {
                values[11] = {
                    let call_arguments = [values[0], relocate("STACK_local_138")];
                    invoke("FUN_140099480", &call_arguments, memory)
                };
                pc = 233;
            }
            235 => {
                values[19] =
                    u64::from(((memory.read_u64(relocate("STACK_local_78"))?) as f32).to_bits());
                pc = 234;
            }
            236 => {
                {
                    let call_arguments = [values[9], relocate("STACK_local_f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            237 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    236
                } else {
                    235
                };
            }
            238 => {
                values[9] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 237;
            }
            239 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 238;
            }
            240 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 239;
            }
            241 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x8u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 240;
            }
            242 => {
                {
                    let destination = (relocate("STACK_local_f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5395u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 241;
            }
            243 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 242;
            }
            244 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64)
                            < (((float_to_integer(
                                (f32::from_bits((values[19]) as u32)) as f64,
                                32,
                                true,
                            )?) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                < (((values[8]) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    243
                } else {
                    0
                };
            }
            245 => {
                values[8] =
                    (float_to_integer((f32::from_bits((values[22]) as u32)) as f64, 32, true)?)
                        as i32 as i64 as u64;
                pc = 244;
            }
            246 => {
                values[22] = u64::from(
                    ((f32::from_bits(
                        (u64::from((((values[18]) as i64) as f32).to_bits())) as u32,
                    )) + ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_78")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as f32))
                        .to_bits(),
                );
                pc = 245;
            }
            247 => {
                pc = if (u64::from(
                    (f32::from_bits(
                        (u64::from(
                            ((f32::from_bits(
                                (u64::from((((values[18]) as i64) as f32).to_bits())) as u32,
                            )) + ((u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_78")).wrapping_add(0x4u64),
                                    )?,
                                )) as u32,
                            )) as f32))
                                .to_bits(),
                        )) as u32,
                    )) <= ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_120")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as f32),
                )) != 0
                {
                    246
                } else {
                    245
                };
            }
            248 => {
                values[22] = u64::from(
                    ((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_120")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) as f32)
                        .to_bits(),
                );
                pc = 247;
            }
            249 => {
                {
                    let call_arguments = [values[8], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            250 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    249
                } else {
                    248
                };
            }
            251 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 250;
            }
            252 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 251;
            }
            253 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 252;
            }
            254 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_138")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5395u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 253;
            }
            255 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 254;
            }
            256 => {
                {
                    let call_arguments = [values[8], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            257 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    256
                } else {
                    255
                };
            }
            258 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x60u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_120"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 257;
            }
            259 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored =
                        (float_to_integer((f32::from_bits((values[19]) as u32)) as f64, 32, true)?)
                            as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 258;
            }
            260 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 259;
            }
            261 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 260;
            }
            262 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x53a7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 261;
            }
            263 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 262;
            }
            264 => {
                values[19] = u64::from(
                    ((f32::from_bits((u64::from((((values[7]) as i64) as f32).to_bits())) as u32))
                        + (f32::from_bits(
                            (u64::from(
                                ((memory.read_u64(relocate("STACK_local_78"))?) as f32).to_bits(),
                            )) as u32,
                        )))
                    .to_bits(),
                );
                pc = 263;
            }
            265 => {
                pc = if (u64::from(
                    (f32::from_bits(
                        (u64::from(
                            ((f32::from_bits(
                                (u64::from((((values[7]) as i64) as f32).to_bits())) as u32,
                            )) + (f32::from_bits(
                                (u64::from(
                                    ((memory.read_u64(relocate("STACK_local_78"))?) as f32)
                                        .to_bits(),
                                )) as u32,
                            )))
                            .to_bits(),
                        )) as u32,
                    )) <= (f32::from_bits(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_120"))?) as f32).to_bits(),
                        )) as u32,
                    )),
                )) != 0
                {
                    264
                } else {
                    263
                };
            }
            266 => {
                values[19] =
                    u64::from(((memory.read_u64(relocate("STACK_local_120"))?) as f32).to_bits());
                pc = 265;
            }
            267 => {
                {
                    let call_arguments = [values[8], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            268 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    267
                } else {
                    266
                };
            }
            269 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 268;
            }
            270 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 269;
            }
            271 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 270;
            }
            272 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5395u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 271;
            }
            273 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 272;
            }
            274 => {
                {
                    let call_arguments = [values[8], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            275 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    274
                } else {
                    273
                };
            }
            276 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x60u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_120"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 275;
            }
            277 => {
                {
                    let destination = relocate("STACK_local_c4");
                    let stored = (values[10]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 276;
            }
            278 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 277;
            }
            279 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 278;
            }
            280 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x53a7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 279;
            }
            281 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 280;
            }
            282 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 281;
            }
            283 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    282
                } else {
                    281
                };
            }
            284 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = (values[18]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 283;
            }
            285 => {
                values[10] = (0x0u64) as i32 as i64 as u64;
                pc = 284;
            }
            286 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    285
                } else {
                    284
                };
            }
            287 => {
                values[10] = (values[7]) as i32 as i64 as u64;
                pc = 286;
            }
            288 => {
                values[18] = (float_to_integer(
                    (f32::from_bits(
                        (u64::from(
                            (((u64::from(
                                (u64::from(memory.read_u32(
                                    (relocate("STACK_local_120")).wrapping_add(0x4u64),
                                )?)) as u32,
                            )) as f32)
                                - (f32::from_bits(
                                    (u64::from(
                                        ((f32::from_bits(
                                            (u64::from(
                                                (((({
                                                    let numerator = (((((values[18])
                                                        .wrapping_add(values[3]))
                                                        as i32
                                                        as i64
                                                        as u64)
                                                        .wrapping_add(
                                                            (0u64.wrapping_sub(0x1u64)) as i32
                                                                as i64
                                                                as u64,
                                                        ))
                                                        as i32
                                                        as i64
                                                        as u64)
                                                        as i32;
                                                    let denominator = (values[8]) as i32;
                                                    if denominator == 0 {
                                                        return Err(AdapterError::Arithmetic(
                                                            "division by zero",
                                                        ));
                                                    }
                                                    numerator.checked_div(denominator).ok_or(
                                                        AdapterError::Arithmetic(
                                                            "signed division overflow",
                                                        ),
                                                    )?
                                                        as u64
                                                })
                                                    as i32
                                                    as i64
                                                    as u64)
                                                    as i64)
                                                    as f32)
                                                    .to_bits(),
                                            )) as u32,
                                        )) * ((u64::from(
                                            (u64::from(memory.read_u32(
                                                (relocate("STACK_local_78")).wrapping_add(0x4u64),
                                            )?)) as u32,
                                        )) as f32))
                                            .to_bits(),
                                    )) as u32,
                                )))
                            .to_bits(),
                        )) as u32,
                    )) as f64,
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 287;
            }
            289 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            290 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    289
                } else {
                    288
                };
            }
            291 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x50u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_120"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 290;
            }
            292 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 291;
            }
            293 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 292;
            }
            294 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5383u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 293;
            }
            295 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 294;
            }
            296 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            297 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    296
                } else {
                    295
                };
            }
            298 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x60u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 297;
            }
            299 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 298;
            }
            300 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 299;
            }
            301 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x53a7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 300;
            }
            302 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 301;
            }
            303 => {
                values[7] = (float_to_integer(
                    (f32::from_bits(
                        (u64::from(
                            ((f32::from_bits(
                                (u64::from(
                                    ((memory.read_u64(relocate("STACK_local_120"))?) as f32)
                                        .to_bits(),
                                )) as u32,
                            )) - (f32::from_bits(
                                (u64::from(
                                    ((f32::from_bits(
                                        (u64::from(
                                            (((({
                                                let numerator = (((((values[10]).wrapping_add(
                                                    (0u64.wrapping_sub(0x1u64)) as i32 as i64
                                                        as u64,
                                                ))
                                                    as i32
                                                    as i64
                                                    as u64)
                                                    .wrapping_add(values[3]))
                                                    as i32
                                                    as i64
                                                    as u64)
                                                    as i32;
                                                let denominator = (values[7]) as i32;
                                                if denominator == 0 {
                                                    return Err(AdapterError::Arithmetic(
                                                        "division by zero",
                                                    ));
                                                }
                                                numerator.checked_rem(denominator).ok_or(
                                                    AdapterError::Arithmetic(
                                                        "signed division overflow",
                                                    ),
                                                )?
                                                    as u64
                                            }) as i32
                                                as i64
                                                as u64)
                                                as i64)
                                                as f32)
                                                .to_bits(),
                                        )) as u32,
                                    )) * (f32::from_bits(
                                        (u64::from(
                                            ((memory.read_u64(relocate("STACK_local_78"))?) as f32)
                                                .to_bits(),
                                        )) as u32,
                                    )))
                                    .to_bits(),
                                )) as u32,
                            )))
                            .to_bits(),
                        )) as u32,
                    )) as f64,
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 302;
            }
            304 => {
                {
                    let call_arguments = [values[9], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            305 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    304
                } else {
                    303
                };
            }
            306 => {
                values[9] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x50u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_120"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 305;
            }
            307 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 306;
            }
            308 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 307;
            }
            309 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5383u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 308;
            }
            310 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 309;
            }
            311 => {
                {
                    let call_arguments = [values[9], relocate("STACK_local_138")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            312 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    311
                } else {
                    310
                };
            }
            313 => {
                values[9] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x60u64),
                        )?,
                        memory.read_u64(values[4])?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 312;
            }
            314 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 313;
            }
            315 => {
                {
                    let destination = relocate("STACK_plStack_130");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 314;
            }
            316 => {
                {
                    let destination = (relocate("STACK_local_138")).wrapping_add(0x0u64);
                    let stored = u64::from((0x53a7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 315;
            }
            317 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 316;
            }
            318 => {
                values[18] = ((u64::from(memory.read_u32((values[0]).wrapping_add(0x70u64))?))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 317;
            }
            319 => {
                values[8] = ({
                    let call_arguments = [values[4]];
                    invoke("FUN_140099400", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 318;
            }
            320 => {
                values[10] = ((u64::from(memory.read_u32((values[0]).wrapping_add(0x70u64))?))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 319;
            }
            321 => {
                values[7] = ({
                    let call_arguments = [values[4]];
                    invoke("FUN_140099400", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 320;
            }
            322 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = (values[3]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 321;
            }
            323 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 322;
            }
            324 => {
                values[4] = (values[0]).wrapping_add(0x68u64);
                pc = 323;
            }
            325 => {
                {
                    let destination = (relocate("STACK_local_b0")).wrapping_add(0x0u64);
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 324;
            }
            326 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_1a8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 325;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14009e430` (`mspaint.c:136506`).
pub fn fun_14009e430(
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
    let mut pc: i32 = 41;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_48"))?) != (0x0u64))) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_50"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_50"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_50"))?) != (0x0u64))) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let call_arguments = [
                        u64::from(
                            (relocate("CONTEXT_IGNORED_FUN_14009e430_extraout_XMM0_Da")) as u32,
                        ),
                        values[8],
                    ];
                    invoke("FUN_1400177a0", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    6
                };
            }
            9 => {
                values[4] = ({
                    let call_arguments = [
                        values[3],
                        relocate("STACK_local_50"),
                        0x1u64,
                        relocate("STACK_local_40"),
                    ];
                    invoke("FUN_140017480", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_50"),
                        relocate("STACK_local_48"),
                        (values[3]).wrapping_add(0x20u64),
                        (values[3]).wrapping_add(0x28u64),
                        (values[3]).wrapping_add(0x30u64),
                        (((values[3]) as i32 as i64 as u64).wrapping_add(0x38u64)) as i32 as i64
                            as u64,
                        (((u64::from(
                            (u64::from(
                                memory.read_u32(relocate("STACK_in_stack_ffffffffffffff9c"))?,
                            )) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl((0x20u64) as u32))
                            | ((0x64u64) & (0xffffffffu64)),
                    ];
                    invoke("GdiplusHelpers::LoadStreamImage", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_pcStack_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                values[3] = memory.read_u64((values[8]).wrapping_add(0x298u64))?;
                pc = 13;
            }
            15 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0x2e8u64))?
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                values[7] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0xf8u64))?,
                        values[6],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    16
                } else {
                    15
                };
            }
            18 => {
                values[6] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = memory.read_u64(relocate("STACK_local_50"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_40")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            21 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    19
                };
            }
            22 => {
                values[5] = ({
                    let call_arguments = [
                        values[9],
                        relocate("DAT_140108230"),
                        relocate("STACK_local_50"),
                    ];
                    invoke(
                        "CreateStreamOverRandomAccessStream",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_pcStack_38");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_40")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0xaa9u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [values[5], relocate("STACK_local_40")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            29 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    27
                };
            }
            30 => {
                values[9] = memory.read_u64(relocate("STACK_local_50"))?;
                pc = 29;
            }
            31 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[2])?)?,
                        values[2],
                        relocate("PTR_140108388"),
                        relocate("STACK_local_50"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_pcStack_38");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_40")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x84bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                pc = if (u64::from((values[2]) != (0x0u64))) != 0 {
                    35
                } else {
                    27
                };
            }
            37 => {
                values[9] = values[7];
                pc = 36;
            }
            38 => {
                values[2] = memory.read_u64(values[1])?;
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520")];
                    invoke("FUN_14006fa00", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                values[7] = 0x0u64;
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_98"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14009e900` (`mspaint.c:136737`).
pub fn fun_14009e900(
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
    let mut pc: i32 = 803;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            2 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            3 => {
                return Ok(None);
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_50")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_50"))?) != (0x0u64))) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_58"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    10
                } else {
                    9
                };
            }
            12 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[21])?).wrapping_add(0x8u64))?,
                        values[21],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
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
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                values[13] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 15;
            }
            17 => {
                values[2] = (values[21]).wrapping_add(0xcu64);
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
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_40"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_40"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    ((((values[9]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
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
                    let destination = values[23];
                    let stored = ((((memory.read_u64(values[23])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                values[9] = memory.read_u64(values[23])?;
                pc = 22;
            }
            24 => {
                values[23] = (memory.read_u64(relocate("STACK_plStack_40"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 23;
            }
            25 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 24;
            }
            26 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_40"))?) != (0x0u64)))
                    != 0
                {
                    25
                } else {
                    11
                };
            }
            27 => {
                values[21] = memory.read_u64(relocate("STACK_plStack_40"))?;
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [relocate("STACK_local_1f0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                pc = if (u64::from((values[28]) != (0x0u64))) != 0 {
                    28
                } else {
                    27
                };
            }
            30 => {
                {
                    let call_arguments = [relocate("STACK_local_230")];
                    invoke("FUN_1400a10d0", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [relocate("STACK_local_1e8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                pc = if (u64::from((values[21]) != (0x0u64))) != 0 {
                    31
                } else {
                    30
                };
            }
            33 => {
                {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_70"))?) != (0x0u64))) != 0
                {
                    33
                } else {
                    32
                };
            }
            35 => {
                {
                    let call_arguments = [relocate("STACK_local_238")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_238"))?) != (0x0u64)))
                    != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    37
                } else {
                    36
                };
            }
            39 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x1du64).wrapping_mul(8)),
                        relocate("STACK_local_90"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("FUN_140092970", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), relocate("STACK_local_88")];
                    invoke("FUN_140093750", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_88"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [
                        values[0],
                        relocate("STACK_local_238"),
                        relocate("STACK_local_258"),
                    ];
                    invoke("FUN_1400a3770", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    46
                } else {
                    45
                };
            }
            48 => {
                {
                    let call_arguments = [relocate("STACK_local_218")];
                    invoke("FUN_140069aa0", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x1bu64).wrapping_mul(8)),
                        values[18],
                    ];
                    invoke("FUN_1400a7470", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                values[18] = {
                    let call_arguments = [relocate("STACK_local_238"), relocate("STACK_local_218")];
                    invoke("FUN_1400a0f90", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), values[0]];
                    invoke("FUN_1400a4da0", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [relocate("STACK_local_70"), relocate("STACK_local_238")];
                    invoke("FUN_1400a0e90", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 52;
            }
            54 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    53
                } else {
                    52
                };
            }
            55 => {
                {
                    let call_arguments = [relocate("STACK_local_218")];
                    invoke("FUN_1400698f0", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x17u64).wrapping_mul(8)),
                        values[18],
                    ];
                    invoke("FUN_1400a7250", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                values[18] = {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x5u64).wrapping_mul(8)),
                        relocate("STACK_local_218"),
                    ];
                    invoke("FUN_1400a0d50", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), values[0]];
                    invoke("FUN_1400a4d20", &call_arguments, memory)
                };
                pc = 57;
            }
            59 => {
                {
                    let call_arguments = [
                        values[0],
                        relocate("DAT_1401d06a0"),
                        relocate("STACK_local_218"),
                    ];
                    invoke("FUN_1400a38d0", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                {
                    let call_arguments = [relocate("STACK_local_218"), relocate("PTR_1401073c8")];
                    invoke("FUN_1400052e0", &call_arguments, memory)
                };
                pc = 59;
            }
            61 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    61
                } else {
                    60
                };
            }
            63 => {
                {
                    let destination = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[23])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                values[23] = {
                    let call_arguments = [
                        relocate("DAT_1401d06c8"),
                        relocate("STACK_local_250"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("FUN_1400786a0", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), values[0]];
                    invoke("FUN_1400a4ca0", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 65;
            }
            67 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    66
                } else {
                    65
                };
            }
            68 => {
                {
                    let call_arguments = [relocate("STACK_local_218")];
                    invoke("FUN_140069b30", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x13u64).wrapping_mul(8)),
                        values[18],
                    ];
                    invoke("FUN_140093350", &call_arguments, memory)
                };
                pc = 68;
            }
            70 => {
                values[18] = {
                    let call_arguments = [relocate("STACK_local_70"), relocate("STACK_local_218")];
                    invoke("FUN_1400927a0", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), values[0]];
                    invoke("FUN_1400a4c20", &call_arguments, memory)
                };
                pc = 70;
            }
            72 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 71;
            }
            73 => {
                pc = if (u64::from((values[25]) != (0x0u64))) != 0 {
                    72
                } else {
                    71
                };
            }
            74 => {
                {
                    let call_arguments = [relocate("STACK_local_1d0")];
                    invoke("FUN_140069a10", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x11u64).wrapping_mul(8)),
                        relocate("STACK_local_1d0"),
                    ];
                    invoke("FUN_1400a7360", &call_arguments, memory)
                };
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_1c8");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let call_arguments = [relocate("STACK_local_200")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    77
                } else {
                    76
                };
            }
            79 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_88"),
                        values[30],
                        relocate("STACK_local_218"),
                    ];
                    invoke("FUN_140002770", &call_arguments, memory)
                };
                pc = 78;
            }
            80 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [values[7], values[3], values[18]];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 79;
            }
            81 => {
                values[18] = {
                    let call_arguments = [relocate("STACK_local_1d0")];
                    invoke("FUN_14000b9b0", &call_arguments, memory)
                };
                pc = 80;
            }
            82 => {
                values[7] = memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("STACK_local_200"))?)?)
                        .wrapping_add((0x3u64).wrapping_mul(8)),
                )?;
                pc = 81;
            }
            83 => {
                values[3] = memory.read_u64(relocate("STACK_local_200"))?;
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [relocate("STACK_local_200")];
                    invoke("FUN_1400185b0", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = (relocate("STACK_local_218")).wrapping_add(0x0u64);
                    let stored = u64::from((0x1160u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add((0x1du64).wrapping_mul(8)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    87
                } else {
                    76
                };
            }
            89 => {
                {
                    let destination = relocate("STACK_local_1d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = relocate("STACK_local_1d0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[13]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    90
                } else {
                    2
                };
            }
            92 => {
                values[16] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 91;
            }
            93 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1du64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x60u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x1du64).wrapping_mul(8)))?,
                        values[25],
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 92;
            }
            94 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = (relocate("STACK_local_218")).wrapping_add(0x0u64);
                    let stored = u64::from((0x8au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                values[25] = memory.read_u64(relocate("STACK_local_250"))?;
                pc = 98;
            }
            100 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_250"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 101;
            }
            103 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 102;
            }
            104 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = relocate("FUN_1400a3140");
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 105;
            }
            107 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_250"))?) != (0x0u64)))
                    != 0
                {
                    106
                } else {
                    98
                };
            }
            108 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let call_arguments = [values[6], values[10]];
                    invoke("FUN_14004cad0", &call_arguments, memory)
                };
                pc = 108;
            }
            110 => {
                values[10] = u64::from(
                    ({
                        let call_arguments = [values[0]];
                        invoke("FUN_1400a3d60", &call_arguments, memory)
                    }) as u8,
                );
                pc = 109;
            }
            111 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 110;
            }
            112 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    111
                } else {
                    110
                };
            }
            113 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[13]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    112
                } else {
                    1
                };
            }
            114 => {
                values[13] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x1du64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x38u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_218");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_218")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x33u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                values[16] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 118;
            }
            120 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_70"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_70"))?,
                        relocate("DAT_14011a2f0"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_70"))?) != (0x0u64))) != 0
                {
                    121
                } else {
                    118
                };
            }
            123 => {
                values[16] = values[25];
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [relocate("STACK_local_70"), relocate("STACK_local_88")];
                    invoke("FUN_1400a0c70", &call_arguments, memory)
                };
                pc = 123;
            }
            125 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_88")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 124;
            }
            126 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                ((values[14] as u32).wrapping_shr((0x16u64) as u32) as u64) as u32,
                            )) & (0x1u64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    125
                } else {
                    123
                };
            }
            127 => {
                values[14] = u64::from(
                    ({
                        let call_arguments =
                            [values[6], (0u64.wrapping_sub(0x14u64)) as i32 as i64 as u64];
                        invoke("GetWindowLongW", &call_arguments, memory)
                    }) as u32,
                );
                pc = 126;
            }
            128 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 127;
            }
            129 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    128
                } else {
                    127
                };
            }
            130 => {
                {
                    let call_arguments = [relocate("STACK_local_1c0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 129;
            }
            131 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1c0"))?) != (0x0u64)))
                    != 0
                {
                    130
                } else {
                    129
                };
            }
            132 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 131;
            }
            133 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_90"))?,
                        memory.read_u64(relocate("STACK_local_1b8"))?,
                    ];
                    invoke("FUN_140069e98", &call_arguments, memory)
                };
                pc = 132;
            }
            134 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    133
                } else {
                    131
                };
            }
            135 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 134;
            }
            136 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_1c0"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_1c0"))?,
                        relocate("DAT_1401151b8"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 135;
            }
            137 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1c0"))?) != (0x0u64)))
                    != 0
                {
                    137
                } else {
                    129
                };
            }
            139 => {
                {
                    let call_arguments = [relocate("STACK_local_250")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 138;
            }
            140 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 139;
            }
            141 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_90"))?, values[9]];
                    invoke("FUN_140069e98", &call_arguments, memory)
                };
                pc = 140;
            }
            142 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    141
                } else {
                    139
                };
            }
            143 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_250"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_250"))?,
                        relocate("DAT_1401151b8"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_250"))?) != (0x0u64)))
                    != 0
                {
                    145
                } else {
                    138
                };
            }
            147 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                values[25] = 0x0u64;
                pc = 147;
            }
            149 => {
                {
                    let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_248"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                pc = 149;
            }
            151 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_250"),
                        (values[0]).wrapping_add((0xfu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140093420", &call_arguments, memory)
                };
                pc = 150;
            }
            152 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored =
                        memory.read_u64((values[19]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 151;
            }
            153 => {
                {
                    let destination = values[19];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 153;
            }
            155 => {
                values[19] = {
                    let call_arguments = [relocate("STACK_local_70"), relocate("STACK_local_1c0")];
                    invoke("FUN_1400a0aa0", &call_arguments, memory)
                };
                pc = 154;
            }
            156 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 155;
            }
            157 => {
                values[25] = memory.read_u64(relocate("STACK_local_250"))?;
                pc = 156;
            }
            158 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_250"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 157;
            }
            159 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 158;
            }
            160 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 159;
            }
            161 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 160;
            }
            162 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = relocate("FUN_1400a1110");
                    memory.write_u64(destination, stored)?;
                }
                pc = 161;
            }
            163 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 162;
            }
            164 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 163;
            }
            165 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_250"))?) != (0x0u64)))
                    != 0
                {
                    164
                } else {
                    156
                };
            }
            166 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 165;
            }
            167 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[13]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    166
                } else {
                    0
                };
            }
            168 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_70"))?)?)
                                .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_70"))?,
                        values[21],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 167;
            }
            169 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                values[25] = 0x0u64;
                pc = 169;
            }
            171 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                {
                    let destination = relocate("STACK_local_218");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_218")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1e5u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 171;
            }
            173 => {
                {
                    let call_arguments = [
                        u64::from(
                            (relocate("CONTEXT_IGNORED_FUN_14009e900_extraout_XMM0_Da_00")) as u32,
                        ),
                        relocate("STACK_local_70"),
                        relocate("STACK_local_258"),
                    ];
                    invoke("FUN_1400acd40", &call_arguments, memory)
                };
                pc = 172;
            }
            174 => {
                {
                    let destination = relocate("_DAT_1401d0108");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = relocate("FUN_1400acfa0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 174;
            }
            176 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 175;
            }
            177 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 176;
            }
            178 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 172;
            }
            179 => {
                {
                    let destination = relocate("_DAT_1401d0108");
                    let stored = (memory.read_u64(relocate("_DAT_1401d0108"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 178;
            }
            180 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 179;
            }
            181 => {
                {
                    let call_arguments = [
                        u64::from(
                            (relocate("CONTEXT_IGNORED_FUN_14009e900_extraout_XMM0_Da_00")) as u32,
                        ),
                        relocate("STACK_local_70"),
                        relocate("DAT_1401d0100"),
                    ];
                    invoke("FUN_1400acec0", &call_arguments, memory)
                };
                pc = 180;
            }
            182 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0100"))?) == (0x0u64))) != 0
                {
                    177
                } else {
                    181
                };
            }
            183 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("_DAT_1401d0108");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d0108"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 184;
            }
            186 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = relocate("DAT_1401d0108");
                    memory.write_u64(destination, stored)?;
                }
                pc = 185;
            }
            187 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            188 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    187
                } else {
                    186
                };
            }
            189 => {
                values[9] = memory.read_u64(relocate("_DAT_1401d0108"))?;
                pc = 188;
            }
            190 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64(memory.read_u64(
                                    (values[0]).wrapping_add((0x3u64).wrapping_mul(8)),
                                )?)?)
                            .wrapping_add(0x1f0u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?,
                        values[25],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 189;
            }
            191 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 190;
            }
            192 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 191;
            }
            193 => {
                {
                    let destination = relocate("STACK_local_218");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_218")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x23ebu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 192;
            }
            194 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 193;
            }
            195 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    194
                } else {
                    193
                };
            }
            196 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            197 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    196
                } else {
                    195
                };
            }
            198 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_90"))?)?)
                                .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_90"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 197;
            }
            199 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 198;
            }
            200 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Content.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 199;
            }
            201 => {
                {
                    let destination = relocate("STACK_local_218");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_218")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0xc8cu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 200;
            }
            202 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 201;
            }
            203 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            204 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    203
                } else {
                    202
                };
            }
            205 => {
                values[16] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 204;
            }
            206 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1du64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x58u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x1du64).wrapping_mul(8)))?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 205;
            }
            207 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 206;
            }
            208 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 207;
            }
            209 => {
                {
                    let destination = (relocate("STACK_local_218")).wrapping_add(0x0u64);
                    let stored = u64::from((0x78u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 208;
            }
            210 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 209;
            }
            211 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_218")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            212 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    211
                } else {
                    210
                };
            }
            213 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1du64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x88u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x1du64).wrapping_mul(8)))?,
                        memory.read_u64(relocate("STACK_local_90"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 212;
            }
            214 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 213;
            }
            215 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 214;
            }
            216 => {
                {
                    let destination = (relocate("STACK_local_218")).wrapping_add(0x0u64);
                    let stored = u64::from((0xe7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 215;
            }
            217 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            218 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    217
                } else {
                    216
                };
            }
            219 => {
                values[25] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 218;
            }
            220 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("DAT_1401d0320"))?,
                        memory.read_u64(relocate("STACK_local_218"))?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 219;
            }
            221 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 220;
            }
            222 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x14011a5b0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 221;
            }
            223 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5eu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 222;
            }
            224 => {
                {
                    let call_arguments = [relocate("STACK_local_250")];
                    invoke("FUN_14000a550", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a550"));
            }
            225 => {
                pc = if (values[29]) != 0 { 224 } else { 223 };
            }
            226 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 225;
            }
            227 => {
                {
                    let destination = relocate("DAT_1401d0350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 226;
            }
            228 => {
                pc = if (values[29]) != 0 { 227 } else { 226 };
            }
            229 => {
                values[29] = u64::from(
                    (u64::from((memory.read_u64(relocate("DAT_1401d0350"))?) == (0x0u64))) != 0,
                );
                pc = 228;
            }
            230 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 229;
            }
            231 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 230;
            }
            232 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x14011a5b0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 231;
            }
            233 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x56u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 232;
            }
            234 => {
                values[6] = memory.read_u64(relocate("STACK_local_218"))?;
                pc = 233;
            }
            235 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 234;
            }
            236 => {
                {
                    let destination = relocate("DAT_1401d0350");
                    let stored = values[24];
                    memory.write_u64(destination, stored)?;
                }
                pc = 235;
            }
            237 => {
                values[24] = memory.read_u64(relocate("DAT_1401d0350"))?;
                pc = 236;
            }
            238 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0350"))?) != (0x0u64))) != 0
                {
                    237
                } else {
                    236
                };
            }
            239 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 238;
            }
            240 => {
                {
                    let call_arguments = [relocate("STACK_local_250")];
                    invoke("FUN_14000a550", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a550"));
            }
            241 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("DAT_1401d0320"))?) == (0x0u64))) != 0)
                        || ((u64::from(
                            ((u64::from(
                                ((u64::from(
                                    ((u64::from(
                                        (memory.read_u64(relocate("DAT_1401d0328"))?) == (0x0u64),
                                    )) != 0)
                                        || ((u64::from(
                                            (memory.read_u64(relocate("DAT_1401d0330"))?)
                                                == (0x0u64),
                                        )) != 0),
                                )) != 0)
                                    || ((u64::from(
                                        (memory.read_u64(relocate("DAT_1401d0338"))?) == (0x0u64),
                                    )) != 0),
                            )) != 0)
                                || ((u64::from(
                                    ((u64::from(
                                        (memory.read_u64(relocate("DAT_1401d0340"))?) == (0x0u64),
                                    )) != 0)
                                        || ((u64::from(
                                            (memory.read_u64(relocate("_DAT_1401d0348"))?)
                                                == (0x0u64),
                                        )) != 0),
                                )) != 0),
                        )) != 0),
                )) != 0
                {
                    240
                } else {
                    239
                };
            }
            242 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 241;
            }
            243 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x14011a5b0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 242;
            }
            244 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x45u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            245 => {
                {
                    let destination = relocate("_DAT_1401d0348");
                    let stored = {
                        let call_arguments =
                            [values[24], relocate("\"Windowing_GetIconFromIconId\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 244;
            }
            246 => {
                {
                    let destination = relocate("DAT_1401d0340");
                    let stored = {
                        let call_arguments =
                            [values[24], relocate("\"Windowing_GetIconIdFromIcon\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 245;
            }
            247 => {
                {
                    let destination = relocate("DAT_1401d0338");
                    let stored = {
                        let call_arguments = [
                            values[24],
                            relocate("\"Windowing_GetMonitorFromDisplayId\""),
                        ];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 246;
            }
            248 => {
                {
                    let destination = relocate("DAT_1401d0330");
                    let stored = {
                        let call_arguments = [
                            values[24],
                            relocate("\"Windowing_GetDisplayIdFromMonitor\""),
                        ];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 247;
            }
            249 => {
                {
                    let destination = relocate("DAT_1401d0328");
                    let stored = {
                        let call_arguments =
                            [values[24], relocate("\"Windowing_GetWindowFromWindowId\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 248;
            }
            250 => {
                {
                    let destination = relocate("DAT_1401d0320");
                    let stored = {
                        let call_arguments =
                            [values[24], relocate("\"Windowing_GetWindowIdFromWindow\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 249;
            }
            251 => {
                pc = if (u64::from(
                    ((values[29]) != 0)
                        && (({
                            let _ = {
                                values[24] = {
                                    let call_arguments =
                                        [relocate("L\"Microsoft.Internal.FrameworkUdk.dll\"")];
                                    invoke("GetModuleHandleW", &call_arguments, memory)
                                };
                                values[24]
                            };
                            u64::from(
                                ((u64::from((values[24]) != (0x0u64))) != 0)
                                    || (({
                                        let _ = {
                                            values[24] = {
                                                let call_arguments = [relocate(
                                                    "L\"Microsoft.Internal.FrameworkUdk.dll\"",
                                                )];
                                                invoke("LoadLibraryW", &call_arguments, memory)
                                            };
                                            values[24]
                                        };
                                        u64::from((values[24]) != (0x0u64))
                                    }) != 0),
                            )
                        }) != 0),
                )) != 0
                {
                    250
                } else {
                    234
                };
            }
            252 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 251;
            }
            253 => {
                {
                    let destination = relocate("DAT_1401d0350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 252;
            }
            254 => {
                pc = if (values[29]) != 0 { 253 } else { 252 };
            }
            255 => {
                values[29] = u64::from(
                    (u64::from((memory.read_u64(relocate("DAT_1401d0350"))?) == (0x0u64))) != 0,
                );
                pc = 254;
            }
            256 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 255;
            }
            257 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 256;
            }
            258 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 257;
            }
            259 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    258
                } else {
                    257
                };
            }
            260 => {
                {
                    let call_arguments = [relocate("STACK_local_1b0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 259;
            }
            261 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1b0"))?) != (0x0u64)))
                    != 0
                {
                    260
                } else {
                    259
                };
            }
            262 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 261;
            }
            263 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_90"))?,
                        memory.read_u64(relocate("STACK_local_1a8"))?,
                    ];
                    invoke("FUN_140018690", &call_arguments, memory)
                };
                pc = 262;
            }
            264 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    263
                } else {
                    261
                };
            }
            265 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 264;
            }
            266 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_1b0"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_1b0"))?,
                        relocate("DAT_140105598"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 265;
            }
            267 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 266;
            }
            268 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1b0"))?) != (0x0u64)))
                    != 0
                {
                    267
                } else {
                    259
                };
            }
            269 => {
                {
                    let call_arguments = [relocate("DAT_1401d0b90"), values[18]];
                    invoke("FUN_140041af0", &call_arguments, memory)
                };
                pc = 268;
            }
            270 => {
                values[18] = {
                    let call_arguments = [relocate("DAT_1401d0b70"), relocate("STACK_local_1b0")];
                    invoke("FUN_14003cd30", &call_arguments, memory)
                };
                pc = 269;
            }
            271 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 270;
            }
            272 => {
                values[25] = memory.read_u64(relocate("STACK_local_250"))?;
                pc = 271;
            }
            273 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_250"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 272;
            }
            274 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 273;
            }
            275 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 274;
            }
            276 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 275;
            }
            277 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = relocate("FUN_14006e3f0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 276;
            }
            278 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = relocate("DAT_1401d0520");
                    memory.write_u64(destination, stored)?;
                }
                pc = 277;
            }
            279 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 278;
            }
            280 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_250"))?) != (0x0u64)))
                    != 0
                {
                    279
                } else {
                    271
                };
            }
            281 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 280;
            }
            282 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[21])?).wrapping_add(0x8u64))?,
                        values[21],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 281;
            }
            283 => {
                pc = if (u64::from((values[21]) != (0x0u64))) != 0 {
                    282
                } else {
                    281
                };
            }
            284 => {
                {
                    let destination = relocate("DAT_1401d0b70");
                    let stored = values[21];
                    memory.write_u64(destination, stored)?;
                }
                pc = 283;
            }
            285 => {
                values[21] = (((u64::from(
                    (u64::from(memory.read_u32(relocate("STACK_uStack_1e4"))?)) as u32,
                )) & (0xffffffffu64))
                    .wrapping_shl((0x20u64) as u32))
                    | (((u64::from(memory.read_u32(relocate("STACK_local_1e8"))?)) as i32 as i64
                        as u64)
                        & (0xffffffffu64));
                pc = 284;
            }
            286 => {
                {
                    let call_arguments = [relocate("DAT_1401d0b70")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 285;
            }
            287 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    286
                } else {
                    285
                };
            }
            288 => {
                {
                    let call_arguments = [relocate("STACK_local_1f8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 287;
            }
            289 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1f8"))?) != (0x0u64)))
                    != 0
                {
                    288
                } else {
                    287
                };
            }
            290 => {
                {
                    let call_arguments = [relocate("STACK_local_238")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 289;
            }
            291 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_238"))?) != (0x0u64)))
                    != 0
                {
                    290
                } else {
                    289
                };
            }
            292 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 291;
            }
            293 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    292
                } else {
                    291
                };
            }
            294 => {
                {
                    let call_arguments = [
                        values[30],
                        relocate("STACK_local_1e8"),
                        relocate("STACK_local_180"),
                    ];
                    invoke("FUN_1400ad120", &call_arguments, memory)
                };
                pc = 293;
            }
            295 => {
                {
                    let destination = relocate("_DAT_1401d0168");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 294;
            }
            296 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 295;
            }
            297 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 296;
            }
            298 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 293;
            }
            299 => {
                {
                    let destination = relocate("_DAT_1401d0168");
                    let stored = (memory.read_u64(relocate("_DAT_1401d0168"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 298;
            }
            300 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 299;
            }
            301 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_180"),
                        relocate("STACK_local_1e8"),
                        relocate("DAT_1401d0160"),
                    ];
                    invoke("FUN_1400acfc0", &call_arguments, memory)
                };
                pc = 300;
            }
            302 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0160"))?) == (0x0u64))) != 0
                {
                    297
                } else {
                    301
                };
            }
            303 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 302;
            }
            304 => {
                {
                    let destination = relocate("_DAT_1401d0168");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d0168"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 303;
            }
            305 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 304;
            }
            306 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = relocate("DAT_1401d0168");
                    memory.write_u64(destination, stored)?;
                }
                pc = 305;
            }
            307 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = relocate("STACK_local_98");
                    memory.write_u64(destination, stored)?;
                }
                pc = 306;
            }
            308 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = relocate("STACK_local_238");
                    memory.write_u64(destination, stored)?;
                }
                pc = 307;
            }
            309 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (values[0]).wrapping_add((0x8u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 308;
            }
            310 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = (values[0]).wrapping_add((0x7u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 309;
            }
            311 => {
                {
                    let destination = relocate("STACK_local_148");
                    let stored = (values[0]).wrapping_add((0xbu64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 310;
            }
            312 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored = (values[0]).wrapping_add((0xau64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 311;
            }
            313 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = relocate("STACK_local_58");
                    memory.write_u64(destination, stored)?;
                }
                pc = 312;
            }
            314 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = (values[0]).wrapping_add((0x6u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 313;
            }
            315 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = relocate("STACK_local_1f8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 314;
            }
            316 => {
                {
                    let destination = relocate("STACK_local_170");
                    let stored = (values[0]).wrapping_add((0x9u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 315;
            }
            317 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 316;
            }
            318 => {
                {
                    let destination = relocate("STACK_local_180");
                    let stored = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 317;
            }
            319 => {
                values[9] = memory.read_u64(relocate("_DAT_1401d0168"))?;
                pc = 318;
            }
            320 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [memory.read_u64(
                            memory.read_u64(
                                (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b80"))?)?)
                                    .wrapping_add((0x1u64).wrapping_mul(8)),
                            )?,
                        )?];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 319;
            }
            321 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b80"))?) != (0x0u64))) != 0
                {
                    320
                } else {
                    319
                };
            }
            322 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = memory.read_u64(relocate("DAT_1401d0b80"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 321;
            }
            323 => {
                {
                    let destination = relocate("STACK_local_238");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 322;
            }
            324 => {
                {
                    let destination = relocate("STACK_local_238");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 322;
            }
            325 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [
                            memory.read_u64(
                                memory.read_u64(memory.read_u64(relocate("STACK_local_60"))?)?,
                            )?,
                            memory.read_u64(relocate("STACK_local_60"))?,
                            relocate("DAT_14011c4e0"),
                            relocate("STACK_local_90"),
                        ];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 324;
            }
            326 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 325;
            }
            327 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) == (0x0u64))) != 0
                {
                    323
                } else {
                    326
                };
            }
            328 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_1f8"))?)?)
                                .wrapping_add(0x8u64),
                        )?];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 327;
            }
            329 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1f8"))?) != (0x0u64)))
                    != 0
                {
                    328
                } else {
                    327
                };
            }
            330 => {
                {
                    let destination = relocate("STACK_local_1f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x5u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 329;
            }
            331 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_258")];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 330;
            }
            332 => {
                pc = if (u64::from((values[20]) != (0x0u64))) != 0 {
                    331
                } else {
                    330
                };
            }
            333 => {
                {
                    let destination = (values[0]).wrapping_add((0xcu64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_200"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 332;
            }
            334 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_90")];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 333;
            }
            335 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            336 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    335
                } else {
                    334
                };
            }
            337 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_90"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_90"))?,
                        values[20],
                        relocate("STACK_local_200"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 336;
            }
            338 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 337;
            }
            339 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x140105630u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 338;
            }
            340 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x626u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 339;
            }
            341 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            342 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    341
                } else {
                    340
                };
            }
            343 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 342;
            }
            344 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x140105630u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 343;
            }
            345 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x624u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 344;
            }
            346 => {
                values[13] = (0x0u64) as i32 as i64 as u64;
                pc = 345;
            }
            347 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_78"))?,
                        relocate("DAT_140105598"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 345;
            }
            348 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) == (0x0u64))) != 0
                {
                    346
                } else {
                    347
                };
            }
            349 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 348;
            }
            350 => {
                {
                    let destination = relocate("STACK_local_200");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 349;
            }
            351 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 350;
            }
            352 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    351
                } else {
                    350
                };
            }
            353 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = values[20];
                    memory.write_u64(destination, stored)?;
                }
                pc = 352;
            }
            354 => {
                values[20] = memory.read_u64(relocate("STACK_local_250"))?;
                pc = 353;
            }
            355 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_250"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 354;
            }
            356 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 355;
            }
            357 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 356;
            }
            358 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 357;
            }
            359 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 358;
            }
            360 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 359;
            }
            361 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_250"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 360;
            }
            362 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_250"))?) != (0x0u64)))
                    != 0
                {
                    361
                } else {
                    353
                };
            }
            363 => {
                values[20] = values[25];
                pc = 362;
            }
            364 => {
                values[16] = memory.read_u64(relocate("STACK_local_88"))?;
                pc = 363;
            }
            365 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = {
                        let call_arguments = [0x18u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 364;
            }
            366 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = relocate("STACK_local_88");
                    memory.write_u64(destination, stored)?;
                }
                pc = 365;
            }
            367 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_88"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 366;
            }
            368 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    367
                } else {
                    366
                };
            }
            369 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0xbu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 368;
            }
            370 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            371 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    370
                } else {
                    369
                };
            }
            372 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[21])?)?).wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(values[21])?,
                        (((0x14011bdu64) & (0xffffffffffffffu64)).wrapping_shl((0x8u64) as u32))
                            | ((u64::from(
                                (((values[13]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )) & (0xffu64)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 371;
            }
            373 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 372;
            }
            374 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x14011bd00u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 373;
            }
            375 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x232u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 374;
            }
            376 => {
                values[13] = ({
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_14005d0a0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 375;
            }
            377 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 376;
            }
            378 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    377
                } else {
                    376
                };
            }
            379 => {
                {
                    let destination = values[21];
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 378;
            }
            380 => {
                {
                    let destination = values[23];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 379;
            }
            381 => {
                values[9] = memory.read_u64(values[23])?;
                pc = 380;
            }
            382 => {
                {
                    let call_arguments = [values[21]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 381;
            }
            383 => {
                pc = if (u64::from(((memory.read_u64(values[21])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    382
                } else {
                    381
                };
            }
            384 => {
                pc = if (u64::from((values[21]) != (values[23]))) != 0 {
                    383
                } else {
                    378
                };
            }
            385 => {
                values[21] = (values[0]).wrapping_add((0xbu64).wrapping_mul(8));
                pc = 384;
            }
            386 => {
                values[23] = {
                    let call_arguments =
                        [memory.read_u64((values[0]).wrapping_add((0xdu64).wrapping_mul(8)))?];
                    invoke(
                        "Paint::AI::PaintAIManager::AIFactory::GetServiceOptions",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 385;
            }
            387 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 386;
            }
            388 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    387
                } else {
                    386
                };
            }
            389 => {
                {
                    let destination = values[21];
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 388;
            }
            390 => {
                {
                    let destination = values[23];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 389;
            }
            391 => {
                values[9] = memory.read_u64(values[23])?;
                pc = 390;
            }
            392 => {
                {
                    let call_arguments = [values[21]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 391;
            }
            393 => {
                pc = if (u64::from(((memory.read_u64(values[21])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    392
                } else {
                    391
                };
            }
            394 => {
                pc = if (u64::from((values[21]) != (values[23]))) != 0 {
                    393
                } else {
                    388
                };
            }
            395 => {
                values[21] = (values[0]).wrapping_add((0xau64).wrapping_mul(8));
                pc = 394;
            }
            396 => {
                values[23] = {
                    let call_arguments =
                        [memory.read_u64((values[0]).wrapping_add((0xdu64).wrapping_mul(8)))?];
                    invoke(
                        "Paint::AI::PaintAIManager::AIFactory::GetPromptModerationService",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 395;
            }
            397 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 396;
            }
            398 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    397
                } else {
                    396
                };
            }
            399 => {
                {
                    let call_arguments = [relocate("STACK_local_258")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 398;
            }
            400 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    399
                } else {
                    398
                };
            }
            401 => {
                {
                    let call_arguments = [relocate("STACK_local_1f8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 400;
            }
            402 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1f8"))?) != (0x0u64)))
                    != 0
                {
                    401
                } else {
                    400
                };
            }
            403 => {
                {
                    let destination = relocate("STACK_local_200");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 402;
            }
            404 => {
                {
                    let call_arguments = [
                        values[22],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_200"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 403;
            }
            405 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 404;
            }
            406 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            407 => {
                pc = if (u64::from(
                    (((((values[13]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    406
                } else {
                    403
                };
            }
            408 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    405
                } else {
                    407
                };
            }
            409 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 408;
            }
            410 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 409;
            }
            411 => {
                values[13] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 410;
            }
            412 => {
                values[3] = (memory.read_u64(relocate("STACK_local_200"))?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                pc = 411;
            }
            413 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 412;
            }
            414 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_200"))?) != (0x0u64)))
                    != 0
                {
                    413
                } else {
                    402
                };
            }
            415 => {
                {
                    let destination = relocate("STACK_local_188");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 414;
            }
            416 => {
                {
                    let call_arguments = [
                        values[22],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_188"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 415;
            }
            417 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 416;
            }
            418 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            419 => {
                pc = if (u64::from(
                    (((((values[13]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    418
                } else {
                    415
                };
            }
            420 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    417
                } else {
                    419
                };
            }
            421 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 420;
            }
            422 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 421;
            }
            423 => {
                values[13] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 422;
            }
            424 => {
                values[2] = (memory.read_u64(relocate("STACK_local_188"))?).wrapping_add(0x18u64);
                pc = 423;
            }
            425 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 424;
            }
            426 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_188"))?) != (0x0u64)))
                    != 0
                {
                    425
                } else {
                    414
                };
            }
            427 => {
                {
                    let destination = relocate("STACK_local_190");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 426;
            }
            428 => {
                {
                    let call_arguments = [
                        values[22],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_190"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 427;
            }
            429 => {
                values[22] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 428;
            }
            430 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            431 => {
                pc = if (u64::from(
                    (((((values[13]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    430
                } else {
                    427
                };
            }
            432 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    429
                } else {
                    431
                };
            }
            433 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 432;
            }
            434 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 433;
            }
            435 => {
                values[13] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 434;
            }
            436 => {
                values[2] = (memory.read_u64(relocate("STACK_local_190"))?).wrapping_add(0x18u64);
                pc = 435;
            }
            437 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 436;
            }
            438 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_190"))?) != (0x0u64)))
                    != 0
                {
                    437
                } else {
                    426
                };
            }
            439 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[21])?).wrapping_add(0x8u64))?,
                        values[21],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 438;
            }
            440 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    439
                } else {
                    438
                };
            }
            441 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 440;
            }
            442 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 441;
            }
            443 => {
                values[13] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 442;
            }
            444 => {
                values[2] = (values[21]).wrapping_add(0xcu64);
                pc = 443;
            }
            445 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 444;
            }
            446 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(values[21])?)?, values[21]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 445;
            }
            447 => {
                pc = if (u64::from(
                    ((((values[9]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    446
                } else {
                    438
                };
            }
            448 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 447;
            }
            449 => {
                {
                    let destination = values[23];
                    let stored = ((((memory.read_u64(values[23])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 448;
            }
            450 => {
                values[9] = memory.read_u64(values[23])?;
                pc = 449;
            }
            451 => {
                values[23] = (values[21]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 450;
            }
            452 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 451;
            }
            453 => {
                pc = if (u64::from((values[21]) != (0x0u64))) != 0 {
                    452
                } else {
                    438
                };
            }
            454 => {
                {
                    let destination = (values[0]).wrapping_add((0xeu64).wrapping_mul(8));
                    let stored = values[20];
                    memory.write_u64(destination, stored)?;
                }
                pc = 453;
            }
            455 => {
                values[21] = memory.read_u64((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?;
                pc = 454;
            }
            456 => {
                {
                    let destination = (values[0]).wrapping_add((0xdu64).wrapping_mul(8));
                    let stored = (values[20]).wrapping_add((0x10u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 455;
            }
            457 => {
                values[25] = 0x0u64;
                pc = 456;
            }
            458 => {
                values[20] = 0x0u64;
                pc = 457;
            }
            459 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 457;
            }
            460 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_a0"))?)?)
                            .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 459;
            }
            461 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_a0"))?) != (0x0u64))) != 0
                {
                    460
                } else {
                    457
                };
            }
            462 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 461;
            }
            463 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_e0"))?)?)
                            .wrapping_add((0x4u64).wrapping_mul(8)),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 462;
            }
            464 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_e0"))?) != (0x0u64))) != 0
                {
                    463
                } else {
                    461
                };
            }
            465 => {
                {
                    let call_arguments = [
                        (values[20]).wrapping_add((0x10u64).wrapping_mul(1)),
                        relocate("STACK_local_118"),
                        memory.read_u64(relocate("STACK_local_1d0"))?,
                        memory.read_u64(relocate("STACK_local_1b0"))?,
                        memory.read_u64(relocate("STACK_local_1c0"))?,
                        (memory.read_u64(relocate("STACK_local_88"))?) as i32 as i64 as u64,
                        (u64::from(memory.read_u32(relocate("STACK_local_1e8"))?)) as i32 as i64
                            as u64,
                        (u64::from(memory.read_u32(relocate("STACK_local_1d8"))?)) as i32 as i64
                            as u64,
                        (u64::from(memory.read_u32(
                            (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4)),
                        )?)) as i32 as i64 as u64,
                        u64::from(
                            (u64::from(memory.read_u8(
                                (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(1)),
                            )?)) != 0,
                        ),
                        relocate("STACK_local_1f8"),
                        relocate("STACK_local_d8"),
                        relocate("STACK_local_48"),
                    ];
                    invoke(
                        "Paint::AI::PaintAIManager::AIFactory::AIFactory",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 464;
            }
            466 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = relocate("STACK_local_118");
                    memory.write_u64(destination, stored)?;
                }
                pc = 465;
            }
            467 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[28])?).wrapping_add(0x8u64))?,
                        values[28],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 466;
            }
            468 => {
                pc = if (u64::from((values[28]) != (0x0u64))) != 0 {
                    467
                } else {
                    466
                };
            }
            469 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = values[21];
                    memory.write_u64(destination, stored)?;
                }
                pc = 468;
            }
            470 => {
                {
                    let destination = relocate("STACK_local_110");
                    let stored = values[28];
                    memory.write_u64(destination, stored)?;
                }
                pc = 469;
            }
            471 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 470;
            }
            472 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 471;
            }
            473 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[25])?).wrapping_add(0x8u64))?,
                        values[25],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 472;
            }
            474 => {
                pc = if (u64::from((values[25]) != (0x0u64))) != 0 {
                    473
                } else {
                    472
                };
            }
            475 => {
                {
                    let destination = (values[21]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 474;
            }
            476 => {
                {
                    let destination = (values[21]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (relocate("STACK_pphStack_228")).wrapping_add(0x0u64),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 475;
            }
            477 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[16])?).wrapping_add(0x8u64))?,
                        values[16],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 476;
            }
            478 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    477
                } else {
                    476
                };
            }
            479 => {
                {
                    let destination = (values[21]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 478;
            }
            480 => {
                {
                    let destination = values[21];
                    let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 479;
            }
            481 => {
                values[21] = {
                    let call_arguments = [0x20u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 480;
            }
            482 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 481;
            }
            483 => {
                {
                    let destination = values[20];
                    let stored = relocate("std::_Ref_count_obj2<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 482;
            }
            484 => {
                {
                    let destination = (values[20]).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 483;
            }
            485 => {
                {
                    let destination = (values[20]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 484;
            }
            486 => {
                {
                    let destination = (values[20]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 485;
            }
            487 => {
                {
                    let destination = values[20];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 486;
            }
            488 => {
                pc = if (u64::from((values[20]) == (0x0u64))) != 0 {
                    458
                } else {
                    487
                };
            }
            489 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = values[20];
                    memory.write_u64(destination, stored)?;
                }
                pc = 488;
            }
            490 => {
                values[20] = {
                    let call_arguments = [0x188u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 489;
            }
            491 => {
                {
                    let destination = relocate("STACK_local_1d0");
                    let stored = {
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("FUN_14004baf0", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 490;
            }
            492 => {
                {
                    let destination = relocate("STACK_local_1b0");
                    let stored = {
                        let call_arguments =
                            [relocate("STACK_local_78"), relocate("STACK_local_188")];
                        invoke("FUN_14004bbf0", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 491;
            }
            493 => {
                {
                    let destination = relocate("STACK_local_1c0");
                    let stored = {
                        let call_arguments =
                            [relocate("STACK_local_78"), relocate("STACK_local_200")];
                        invoke("FUN_14004bcf0", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 492;
            }
            494 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_88")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[30]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 493;
            }
            495 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("FUN_14008c370", &call_arguments, memory)
                    }) as u32,
                );
                pc = 494;
            }
            496 => {
                {
                    let destination = relocate("STACK_local_1e8");
                    let stored = ({
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("FUN_14008c610", &call_arguments, memory)
                    }) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 495;
            }
            497 => {
                {
                    let destination = relocate("STACK_local_1d8");
                    let stored = ({
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("FUN_14008c530", &call_arguments, memory)
                    }) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 496;
            }
            498 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = ({
                        let call_arguments = [relocate("STACK_local_78")];
                        invoke("FUN_14008c450", &call_arguments, memory)
                    }) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 497;
            }
            499 => {
                {
                    let destination =
                        (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ({
                                let call_arguments = [relocate("STACK_local_78")];
                                invoke("FUN_14006c880", &call_arguments, memory)
                            }) != 0,
                        )) != 0,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 498;
            }
            500 => {
                {
                    let destination = relocate("STACK_local_1f8");
                    let stored = memory.read_u64(relocate("STACK_local_88"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 499;
            }
            501 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            502 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    501
                } else {
                    500
                };
            }
            503 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?)
                                .wrapping_add(0x40u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 502;
            }
            504 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 503;
            }
            505 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x140108490u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 504;
            }
            506 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_250")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x23du64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 505;
            }
            507 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 506;
            }
            508 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = memory.read_u64(relocate("STACK_local_98"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 507;
            }
            509 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_250")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            510 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    509
                } else {
                    508
                };
            }
            511 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[19])?)?).wrapping_add(0x50u64),
                        )?,
                        memory.read_u64(values[19])?,
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 510;
            }
            512 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 511;
            }
            513 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = 0x140108490u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 512;
            }
            514 => {
                {
                    let destination = (relocate("STACK_local_250")).wrapping_add(0x0u64);
                    let stored = u64::from((0x12fu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 513;
            }
            515 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 514;
            }
            516 => {
                values[19] = {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140018970", &call_arguments, memory)
                };
                pc = 515;
            }
            517 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 516;
            }
            518 => {
                {
                    let destination = relocate("STACK_local_238");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 517;
            }
            519 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = (((u64::from(
                        (u64::from(
                            memory
                                .read_u32((relocate("STACK_pphStack_228")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[26]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 518;
            }
            520 => {
                values[26] = ((u64::from(memory.read_u32(
                    (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 519;
            }
            521 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    520
                } else {
                    519
                };
            }
            522 => {
                values[26] = (0x1u64) as i32 as i64 as u64;
                pc = 521;
            }
            523 => {
                values[25] = memory.read_u64(relocate("STACK_local_238"))?;
                pc = 522;
            }
            524 => {
                values[12] = ({
                    let call_arguments = [relocate("STACK_local_68"), relocate("STACK_local_88")];
                    invoke("CoGetApartmentType", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 523;
            }
            525 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = memory.read_u64(relocate("STACK_local_98"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 524;
            }
            526 => {
                values[16] = memory.read_u64(relocate("STACK_local_98"))?;
                pc = 525;
            }
            527 => {
                {
                    let call_arguments = [relocate("DAT_140105fb0"), relocate("STACK_local_98")];
                    invoke("CoGetObjectContext", &call_arguments, memory)
                };
                pc = 526;
            }
            528 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 527;
            }
            529 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 528;
            }
            530 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 529;
            }
            531 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 530;
            }
            532 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            533 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    532
                } else {
                    531
                };
            }
            534 => {
                values[13] = ({
                    let call_arguments = [values[7], values[25], relocate("STACK_local_238")];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 533;
            }
            535 => {
                {
                    let call_arguments = [relocate("STACK_local_238")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 534;
            }
            536 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_238"))?) != (0x0u64)))
                    != 0
                {
                    535
                } else {
                    534
                };
            }
            537 => {
                values[7] = memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?)
                        .wrapping_add(0x18u64),
                )?;
                pc = 536;
            }
            538 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(relocate("STACK_local_98"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 537;
            }
            539 => {
                values[25] = memory.read_u64(relocate("STACK_local_98"))?;
                pc = 538;
            }
            540 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[19])?)?,
                        values[19],
                        relocate("DAT_140105490"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 539;
            }
            541 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 540;
            }
            542 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 541;
            }
            543 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105690u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 542;
            }
            544 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1160u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 543;
            }
            545 => {
                pc = if (u64::from((values[19]) != (0x0u64))) != 0 {
                    544
                } else {
                    530
                };
            }
            546 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = values[28];
                    memory.write_u64(destination, stored)?;
                }
                pc = 545;
            }
            547 => {
                values[19] = memory.read_u64((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?;
                pc = 546;
            }
            548 => {
                {
                    let destination = relocate("STACK_local_238");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 547;
            }
            549 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 548;
            }
            550 => {
                values[28] = memory.read_u64(relocate("STACK_local_88"))?;
                pc = 549;
            }
            551 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 550;
            }
            552 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            553 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    552
                } else {
                    551
                };
            }
            554 => {
                values[13] = ({
                    let call_arguments = [values[7], values[28], relocate("STACK_local_88")];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 553;
            }
            555 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 554;
            }
            556 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    555
                } else {
                    554
                };
            }
            557 => {
                values[7] = memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?)
                        .wrapping_add(0x18u64),
                )?;
                pc = 556;
            }
            558 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(relocate("STACK_local_98"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 557;
            }
            559 => {
                values[28] = memory.read_u64(relocate("STACK_local_98"))?;
                pc = 558;
            }
            560 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[19])?)?,
                        values[19],
                        relocate("DAT_140105490"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 559;
            }
            561 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 560;
            }
            562 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 561;
            }
            563 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105690u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 562;
            }
            564 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1160u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 563;
            }
            565 => {
                pc = if (u64::from((values[19]) != (0x0u64))) != 0 {
                    564
                } else {
                    549
                };
            }
            566 => {
                values[19] = memory.read_u64((values[0]).wrapping_add((0x9u64).wrapping_mul(8)))?;
                pc = 565;
            }
            567 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 566;
            }
            568 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 567;
            }
            569 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    568
                } else {
                    567
                };
            }
            570 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x9u64).wrapping_mul(8)),
                        values[18],
                    ];
                    invoke("FUN_14004a6e0", &call_arguments, memory)
                };
                pc = 569;
            }
            571 => {
                values[18] = {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_1400ad530", &call_arguments, memory)
                };
                pc = 570;
            }
            572 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) != 0,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    571
                } else {
                    567
                };
            }
            573 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 572;
            }
            574 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            575 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    574
                } else {
                    573
                };
            }
            576 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?)
                                .wrapping_add(0x40u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 575;
            }
            577 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 576;
            }
            578 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 577;
            }
            579 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x2481u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 578;
            }
            580 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            581 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    580
                } else {
                    579
                };
            }
            582 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 581;
            }
            583 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 582;
            }
            584 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x247fu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 583;
            }
            585 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_78"))?,
                        relocate("DAT_140105538"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 584;
            }
            586 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    585
                } else {
                    584
                };
            }
            587 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 586;
            }
            588 => {
                {
                    let destination =
                        (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 587;
            }
            589 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 588;
            }
            590 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    589
                } else {
                    588
                };
            }
            591 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x7u64).wrapping_mul(8)),
                        values[18],
                    ];
                    invoke("FUN_14004a6e0", &call_arguments, memory)
                };
                pc = 590;
            }
            592 => {
                values[18] = {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_1400a0590", &call_arguments, memory)
                };
                pc = 591;
            }
            593 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) != 0,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    592
                } else {
                    588
                };
            }
            594 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 593;
            }
            595 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            596 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    595
                } else {
                    594
                };
            }
            597 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?)
                                .wrapping_add(0x1a0u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 596;
            }
            598 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 597;
            }
            599 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 598;
            }
            600 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x277cu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 599;
            }
            601 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            602 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    601
                } else {
                    600
                };
            }
            603 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 602;
            }
            604 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 603;
            }
            605 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x277au64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 604;
            }
            606 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_78"))?,
                        relocate("DAT_140105538"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 605;
            }
            607 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    606
                } else {
                    605
                };
            }
            608 => {
                values[13] = (values[11]) as i32 as i64 as u64;
                pc = 607;
            }
            609 => {
                values[11] = (0x0u64) as i32 as i64 as u64;
                pc = 608;
            }
            610 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 609;
            }
            611 => {
                {
                    let destination =
                        (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 610;
            }
            612 => {
                {
                    let call_arguments = [
                        values[17],
                        relocate("STACK_local_48"),
                        relocate("DAT_1401d06b8"),
                    ];
                    invoke(
                        "Paint::AI::StableDiffusionHelpers::Initialize",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 611;
            }
            613 => {
                values[17] = {
                    let call_arguments = [];
                    invoke(
                        "Paint::AI::StableDiffusionHelpers::GetInstance",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 612;
            }
            614 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 613;
            }
            615 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_98"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_98"))?,
                        relocate("STACK_local_48"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 614;
            }
            616 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            617 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    616
                } else {
                    615
                };
            }
            618 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_60"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_60"))?,
                        relocate("DAT_140116ae0"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 617;
            }
            619 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 618;
            }
            620 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105690u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 619;
            }
            621 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x84bu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 620;
            }
            622 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    621
                } else {
                    615
                };
            }
            623 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 622;
            }
            624 => {
                {
                    let destination = relocate("STACK_plStack_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 623;
            }
            625 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 624;
            }
            626 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[21])?).wrapping_add(0x8u64))?,
                        values[21],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 625;
            }
            627 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    626
                } else {
                    625
                };
            }
            628 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 627;
            }
            629 => {
                {
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 628;
            }
            630 => {
                values[13] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 629;
            }
            631 => {
                values[2] = (values[21]).wrapping_add(0xcu64);
                pc = 630;
            }
            632 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 631;
            }
            633 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(values[21])?)?, values[21]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 632;
            }
            634 => {
                pc = if (u64::from(
                    ((((values[9]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    633
                } else {
                    625
                };
            }
            635 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 634;
            }
            636 => {
                {
                    let destination = values[23];
                    let stored = ((((memory.read_u64(values[23])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 635;
            }
            637 => {
                values[9] = memory.read_u64(values[23])?;
                pc = 636;
            }
            638 => {
                values[23] = (values[21]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 637;
            }
            639 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 638;
            }
            640 => {
                pc = if (u64::from((values[21]) != (0x0u64))) != 0 {
                    639
                } else {
                    625
                };
            }
            641 => {
                {
                    let call_arguments = [relocate("STACK_local_198")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 640;
            }
            642 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    641
                } else {
                    640
                };
            }
            643 => {
                {
                    let destination = values[4];
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 640;
            }
            644 => {
                {
                    let call_arguments = [values[4]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 643;
            }
            645 => {
                pc = if (u64::from((memory.read_u64(values[4])?) != (0x0u64))) != 0 {
                    644
                } else {
                    643
                };
            }
            646 => {
                pc = if (u64::from((values[4]) == (relocate("STACK_local_198")))) != 0 {
                    642
                } else {
                    645
                };
            }
            647 => {
                {
                    let destination = relocate("STACK_local_198");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 646;
            }
            648 => {
                values[4] = (values[0]).wrapping_add((0x8u64).wrapping_mul(8));
                pc = 647;
            }
            649 => {
                values[16] = values[28];
                pc = 648;
            }
            650 => {
                pc = if (u64::from((values[25]) == (0x0u64))) != 0 {
                    649
                } else {
                    648
                };
            }
            651 => {
                values[16] = (values[25]).wrapping_add((0x10u64).wrapping_mul(1));
                pc = 650;
            }
            652 => {
                values[25] = values[16];
                pc = 651;
            }
            653 => {
                values[21] = memory.read_u64(relocate("STACK_local_248"))?;
                pc = 652;
            }
            654 => {
                {
                    let destination = values[16];
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 653;
            }
            655 => {
                {
                    let call_arguments = [values[16], relocate("STACK_local_250")];
                    invoke("FUN_14000ac60", &call_arguments, memory)
                };
                pc = 654;
            }
            656 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    655
                } else {
                    651
                };
            }
            657 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 656;
            }
            658 => {
                values[25] = values[28];
                pc = 657;
            }
            659 => {
                values[16] = {
                    let call_arguments = [0x60u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 658;
            }
            660 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = memory.read_u64(relocate("DAT_1401d0ba8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 659;
            }
            661 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = memory.read_u64(relocate("DAT_1401d0ba0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 660;
            }
            662 => {
                values[21] = memory.read_u64(relocate("DAT_1401d0ba8"))?;
                pc = 661;
            }
            663 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 662;
            }
            664 => {
                {
                    let destination =
                        (memory.read_u64(relocate("DAT_1401d0ba8"))?).wrapping_add(0x8u64);
                    let stored = ((((u64::from(memory.read_u32(
                        (memory.read_u64(relocate("DAT_1401d0ba8"))?).wrapping_add(0x8u64),
                    )?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 663;
            }
            665 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 664;
            }
            666 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("DAT_1401d0ba8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    665
                } else {
                    662
                };
            }
            667 => {
                {
                    let call_arguments = [relocate("STACK_local_1a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 666;
            }
            668 => {
                pc = if (u64::from((values[25]) != (0x0u64))) != 0 {
                    667
                } else {
                    666
                };
            }
            669 => {
                {
                    let destination = values[4];
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 666;
            }
            670 => {
                {
                    let call_arguments = [values[4]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 669;
            }
            671 => {
                pc = if (u64::from((memory.read_u64(values[4])?) != (0x0u64))) != 0 {
                    670
                } else {
                    669
                };
            }
            672 => {
                pc = if (u64::from((values[4]) == (relocate("STACK_local_1a0")))) != 0 {
                    668
                } else {
                    671
                };
            }
            673 => {
                {
                    let destination = relocate("STACK_local_1a0");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 672;
            }
            674 => {
                values[4] = (values[0]).wrapping_add((0x6u64).wrapping_mul(8));
                pc = 673;
            }
            675 => {
                values[25] = values[28];
                pc = 674;
            }
            676 => {
                pc = 674;
            }
            677 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1f0"))?) != (0x0u64)))
                    != 0
                {
                    676
                } else {
                    675
                };
            }
            678 => {
                values[25] = (memory.read_u64(relocate("STACK_local_1f0"))?)
                    .wrapping_add((0x10u64).wrapping_mul(1));
                pc = 677;
            }
            679 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_1f0"))?;
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 678;
            }
            680 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_1f0"))?)
                        .wrapping_add((0x28u64).wrapping_mul(1));
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 679;
            }
            681 => {
                values[25] =
                    memory.read_u64((values[23]).wrapping_add((0x5u64).wrapping_mul(8)))?;
                pc = 680;
            }
            682 => {
                pc = if (u64::from((values[23]) != (memory.read_u64(relocate("DAT_1401d0720"))?)))
                    != 0
                {
                    681
                } else {
                    680
                };
            }
            683 => {
                values[25] = values[28];
                pc = 682;
            }
            684 => {
                values[23] = memory.read_u64(relocate("DAT_1401d0720"))?;
                pc = 683;
            }
            685 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8((values[23]).wrapping_add(0x19u64))?)) as i8
                            as i64 as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            (((0x2efu64) as i32 as i64 as u64) as i64)
                                < ((((memory.read_u64(
                                    (values[23]).wrapping_add((0x4u64).wrapping_mul(8)),
                                )?) as i32 as i64 as u64) as i32
                                    as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    684
                } else {
                    683
                };
            }
            686 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    693
                } else {
                    685
                };
            }
            687 => {
                values[5] = ((u64::from(memory.read_u8((values[21]).wrapping_add(0x19u64))?)) as i8
                    as i64 as u64) as i8 as i64 as u64;
                pc = 686;
            }
            688 => {
                values[23] = values[27];
                pc = 687;
            }
            689 => {
                values[21] = memory.read_u64(values[21])?;
                pc = 688;
            }
            690 => {
                values[27] = values[23];
                pc = 689;
            }
            691 => {
                values[21] = (values[21]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 690;
            }
            692 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[21]).wrapping_add((0x4u64).wrapping_mul(8)))?)
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x2efu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    691
                } else {
                    689
                };
            }
            693 => {
                values[27] = values[21];
                pc = 692;
            }
            694 => {
                values[23] = memory.read_u64(relocate("DAT_1401d0720"))?;
                pc = 686;
            }
            695 => {
                values[5] = ((u64::from(memory.read_u8((values[21]).wrapping_add(0x19u64))?)) as i8
                    as i64 as u64) as i8 as i64 as u64;
                pc = 694;
            }
            696 => {
                values[21] = memory.read_u64(
                    (memory.read_u64(relocate("DAT_1401d0720"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8)),
                )?;
                pc = 695;
            }
            697 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_1f0"))?)
                        .wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = relocate("DAT_1401d06a0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 696;
            }
            698 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_1f0"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 697;
            }
            699 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_1f0"))?;
                    let stored = relocate("Paint::GenerativeFillManager::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 698;
            }
            700 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_1f0"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 699;
            }
            701 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 700;
            }
            702 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 701;
            }
            703 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 702;
            }
            704 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_1f0"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = relocate("winrt::impl::produce<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 703;
            }
            705 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1f0"))?) != (0x0u64)))
                    != 0
                {
                    704
                } else {
                    675
                };
            }
            706 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = {
                        let call_arguments = [0x30u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 705;
            }
            707 => {
                {
                    let call_arguments = [
                        values[30],
                        relocate("STACK_local_58"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("FUN_1400ad5b0", &call_arguments, memory)
                };
                pc = 706;
            }
            708 => {
                {
                    let destination = relocate("_DAT_1401d0268");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 707;
            }
            709 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = relocate("FUN_1400ad810");
                    memory.write_u64(destination, stored)?;
                }
                pc = 708;
            }
            710 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 709;
            }
            711 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 710;
            }
            712 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 706;
            }
            713 => {
                {
                    let destination = relocate("_DAT_1401d0268");
                    let stored = (memory.read_u64(relocate("_DAT_1401d0268"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 712;
            }
            714 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 713;
            }
            715 => {
                {
                    let call_arguments = [
                        values[30],
                        relocate("STACK_local_58"),
                        relocate("DAT_1401d0260"),
                    ];
                    invoke("FUN_1400ad730", &call_arguments, memory)
                };
                pc = 714;
            }
            716 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0260"))?) == (0x0u64))) != 0
                {
                    711
                } else {
                    715
                };
            }
            717 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 716;
            }
            718 => {
                {
                    let destination = relocate("_DAT_1401d0268");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d0268"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 717;
            }
            719 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 718;
            }
            720 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = relocate("DAT_1401d0268");
                    memory.write_u64(destination, stored)?;
                }
                pc = 719;
            }
            721 => {
                values[9] = memory.read_u64(relocate("_DAT_1401d0268"))?;
                pc = 720;
            }
            722 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d06f8"))?)?)
                                .wrapping_add((0x1u64).wrapping_mul(8)),
                        )?];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    }) as u32,
                );
                pc = 721;
            }
            723 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06f8"))?) != (0x0u64))) != 0
                {
                    722
                } else {
                    721
                };
            }
            724 => {
                {
                    let destination = relocate("DAT_1401d06f8");
                    let stored = memory.read_u64(values[4])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 723;
            }
            725 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 724;
            }
            726 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d06f8"))?) != (0x0u64))) != 0
                {
                    725
                } else {
                    724
                };
            }
            727 => {
                pc = if (u64::from((values[4]) != (relocate("DAT_1401d06f8")))) != 0 {
                    726
                } else {
                    721
                };
            }
            728 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_88")];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 727;
            }
            729 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    728
                } else {
                    727
                };
            }
            730 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_1e0")];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 729;
            }
            731 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_1e0"))?) != (0x0u64)))
                    != 0
                {
                    730
                } else {
                    729
                };
            }
            732 => {
                {
                    let destination = values[4];
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 729;
            }
            733 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [values[4]];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 732;
            }
            734 => {
                pc = if (u64::from((memory.read_u64(values[4])?) != (0x0u64))) != 0 {
                    733
                } else {
                    732
                };
            }
            735 => {
                pc = if (u64::from((values[4]) == (relocate("STACK_local_1e0")))) != 0 {
                    731
                } else {
                    734
                };
            }
            736 => {
                values[4] = (values[0]).wrapping_add((0x5u64).wrapping_mul(8));
                pc = 735;
            }
            737 => {
                values[25] = memory.read_u64(relocate("STACK_local_1e0"))?;
                pc = 736;
            }
            738 => {
                values[30] = u64::from(
                    ({
                        let call_arguments = [
                            0x0u64,
                            relocate("STACK_local_1e0"),
                            relocate("STACK_local_230"),
                        ];
                        invoke("FUN_1400ad830", &call_arguments, memory)
                    }) as u32,
                );
                pc = 737;
            }
            739 => {
                {
                    let destination = relocate("_DAT_1401d0188");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 738;
            }
            740 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 739;
            }
            741 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 740;
            }
            742 => {
                values[30] = u64::from(
                    (u64::from((relocate("CONTEXT_IGNORED_FUN_14009e900_extraout_XMM0_Da")) as u32))
                        as u32,
                );
                pc = 737;
            }
            743 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 742;
            }
            744 => {
                {
                    let destination = relocate("_DAT_1401d0188");
                    let stored = (memory.read_u64(relocate("_DAT_1401d0188"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 743;
            }
            745 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 744;
            }
            746 => {
                {
                    let destination = relocate("STACK_local_1e0");
                    let stored = memory.read_u64(relocate("STACK_local_98"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 745;
            }
            747 => {
                {
                    let call_arguments = [values[13], relocate("STACK_local_230")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            748 => {
                pc = if (u64::from(
                    (((values[13]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    747
                } else {
                    746
                };
            }
            749 => {
                values[13] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d0180"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d0180"))?,
                        memory.read_u64(relocate("STACK_local_78"))?,
                        memory.read_u64(relocate("STACK_local_50"))?,
                        memory.read_u64(relocate("STACK_local_88"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 748;
            }
            750 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 749;
            }
            751 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 750;
            }
            752 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_230")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1064u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 751;
            }
            753 => {
                {
                    let destination = (relocate("STACK_local_230")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_230"))?)
                                .wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 752;
            }
            754 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 753;
            }
            755 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0180"))?) == (0x0u64))) != 0
                {
                    741
                } else {
                    754
                };
            }
            756 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 755;
            }
            757 => {
                {
                    let destination = relocate("_DAT_1401d0188");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d0188"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 756;
            }
            758 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 757;
            }
            759 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = relocate("DAT_1401d0188");
                    memory.write_u64(destination, stored)?;
                }
                pc = 758;
            }
            760 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored = relocate("STACK_local_88");
                    memory.write_u64(destination, stored)?;
                }
                pc = 759;
            }
            761 => {
                {
                    let destination = relocate("STACK_pphStack_228");
                    let stored = relocate("STACK_local_50");
                    memory.write_u64(destination, stored)?;
                }
                pc = 760;
            }
            762 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = relocate("STACK_local_78");
                    memory.write_u64(destination, stored)?;
                }
                pc = 761;
            }
            763 => {
                values[9] = memory.read_u64(relocate("_DAT_1401d0188"))?;
                pc = 762;
            }
            764 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 763;
            }
            765 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = memory.read_u64(relocate("STACK_local_98"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 763;
            }
            766 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_60"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_60"))?,
                        relocate("DAT_14011c4e0"),
                        relocate("STACK_local_98"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 765;
            }
            767 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 766;
            }
            768 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) == (0x0u64))) != 0
                {
                    764
                } else {
                    767
                };
            }
            769 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_50"))?,
                        relocate("STACK_local_60"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("FUN_1400ada60", &call_arguments, memory)
                };
                pc = 768;
            }
            770 => {
                {
                    let destination = relocate("_DAT_1401d0308");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 769;
            }
            771 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = relocate("FUN_1400adcc0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 770;
            }
            772 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 771;
            }
            773 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 772;
            }
            774 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 768;
            }
            775 => {
                {
                    let destination = relocate("_DAT_1401d0308");
                    let stored = (memory.read_u64(relocate("_DAT_1401d0308"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 774;
            }
            776 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 775;
            }
            777 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_50"))?,
                        relocate("STACK_local_60"),
                        relocate("DAT_1401d0300"),
                    ];
                    invoke("FUN_1400adbe0", &call_arguments, memory)
                };
                pc = 776;
            }
            778 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0300"))?) == (0x0u64))) != 0
                {
                    773
                } else {
                    777
                };
            }
            779 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 778;
            }
            780 => {
                {
                    let destination = relocate("_DAT_1401d0308");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d0308"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 779;
            }
            781 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 780;
            }
            782 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = relocate("DAT_1401d0308");
                    memory.write_u64(destination, stored)?;
                }
                pc = 781;
            }
            783 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = values[28];
                    memory.write_u64(destination, stored)?;
                }
                pc = 782;
            }
            784 => {
                pc = if (u64::from(((values[15]) as i64) == ((0x0u64) as i64))) != 0 {
                    783
                } else {
                    782
                };
            }
            785 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = (values[15]).wrapping_add(0x18u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 784;
            }
            786 => {
                values[9] = memory.read_u64(relocate("_DAT_1401d0308"))?;
                pc = 785;
            }
            787 => {
                values[15] = {
                    let call_arguments = [
                        relocate("DAT_1401d06a0"),
                        (values[0]).wrapping_add((0x1u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_1400a47a0", &call_arguments, memory)
                };
                pc = 786;
            }
            788 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b78"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 787;
            }
            789 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b78"))?) != (0x0u64))) != 0
                {
                    788
                } else {
                    787
                };
            }
            790 => {
                {
                    let destination = relocate("STACK_local_218");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 789;
            }
            791 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = memory.read_u64(relocate("DAT_1401d0b78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 790;
            }
            792 => {
                {
                    let destination = (values[0]).wrapping_add(0x14u64);
                    let stored = ((values[8]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 791;
            }
            793 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (relocate("STACK_pcStack_210")).wrapping_add(0x0u64),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 792;
            }
            794 => {
                {
                    let destination = (values[0]).wrapping_add(0xcu64);
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_218")).wrapping_add(0x4u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 793;
            }
            795 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from(
                        (u64::from((memory.read_u64(relocate("STACK_local_218"))?) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 794;
            }
            796 => {
                {
                    let destination = (relocate("STACK_local_218")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(((values[6]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 795;
            }
            797 => {
                {
                    let destination = (relocate("STACK_local_218")).wrapping_add(0x0u64);
                    let stored = u64::from(
                        (u64::from(
                            ((values[6]).wrapping_shr(
                                (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 796;
            }
            798 => {
                {
                    let destination = relocate("STACK_pcStack_210");
                    let stored =
                        (memory.read_u64(relocate("STACK_pcStack_210"))?) & (0xffffffffffffff00u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 797;
            }
            799 => {
                values[8] = (memory.read_u64(relocate("STACK_pcStack_210"))?)
                    .wrapping_shr((0x20u64) as u32);
                pc = 798;
            }
            800 => {
                values[6] = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
                pc = 799;
            }
            801 => {
                {
                    let destination = values[0];
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 800;
            }
            802 => {
                values[28] = 0x0u64;
                pc = 801;
            }
            803 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_2c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 802;
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
    ("FUN_140023b70", 3, fun_140023b70),
    ("FUN_1400275a0", 5, fun_1400275a0),
    ("FUN_140045890", 6, fun_140045890),
    ("FUN_14005eb50", 3, fun_14005eb50),
    ("FUN_14006aaf0", 0, fun_14006aaf0),
    ("FUN_14007e010", 1, fun_14007e010),
    ("FUN_14008b2c0", 4, fun_14008b2c0),
    ("FUN_140098890", 4, fun_140098890),
    ("FUN_14009e430", 2, fun_14009e430),
    ("FUN_14009e900", 2, fun_14009e900),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_140023b70",
        &[
            ("STACK_auStack_e8", -232),
            ("STACK_local_c8", -200),
            ("STACK_local_c0", -192),
            ("STACK_local_b8", -184),
            ("STACK_local_a8", -168),
            ("STACK_ppuStack_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_74", -116),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_plStack_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
        ],
    ),
    (
        "FUN_1400275a0",
        &[
            ("STACK_auStackY_728", -1832),
            ("STACK_local_6e8", -1768),
            ("STACK_uStack_6e4", -1764),
            ("STACK_uStack_6e0", -1760),
            ("STACK_local_6d8", -1752),
            ("STACK_local_6d0", -1744),
            ("STACK_uStack_6cc", -1740),
            ("STACK_uStack_6c8", -1736),
            ("STACK_uStack_6c4", -1732),
            ("STACK_local_6c0", -1728),
            ("STACK_uStack_6b8", -1720),
            ("STACK_local_6a8", -1704),
            ("STACK_local_6a0", -1696),
            ("STACK_local_698", -1688),
            ("STACK_local_690", -1680),
            ("STACK_local_688", -1672),
            ("STACK_local_680", -1664),
            ("STACK_local_678", -1656),
            ("STACK_local_670", -1648),
            ("STACK_local_668", -1640),
            ("STACK_local_660", -1632),
            ("STACK_local_658", -1624),
            ("STACK_local_650", -1616),
            ("STACK_local_648", -1608),
            ("STACK_local_640", -1600),
            ("STACK_local_638", -1592),
            ("STACK_local_628", -1576),
            ("STACK_local_620", -1568),
            ("STACK_local_618", -1560),
            ("STACK_uStack_610", -1552),
            ("STACK_uStack_60c", -1548),
            ("STACK_local_608", -1544),
            ("STACK_local_600", -1536),
            ("STACK_local_5f8", -1528),
            ("STACK_local_5f7", -1527),
            ("STACK_local_5f6", -1526),
            ("STACK_local_5f5", -1525),
            ("STACK_local_5f4", -1524),
            ("STACK_local_5f0", -1520),
            ("STACK_local_5ec", -1516),
            ("STACK_local_5e8", -1512),
            ("STACK_uStack_5e0", -1504),
            ("STACK_local_5d8", -1496),
            ("STACK_uStack_5d0", -1488),
            ("STACK_local_5c8", -1480),
            ("STACK_acStack_5c7", -1479),
            ("STACK_local_5f", -95),
            ("STACK_local_58", -88),
            ("STACK_PARAM_param_5", 40),
        ],
    ),
    (
        "FUN_140045890",
        &[
            ("STACK_auStackY_e8", -232),
            ("STACK_local_98", -152),
            ("STACK_uStack_90", -144),
            ("STACK_local_88", -136),
            ("STACK_uStack_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_68", -104),
            ("STACK_LStack_60", -96),
            ("STACK_LStack_5c", -92),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
        ],
    ),
    (
        "FUN_14005eb50",
        &[
            ("STACK_auStack_1c8", -456),
            ("STACK_local_198", -408),
            ("STACK_local_190", -400),
            ("STACK_local_188", -392),
            ("STACK_local_180", -384),
            ("STACK_local_178", -376),
            ("STACK_local_170", -368),
            ("STACK_local_168", -360),
            ("STACK_local_160", -352),
            ("STACK_uStack_158", -344),
            ("STACK_local_150", -336),
            ("STACK_local_148", -328),
            ("STACK_local_140", -320),
            ("STACK_local_138", -312),
            ("STACK_local_130", -304),
            ("STACK_local_12c", -300),
            ("STACK_local_120", -288),
            ("STACK_local_118", -280),
            ("STACK_local_110", -272),
            ("STACK_local_10c", -268),
            ("STACK_local_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
            ("STACK_local_ec", -236),
            ("STACK_local_e0", -224),
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
            ("STACK_uStack_84", -132),
            ("STACK_uStack_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_48", -72),
        ],
    ),
    (
        "FUN_14006aaf0",
        &[
            ("STACK_auStackY_a8", -168),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_4c", -76),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14007e010",
        &[
            ("STACK_auStackY_328", -808),
            ("STACK_local_2d8", -728),
            ("STACK_local_2d0", -720),
            ("STACK_local_2c8", -712),
            ("STACK_local_2c0", -704),
            ("STACK_plStack_2b8", -696),
            ("STACK_local_2b0", -688),
            ("STACK_local_2a8", -680),
            ("STACK_local_2a4", -676),
            ("STACK_local_298", -664),
            ("STACK_local_290", -656),
            ("STACK_local_288", -648),
            ("STACK_local_280", -640),
            ("STACK_local_278", -632),
            ("STACK_local_270", -624),
            ("STACK_local_268", -616),
            ("STACK_uStack_264", -612),
            ("STACK_local_260", -608),
            ("STACK_local_258", -600),
            ("STACK_local_248", -584),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_14008b2c0",
        &[
            ("STACK_auStack_1b8", -440),
            ("STACK_local_198", -408),
            ("STACK_local_190", -400),
            ("STACK_local_188", -392),
            ("STACK_local_180", -384),
            ("STACK_local_178", -376),
            ("STACK_uStack_170", -368),
            ("STACK_local_168", -360),
            ("STACK_local_160", -352),
            ("STACK_local_158", -344),
            ("STACK_uStack_150", -336),
            ("STACK_local_148", -328),
            ("STACK_local_140", -320),
            ("STACK_local_138", -312),
            ("STACK_local_128", -296),
            ("STACK_local_120", -288),
            ("STACK_local_118", -280),
            ("STACK_local_108", -264),
            ("STACK_local_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_uStack_f0", -240),
            ("STACK_local_e8", -232),
            ("STACK_uStack_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_auStack_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_c0", -192),
            ("STACK_local_b8", -184),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
        ],
    ),
    (
        "FUN_140098890",
        &[
            ("STACK_auStack_1a8", -424),
            ("STACK_local_188", -392),
            ("STACK_local_180", -384),
            ("STACK_local_178", -376),
            ("STACK_local_170", -368),
            ("STACK_local_168", -360),
            ("STACK_local_160", -352),
            ("STACK_local_158", -344),
            ("STACK_local_150", -336),
            ("STACK_local_148", -328),
            ("STACK_local_140", -320),
            ("STACK_local_138", -312),
            ("STACK_plStack_130", -304),
            ("STACK_local_128", -296),
            ("STACK_local_120", -288),
            ("STACK_local_118", -280),
            ("STACK_local_110", -272),
            ("STACK_local_108", -264),
            ("STACK_local_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_local_e8", -232),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_c4", -196),
            ("STACK_local_c0", -192),
            ("STACK_local_bc", -188),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_a0", -160),
            ("STACK_uStack_98", -152),
            ("STACK_local_90", -144),
            ("STACK_uStack_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
        ],
    ),
    (
        "FUN_14009e430",
        &[
            ("STACK_in_stack_ffffffffffffff9c", -100),
            ("STACK_auStackY_98", -152),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_pcStack_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14009e900",
        &[
            ("STACK_auStackY_2c8", -712),
            ("STACK_local_258", -600),
            ("STACK_local_250", -592),
            ("STACK_local_248", -584),
            ("STACK_local_240", -576),
            ("STACK_local_238", -568),
            ("STACK_local_230", -560),
            ("STACK_pphStack_228", -552),
            ("STACK_local_220", -544),
            ("STACK_local_218", -536),
            ("STACK_pcStack_210", -528),
            ("STACK_local_208", -520),
            ("STACK_local_200", -512),
            ("STACK_local_1f8", -504),
            ("STACK_local_1f0", -496),
            ("STACK_local_1e8", -488),
            ("STACK_uStack_1e4", -484),
            ("STACK_local_1e0", -480),
            ("STACK_local_1d8", -472),
            ("STACK_local_1d0", -464),
            ("STACK_local_1c8", -456),
            ("STACK_local_1c0", -448),
            ("STACK_local_1b8", -440),
            ("STACK_local_1b0", -432),
            ("STACK_local_1a8", -424),
            ("STACK_local_1a0", -416),
            ("STACK_local_198", -408),
            ("STACK_local_190", -400),
            ("STACK_local_188", -392),
            ("STACK_local_180", -384),
            ("STACK_local_178", -376),
            ("STACK_local_170", -368),
            ("STACK_local_168", -360),
            ("STACK_local_160", -352),
            ("STACK_local_158", -344),
            ("STACK_local_150", -336),
            ("STACK_local_148", -328),
            ("STACK_local_140", -320),
            ("STACK_local_138", -312),
            ("STACK_local_130", -304),
            ("STACK_local_128", -296),
            ("STACK_local_118", -280),
            ("STACK_local_110", -272),
            ("STACK_local_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_plStack_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_140023b70",
        &[
            ("STACK_auStack_e8", 32),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 8),
            ("STACK_local_a8", 8),
            ("STACK_ppuStack_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 4),
            ("STACK_local_74", 4),
            ("STACK_local_68", 8),
            ("STACK_local_60", 4),
            ("STACK_local_58", 8),
            ("STACK_plStack_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
        ],
    ),
    (
        "FUN_1400275a0",
        &[
            ("STACK_auStackY_728", 32),
            ("STACK_local_6e8", 4),
            ("STACK_uStack_6e4", 4),
            ("STACK_uStack_6e0", 8),
            ("STACK_local_6d8", 8),
            ("STACK_local_6d0", 4),
            ("STACK_uStack_6cc", 4),
            ("STACK_uStack_6c8", 4),
            ("STACK_uStack_6c4", 4),
            ("STACK_local_6c0", 8),
            ("STACK_uStack_6b8", 8),
            ("STACK_local_6a8", 8),
            ("STACK_local_6a0", 8),
            ("STACK_local_698", 8),
            ("STACK_local_690", 8),
            ("STACK_local_688", 8),
            ("STACK_local_680", 8),
            ("STACK_local_678", 8),
            ("STACK_local_670", 8),
            ("STACK_local_668", 8),
            ("STACK_local_660", 8),
            ("STACK_local_658", 8),
            ("STACK_local_650", 8),
            ("STACK_local_648", 8),
            ("STACK_local_640", 8),
            ("STACK_local_638", 8),
            ("STACK_local_628", 8),
            ("STACK_local_620", 8),
            ("STACK_local_618", 8),
            ("STACK_uStack_610", 4),
            ("STACK_uStack_60c", 4),
            ("STACK_local_608", 8),
            ("STACK_local_600", 8),
            ("STACK_local_5f8", 1),
            ("STACK_local_5f7", 1),
            ("STACK_local_5f6", 1),
            ("STACK_local_5f5", 1),
            ("STACK_local_5f4", 4),
            ("STACK_local_5f0", 4),
            ("STACK_local_5ec", 4),
            ("STACK_local_5e8", 8),
            ("STACK_uStack_5e0", 8),
            ("STACK_local_5d8", 8),
            ("STACK_uStack_5d0", 8),
            ("STACK_local_5c8", 1),
            ("STACK_acStack_5c7", 1384),
            ("STACK_local_5f", 7),
            ("STACK_local_58", 8),
            ("STACK_PARAM_param_5", 8),
        ],
    ),
    (
        "FUN_140045890",
        &[
            ("STACK_auStackY_e8", 32),
            ("STACK_local_98", 8),
            ("STACK_uStack_90", 8),
            ("STACK_local_88", 8),
            ("STACK_uStack_80", 8),
            ("STACK_local_78", 16),
            ("STACK_local_68", 8),
            ("STACK_LStack_60", 4),
            ("STACK_LStack_5c", 4),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
        ],
    ),
    (
        "FUN_14005eb50",
        &[
            ("STACK_auStack_1c8", 32),
            ("STACK_local_198", 8),
            ("STACK_local_190", 8),
            ("STACK_local_188", 8),
            ("STACK_local_180", 8),
            ("STACK_local_178", 8),
            ("STACK_local_170", 8),
            ("STACK_local_168", 4),
            ("STACK_local_160", 8),
            ("STACK_uStack_158", 8),
            ("STACK_local_150", 8),
            ("STACK_local_148", 8),
            ("STACK_local_140", 8),
            ("STACK_local_138", 8),
            ("STACK_local_130", 4),
            ("STACK_local_12c", 4),
            ("STACK_local_120", 8),
            ("STACK_local_118", 8),
            ("STACK_local_110", 4),
            ("STACK_local_10c", 4),
            ("STACK_local_100", 8),
            ("STACK_local_f8", 8),
            ("STACK_local_f0", 4),
            ("STACK_local_ec", 4),
            ("STACK_local_e0", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 4),
            ("STACK_uStack_84", 4),
            ("STACK_uStack_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 16),
            ("STACK_local_60", 8),
            ("STACK_local_58", 16),
            ("STACK_local_48", 8),
        ],
    ),
    (
        "FUN_14006aaf0",
        &[
            ("STACK_auStackY_a8", 32),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 4),
            ("STACK_local_4c", 4),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14007e010",
        &[
            ("STACK_auStackY_328", 32),
            ("STACK_local_2d8", 8),
            ("STACK_local_2d0", 8),
            ("STACK_local_2c8", 8),
            ("STACK_local_2c0", 8),
            ("STACK_plStack_2b8", 8),
            ("STACK_local_2b0", 8),
            ("STACK_local_2a8", 4),
            ("STACK_local_2a4", 4),
            ("STACK_local_298", 8),
            ("STACK_local_290", 8),
            ("STACK_local_288", 8),
            ("STACK_local_280", 8),
            ("STACK_local_278", 8),
            ("STACK_local_270", 8),
            ("STACK_local_268", 4),
            ("STACK_uStack_264", 4),
            ("STACK_local_260", 8),
            ("STACK_local_258", 8),
            ("STACK_local_248", 528),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_14008b2c0",
        &[
            ("STACK_auStack_1b8", 32),
            ("STACK_local_198", 1),
            ("STACK_local_190", 8),
            ("STACK_local_188", 8),
            ("STACK_local_180", 8),
            ("STACK_local_178", 8),
            ("STACK_uStack_170", 8),
            ("STACK_local_168", 8),
            ("STACK_local_160", 8),
            ("STACK_local_158", 8),
            ("STACK_uStack_150", 8),
            ("STACK_local_148", 8),
            ("STACK_local_140", 8),
            ("STACK_local_138", 16),
            ("STACK_local_128", 8),
            ("STACK_local_120", 8),
            ("STACK_local_118", 16),
            ("STACK_local_108", 8),
            ("STACK_local_100", 8),
            ("STACK_local_f8", 8),
            ("STACK_uStack_f0", 8),
            ("STACK_local_e8", 8),
            ("STACK_uStack_e0", 8),
            ("STACK_local_d8", 8),
            ("STACK_auStack_d0", 8),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 16),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 16),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 16),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
        ],
    ),
    (
        "FUN_140098890",
        &[
            ("STACK_auStack_1a8", 32),
            ("STACK_local_188", 4),
            ("STACK_local_180", 4),
            ("STACK_local_178", 4),
            ("STACK_local_170", 4),
            ("STACK_local_168", 4),
            ("STACK_local_160", 4),
            ("STACK_local_158", 4),
            ("STACK_local_150", 8),
            ("STACK_local_148", 8),
            ("STACK_local_140", 8),
            ("STACK_local_138", 8),
            ("STACK_plStack_130", 8),
            ("STACK_local_128", 8),
            ("STACK_local_120", 8),
            ("STACK_local_118", 4),
            ("STACK_local_110", 8),
            ("STACK_local_108", 8),
            ("STACK_local_100", 8),
            ("STACK_local_f8", 16),
            ("STACK_local_e8", 8),
            ("STACK_local_e0", 4),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 4),
            ("STACK_local_c4", 4),
            ("STACK_local_c0", 4),
            ("STACK_local_bc", 4),
            ("STACK_local_b8", 4),
            ("STACK_local_b0", 16),
            ("STACK_local_a0", 8),
            ("STACK_uStack_98", 8),
            ("STACK_local_90", 8),
            ("STACK_uStack_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
        ],
    ),
    (
        "FUN_14009e430",
        &[
            ("STACK_in_stack_ffffffffffffff9c", 4),
            ("STACK_auStackY_98", 32),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_pcStack_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14009e900",
        &[
            ("STACK_auStackY_2c8", 32),
            ("STACK_local_258", 8),
            ("STACK_local_250", 8),
            ("STACK_local_248", 8),
            ("STACK_local_240", 8),
            ("STACK_local_238", 8),
            ("STACK_local_230", 8),
            ("STACK_pphStack_228", 8),
            ("STACK_local_220", 8),
            ("STACK_local_218", 8),
            ("STACK_pcStack_210", 8),
            ("STACK_local_208", 8),
            ("STACK_local_200", 8),
            ("STACK_local_1f8", 8),
            ("STACK_local_1f0", 8),
            ("STACK_local_1e8", 4),
            ("STACK_uStack_1e4", 4),
            ("STACK_local_1e0", 8),
            ("STACK_local_1d8", 4),
            ("STACK_local_1d0", 8),
            ("STACK_local_1c8", 8),
            ("STACK_local_1c0", 8),
            ("STACK_local_1b8", 8),
            ("STACK_local_1b0", 8),
            ("STACK_local_1a8", 8),
            ("STACK_local_1a0", 8),
            ("STACK_local_198", 8),
            ("STACK_local_190", 8),
            ("STACK_local_188", 8),
            ("STACK_local_180", 8),
            ("STACK_local_178", 8),
            ("STACK_local_170", 8),
            ("STACK_local_168", 8),
            ("STACK_local_160", 8),
            ("STACK_local_158", 8),
            ("STACK_local_150", 8),
            ("STACK_local_148", 8),
            ("STACK_local_140", 8),
            ("STACK_local_138", 8),
            ("STACK_local_130", 8),
            ("STACK_local_128", 8),
            ("STACK_local_118", 8),
            ("STACK_local_110", 8),
            ("STACK_local_e0", 8),
            ("STACK_local_d8", 56),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_plStack_40", 8),
            ("STACK_local_38", 8),
        ],
    ),
];
