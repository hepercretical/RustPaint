//! Checked translations generated from original bodies by batch 021.
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
/// Translation of `FUN_140007750` (`mspaint.c:11362`).
pub fn fun_140007750(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64];
    let mut pc: i32 = 34;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[6]));
            }
            1 => {
                values[6] = values[3];
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("STACK_local_28")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from(((values[1]) as i64) != ((0x0u64) as i64))) != 0 {
                    2
                } else {
                    1
                };
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from((values[2]) != (0x0u64))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_20")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64))) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                values[3] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_20")];
                        invoke("FUN_1400078c0", &call_arguments, memory)
                    }) as u8,
                );
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_48")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            10 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    8
                };
            }
            11 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x160u64))?,
                        values[2],
                        relocate("STACK_local_20"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x100eu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = memory.read_u64(relocate("STACK_local_20"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_48")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            18 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    16
                };
            }
            19 => {
                values[2] = memory.read_u64(relocate("STACK_local_20"))?;
                pc = 18;
            }
            20 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0xa8u64))?,
                        values[1],
                        relocate("STACK_local_20"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
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
                pc = 24;
            }
            26 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    25
                } else {
                    24
                };
            }
            27 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                values[6] = ((((values[5]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                    | ((values[4]) & (0xffffffffu64)))
                    & (0xffffffffffffff00u64);
                pc = 0;
            }
            29 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x25bu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    28
                };
            }
            30 => {
                values[1] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 29;
            }
            31 => {
                values[5] = u64::from(
                    (u64::from(((values[0]).wrapping_shr(0x20u64 as u32)) as u32)) as u32,
                );
                pc = 30;
            }
            32 => {
                values[4] = ((values[0]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 31;
            }
            33 => {
                values[0] = {
                    let call_arguments = [memory.read_u64(relocate("DAT_1401d06e0"))?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140007940` (`mspaint.c:11460`).
pub fn fun_140007940(
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
        (arguments[0]) as i32 as i64 as u64,
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
                return Ok(Some(values[6]));
            }
            1 => {
                values[6] = values[7];
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    2
                } else {
                    1
                };
            }
            4 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            5 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 5;
            }
            7 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x60u64))?,
                        values[3],
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x6e9bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination =
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            13 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    12
                } else {
                    3
                };
            }
            14 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 13;
            }
            15 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x58u64))?,
                        values[3],
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x6e89u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination =
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            21 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    3
                };
            }
            22 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 21;
            }
            23 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x68u64))?,
                        values[3],
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x6eadu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination =
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            29 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    3
                };
            }
            30 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 29;
            }
            31 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x78u64))?,
                        values[3],
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x6ed1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination =
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            37 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    36
                } else {
                    3
                };
            }
            38 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 37;
            }
            39 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x70u64))?,
                        values[3],
                        relocate("STACK_local_28"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x6ebfu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination =
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 42;
            }
            44 => {
                values[7] = u64::from((0x0u64) as u8);
                pc = 3;
            }
            45 => {
                pc = if (u64::from(
                    (((values[0]) as i32 as i64 as u64) as i64)
                        == (((0xe12cu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    43
                } else {
                    44
                };
            }
            46 => {
                pc = if (u64::from(
                    (((values[0]) as i32 as i64 as u64) as i64)
                        == (((0xe12bu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    45
                };
            }
            47 => {
                pc = if (u64::from(
                    (((values[0]) as i32 as i64 as u64) as i64)
                        == (((0xe125u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    46
                };
            }
            48 => {
                pc = if (u64::from(
                    (((values[0]) as i32 as i64 as u64) as i64)
                        == (((0xe123u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    47
                };
            }
            49 => {
                pc = if (u64::from(
                    (((values[0]) as i32 as i64 as u64) as i64)
                        == (((0xe122u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    48
                };
            }
            50 => {
                {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            53 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                values[3] = memory.read_u64(relocate("STACK_local_30"))?;
                pc = 53;
            }
            55 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x160u64))?,
                        values[3],
                        relocate("STACK_local_30"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x100eu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = memory.read_u64(relocate("STACK_local_30"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_58")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            62 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    61
                } else {
                    60
                };
            }
            63 => {
                values[3] = memory.read_u64(relocate("STACK_local_30"))?;
                pc = 62;
            }
            64 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0xa8u64))?,
                        values[2],
                        relocate("STACK_local_30"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
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
                pc = 68;
            }
            70 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    69
                } else {
                    68
                };
            }
            71 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                values[6] = ((((values[5]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                    | ((values[4]) & (0xffffffffu64)))
                    & (0xffffffffffffff00u64);
                pc = 0;
            }
            73 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x25bu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    71
                } else {
                    72
                };
            }
            74 => {
                values[2] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 73;
            }
            75 => {
                values[5] = u64::from(
                    (u64::from(((values[1]).wrapping_shr(0x20u64 as u32)) as u32)) as u32,
                );
                pc = 74;
            }
            76 => {
                values[4] = ((values[1]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 75;
            }
            77 => {
                values[1] = {
                    let call_arguments = [memory.read_u64(relocate("DAT_1401d06e0"))?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                {
                    let destination = relocate("STACK_local_20");
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

/// Translation of `FUN_140007c00` (`mspaint.c:11594`).
pub fn fun_140007c00(
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
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 95;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[7]));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_140001b30", &call_arguments, memory)
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
                values[6] = values[3];
                pc = 2;
            }
            4 => {
                values[7] = 0x0u64;
                pc = 3;
            }
            5 => {
                values[7] = 0x1u64;
                pc = 2;
            }
            6 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            7 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                                .wrapping_add(0x1a0u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x713cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                pc = 5;
            }
            13 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            14 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                                .wrapping_add(0x1a8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x714cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                pc = 5;
            }
            20 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
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
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                                .wrapping_add(0x1b0u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x715cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                pc = 5;
            }
            27 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            28 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    26
                };
            }
            29 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                                .wrapping_add(0x198u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x712cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                pc = 4;
            }
            34 => {
                pc = 5;
            }
            35 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            36 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                                .wrapping_add(0x188u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x710cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 39;
            }
            41 => {
                pc = 5;
            }
            42 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            43 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                                .wrapping_add(0x190u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_38"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x711cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 46;
            }
            48 => {
                pc = if (u64::from(
                    (u64::from((values[8]) as u32)) == (u64::from((0xe12cu64) as u32)),
                )) != 0
                {
                    11
                } else {
                    33
                };
            }
            49 => {
                pc = if (u64::from(
                    (u64::from((values[8]) as u32)) == (u64::from((0xe12bu64) as u32)),
                )) != 0
                {
                    18
                } else {
                    48
                };
            }
            50 => {
                pc = if (u64::from(
                    (u64::from((values[8]) as u32)) == (u64::from((0xe12au64) as u32)),
                )) != 0
                {
                    25
                } else {
                    49
                };
            }
            51 => {
                pc = if (u64::from(
                    (u64::from((values[8]) as u32)) == (u64::from((0xe125u64) as u32)),
                )) != 0
                {
                    32
                } else {
                    50
                };
            }
            52 => {
                pc = if (u64::from(
                    (u64::from((values[8]) as u32)) == (u64::from((0xe123u64) as u32)),
                )) != 0
                {
                    40
                } else {
                    51
                };
            }
            53 => {
                pc = if (u64::from(
                    (u64::from((values[8]) as u32)) == (u64::from((0xe122u64) as u32)),
                )) != 0
                {
                    47
                } else {
                    52
                };
            }
            54 => {
                values[8] = u64::from((values[0]) as u32);
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(
                        (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?)) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    54
                };
            }
            56 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            57 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    56
                } else {
                    55
                };
            }
            58 => {
                values[6] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 57;
            }
            59 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x48u64))?,
                        values[3],
                        relocate("STACK_local_30"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x6e65u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination =
                        (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                pc = if (u64::from(((values[2]) as i64) != ((0x0u64) as i64))) != 0 {
                    64
                } else {
                    63
                };
            }
            66 => {
                {
                    let call_arguments = [relocate("STACK_local_50")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 65;
            }
            67 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    66
                } else {
                    65
                };
            }
            68 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            70 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    69
                } else {
                    68
                };
            }
            71 => {
                values[3] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 70;
            }
            72 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0x160u64))?,
                        values[6],
                        relocate("STACK_local_38"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x100eu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_68")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            79 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    78
                } else {
                    77
                };
            }
            80 => {
                values[6] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 79;
            }
            81 => {
                values[4] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0xa8u64))?,
                        values[2],
                        relocate("STACK_local_38"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination =
                        (relocate("STACK_local_68")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
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
                pc = 85;
            }
            87 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    86
                } else {
                    85
                };
            }
            88 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                return Ok(Some(
                    ((((values[5]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                        | ((values[4]) & (0xffffffffu64)))
                        & (0xffffffffffffff00u64),
                ));
            }
            90 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x25bu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    89
                } else {
                    88
                };
            }
            91 => {
                values[2] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 90;
            }
            92 => {
                values[5] = u64::from(
                    (u64::from(((values[1]).wrapping_shr(0x20u64 as u32)) as u32)) as u32,
                );
                pc = 91;
            }
            93 => {
                values[4] = ((values[1]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 92;
            }
            94 => {
                values[1] = {
                    let call_arguments = [memory.read_u64(relocate("DAT_1401d06e0"))?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                };
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_88"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400160b0` (`mspaint.c:23869`).
pub fn fun_1400160b0(
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
    let mut pc: i32 = 93;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[11]));
            }
            1 => {
                {
                    let call_arguments = [values[1]];
                    invoke("GlobalUnlock", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[1]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_70"))?];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_70"))?) != (0x0u64))) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [relocate("STACK_local_a8")];
                    invoke("CClientDC::~CClientDC", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[5]];
                    invoke("DeleteObject", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (values[13]) != 0 { 6 } else { 5 };
            }
            8 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[10])?)?,
                        values[10],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    8
                } else {
                    7
                };
            }
            10 => {
                values[11] = 0x0u64;
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, values[12], 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_e8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                values[12] = 0x409u64;
                pc = 13;
            }
            15 => {
                pc = 44;
            }
            16 => {
                pc = if (u64::from(!((values[6]) != 0))) != 0 {
                    15
                } else {
                    14
                };
            }
            17 => {
                values[6] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke(
                            "Paint::PlatformHelpers::IsMissingMediaFramework",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [values[14], values[2]];
                    invoke("FUN_1400177a0", &call_arguments, memory)
                };
                pc = 9;
            }
            19 => {
                values[14] = u64::from(
                    ({
                        let call_arguments = [values[0], 0x1u64, 0x1u64];
                        invoke("FUN_1400153f0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 18;
            }
            20 => {
                values[11] = 0x1u64;
                pc = 19;
            }
            21 => {
                pc = 44;
            }
            22 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    20
                };
            }
            23 => {
                values[7] = ({
                    let call_arguments = [values[0], values[10], 0x0u64];
                    invoke("FUN_140015700", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 22;
            }
            24 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = ((((values[4]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_80"))
                                            .wrapping_add((0x0u64).wrapping_mul(4)),
                                    )?,
                                )) as u32,
                            )) as u32,
                        )) & (0xffffffffu64)))
                        & (0xffffff00ffffffffu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_uStack_60"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = values[0];
                    let stored = memory.read_u64(relocate("STACK_local_68"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_uStack_d4");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_uStack_d4"))?)) as u32,
                            )) & (0xffffff00u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = u64::from(
                        (u64::from(memory.read_u32(
                            (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(4)),
                        )?)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[4] = u64::from(
                    (u64::from((u64::from(memory.read_u32(relocate("STACK_uStack_d4"))?)) as u32))
                        as u32,
                );
                pc = 30;
            }
            32 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("GdipGetImagePixelFormat", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 31;
            }
            33 => {
                {
                    let call_arguments = [
                        values[0],
                        u64::from(
                            (u64::from(memory.read_u32(
                                (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(4)),
                            )?)) as u32,
                        ),
                        u64::from(
                            (u64::from(memory.read_u32(
                                (relocate("STACK_local_78")).wrapping_add((0x0u64).wrapping_mul(4)),
                            )?)) as u32,
                        ),
                    ];
                    invoke("FUN_140017a60", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f64::from_bits(0x0u64))
                            < ((f32::from_bits(
                                (u64::from(
                                    (u64::from(
                                        memory.read_u32(
                                            (relocate("STACK_local_80"))
                                                .wrapping_add((0x0u64).wrapping_mul(4)),
                                        )?,
                                    )) as u32,
                                )) as u32,
                            )) as f64),
                    )) != 0)
                        && ((u64::from(
                            (f64::from_bits(0x0u64))
                                < ((f32::from_bits(
                                    (u64::from(
                                        (u64::from(
                                            memory.read_u32(
                                                (relocate("STACK_local_78"))
                                                    .wrapping_add((0x0u64).wrapping_mul(4)),
                                            )?,
                                        )) as u32,
                                    )) as u32,
                                )) as f64),
                        )) != 0),
                )) != 0
                {
                    33
                } else {
                    32
                };
            }
            35 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("GdipGetImageVerticalResolution", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let destination =
                        (relocate("STACK_local_78")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from(((f64::from_bits(0x0u64)) as f32).to_bits());
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("GdipGetImageHorizontalResolution", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 40;
            }
            42 => {
                {
                    let destination =
                        (relocate("STACK_local_80")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = values[3];
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            43 => {
                pc = 13;
            }
            44 => {
                values[12] = 0x3f4u64;
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    ((f32::from_bits((values[3]) as u32)) as f64) != (f64::from_bits(0x0u64)),
                )) != 0
                {
                    44
                } else {
                    42
                };
            }
            46 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 45;
            }
            47 => {
                values[3] = u64::from(
                    (u64::from(
                        memory.read_u32((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?,
                    )) as u32,
                );
                pc = 46;
            }
            48 => {
                pc = if (u64::from((values[10]) == (0x0u64))) != 0 {
                    17
                } else {
                    47
                };
            }
            49 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x0u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_e8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x4u64);
                    let stored = u64::from((0x2ef31ef8u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x0u64);
                    let stored = u64::from((0x7b9du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x6u64);
                    let stored = u64::from((0x11d3u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x4u64);
                    let stored = u64::from((0x728u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x0u64);
                    let stored =
                        u64::from(((0u64.wrapping_sub(0x4694c357u64)) as i32 as i64 as u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                pc = 56;
            }
            58 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    58
                } else {
                    51
                };
            }
            60 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64((values[10]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        relocate("STACK_local_68"),
                    ];
                    invoke("GdipGetImageRawFormat", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                pc = if (u64::from((values[10]) == (0x0u64))) != 0 {
                    56
                } else {
                    60
                };
            }
            62 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x4u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x0u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x6u64);
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x4u64);
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x0u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 66;
            }
            68 => {
                values[10] = {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_c8"))?,
                        memory.read_u64(relocate("STACK_local_70"))?,
                    ];
                    invoke("FUN_14001a000", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let destination = (values[0]).wrapping_add((0x146u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 68;
            }
            70 => {
                pc = 7;
            }
            71 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x3f4u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = relocate("STACK_local_e8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 72;
            }
            74 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_c8"))?) == (0x0u64))) != 0
                {
                    73
                } else {
                    69
                };
            }
            75 => {
                values[13] = u64::from(
                    (u64::from((memory.read_u64(relocate("STACK_local_c8"))?) != (0x0u64))) != 0,
                );
                pc = 74;
            }
            76 => {
                values[5] = memory.read_u64(relocate("STACK_local_c8"))?;
                pc = 75;
            }
            77 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_c8"),
                        values[9],
                        memory.read_u64(relocate("STACK_uStack_a0"))?,
                    ];
                    invoke("FUN_140045bb0", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let call_arguments = [relocate("STACK_local_a8"), 0x0u64];
                    invoke("CClientDC::CClientDC", &call_arguments, memory)
                };
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("STACK_uStack_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_uStack_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke("CreateDibPalette", &call_arguments, memory)
                };
                pc = 84;
            }
            86 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                values[1] = values[11];
                pc = 88;
            }
            90 => {
                pc = if (u64::from((values[9]) == (0x0u64))) != 0 {
                    89
                } else {
                    88
                };
            }
            91 => {
                values[11] = 0x0u64;
                pc = 90;
            }
            92 => {
                values[9] = {
                    let call_arguments = [values[1]];
                    invoke("GlobalLock", &call_arguments, memory)
                };
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_118"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140017480` (`mspaint.c:24770`).
pub fn fun_140017480(
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
        (arguments[2]) as i32 as i64 as u64,
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
    ];
    let mut pc: i32 = 105;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, values[16], 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = relocate("STACK_local_88");
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 2;
            }
            4 => {
                values[16] = 0x409u64;
                pc = 3;
            }
            5 => {
                values[16] = 0x3f4u64;
                pc = 3;
            }
            6 => {
                return Ok(Some(0x1u64));
            }
            7 => {
                {
                    let call_arguments = [values[8], 0x58u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_58"))?];
                    invoke("BasePaint::Image::~Image", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) != (0x0u64))) != 0
                {
                    8
                } else {
                    6
                };
            }
            10 => {
                values[8] = memory.read_u64(relocate("STACK_local_58"))?;
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[13])?).wrapping_add(0x8u64))?,
                        values[13],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 12;
            }
            14 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                values[11] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 14;
            }
            16 => {
                values[4] = (values[13]).wrapping_add(0xcu64);
                pc = 15;
            }
            17 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_60"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_60"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    10
                };
            }
            20 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 19;
            }
            21 => {
                {
                    let destination = values[15];
                    let stored = ((((memory.read_u64(values[15])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                values[5] = memory.read_u64(values[15])?;
                pc = 21;
            }
            23 => {
                values[15] = (memory.read_u64(relocate("STACK_plStack_60"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_60"))?) != (0x0u64)))
                    != 0
                {
                    24
                } else {
                    10
                };
            }
            26 => {
                values[13] = memory.read_u64(relocate("STACK_plStack_60"))?;
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [values[14], 0x0u64, values[6]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                values[14] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            30 => {
                pc = if (u64::from(
                    (((((values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    26
                };
            }
            31 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    30
                };
            }
            32 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 31;
            }
            33 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 32;
            }
            34 => {
                values[11] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 33;
            }
            35 => {
                values[4] = (values[6]).wrapping_add(0x18u64);
                pc = 34;
            }
            36 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 35;
            }
            37 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    36
                } else {
                    26
                };
            }
            38 => {
                values[6] = (((u64::from(
                    (u64::from(memory.read_u32(relocate("STACK_uStack_4c"))?)) as u32,
                )) & (0xffffffffu64))
                    .wrapping_shl(0x20u64 as u32))
                    | ((u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_50"))?)) as u32,
                        )) as u32,
                    )) & (0xffffffffu64));
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [values[15]];
                    invoke("FUN_140041100", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                {
                    let call_arguments = [relocate("STACK_local_50"), relocate("STACK_local_88")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = relocate("L\"Current\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [values[15], relocate("STACK_local_68")];
                    invoke("FUN_140040ed0", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [(values[15]).wrapping_add((0xfu64).wrapping_mul(8))];
                    invoke("FUN_1400414a0", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [values[15]];
                    invoke("FUN_1400401a0", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                {
                    let destination = relocate("STACK_plStack_60");
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                values[13] = values[12];
                pc = 48;
            }
            50 => {
                {
                    let destination = (values[12]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = values[12];
                    let stored = relocate("std::_Ref_count_resource<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (values[12]).wrapping_add(0xcu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = values[12];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                pc = if (u64::from((values[12]) != (0x0u64))) != 0 {
                    55
                } else {
                    48
                };
            }
            57 => {
                values[12] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) != (0x0u64))) != 0
                {
                    57
                } else {
                    45
                };
            }
            59 => {
                {
                    let destination = relocate("STACK_plStack_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                values[15] = values[13];
                pc = 60;
            }
            62 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x133u64).wrapping_mul(8)))?)
                        == (0x0u64),
                )) != 0
                {
                    61
                } else {
                    60
                };
            }
            63 => {
                values[15] = (memory
                    .read_u64((values[0]).wrapping_add((0x133u64).wrapping_mul(8)))?)
                .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                pc = 62;
            }
            64 => {
                values[8] = memory.read_u64(relocate("STACK_local_58"))?;
                pc = 63;
            }
            65 => {
                {
                    let call_arguments = [relocate("STACK_local_58"), relocate("DAT_1401d06b8")];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                pc = 3;
            }
            68 => {
                values[16] = 0x3f4u64;
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    68
                } else {
                    6
                };
            }
            70 => {
                values[10] = ({
                    let call_arguments = [
                        values[0],
                        memory.read_u64(values[1])?,
                        u64::from(
                            (((values[2]) as i32 as i64 as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        ),
                    ];
                    invoke("FUN_140015700", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 69;
            }
            71 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    66
                } else {
                    70
                };
            }
            72 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = ((((values[7]) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_58")).wrapping_add(0x0u64))?,
                            )) as u32,
                        )) & (0xffffffffu64)))
                        & (0xffffff00ffffffffu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = values[0];
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_uStack_74");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_uStack_74"))?)) as u32,
                            )) & (0xffffff00u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_58")).wrapping_add(0x0u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                values[16] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 76;
            }
            78 => {
                {
                    let destination = (values[5]).wrapping_add(0x10u64);
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 77;
            }
            79 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    78
                } else {
                    77
                };
            }
            80 => {
                values[7] = u64::from(
                    (u64::from((u64::from(memory.read_u32(relocate("STACK_uStack_74"))?)) as u32))
                        as u32,
                );
                pc = 79;
            }
            81 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64((values[5]).wrapping_add(0x8u64))?,
                        relocate("STACK_local_58"),
                    ];
                    invoke("GdipGetImagePixelFormat", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 80;
            }
            82 => {
                values[5] = memory.read_u64(values[1])?;
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [
                        values[0],
                        u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_58")).wrapping_add(0x0u64))?,
                            )) as u32,
                        ),
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_50"))?)) as u32),
                    ];
                    invoke("FUN_140017a60", &call_arguments, memory)
                };
                pc = 82;
            }
            84 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f64::from_bits(0x0u64))
                            < ((u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_58")).wrapping_add(0x0u64),
                                    )?,
                                )) as u32,
                            )) as f64),
                    )) != 0)
                        && ((u64::from(
                            (f64::from_bits(0x0u64))
                                < ((f32::from_bits(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("STACK_local_50"))?))
                                            as u32,
                                    )) as u32,
                                )) as f64),
                        )) != 0),
                )) != 0
                {
                    83
                } else {
                    82
                };
            }
            85 => {
                {
                    let destination = (values[5]).wrapping_add(0x10u64);
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 84;
            }
            86 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    85
                } else {
                    84
                };
            }
            87 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64((values[5]).wrapping_add(0x8u64))?,
                        relocate("STACK_local_50"),
                    ];
                    invoke("GdipGetImageVerticalResolution", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = u64::from(((f64::from_bits(0x0u64)) as f32).to_bits());
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 87;
            }
            89 => {
                values[5] = memory.read_u64(values[1])?;
                pc = 88;
            }
            90 => {
                {
                    let destination = (values[5]).wrapping_add(0x10u64);
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 89;
            }
            91 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    90
                } else {
                    89
                };
            }
            92 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64((values[5]).wrapping_add(0x8u64))?,
                        relocate("STACK_local_58"),
                    ];
                    invoke("GdipGetImageHorizontalResolution", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_58"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                values[5] = memory.read_u64(values[1])?;
                pc = 93;
            }
            95 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    94
                } else {
                    5
                };
            }
            96 => {
                {
                    let destination = (values[5]).wrapping_add(0x10u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 95;
            }
            97 => {
                values[13] = 0x0u64;
                pc = 96;
            }
            98 => {
                values[11] = ((u64::from(memory.read_u32((values[5]).wrapping_add(0x10u64))?))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 97;
            }
            99 => {
                pc = if (u64::from(((values[5]) as i64) != ((0x0u64) as i64))) != 0 {
                    98
                } else {
                    5
                };
            }
            100 => {
                values[5] = memory.read_u64(values[1])?;
                pc = 99;
            }
            101 => {
                pc = if (u64::from(
                    ((u64::from(((memory.read_u64(values[1])?) as i64) == ((0x0u64) as i64))) != 0)
                        && (({
                            let _ = {
                                values[9] = u64::from(
                                    ({
                                        let call_arguments = [];
                                        invoke(
                                            "Paint::PlatformHelpers::IsMissingMediaFramework",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[9]
                            };
                            values[9]
                        }) != 0),
                )) != 0
                {
                    4
                } else {
                    100
                };
            }
            102 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x0u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = relocate("STACK_local_88");
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14003fcf0` (`mspaint.c:59426`).
pub fn fun_14003fcf0(
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
    ];
    let mut pc: i32 = 133;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x1u64));
            }
            1 => {
                {
                    let call_arguments = [values[15], 0x0u64, values[6]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[15] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            4 => {
                pc = if (u64::from(
                    (((((values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    0
                };
            }
            5 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    4
                };
            }
            6 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 5;
            }
            7 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                values[10] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 7;
            }
            9 => {
                values[3] = (memory.read_u64(relocate("STACK_local_a0"))?).wrapping_add(0x18u64);
                pc = 8;
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_a0"))?) != (0x0u64))) != 0
                {
                    10
                } else {
                    0
                };
            }
            12 => {
                values[6] = memory.read_u64(relocate("STACK_local_a0"))?;
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_140041100", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [relocate("STACK_local_a0"), relocate("STACK_local_80")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = relocate("L\"Current\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [(values[0]).wrapping_add(0x40u64), values[8]];
                    invoke("FUN_140015a30", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                values[8] = u64::from(
                    ({
                        let call_arguments = [values[16]];
                        invoke(
                            "Paint::ProjectFile::Background::IsVisible",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 17;
            }
            19 => {
                values[16] = {
                    let call_arguments = [values[1]];
                    invoke(
                        "Paint::ProjectFile::LayerStack::Background",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14001aa10", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [values[16]];
                    invoke(
                        "Paint::ProjectFile::Background::Color",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 20;
            }
            22 => {
                values[16] = {
                    let call_arguments = [values[1]];
                    invoke(
                        "Paint::ProjectFile::LayerStack::Background",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (values[17])
                        < ({
                            let numerator = ((memory
                                .read_u64((values[11]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                            .wrapping_sub(memory.read_u64(values[11])?))
                                as i64;
                            let denominator = (0x18u64) as i64;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                as u64
                        }),
                )) != 0
                {
                    127
                } else {
                    22
                };
            }
            24 => {
                values[11] = {
                    let call_arguments = [values[1]];
                    invoke(
                        "Paint::ProjectFile::LayerStack::Layers",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 23;
            }
            25 => {
                values[17] =
                    (((values[17]) as i32 as i64 as u64).wrapping_add(0x1u64)) as i32 as i64 as u64;
                pc = 24;
            }
            26 => {
                {
                    let call_arguments = [values[7], 0x58u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_60"))?];
                    invoke("BasePaint::Image::~Image", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    27
                } else {
                    25
                };
            }
            29 => {
                values[7] = memory.read_u64(relocate("STACK_local_60"))?;
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [relocate("STACK_local_a8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_a8"))?) != (0x0u64))) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                pc = 124;
            }
            34 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    32
                };
            }
            35 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_70"))?)?)
                                .wrapping_add(0x48u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_70"))?,
                        values[8],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x4435u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_98")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            40 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x4433u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_a8"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_a8"))?,
                        relocate("DAT_140105578"),
                        relocate("STACK_local_70"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_a8"))?) != (0x0u64))) != 0
                {
                    44
                } else {
                    43
                };
            }
            46 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                values[8] = u64::from(
                    ({
                        let call_arguments = [values[4]];
                        invoke(
                            "Paint::ProjectFile::ImageLayer::IsVisible",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let call_arguments = [values[15], 0x0u64, values[6]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                values[15] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            52 => {
                pc = if (u64::from(
                    (((((values[9]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    51
                } else {
                    48
                };
            }
            53 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    52
                };
            }
            54 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 53;
            }
            55 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                values[9] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 55;
            }
            57 => {
                values[3] = (memory.read_u64(relocate("STACK_local_a0"))?).wrapping_add(0x18u64);
                pc = 56;
            }
            58 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 57;
            }
            59 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_a0"))?) != (0x0u64))) != 0
                {
                    58
                } else {
                    47
                };
            }
            60 => {
                {
                    let call_arguments = [values[9], relocate("STACK_local_98")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            61 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    60
                } else {
                    59
                };
            }
            62 => {
                values[6] = memory.read_u64(relocate("STACK_local_a0"))?;
                pc = 61;
            }
            63 => {
                values[9] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_a8"))?)?)
                            .wrapping_add(0x30u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 62;
            }
            64 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x43d0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let call_arguments = [values[4]];
                    invoke(
                        "Paint::ProjectFile::ImageLayer::Name",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 66;
            }
            68 => {
                {
                    let call_arguments = [values[0], relocate("STACK_local_a8")];
                    invoke("FUN_140040540", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let call_arguments = [values[0], values[17]];
                    invoke("FUN_140041070", &call_arguments, memory)
                };
                pc = 68;
            }
            70 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x38u64),
                        values[17],
                        relocate("STACK_local_a8"),
                    ];
                    invoke("FUN_14003f420", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[14])?).wrapping_add(0x8u64))?,
                        values[14],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 70;
            }
            72 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    71
                } else {
                    70
                };
            }
            73 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 72;
            }
            74 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 73;
            }
            75 => {
                values[9] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 74;
            }
            76 => {
                values[3] = (values[14]).wrapping_add(0xcu64);
                pc = 75;
            }
            77 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 76;
            }
            78 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_plStack_68"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_plStack_68"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 77;
            }
            79 => {
                pc = if (u64::from(
                    ((((values[5]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    78
                } else {
                    70
                };
            }
            80 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 79;
            }
            81 => {
                {
                    let destination = values[2];
                    let stored = ((((memory.read_u64(values[2])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 80;
            }
            82 => {
                values[5] = memory.read_u64(values[2])?;
                pc = 81;
            }
            83 => {
                values[2] = (memory.read_u64(relocate("STACK_plStack_68"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 82;
            }
            84 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 83;
            }
            85 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_plStack_68"))?) != (0x0u64)))
                    != 0
                {
                    84
                } else {
                    70
                };
            }
            86 => {
                values[14] = memory.read_u64(relocate("STACK_plStack_68"))?;
                pc = 85;
            }
            87 => {
                {
                    let call_arguments = [
                        values[0],
                        relocate("STACK_local_a8"),
                        relocate("STACK_local_70"),
                    ];
                    invoke("FUN_140040280", &call_arguments, memory)
                };
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("STACK_plStack_68");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination = (values[14]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination = values[14];
                    let stored = relocate("std::_Ref_count_resource<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = (values[14]).wrapping_add(0xcu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = (values[14]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = (values[14]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = values[14];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    96
                } else {
                    89
                };
            }
            98 => {
                {
                    let destination = relocate("STACK_plStack_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                values[14] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    99
                } else {
                    87
                };
            }
            101 => {
                {
                    let destination = relocate("STACK_plStack_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                values[7] = memory.read_u64(relocate("STACK_local_60"))?;
                pc = 102;
            }
            104 => {
                {
                    let call_arguments = [values[0], relocate("STACK_local_80"), 0x1u64];
                    invoke("FUN_14003d1f0", &call_arguments, memory)
                };
                pc = 103;
            }
            105 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = (((u64::from(
                        (u64::from(
                            (((memory.read_u64(relocate("STACK_local_70"))?) & (0xffffffffu64))
                                as f32)
                                .to_bits(),
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((u64::from((u64::from(((values[13]) as f32).to_bits())) as u32))
                            & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[12]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = (values[9]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 105;
            }
            107 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    106
                } else {
                    105
                };
            }
            108 => {
                values[9] = ({
                    let call_arguments = [
                        memory.read_u64((values[12]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        relocate("STACK_local_70"),
                    ];
                    invoke("GdipGetImageHeight", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_70"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                values[13] = (memory.read_u64(relocate("STACK_local_70"))?) & (0xffffffffu64);
                pc = 109;
            }
            111 => {
                {
                    let destination = (values[12]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = (values[9]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 110;
            }
            112 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    111
                } else {
                    110
                };
            }
            113 => {
                values[9] = ({
                    let call_arguments = [
                        memory.read_u64((values[12]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                        relocate("STACK_local_70"),
                    ];
                    invoke("GdipGetImageWidth", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_70"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                pc = if (u64::from(
                    ((((values[17]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    114
                } else {
                    103
                };
            }
            116 => {
                values[10] = (0x0u64) as i32 as i64 as u64;
                pc = 115;
            }
            117 => {
                return Ok(Some(0x0u64));
            }
            118 => {
                {
                    let destination = relocate("_DAT_1401d0858");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d0858"))?)) as u16,
                        )) | (0x1au64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 117;
            }
            119 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) == (0x0u64))) != 0
                {
                    118
                } else {
                    116
                };
            }
            120 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_60"),
                        (memory.read_u64((values[0]).wrapping_add(0x28u64))?).wrapping_add(0x18u64),
                    ];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                values[12] = {
                    let call_arguments = [values[4]];
                    invoke(
                        "Paint::ProjectFile::ImageLayer::Bitmap",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 121;
            }
            123 => {
                values[4] =
                    (memory.read_u64(values[11])?).wrapping_add((values[17]).wrapping_mul(0x18u64));
                pc = 122;
            }
            124 => {
                {
                    let call_arguments = [values[10], relocate("STACK_local_98")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            125 => {
                values[10] = ({
                    let call_arguments = [];
                    invoke("FUN_140042cd0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 124;
            }
            126 => {
                pc = if (u64::from(
                    ({
                        let numerator = ((memory
                            .read_u64((values[11]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                        .wrapping_sub(memory.read_u64(values[11])?))
                            as i64;
                        let denominator = (0x18u64) as i64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    }) <= (values[17]),
                )) != 0
                {
                    125
                } else {
                    123
                };
            }
            127 => {
                values[11] = {
                    let call_arguments = [values[1]];
                    invoke(
                        "Paint::ProjectFile::LayerStack::Layers",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 126;
            }
            128 => {
                pc = if (u64::from(
                    ((({
                        let numerator = ((memory
                            .read_u64((values[11]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                        .wrapping_sub(memory.read_u64(values[11])?))
                            as i64;
                        let denominator = (0x18u64) as i64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    })
                    .wrapping_add(values[5])) as i64)
                        != ((values[5]) as i64),
                )) != 0
                {
                    127
                } else {
                    22
                };
            }
            129 => {
                values[17] = 0x0u64;
                pc = 128;
            }
            130 => {
                values[5] = ((memory
                    .read_u64((values[11]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                .wrapping_sub(memory.read_u64(values[11])?) as i64)
                    .wrapping_shr(0x3fu64 as u32) as u64;
                pc = 129;
            }
            131 => {
                values[11] = {
                    let call_arguments = [values[1]];
                    invoke(
                        "Paint::ProjectFile::LayerStack::Layers",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 130;
            }
            132 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400401a0", &call_arguments, memory)
                };
                pc = 131;
            }
            133 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 132;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400517d0` (`mspaint.c:74143`).
pub fn fun_1400517d0(
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
    let mut pc: i32 = 89;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            4 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0xd8u64))?,
                        values[10],
                        0x0u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0xee3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
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
                    let call_arguments = [values[7], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            11 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    2
                };
            }
            12 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0xd8u64))?,
                        values[10],
                        0x0u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0xee3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 2;
            }
            18 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    17
                } else {
                    2
                };
            }
            19 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            20 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_50"))?)?)
                                .wrapping_add(0xd8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_50"))?,
                        values[4],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0xee3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                values[10] = memory.read_u64(relocate("STACK_local_50"))?;
                pc = 24;
            }
            26 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    26
                } else {
                    25
                };
            }
            28 => {
                values[4] = memory.read_u64(relocate("STACK_local_60"))?;
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            30 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_58"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_58"))?,
                        relocate("DAT_140111898"),
                        relocate("STACK_local_60"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) != (0x0u64))) != 0
                {
                    34
                } else {
                    28
                };
            }
            36 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                values[5] = memory.read_u64(relocate("STACK_local_58"))?;
                pc = 36;
            }
            38 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_58"),
                        relocate("STACK_local_60"),
                    ];
                    invoke("FUN_14005b570", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("_DAT_1401cfdc8");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 39;
            }
            41 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 40;
            }
            42 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 37;
            }
            43 => {
                {
                    let destination = relocate("_DAT_1401cfdc8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfdc8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_88")];
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
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfdc0"))?)?)
                                .wrapping_add(0xc0u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfdc0"))?,
                        (((u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_fStack_44"))?)) as u32,
                            )) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl(0x20u64 as u32))
                            | ((u64::from((values[12]) as u32)) & (0xffffffffu64)),
                        relocate("STACK_local_58"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x633u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfdc0"))?) == (0x0u64))) != 0
                {
                    41
                } else {
                    51
                };
            }
            53 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[12];
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 52;
            }
            54 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("_DAT_1401cfdc8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfdc8"))?).wrapping_add(0x1u64);
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
                    let destination = relocate("STACK_local_68");
                    let stored = relocate("DAT_1401cfdc8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = relocate("STACK_local_48");
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_fStack_44");
                    let stored = u64::from((((values[8]) as i64) as f32).to_bits());
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 58;
            }
            60 => {
                values[3] = memory.read_u64(relocate("_DAT_1401cfdc8"))?;
                pc = 59;
            }
            61 => {
                values[8] = ({
                    let call_arguments = [values[11]];
                    invoke("PGS::PGSRect::Height", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 60;
            }
            62 => {
                values[12] = u64::from((((values[8]) as i64) as f32).to_bits());
                pc = 61;
            }
            63 => {
                values[8] = ({
                    let call_arguments = [values[11]];
                    invoke("PGS::PGSRect::Width", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 62;
            }
            64 => {
                values[11] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0xf0u64))?,
                        values[10],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                values[10] = {
                    let call_arguments = [
                        memory.read_u64((values[1]).wrapping_add(0x10u64))?,
                        0x0u64,
                        relocate("IPaintPublisher::RTTI_Type_Descriptor"),
                        relocate("COperation::RTTI_Type_Descriptor"),
                    ];
                    invoke("__RTDynamicCast", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 65;
            }
            67 => {
                pc = if (u64::from(
                    ((u64::from((values[2]) == (0x0u64))) != 0)
                        || (({
                            let _ = {
                                values[6] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(values[2])?).wrapping_add(0x68u64),
                                        )?,
                                        values[2],
                                        0x12u64,
                                    ];
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
                    16
                } else {
                    66
                };
            }
            68 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32((values[1]).wrapping_add(0x8u64))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x15u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    68
                };
            }
            70 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    70
                } else {
                    69
                };
            }
            72 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = memory.read_u64(relocate("STACK_local_50"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_88")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            74 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    73
                } else {
                    72
                };
            }
            75 => {
                values[10] = memory.read_u64(relocate("STACK_local_50"))?;
                pc = 74;
            }
            76 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0xa8u64))?,
                        values[4],
                        relocate("STACK_local_50"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x33au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
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
                pc = 80;
            }
            82 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64))) != 0
                {
                    81
                } else {
                    80
                };
            }
            83 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                values[4] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 83;
            }
            85 => {
                {
                    let call_arguments = [(memory.read_u64(
                        (memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                            .wrapping_add(0x40u64),
                    )?)
                    .wrapping_add(0x1e0u64)];
                    invoke("FUN_1400a2fa0", &call_arguments, memory)
                };
                pc = 84;
            }
            86 => {
                pc = if (u64::from(
                    ((memory.read_u64(
                        (memory.read_u64((values[9]).wrapping_add((0x8u64).wrapping_mul(1)))?)
                            .wrapping_add(0x40u64),
                    )?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    85
                } else {
                    84
                };
            }
            87 => {
                values[9] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[1]).wrapping_add(0x10u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    87
                } else {
                    0
                };
            }
            89 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140053840` (`mspaint.c:75890`).
pub fn fun_140053840(
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
    let mut pc: i32 = 20;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [values[2], relocate("STACK_local_38")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            1 => {
                return Ok(None);
            }
            2 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_40"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    2
                } else {
                    1
                };
            }
            4 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[2]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    0
                };
            }
            5 => {
                values[2] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[5])?)?).wrapping_add(0x148u64),
                        )?,
                        memory.read_u64(values[5])?,
                        (((u64::from(
                            (u64::from((((values[4]) as i64) as f32).to_bits())) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl(0x20u64 as u32))
                            | ((u64::from(
                                (u64::from((((values[3]) as i64) as f32).to_bits())) as u32,
                            )) & (0xffffffffu64)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
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
                    let destination = relocate("STACK_local_30");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x708cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                values[4] = ({
                    let call_arguments = [values[1]];
                    invoke("PGS::PGSRect::Height", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 8;
            }
            10 => {
                values[3] = ({
                    let call_arguments = [values[1]];
                    invoke("PGS::PGSRect::Width", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                values[5] = {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_14004e8c0", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_48"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                {
                    let call_arguments = [values[2], relocate("STACK_local_38")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            15 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    14
                } else {
                    13
                };
            }
            16 => {
                values[2] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(values[5])?)?).wrapping_add(0x140u64),
                        )?,
                        memory.read_u64(values[5])?,
                        (((u64::from(
                            (u64::from(
                                ((((u64::from(memory.read_u32(
                                    (values[1]).wrapping_add((0x4u64).wrapping_mul(1)),
                                )?)) as i32 as i64 as u64) as i64)
                                    as f32)
                                    .to_bits(),
                            )) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl(0x20u64 as u32))
                            | ((u64::from(
                                (u64::from(
                                    ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64
                                        as u64) as i64)
                                        as f32)
                                        .to_bits(),
                                )) as u32,
                            )) & (0xffffffffu64)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x707cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                values[5] = {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_14004e8c0", &call_arguments, memory)
                };
                pc = 19;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400a51f0` (`mspaint.c:141619`).
pub fn fun_1400a51f0(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64];
    let mut pc: i32 = 3;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = values[1];
                    let stored = (((u64::from(
                        (u64::from(
                            (((float_to_integer(
                                f64::from_bits(
                                    (((f32::from_bits(
                                        (u64::from((((values[3]) as i64) as f32).to_bits())) as u32,
                                    )) as f64)
                                        * (f64::from_bits(0x3fe3333333333333u64)))
                                    .to_bits(),
                                ),
                                32,
                                true,
                            )?) as i64) as f32)
                                .to_bits(),
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(
                                (((float_to_integer(
                                    f64::from_bits(
                                        (((f32::from_bits(
                                            (u64::from((((values[2]) as i64) as f32).to_bits()))
                                                as u32,
                                        )) as f64)
                                            * (f64::from_bits(0x3fe3333333333333u64)))
                                        .to_bits(),
                                    ),
                                    32,
                                    true,
                                )?) as i64) as f32)
                                    .to_bits(),
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                values[3] = ({
                    let call_arguments = [0x1u64];
                    invoke("GetSystemMetrics", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 1;
            }
            3 => {
                values[2] = ({
                    let call_arguments = [0x0u64];
                    invoke("GetSystemMetrics", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 2;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400c0dc0` (`mspaint.c:168614`).
pub fn fun_1400c0dc0(
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
    ];
    let mut pc: i32 = 355;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x2f0u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x8au64))?)) as i16
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
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_220"))?) != (0x0u64)))
                    != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
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
                    ((memory.read_u64(relocate("STACK_local_f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    9
                } else {
                    8
                };
            }
            11 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x14u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_1f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                {
                    let destination = relocate("STACK_local_1f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x15u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                pc = if (u64::from(((values[13]) as i64) == ((0x1u64) as i64))) != 0 {
                    20
                } else {
                    14
                };
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                {
                    let destination = values[19];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                values[13] = memory.read_u64(values[19])?;
                pc = 17;
            }
            19 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = values[19];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            22 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 21;
            }
            23 => {
                {
                    let destination = values[19];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                values[13] = memory.read_u64(values[19])?;
                pc = 23;
            }
            25 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 24;
            }
            26 => {
                values[19] =
                    memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?;
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    26
                } else {
                    14
                };
            }
            28 => {
                pc = 3;
            }
            29 => {
                return Ok(None);
            }
            30 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[8] = ({
                    let call_arguments = [];
                    invoke("FUN_14000c570", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 30;
            }
            32 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_200"))?;
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[0]).wrapping_add((0x2eu64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_220"))?) != (0x0u64)))
                    != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let call_arguments = [values[7]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    38
                } else {
                    37
                };
            }
            40 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = memory.read_u64(values[7])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let call_arguments = [values[19]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_100"))?) != (0x0u64)))
                    != 0
                {
                    41
                } else {
                    40
                };
            }
            43 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 34;
            }
            45 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_220"))?) != (0x0u64)))
                    != 0
                {
                    44
                } else {
                    34
                };
            }
            46 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let call_arguments = [values[7]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = memory.read_u64(values[7])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [values[19]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_100"))?) != (0x0u64)))
                    != 0
                {
                    50
                } else {
                    49
                };
            }
            52 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let call_arguments = [];
                    invoke("GdipDisposeImage", &call_arguments, memory)
                };
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x21u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let call_arguments = [];
                    invoke("GdipDeleteMatrix", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x5au64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [];
                    invoke("GdipDeleteMatrix", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x2bu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                {
                    let call_arguments = [0x1u64];
                    invoke("FUN_140033a00", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                {
                    let call_arguments = [values[17], 0x0u64, values[5]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 59;
            }
            61 => {
                values[17] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                {
                    let destination = relocate("STACK_local_210");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            64 => {
                pc = if (u64::from(
                    (((((values[11]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    59
                };
            }
            65 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    62
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
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 66;
            }
            68 => {
                values[11] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 67;
            }
            69 => {
                values[1] = (values[5]).wrapping_add(0x18u64);
                pc = 68;
            }
            70 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 69;
            }
            71 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0x2du64).wrapping_mul(8)))?;
                pc = 70;
            }
            72 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x2du64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    71
                } else {
                    59
                };
            }
            73 => {
                {
                    let call_arguments = [
                        values[20],
                        (values[0]).wrapping_add((0x2du64).wrapping_mul(8)),
                        (values[0]).wrapping_add((0x20u64).wrapping_mul(8)),
                        (values[0]).wrapping_add((0x2bu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_1400365d0", &call_arguments, memory)
                };
                pc = 72;
            }
            74 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x2du64).wrapping_mul(8))];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let destination = (values[0]).wrapping_add((0x5cu64).wrapping_mul(8));
                    let stored = 0x9u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = (values[0]).wrapping_add((0x5bu64).wrapping_mul(8));
                    let stored = relocate("L\"Draw text\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                values[20] = (memory.read_u64(values[7])?)
                    .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
                pc = 76;
            }
            78 => {
                pc = if (u64::from(((memory.read_u64(values[7])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    77
                } else {
                    76
                };
            }
            79 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    79
                } else {
                    78
                };
            }
            81 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_140"))?,
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_e8"))?)) as u32),
                        u64::from((u64::from(memory.read_u32(relocate("STACK_local_e4"))?)) as u32),
                        0x1u64,
                    ];
                    invoke("GdipTranslateMatrix", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[0]).wrapping_add((0x1eu64).wrapping_mul(8)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_local_e4");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[0]).wrapping_add(0xf4u64))?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 84;
            }
            86 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    85
                } else {
                    84
                };
            }
            87 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_140"))?,
                        ((f64::from_bits(0x4058000000000000u64))
                            / ((f32::from_bits((u64::from(((values[12]) as f32).to_bits())) as u32))
                                as f64))
                            .to_bits(),
                        ((f64::from_bits(0x4058000000000000u64))
                            / ((f32::from_bits((u64::from(((values[12]) as f32).to_bits())) as u32))
                                as f64))
                            .to_bits(),
                        0x1u64,
                    ];
                    invoke("GdipScaleMatrix", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                values[12] = u64::from(
                    ({
                        let call_arguments = [memory
                            .read_u64((values[15]).wrapping_add((0x40u64).wrapping_mul(1)))?];
                        invoke("GetDpiForWindow", &call_arguments, memory)
                    }) as u32,
                );
                pc = 88;
            }
            90 => {
                values[15] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[14])?).wrapping_add(0xf8u64))?,
                        values[14],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 89;
            }
            91 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    90
                } else {
                    89
                };
            }
            92 => {
                values[15] = values[20];
                pc = 91;
            }
            93 => {
                values[14] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 92;
            }
            94 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 93;
            }
            95 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    94
                } else {
                    93
                };
            }
            96 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_140"))?,
                        (((f32::from_bits((values[21]) as u32)) as f64)
                            * (f64::from_bits(0x3fe0000000000000u64)))
                        .to_bits(),
                        (((f32::from_bits(
                            (u64::from((((values[11]) as i64) as f32).to_bits())) as u32,
                        )) as f64)
                            * (f64::from_bits(0x3fe0000000000000u64)))
                        .to_bits(),
                        0x1u64,
                    ];
                    invoke("GdipTranslateMatrix", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = (values[10]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 97;
            }
            99 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    98
                } else {
                    97
                };
            }
            100 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_140"))?,
                        memory.read_u64(relocate("STACK_local_138"))?,
                        0x1u64,
                    ];
                    invoke("GdipMultiplyMatrix", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = (values[10]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 102;
            }
            104 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    103
                } else {
                    102
                };
            }
            105 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_140"))?,
                        (((0x80000000u64) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                            | (((-(f64::from_bits(
                                (((f32::from_bits((values[21]) as u32)) as f64)
                                    * (f64::from_bits(0x3fe0000000000000u64)))
                                .to_bits(),
                            )))
                            .to_bits())
                                & (0xffffffffu64)),
                        (((0x80000000u64) & (0xffffffffu64)).wrapping_shl(0x20u64 as u32))
                            | (((-(f64::from_bits(
                                (((f32::from_bits(
                                    (u64::from((((values[11]) as i64) as f32).to_bits())) as u32,
                                )) as f64)
                                    * (f64::from_bits(0x3fe0000000000000u64)))
                                .to_bits(),
                            )))
                            .to_bits())
                                & (0xffffffffu64)),
                        0x1u64,
                    ];
                    invoke("GdipTranslateMatrix", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[0]).wrapping_add((0x2bu64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_140"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = memory.read_u64(values[3])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 106;
            }
            108 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = u64::from((values[9]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 107;
            }
            109 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [values[3]];
                        invoke("GdipCreateMatrix", &call_arguments, memory)
                    }) as u32,
                );
                pc = 108;
            }
            110 => {
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                values[3] = (values[0]).wrapping_add((0x59u64).wrapping_mul(8));
                pc = 110;
            }
            112 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
            }
            113 => {
                values[21] =
                    u64::from(
                        ((((memory.read_u64(relocate("STACK_local_210"))?) as i32 as i64 as u64)
                            as i64) as f32)
                            .to_bits(),
                    );
                pc = 112;
            }
            114 => {
                {
                    let destination = (values[0]).wrapping_add((0x23u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = (values[0]).wrapping_add((0x24u64).wrapping_mul(8));
                    let stored = u64::from((values[9]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 114;
            }
            116 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [
                            u64::from(
                                ((f64::from_bits(memory.read_u64(relocate("STACK_local_1c8"))?))
                                    as f32)
                                    .to_bits(),
                            ),
                            u64::from(
                                ((f64::from_bits(memory.read_u64(relocate("STACK_local_1c0"))?))
                                    as f32)
                                    .to_bits(),
                            ),
                            u64::from(
                                ((f64::from_bits(memory.read_u64(relocate("STACK_local_1b8"))?))
                                    as f32)
                                    .to_bits(),
                            ),
                            u64::from(
                                ((f64::from_bits(memory.read_u64(relocate("STACK_local_1b0"))?))
                                    as f32)
                                    .to_bits(),
                            ),
                        ];
                        invoke("GdipCreateMatrix2", &call_arguments, memory)
                    }) as u32,
                );
                pc = 115;
            }
            117 => {
                {
                    let destination = relocate("STACK_local_238");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_248")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(
                                ((f64::from_bits(memory.read_u64(relocate("STACK_local_1a8"))?))
                                    as f32)
                                    .to_bits(),
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_240")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((u64::from(
                            (u64::from(
                                ((f64::from_bits(memory.read_u64(relocate("STACK_local_1a0"))?))
                                    as f32)
                                    .to_bits(),
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = relocate("STACK_local_1c8");
                    let stored = ((memory
                        .read_u64((values[0]).wrapping_add((0x25u64).wrapping_mul(8)))?)
                        as f64)
                        .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_local_1c0");
                    let stored = ((memory
                        .read_u64((values[0]).wrapping_add((0x26u64).wrapping_mul(8)))?)
                        as f64)
                        .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let destination = relocate("STACK_local_1b8");
                    let stored = ((memory
                        .read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                        as f64)
                        .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let destination = relocate("STACK_local_1b0");
                    let stored = ((memory
                        .read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?)
                        as f64)
                        .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let destination = relocate("STACK_local_1a8");
                    let stored = ((memory
                        .read_u64((values[0]).wrapping_add((0x29u64).wrapping_mul(8)))?)
                        as f64)
                        .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 123;
            }
            125 => {
                {
                    let destination = relocate("STACK_local_1a0");
                    let stored = ((memory
                        .read_u64((values[0]).wrapping_add((0x2au64).wrapping_mul(8)))?)
                        as f64)
                        .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 124;
            }
            126 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 125;
            }
            127 => {
                values[2] = (values[0]).wrapping_add((0x5au64).wrapping_mul(8));
                pc = 126;
            }
            128 => {
                {
                    let call_arguments = [
                        values[10],
                        (values[0]).wrapping_add((0x56u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            129 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    128
                } else {
                    127
                };
            }
            130 => {
                values[10] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_100"))?)?)
                            .wrapping_add(0x48u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 129;
            }
            131 => {
                {
                    let destination = (values[0]).wrapping_add((0x58u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 130;
            }
            132 => {
                {
                    let destination = (values[0]).wrapping_add((0x57u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 131;
            }
            133 => {
                {
                    let destination = (values[0]).wrapping_add((0x56u64).wrapping_mul(8));
                    let stored = u64::from((0x71c7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 132;
            }
            134 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                {
                    let destination = (values[0]).wrapping_add((0x2au64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 134;
            }
            136 => {
                {
                    let destination = (values[0]).wrapping_add((0x29u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let destination = (values[0]).wrapping_add((0x28u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let destination = (values[0]).wrapping_add((0x26u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let destination = (values[0]).wrapping_add((0x25u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = (values[0]).wrapping_add((0x24u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = (values[0]).wrapping_add((0x23u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 141;
            }
            143 => {
                {
                    let destination = (values[0]).wrapping_add((0x21u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_110"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = relocate("STACK_local_110");
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 143;
            }
            145 => {
                {
                    let destination = (values[0]).wrapping_add((0x22u64).wrapping_mul(8));
                    let stored = u64::from((values[9]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 144;
            }
            146 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [
                            (memory.read_u64(relocate("STACK_local_210"))?) & (0xffffffffu64),
                            (memory.read_u64(relocate("STACK_local_140"))?) & (0xffffffffu64),
                            u64::from(
                                ((u64::from(
                                    ((u64::from(
                                        (((((memory.read_u64(relocate("STACK_local_210"))?) as i32
                                            as i64
                                            as u64)
                                            .wrapping_mul(0x20u64))
                                            as i32 as i64
                                            as u64)
                                            .wrapping_add(0x1fu64))
                                            as u32,
                                    ) as u32)
                                        .wrapping_shr(0x3u64 as u32)
                                        as u64) as u32,
                                )) & (0x1ffffffcu64)) as u32,
                            ),
                            0xe200bu64,
                        ];
                        invoke("GdipCreateBitmapFromScan0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                values[11] = ((memory.read_u64(relocate("STACK_local_140"))?) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 147;
            }
            149 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                values[2] = (values[0]).wrapping_add((0x55u64).wrapping_mul(8));
                pc = 149;
            }
            151 => {
                {
                    let destination = (values[0]).wrapping_add((0x20u64).wrapping_mul(8));
                    let stored = relocate("Gdiplus::Bitmap::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 150;
            }
            152 => {
                {
                    let destination = (values[0]).wrapping_add((0x22u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 151;
            }
            153 => {
                {
                    let destination = (values[0]).wrapping_add((0x21u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1fu64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 153;
            }
            155 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_130"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    154
                } else {
                    153
                };
            }
            156 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = memory.read_u64(relocate("STACK_local_248"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = memory.read_u64(values[18])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x4fu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 157;
            }
            159 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x50u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 158;
            }
            160 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[15])?).wrapping_add(0x18u64))?,
                        values[15],
                        (values[0]).wrapping_add((0x4fu64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 159;
            }
            161 => {
                {
                    let destination = (values[0]).wrapping_add((0x50u64).wrapping_mul(8));
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 160;
            }
            162 => {
                values[15] = memory.read_u64(relocate("STACK_local_120"))?;
                pc = 161;
            }
            163 => {
                {
                    let destination = relocate("STACK_local_120");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x51u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 162;
            }
            164 => {
                {
                    let call_arguments =
                        [values[11], memory.read_u64(relocate("STACK_local_138"))?];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            165 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    164
                } else {
                    163
                };
            }
            166 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(memory.read_u64(values[18])?)?)?,
                        memory.read_u64(relocate("STACK_local_130"))?,
                        relocate("DAT_140111ad8"),
                        (values[0]).wrapping_add((0x51u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 165;
            }
            167 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = memory.read_u64(values[18])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 166;
            }
            168 => {
                {
                    let destination = (values[0]).wrapping_add((0x54u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let destination = (values[0]).wrapping_add((0x53u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_138"))?;
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 169;
            }
            171 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = (values[0]).wrapping_add((0x52u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                {
                    let destination = (values[0]).wrapping_add((0x51u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 171;
            }
            173 => {
                pc = if (u64::from(((memory.read_u64(values[18])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    172
                } else {
                    161
                };
            }
            174 => {
                values[15] = values[20];
                pc = 173;
            }
            175 => {
                {
                    let destination = (values[0]).wrapping_add((0x4fu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 174;
            }
            176 => {
                {
                    let destination = (values[0]).wrapping_add((0x1fu64).wrapping_mul(8));
                    let stored = memory.read_u64(values[18])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x4cu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            178 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    176
                };
            }
            179 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_100"))?)?)
                                .wrapping_add(0x40u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_100"))?,
                        values[18],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 178;
            }
            180 => {
                {
                    let destination = (values[0]).wrapping_add((0x4eu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 179;
            }
            181 => {
                {
                    let destination = (values[0]).wrapping_add((0x4du64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 180;
            }
            182 => {
                {
                    let destination = (values[0]).wrapping_add((0x4cu64).wrapping_mul(8));
                    let stored = u64::from((0x71b5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 181;
            }
            183 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 182;
            }
            184 => {
                {
                    let destination = values[18];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                values[18] = (values[0]).wrapping_add((0x4bu64).wrapping_mul(8));
                pc = 184;
            }
            186 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x48u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            187 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    186
                } else {
                    185
                };
            }
            188 => {
                values[11] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_100"))?)?)
                            .wrapping_add(0x30u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 187;
            }
            189 => {
                {
                    let destination = (values[0]).wrapping_add((0x4au64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 188;
            }
            190 => {
                {
                    let destination = (values[0]).wrapping_add((0x49u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 189;
            }
            191 => {
                {
                    let destination = (values[0]).wrapping_add((0x48u64).wrapping_mul(8));
                    let stored = u64::from((0x7191u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 190;
            }
            192 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 191;
            }
            193 => {
                {
                    let destination = (values[0]).wrapping_add((0x1eu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 192;
            }
            194 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_140")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((float_to_integer(
                            (f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32(relocate("STACK_local_ec"))?))
                                        as u32,
                                )) as u32,
                            )) as f64,
                            32,
                            true,
                        )?) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 193;
            }
            195 => {
                {
                    let destination = relocate("STACK_local_ec");
                    let stored = u64::from(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0xecu64))?)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 194;
            }
            196 => {
                {
                    let destination = relocate("STACK_local_210");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_210")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl(0x20u64 as u32))
                        | ((float_to_integer(
                            (f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32(relocate("STACK_local_f0"))?))
                                        as u32,
                                )) as u32,
                            )) as f64,
                            32,
                            true,
                        )?) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 195;
            }
            197 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored = u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x1du64).wrapping_mul(8)))?,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 196;
            }
            198 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x45u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            199 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    198
                } else {
                    197
                };
            }
            200 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_100"))?)?)
                                .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_100"))?,
                        (values[0]).wrapping_add((0x1du64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 199;
            }
            201 => {
                {
                    let destination = (values[0]).wrapping_add((0x47u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 200;
            }
            202 => {
                {
                    let destination = (values[0]).wrapping_add((0x46u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 201;
            }
            203 => {
                {
                    let destination = (values[0]).wrapping_add((0x45u64).wrapping_mul(8));
                    let stored = u64::from((0x71a3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 202;
            }
            204 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 203;
            }
            205 => {
                {
                    let destination = (values[0]).wrapping_add((0x1du64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 204;
            }
            206 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_228"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    43
                } else {
                    205
                };
            }
            207 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1cu64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 206;
            }
            208 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    207
                } else {
                    206
                };
            }
            209 => {
                {
                    let destination = relocate("STACK_local_228");
                    let stored = ({
                        let call_arguments = [
                            values[18],
                            values[7],
                            (values[0]).wrapping_add((0x44u64).wrapping_mul(8)),
                        ];
                        invoke("FUN_14003d0f0", &call_arguments, memory)
                    }) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 208;
            }
            210 => {
                values[7] = (values[0]).wrapping_add((0x14u64).wrapping_mul(8));
                pc = 209;
            }
            211 => {
                values[18] = relocate("DAT_00000038");
                pc = 210;
            }
            212 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                        values[4],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 210;
            }
            213 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    211
                } else {
                    212
                };
            }
            214 => {
                values[18] = (values[4]).wrapping_add((0x5u64).wrapping_mul(8));
                pc = 213;
            }
            215 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 214;
            }
            216 => {
                values[4] = memory.read_u64(
                    (memory.read_u64((values[15]).wrapping_add((0x298u64).wrapping_mul(1)))?)
                        .wrapping_add(0x998u64),
                )?;
                pc = 215;
            }
            217 => {
                values[15] =
                    memory.read_u64((values[16]).wrapping_add((0xe8u64).wrapping_mul(1)))?;
                pc = 216;
            }
            218 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    217
                } else {
                    216
                };
            }
            219 => {
                values[15] = values[20];
                pc = 218;
            }
            220 => {
                values[16] = {
                    let call_arguments = [values[15]];
                    invoke("CFrameWnd::GetActiveView", &call_arguments, memory)
                };
                pc = 219;
            }
            221 => {
                values[15] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[14])?).wrapping_add(0xf8u64))?,
                        values[14],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 220;
            }
            222 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    221
                } else {
                    220
                };
            }
            223 => {
                values[15] = values[20];
                pc = 222;
            }
            224 => {
                values[14] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 223;
            }
            225 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 34;
            }
            226 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_220"))?) != (0x0u64)))
                    != 0
                {
                    225
                } else {
                    34
                };
            }
            227 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 226;
            }
            228 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 227;
            }
            229 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    228
                } else {
                    227
                };
            }
            230 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x14u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 229;
            }
            231 => {
                {
                    let call_arguments = [values[19]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 230;
            }
            232 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_100"))?) != (0x0u64)))
                    != 0
                {
                    231
                } else {
                    230
                };
            }
            233 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = memory.read_u64(values[19])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 232;
            }
            234 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    224
                } else {
                    233
                };
            }
            235 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1bu64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 234;
            }
            236 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_1d0"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    235
                } else {
                    234
                };
            }
            237 => {
                {
                    let destination = relocate("STACK_local_1d0");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x1bu64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 236;
            }
            238 => {
                values[8] = ({
                    let call_arguments = [
                        values[19],
                        (values[0]).wrapping_add((0x1bu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 237;
            }
            239 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 238;
            }
            240 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 239;
            }
            241 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_1f8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    240
                } else {
                    239
                };
            }
            242 => {
                {
                    let destination = relocate("STACK_local_1f8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x15u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 241;
            }
            243 => {
                pc = if (u64::from(((values[13]) as i64) == ((0x1u64) as i64))) != 0 {
                    248
                } else {
                    242
                };
            }
            244 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 243;
            }
            245 => {
                {
                    let destination = values[18];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 244;
            }
            246 => {
                values[13] = memory.read_u64(values[18])?;
                pc = 245;
            }
            247 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 246;
            }
            248 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 247;
            }
            249 => {
                {
                    let destination = relocate("STACK_local_1f0");
                    let stored = values[18];
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            250 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 249;
            }
            251 => {
                {
                    let destination = values[18];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 250;
            }
            252 => {
                values[13] = memory.read_u64(values[18])?;
                pc = 251;
            }
            253 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 252;
            }
            254 => {
                values[18] =
                    memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?;
                pc = 253;
            }
            255 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    254
                } else {
                    242
                };
            }
            256 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x41u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            257 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    256
                } else {
                    255
                };
            }
            258 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[18])?).wrapping_add(0x40u64))?,
                        values[18],
                        values[19],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 257;
            }
            259 => {
                {
                    let destination = (values[0]).wrapping_add((0x43u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 258;
            }
            260 => {
                {
                    let destination = (values[0]).wrapping_add((0x42u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 259;
            }
            261 => {
                {
                    let destination = (values[0]).wrapping_add((0x41u64).wrapping_mul(8));
                    let stored = u64::from((0x16fu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 260;
            }
            262 => {
                values[18] = memory.read_u64(memory.read_u64(relocate("STACK_local_1e8"))?)?;
                pc = 261;
            }
            263 => {
                {
                    let destination = relocate("STACK_local_1e8");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 262;
            }
            264 => {
                {
                    let destination = (values[0]).wrapping_add((0x33u64).wrapping_mul(8));
                    let stored = u64::from((0x17u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 263;
            }
            265 => {
                {
                    let destination = values[19];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 264;
            }
            266 => {
                values[19] = (values[0]).wrapping_add((0x1au64).wrapping_mul(8));
                pc = 265;
            }
            267 => {
                {
                    let call_arguments = [relocate("STACK_local_180"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            268 => {
                {
                    let call_arguments = [relocate("STACK_local_180")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 267;
            }
            269 => {
                {
                    let destination = (values[0]).wrapping_add((0x40u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 268;
            }
            270 => {
                {
                    let destination = (values[0]).wrapping_add((0x3fu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 269;
            }
            271 => {
                {
                    let destination = (values[0]).wrapping_add((0x3eu64).wrapping_mul(8));
                    let stored = u64::from((0x1404u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 270;
            }
            272 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_1e0"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    271
                } else {
                    266
                };
            }
            273 => {
                {
                    let destination = relocate("STACK_local_1e0");
                    let stored = ((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0x18u64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 272;
            }
            274 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            275 => {
                {
                    let destination = relocate("STACK_local_1dc");
                    let stored = (u64::from(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0xc4u64))?)) as u32,
                    )) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 274;
            }
            276 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_1dc"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    275
                } else {
                    273
                };
            }
            277 => {
                {
                    let destination = relocate("STACK_local_1dc");
                    let stored = ((u64::from(memory.read_u32((values[0]).wrapping_add(0xc4u64))?))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 276;
            }
            278 => {
                {
                    let destination = (values[0]).wrapping_add((0x3du64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 277;
            }
            279 => {
                {
                    let destination = (values[0]).wrapping_add((0x3cu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 278;
            }
            280 => {
                {
                    let destination = (values[0]).wrapping_add((0x3bu64).wrapping_mul(8));
                    let stored = u64::from((0x146bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 279;
            }
            281 => {
                values[20] = 0x0u64;
                pc = 280;
            }
            282 => {
                pc = 3;
            }
            283 => {
                pc = 281;
            }
            284 => {
                return Ok(None);
            }
            285 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    284
                } else {
                    283
                };
            }
            286 => {
                values[8] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add((0x16u64).wrapping_mul(8)),
                        values[0],
                    ];
                    invoke("FUN_140055b40", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 285;
            }
            287 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_200"))?;
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 286;
            }
            288 => {
                {
                    let destination = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 287;
            }
            289 => {
                {
                    let destination = (values[0]).wrapping_add((0x18u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 288;
            }
            290 => {
                {
                    let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x15u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 289;
            }
            291 => {
                {
                    let destination = (values[0]).wrapping_add((0x16u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 290;
            }
            292 => {
                {
                    let call_arguments = [relocate("STACK_local_198"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            293 => {
                {
                    let call_arguments = [relocate("STACK_local_198")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 292;
            }
            294 => {
                {
                    let destination = (values[0]).wrapping_add((0x3au64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 293;
            }
            295 => {
                {
                    let destination = (values[0]).wrapping_add((0x39u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 294;
            }
            296 => {
                {
                    let destination = (values[0]).wrapping_add((0x38u64).wrapping_mul(8));
                    let stored = u64::from((0x1628u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 295;
            }
            297 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    296
                } else {
                    291
                };
            }
            298 => {
                {
                    let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_128"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 297;
            }
            299 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x34u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 298;
            }
            300 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x35u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            301 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    300
                } else {
                    299
                };
            }
            302 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_220"))?)?)
                                .wrapping_add(0x1b8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_220"))?,
                        (values[0]).wrapping_add((0x34u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 301;
            }
            303 => {
                {
                    let destination = (values[0]).wrapping_add((0x37u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 302;
            }
            304 => {
                {
                    let destination = (values[0]).wrapping_add((0x36u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 303;
            }
            305 => {
                {
                    let destination = (values[0]).wrapping_add((0x35u64).wrapping_mul(8));
                    let stored = u64::from((0x716du64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 304;
            }
            306 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 305;
            }
            307 => {
                {
                    let destination = (values[0]).wrapping_add((0x34u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 306;
            }
            308 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x13u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 307;
            }
            309 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    308
                } else {
                    307
                };
            }
            310 => {
                {
                    let call_arguments = [
                        values[18],
                        (values[0]).wrapping_add((0x14u64).wrapping_mul(8)),
                        (memory.read_u64(values[19])?) as i32 as i64 as u64,
                    ];
                    invoke("FUN_1400072f0", &call_arguments, memory)
                };
                pc = 309;
            }
            311 => {
                values[18] = relocate("DAT_00000038");
                pc = 310;
            }
            312 => {
                values[19] = relocate("DAT_00000048");
                pc = 311;
            }
            313 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                        values[4],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 310;
            }
            314 => {
                pc = if (u64::from((values[4]) == (0x0u64))) != 0 {
                    312
                } else {
                    313
                };
            }
            315 => {
                values[19] = (values[4]).wrapping_add((0x7u64).wrapping_mul(8));
                pc = 314;
            }
            316 => {
                values[18] = (values[4]).wrapping_add((0x5u64).wrapping_mul(8));
                pc = 315;
            }
            317 => {
                {
                    let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 316;
            }
            318 => {
                values[4] = memory.read_u64(
                    (memory.read_u64((values[20]).wrapping_add((0x298u64).wrapping_mul(1)))?)
                        .wrapping_add(0x998u64),
                )?;
                pc = 317;
            }
            319 => {
                values[20] =
                    memory.read_u64((values[16]).wrapping_add((0xe8u64).wrapping_mul(1)))?;
                pc = 318;
            }
            320 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    319
                } else {
                    318
                };
            }
            321 => {
                values[16] = {
                    let call_arguments = [values[15]];
                    invoke("CFrameWnd::GetActiveView", &call_arguments, memory)
                };
                pc = 320;
            }
            322 => {
                values[15] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[14])?).wrapping_add(0xf8u64))?,
                        values[14],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 321;
            }
            323 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    322
                } else {
                    321
                };
            }
            324 => {
                values[15] = values[20];
                pc = 323;
            }
            325 => {
                values[14] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 324;
            }
            326 => {
                {
                    let call_arguments = [(values[13]).wrapping_add(0x1e0u64), 0x0u64];
                    invoke("FUN_1400a3ec0", &call_arguments, memory)
                };
                pc = 325;
            }
            327 => {
                pc = if (u64::from(
                    ((u64::from((values[14]) != (0x0u64))) != 0)
                        && (({
                            let _ = {
                                values[13] = {
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(values[14])?).wrapping_add(0xf8u64),
                                        )?,
                                        values[14],
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                };
                                values[13]
                            };
                            u64::from(((values[13]) as i64) != ((0x0u64) as i64))
                        }) != 0),
                )) != 0
                {
                    326
                } else {
                    325
                };
            }
            328 => {
                values[14] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 327;
            }
            329 => {
                {
                    let call_arguments = [
                        0x1u64,
                        u64::from(
                            ((((u64::from(memory.read_u8(relocate("STACK_local_108"))?)) as i8
                                as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        ),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceOperationLogger::SendTextOperation",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 328;
            }
            330 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = ((u64::from(
                        memory.read_u8((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?,
                    )) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 329;
            }
            331 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x30u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            332 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    331
                } else {
                    330
                };
            }
            333 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_220"))?)?)
                                .wrapping_add(0xd8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_220"))?,
                        (values[0]).wrapping_add((0x2fu64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 332;
            }
            334 => {
                {
                    let destination = (values[0]).wrapping_add((0x32u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 333;
            }
            335 => {
                {
                    let destination = (values[0]).wrapping_add((0x31u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 334;
            }
            336 => {
                {
                    let destination = (values[0]).wrapping_add((0x30u64).wrapping_mul(8));
                    let stored = u64::from((0x6f9fu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 335;
            }
            337 => {
                {
                    let destination = relocate("STACK_local_220");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 336;
            }
            338 => {
                {
                    let destination = (values[0]).wrapping_add((0x2fu64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 337;
            }
            339 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x12u64).wrapping_mul(8))];
                    invoke("FUN_14004e8c0", &call_arguments, memory)
                };
                pc = 338;
            }
            340 => {
                {
                    let destination = (values[0]).wrapping_add((0x33u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 339;
            }
            341 => {
                values[20] = 0x0u64;
                pc = 340;
            }
            342 => {
                return Ok(None);
            }
            343 => {
                {
                    let call_arguments = [values[6]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 342;
            }
            344 => {
                values[6] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 343;
            }
            345 => {
                pc = if (u64::from(
                    (((values[22]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    344
                };
            }
            346 => {
                pc = if (u64::from(
                    (((values[22]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    345
                };
            }
            347 => {
                pc = if (u64::from(
                    (((values[22]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    346
                };
            }
            348 => {
                pc = if (u64::from(
                    (((values[22]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    281
                } else {
                    347
                };
            }
            349 => {
                pc = if (u64::from(
                    (((values[22]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    282
                } else {
                    348
                };
            }
            350 => {
                pc = if (u64::from(
                    (((values[22]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    341
                } else {
                    349
                };
            }
            351 => {
                values[22] =
                    (u64::from((u64::from(memory.read_u16(relocate("STACK_local_218"))?)) as u16))
                        as i32 as i64 as u64;
                pc = 350;
            }
            352 => {
                {
                    let destination = relocate("STACK_local_208");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 351;
            }
            353 => {
                {
                    let destination = relocate("STACK_local_218");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory.read_u16(memory.read_u64(relocate("STACK_local_200"))?)?,
                            )) as u16,
                        )) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 352;
            }
            354 => {
                {
                    let destination = relocate("STACK_local_200");
                    let stored = (values[0]).wrapping_add((0x11u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 353;
            }
            355 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_268"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 354;
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
    ("FUN_140007750", 0, fun_140007750),
    ("FUN_140007940", 1, fun_140007940),
    ("FUN_140007c00", 1, fun_140007c00),
    ("FUN_1400160b0", 3, fun_1400160b0),
    ("FUN_140017480", 4, fun_140017480),
    ("FUN_14003fcf0", 2, fun_14003fcf0),
    ("FUN_1400517d0", 2, fun_1400517d0),
    ("FUN_140053840", 2, fun_140053840),
    ("FUN_1400a51f0", 2, fun_1400a51f0),
    ("FUN_1400c0dc0", 1, fun_1400c0dc0),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_140007750",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140007940",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_140007c00",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
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
        "FUN_1400160b0",
        &[
            ("STACK_auStack_118", -280),
            ("STACK_local_f8", -248),
            ("STACK_local_e8", -232),
            ("STACK_local_d8", -216),
            ("STACK_uStack_d4", -212),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_c0", -192),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_a8", -168),
            ("STACK_uStack_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_uStack_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_uStack_60", -96),
            ("STACK_local_58", -88),
        ],
    ),
    (
        "FUN_140017480",
        &[
            ("STACK_auStack_b8", -184),
            ("STACK_local_98", -152),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_uStack_74", -116),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_plStack_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_uStack_4c", -76),
            ("STACK_local_48", -72),
        ],
    ),
    (
        "FUN_14003fcf0",
        &[
            ("STACK_auStack_c8", -200),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_plStack_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
        ],
    ),
    (
        "FUN_1400517d0",
        &[
            ("STACK_auStack_b8", -184),
            ("STACK_local_98", -152),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_fStack_44", -68),
            ("STACK_local_40", -64),
        ],
    ),
    (
        "FUN_140053840",
        &[
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400c0dc0",
        &[
            ("STACK_auStack_268", -616),
            ("STACK_local_248", -584),
            ("STACK_local_240", -576),
            ("STACK_local_238", -568),
            ("STACK_local_228", -552),
            ("STACK_local_220", -544),
            ("STACK_local_218", -536),
            ("STACK_local_210", -528),
            ("STACK_local_208", -520),
            ("STACK_local_200", -512),
            ("STACK_local_1f8", -504),
            ("STACK_local_1f0", -496),
            ("STACK_local_1e8", -488),
            ("STACK_local_1e0", -480),
            ("STACK_local_1dc", -476),
            ("STACK_local_1d0", -464),
            ("STACK_local_1c8", -456),
            ("STACK_local_1c0", -448),
            ("STACK_local_1b8", -440),
            ("STACK_local_1b0", -432),
            ("STACK_local_1a8", -424),
            ("STACK_local_1a0", -416),
            ("STACK_local_198", -408),
            ("STACK_local_180", -384),
            ("STACK_local_168", -360),
            ("STACK_local_150", -336),
            ("STACK_local_140", -320),
            ("STACK_local_138", -312),
            ("STACK_local_130", -304),
            ("STACK_local_128", -296),
            ("STACK_local_120", -288),
            ("STACK_local_118", -280),
            ("STACK_local_110", -272),
            ("STACK_local_108", -264),
            ("STACK_local_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
            ("STACK_local_ec", -236),
            ("STACK_local_e8", -232),
            ("STACK_local_e4", -228),
            ("STACK_local_50", -80),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_140007750",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140007940",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_140007c00",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
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
        "FUN_1400160b0",
        &[
            ("STACK_auStack_118", 32),
            ("STACK_local_f8", 8),
            ("STACK_local_e8", 16),
            ("STACK_local_d8", 4),
            ("STACK_uStack_d4", 4),
            ("STACK_local_d0", 1),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 8),
            ("STACK_uStack_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_uStack_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_uStack_60", 8),
            ("STACK_local_58", 8),
        ],
    ),
    (
        "FUN_140017480",
        &[
            ("STACK_auStack_b8", 32),
            ("STACK_local_98", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 4),
            ("STACK_uStack_74", 4),
            ("STACK_local_70", 1),
            ("STACK_local_68", 8),
            ("STACK_plStack_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 4),
            ("STACK_uStack_4c", 4),
            ("STACK_local_48", 8),
        ],
    ),
    (
        "FUN_14003fcf0",
        &[
            ("STACK_auStack_c8", 32),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_plStack_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
        ],
    ),
    (
        "FUN_1400517d0",
        &[
            ("STACK_auStack_b8", 32),
            ("STACK_local_98", 4),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 4),
            ("STACK_fStack_44", 4),
            ("STACK_local_40", 8),
        ],
    ),
    (
        "FUN_140053840",
        &[
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400c0dc0",
        &[
            ("STACK_auStack_268", 32),
            ("STACK_local_248", 8),
            ("STACK_local_240", 8),
            ("STACK_local_238", 8),
            ("STACK_local_228", 1),
            ("STACK_local_220", 8),
            ("STACK_local_218", 2),
            ("STACK_local_210", 8),
            ("STACK_local_208", 8),
            ("STACK_local_200", 8),
            ("STACK_local_1f8", 8),
            ("STACK_local_1f0", 8),
            ("STACK_local_1e8", 8),
            ("STACK_local_1e0", 4),
            ("STACK_local_1dc", 4),
            ("STACK_local_1d0", 8),
            ("STACK_local_1c8", 8),
            ("STACK_local_1c0", 8),
            ("STACK_local_1b8", 8),
            ("STACK_local_1b0", 8),
            ("STACK_local_1a8", 8),
            ("STACK_local_1a0", 8),
            ("STACK_local_198", 24),
            ("STACK_local_180", 24),
            ("STACK_local_168", 8),
            ("STACK_local_150", 8),
            ("STACK_local_140", 8),
            ("STACK_local_138", 8),
            ("STACK_local_130", 8),
            ("STACK_local_128", 8),
            ("STACK_local_120", 8),
            ("STACK_local_118", 8),
            ("STACK_local_110", 8),
            ("STACK_local_108", 1),
            ("STACK_local_100", 8),
            ("STACK_local_f8", 8),
            ("STACK_local_f0", 4),
            ("STACK_local_ec", 4),
            ("STACK_local_e8", 4),
            ("STACK_local_e4", 4),
            ("STACK_local_50", 8),
        ],
    ),
];
