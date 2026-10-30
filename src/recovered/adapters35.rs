//! Checked translations generated from original bodies by batch 035.
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
/// Translation of `FUN_140027f70` (`mspaint.c:38581`).
pub fn fun_140027f70(
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
    ];
    let mut pc: i32 = 204;
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
                        || ((values[25]) != 0),
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
                        .wrapping_add(values[14])) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        != (((values[23]) as i32 as i64 as u64) as i64),
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
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                values[17] = (values[17]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                );
                pc = 26;
            }
            28 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[13]) as i32 as i64 as u64) as i64)
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
                        values[11] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?))
                            as i32 as i64 as u64) as i32 as i64
                            as u64;
                        values[11]
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
                    let stored = (values[14]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                values[14] =
                    (((values[14]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
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
                            (values[17]) == (memory.read_u64(relocate("STACK_local_618"))?),
                        )) != 0)
                            && ((u64::from(
                                ((u64::from(
                                    (f64::from_bits((values[2]) & (0x7fffffffffffffffu64)))
                                        <= (f64::from_bits(0x3ff0000000000000u64)),
                                )) != 0)
                                    && ((u64::from(
                                        (f64::from_bits((values[2]) & (0x7fffffffffffffffu64)))
                                            != (f64::from_bits(0x3ff0000000000000u64)),
                                    )) != 0),
                            )) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    let destination = relocate("STACK_local_5f0");
                                    let stored = (values[14]) as i32 as i64 as u64;
                                    memory.write_u32(destination, stored as u32)?;
                                    stored
                                };
                                {
                                    values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                                    values[17]
                                }
                            };
                            u64::from((f64::from_bits(values[2])) != (f64::from_bits(0x0u64)))
                        }) != 0),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            38 => {
                values[11] = (values[14]) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                values[14] = (((((values[13]).wrapping_sub(values[19])) as i32 as i64 as u64)
                    .wrapping_add(values[12])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 38;
            }
            40 => {
                values[19] = (((values[19])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    39
                };
            }
            42 => {
                values[19] = ((((values[17]) as i32 as i64 as u64).wrapping_sub(
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
                values[11] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                values[14] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    (((values[24]) as i32 as i64 as u64) as i64)
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
                    (((values[24]) as i32 as i64 as u64) as i64)
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
                    (((values[24]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    28
                };
            }
            50 => {
                values[11] = (values[13]) as i32 as i64 as u64;
                pc = 49;
            }
            51 => {
                values[14] = (values[13]) as i32 as i64 as u64;
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_5ec");
                    let stored = (values[19]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                values[18] = ((u64::from(memory.read_u8(relocate("STACK_local_5f7"))?)) as i8 as i64
                    as u64) as i8 as i64 as u64;
                pc = 52;
            }
            54 => {
                values[17] = memory.read_u64(relocate("STACK_local_620"))?;
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(values[19])) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                pc = if (u64::from((values[21]) < (values[20]))) != 0 {
                    61
                } else {
                    55
                };
            }
            57 => {
                values[21] = (u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64;
                pc = 56;
            }
            58 => {
                values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    ((values[17]).wrapping_add((0x1u64).wrapping_mul(1))) != (values[5]),
                )) != 0
                {
                    58
                } else {
                    57
                };
            }
            60 => {
                values[19] =
                    (((values[19]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                values[20] = (values[20]).wrapping_sub((values[21]) as i8 as i64 as u64);
                pc = 60;
            }
            62 => {
                values[5] = (values[17])
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
                                values[21] =
                                    (u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64;
                                values[21]
                            };
                            u64::from((values[21]) < (values[20]))
                        }) != 0),
                )) != 0
                {
                    62
                } else {
                    55
                };
            }
            64 => {
                values[20] = (memory.read_u64(relocate("STACK_local_608"))?)
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
                    ((u64::from((memory.read_u64(relocate("STACK_p_Stack_6e0"))?) != (0x0u64)))
                        != 0)
                        && (({
                            let _ = {
                                values[16] = {
                                    let call_arguments = [memory.read_u64(
                                        (memory.read_u64(
                                            memory.read_u64(relocate("STACK_p_Stack_6e0"))?,
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
                    let destination = relocate("STACK_p_Stack_6e0");
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
                        (memory.read_u64(memory.read_u64(relocate("STACK_p_Stack_6e0"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 83;
            }
            86 => {
                {
                    let destination = relocate("STACK_p_Stack_6e0");
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
                values[18] = (0x1u64) as i8 as i64 as u64;
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
                            (values[5]) == (memory.read_u64(relocate("STACK_local_6e8"))?),
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
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                pc = if (u64::from((values[17]) < (values[5]))) != 0 {
                    93
                } else {
                    92
                };
            }
            95 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                pc = if (u64::from((values[9]) < (memory.read_u64(relocate("STACK_local_6e8"))?)))
                    != 0
                {
                    104
                } else {
                    95
                };
            }
            97 => {
                let _ = {
                    values[9] = (values[9]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[9]
                };
                pc = 96;
            }
            98 => {
                values[5] = values[22];
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_local_600");
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                values[17] = values[9];
                pc = 100;
            }
            102 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(values[9])?)) as i8 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x2eu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    values[22] = values[5];
                                    values[22]
                                };
                                {
                                    values[10] = memory.read_u64(relocate("STACK_local_600"))?;
                                    values[10]
                                }
                            };
                            u64::from(
                                ((((u64::from(memory.read_u8(values[9])?)) as i8 as i64 as u64)
                                    as i32 as i64 as u64) as i64)
                                    == (((values[23]) as i32 as i64 as u64) as i64),
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
                values[10] = values[9];
                pc = 102;
            }
            104 => {
                values[22] = values[9];
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
                values[9] = memory.read_u64(relocate("STACK_local_628"))?;
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 106;
            }
            108 => {
                values[5] = memory.read_u64(relocate("STACK_local_6e8"))?;
                pc = 107;
            }
            109 => {
                values[17] = memory.read_u64(relocate("STACK_local_6e8"))?;
                pc = 108;
            }
            110 => {
                pc = 24;
            }
            111 => {
                pc = if (values[25]) != 0 { 110 } else { 109 };
            }
            112 => {
                values[11] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 111;
            }
            113 => {
                {
                    let destination = relocate("STACK_local_600");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("STACK_local_618");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                values[14] = (0x0u64) as i32 as i64 as u64;
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_5e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("STACK_uStack_5d0");
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = relocate("STACK_local_5d8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_uStack_5e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let destination = relocate("STACK_local_5ec");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let destination = relocate("STACK_local_5f7");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 123;
            }
            125 => {
                values[25] = u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((float_to_integer(f64::from_bits(values[2]), 64, false)?)
                                        .wrapping_shr((0x34u64) as u32))
                                        as u32,
                                )) & (0x7ffu64)) as u32,
                            )) as u32,
                        )) == (u64::from((0x7ffu64) as u32)),
                    )) != 0,
                );
                pc = 124;
            }
            126 => {
                values[23] = (((values[23])
                    .wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 125;
            }
            127 => {
                pc = if (u64::from((values[17]) != (memory.read_u64(relocate("STACK_local_6e8"))?)))
                    != 0
                {
                    131
                } else {
                    126
                };
            }
            128 => {
                let _ = {
                    values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[17]
                };
                pc = 127;
            }
            129 => {
                {
                    let destination = values[17];
                    let stored = (((values[6])
                        .wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 128;
            }
            130 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((0x60u64) as i32 as i64 as u64) as i64)
                            < (((values[6]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (((values[6]) as i32 as i64 as u64) as i64)
                                < (((0x7bu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    129
                } else {
                    128
                };
            }
            131 => {
                values[6] = ((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i8
                    as i64 as u64;
                pc = 130;
            }
            132 => {
                pc = if (values[8]) != 0 { 127 } else { 125 };
            }
            133 => {
                values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                pc = 132;
            }
            134 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 133;
            }
            135 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    134
                } else {
                    133
                };
            }
            136 => {
                {
                    let destination = relocate("STACK_local_628");
                    let stored = relocate("STACK_acStack_5c7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            137 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f6"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    135
                } else {
                    136
                };
            }
            138 => {
                values[18] = (0x0u64) as i8 as i64 as u64;
                pc = 137;
            }
            139 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((memory.read_u64(relocate("STACK_local_6e8"))?) as i32 as i64
                        as u64)
                        .wrapping_sub(
                            (memory.read_u64(relocate("STACK_local_628"))?) as i32 as i64 as u64,
                        )) as i32 as i64 as u64) as i32 as i64
                        as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let destination = relocate("STACK_local_628");
                    let stored = relocate("STACK_local_5c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x3u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = (0x6eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 141;
            }
            143 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (0x61u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x6eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = relocate("STACK_local_5c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    145
                } else {
                    144
                };
            }
            147 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = relocate("STACK_acStack_5c7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_p_Stack_610");
                    let stored = memory.read_u64(relocate("STACK_p_Stack_6e0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            149 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14002eb40", &call_arguments, memory)
                };
                pc = 148;
            }
            150 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14002da40", &call_arguments, memory)
                };
                pc = 148;
            }
            151 => {
                pc = if (values[25]) != 0 { 149 } else { 150 };
            }
            152 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_6e8"),
                        relocate("STACK_local_5c8"),
                        relocate("STACK_local_5f"),
                        u64::from(
                            ((float_to_integer(f64::from_bits(values[2]), 64, false)?)
                                .wrapping_shr(
                                    (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                                )) as u32,
                        ),
                    ];
                    invoke("FUN_14002d860", &call_arguments, memory)
                };
                pc = 148;
            }
            153 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    151
                } else {
                    152
                };
            }
            154 => {
                pc = if (u64::from(
                    (({
                        let call_arguments = [values[2]];
                        invoke("NAN", &call_arguments, memory)
                    }) != 0)
                        || (({
                            let call_arguments = [values[2]];
                            invoke("NAN", &call_arguments, memory)
                        }) != 0),
                )) != 0
                {
                    147
                } else {
                    153
                };
            }
            155 => {
                {
                    let destination = relocate("STACK_local_5f6");
                    let stored = (u64::from(
                        (((values[14]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 154;
            }
            156 => {
                values[14] = ({
                    let call_arguments = [u64::from(
                        ((float_to_integer(f64::from_bits(values[2]), 64, false)?).wrapping_shr(
                            (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                        )) as u32,
                    )];
                    invoke("_dsign", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_6d8");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                values[13] = (values[14]) as i32 as i64 as u64;
                pc = 157;
            }
            159 => {
                values[12] = (0x432u64) as i32 as i64 as u64;
                pc = 158;
            }
            160 => {
                pc = if (u64::from(
                    (((0x432u64) as i32 as i64 as u64) as i64)
                        < (((values[12]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    159
                } else {
                    157
                };
            }
            161 => {
                values[13] = (0x0u64) as i32 as i64 as u64;
                pc = 160;
            }
            162 => {
                values[14] = (((values[12])
                    .wrapping_add((0u64.wrapping_sub(0x432u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 161;
            }
            163 => {
                values[12] = (0x6u64) as i32 as i64 as u64;
                pc = 162;
            }
            164 => {
                values[12] = (0x1u64) as i32 as i64 as u64;
                pc = 162;
            }
            165 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    164
                } else {
                    162
                };
            }
            166 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    163
                } else {
                    165
                };
            }
            167 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 166;
            }
            168 => {
                pc = 162;
            }
            169 => {
                values[24] = (0x2u64) as i32 as i64 as u64;
                pc = 168;
            }
            170 => {
                values[12] = (0x6u64) as i32 as i64 as u64;
                pc = 169;
            }
            171 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    170
                } else {
                    169
                };
            }
            172 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 171;
            }
            173 => {
                pc = 162;
            }
            174 => {
                values[24] = (0x1u64) as i32 as i64 as u64;
                pc = 173;
            }
            175 => {
                values[12] = (0x6u64) as i32 as i64 as u64;
                pc = 174;
            }
            176 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    175
                } else {
                    174
                };
            }
            177 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 176;
            }
            178 => {
                pc = 162;
            }
            179 => {
                values[25] = u64::from((values[7]) != 0);
                pc = 178;
            }
            180 => {
                pc = 162;
            }
            181 => {
                values[23] = (0x70u64) as i8 as i64 as u64;
                pc = 180;
            }
            182 => {
                values[24] = (0x4u64) as i32 as i64 as u64;
                pc = 181;
            }
            183 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 182;
            }
            184 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x67u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    166
                } else {
                    179
                };
            }
            185 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x47u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    167
                } else {
                    184
                };
            }
            186 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x66u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    171
                } else {
                    185
                };
            }
            187 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x46u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    172
                } else {
                    186
                };
            }
            188 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x65u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    176
                } else {
                    187
                };
            }
            189 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x45u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    188
                };
            }
            190 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x61u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    182
                } else {
                    189
                };
            }
            191 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x41u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    183
                } else {
                    190
                };
            }
            192 => {
                values[26] = (u64::from(
                    (u64::from(memory.read_u8((values[3]).wrapping_add(0x8u64))?)) as u8,
                )) as i32 as i64 as u64;
                pc = 191;
            }
            193 => {
                values[7] = 1u64;
                pc = 192;
            }
            194 => {
                values[25] = 0u64;
                pc = 193;
            }
            195 => {
                values[12] = ((u64::from(memory.read_u32((values[3]).wrapping_add(0x4u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 194;
            }
            196 => {
                values[23] = (0x65u64) as i8 as i64 as u64;
                pc = 195;
            }
            197 => {
                values[24] = (0x3u64) as i32 as i64 as u64;
                pc = 196;
            }
            198 => {
                values[8] = 0u64;
                pc = 197;
            }
            199 => {
                {
                    let destination = relocate("STACK_local_5f8");
                    let stored = (0x2u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 198;
            }
            200 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    199
                } else {
                    198
                };
            }
            201 => {
                {
                    let destination = relocate("STACK_local_5f8");
                    let stored = ((u64::from(memory.read_u8((values[3]).wrapping_add(0xau64))?))
                        as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 200;
            }
            202 => {
                values[19] = (0x0u64) as i32 as i64 as u64;
                pc = 201;
            }
            203 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_728"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 202;
            }
            204 => {
                {
                    let destination = relocate("STACK_PARAM_param_5");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 203;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400285f0` (`mspaint.c:38922`).
pub fn fun_1400285f0(
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
    ];
    let mut pc: i32 = 204;
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
                        || ((values[25]) != 0),
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
                        .wrapping_add(values[14])) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (values[11]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i32 as i64
                        as u64) as i64)
                        != (((values[23]) as i32 as i64 as u64) as i64),
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
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                values[17] = (values[17]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                );
                pc = 26;
            }
            28 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[13]) as i32 as i64 as u64) as i64)
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
                        values[11] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?))
                            as i32 as i64 as u64) as i32 as i64
                            as u64;
                        values[11]
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
                    let stored = (values[14]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                values[14] =
                    (((values[14]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
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
                            (values[17]) == (memory.read_u64(relocate("STACK_local_618"))?),
                        )) != 0)
                            && ((u64::from(
                                ((u64::from(
                                    (f64::from_bits((values[2]) & (0x7fffffffffffffffu64)))
                                        <= (f64::from_bits(0x3ff0000000000000u64)),
                                )) != 0)
                                    && ((u64::from(
                                        (f64::from_bits((values[2]) & (0x7fffffffffffffffu64)))
                                            != (f64::from_bits(0x3ff0000000000000u64)),
                                    )) != 0),
                            )) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    let destination = relocate("STACK_local_5f0");
                                    let stored = (values[14]) as i32 as i64 as u64;
                                    memory.write_u32(destination, stored as u32)?;
                                    stored
                                };
                                {
                                    values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                                    values[17]
                                }
                            };
                            u64::from((f64::from_bits(values[2])) != (f64::from_bits(0x0u64)))
                        }) != 0),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            38 => {
                values[11] = (values[14]) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                values[14] = (((((values[13]).wrapping_sub(values[19])) as i32 as i64 as u64)
                    .wrapping_add(values[12])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 38;
            }
            40 => {
                values[19] = (((values[19])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                pc = if (u64::from(
                    (((values[18]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    39
                };
            }
            42 => {
                values[19] = ((((values[17]) as i32 as i64 as u64).wrapping_sub(
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
                values[11] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                values[14] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    (((values[24]) as i32 as i64 as u64) as i64)
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
                    (((values[24]) as i32 as i64 as u64) as i64)
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
                    (((values[24]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    28
                };
            }
            50 => {
                values[11] = (values[13]) as i32 as i64 as u64;
                pc = 49;
            }
            51 => {
                values[14] = (values[13]) as i32 as i64 as u64;
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_5ec");
                    let stored = (values[19]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                values[18] = ((u64::from(memory.read_u8(relocate("STACK_local_5f7"))?)) as i8 as i64
                    as u64) as i8 as i64 as u64;
                pc = 52;
            }
            54 => {
                values[17] = memory.read_u64(relocate("STACK_local_620"))?;
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(values[19])) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 54;
            }
            56 => {
                pc = if (u64::from((values[21]) < (values[20]))) != 0 {
                    61
                } else {
                    55
                };
            }
            57 => {
                values[21] = (u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64;
                pc = 56;
            }
            58 => {
                values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    ((values[17]).wrapping_add((0x1u64).wrapping_mul(1))) != (values[5]),
                )) != 0
                {
                    58
                } else {
                    57
                };
            }
            60 => {
                values[19] =
                    (((values[19]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                values[20] = (values[20]).wrapping_sub((values[21]) as i8 as i64 as u64);
                pc = 60;
            }
            62 => {
                values[5] = (values[17])
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
                                values[21] =
                                    (u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64;
                                values[21]
                            };
                            u64::from((values[21]) < (values[20]))
                        }) != 0),
                )) != 0
                {
                    62
                } else {
                    55
                };
            }
            64 => {
                values[20] = (memory.read_u64(relocate("STACK_local_608"))?)
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
                    ((u64::from((memory.read_u64(relocate("STACK_p_Stack_6e0"))?) != (0x0u64)))
                        != 0)
                        && (({
                            let _ = {
                                values[16] = {
                                    let call_arguments = [memory.read_u64(
                                        (memory.read_u64(
                                            memory.read_u64(relocate("STACK_p_Stack_6e0"))?,
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
                    let destination = relocate("STACK_p_Stack_6e0");
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
                        (memory.read_u64(memory.read_u64(relocate("STACK_p_Stack_6e0"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 83;
            }
            86 => {
                {
                    let destination = relocate("STACK_p_Stack_6e0");
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
                values[18] = (0x1u64) as i8 as i64 as u64;
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
                            (values[5]) == (memory.read_u64(relocate("STACK_local_6e8"))?),
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
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                pc = if (u64::from((values[17]) < (values[5]))) != 0 {
                    93
                } else {
                    92
                };
            }
            95 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                pc = if (u64::from((values[9]) < (memory.read_u64(relocate("STACK_local_6e8"))?)))
                    != 0
                {
                    104
                } else {
                    95
                };
            }
            97 => {
                let _ = {
                    values[9] = (values[9]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[9]
                };
                pc = 96;
            }
            98 => {
                values[5] = values[22];
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_local_600");
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                values[17] = values[9];
                pc = 100;
            }
            102 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(values[9])?)) as i8 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x2eu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    values[22] = values[5];
                                    values[22]
                                };
                                {
                                    values[10] = memory.read_u64(relocate("STACK_local_600"))?;
                                    values[10]
                                }
                            };
                            u64::from(
                                ((((u64::from(memory.read_u8(values[9])?)) as i8 as i64 as u64)
                                    as i32 as i64 as u64) as i64)
                                    == (((values[23]) as i32 as i64 as u64) as i64),
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
                values[10] = values[9];
                pc = 102;
            }
            104 => {
                values[22] = values[9];
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
                values[9] = memory.read_u64(relocate("STACK_local_628"))?;
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 106;
            }
            108 => {
                values[5] = memory.read_u64(relocate("STACK_local_6e8"))?;
                pc = 107;
            }
            109 => {
                values[17] = memory.read_u64(relocate("STACK_local_6e8"))?;
                pc = 108;
            }
            110 => {
                pc = 24;
            }
            111 => {
                pc = if (values[25]) != 0 { 110 } else { 109 };
            }
            112 => {
                values[11] = ((u64::from(memory.read_u32(relocate("STACK_local_5f0"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 111;
            }
            113 => {
                {
                    let destination = relocate("STACK_local_600");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_608");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("STACK_local_618");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("STACK_local_620");
                    let stored = memory.read_u64(relocate("STACK_local_6e8"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                values[14] = (0x0u64) as i32 as i64 as u64;
                pc = 116;
            }
            118 => {
                {
                    let destination = relocate("STACK_local_5e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("STACK_uStack_5d0");
                    let stored = 0xfu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = relocate("STACK_local_5d8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = relocate("STACK_uStack_5e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let destination = relocate("STACK_local_5ec");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let destination = relocate("STACK_local_5f0");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let destination = relocate("STACK_local_5f7");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 123;
            }
            125 => {
                values[25] = u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((float_to_integer(f64::from_bits(values[2]), 64, false)?)
                                        .wrapping_shr((0x34u64) as u32))
                                        as u32,
                                )) & (0x7ffu64)) as u32,
                            )) as u32,
                        )) == (u64::from((0x7ffu64) as u32)),
                    )) != 0,
                );
                pc = 124;
            }
            126 => {
                values[23] = (((values[23])
                    .wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 125;
            }
            127 => {
                pc = if (u64::from((values[17]) != (memory.read_u64(relocate("STACK_local_6e8"))?)))
                    != 0
                {
                    131
                } else {
                    126
                };
            }
            128 => {
                let _ = {
                    values[17] = (values[17]).wrapping_add((0x1u64).wrapping_mul(1));
                    values[17]
                };
                pc = 127;
            }
            129 => {
                {
                    let destination = values[17];
                    let stored = (((values[6])
                        .wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 128;
            }
            130 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((0x60u64) as i32 as i64 as u64) as i64)
                            < (((values[6]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (((values[6]) as i32 as i64 as u64) as i64)
                                < (((0x7bu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    129
                } else {
                    128
                };
            }
            131 => {
                values[6] = ((u64::from(memory.read_u8(values[17])?)) as i8 as i64 as u64) as i8
                    as i64 as u64;
                pc = 130;
            }
            132 => {
                pc = if (values[8]) != 0 { 127 } else { 125 };
            }
            133 => {
                values[17] = memory.read_u64(relocate("STACK_local_628"))?;
                pc = 132;
            }
            134 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_5f4"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 133;
            }
            135 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    134
                } else {
                    133
                };
            }
            136 => {
                {
                    let destination = relocate("STACK_local_628");
                    let stored = relocate("STACK_acStack_5c7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            137 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f6"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    135
                } else {
                    136
                };
            }
            138 => {
                values[18] = (0x0u64) as i8 as i64 as u64;
                pc = 137;
            }
            139 => {
                {
                    let destination = relocate("STACK_local_5f4");
                    let stored = ((((memory.read_u64(relocate("STACK_local_6e8"))?) as i32 as i64
                        as u64)
                        .wrapping_sub(
                            (memory.read_u64(relocate("STACK_local_628"))?) as i32 as i64 as u64,
                        )) as i32 as i64 as u64) as i32 as i64
                        as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 138;
            }
            140 => {
                {
                    let destination = relocate("STACK_local_628");
                    let stored = relocate("STACK_local_5c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x3u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = (0x6eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 141;
            }
            143 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (0x61u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_6e8"))?)
                        .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x6eu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = relocate("STACK_local_5c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    145
                } else {
                    144
                };
            }
            147 => {
                {
                    let destination = relocate("STACK_local_6e8");
                    let stored = relocate("STACK_acStack_5c7");
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_p_Stack_610");
                    let stored = memory.read_u64(relocate("STACK_p_Stack_6e0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 140;
            }
            149 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14002eb40", &call_arguments, memory)
                };
                pc = 148;
            }
            150 => {
                {
                    let call_arguments = [];
                    invoke("FUN_14002da40", &call_arguments, memory)
                };
                pc = 148;
            }
            151 => {
                pc = if (values[25]) != 0 { 149 } else { 150 };
            }
            152 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_6e8"),
                        relocate("STACK_local_5c8"),
                        relocate("STACK_local_5f"),
                        u64::from(
                            ((float_to_integer(f64::from_bits(values[2]), 64, false)?)
                                .wrapping_shr(
                                    (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                                )) as u32,
                        ),
                    ];
                    invoke("FUN_14002d860", &call_arguments, memory)
                };
                pc = 148;
            }
            153 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    151
                } else {
                    152
                };
            }
            154 => {
                pc = if (u64::from(
                    (({
                        let call_arguments = [values[2]];
                        invoke("NAN", &call_arguments, memory)
                    }) != 0)
                        || (({
                            let call_arguments = [values[2]];
                            invoke("NAN", &call_arguments, memory)
                        }) != 0),
                )) != 0
                {
                    147
                } else {
                    153
                };
            }
            155 => {
                {
                    let destination = relocate("STACK_local_5f6");
                    let stored = (u64::from(
                        (((values[14]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 154;
            }
            156 => {
                values[14] = ({
                    let call_arguments = [u64::from(
                        ((float_to_integer(f64::from_bits(values[2]), 64, false)?).wrapping_shr(
                            (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                        )) as u32,
                    )];
                    invoke("_ldsign", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_6d8");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                values[13] = (values[14]) as i32 as i64 as u64;
                pc = 157;
            }
            159 => {
                values[12] = (0x432u64) as i32 as i64 as u64;
                pc = 158;
            }
            160 => {
                pc = if (u64::from(
                    (((0x432u64) as i32 as i64 as u64) as i64)
                        < (((values[12]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    159
                } else {
                    157
                };
            }
            161 => {
                values[13] = (0x0u64) as i32 as i64 as u64;
                pc = 160;
            }
            162 => {
                values[14] = (((values[12])
                    .wrapping_add((0u64.wrapping_sub(0x432u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 161;
            }
            163 => {
                values[12] = (0x6u64) as i32 as i64 as u64;
                pc = 162;
            }
            164 => {
                values[12] = (0x1u64) as i32 as i64 as u64;
                pc = 162;
            }
            165 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    164
                } else {
                    162
                };
            }
            166 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    163
                } else {
                    165
                };
            }
            167 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 166;
            }
            168 => {
                pc = 162;
            }
            169 => {
                values[24] = (0x2u64) as i32 as i64 as u64;
                pc = 168;
            }
            170 => {
                values[12] = (0x6u64) as i32 as i64 as u64;
                pc = 169;
            }
            171 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    170
                } else {
                    169
                };
            }
            172 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 171;
            }
            173 => {
                pc = 162;
            }
            174 => {
                values[24] = (0x1u64) as i32 as i64 as u64;
                pc = 173;
            }
            175 => {
                values[12] = (0x6u64) as i32 as i64 as u64;
                pc = 174;
            }
            176 => {
                pc = if (u64::from(
                    (((values[12]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    175
                } else {
                    174
                };
            }
            177 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 176;
            }
            178 => {
                pc = 162;
            }
            179 => {
                values[25] = u64::from((values[7]) != 0);
                pc = 178;
            }
            180 => {
                pc = 162;
            }
            181 => {
                values[23] = (0x70u64) as i8 as i64 as u64;
                pc = 180;
            }
            182 => {
                values[24] = (0x4u64) as i32 as i64 as u64;
                pc = 181;
            }
            183 => {
                values[8] = u64::from((values[7]) != 0);
                pc = 182;
            }
            184 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x67u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    166
                } else {
                    179
                };
            }
            185 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x47u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    167
                } else {
                    184
                };
            }
            186 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x66u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    171
                } else {
                    185
                };
            }
            187 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x46u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    172
                } else {
                    186
                };
            }
            188 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x65u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    176
                } else {
                    187
                };
            }
            189 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x45u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    188
                };
            }
            190 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x61u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    182
                } else {
                    189
                };
            }
            191 => {
                pc = if (u64::from(
                    (((values[26]) as i32 as i64 as u64) as i64)
                        == (((0x41u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    183
                } else {
                    190
                };
            }
            192 => {
                values[26] = (u64::from(
                    (u64::from(memory.read_u8((values[3]).wrapping_add(0x8u64))?)) as u8,
                )) as i32 as i64 as u64;
                pc = 191;
            }
            193 => {
                values[7] = 1u64;
                pc = 192;
            }
            194 => {
                values[25] = 0u64;
                pc = 193;
            }
            195 => {
                values[12] = ((u64::from(memory.read_u32((values[3]).wrapping_add(0x4u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 194;
            }
            196 => {
                values[23] = (0x65u64) as i8 as i64 as u64;
                pc = 195;
            }
            197 => {
                values[24] = (0x3u64) as i32 as i64 as u64;
                pc = 196;
            }
            198 => {
                values[8] = 0u64;
                pc = 197;
            }
            199 => {
                {
                    let destination = relocate("STACK_local_5f8");
                    let stored = (0x2u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 198;
            }
            200 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_5f8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    199
                } else {
                    198
                };
            }
            201 => {
                {
                    let destination = relocate("STACK_local_5f8");
                    let stored = ((u64::from(memory.read_u8((values[3]).wrapping_add(0xau64))?))
                        as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 200;
            }
            202 => {
                values[19] = (0x0u64) as i32 as i64 as u64;
                pc = 201;
            }
            203 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_728"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 202;
            }
            204 => {
                {
                    let destination = relocate("STACK_PARAM_param_5");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 203;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002c190` (`mspaint.c:41936`).
pub fn fun_14002c190(
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
    ];
    let mut pc: i32 = 111;
    loop {
        match pc {
            0 => {
                pc = 81;
            }
            1 => {
                return Ok(Some(values[0]));
            }
            2 => {
                pc = if (u64::from(!((values[14]) != 0))) != 0 {
                    1
                } else {
                    0
                };
            }
            3 => {
                return Ok(Some(values[0]));
            }
            4 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        != (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                return Ok(Some(values[0]));
            }
            6 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((0xcu64) as i32 as i64 as u64) as i64)
                                    < (((values[17]) as i32 as i64 as u64) as i64),
                            )) != 0)
                                || (({
                                    let _ = {
                                        values[3] = u64::from((values[4]) != 0);
                                        values[3]
                                    };
                                    u64::from(
                                        (u64::from(
                                            (u64::from(
                                                ((u64::from(
                                                    ((0x1204u64 as u32).wrapping_shr(
                                                        (u64::from(
                                                            ((values[11]) & (0x1fu64)) as u32,
                                                        ))
                                                            as u32,
                                                    )
                                                        as u64)
                                                        as u32,
                                                )) & (0x1u64))
                                                    as u32,
                                            )) as u32,
                                        )) == (u64::from((0x0u64) as u32)),
                                    )
                                }) != 0),
                        )) != 0)
                            && (({
                                let _ = {
                                    values[3] = u64::from((values[4]) != 0);
                                    values[3]
                                };
                                u64::from(
                                    (((values[10]) as i32 as i64 as u64) as i64)
                                        != (((0x7u64) as i32 as i64 as u64) as i64),
                                )
                            }) != 0),
                    )) != 0)
                        && ((u64::from(
                            ((u64::from(!((values[6]) != 0))) != 0) || ((values[4]) != 0),
                        )) != 0),
                )) != 0
                {
                    6
                } else {
                    0
                };
            }
            8 => {
                pc = 81;
            }
            9 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[17]) as i32 as i64 as u64) as i64)
                            < (((0xcu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[3] = u64::from((values[4]) != 0);
                                values[3]
                            };
                            u64::from(
                                (u64::from(
                                    (u64::from(
                                        ((u64::from(
                                            ((0x868u64 as u32).wrapping_shr(
                                                (u64::from(((values[11]) & (0x1fu64)) as u32))
                                                    as u32,
                                            ) as u64)
                                                as u32,
                                        )) & (0x1u64))
                                            as u32,
                                    )) as u32,
                                )) != (u64::from((0x0u64) as u32)),
                            )
                        }) != 0),
                )) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                pc = 81;
            }
            11 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((values[10]) as i32 as i64 as u64) as i64)
                                    == (((0x5u64) as i32 as i64 as u64) as i64),
                            )) != 0)
                                || ((u64::from(
                                    (((values[10]) as i32 as i64 as u64) as i64)
                                        == (((0xbu64) as i32 as i64 as u64) as i64),
                                )) != 0),
                        )) != 0)
                            && (({
                                let _ = {
                                    values[3] = u64::from((values[4]) != 0);
                                    values[3]
                                };
                                u64::from(
                                    (((u64::from(
                                        (((values[17]).wrapping_sub(0xau64)) as i32 as i64 as u64)
                                            as u8,
                                    )) as i32 as i64 as u64)
                                        as i64)
                                        < (((0x2u64) as i32 as i64 as u64) as i64),
                                )
                            }) != 0),
                    )) != 0)
                        || ((u64::from(
                            ((u64::from(
                                (u64::from(
                                    (u64::from(
                                        ((u64::from(((values[12]).wrapping_sub(0x6u64)) as u32))
                                            & (0xfbu64))
                                            as u32,
                                    )) as u32,
                                )) == (u64::from((0x0u64) as u32)),
                            )) != 0)
                                && (({
                                    let _ = {
                                        values[3] = u64::from((values[4]) != 0);
                                        values[3]
                                    };
                                    u64::from(
                                        (((values[17]) as i32 as i64 as u64) as i64)
                                            == (((0xau64) as i32 as i64 as u64) as i64),
                                    )
                                }) != 0),
                        )) != 0),
                )) != 0
                {
                    10
                } else {
                    7
                };
            }
            12 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    11
                };
            }
            13 => {
                values[13] = u64::from((values[14]) != 0);
                pc = 12;
            }
            14 => {
                return Ok(Some(values[0]));
            }
            15 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((u64::from(((values[11]) & (0xfau64)) as u32)) as u32))
                            == (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            (((values[17]) as i32 as i64 as u64) as i64)
                                != (((0x5u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    14
                } else {
                    13
                };
            }
            16 => {
                return Ok(Some(values[0]));
            }
            17 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    16
                } else {
                    15
                };
            }
            18 => {
                values[14] = u64::from((values[13]) != 0);
                pc = 17;
            }
            19 => {
                return Ok(Some(values[0]));
            }
            20 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                pc = 81;
            }
            22 => {
                return Ok(Some(values[0]));
            }
            23 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        != (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[3] = u64::from((values[4]) != 0);
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    24
                } else {
                    20
                };
            }
            26 => {
                values[13] = u64::from((values[5]) != 0);
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    25
                };
            }
            28 => {
                values[13] = u64::from(
                    (u64::from((((values[13]) ^ (0x1u64)) as i32 as i64 as u64) != 0)) != 0,
                );
                pc = 27;
            }
            29 => {
                values[6] = 0u64;
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 29;
            }
            31 => {
                pc = if (u64::from(!((values[3]) != 0))) != 0 {
                    30
                } else {
                    29
                };
            }
            32 => {
                values[6] = 1u64;
                pc = 15;
            }
            33 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 32;
            }
            34 => {
                pc = 29;
            }
            35 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    15
                };
            }
            37 => {
                values[6] = 0u64;
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        == (((0xcu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    37
                };
            }
            39 => {
                values[14] = u64::from((values[5]) != 0);
                pc = 38;
            }
            40 => {
                pc = 29;
            }
            41 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[0]).wrapping_add((0x4u64).wrapping_mul(8)))?) as i8
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    39
                };
            }
            42 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[0]).wrapping_add((0x4u64).wrapping_mul(8)))?) as i8
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    41
                };
            }
            43 => {
                values[17] = u64::from((u64::from((values[11]) as u8)) as u8);
                pc = 42;
            }
            44 => {
                values[10] = ((values[12]) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 43;
            }
            45 => {
                values[4] = 1u64;
                pc = 44;
            }
            46 => {
                values[4] = u64::from((values[5]) != 0);
                pc = 44;
            }
            47 => {
                pc = if (u64::from(
                    (u64::from((values[1]) as u32))
                        < (u64::from(
                            (u64::from(
                                ((u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u16(
                                            (relocate("UNK_140109926")).wrapping_add(
                                                ((values[16]).wrapping_mul(0x2u64)).wrapping_mul(8),
                                            ),
                                        )?)) as u16,
                                    )) as u32,
                                ))
                                .wrapping_add(
                                    (u64::from(memory.read_u32(
                                        (relocate("UNK_1401097ec")).wrapping_add(
                                            ((values[16]).wrapping_mul(0x4u64)).wrapping_mul(8),
                                        ),
                                    )?)) as i32 as i64 as u64,
                                )) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    46
                } else {
                    44
                };
            }
            48 => {
                values[4] = 1u64;
                pc = 47;
            }
            49 => {
                pc = if (u64::from(((values[16]) as i64) == ((0x0u64) as i64))) != 0 {
                    45
                } else {
                    48
                };
            }
            50 => {
                values[16] = ((values[15]).wrapping_sub(relocate("DAT_1401097f0")) as i64)
                    .wrapping_shr((0x2u64) as u32) as u64;
                pc = 49;
            }
            51 => {
                pc = if (u64::from(((0x0u64) as i64) < ((values[8]) as i64))) != 0 {
                    56
                } else {
                    50
                };
            }
            52 => {
                values[9] = values[8];
                pc = 51;
            }
            53 => {
                values[8] = (values[9]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_sub(values[8]),
                );
                pc = 52;
            }
            54 => {
                values[15] = ((values[15])
                    .wrapping_add(((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1)))
                .wrapping_add((0x4u64).wrapping_mul(1));
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[15]).wrapping_add(
                                ((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1),
                            ))?)) as u32,
                        )) as u32,
                    )) <= (u64::from((values[1]) as u32)),
                )) != 0
                {
                    54
                } else {
                    52
                };
            }
            56 => {
                values[8] = (values[9]).wrapping_shr((0x1u64) as u32);
                pc = 55;
            }
            57 => {
                values[9] = 0x4eu64;
                pc = 56;
            }
            58 => {
                values[15] = relocate("DAT_1401097f0");
                pc = 57;
            }
            59 => {
                values[11] = u64::from((0xffu64) as u32);
                pc = 58;
            }
            60 => {
                values[11] = u64::from(
                    (u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(
                                (relocate("UNK_14010af3a")).wrapping_add(
                                    ((values[16]).wrapping_mul(0x2u64)).wrapping_mul(8),
                                ),
                            )?)) as u16,
                        ) as i32)
                            .wrapping_shr((0xcu64) as u32) as u64) as i32
                            as i64 as u64) as u32,
                    )) as u32,
                );
                pc = 58;
            }
            61 => {
                pc = if (u64::from(
                    ((u64::from(((values[16]) as i64) == ((0x0u64) as i64))) != 0)
                        || ((u64::from(
                            (u64::from(
                                (((((u64::from(
                                    (u64::from(memory.read_u16(
                                        (relocate("UNK_14010af3a")).wrapping_add(
                                            ((values[16]).wrapping_mul(0x2u64)).wrapping_mul(8),
                                        ),
                                    )?)) as u16,
                                )) & (0xfffu64)) as i32 as i64
                                    as u64)
                                    .wrapping_add(
                                        (u64::from(memory.read_u32(
                                            (relocate("UNK_1401099cc")).wrapping_add(
                                                ((values[16]).wrapping_mul(0x4u64)).wrapping_mul(8),
                                            ),
                                        )?)) as i32 as i64
                                            as u64,
                                    )) as i32 as i64 as u64) as u32,
                            )) <= (u64::from((values[1]) as u32)),
                        )) != 0),
                )) != 0
                {
                    59
                } else {
                    60
                };
            }
            62 => {
                values[16] = ((values[15]).wrapping_sub(relocate("DAT_1401099d0")) as i64)
                    .wrapping_shr((0x2u64) as u32) as u64;
                pc = 61;
            }
            63 => {
                pc = if (u64::from(((0x0u64) as i64) < ((values[8]) as i64))) != 0 {
                    68
                } else {
                    62
                };
            }
            64 => {
                values[9] = values[8];
                pc = 63;
            }
            65 => {
                values[8] = (values[9]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_sub(values[8]),
                );
                pc = 64;
            }
            66 => {
                values[15] = ((values[15])
                    .wrapping_add(((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1)))
                .wrapping_add((0x4u64).wrapping_mul(1));
                pc = 65;
            }
            67 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[15]).wrapping_add(
                                ((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1),
                            ))?)) as u32,
                        )) as u32,
                    )) <= (u64::from((values[1]) as u32)),
                )) != 0
                {
                    66
                } else {
                    64
                };
            }
            68 => {
                values[8] = (values[9]).wrapping_shr((0x1u64) as u32);
                pc = 67;
            }
            69 => {
                values[9] = 0x55bu64;
                pc = 68;
            }
            70 => {
                values[15] = relocate("DAT_1401099d0");
                pc = 69;
            }
            71 => {
                values[1] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 70;
            }
            72 => {
                return Ok(Some(values[0]));
            }
            73 => {
                pc = if (u64::from(
                    ((memory.read_u64(values[0])?) as i64)
                        == ((memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            as i64),
                )) != 0
                {
                    72
                } else {
                    71
                };
            }
            74 => {
                {
                    let destination = (values[0]).wrapping_add(0x14u64);
                    let stored = u64::from((values[2]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = u64::from(
                        (u64::from((u64::from(memory.read_u32(values[7])?)) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 74;
            }
            76 => {
                values[2] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[7]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 75;
            }
            77 => {
                values[7] = {
                    let call_arguments = [
                        relocate("STACK_local_48"),
                        values[16],
                        memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        (values[0]).wrapping_add((0x3u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_14002c790", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                pc = if (u64::from(
                    ((values[16]) as i64)
                        != ((memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            as i64),
                )) != 0
                {
                    77
                } else {
                    73
                };
            }
            79 => {
                {
                    let destination = values[0];
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                values[16] = memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 79;
            }
            81 => {
                values[12] = u64::from((values[11]) as u32);
                pc = 80;
            }
            82 => {
                values[3] = 1u64;
                pc = 81;
            }
            83 => {
                values[3] = u64::from((values[5]) != 0);
                pc = 81;
            }
            84 => {
                pc = if (u64::from(
                    (u64::from((values[12]) as u32))
                        < (u64::from(
                            (u64::from(
                                ((u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u16(
                                            (relocate("UNK_140109926")).wrapping_add(
                                                ((values[16]).wrapping_mul(0x2u64)).wrapping_mul(8),
                                            ),
                                        )?)) as u16,
                                    )) as u32,
                                ))
                                .wrapping_add(
                                    (u64::from(memory.read_u32(
                                        (relocate("UNK_1401097ec")).wrapping_add(
                                            ((values[16]).wrapping_mul(0x4u64)).wrapping_mul(8),
                                        ),
                                    )?)) as i32 as i64 as u64,
                                )) as u32,
                            )) as u32,
                        )),
                )) != 0
                {
                    83
                } else {
                    81
                };
            }
            85 => {
                values[3] = 1u64;
                pc = 84;
            }
            86 => {
                pc = if (u64::from(((values[16]) as i64) == ((0x0u64) as i64))) != 0 {
                    82
                } else {
                    85
                };
            }
            87 => {
                values[13] = u64::from((values[5]) != 0);
                pc = 86;
            }
            88 => {
                values[5] = 0u64;
                pc = 87;
            }
            89 => {
                values[16] = ((values[15]).wrapping_sub(relocate("DAT_1401097f0")) as i64)
                    .wrapping_shr((0x2u64) as u32) as u64;
                pc = 88;
            }
            90 => {
                pc = if (u64::from(((0x0u64) as i64) < ((values[8]) as i64))) != 0 {
                    95
                } else {
                    89
                };
            }
            91 => {
                values[9] = values[8];
                pc = 90;
            }
            92 => {
                values[8] = (values[9]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_sub(values[8]),
                );
                pc = 91;
            }
            93 => {
                values[15] = ((values[15])
                    .wrapping_add(((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1)))
                .wrapping_add((0x4u64).wrapping_mul(1));
                pc = 92;
            }
            94 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[15]).wrapping_add(
                                ((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1),
                            ))?)) as u32,
                        )) as u32,
                    )) <= (u64::from((values[12]) as u32)),
                )) != 0
                {
                    93
                } else {
                    91
                };
            }
            95 => {
                values[8] = (values[9]).wrapping_shr((0x1u64) as u32);
                pc = 94;
            }
            96 => {
                values[9] = 0x4eu64;
                pc = 95;
            }
            97 => {
                values[15] = relocate("DAT_1401097f0");
                pc = 96;
            }
            98 => {
                values[11] = u64::from((0xffu64) as u32);
                pc = 97;
            }
            99 => {
                values[11] = u64::from(
                    (u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(
                                (relocate("UNK_14010af3a")).wrapping_add(
                                    ((values[16]).wrapping_mul(0x2u64)).wrapping_mul(8),
                                ),
                            )?)) as u16,
                        ) as i32)
                            .wrapping_shr((0xcu64) as u32) as u64) as i32
                            as i64 as u64) as u32,
                    )) as u32,
                );
                pc = 97;
            }
            100 => {
                pc = if (u64::from(
                    ((u64::from(((values[16]) as i64) == ((0x0u64) as i64))) != 0)
                        || ((u64::from(
                            (u64::from(
                                (((((u64::from(
                                    (u64::from(memory.read_u16(
                                        (relocate("UNK_14010af3a")).wrapping_add(
                                            ((values[16]).wrapping_mul(0x2u64)).wrapping_mul(8),
                                        ),
                                    )?)) as u16,
                                )) & (0xfffu64)) as i32 as i64
                                    as u64)
                                    .wrapping_add(
                                        (u64::from(memory.read_u32(
                                            (relocate("UNK_1401099cc")).wrapping_add(
                                                ((values[16]).wrapping_mul(0x4u64)).wrapping_mul(8),
                                            ),
                                        )?)) as i32 as i64
                                            as u64,
                                    )) as i32 as i64 as u64) as u32,
                            )) <= (u64::from((values[12]) as u32)),
                        )) != 0),
                )) != 0
                {
                    98
                } else {
                    99
                };
            }
            101 => {
                values[16] = ((values[15]).wrapping_sub(relocate("DAT_1401099d0")) as i64)
                    .wrapping_shr((0x2u64) as u32) as u64;
                pc = 100;
            }
            102 => {
                pc = if (u64::from(((0x0u64) as i64) < ((values[8]) as i64))) != 0 {
                    107
                } else {
                    101
                };
            }
            103 => {
                values[9] = values[8];
                pc = 102;
            }
            104 => {
                values[8] = (values[9]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_sub(values[8]),
                );
                pc = 103;
            }
            105 => {
                values[15] = ((values[15])
                    .wrapping_add(((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1)))
                .wrapping_add((0x4u64).wrapping_mul(1));
                pc = 104;
            }
            106 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32((values[15]).wrapping_add(
                                ((values[8]).wrapping_mul(0x4u64)).wrapping_mul(1),
                            ))?)) as u32,
                        )) as u32,
                    )) <= (u64::from((values[12]) as u32)),
                )) != 0
                {
                    105
                } else {
                    103
                };
            }
            107 => {
                values[8] = (values[9]).wrapping_shr((0x1u64) as u32);
                pc = 106;
            }
            108 => {
                values[9] = 0x55bu64;
                pc = 107;
            }
            109 => {
                values[15] = relocate("DAT_1401099d0");
                pc = 108;
            }
            110 => {
                values[12] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140034d90` (`mspaint.c:49148`).
pub fn fun_140034d90(
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
    ];
    let mut pc: i32 = 154;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_68"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64))) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_pcStack_40"))?];
                    invoke("GdipDisposeImage", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("Gdiplus::Image::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let destination = values[1];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                values[2] = memory.read_u64(relocate("STACK_local_50"))?;
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    10
                } else {
                    9
                };
            }
            12 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_b8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            14 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x40u64),
                        )?,
                        memory.read_u64(
                            (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                        values[4],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Media.Imaging.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x263u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_b8")];
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
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Media.Imaging.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x261u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_50"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_50"))?,
                        relocate("DAT_14010dd70"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 23;
            }
            25 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_50"))?) != (0x0u64))) != 0
                {
                    24
                } else {
                    23
                };
            }
            26 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                values[4] = memory.read_u64(relocate("STACK_local_78"))?;
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = memory.read_u64(
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            30 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_b8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            31 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_60"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_60"))?,
                        relocate("DAT_14010dd80"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) == (0x0u64))) != 0
                {
                    28
                } else {
                    36
                };
            }
            38 => {
                {
                    let call_arguments = [
                        values[12],
                        relocate("STACK_local_50"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("FUN_14003b8b0", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("_DAT_1401cfd48");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = relocate("FUN_14003bb10");
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 40;
            }
            42 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 41;
            }
            43 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 37;
            }
            44 => {
                {
                    let destination = relocate("_DAT_1401cfd48");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfd48"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
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
                    let call_arguments = [
                        values[12],
                        relocate("STACK_local_50"),
                        relocate("DAT_1401cfd40"),
                    ];
                    invoke("FUN_14003ba30", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfd40"))?) == (0x0u64))) != 0
                {
                    42
                } else {
                    46
                };
            }
            48 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("_DAT_1401cfd48");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfd48"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 49;
            }
            51 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = relocate("DAT_1401cfd48");
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_68"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            53 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64))) != 0
                {
                    52
                } else {
                    0
                };
            }
            54 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_pcStack_40"))?];
                    invoke("GdipDisposeImage", &call_arguments, memory)
                };
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("Gdiplus::Image::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    56
                } else {
                    55
                };
            }
            58 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    51
                } else {
                    58
                };
            }
            60 => {
                values[3] = memory.read_u64(relocate("_DAT_1401cfd48"))?;
                pc = 59;
            }
            61 => {
                values[12] = {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    61
                } else {
                    60
                };
            }
            63 => {
                values[12] = relocate("CONTEXT_IGNORED_FUN_140034d90_extraout_XMM0_Qa");
                pc = 62;
            }
            64 => {
                values[6] = ({
                    let call_arguments = [relocate("STACK_local_60"), relocate("STACK_local_78")];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_60"),
                        (relocate("STACK_local_48")) as i32 as i64 as u64,
                    ];
                    invoke(
                        "GdiplusHelpers::CreateStreamFromBitmap",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 65;
            }
            67 => {
                {
                    let call_arguments = [values[4]];
                    invoke("GdipDeleteGraphics", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                {
                    let call_arguments = [values[4], values[12], 0x0u64, 0x0u64];
                    invoke("GdipDrawImageRectRectI", &call_arguments, memory)
                };
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_108")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[10]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_100")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[11]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 71;
            }
            73 => {
                values[7] = (0x0u64) as i32 as i64 as u64;
                pc = 72;
            }
            74 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_e0");
                    let stored = ((u64::from(memory.read_u32(relocate("STACK_local_6c"))?)) as i32
                        as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = u64::from((0x2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
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
                    let destination = relocate("STACK_local_c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                values[12] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_68"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8)),
                )?;
                pc = 79;
            }
            81 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64))) != 0
                {
                    80
                } else {
                    79
                };
            }
            82 => {
                values[12] = 0x0u64;
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [values[4], values[8]];
                    invoke("GdipSetInterpolationMode", &call_arguments, memory)
                };
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [values[4], 0x4u64];
                    invoke("GdipSetPixelOffsetMode", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                        0x1u64,
                    ];
                    invoke("GdipSetCompositingMode", &call_arguments, memory)
                };
                pc = 84;
            }
            86 => {
                values[4] = memory.read_u64(
                    (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 85;
            }
            87 => {
                {
                    let call_arguments = [values[4], relocate("STACK_local_88")];
                    invoke("GdipGetImageGraphicsContext", &call_arguments, memory)
                };
                pc = 86;
            }
            88 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_68"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 0;
            }
            90 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64))) != 0
                {
                    89
                } else {
                    0
                };
            }
            91 => {
                {
                    let call_arguments = [];
                    invoke("GdipDisposeImage", &call_arguments, memory)
                };
                pc = 90;
            }
            92 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("Gdiplus::Image::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                pc = if (u64::from(
                    (((values[9]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    88
                } else {
                    93
                };
            }
            95 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_38"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_pcStack_40");
                    let stored = memory.read_u64(
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                values[4] = memory.read_u64(
                    (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 96;
            }
            98 => {
                values[9] = ({
                    let call_arguments = [values[10], values[11], 0x0u64, 0xe200bu64];
                    invoke("GdipCreateBitmapFromScan0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                {
                    let destination = relocate("STACK_local_100");
                    let stored = relocate("STACK_local_88");
                    memory.write_u64(destination, stored)?;
                }
                pc = 99;
            }
            101 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = relocate("Gdiplus::Bitmap::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let destination = relocate("STACK_pcStack_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                values[8] = u64::from((0x5u64) as u32);
                pc = 104;
            }
            106 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_6c"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((values[11]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    105
                } else {
                    104
                };
            }
            107 => {
                values[8] = u64::from((0x3u64) as u32);
                pc = 106;
            }
            108 => {
                values[10] = (0x1u64) as i32 as i64 as u64;
                pc = 107;
            }
            109 => {
                pc = if (u64::from(
                    (((float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from((((values[11]) as i64) as f32).to_bits())) as u32,
                                )) * (f32::from_bits((values[13]) as u32)))
                                .to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64) as i64)
                        < (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    108
                } else {
                    107
                };
            }
            110 => {
                values[10] = (float_to_integer(
                    (f32::from_bits(
                        (u64::from(
                            ((f32::from_bits(
                                (u64::from((((values[11]) as i64) as f32).to_bits())) as u32,
                            )) * (f32::from_bits((values[13]) as u32)))
                            .to_bits(),
                        )) as u32,
                    )) as f64,
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 109;
            }
            111 => {
                values[11] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32(relocate("STACK_local_54"))?))
                                        as u32,
                                )) as u32,
                            )) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(
                            ((memory.read_u64(relocate("STACK_local_78"))?) as f64).to_bits(),
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 110;
            }
            112 => {
                values[8] = u64::from((0x3u64) as u32);
                pc = 104;
            }
            113 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        <= ((((u64::from(memory.read_u32(relocate("STACK_local_70"))?)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    112
                } else {
                    104
                };
            }
            114 => {
                values[8] = u64::from((0x5u64) as u32);
                pc = 113;
            }
            115 => {
                values[11] = (0x1u64) as i32 as i64 as u64;
                pc = 114;
            }
            116 => {
                pc = if (u64::from(
                    (((float_to_integer(
                        (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from((((values[10]) as i64) as f32).to_bits())) as u32,
                                )) / (f32::from_bits((values[13]) as u32)))
                                .to_bits(),
                            )) as u32,
                        )) as f64,
                        32,
                        true,
                    )?) as i32 as i64 as u64) as i64)
                        < (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    115
                } else {
                    114
                };
            }
            117 => {
                values[11] = (float_to_integer(
                    (f32::from_bits(
                        (u64::from(
                            ((f32::from_bits(
                                (u64::from((((values[10]) as i64) as f32).to_bits())) as u32,
                            )) / (f32::from_bits((values[13]) as u32)))
                            .to_bits(),
                        )) as u32,
                    )) as f64,
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 116;
            }
            118 => {
                values[10] = (float_to_integer(
                    f64::from_bits(
                        ((f64::from_bits(
                            ((f32::from_bits(
                                (u64::from(
                                    (u64::from(memory.read_u32(relocate("STACK_local_58"))?))
                                        as u32,
                                )) as u32,
                            )) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(
                            ((memory.read_u64(relocate("STACK_local_78"))?) as f64).to_bits(),
                        )))
                        .to_bits(),
                    ),
                    32,
                    true,
                )?) as i32 as i64 as u64;
                pc = 117;
            }
            119 => {
                pc = if (u64::from(
                    (f32::from_bits((values[13]) as u32))
                        < (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("STACK_local_58"))?))
                                            as u32,
                                    )) as u32,
                                )) / (f32::from_bits(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("STACK_local_54"))?))
                                            as u32,
                                    )) as u32,
                                )))
                                .to_bits(),
                            )) as u32,
                        )),
                )) != 0
                {
                    111
                } else {
                    118
                };
            }
            120 => {
                values[13] = u64::from(
                    ((f32::from_bits(
                        (u64::from(
                            ((((u64::from(memory.read_u32(relocate("STACK_local_70"))?)) as i32
                                as i64 as u64) as i64) as f32)
                                .to_bits(),
                        )) as u32,
                    )) / (f32::from_bits(
                        (u64::from(
                            ((((u64::from(memory.read_u32(relocate("STACK_local_6c"))?)) as i32
                                as i64 as u64) as i64) as f32)
                                .to_bits(),
                        )) as u32,
                    )))
                    .to_bits(),
                );
                pc = 119;
            }
            121 => {
                values[7] = ((u64::from(memory.read_u32(relocate("STACK_local_70"))?)) as i32 as i64
                    as u64) as i32 as i64 as u64;
                pc = 120;
            }
            122 => {
                {
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 121;
            }
            123 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_b8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            124 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    123
                } else {
                    122
                };
            }
            125 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x258u64),
                        )?,
                        memory.read_u64(
                            (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 124;
            }
            126 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 125;
            }
            127 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x290au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 127;
            }
            129 => {
                {
                    let call_arguments = [values[11], relocate("STACK_local_b8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            130 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    129
                } else {
                    128
                };
            }
            131 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 130;
            }
            132 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 131;
            }
            133 => {
                {
                    let destination =
                        (relocate("STACK_local_b8")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x2908u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 132;
            }
            134 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[2])?)?,
                        values[2],
                        relocate("DAT_140105538"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 133;
            }
            135 => {
                pc = if (u64::from((values[2]) != (0x0u64))) != 0 {
                    134
                } else {
                    133
                };
            }
            136 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x28u64))?;
                pc = 136;
            }
            138 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 137;
            }
            139 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x28u64),
                        relocate("STACK_local_58"),
                    ];
                    invoke("FUN_1400028d0", &call_arguments, memory)
                };
                pc = 138;
            }
            140 => {
                {
                    let destination = (values[5]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 139;
            }
            141 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    140
                } else {
                    139
                };
            }
            142 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(relocate("STACK_local_68"))?)
                                .wrapping_add((0x1u64).wrapping_mul(8)),
                        )?,
                        relocate("STACK_local_6c"),
                    ];
                    invoke("GdipGetImageHeight", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 141;
            }
            143 => {
                {
                    let destination = relocate("STACK_local_6c");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 143;
            }
            145 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    144
                } else {
                    143
                };
            }
            146 => {
                values[5] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 145;
            }
            147 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(relocate("STACK_local_68"))?)
                                .wrapping_add((0x1u64).wrapping_mul(8)),
                        )?,
                        relocate("STACK_local_70"),
                    ];
                    invoke("GdipGetImageWidth", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 147;
            }
            149 => {
                values[11] = (0x0u64) as i32 as i64 as u64;
                pc = 148;
            }
            150 => {
                values[2] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 149;
            }
            151 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        relocate("STACK_local_68"),
                    ];
                    invoke("BasePaint::Image::Bitmap", &call_arguments, memory)
                };
                pc = 150;
            }
            152 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 151;
            }
            153 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_128"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 153;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400365d0` (`mspaint.c:50435`).
pub fn fun_1400365d0(
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
    ];
    let mut pc: i32 = 48;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[4]];
                    invoke("GdipFree", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments =
                        [memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?];
                    invoke("GdipDeleteGraphics", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64))) != 0
                {
                    2
                } else {
                    0
                };
            }
            4 => {
                values[4] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [values[0], relocate("STACK_local_88"), 0x0u64];
                    invoke("FUN_140036140", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_88"), relocate("STACK_local_98")];
                    invoke("CopyRect", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = ((u64::from(
                        memory.read_u32((values[7]).wrapping_add((0xcu64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = ((u64::from(
                        memory.read_u32((values[7]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = ((u64::from(
                        memory.read_u32((values[7]).wrapping_add((0x4u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = ((u64::from(
                        memory.read_u32((values[7]).wrapping_add((0x0u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 9;
            }
            11 => {
                values[7] = {
                    let call_arguments = [relocate("STACK_local_78"), relocate("STACK_local_58")];
                    invoke("ToNative", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_58"),
                        relocate("STACK_local_98"),
                        values[3],
                    ];
                    invoke("FUN_1400349f0", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = ((u64::from(memory.read_u32(relocate("STACK_local_60"))?)) as i32
                        as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = ((u64::from(memory.read_u32(relocate("STACK_local_5c"))?)) as i32
                        as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination =
                        (relocate("STACK_local_98")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[2]).wrapping_add(0x10u64);
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    16
                };
            }
            19 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64((values[2]).wrapping_add(0x8u64))?,
                        relocate("STACK_local_5c"),
                    ];
                    invoke("GdipGetImageHeight", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_5c");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[2]).wrapping_add(0x10u64);
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
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
                    20
                };
            }
            23 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64((values[2]).wrapping_add(0x8u64))?,
                        relocate("STACK_local_60"),
                    ];
                    invoke("GdipGetImageWidth", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[5]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    24
                };
            }
            27 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        values[8],
                        0x0u64,
                        0x0u64,
                    ];
                    invoke("GdipDrawImageI", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 26;
            }
            28 => {
                values[8] = memory.read_u64((values[2]).wrapping_add(0x8u64))?;
                pc = 27;
            }
            29 => {
                pc = if (u64::from(((values[2]) as i64) != ((0x0u64) as i64))) != 0 {
                    28
                } else {
                    27
                };
            }
            30 => {
                {
                    let destination = (values[4]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 29;
            }
            31 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                values[5] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 31;
            }
            33 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        0x4u64,
                    ];
                    invoke("GdipSetPixelOffsetMode", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[5]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                values[4] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 35;
            }
            37 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        0x7u64,
                    ];
                    invoke("GdipSetInterpolationMode", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let destination = (values[4]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[6]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    37
                };
            }
            40 => {
                values[5] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 39;
            }
            41 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(relocate("STACK_local_68"))?)?,
                        memory.read_u64(values[3])?,
                    ];
                    invoke("GdipSetWorldTransform", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 40;
            }
            42 => {
                values[4] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0x40u64))?,
                        relocate("STACK_local_68"),
                    ];
                    invoke("BasePaint::Image::Graphics", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                values[8] = 0x0u64;
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [
                        values[0],
                        values[1],
                        memory.read_u64(relocate("STACK_local_68"))?,
                    ];
                    invoke("FUN_140035f80", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                {
                    let destination = (relocate("STACK_local_68")).wrapping_add(0x0u64);
                    let stored =
                        ((u64::from((memory.read_u64(relocate("STACK_local_68"))?) as u32))
                            & 0xffffffffffu64)
                            & 0xffffffffffu64;
                    memory.write_word(destination, 5, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140044400` (`mspaint.c:63479`).
pub fn fun_140044400(
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
    ];
    let mut pc: i32 = 75;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                pc = 30;
            }
            2 => {
                values[7] = values[3];
                pc = 1;
            }
            3 => {
                {
                    let destination = (values[10]).wrapping_add((values[9]).wrapping_mul(0x10u64));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[11]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = values[6];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                values[6] = memory.read_u64((values[11]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 7;
            }
            9 => {
                {
                    let destination = values[5];
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[5] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 9;
            }
            11 => {
                {
                    let destination = values[4];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    (memory
                        .read_u64((values[10]).wrapping_add((values[9]).wrapping_mul(0x10u64)))?)
                        != (values[11]),
                )) != 0
                {
                    28
                } else {
                    12
                };
            }
            14 => {
                pc = 30;
            }
            15 => {
                values[7] = values[3];
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[10]).wrapping_add(0x8u64);
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = values[5];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                values[5] = memory.read_u64((values[10]).wrapping_add(0x8u64))?;
                pc = 19;
            }
            21 => {
                {
                    let destination = values[4];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                values[4] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 21;
            }
            23 => {
                {
                    let destination = values[11];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                values[11] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 23;
            }
            25 => {
                values[10] = memory.read_u64(values[11])?;
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    13
                };
            }
            27 => {
                values[8] = ({
                    let call_arguments = [
                        (values[7]).wrapping_add((0x2u64).wrapping_mul(8)),
                        (values[11]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 26;
            }
            28 => {
                values[11] =
                    memory.read_u64((values[11]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 27;
            }
            29 => {
                pc = if (u64::from(
                    (memory
                        .read_u64((values[10]).wrapping_add((values[9]).wrapping_mul(0x10u64)))?)
                        != (values[11]),
                )) != 0
                {
                    28
                } else {
                    12
                };
            }
            30 => {
                pc = 48;
            }
            31 => {
                values[7] = values[3];
                pc = 30;
            }
            32 => {
                {
                    let destination = ((values[10]).wrapping_add(0x8u64))
                        .wrapping_add((values[9]).wrapping_mul(0x10u64));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = (values[11]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = values[6];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                values[6] = memory.read_u64((values[11]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 36;
            }
            38 => {
                {
                    let destination = values[5];
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                values[5] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 38;
            }
            40 => {
                {
                    let destination = values[4];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                values[4] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 40;
            }
            42 => {
                pc = if (u64::from((values[11]) != (values[7]))) != 0 {
                    41
                } else {
                    32
                };
            }
            43 => {
                values[11] = memory.read_u64(values[11])?;
                pc = 42;
            }
            44 => {
                pc = 29;
            }
            45 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    43
                };
            }
            46 => {
                values[8] = ({
                    let call_arguments = [
                        (values[7]).wrapping_add((0x2u64).wrapping_mul(8)),
                        (values[11]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140001a20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 45;
            }
            47 => {
                values[11] = memory.read_u64(
                    ((values[10]).wrapping_add(0x8u64))
                        .wrapping_add((values[9]).wrapping_mul(0x10u64)),
                )?;
                pc = 46;
            }
            48 => {
                pc = 59;
            }
            49 => {
                values[7] = values[3];
                pc = 48;
            }
            50 => {
                {
                    let destination = ((values[10]).wrapping_add(0x8u64))
                        .wrapping_add((values[9]).wrapping_mul(0x10u64));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = (values[10]).wrapping_add((values[9]).wrapping_mul(0x10u64));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                pc = 47;
            }
            53 => {
                pc = if (u64::from(
                    (memory
                        .read_u64((values[10]).wrapping_add((values[9]).wrapping_mul(0x10u64)))?)
                        != (values[2]),
                )) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                values[9] &= memory.read_u64((values[0]).wrapping_add(0x30u64))?;
                pc = 53;
            }
            55 => {
                values[10] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
                pc = 54;
            }
            56 => {
                values[9] = {
                    let call_arguments = [];
                    invoke("FUN_1400445c0", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                values[3] = memory.read_u64(values[7])?;
                pc = 56;
            }
            58 => {
                return Ok(None);
            }
            59 => {
                pc = if (u64::from((values[7]) == (values[2]))) != 0 {
                    58
                } else {
                    57
                };
            }
            60 => {
                values[7] = memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?;
                pc = 30;
            }
            61 => {
                {
                    let destination = (values[0]).wrapping_add(0x30u64);
                    let stored =
                        (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = (values[0]).wrapping_add(0x38u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x18u64),
                        (values[10]).wrapping_mul(0x2u64),
                        values[2],
                    ];
                    invoke("FUN_140044150", &call_arguments, memory)
                };
                pc = 62;
            }
            64 => {
                values[10] = ((0x1u64 as i32).wrapping_shl(
                    (u64::from(
                        ((u64::from(
                            (((values[10]) as i8 as i64 as u64).wrapping_add(0x1u64)) as u32,
                        )) & (0x3fu64)) as u32,
                    )) as u32,
                ) as u64) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                pc = if (u64::from(((values[9]).wrapping_shr((values[10]) as u32)) == (0x0u64)))
                    != 0
                {
                    66
                } else {
                    64
                };
            }
            66 => {
                let _ = {
                    values[10] =
                        (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    values[10]
                };
                pc = 65;
            }
            67 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    65
                } else {
                    64
                };
            }
            68 => {
                values[10] = 0x3fu64;
                pc = 67;
            }
            69 => {
                values[9] = ((values[1]).wrapping_sub(0x1u64)) | (0x1u64);
                pc = 68;
            }
            70 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [relocate("\"invalid hash bucket count\"")];
                    invoke("std::_Xlength_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xlength_error"));
            }
            72 => {
                pc = if (u64::from(
                    (((0x1u64 as i32).wrapping_shl(
                        (((u64::from((values[10]) as u8)) & (0x3fu64)) as i32 as i64 as u64) as u32,
                    ) as u64) as i32 as i64 as u64)
                        < (values[1]),
                )) != 0
                {
                    71
                } else {
                    70
                };
            }
            73 => {
                pc = if (u64::from(
                    ((0xfffffffffffffffu64).wrapping_shr((values[10]) as u32)) == (0x0u64),
                )) != 0
                {
                    74
                } else {
                    72
                };
            }
            74 => {
                let _ = {
                    values[10] =
                        (values[10]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    values[10]
                };
                pc = 73;
            }
            75 => {
                let _ = {
                    values[10] = 0x3fu64;
                    values[10]
                };
                pc = 73;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14005d180` (`mspaint.c:84810`).
pub fn fun_14005d180(
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
        u64::from((arguments[0]) as u8),
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
    ];
    let mut pc: i32 = 72;
    loop {
        match pc {
            0 => {
                pc = 6;
            }
            1 => {
                pc = 56;
            }
            2 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[6]) as i32 as i64 as u64) as i64)
                            == (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[7] = (values[15]).wrapping_add(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                        .wrapping_mul(2),
                                );
                                values[7]
                            };
                            u64::from(
                                (((values[6]) as i32 as i64 as u64) as i64)
                                    == (((0x2fu64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                values[6] = ((u64::from(memory.read_u16((values[15]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                ))?)) as i16 as i64 as u64) as i16 as i64 as u64;
                pc = 2;
            }
            4 => {
                pc = 56;
            }
            5 => {
                pc = if (u64::from((values[9]) == (values[15]))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[15] = values[7];
                pc = 5;
            }
            7 => {
                pc = if (u64::from((values[9]) != (values[5]))) != 0 {
                    11
                } else {
                    6
                };
            }
            8 => {
                values[9] = (values[9]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 7;
            }
            9 => {
                pc = 6;
            }
            10 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[9])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u16(values[9])?)) as i16 as i64 as u64) as i32
                                as i64 as u64) as i64)
                                != (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    9
                } else {
                    8
                };
            }
            11 => {
                values[7] = values[5];
                pc = 10;
            }
            12 => {
                return Ok(Some(u64::from((values[14]) as u8)));
            }
            13 => {
                {
                    let call_arguments = [relocate("STACK_local_68")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                values[14] = u64::from((0x0u64) as u8);
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [
                        values[12],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_98"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            18 => {
                pc = if (u64::from(
                    (((((values[8]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    14
                };
            }
            19 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    16
                } else {
                    18
                };
            }
            20 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 19;
            }
            21 => {
                {
                    let destination = values[4];
                    let stored = ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                values[8] = ((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 21;
            }
            23 => {
                values[4] = (memory.read_u64(relocate("STACK_local_98"))?).wrapping_add(0x18u64);
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_98"))?) != (0x0u64))) != 0
                {
                    24
                } else {
                    14
                };
            }
            26 => {
                {
                    let call_arguments = [values[12], 0x0u64, values[10]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            29 => {
                pc = if (u64::from(
                    (((((values[8]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    28
                } else {
                    25
                };
            }
            30 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    29
                };
            }
            31 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 30;
            }
            32 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 31;
            }
            33 => {
                values[8] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 32;
            }
            34 => {
                values[3] = (memory.read_u64(relocate("STACK_local_90"))?)
                    .wrapping_add((0xcu64).wrapping_mul(2));
                pc = 33;
            }
            35 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 34;
            }
            36 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    35
                } else {
                    25
                };
            }
            37 => {
                values[10] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 36;
            }
            38 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140044770", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [relocate("STACK_local_90"), relocate("STACK_local_80")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x17u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = relocate("L\"CannotCompleteOperation\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [relocate("STACK_local_98"), relocate("STACK_local_90")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0xbu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = relocate("L\"ExportError\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    13
                };
            }
            46 => {
                values[14] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("STACK_local_70")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 45;
            }
            47 => {
                values[8] = ({
                    let call_arguments =
                        [relocate("DAT_1401d0520"), values[0], values[1], values[2]];
                    invoke("FUN_140070390", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = relocate("STACK_local_70");
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination =
                        (relocate("STACK_local_70")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [values[1], values[11]];
                    invoke("ATL::CStringT<>::operator=", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                values[11] = {
                    let call_arguments = [relocate("STACK_local_98"), values[10]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                values[10] = memory.read_u64(values[10])?;
                pc = 52;
            }
            54 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[10]).wrapping_add((0xcu64).wrapping_mul(2)))?),
                )) != 0
                {
                    53
                } else {
                    52
                };
            }
            55 => {
                values[10] = {
                    let call_arguments = [relocate("STACK_local_68")];
                    invoke("FUN_140066f10", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                pc = if (u64::from((0x1u64) < ((values[5]).wrapping_sub(values[15])))) != 0 {
                    55
                } else {
                    49
                };
            }
            57 => {
                pc = if (u64::from((values[9]) == (values[5]))) != 0 {
                    56
                } else {
                    11
                };
            }
            58 => {
                values[15] = values[5];
                pc = 57;
            }
            59 => {
                values[9] = {
                    let call_arguments = [values[13], values[5]];
                    invoke("FUN_140019de0", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                values[5] = (values[13]).wrapping_add(
                    (memory.read_u64(relocate("STACK_local_58"))?).wrapping_mul(0x2u64),
                );
                pc = 59;
            }
            61 => {
                values[13] = memory.read_u64(relocate("STACK_local_68"))?;
                pc = 60;
            }
            62 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_50"))?))) != 0
                {
                    61
                } else {
                    60
                };
            }
            63 => {
                values[13] = relocate("STACK_local_68");
                pc = 62;
            }
            64 => {
                {
                    let call_arguments = [relocate("STACK_local_68")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
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
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory.read_u64(values[1])?)
                                .wrapping_add((values[16]).wrapping_mul(0x2u64)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    70
                } else {
                    68
                };
            }
            70 => {
                values[16] = (values[16]).wrapping_add(0x1u64);
                pc = 69;
            }
            71 => {
                values[16] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140064070` (`mspaint.c:90252`).
pub fn fun_140064070(
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
    ];
    let mut pc: i32 = 57;
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
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = ((values[4]).wrapping_add(0x8u64))
                        .wrapping_add((values[10]).wrapping_mul(0x10u64));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (memory.read_u64(relocate("DAT_1401d0e68"))?)
                        .wrapping_add((values[10]).wrapping_mul(0x10u64));
                    let stored = values[5];
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
                        ((memory.read_u64(relocate("DAT_1401d0e68"))?).wrapping_add(0x8u64))
                            .wrapping_add((values[10]).wrapping_mul(0x10u64)),
                    )?) != (values[3]),
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
                    let destination = (memory.read_u64(relocate("DAT_1401d0e68"))?)
                        .wrapping_add((values[10]).wrapping_mul(0x10u64));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((values[8]) as i64) == ((memory.read_u64(relocate("STACK_local_38"))?) as i64),
                )) != 0
                {
                    8
                } else {
                    6
                };
            }
            10 => {
                pc = if (u64::from((values[8]) == (memory.read_u64(relocate("DAT_1401d0e58"))?)))
                    != 0
                {
                    4
                } else {
                    9
                };
            }
            11 => {
                values[8] = memory.read_u64(
                    (memory.read_u64(relocate("DAT_1401d0e68"))?)
                        .wrapping_add((values[10]).wrapping_mul(0x10u64)),
                )?;
                pc = 10;
            }
            12 => {
                values[10] &= memory.read_u64(relocate("DAT_1401d0e80"))?;
                pc = 11;
            }
            13 => {
                values[4] = memory.read_u64(relocate("DAT_1401d0e68"))?;
                pc = 12;
            }
            14 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_local_38"))?).wrapping_add(0x8u64);
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = values[3];
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[5]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = values[5];
                    let stored = memory.read_u64(relocate("STACK_local_38"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = relocate("DAT_1401d0e60");
                    let stored = (memory.read_u64(relocate("DAT_1401d0e60"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                values[3] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_38"))?).wrapping_add(0x8u64),
                )?;
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = memory.read_u64(values[6])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                values[6] = {
                    let call_arguments = [
                        values[13],
                        relocate("STACK_local_28"),
                        (values[5]).wrapping_add((0x2u64).wrapping_mul(8)),
                        values[10],
                    ];
                    invoke("FUN_140063f70", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                values[13] = u64::from(
                    ({
                        let call_arguments = [values[9], values[11]];
                        invoke("FUN_140065c80", &call_arguments, memory)
                    }) as u32,
                );
                pc = 21;
            }
            23 => {
                values[11] = values[9];
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    ((u64::from((values[7]) < (values[9]))) != 0)
                        && ((u64::from(
                            ((u64::from((0x1ffu64) < (values[7]))) != 0)
                                || (({
                                    let _ = {
                                        values[11] = (values[7]).wrapping_mul(0x8u64);
                                        values[11]
                                    };
                                    u64::from(((values[7]).wrapping_mul(0x8u64)) < (values[9]))
                                }) != 0),
                        )) != 0),
                )) != 0
                {
                    23
                } else {
                    22
                };
            }
            25 => {
                values[11] = values[7];
                pc = 24;
            }
            26 => {
                values[9] =
                    (float_to_integer((f32::from_bits((values[12]) as u32)) as f64, 64, true)?)
                        .wrapping_add(values[8]);
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    (0x8u64)
                        < ((float_to_integer(
                            (f32::from_bits((values[12]) as u32)) as f64,
                            64,
                            true,
                        )?)
                        .wrapping_add(values[8])),
                )) != 0
                {
                    26
                } else {
                    25
                };
            }
            28 => {
                values[9] = 0x8u64;
                pc = 27;
            }
            29 => {
                values[8] = 0u64.wrapping_sub(0x8000000000000000u64);
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f64::from_bits(0x43dffffffddad230u64))
                            <= ((f32::from_bits((values[12]) as u32)) as f64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[12] = u64::from(
                                    ((f64::from_bits(
                                        (((f32::from_bits((values[12]) as u32)) as f64)
                                            - (f64::from_bits(0x43dffffffddad230u64)))
                                        .to_bits(),
                                    )) as f32)
                                        .to_bits(),
                                );
                                values[12]
                            };
                            u64::from(
                                ((f32::from_bits((values[12]) as u32)) as f64)
                                    < (f64::from_bits(0x43dffffffddad230u64)),
                            )
                        }) != 0),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[8] = 0x0u64;
                pc = 30;
            }
            32 => {
                values[12] = {
                    let call_arguments = [u64::from(
                        ((f32::from_bits(
                            (u64::from(
                                (((memory.read_u64(relocate("DAT_1401d0e60"))?)
                                    .wrapping_add(0x1u64)) as f32)
                                    .to_bits(),
                            )) as u32,
                        )) / ((u64::from(
                            (u64::from(memory.read_u8(relocate("DAT_1401d0e50"))?)) as u8,
                        )) as f32))
                            .to_bits(),
                    )];
                    invoke("ceilf", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    ((u64::from((u64::from(memory.read_u8(relocate("DAT_1401d0e50"))?)) as u8))
                        as f32)
                        < (f32::from_bits(
                            (u64::from(
                                ((f32::from_bits(
                                    (u64::from(
                                        (((memory.read_u64(relocate("DAT_1401d0e60"))?)
                                            .wrapping_add(0x1u64))
                                            as f32)
                                            .to_bits(),
                                    )) as u32,
                                )) / (f32::from_bits(
                                    (u64::from(
                                        ((memory.read_u64(relocate("DAT_1401d0e88"))?) as f32)
                                            .to_bits(),
                                    )) as u32,
                                )))
                                .to_bits(),
                            )) as u32,
                        )),
                )) != 0
                {
                    33
                } else {
                    19
                };
            }
            35 => {
                values[7] = memory.read_u64(relocate("DAT_1401d0e88"))?;
                pc = 34;
            }
            36 => {
                {
                    let destination = (values[5]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                values[8] = memory.read_u64(values[2])?;
                pc = 37;
            }
            39 => {
                values[5] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = relocate("DAT_1401d0e58");
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [relocate("\"unordered_map/set too long\"")];
                    invoke("std::_Xlength_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xlength_error"));
            }
            43 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("DAT_1401d0e60"))?) == (0xaaaaaaaaaaaaaaau64),
                )) != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                return Ok(Some(values[1]));
            }
            45 => {
                {
                    let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = values[1];
                    let stored = memory.read_u64(relocate("STACK_lStack_30"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_lStack_30"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    46
                } else {
                    43
                };
            }
            48 => {
                {
                    let call_arguments =
                        [values[7], relocate("STACK_local_38"), values[2], values[10]];
                    invoke("FUN_140063f70", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    (values[7])
                        < ((u64::from(
                            (u64::from(memory.read_u32((values[8]).wrapping_add(0x4u64))?)) as u32,
                        ))
                        .wrapping_mul(0x2u64)),
                )) != 0
                {
                    51
                } else {
                    48
                };
            }
            50 => {
                values[7] = (values[7]).wrapping_add(0x1u64);
                pc = 49;
            }
            51 => {
                values[10] = ((values[10])
                    ^ (u64::from(
                        (u64::from(
                            memory.read_u8(
                                (memory.read_u64((values[8]).wrapping_add(0x10u64))?)
                                    .wrapping_add(values[7]),
                            )?,
                        )) as u8,
                    )))
                .wrapping_mul(0x100000001b3u64);
                pc = 50;
            }
            52 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[8]).wrapping_add(0x4u64))?)) as u32,
                    )) != (0x0u64),
                )) != 0
                {
                    51
                } else {
                    48
                };
            }
            53 => {
                values[7] = 0x0u64;
                pc = 52;
            }
            54 => {
                pc = if (u64::from(((values[8]) as i64) != ((0x0u64) as i64))) != 0 {
                    53
                } else {
                    48
                };
            }
            55 => {
                values[7] = 0x0u64;
                pc = 54;
            }
            56 => {
                values[10] = 0xcbf29ce484222325u64;
                pc = 55;
            }
            57 => {
                values[8] = memory.read_u64(values[2])?;
                pc = 56;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14006f820` (`mspaint.c:99751`).
pub fn fun_14006f820(
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
                return Ok(Some(values[7]));
            }
            1 => {
                values[7] = 0x1u64;
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [values[0], 0x1u64];
                    invoke("FUN_140075fe0", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let destination = (values[0]).wrapping_add(0x2a4u64);
                    let stored =
                        ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x2a4u64))?)) as i16
                            as i64 as u64)
                            .wrapping_add(0x1u64)) as i32 as i64 as u64)
                            as i16 as i64 as u64;
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 2;
            }
            4 => {
                values[7] = 0x0u64;
                pc = 0;
            }
            5 => {
                {
                    let call_arguments = [
                        values[6],
                        0x0u64,
                        memory.read_u64(relocate("STACK_local_238"))?,
                    ];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                values[6] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            8 => {
                pc = if (u64::from(
                    (((((values[5]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    4
                };
            }
            9 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
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
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                values[5] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 11;
            }
            13 => {
                values[3] = (memory.read_u64(relocate("STACK_local_238"))?).wrapping_add(0x18u64);
                pc = 12;
            }
            14 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_238"))?) != (0x0u64)))
                    != 0
                {
                    14
                } else {
                    4
                };
            }
            16 => {
                {
                    let call_arguments = [values[6], 0x0u64, values[4]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                values[6] = {
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
                    (((((values[5]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
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
                    (((values[5]) as i32 as i64 as u64) as i64)
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
                    let destination = values[2];
                    let stored = ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                values[5] = ((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 22;
            }
            24 => {
                values[2] = (memory.read_u64(relocate("STACK_local_248"))?).wrapping_add(0xcu64);
                pc = 23;
            }
            25 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 24;
            }
            26 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_248"))?) != (0x0u64)))
                    != 0
                {
                    25
                } else {
                    15
                };
            }
            27 => {
                values[4] = memory.read_u64(relocate("STACK_local_248"))?;
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [relocate("STACK_local_248")];
                    invoke("FUN_140044770", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [relocate("STACK_local_248"), relocate("STACK_local_230")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_228");
                    let stored = 0x1du64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_230");
                    let stored = relocate("L\"EnterpriseFileOpenErrorHeader\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [relocate("STACK_local_238"), relocate("STACK_local_248")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_240");
                    let stored = 0x17u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = relocate("L\"EnterpriseFileOpenError\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [values[0], 0x0u64];
                    invoke("FUN_140075fe0", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    35
                };
            }
            37 => {
                values[5] = ({
                    let call_arguments = [values[1], relocate("STACK_local_218")];
                    invoke("FUN_14000fd00", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_local_248");
                    let stored = (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_248")).wrapping_add(0x4u64))?,
                        )) as u32,
                    ))
                    .wrapping_shl((0x20u64) as u32);
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [relocate("STACK_local_218"), 0x0u64, 0x1fau64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_18");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_268"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14006feb0` (`mspaint.c:100040`).
pub fn fun_14006feb0(
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
    let mut pc: i32 = 85;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("STACK_local_80")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [values[1], values[9]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                values[9] = memory.read_u64(relocate("STACK_local_80"))?;
                pc = 3;
            }
            5 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_68"))?))) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[9] = relocate("STACK_local_80");
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_88"))?];
                    invoke("CoTaskMemFree", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    7
                } else {
                    6
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_80"), relocate("STACK_local_40")];
                    invoke("FUN_14007c400", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_40"),
                        memory.read_u64(relocate("STACK_local_88"))?,
                        values[12],
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_uStack_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_uStack_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory.read_u64(relocate("STACK_local_88"))?)
                                .wrapping_add((values[12]).wrapping_mul(0x2u64)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    15
                };
            }
            17 => {
                values[12] = (values[12]).wrapping_add(0x1u64);
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x4d9u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            19 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                values[6] = ({
                    let call_arguments = [
                        relocate("DAT_1400e4eb0"),
                        0x8000u64,
                        0x0u64,
                        relocate("STACK_local_88"),
                    ];
                    invoke("SHGetKnownFolderPath", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let call_arguments = [values[7]];
                    invoke("SetLastError", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [values[4]];
                    invoke("CoTaskMemFree", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                values[7] = u64::from(
                    ({
                        let call_arguments = [];
                        invoke("GetLastError", &call_arguments, memory)
                    }) as u32,
                );
                pc = 23;
            }
            25 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    24
                } else {
                    21
                };
            }
            26 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    17
                };
            }
            27 => {
                values[4] = memory.read_u64(relocate("STACK_local_88"))?;
                pc = 26;
            }
            28 => {
                values[6] = ({
                    let call_arguments = [
                        relocate("DAT_1400e4ec0"),
                        0x8000u64,
                        0x0u64,
                        relocate("STACK_local_88"),
                    ];
                    invoke("SHGetKnownFolderPath", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                pc = 34;
            }
            31 => {
                values[5] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                ((u64::from((memory.read_u64(relocate("STACK_local_30"))?) as u32))
                                    & (0x10u64)) as u32,
                            )) | (0x20u64)) as u32,
                        ) as u32)
                            .wrapping_shr((0x4u64) as u32) as u64) as u32,
                    )) as u32,
                );
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((u64::from(
                                        (memory.read_u64(relocate("STACK_local_30"))?) as u32,
                                    ) as u32)
                                        .wrapping_shr((0xau64) as u32)
                                        as u64) as u32,
                                )) & (0x1u64)) as u32,
                            )) as u32,
                        )) == (u64::from((0x0u64) as u32)),
                    )) != 0)
                        || ((u64::from(
                            ((u64::from(
                                (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(
                                            (relocate("STACK_local_30")).wrapping_add(0x4u64),
                                        )?)) as u32,
                                    )) as u32,
                                )) != (u64::from(
                                    ((0u64.wrapping_sub(0x5ffffff4u64)) as i32 as i64 as u64)
                                        as u32,
                                )),
                            )) != 0)
                                && ((u64::from(
                                    (u64::from(
                                        (u64::from(
                                            (u64::from(memory.read_u32(
                                                (relocate("STACK_local_30")).wrapping_add(0x4u64),
                                            )?)) as u32,
                                        )) as u32,
                                    )) != (u64::from(
                                        ((0u64.wrapping_sub(0x5ffffffdu64)) as i32 as i64 as u64)
                                            as u32,
                                    )),
                                )) != 0),
                        )) != 0),
                )) != 0
                {
                    31
                } else {
                    29
                };
            }
            33 => {
                pc = 6;
            }
            34 => {
                pc = if (u64::from((u64::from((values[5]) as u32)) == (u64::from((0x3u64) as u32))))
                    != 0
                {
                    33
                } else {
                    29
                };
            }
            35 => {
                values[5] = u64::from((0x1u64) as u32);
                pc = 34;
            }
            36 => {
                pc = 34;
            }
            37 => {
                pc = if (u64::from(!((values[14]) != 0))) != 0 {
                    36
                } else {
                    35
                };
            }
            38 => {
                values[5] = u64::from((0x0u64) as u32);
                pc = 37;
            }
            39 => {
                values[14] = u64::from(
                    (u64::from((u64::from((values[5]) as u32)) == (u64::from((0x35u64) as u32))))
                        != 0,
                );
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (u64::from((values[5]) as u32)) != (u64::from((0x40u64) as u32)),
                        )) != 0)
                            && ((u64::from(
                                (u64::from((values[5]) as u32)) != (u64::from((0x2u64) as u32)),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(
                            (u64::from((values[5]) as u32)) != (u64::from((0x3u64) as u32)),
                        )) != 0),
                )) != 0
                {
                    39
                } else {
                    35
                };
            }
            41 => {
                pc = 38;
            }
            42 => {
                values[14] = u64::from(
                    (u64::from((u64::from((values[5]) as u32)) == (u64::from((0x10bu64) as u32))))
                        != 0,
                );
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    (u64::from((values[5]) as u32)) != (u64::from((0x7bu64) as u32)),
                )) != 0
                {
                    42
                } else {
                    35
                };
            }
            44 => {
                pc = if (u64::from((u64::from((values[5]) as u32)) < (u64::from((0x41u64) as u32))))
                    != 0
                {
                    40
                } else {
                    43
                };
            }
            45 => {
                pc = if (u64::from((u64::from((values[5]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    32
                } else {
                    44
                };
            }
            46 => {
                values[5] = u64::from(
                    ({
                        let call_arguments =
                            [values[9], relocate("STACK_local_40"), 0x3u64, 0xffffffffu64];
                        invoke("FUN_1400b173c", &call_arguments, memory)
                    }) as u32,
                );
                pc = 45;
            }
            47 => {
                values[9] = memory.read_u64(relocate("STACK_local_80"))?;
                pc = 46;
            }
            48 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_68"))?))) != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                values[9] = relocate("STACK_local_80");
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_80"),
                        values[11],
                        ((values[10]).wrapping_sub(values[11]) as i64).wrapping_shr((0x1u64) as u32)
                            as u64,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_uStack_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[2])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            == (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            ((((u64::from(memory.read_u16(values[2])?)) as i16 as i64 as u64) as i32
                                as i64 as u64) as i64)
                                == (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    59
                } else {
                    54
                };
            }
            56 => {
                values[2] = (values[10]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                );
                pc = 55;
            }
            57 => {
                pc = 54;
            }
            58 => {
                pc = if (u64::from((values[8]) == (values[10]))) != 0 {
                    57
                } else {
                    56
                };
            }
            59 => {
                values[10] = values[2];
                pc = 58;
            }
            60 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[3]) as i32 as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[10] = (values[10]).wrapping_add(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                        .wrapping_mul(2),
                                );
                                values[10]
                            };
                            u64::from(
                                (((values[3]) as i32 as i64 as u64) as i64)
                                    != (((0x2fu64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    64
                } else {
                    59
                };
            }
            61 => {
                values[2] = values[10];
                pc = 60;
            }
            62 => {
                values[3] = ((u64::from(memory.read_u16((values[10]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                ))?)) as i16 as i64 as u64) as i16 as i64 as u64;
                pc = 61;
            }
            63 => {
                pc = 54;
            }
            64 => {
                pc = if (u64::from((values[8]) == (values[10]))) != 0 {
                    63
                } else {
                    62
                };
            }
            65 => {
                pc = if (u64::from((values[8]) != (values[10]))) != 0 {
                    68
                } else {
                    64
                };
            }
            66 => {
                values[8] = (values[8]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 65;
            }
            67 => {
                pc = 64;
            }
            68 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[8])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u16(values[8])?)) as i16 as i64 as u64) as i32
                                as i64 as u64) as i64)
                                != (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    67
                } else {
                    66
                };
            }
            69 => {
                pc = if (u64::from((values[8]) != (values[10]))) != 0 {
                    68
                } else {
                    54
                };
            }
            70 => {
                values[8] = {
                    let call_arguments = [values[11], values[10]];
                    invoke("FUN_140019de0", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                values[10] = (values[11]).wrapping_add(
                    (memory.read_u64(relocate("STACK_local_50"))?).wrapping_mul(0x2u64),
                );
                pc = 70;
            }
            72 => {
                values[11] = memory.read_u64(relocate("STACK_local_60"))?;
                pc = 71;
            }
            73 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_48"))?))) != 0
                {
                    72
                } else {
                    71
                };
            }
            74 => {
                values[11] = relocate("STACK_local_60");
                pc = 73;
            }
            75 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let destination = relocate("STACK_uStack_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory.read_u64((values[0]).wrapping_add(0x6a8u64))?)
                                .wrapping_add((values[13]).wrapping_mul(0x2u64)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    82
                } else {
                    80
                };
            }
            82 => {
                values[13] = (values[13]).wrapping_add(0x1u64);
                pc = 81;
            }
            83 => {
                values[13] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 82;
            }
            84 => {
                values[12] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140091b50` (`mspaint.c:126002`).
pub fn fun_140091b50(
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
    ];
    let mut pc: i32 = 357;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[14]));
            }
            1 => {
                values[14] = 0x0u64;
                pc = 0;
            }
            2 => {
                values[14] = 0x1u64;
                pc = 0;
            }
            3 => {
                {
                    let call_arguments = [relocate("STACK_local_288")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((values[8]) != (0x0u64))) != 0 {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let call_arguments = [relocate("STACK_local_280")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_280"))?) != (0x0u64)))
                    != 0
                {
                    5
                } else {
                    4
                };
            }
            7 => {
                {
                    let call_arguments = [relocate("STACK_local_290")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from((values[17]) != (0x0u64))) != 0 {
                    7
                } else {
                    6
                };
            }
            9 => {
                {
                    let call_arguments = [relocate("STACK_local_298")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) != (0x0u64)))
                    != 0
                {
                    9
                } else {
                    8
                };
            }
            11 => {
                {
                    let call_arguments = [relocate("STACK_local_2f8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2f8"))?) != (0x0u64)))
                    != 0
                {
                    11
                } else {
                    10
                };
            }
            13 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_2a0"))?,
                        memory.read_u64(relocate("STACK_local_2f0"))?,
                    ];
                    invoke("FUN_140093524", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    14
                } else {
                    12
                };
            }
            16 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2f8"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2f8"))?,
                        relocate("DAT_14011a2f0"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2f8"))?) != (0x0u64)))
                    != 0
                {
                    17
                } else {
                    10
                };
            }
            19 => {
                {
                    let call_arguments = [relocate("STACK_local_2d8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let call_arguments =
                        [memory.read_u64(relocate("STACK_local_2a0"))?, values[14]];
                    invoke("FUN_140093524", &call_arguments, memory)
                };
                pc = 20;
            }
            22 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    21
                } else {
                    19
                };
            }
            23 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2d8"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2d8"))?,
                        relocate("DAT_14011a2f0"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2d8"))?) != (0x0u64)))
                    != 0
                {
                    24
                } else {
                    18
                };
            }
            26 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = (values[0]).wrapping_add((0x218u64).wrapping_mul(1));
                    let stored = memory.read_u64(relocate("STACK_local_2d0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x218u64).wrapping_mul(1)))?;
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_2d8"),
                        (values[0]).wrapping_add((0x210u64).wrapping_mul(1)),
                    ];
                    invoke("FUN_140093420", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored =
                        memory.read_u64((values[15]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = values[15];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = memory.read_u64(values[15])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                values[15] = {
                    let call_arguments = [relocate("STACK_local_280"), relocate("STACK_local_2f8")];
                    invoke("FUN_140092bd0", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                values[16] = memory.read_u64(relocate("STACK_local_2a0"))?;
                pc = 34;
            }
            36 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_2a0"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 37;
            }
            39 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 38;
            }
            40 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = relocate("FUN_140092e10");
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            43 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    42
                } else {
                    34
                };
            }
            44 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let call_arguments = [relocate("STACK_local_298")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) != (0x0u64)))
                    != 0
                {
                    45
                } else {
                    44
                };
            }
            47 => {
                {
                    let call_arguments = [relocate("STACK_local_2c0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2c0"))?) != (0x0u64)))
                    != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_2a0"))?,
                        (((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_uStack_2b4"))?)) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl((0x20u64) as u32))
                            | ((u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_2b8"))?)) as u32,
                            )) & (0xffffffffu64)),
                    ];
                    invoke("FUN_140093514", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    50
                } else {
                    48
                };
            }
            52 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2c0"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2c0"))?,
                        relocate("DAT_14011a2f0"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2c0"))?) != (0x0u64)))
                    != 0
                {
                    53
                } else {
                    46
                };
            }
            55 => {
                {
                    let call_arguments = [relocate("STACK_local_2d8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                {
                    let call_arguments =
                        [memory.read_u64(relocate("STACK_local_2a0"))?, values[14]];
                    invoke("FUN_140093514", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    57
                } else {
                    55
                };
            }
            59 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2d8"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2d8"))?,
                        relocate("DAT_14011a2f0"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2d8"))?) != (0x0u64)))
                    != 0
                {
                    60
                } else {
                    54
                };
            }
            62 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (values[0]).wrapping_add((0x208u64).wrapping_mul(1));
                    let stored = memory.read_u64(relocate("STACK_local_2d0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x208u64).wrapping_mul(1)))?;
                pc = 63;
            }
            65 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_2d8"),
                        (values[0]).wrapping_add((0x200u64).wrapping_mul(1)),
                    ];
                    invoke("FUN_140093420", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored =
                        memory.read_u64((values[15]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination = values[15];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = memory.read_u64(values[15])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                values[15] = {
                    let call_arguments = [relocate("STACK_local_280"), relocate("STACK_local_2c0")];
                    invoke("FUN_140092a00", &call_arguments, memory)
                };
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = memory.read_u64(relocate("STACK_local_2a0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_2a0"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 73;
            }
            75 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = relocate("FUN_140092da0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 76;
            }
            78 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    77
                } else {
                    69
                };
            }
            79 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 80;
            }
            82 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) != (0x0u64)))
                    != 0
                {
                    81
                } else {
                    80
                };
            }
            83 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = memory.read_u64(relocate("STACK_local_298"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2d8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            85 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    84
                } else {
                    83
                };
            }
            86 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)),
                            )?)?)
                            .wrapping_add(0x80u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)))?,
                        memory.read_u64(relocate("STACK_local_290"))?,
                        relocate("STACK_local_298"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 85;
            }
            87 => {
                {
                    let destination = relocate("STACK_local_2c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2d8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0xd6u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                values[17] = memory.read_u64(relocate("STACK_local_290"))?;
                pc = 90;
            }
            92 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_290"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("FUN_140093540", &call_arguments, memory)
                };
                pc = 91;
            }
            93 => {
                {
                    let destination = relocate("_DAT_1401d00c8");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 93;
            }
            95 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 94;
            }
            96 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 91;
            }
            97 => {
                {
                    let destination = relocate("_DAT_1401d00c8");
                    let stored = (memory.read_u64(relocate("_DAT_1401d00c8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 97;
            }
            99 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2d8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            100 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    99
                } else {
                    98
                };
            }
            101 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d00c0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d00c0"))?,
                        0x0u64,
                        relocate("STACK_local_290"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("STACK_local_2c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = relocate("STACK_local_2d0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2d8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x26au64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = relocate("STACK_local_290");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            106 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d00c0"))?) == (0x0u64))) != 0
                {
                    95
                } else {
                    105
                };
            }
            107 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 106;
            }
            108 => {
                {
                    let destination = relocate("_DAT_1401d00c8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d00c8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = relocate("DAT_1401d00c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = relocate("STACK_local_2e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_2e0"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
            }
            113 => {
                values[3] = memory.read_u64(relocate("_DAT_1401d00c8"))?;
                pc = 112;
            }
            114 => {
                {
                    let call_arguments = [relocate("STACK_local_298")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 113;
            }
            115 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) != (0x0u64)))
                    != 0
                {
                    114
                } else {
                    113
                };
            }
            116 => {
                {
                    let call_arguments = [relocate("STACK_local_2d8")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 115;
            }
            117 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2d8"))?) != (0x0u64)))
                    != 0
                {
                    116
                } else {
                    115
                };
            }
            118 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 117;
            }
            119 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_290"))?,
                        memory.read_u64(relocate("STACK_local_2d0"))?,
                    ];
                    invoke("FUN_140069ea8", &call_arguments, memory)
                };
                pc = 118;
            }
            120 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_290"))?) != (0x0u64)))
                    != 0
                {
                    119
                } else {
                    117
                };
            }
            121 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = memory.read_u64(relocate("STACK_local_290"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2d8"))?)?)
                                .wrapping_add(0x18u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2d8"))?,
                        relocate("DAT_1401151b8"),
                        relocate("STACK_local_290"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 121;
            }
            123 => {
                {
                    let destination = relocate("STACK_local_290");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 122;
            }
            124 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2d8"))?) != (0x0u64)))
                    != 0
                {
                    123
                } else {
                    115
                };
            }
            125 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x1f0u64).wrapping_mul(1)),
                        values[14],
                    ];
                    invoke("FUN_140093350", &call_arguments, memory)
                };
                pc = 124;
            }
            126 => {
                values[14] = {
                    let call_arguments = [relocate("STACK_local_280"), relocate("STACK_local_2d8")];
                    invoke("FUN_1400927a0", &call_arguments, memory)
                };
                pc = 125;
            }
            127 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = memory.read_u64(relocate("STACK_local_2a0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_2a0"))?;
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 127;
            }
            129 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 128;
            }
            130 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 129;
            }
            131 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 130;
            }
            132 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = relocate("FUN_140092e80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 131;
            }
            133 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 132;
            }
            134 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_2a0"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 133;
            }
            135 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2a0"))?) != (0x0u64)))
                    != 0
                {
                    134
                } else {
                    126
                };
            }
            136 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = {
                        let call_arguments = [0x20u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let call_arguments = [relocate("STACK_local_290")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 137;
            }
            139 => {
                pc = if (u64::from((values[17]) != (0x0u64))) != 0 {
                    138
                } else {
                    137
                };
            }
            140 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            141 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    140
                } else {
                    139
                };
            }
            142 => {
                values[6] = ({
                    let call_arguments = [memory.read_u64(
                        (memory
                            .read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)),
                            )?)?)
                        .wrapping_add(0x38u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 141;
            }
            143 => {
                {
                    let destination = relocate("STACK_local_290");
                    let stored = values[17];
                    memory.write_u64(destination, stored)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x33u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 145;
            }
            147 => {
                values[17] = memory.read_u64(relocate("STACK_local_290"))?;
                pc = 146;
            }
            148 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_280"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_280"))?,
                        relocate("DAT_14011a2f0"),
                        relocate("STACK_local_290"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 147;
            }
            149 => {
                {
                    let destination = relocate("STACK_local_290");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_280"))?) != (0x0u64)))
                    != 0
                {
                    149
                } else {
                    146
                };
            }
            151 => {
                values[17] = values[16];
                pc = 150;
            }
            152 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            153 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    152
                } else {
                    151
                };
            }
            154 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_280"))?)?)
                                .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_280"))?,
                        values[8],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 153;
            }
            155 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 154;
            }
            156 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1a33u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let call_arguments = [
                        values[20],
                        relocate("STACK_local_280"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("FUN_140093820", &call_arguments, memory)
                };
                pc = 157;
            }
            159 => {
                {
                    let destination = relocate("_DAT_1401d01c8");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 158;
            }
            160 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = relocate("FUN_140093a80");
                    memory.write_u64(destination, stored)?;
                }
                pc = 159;
            }
            161 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 160;
            }
            162 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 161;
            }
            163 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 157;
            }
            164 => {
                {
                    let destination = relocate("_DAT_1401d01c8");
                    let stored = (memory.read_u64(relocate("_DAT_1401d01c8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 163;
            }
            165 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 164;
            }
            166 => {
                {
                    let call_arguments = [
                        values[20],
                        relocate("STACK_local_280"),
                        relocate("DAT_1401d01c0"),
                    ];
                    invoke("FUN_1400939a0", &call_arguments, memory)
                };
                pc = 165;
            }
            167 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d01c0"))?) == (0x0u64))) != 0
                {
                    162
                } else {
                    166
                };
            }
            168 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 167;
            }
            169 => {
                {
                    let destination = relocate("_DAT_1401d01c8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d01c8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 168;
            }
            170 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 169;
            }
            171 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = relocate("DAT_1401d01c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                values[3] = memory.read_u64(relocate("_DAT_1401d01c8"))?;
                pc = 171;
            }
            173 => {
                values[20] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_298")];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 172;
            }
            174 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) != (0x0u64)))
                    != 0
                {
                    173
                } else {
                    172
                };
            }
            175 => {
                values[20] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_2e0")];
                        invoke("FUN_140001b30", &call_arguments, memory)
                    }) as u32,
                );
                pc = 174;
            }
            176 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64)))
                    != 0
                {
                    175
                } else {
                    174
                };
            }
            177 => {
                values[8] = memory.read_u64(relocate("STACK_local_288"))?;
                pc = 176;
            }
            178 => {
                values[20] = u64::from(
                    ({
                        let call_arguments = [
                            0x0u64,
                            relocate("STACK_local_288"),
                            relocate("STACK_local_2f8"),
                        ];
                        invoke("FUN_140093aa0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 177;
            }
            179 => {
                {
                    let destination = relocate("_DAT_1401d01a8");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 178;
            }
            180 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 179;
            }
            181 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 180;
            }
            182 => {
                values[20] = u64::from(
                    (u64::from((relocate("CONTEXT_IGNORED_FUN_140091b50_extraout_XMM0_Da")) as u32))
                        as u32,
                );
                pc = 177;
            }
            183 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("_DAT_1401d01a8");
                    let stored = (memory.read_u64(relocate("_DAT_1401d01a8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
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
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            187 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    186
                } else {
                    185
                };
            }
            188 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d01a0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401d01a0"))?,
                        memory.read_u64(relocate("STACK_local_298"))?,
                        memory.read_u64(relocate("STACK_local_2e0"))?,
                        relocate("STACK_local_288"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 187;
            }
            189 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 188;
            }
            190 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 189;
            }
            191 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1ad4u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 190;
            }
            192 => {
                {
                    let destination = (relocate("STACK_local_2f8")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_2f8"))?)
                                .wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 191;
            }
            193 => {
                {
                    let destination = relocate("STACK_local_288");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 192;
            }
            194 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d01a0"))?) == (0x0u64))) != 0
                {
                    181
                } else {
                    193
                };
            }
            195 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 194;
            }
            196 => {
                {
                    let destination = relocate("_DAT_1401d01a8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401d01a8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 195;
            }
            197 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 196;
            }
            198 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = relocate("DAT_1401d01a8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 197;
            }
            199 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored = relocate("STACK_local_2e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 198;
            }
            200 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = relocate("STACK_local_298");
                    memory.write_u64(destination, stored)?;
                }
                pc = 199;
            }
            201 => {
                values[3] = memory.read_u64(relocate("_DAT_1401d01a8"))?;
                pc = 200;
            }
            202 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b78"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 201;
            }
            203 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b78"))?) != (0x0u64))) != 0
                {
                    202
                } else {
                    201
                };
            }
            204 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = memory.read_u64(relocate("DAT_1401d0b78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 203;
            }
            205 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_298"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 204;
            }
            206 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) != (0x0u64)))
                    != 0
                {
                    205
                } else {
                    204
                };
            }
            207 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = memory.read_u64(
                        (memory.read_u64((values[0]).wrapping_add((0x1e0u64).wrapping_mul(1)))?)
                            .wrapping_add(0x28u64),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 206;
            }
            208 => {
                {
                    let call_arguments = [relocate("STACK_local_2a0")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 207;
            }
            209 => {
                pc = if (u64::from((values[8]) != (0x0u64))) != 0 {
                    208
                } else {
                    207
                };
            }
            210 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            211 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    210
                } else {
                    209
                };
            }
            212 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_288"))?)?)
                                .wrapping_add(0x38u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_288"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 211;
            }
            213 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 212;
            }
            214 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Content.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 213;
            }
            215 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0xc8cu64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 214;
            }
            216 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = memory.read_u64(relocate("STACK_local_288"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 215;
            }
            217 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            218 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    217
                } else {
                    216
                };
            }
            219 => {
                values[8] = memory.read_u64(relocate("STACK_local_288"))?;
                pc = 218;
            }
            220 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)),
                            )?)?)
                            .wrapping_add(0x58u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)))?,
                        relocate("STACK_local_288"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 219;
            }
            221 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 220;
            }
            222 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 221;
            }
            223 => {
                {
                    let destination = (relocate("STACK_local_2f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x78u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 222;
            }
            224 => {
                {
                    let destination = relocate("STACK_local_288");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 223;
            }
            225 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            226 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    225
                } else {
                    224
                };
            }
            227 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)),
                            )?)?)
                            .wrapping_add(0x88u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x1e8u64).wrapping_mul(1)))?,
                        memory.read_u64(relocate("STACK_local_288"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 226;
            }
            228 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 227;
            }
            229 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 228;
            }
            230 => {
                {
                    let destination = (relocate("STACK_local_2f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0xe7u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 229;
            }
            231 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            232 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    231
                } else {
                    230
                };
            }
            233 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("DAT_1401d0320"))?,
                        memory.read_u64((values[0]).wrapping_add((0x40u64).wrapping_mul(1)))?,
                        relocate("STACK_local_288"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 232;
            }
            234 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 233;
            }
            235 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.WindowsAppSDK.InteractiveExperiences.2.0.13\\\\include\\\\winrt\\\\Microsoft.UI.Interop.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 234;
            }
            236 => {
                {
                    let destination = (relocate("STACK_local_2f8")).wrapping_add(0x0u64);
                    let stored = u64::from((0x5eu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 235;
            }
            237 => {
                {
                    let call_arguments = [relocate("STACK_local_2f8")];
                    invoke("FUN_14000a550", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a550"));
            }
            238 => {
                pc = if (values[4]) != 0 { 237 } else { 236 };
            }
            239 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 238;
            }
            240 => {
                {
                    let destination = relocate("DAT_1401d0350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 239;
            }
            241 => {
                pc = if (values[4]) != 0 { 240 } else { 239 };
            }
            242 => {
                values[4] = u64::from(
                    (u64::from((memory.read_u64(relocate("DAT_1401d0350"))?) == (0x0u64))) != 0,
                );
                pc = 241;
            }
            243 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 242;
            }
            244 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            245 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.WindowsAppSDK.InteractiveExperiences.2.0.13\\\\include\\\\winrt\\\\Microsoft.UI.Interop.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 244;
            }
            246 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x56u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 245;
            }
            247 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 246;
            }
            248 => {
                {
                    let destination = relocate("DAT_1401d0350");
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 247;
            }
            249 => {
                values[11] = memory.read_u64(relocate("DAT_1401d0350"))?;
                pc = 248;
            }
            250 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0350"))?) != (0x0u64))) != 0
                {
                    249
                } else {
                    248
                };
            }
            251 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 250;
            }
            252 => {
                {
                    let call_arguments = [relocate("STACK_local_2f8")];
                    invoke("FUN_14000a550", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a550"));
            }
            253 => {
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
                    252
                } else {
                    251
                };
            }
            254 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 253;
            }
            255 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.WindowsAppSDK.InteractiveExperiences.2.0.13\\\\include\\\\winrt\\\\Microsoft.UI.Interop.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 254;
            }
            256 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x45u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 255;
            }
            257 => {
                {
                    let destination = relocate("_DAT_1401d0348");
                    let stored = {
                        let call_arguments =
                            [values[11], relocate("\"Windowing_GetIconFromIconId\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 256;
            }
            258 => {
                {
                    let destination = relocate("DAT_1401d0340");
                    let stored = {
                        let call_arguments =
                            [values[11], relocate("\"Windowing_GetIconIdFromIcon\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 257;
            }
            259 => {
                {
                    let destination = relocate("DAT_1401d0338");
                    let stored = {
                        let call_arguments = [
                            values[11],
                            relocate("\"Windowing_GetMonitorFromDisplayId\""),
                        ];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 258;
            }
            260 => {
                {
                    let destination = relocate("DAT_1401d0330");
                    let stored = {
                        let call_arguments = [
                            values[11],
                            relocate("\"Windowing_GetDisplayIdFromMonitor\""),
                        ];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 259;
            }
            261 => {
                {
                    let destination = relocate("DAT_1401d0328");
                    let stored = {
                        let call_arguments =
                            [values[11], relocate("\"Windowing_GetWindowFromWindowId\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 260;
            }
            262 => {
                {
                    let destination = relocate("DAT_1401d0320");
                    let stored = {
                        let call_arguments =
                            [values[11], relocate("\"Windowing_GetWindowIdFromWindow\"")];
                        invoke("GetProcAddress", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 261;
            }
            263 => {
                pc = if (u64::from(
                    ((values[4]) != 0)
                        && (({
                            let _ = {
                                values[11] = {
                                    let call_arguments =
                                        [relocate("L\"Microsoft.Internal.FrameworkUdk.dll\"")];
                                    invoke("GetModuleHandleW", &call_arguments, memory)
                                };
                                values[11]
                            };
                            u64::from(
                                ((u64::from((values[11]) != (0x0u64))) != 0)
                                    || (({
                                        let _ = {
                                            values[11] = {
                                                let call_arguments = [relocate(
                                                    "L\"Microsoft.Internal.FrameworkUdk.dll\"",
                                                )];
                                                invoke("LoadLibraryW", &call_arguments, memory)
                                            };
                                            values[11]
                                        };
                                        u64::from((values[11]) != (0x0u64))
                                    }) != 0),
                            )
                        }) != 0),
                )) != 0
                {
                    262
                } else {
                    246
                };
            }
            264 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 263;
            }
            265 => {
                {
                    let destination = relocate("DAT_1401d0350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 264;
            }
            266 => {
                pc = if (values[4]) != 0 { 265 } else { 264 };
            }
            267 => {
                values[4] = u64::from(
                    (u64::from((memory.read_u64(relocate("DAT_1401d0350"))?) == (0x0u64))) != 0,
                );
                pc = 266;
            }
            268 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 267;
            }
            269 => {
                {
                    let destination = relocate("STACK_local_288");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 268;
            }
            270 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x40u64).wrapping_mul(1)))?,
                        values[5],
                    ];
                    invoke("FUN_14004cad0", &call_arguments, memory)
                };
                pc = 269;
            }
            271 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [memory
                            .read_u64((values[0]).wrapping_add((0x1e0u64).wrapping_mul(1)))?];
                        invoke("FUN_1400a3d60", &call_arguments, memory)
                    }) as u8,
                );
                pc = 270;
            }
            272 => {
                {
                    let call_arguments = [values[8], 0x0u64];
                    invoke("CWnd::EnableWindow", &call_arguments, memory)
                };
                pc = 271;
            }
            273 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0xf8u64))?,
                        values[7],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 272;
            }
            274 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    273
                } else {
                    272
                };
            }
            275 => {
                values[8] = values[16];
                pc = 274;
            }
            276 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 275;
            }
            277 => {
                {
                    let destination = (values[0]).wrapping_add((0x1d8u64).wrapping_mul(1));
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 276;
            }
            278 => {
                values[13] = {
                    let call_arguments = [
                        values[8],
                        (0u64.wrapping_sub(0xeu64)) as i32 as i64 as u64,
                        values[12],
                    ];
                    invoke("SetClassLongPtrW", &call_arguments, memory)
                };
                pc = 277;
            }
            279 => {
                values[8] = memory.read_u64((values[0]).wrapping_add((0x40u64).wrapping_mul(1)))?;
                pc = 278;
            }
            280 => {
                pc = if (u64::from((values[0]) != (0x0u64))) != 0 {
                    279
                } else {
                    278
                };
            }
            281 => {
                values[8] = values[16];
                pc = 280;
            }
            282 => {
                values[12] = {
                    let call_arguments = [values[11], relocate("DAT_00000002")];
                    invoke("LoadIconW", &call_arguments, memory)
                };
                pc = 281;
            }
            283 => {
                values[11] = {
                    let call_arguments = [0x0u64];
                    invoke("GetModuleHandleW", &call_arguments, memory)
                };
                pc = 282;
            }
            284 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    283
                };
            }
            285 => {
                {
                    let call_arguments = [values[10], 0x0u64, values[8]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 284;
            }
            286 => {
                values[10] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 285;
            }
            287 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            288 => {
                pc = if (u64::from(
                    (((((values[2]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    287
                } else {
                    284
                };
            }
            289 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    286
                } else {
                    288
                };
            }
            290 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 289;
            }
            291 => {
                {
                    let destination = values[17];
                    let stored = ((((u64::from(memory.read_u32(values[17])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 290;
            }
            292 => {
                values[2] = ((u64::from(memory.read_u32(values[17])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 291;
            }
            293 => {
                values[17] = (values[8]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 292;
            }
            294 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 293;
            }
            295 => {
                pc = if (u64::from(
                    ((values[18]) != 0) && ((u64::from((values[8]) != (0x0u64))) != 0),
                )) != 0
                {
                    294
                } else {
                    284
                };
            }
            296 => {
                values[6] = ({
                    let call_arguments = [
                        values[0],
                        0x0u64,
                        values[19],
                        0x92000000u64,
                        (relocate("STACK_local_278")).wrapping_add((0x4u64).wrapping_mul(1)),
                        0x0u64,
                        0x0u64,
                        0x200000u64,
                        0x0u64,
                    ];
                    invoke("CFrameWnd::Create", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 295;
            }
            297 => {
                values[8] = memory.read_u64(relocate("STACK_local_298"))?;
                pc = 296;
            }
            298 => {
                values[19] = relocate("L\"\"");
                pc = 297;
            }
            299 => {
                values[19] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_298"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                )?;
                pc = 297;
            }
            300 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_298"))?) == (0x0u64)))
                    != 0
                {
                    298
                } else {
                    299
                };
            }
            301 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2e0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[20]) & (0xffffffffu64)))
                        | (0x3u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 300;
            }
            302 => {
                values[18] = 1u64;
                pc = 301;
            }
            303 => {
                values[20] = u64::from((0x4u64) as u32);
                pc = 302;
            }
            304 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_298"),
                        relocate("STACK_local_2a0"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 303;
            }
            305 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 304;
            }
            306 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 305;
            }
            307 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 306;
            }
            308 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 302;
            }
            309 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 308;
            }
            310 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 309;
            }
            311 => {
                values[20] = u64::from((0x1cu64) as u32);
                pc = 310;
            }
            312 => {
                {
                    let destination = relocate("STACK_local_298");
                    let stored = memory.read_u64(relocate("STACK_local_288"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 311;
            }
            313 => {
                {
                    let call_arguments = [values[6], relocate("STACK_local_2f8")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            314 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    313
                } else {
                    312
                };
            }
            315 => {
                values[6] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_2c0"))?,
                        relocate("STACK_local_288"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 314;
            }
            316 => {
                {
                    let destination = relocate("STACK_local_2e8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 315;
            }
            317 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 316;
            }
            318 => {
                {
                    let destination = relocate("STACK_local_2f8");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_2f8")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 317;
            }
            319 => {
                {
                    let destination = relocate("STACK_local_288");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 318;
            }
            320 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    307
                } else {
                    319
                };
            }
            321 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 320;
            }
            322 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 321;
            }
            323 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 322;
            }
            324 => {
                {
                    let destination = relocate("STACK_local_290");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 323;
            }
            325 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = relocate("STACK_local_2c0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 324;
            }
            326 => {
                {
                    let destination = relocate("STACK_local_2c0");
                    let stored = relocate("STACK_local_2b8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 325;
            }
            327 => {
                {
                    let destination = relocate("STACK_local_2a8");
                    let stored = relocate("L\"UntitledLabel\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 326;
            }
            328 => {
                {
                    let destination = relocate("STACK_uStack_2b4");
                    let stored = u64::from((0xdu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 327;
            }
            329 => {
                {
                    let destination = relocate("STACK_local_2b8");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 328;
            }
            330 => {
                pc = if (values[4]) != 0 { 329 } else { 297 };
            }
            331 => {
                values[3] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 330;
            }
            332 => {
                values[4] = u64::from(
                    ({
                        let call_arguments = [values[1]];
                        invoke(
                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 331;
            }
            333 => {
                {
                    let call_arguments = [values[9], relocate("STACK_local_278")];
                    invoke("GetMonitorInfoW", &call_arguments, memory)
                };
                pc = 332;
            }
            334 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x4u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 333;
            }
            335 => {
                {
                    let destination =
                        (relocate("STACK_local_278")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x28u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 334;
            }
            336 => {
                {
                    let destination =
                        (relocate("STACK_local_278")).wrapping_add((0x24u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 335;
            }
            337 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x14u64).wrapping_mul(1)))
                    .wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 336;
            }
            338 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x14u64).wrapping_mul(1)))
                    .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 337;
            }
            339 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x14u64).wrapping_mul(1)))
                    .wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 338;
            }
            340 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x14u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 339;
            }
            341 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x4u64).wrapping_mul(1)))
                    .wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 340;
            }
            342 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x4u64).wrapping_mul(1)))
                    .wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 341;
            }
            343 => {
                {
                    let destination = ((relocate("STACK_local_278"))
                        .wrapping_add((0x4u64).wrapping_mul(1)))
                    .wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 342;
            }
            344 => {
                values[9] = {
                    let call_arguments = [
                        memory.read_u64((values[8]).wrapping_add((0x40u64).wrapping_mul(1)))?,
                        0x2u64,
                    ];
                    invoke("MonitorFromWindow", &call_arguments, memory)
                };
                pc = 343;
            }
            345 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0xf8u64))?,
                        values[7],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 344;
            }
            346 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    345
                } else {
                    344
                };
            }
            347 => {
                values[8] = values[16];
                pc = 346;
            }
            348 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 347;
            }
            349 => {
                {
                    let call_arguments = [memory.read_u64(values[1])?, values[19], 0x104u64];
                    invoke("GetFileTitleW", &call_arguments, memory)
                };
                pc = 348;
            }
            350 => {
                pc = if (u64::from(!((values[4]) != 0))) != 0 {
                    349
                } else {
                    348
                };
            }
            351 => {
                values[4] = u64::from(
                    ({
                        let call_arguments = [values[1]];
                        invoke(
                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 350;
            }
            352 => {
                {
                    let call_arguments = [values[19], 0x0u64, 0x208u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 351;
            }
            353 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_2e0"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 352;
            }
            354 => {
                values[18] = 0u64;
                pc = 353;
            }
            355 => {
                values[16] = 0x0u64;
                pc = 354;
            }
            356 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_348"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 355;
            }
            357 => {
                values[19] = relocate("STACK_local_248");
                pc = 356;
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
    ("FUN_140027f70", 5, fun_140027f70),
    ("FUN_1400285f0", 5, fun_1400285f0),
    ("FUN_14002c190", 1, fun_14002c190),
    ("FUN_140034d90", 2, fun_140034d90),
    ("FUN_1400365d0", 4, fun_1400365d0),
    ("FUN_140044400", 2, fun_140044400),
    ("FUN_14005d180", 3, fun_14005d180),
    ("FUN_140064070", 3, fun_140064070),
    ("FUN_14006f820", 2, fun_14006f820),
    ("FUN_14006feb0", 2, fun_14006feb0),
    ("FUN_140091b50", 2, fun_140091b50),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_140027f70",
        &[
            ("STACK_auStackY_728", -1832),
            ("STACK_local_6e8", -1768),
            ("STACK_p_Stack_6e0", -1760),
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
            ("STACK_p_Stack_610", -1552),
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
        "FUN_1400285f0",
        &[
            ("STACK_auStackY_728", -1832),
            ("STACK_local_6e8", -1768),
            ("STACK_p_Stack_6e0", -1760),
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
            ("STACK_p_Stack_610", -1552),
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
        "FUN_14002c190",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140034d90",
        &[
            ("STACK_auStack_128", -296),
            ("STACK_local_108", -264),
            ("STACK_local_100", -256),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
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
            ("STACK_local_88", -136),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_6c", -108),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_54", -84),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_pcStack_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
        ],
    ),
    (
        "FUN_1400365d0",
        &[
            ("STACK_auStack_b8", -184),
            ("STACK_local_98", -152),
            ("STACK_local_88", -136),
            ("STACK_local_78", -120),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_5c", -92),
            ("STACK_local_58", -88),
            ("STACK_local_48", -72),
        ],
    ),
    (
        "FUN_14005d180",
        &[
            ("STACK_auStack_c8", -200),
            ("STACK_local_a8", -168),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_uStack_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
        ],
    ),
    (
        "FUN_140064070",
        &[
            ("STACK_local_38", -56),
            ("STACK_lStack_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_14006f820",
        &[
            ("STACK_auStack_268", -616),
            ("STACK_local_248", -584),
            ("STACK_local_240", -576),
            ("STACK_local_238", -568),
            ("STACK_local_230", -560),
            ("STACK_local_228", -552),
            ("STACK_local_218", -536),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14006feb0",
        &[
            ("STACK_auStack_b8", -184),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_uStack_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_uStack_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_uStack_38", -56),
            ("STACK_local_30", -48),
            ("STACK_uStack_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_140091b50",
        &[
            ("STACK_auStackY_348", -840),
            ("STACK_local_2f8", -760),
            ("STACK_local_2f0", -752),
            ("STACK_local_2e8", -744),
            ("STACK_local_2e0", -736),
            ("STACK_local_2d8", -728),
            ("STACK_local_2d0", -720),
            ("STACK_local_2c8", -712),
            ("STACK_local_2c0", -704),
            ("STACK_local_2b8", -696),
            ("STACK_uStack_2b4", -692),
            ("STACK_local_2a8", -680),
            ("STACK_local_2a0", -672),
            ("STACK_local_298", -664),
            ("STACK_local_290", -656),
            ("STACK_local_288", -648),
            ("STACK_local_280", -640),
            ("STACK_local_278", -632),
            ("STACK_local_248", -584),
            ("STACK_local_38", -56),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_140027f70",
        &[
            ("STACK_auStackY_728", 32),
            ("STACK_local_6e8", 8),
            ("STACK_p_Stack_6e0", 8),
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
            ("STACK_p_Stack_610", 8),
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
        "FUN_1400285f0",
        &[
            ("STACK_auStackY_728", 32),
            ("STACK_local_6e8", 8),
            ("STACK_p_Stack_6e0", 8),
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
            ("STACK_p_Stack_610", 8),
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
        "FUN_14002c190",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 16),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140034d90",
        &[
            ("STACK_auStack_128", 32),
            ("STACK_local_108", 8),
            ("STACK_local_100", 8),
            ("STACK_local_f8", 4),
            ("STACK_local_f0", 4),
            ("STACK_local_e8", 4),
            ("STACK_local_e0", 4),
            ("STACK_local_d8", 4),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 8),
            ("STACK_local_88", 16),
            ("STACK_local_78", 8),
            ("STACK_local_70", 4),
            ("STACK_local_6c", 4),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 4),
            ("STACK_local_54", 4),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_pcStack_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
        ],
    ),
    (
        "FUN_1400365d0",
        &[
            ("STACK_auStack_b8", 32),
            ("STACK_local_98", 16),
            ("STACK_local_88", 16),
            ("STACK_local_78", 16),
            ("STACK_local_68", 8),
            ("STACK_local_60", 4),
            ("STACK_local_5c", 4),
            ("STACK_local_58", 16),
            ("STACK_local_48", 8),
        ],
    ),
    (
        "FUN_14005d180",
        &[
            ("STACK_auStack_c8", 32),
            ("STACK_local_a8", 8),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_uStack_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
        ],
    ),
    (
        "FUN_140064070",
        &[
            ("STACK_local_38", 8),
            ("STACK_lStack_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_14006f820",
        &[
            ("STACK_auStack_268", 32),
            ("STACK_local_248", 8),
            ("STACK_local_240", 8),
            ("STACK_local_238", 8),
            ("STACK_local_230", 8),
            ("STACK_local_228", 8),
            ("STACK_local_218", 512),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14006feb0",
        &[
            ("STACK_auStack_b8", 40),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_uStack_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_uStack_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_uStack_38", 8),
            ("STACK_local_30", 8),
            ("STACK_uStack_28", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_140091b50",
        &[
            ("STACK_auStackY_348", 32),
            ("STACK_local_2f8", 8),
            ("STACK_local_2f0", 8),
            ("STACK_local_2e8", 8),
            ("STACK_local_2e0", 8),
            ("STACK_local_2d8", 8),
            ("STACK_local_2d0", 8),
            ("STACK_local_2c8", 8),
            ("STACK_local_2c0", 8),
            ("STACK_local_2b8", 4),
            ("STACK_uStack_2b4", 4),
            ("STACK_local_2a8", 8),
            ("STACK_local_2a0", 8),
            ("STACK_local_298", 8),
            ("STACK_local_290", 8),
            ("STACK_local_288", 8),
            ("STACK_local_280", 8),
            ("STACK_local_278", 40),
            ("STACK_local_248", 528),
            ("STACK_local_38", 8),
        ],
    ),
];
