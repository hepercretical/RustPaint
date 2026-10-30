//! Checked translations generated from original bodies by batch 036.
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
/// Translation of `FUN_14000d4c0` (`mspaint.c:16813`).
pub fn fun_14000d4c0(
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
    let mut pc: i32 = 166;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let destination = (values[0]).wrapping_add(0x8u64);
                    let stored = (values[4]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            2 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) < (u64::from((0xdbu64) as u32))))
                    != 0
                {
                    5
                } else {
                    1
                };
            }
            3 => {
                values[8] =
                    u64::from((u64::from(((values[8]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 2;
            }
            4 => {
                values[6] = (values[6]).wrapping_add((0x4u64).wrapping_mul(1));
                pc = 3;
            }
            5 => {
                values[4] = (((values[4])
                    .wrapping_add((u64::from(memory.read_u32(values[6])?)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 4;
            }
            6 => {
                values[6] = (values[6]).wrapping_add(
                    (((values[8]) as i32 as i64 as u64).wrapping_mul(0x4u64)).wrapping_mul(1),
                );
                pc = 5;
            }
            7 => {
                pc = if (u64::from(((values[8]) as i32 as i64 as u64) < (0xdbu64))) != 0 {
                    6
                } else {
                    1
                };
            }
            8 => {
                values[4] =
                    (((((((((((((((values[19]).wrapping_add(values[4])) as i32 as i64 as u64)
                        .wrapping_add(values[21])) as i32 as i64
                        as u64)
                        .wrapping_add(values[17])) as i32 as i64
                        as u64)
                        .wrapping_add(values[20])) as i32 as i64 as u64)
                        .wrapping_add(values[16])) as i32 as i64 as u64)
                        .wrapping_add(values[22])) as i32 as i64 as u64)
                        .wrapping_add(values[18])) as i32 as i64 as u64) as i32
                        as i64 as u64;
                pc = 7;
            }
            9 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) < (u64::from((0xd8u64) as u32))))
                    != 0
                {
                    19
                } else {
                    8
                };
            }
            10 => {
                values[5] = (values[5]).wrapping_add((0x20u64).wrapping_mul(1));
                pc = 9;
            }
            11 => {
                values[22] = (((values[22]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x1cu64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 10;
            }
            12 => {
                values[21] = (((values[21]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x18u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 11;
            }
            13 => {
                values[20] = (((values[20]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x14u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 12;
            }
            14 => {
                values[19] = (((values[19]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x10u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 13;
            }
            15 => {
                values[18] = (((values[18]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0xcu64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                values[17] = (((values[17]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x8u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 15;
            }
            17 => {
                values[16] = (((values[16]).wrapping_add(
                    (u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x4u64).wrapping_mul(1)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 16;
            }
            18 => {
                values[4] = (((values[4])
                    .wrapping_add((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 17;
            }
            19 => {
                values[8] =
                    u64::from((u64::from(((values[8]).wrapping_add(0x8u64)) as u32)) as u32);
                pc = 18;
            }
            20 => {
                values[5] = values[6];
                pc = 19;
            }
            21 => {
                values[22] = (0x0u64) as i32 as i64 as u64;
                pc = 20;
            }
            22 => {
                values[21] = (0x0u64) as i32 as i64 as u64;
                pc = 21;
            }
            23 => {
                values[20] = (0x0u64) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                values[19] = (0x0u64) as i32 as i64 as u64;
                pc = 23;
            }
            25 => {
                values[18] = (0x0u64) as i32 as i64 as u64;
                pc = 24;
            }
            26 => {
                values[17] = (0x0u64) as i32 as i64 as u64;
                pc = 25;
            }
            27 => {
                values[16] = (0x0u64) as i32 as i64 as u64;
                pc = 26;
            }
            28 => {
                values[4] = (0x0u64) as i32 as i64 as u64;
                pc = 27;
            }
            29 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) < (u64::from((0x90u64) as u32))))
                    != 0
                {
                    50
                } else {
                    28
                };
            }
            30 => {
                values[11] = (values[11]).wrapping_add((0x5u64).wrapping_mul(1));
                pc = 29;
            }
            31 => {
                {
                    let destination = (values[11]).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from(
                        ((float_to_integer(f64::from_bits(values[15]), 64, true)?) as i8 as i64
                            as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 30;
            }
            32 => {
                values[9] =
                    u64::from((u64::from(((values[9]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 31;
            }
            33 => {
                values[15] = (f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(values[15]))
                            - (f64::from_bits(
                                ((f64::from_bits(
                                    ((((float_to_integer(f64::from_bits(values[14]), 64, true)?)
                                        & (0xffffffffu64))
                                        as i64) as f64)
                                        .to_bits(),
                                )) * (f64::from_bits(0x3e70000000000000u64)))
                                .to_bits(),
                            )))
                        .to_bits(),
                    )) * (f64::from_bits(0x41f0000000000000u64)))
                    .to_bits(),
                ))
                .floor()
                .to_bits();
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[11]).wrapping_add((0x3u64).wrapping_mul(1));
                    let stored = u64::from(
                        ((float_to_integer(f64::from_bits(values[14]), 64, true)?) as i8 as i64
                            as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 33;
            }
            35 => {
                values[14] = (f64::from_bits(
                    ((f64::from_bits(values[15])) * (f64::from_bits(0x4170000000000000u64)))
                        .to_bits(),
                ))
                .floor()
                .to_bits();
                pc = 34;
            }
            36 => {
                values[15] = ((f64::from_bits(values[15]))
                    - (f64::from_bits(
                        ((f64::from_bits(
                            ((((float_to_integer(f64::from_bits(values[14]), 64, true)?)
                                & (0xffffffffu64)) as i64) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(0x3ef0000000000000u64)))
                        .to_bits(),
                    )))
                .to_bits();
                pc = 35;
            }
            37 => {
                {
                    let destination = (values[11]).wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = u64::from(
                        ((float_to_integer(f64::from_bits(values[14]), 64, true)?) as i8 as i64
                            as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 36;
            }
            38 => {
                values[14] = (f64::from_bits(
                    ((f64::from_bits(values[15])) * (f64::from_bits(0x40f0000000000000u64)))
                        .to_bits(),
                ))
                .floor()
                .to_bits();
                pc = 37;
            }
            39 => {
                values[15] = ((f64::from_bits(values[15]))
                    - (f64::from_bits(
                        ((f64::from_bits(
                            ((((float_to_integer(f64::from_bits(values[14]), 64, true)?)
                                & (0xffffffffu64)) as i64) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(0x3f70000000000000u64)))
                        .to_bits(),
                    )))
                .to_bits();
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[11]).wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = u64::from(
                        ((float_to_integer(f64::from_bits(values[14]), 64, true)?) as i8 as i64
                            as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 39;
            }
            41 => {
                values[14] = (f64::from_bits(
                    ((f64::from_bits(values[15])) * (f64::from_bits(0x4070000000000000u64)))
                        .to_bits(),
                ))
                .floor()
                .to_bits();
                pc = 40;
            }
            42 => {
                values[15] = ((f64::from_bits(values[15]))
                    - (f64::from_bits(
                        (((float_to_integer(f64::from_bits(values[14]), 32, true)?) as i64) as f64)
                            .to_bits(),
                    )))
                .to_bits();
                pc = 41;
            }
            43 => {
                {
                    let destination = values[11];
                    let stored = u64::from(
                        ((float_to_integer(f64::from_bits(values[14]), 32, true)?) as i8 as i64
                            as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 42;
            }
            44 => {
                values[14] = (f64::from_bits(values[15])).floor().to_bits();
                pc = 43;
            }
            45 => {
                values[15] = ((f64::from_bits(
                    memory
                        .read_u64(((values[0]).wrapping_add(0x39cu64)).wrapping_add(
                            ((values[7]) as i32 as i64 as u64).wrapping_mul(0x8u64),
                        ))?,
                )) / (f64::from_bits(values[1])))
                .to_bits();
                pc = 44;
            }
            46 => {
                values[7] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                ({
                                    let numerator = (0x90u64) as u32;
                                    let denominator = (u64::from((values[3]) as u32)) as u32;
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_div(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )? as u64
                                }) as u32,
                            ))
                            .wrapping_mul(
                                ({
                                    let numerator = ((values[9]) as i32 as i64 as u64) as i32;
                                    let denominator = ((u64::from((values[3]) as u32)) as i32 as i64
                                        as u64)
                                        as i32;
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )? as u64
                                }) as i32 as i64 as u64,
                            )) as u32,
                        ))
                        .wrapping_add(
                            ({
                                let numerator = ((values[9]) as i32 as i64 as u64) as i32;
                                let denominator =
                                    ((u64::from((values[3]) as u32)) as i32 as i64 as u64) as i32;
                                if denominator == 0 {
                                    return Err(AdapterError::Arithmetic("division by zero"));
                                }
                                numerator
                                    .checked_div(denominator)
                                    .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                    as u64
                            }) as i32 as i64 as u64,
                        )) as u32,
                    )) as u32,
                );
                pc = 45;
            }
            47 => {
                values[3] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("DAT_1401063e8")).wrapping_add((values[3]).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 46;
            }
            48 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    47
                } else {
                    45
                };
            }
            49 => {
                values[7] = u64::from((values[9]) as u32);
                pc = 48;
            }
            50 => {
                values[3] = u64::from(
                    (((u64::from(
                        (u64::from(memory.read_u8((values[0]).wrapping_add(0x31u64))?)) as u8,
                    )) & (0x7u64)) as i32 as i64 as u64) as u8,
                );
                pc = 49;
            }
            51 => {
                values[9] = u64::from((0x0u64) as u32);
                pc = 50;
            }
            52 => {
                values[1] = ((f64::from_bits(0x3ff0000000000000u64))
                    / (f64::from_bits(
                        (((((0x1u64 as i32).wrapping_shl(
                            ((((0u64.wrapping_sub(values[3])) as i32 as i64 as u64) & (0x3fu64))
                                as i32 as i64 as u64) as u32,
                        ) as u64) as i32 as i64 as u64) as i64) as f64)
                            .to_bits(),
                    )))
                .to_bits();
                pc = 51;
            }
            53 => {
                values[1] = (((((0x1u64 as i32)
                    .wrapping_shl((((values[3]) & (0x3fu64)) as i32 as i64 as u64) as u32)
                    as u64) as i32 as i64 as u64) as i64) as f64)
                    .to_bits();
                pc = 51;
            }
            54 => {
                pc = if (u64::from(
                    ((((values[3]) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    52
                } else {
                    53
                };
            }
            55 => {
                {
                    let destination = values[6];
                    let stored = u64::from((values[3]) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 54;
            }
            56 => {
                values[3] = u64::from((0xc2u64) as u8);
                pc = 55;
            }
            57 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < ((((0u64.wrapping_sub(0x3eu64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    56
                } else {
                    55
                };
            }
            58 => {
                values[3] = u64::from((u64::from((values[4]) as u8)) as u8);
                pc = 57;
            }
            59 => {
                values[3] = u64::from((0x3eu64) as u8);
                pc = 55;
            }
            60 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x3fu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    58
                } else {
                    59
                };
            }
            61 => {
                pc = if (u64::from(
                    (f64::from_bits(values[14])) < (f64::from_bits(0x403f000000000000u64)),
                )) != 0
                {
                    63
                } else {
                    60
                };
            }
            62 => {
                let _ = {
                    values[14] =
                        ((f64::from_bits(values[14])) + (f64::from_bits(values[14]))).to_bits();
                    values[14]
                };
                pc = 61;
            }
            63 => {
                values[4] = (((values[4])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 62;
            }
            64 => {
                pc = if (u64::from(
                    (f64::from_bits(0x405fc00000000000u64)) < (f64::from_bits(values[14])),
                )) != 0
                {
                    66
                } else {
                    61
                };
            }
            65 => {
                let _ = {
                    values[14] = ((f64::from_bits(values[14]))
                        * (f64::from_bits(0x3fe0000000000000u64)))
                    .to_bits();
                    values[14]
                };
                pc = 64;
            }
            66 => {
                values[4] =
                    (((values[4]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 65;
            }
            67 => {
                values[4] = (0x0u64) as i32 as i64 as u64;
                pc = 64;
            }
            68 => {
                pc = if (u64::from((f64::from_bits(values[14])) != (f64::from_bits(0x0u64)))) != 0 {
                    67
                } else {
                    51
                };
            }
            69 => {
                {
                    let destination = values[6];
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 68;
            }
            70 => {
                values[6] = (values[0]).wrapping_add(0x30u64);
                pc = 69;
            }
            71 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) < (u64::from((0x90u64) as u32))))
                    != 0
                {
                    79
                } else {
                    70
                };
            }
            72 => {
                values[15] = values[14];
                pc = 71;
            }
            73 => {
                values[12] = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 72;
            }
            74 => {
                values[9] =
                    u64::from((u64::from(((values[9]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 73;
            }
            75 => {
                values[14] = values[15];
                pc = 74;
            }
            76 => {
                pc = if (u64::from((f64::from_bits(values[14])) <= (f64::from_bits(values[15]))))
                    != 0
                {
                    75
                } else {
                    74
                };
            }
            77 => {
                values[14] = (-(f64::from_bits(values[14]))).to_bits();
                pc = 76;
            }
            78 => {
                pc = if (u64::from((f64::from_bits(values[14])) < (f64::from_bits(0x0u64)))) != 0 {
                    77
                } else {
                    76
                };
            }
            79 => {
                values[14] = memory.read_u64(values[12])?;
                pc = 78;
            }
            80 => {
                values[15] = 0x0u64;
                pc = 79;
            }
            81 => {
                values[9] = u64::from((0x0u64) as u32);
                pc = 80;
            }
            82 => {
                pc = if (u64::from((u64::from((values[9]) as u32)) < (u64::from((0x90u64) as u32))))
                    != 0
                {
                    98
                } else {
                    81
                };
            }
            83 => {
                values[13] = (values[13]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 82;
            }
            84 => {
                {
                    let destination = values[13];
                    let stored = u64::from(((values[7]) as i8 as i64 as u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = ((values[0]).wrapping_add(0x81cu64))
                        .wrapping_add((values[10]).wrapping_mul(0x8u64));
                    let stored = ((u64::from(((values[7]) & (0xffu64)) as u32)) as f64).to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                values[7] = u64::from(
                    (float_to_integer(
                        f64::from_bits(
                            ((f64::from_bits(
                                ((f64::from_bits(values[15]))
                                    + (f64::from_bits(0x3fe0000000000000u64)))
                                .to_bits(),
                            )) - (f64::from_bits(values[14])))
                            .to_bits(),
                        ),
                        32,
                        false,
                    )?) as u32,
                );
                pc = 85;
            }
            87 => {
                {
                    let destination = ((values[0]).wrapping_add(0x39cu64))
                        .wrapping_add((values[10]).wrapping_mul(0x8u64));
                    let stored = ((f64::from_bits(
                        memory.read_u64(
                            ((values[0]).wrapping_add(0x39cu64))
                                .wrapping_add((values[10]).wrapping_mul(0x8u64)),
                        )?,
                    )) - (f64::from_bits(values[14])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                values[9] =
                    u64::from((u64::from(((values[9]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 87;
            }
            89 => {
                values[14] = ((f64::from_bits(values[15]))
                    - (f64::from_bits(0x406fe00000000000u64)))
                .to_bits();
                pc = 88;
            }
            90 => {
                values[14] = ((f64::from_bits(values[15]))
                    % (f64::from_bits(0x3ff0000000000000u64)))
                .to_bits();
                pc = 88;
            }
            91 => {
                pc = if (u64::from(
                    ((u64::from((f64::from_bits(values[15])) < (f64::from_bits(0x0u64)))) != 0)
                        || ((u64::from(
                            (f64::from_bits(0x406fe00000000000u64)) < (f64::from_bits(values[15])),
                        )) != 0),
                )) != 0
                {
                    89
                } else {
                    90
                };
            }
            92 => {
                values[15] = memory.read_u64(
                    ((values[0]).wrapping_add(0x81cu64))
                        .wrapping_add((values[10]).wrapping_mul(0x8u64)),
                )?;
                pc = 91;
            }
            93 => {
                values[10] = (values[7]) as i32 as i64 as u64;
                pc = 92;
            }
            94 => {
                values[7] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                ({
                                    let numerator = (0x90u64) as u32;
                                    let denominator = (u64::from((values[3]) as u32)) as u32;
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_div(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )? as u64
                                }) as u32,
                            ))
                            .wrapping_mul(
                                ({
                                    let numerator = ((values[9]) as i32 as i64 as u64) as i32;
                                    let denominator = ((u64::from((values[3]) as u32)) as i32 as i64
                                        as u64)
                                        as i32;
                                    if denominator == 0 {
                                        return Err(AdapterError::Arithmetic("division by zero"));
                                    }
                                    numerator.checked_rem(denominator).ok_or(
                                        AdapterError::Arithmetic("signed division overflow"),
                                    )? as u64
                                }) as i32 as i64 as u64,
                            )) as u32,
                        ))
                        .wrapping_add(
                            ({
                                let numerator = ((values[9]) as i32 as i64 as u64) as i32;
                                let denominator =
                                    ((u64::from((values[3]) as u32)) as i32 as i64 as u64) as i32;
                                if denominator == 0 {
                                    return Err(AdapterError::Arithmetic("division by zero"));
                                }
                                numerator
                                    .checked_div(denominator)
                                    .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                    as u64
                            }) as i32 as i64 as u64,
                        )) as u32,
                    )) as u32,
                );
                pc = 93;
            }
            95 => {
                values[3] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u8(
                            (relocate("DAT_1401063e8")).wrapping_add((values[3]).wrapping_mul(1)),
                        )?)) as u8,
                    )) as u8,
                );
                pc = 94;
            }
            96 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    95
                } else {
                    93
                };
            }
            97 => {
                values[7] = u64::from((values[9]) as u32);
                pc = 96;
            }
            98 => {
                values[3] = u64::from(
                    (((u64::from(
                        (u64::from(memory.read_u8((values[0]).wrapping_add(0x31u64))?)) as u8,
                    )) & (0x7u64)) as i32 as i64 as u64) as u8,
                );
                pc = 97;
            }
            99 => {
                values[9] = u64::from((0x0u64) as u32);
                pc = 98;
            }
            100 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (((values[2]).wrapping_add(
                        (u64::from(memory.read_u8((values[0]).wrapping_add(0x3bbu64))?)) as i8
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 99;
            }
            101 => {
                values[8] = u64::from((0x0u64) as u32);
                pc = 100;
            }
            102 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 101;
            }
            103 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3bau64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 102;
            }
            104 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 103;
            }
            105 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b9u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 105;
            }
            107 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b8u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 106;
            }
            108 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 107;
            }
            109 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b7u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 108;
            }
            110 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 109;
            }
            111 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b6u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 110;
            }
            112 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 111;
            }
            113 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b5u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 112;
            }
            114 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 113;
            }
            115 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b4u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 114;
            }
            116 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 115;
            }
            117 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b3u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 116;
            }
            118 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 117;
            }
            119 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b2u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 118;
            }
            120 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 119;
            }
            121 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b1u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 120;
            }
            122 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 121;
            }
            123 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3b0u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 122;
            }
            124 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 123;
            }
            125 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3afu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 124;
            }
            126 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 125;
            }
            127 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3aeu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 126;
            }
            128 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 127;
            }
            129 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3adu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 128;
            }
            130 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 129;
            }
            131 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3acu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 130;
            }
            132 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 131;
            }
            133 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3abu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 132;
            }
            134 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 133;
            }
            135 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3aau64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 134;
            }
            136 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 135;
            }
            137 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a9u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 136;
            }
            138 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 137;
            }
            139 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a8u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 138;
            }
            140 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 139;
            }
            141 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a7u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 140;
            }
            142 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 141;
            }
            143 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a6u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 142;
            }
            144 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 143;
            }
            145 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a5u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 144;
            }
            146 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 145;
            }
            147 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a4u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 146;
            }
            148 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 147;
            }
            149 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a3u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 148;
            }
            150 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 149;
            }
            151 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a2u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 150;
            }
            152 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 151;
            }
            153 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a1u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 152;
            }
            154 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 153;
            }
            155 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x3a0u64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 154;
            }
            156 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 155;
            }
            157 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x39fu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 156;
            }
            158 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 157;
            }
            159 => {
                values[2] = (((values[2]).wrapping_add(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x39eu64))?)) as i8 as i64
                        as u64,
                )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 158;
            }
            160 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = (values[2]) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 159;
            }
            161 => {
                values[2] = ((((u64::from(memory.read_u8(values[12])?)) as i8 as i64 as u64)
                    .wrapping_add(
                        (u64::from(memory.read_u8((values[0]).wrapping_add(0x39du64))?)) as i8
                            as i64 as u64,
                    )) as i32 as i64 as u64) as i8 as i64 as u64;
                pc = 160;
            }
            162 => {
                {
                    let destination = (values[0]).wrapping_add(0x31u64);
                    let stored = ((u64::from(memory.read_u8(values[12])?)) as i8 as i64 as u64)
                        as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 161;
            }
            163 => {
                values[1] = 0x0u64;
                pc = 162;
            }
            164 => {
                values[11] = (values[0]).wrapping_add(0xccu64);
                pc = 163;
            }
            165 => {
                values[13] = (values[0]).wrapping_add(0x3cu64);
                pc = 164;
            }
            166 => {
                values[12] = (values[0]).wrapping_add(0x39cu64);
                pc = 165;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14000d9e0` (`mspaint.c:17034`).
pub fn fun_14000d9e0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
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
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
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
    let mut pc: i32 = 284;
    loop {
        match pc {
            0 => {
                return Ok(Some(0xffffe4a2u64));
            }
            1 => {
                return Ok(Some(0x0u64));
            }
            2 => {
                {
                    let destination = (values[0]).wrapping_add(0x16c0u64);
                    let stored = (values[2]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = (values[0]).wrapping_add(0x16bcu64);
                    let stored = (values[1]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add(0x16b4u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    (u64::from((values[7]) as u32)) < (u64::from((0x120u64) as u32)),
                )) != 0
                {
                    15
                } else {
                    4
                };
            }
            6 => {
                values[20] = (values[20]).wrapping_add((0x8u64).wrapping_mul(8));
                pc = 5;
            }
            7 => {
                {
                    let destination = (values[20]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((f64::from_bits(
                        memory.read_u64((values[20]).wrapping_add((0x3u64).wrapping_mul(8)))?,
                    )) / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = (values[20]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = ((f64::from_bits(
                        memory.read_u64((values[20]).wrapping_add((0x2u64).wrapping_mul(8)))?,
                    )) / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[20]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = ((f64::from_bits(
                        memory.read_u64((values[20]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                    )) / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = (values[20]).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((f64::from_bits(
                        memory.read_u64((values[20]).wrapping_add((0x0u64).wrapping_mul(8)))?,
                    )) / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x2u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x2u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x3u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x3u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[20]).wrapping_add(
                        ((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        / (f64::from_bits(values[34])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[7] =
                    u64::from((u64::from(((values[7]).wrapping_add(0x8u64)) as u32)) as u32);
                pc = 14;
            }
            16 => {
                values[20] = (values[0]).wrapping_add(0x1a64u64);
                pc = 15;
            }
            17 => {
                values[41] = u64::from(
                    (u64::from(
                        ((float_to_integer(f64::from_bits(values[34]), 64, false)?)
                            .wrapping_shr((0x20u64) as u32)) as u32,
                    )) as u32,
                );
                pc = 16;
            }
            18 => {
                values[40] = u64::from((u64::from((values[34]) as u32)) as u32);
                pc = 17;
            }
            19 => {
                values[34] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(values[34])) * (f64::from_bits(0x3ff07b3705abb25cu64)))
                            .to_bits(),
                    )) * (f64::from_bits(values[37])))
                    .to_bits(),
                )) * (f64::from_bits(0x4008101010101010u64)))
                .to_bits();
                pc = 18;
            }
            20 => {
                values[7] = u64::from((0x0u64) as u32);
                pc = 19;
            }
            21 => {
                pc = if (u64::from(
                    ((((values[47]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x19u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    147
                } else {
                    20
                };
            }
            22 => {
                values[27] = (values[27]).wrapping_add((0x68u64).wrapping_mul(8));
                pc = 21;
            }
            23 => {
                values[47] =
                    u64::from((u64::from(((values[47]).wrapping_add(0x4u64)) as u32)) as u32);
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_res20"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((0x19u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    145
                } else {
                    23
                };
            }
            25 => {
                values[26] = (values[26]).wrapping_add((0x4u64).wrapping_mul(8));
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_res20"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(0x4u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_108"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    142
                } else {
                    26
                };
            }
            28 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = (memory.read_u64(relocate("STACK_local_108"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                values[21] = (values[21]).wrapping_add((0x1au64).wrapping_mul(8));
                pc = 28;
            }
            30 => {
                values[48] =
                    (((values[48]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 29;
            }
            31 => {
                pc = if (u64::from(((values[19]) as i64) != ((0x0u64) as i64))) != 0 {
                    137
                } else {
                    30
                };
            }
            32 => {
                values[7] = u64::from((values[8]) as u32);
                pc = 31;
            }
            33 => {
                values[19] =
                    (values[19]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 32;
            }
            34 => {
                values[12] = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 33;
            }
            35 => {
                values[14] =
                    (((values[14]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[11]).wrapping_add(values[6])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[43]))
                            * (f64::from_bits(
                                (((values[41]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                                    | ((values[40]) & (0xffffffffu64)),
                            )))
                        .to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[11]).wrapping_add(values[6])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[6])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[43])) * (f64::from_bits(values[46]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[6])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                values[11] = (0x3u64) as i32 as i64 as u64;
                pc = 37;
            }
            39 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    38
                } else {
                    37
                };
            }
            40 => {
                values[11] = (0x93u64) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[11]).wrapping_add(values[6])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[43])) * (f64::from_bits(values[45]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[11]).wrapping_add(values[6])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                values[9] = (0x2u64) as i32 as i64 as u64;
                pc = 41;
            }
            43 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                values[9] = (0x92u64) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[3]).wrapping_add(
                        ((u64::from(((values[6]).wrapping_add(values[8])) as u32)) as i32 as i64
                            as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[43])) * (f64::from_bits(values[5]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                ((u64::from(((values[6]).wrapping_add(values[8])) as u32)) as i32
                                    as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                values[11] = (0x1u64) as i32 as i64 as u64;
                pc = 45;
            }
            47 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    46
                } else {
                    45
                };
            }
            48 => {
                values[11] = (0x91u64) as i32 as i64 as u64;
                pc = 47;
            }
            49 => {
                values[6] = (((((((values[6])
                    .wrapping_add(((values[11]).wrapping_mul(0x6u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64)
                    .wrapping_mul(0x4u64)) as i32 as i64 as u64)
                    .wrapping_add(0x1cu64)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 48;
            }
            50 => {
                values[43] = (-(f64::from_bits(values[43]))).to_bits();
                pc = 49;
            }
            51 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    50
                } else {
                    49
                };
            }
            52 => {
                values[43] =
                    ((f64::from_bits(values[43])) * (f64::from_bits(values[44]))).to_bits();
                pc = 51;
            }
            53 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    52
                } else {
                    35
                };
            }
            54 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31]))
                            * (f64::from_bits(
                                (((values[41]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                                    | ((values[40]) & (0xffffffffu64)),
                            )))
                        .to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[10]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[46]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[10]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                values[9] = (0x3u64) as i32 as i64 as u64;
                pc = 55;
            }
            57 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    56
                } else {
                    55
                };
            }
            58 => {
                values[9] = (0x93u64) as i32 as i64 as u64;
                pc = 57;
            }
            59 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[45]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                values[10] = (0x2u64) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    60
                } else {
                    59
                };
            }
            62 => {
                values[10] = (0x92u64) as i32 as i64 as u64;
                pc = 61;
            }
            63 => {
                {
                    let destination = (values[3]).wrapping_add(
                        ((u64::from(((values[15]).wrapping_add(values[8])) as u32)) as i32 as i64
                            as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[5]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                ((u64::from(((values[15]).wrapping_add(values[8])) as u32)) as i32
                                    as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                values[9] = (0x1u64) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    64
                } else {
                    63
                };
            }
            66 => {
                values[9] = (0x91u64) as i32 as i64 as u64;
                pc = 65;
            }
            67 => {
                values[15] = (((((values[6]).wrapping_add(
                    ((((values[11]).wrapping_add(0x1u64)) as i32 as i64 as u64)
                        .wrapping_mul(0x6u64)) as i32 as i64 as u64,
                )) as i32 as i64 as u64)
                    .wrapping_mul(0x4u64)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 66;
            }
            68 => {
                values[31] = (-(f64::from_bits(values[31]))).to_bits();
                pc = 67;
            }
            69 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    68
                } else {
                    67
                };
            }
            70 => {
                values[31] = ((f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[43])))
                        .to_bits(),
                )) * (f64::from_bits(values[44])))
                .to_bits();
                pc = 69;
            }
            71 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[6]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    70
                } else {
                    53
                };
            }
            72 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    71
                } else {
                    35
                };
            }
            73 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31]))
                            * (f64::from_bits(
                                (((values[41]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                                    | ((values[40]) & (0xffffffffu64)),
                            )))
                        .to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                values[9] = (0x7u64) as i32 as i64 as u64;
                pc = 73;
            }
            75 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    74
                } else {
                    73
                };
            }
            76 => {
                values[9] = (0x97u64) as i32 as i64 as u64;
                pc = 75;
            }
            77 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[46]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                values[9] = (0x6u64) as i32 as i64 as u64;
                pc = 77;
            }
            79 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    78
                } else {
                    77
                };
            }
            80 => {
                values[9] = (0x96u64) as i32 as i64 as u64;
                pc = 79;
            }
            81 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[45]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                values[9] = (0x5u64) as i32 as i64 as u64;
                pc = 81;
            }
            83 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    82
                } else {
                    81
                };
            }
            84 => {
                values[9] = (0x95u64) as i32 as i64 as u64;
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[5]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                values[9] = (0x4u64) as i32 as i64 as u64;
                pc = 85;
            }
            87 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    86
                } else {
                    85
                };
            }
            88 => {
                values[9] = (0x94u64) as i32 as i64 as u64;
                pc = 87;
            }
            89 => {
                values[15] = (((((values[6])
                    .wrapping_add(((values[11]).wrapping_mul(0x6u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64)
                    .wrapping_mul(0x4u64)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 88;
            }
            90 => {
                values[31] = (-(f64::from_bits(values[31]))).to_bits();
                pc = 89;
            }
            91 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    90
                } else {
                    89
                };
            }
            92 => {
                values[31] = ((f64::from_bits(values[43]))
                    * (f64::from_bits(
                        ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[44])))
                            .to_bits(),
                    )))
                .to_bits();
                pc = 91;
            }
            93 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    92
                } else {
                    72
                };
            }
            94 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31]))
                            * (f64::from_bits(
                                (((values[41]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                                    | ((values[40]) & (0xffffffffu64)),
                            )))
                        .to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                values[9] = (0x3u64) as i32 as i64 as u64;
                pc = 94;
            }
            96 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    95
                } else {
                    94
                };
            }
            97 => {
                values[9] = (0x93u64) as i32 as i64 as u64;
                pc = 96;
            }
            98 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[46]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                values[9] = (0x2u64) as i32 as i64 as u64;
                pc = 98;
            }
            100 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    99
                } else {
                    98
                };
            }
            101 => {
                values[9] = (0x92u64) as i32 as i64 as u64;
                pc = 100;
            }
            102 => {
                {
                    let destination = (values[3]).wrapping_add(
                        (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[45]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                (((values[9]).wrapping_add(values[15])) as i32 as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                values[9] = (0x1u64) as i32 as i64 as u64;
                pc = 102;
            }
            104 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    103
                } else {
                    102
                };
            }
            105 => {
                values[9] = (0x91u64) as i32 as i64 as u64;
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[3]).wrapping_add(
                        ((u64::from(((values[15]).wrapping_add(values[8])) as u32)) as i32 as i64
                            as u64)
                            .wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(
                        ((f64::from_bits(values[31])) * (f64::from_bits(values[5]))).to_bits(),
                    )) + (f64::from_bits(
                        memory.read_u64(
                            (values[3]).wrapping_add(
                                ((u64::from(((values[15]).wrapping_add(values[8])) as u32)) as i32
                                    as i64 as u64)
                                    .wrapping_mul(8),
                            ),
                        )?,
                    )))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 105;
            }
            107 => {
                values[15] = (((((values[6])
                    .wrapping_add(((values[11]).wrapping_mul(0x6u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64)
                    .wrapping_mul(0x4u64)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 106;
            }
            108 => {
                values[31] = (-(f64::from_bits(values[31]))).to_bits();
                pc = 107;
            }
            109 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    108
                } else {
                    107
                };
            }
            110 => {
                values[31] = ((f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[43])))
                        .to_bits(),
                )) * (f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[44])))
                        .to_bits(),
                )))
                .to_bits();
                pc = 109;
            }
            111 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[6]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    110
                } else {
                    93
                };
            }
            112 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[11]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    111
                } else {
                    72
                };
            }
            113 => {
                values[43] = ((f64::from_bits(values[43]))
                    - (f64::from_bits((((values[6]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 112;
            }
            114 => {
                values[44] = ((f64::from_bits(values[44]))
                    - (f64::from_bits((((values[11]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 113;
            }
            115 => {
                values[6] =
                    (float_to_integer(f64::from_bits(values[31]), 32, true)?) as i32 as i64 as u64;
                pc = 114;
            }
            116 => {
                values[31] = (f64::from_bits(values[43])).floor().to_bits();
                pc = 115;
            }
            117 => {
                values[11] =
                    (float_to_integer(f64::from_bits(values[31]), 32, true)?) as i32 as i64 as u64;
                pc = 116;
            }
            118 => {
                values[31] = (f64::from_bits(values[44])).floor().to_bits();
                pc = 117;
            }
            119 => {
                values[43] = ((f64::from_bits(
                    ((f64::from_bits((((values[14]) as i64) as f64).to_bits()))
                        - (f64::from_bits(0x4004000000000000u64)))
                    .to_bits(),
                )) * (f64::from_bits(0x3fd0000000000000u64)))
                .to_bits();
                pc = 118;
            }
            120 => {
                values[44] = ((f64::from_bits(
                    ((f64::from_bits(values[42])) - (f64::from_bits(0x4004000000000000u64)))
                        .to_bits(),
                )) * (f64::from_bits(0x3fd0000000000000u64)))
                .to_bits();
                pc = 119;
            }
            121 => {
                values[41] = u64::from(
                    (u64::from(
                        (((-(f64::from_bits(values[43]))).to_bits()).wrapping_shr((0x20u64) as u32))
                            as u32,
                    )) as u32,
                );
                pc = 120;
            }
            122 => {
                values[40] = u64::from((u64::from((values[43]) as u32)) as u32);
                pc = 121;
            }
            123 => {
                values[46] = 0x0u64;
                pc = 122;
            }
            124 => {
                values[46] = values[43];
                pc = 120;
            }
            125 => {
                values[41] = u64::from((0x0u64) as u32);
                pc = 124;
            }
            126 => {
                values[40] = u64::from((0x0u64) as u32);
                pc = 125;
            }
            127 => {
                pc = if (u64::from((f64::from_bits(values[43])) <= (f64::from_bits(0x0u64)))) != 0 {
                    123
                } else {
                    126
                };
            }
            128 => {
                values[45] = (-(f64::from_bits(values[46]))).to_bits();
                pc = 127;
            }
            129 => {
                values[5] = 0x0u64;
                pc = 128;
            }
            130 => {
                values[5] = values[46];
                pc = 127;
            }
            131 => {
                values[45] = 0x0u64;
                pc = 130;
            }
            132 => {
                pc = if (u64::from((f64::from_bits(values[46])) <= (f64::from_bits(0x0u64)))) != 0 {
                    129
                } else {
                    131
                };
            }
            133 => {
                values[8] = u64::from((0x0u64) as u32);
                pc = 132;
            }
            134 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    133
                } else {
                    132
                };
            }
            135 => {
                values[8] = u64::from((0x90u64) as u32);
                pc = 134;
            }
            136 => {
                values[43] = ((f64::from_bits(memory.read_u64((values[12]).wrapping_add(
                    ((0u64.wrapping_sub(0x34u64)) as i32 as i64 as u64).wrapping_mul(8),
                ))?))
                    - (f64::from_bits(memory.read_u64(values[12])?)))
                .to_bits();
                pc = 135;
            }
            137 => {
                values[46] = ((f64::from_bits(memory.read_u64((values[12]).wrapping_add(
                    ((0u64.wrapping_sub(0x1bu64)) as i32 as i64 as u64).wrapping_mul(8),
                ))?))
                    - (f64::from_bits(memory.read_u64((values[12]).wrapping_add(
                        ((0u64.wrapping_sub(0x19u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?)))
                .to_bits();
                pc = 136;
            }
            138 => {
                values[7] = u64::from((values[8]) as u32);
                pc = 137;
            }
            139 => {
                values[14] = ((u64::from(memory.read_u32(relocate("STACK_local_res20"))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 138;
            }
            140 => {
                values[12] = values[21];
                pc = 139;
            }
            141 => {
                values[19] = 0x4u64;
                pc = 140;
            }
            142 => {
                values[42] = (((values[48]) as i64) as f64).to_bits();
                pc = 141;
            }
            143 => {
                values[48] = (values[47]) as i32 as i64 as u64;
                pc = 142;
            }
            144 => {
                values[21] = values[26];
                pc = 143;
            }
            145 => {
                {
                    let destination = relocate("STACK_local_108");
                    let stored = 0x4u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 144;
            }
            146 => {
                values[26] = values[27];
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = (0x1u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 146;
            }
            148 => {
                values[47] = u64::from((0x1u64) as u32);
                pc = 147;
            }
            149 => {
                {
                    let destination = (values[0]).wrapping_add(0x38u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 148;
            }
            150 => {
                values[27] = (values[0]).wrapping_add(0x270u64);
                pc = 149;
            }
            151 => {
                pc = if (u64::from(
                    (u64::from((values[7]) as u32)) < (u64::from((0x120u64) as u32)),
                )) != 0
                {
                    154
                } else {
                    150
                };
            }
            152 => {
                values[13] = (values[13]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 151;
            }
            153 => {
                {
                    let destination = values[13];
                    let stored = (((0x0u64) as i64) as f64).to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                values[7] =
                    u64::from((u64::from(((values[7]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 153;
            }
            155 => {
                values[7] = u64::from((values[8]) as u32);
                pc = 154;
            }
            156 => {
                values[13] = values[3];
                pc = 155;
            }
            157 => {
                values[3] = (values[0]).wrapping_add(0x1a44u64);
                pc = 156;
            }
            158 => {
                values[8] = u64::from((0x0u64) as u32);
                pc = 157;
            }
            159 => {
                pc = if (u64::from(
                    ((((values[47]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x1au64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    247
                } else {
                    158
                };
            }
            160 => {
                values[18] = values[47];
                pc = 159;
            }
            161 => {
                values[47] =
                    u64::from((u64::from(((values[47]).wrapping_add(0x1u64)) as u32)) as u32);
                pc = 160;
            }
            162 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(relocate("STACK_local_res20"))?)) as i32 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        < (((0x1au64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    245
                } else {
                    161
                };
            }
            163 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = ((((u64::from(memory.read_u32(relocate("STACK_local_res20"))?))
                        as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 162;
            }
            164 => {
                values[27] = (values[27]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 163;
            }
            165 => {
                pc = if (u64::from(((values[21]) as i64) < ((0x140107438u64) as i64))) != 0 {
                    240
                } else {
                    164
                };
            }
            166 => {
                {
                    let destination = values[27];
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 165;
            }
            167 => {
                values[5] = ((f64::from_bits(values[5])) + (f64::from_bits(values[31]))).to_bits();
                pc = 166;
            }
            168 => {
                pc = if (u64::from(((values[19]) as i64) != ((0x0u64) as i64))) != 0 {
                    238
                } else {
                    167
                };
            }
            169 => {
                values[19] =
                    (values[19]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                pc = 168;
            }
            170 => {
                values[31] = ((f64::from_bits(
                    ((f64::from_bits(values[31]))
                        + (f64::from_bits(
                            ((f64::from_bits(
                                ((f64::from_bits(values[32]))
                                    + (f64::from_bits(
                                        ((f64::from_bits(
                                            ((f64::from_bits(values[29]))
                                                * (f64::from_bits(values[28])))
                                            .to_bits(),
                                        )) * (f64::from_bits(values[44])))
                                        .to_bits(),
                                    )))
                                .to_bits(),
                            )) * (f64::from_bits(memory.read_u64(values[12])?)))
                            .to_bits(),
                        )))
                    .to_bits(),
                )) + (f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(values[33]))
                            + (f64::from_bits(
                                ((f64::from_bits(
                                    ((f64::from_bits(values[30])) * (f64::from_bits(values[38])))
                                        .to_bits(),
                                )) * (f64::from_bits(values[35])))
                                .to_bits(),
                            )))
                        .to_bits(),
                    )) * (f64::from_bits(values[36])))
                    .to_bits(),
                )))
                .to_bits();
                pc = 169;
            }
            171 => {
                values[21] = (values[21]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 170;
            }
            172 => {
                values[36] = memory.read_u64(values[21])?;
                pc = 171;
            }
            173 => {
                values[26] = (values[26]).wrapping_add((0x4u64).wrapping_mul(8));
                pc = 172;
            }
            174 => {
                values[33] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((u64::from(
                                    (u64::from(
                                        memory.read_u32(
                                            (values[22])
                                                .wrapping_add((values[17]).wrapping_mul(0x4u64)),
                                        )?,
                                    )) as u32,
                                )) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[36])))
                            .to_bits(),
                        )) + (f64::from_bits(0x0u64)))
                        .to_bits(),
                    )) + (f64::from_bits(
                        ((f64::from_bits(
                            ((u64::from(
                                (u64::from(memory.read_u32(
                                    (values[22]).wrapping_add((values[23]).wrapping_mul(0x4u64)),
                                )?)) as u32,
                            )) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(values[39])))
                        .to_bits(),
                    )))
                    .to_bits(),
                )) + (f64::from_bits(
                    ((f64::from_bits(
                        ((u64::from(
                            (u64::from(memory.read_u32(
                                (values[22]).wrapping_add((values[16]).wrapping_mul(0x4u64)),
                            )?)) as u32,
                        )) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(values[33])))
                    .to_bits(),
                )))
                .to_bits();
                pc = 173;
            }
            175 => {
                values[30] =
                    ((u64::from(
                        (u64::from(memory.read_u32(
                            (values[22]).wrapping_add((values[24]).wrapping_mul(0x4u64)),
                        )?)) as u32,
                    )) as f64)
                        .to_bits();
                pc = 174;
            }
            176 => {
                values[30] = ((values[18]) as f64).to_bits();
                pc = 173;
            }
            177 => {
                values[30] = (((values[18]) as i64) as f64).to_bits();
                pc = 173;
            }
            178 => {
                pc = if (u64::from(((values[18]) as i64) < ((0x0u64) as i64))) != 0 {
                    176
                } else {
                    177
                };
            }
            179 => {
                values[33] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((memory.read_u64(
                                    (values[25]).wrapping_add((values[17]).wrapping_mul(0x8u64)),
                                )?) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[36])))
                            .to_bits(),
                        )) + (f64::from_bits(0x0u64)))
                        .to_bits(),
                    )) + (f64::from_bits(
                        ((f64::from_bits(
                            ((memory.read_u64(
                                (values[25]).wrapping_add((values[23]).wrapping_mul(0x8u64)),
                            )?) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(values[39])))
                        .to_bits(),
                    )))
                    .to_bits(),
                )) + (f64::from_bits(
                    ((f64::from_bits(
                        ((memory.read_u64(
                            (values[25]).wrapping_add((values[16]).wrapping_mul(0x8u64)),
                        )?) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(values[33])))
                    .to_bits(),
                )))
                .to_bits();
                pc = 178;
            }
            180 => {
                values[18] = memory
                    .read_u64((values[25]).wrapping_add((values[24]).wrapping_mul(0x8u64)))?;
                pc = 179;
            }
            181 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    175
                } else {
                    180
                };
            }
            182 => {
                values[36] = ((f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[38])))
                        .to_bits(),
                )) * (f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[35])))
                        .to_bits(),
                )))
                .to_bits();
                pc = 181;
            }
            183 => {
                values[33] = ((f64::from_bits(values[35]))
                    * (f64::from_bits(
                        ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[38])))
                            .to_bits(),
                    )))
                .to_bits();
                pc = 182;
            }
            184 => {
                values[24] = ((((values[11]).wrapping_add(values[6])) as i32 as i64 as u64)
                    .wrapping_add(0x1u64)) as i32 as i64 as u64;
                pc = 183;
            }
            185 => {
                values[39] = ((f64::from_bits(values[38]))
                    * (f64::from_bits(
                        ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[35])))
                            .to_bits(),
                    )))
                .to_bits();
                pc = 184;
            }
            186 => {
                values[16] = ((((((values[14]).wrapping_mul(values[1])) as i32 as i64 as u64)
                    .wrapping_add(values[6])) as i32 as i64 as u64)
                    .wrapping_add(0x1u64)) as i32 as i64 as u64;
                pc = 185;
            }
            187 => {
                values[23] = ((values[11]).wrapping_add(values[6])) as i32 as i64 as u64;
                pc = 186;
            }
            188 => {
                values[38] = ((f64::from_bits(values[38]))
                    - (f64::from_bits((((values[14]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 187;
            }
            189 => {
                values[17] = ((((values[14]).wrapping_mul(values[1])) as i32 as i64 as u64)
                    .wrapping_add(values[6])) as i32 as i64 as u64;
                pc = 188;
            }
            190 => {
                values[11] = (((((values[14]).wrapping_add(0x1u64)) as i32 as i64 as u64)
                    .wrapping_mul(values[1])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 189;
            }
            191 => {
                values[35] = ((f64::from_bits(values[35]))
                    - (f64::from_bits((((values[6]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 190;
            }
            192 => {
                values[14] =
                    (float_to_integer(f64::from_bits(values[38]), 32, true)?) as i32 as i64 as u64;
                pc = 191;
            }
            193 => {
                values[6] =
                    (float_to_integer(f64::from_bits(values[35]), 32, true)?) as i32 as i64 as u64;
                pc = 192;
            }
            194 => {
                values[38] = values[42];
                pc = 193;
            }
            195 => {
                pc = if (u64::from((f64::from_bits(values[42])) <= (f64::from_bits(values[38]))))
                    != 0
                {
                    194
                } else {
                    193
                };
            }
            196 => {
                values[38] = values[33];
                pc = 195;
            }
            197 => {
                pc = if (u64::from((f64::from_bits(0x0u64)) <= (f64::from_bits(values[33])))) != 0 {
                    196
                } else {
                    195
                };
            }
            198 => {
                values[38] = 0x0u64;
                pc = 197;
            }
            199 => {
                values[35] = values[43];
                pc = 198;
            }
            200 => {
                pc = if (u64::from((f64::from_bits(values[43])) <= (f64::from_bits(values[35]))))
                    != 0
                {
                    199
                } else {
                    198
                };
            }
            201 => {
                values[35] = values[38];
                pc = 200;
            }
            202 => {
                pc = if (u64::from((f64::from_bits(0x0u64)) <= (f64::from_bits(values[38])))) != 0 {
                    201
                } else {
                    200
                };
            }
            203 => {
                values[35] = 0x0u64;
                pc = 202;
            }
            204 => {
                values[12] = (values[21]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                );
                pc = 203;
            }
            205 => {
                values[38] = ((f64::from_bits(values[45]))
                    + (f64::from_bits(memory.read_u64((values[26]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?)))
                .to_bits();
                pc = 204;
            }
            206 => {
                values[33] = ((f64::from_bits(values[46]))
                    + (f64::from_bits(memory.read_u64(values[26])?)))
                .to_bits();
                pc = 205;
            }
            207 => {
                values[32] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((u64::from(
                                    (u64::from(
                                        memory.read_u32(
                                            (values[22])
                                                .wrapping_add((values[17]).wrapping_mul(0x4u64)),
                                        )?,
                                    )) as u32,
                                )) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[35])))
                            .to_bits(),
                        )) + (f64::from_bits(0x0u64)))
                        .to_bits(),
                    )) + (f64::from_bits(
                        ((f64::from_bits(
                            ((u64::from(
                                (u64::from(memory.read_u32(
                                    (values[22]).wrapping_add((values[23]).wrapping_mul(0x4u64)),
                                )?)) as u32,
                            )) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(values[38])))
                        .to_bits(),
                    )))
                    .to_bits(),
                )) + (f64::from_bits(
                    ((f64::from_bits(
                        ((u64::from(
                            (u64::from(memory.read_u32(
                                (values[22]).wrapping_add((values[16]).wrapping_mul(0x4u64)),
                            )?)) as u32,
                        )) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(values[32])))
                    .to_bits(),
                )))
                .to_bits();
                pc = 206;
            }
            208 => {
                values[29] =
                    ((u64::from(
                        (u64::from(memory.read_u32(
                            (values[22]).wrapping_add((values[24]).wrapping_mul(0x4u64)),
                        )?)) as u32,
                    )) as f64)
                        .to_bits();
                pc = 207;
            }
            209 => {
                values[29] = ((values[18]) as f64).to_bits();
                pc = 206;
            }
            210 => {
                values[29] = (((values[18]) as i64) as f64).to_bits();
                pc = 206;
            }
            211 => {
                pc = if (u64::from(((values[18]) as i64) < ((0x0u64) as i64))) != 0 {
                    209
                } else {
                    210
                };
            }
            212 => {
                values[32] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((f64::from_bits(
                            ((f64::from_bits(
                                ((memory.read_u64(
                                    (values[25]).wrapping_add((values[17]).wrapping_mul(0x8u64)),
                                )?) as f64)
                                    .to_bits(),
                            )) * (f64::from_bits(values[35])))
                            .to_bits(),
                        )) + (f64::from_bits(0x0u64)))
                        .to_bits(),
                    )) + (f64::from_bits(
                        ((f64::from_bits(
                            ((memory.read_u64(
                                (values[25]).wrapping_add((values[23]).wrapping_mul(0x8u64)),
                            )?) as f64)
                                .to_bits(),
                        )) * (f64::from_bits(values[38])))
                        .to_bits(),
                    )))
                    .to_bits(),
                )) + (f64::from_bits(
                    ((f64::from_bits(
                        ((memory.read_u64(
                            (values[25]).wrapping_add((values[16]).wrapping_mul(0x8u64)),
                        )?) as f64)
                            .to_bits(),
                    )) * (f64::from_bits(values[32])))
                    .to_bits(),
                )))
                .to_bits();
                pc = 211;
            }
            213 => {
                values[18] = memory
                    .read_u64((values[25]).wrapping_add((values[24]).wrapping_mul(0x8u64)))?;
                pc = 212;
            }
            214 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    208
                } else {
                    213
                };
            }
            215 => {
                values[35] = ((f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[28])))
                        .to_bits(),
                )) * (f64::from_bits(
                    ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[44])))
                        .to_bits(),
                )))
                .to_bits();
                pc = 214;
            }
            216 => {
                values[32] = ((f64::from_bits(values[44]))
                    * (f64::from_bits(
                        ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[28])))
                            .to_bits(),
                    )))
                .to_bits();
                pc = 215;
            }
            217 => {
                values[24] = ((((values[11]).wrapping_add(values[6])) as i32 as i64 as u64)
                    .wrapping_add(0x1u64)) as i32 as i64 as u64;
                pc = 216;
            }
            218 => {
                values[38] = ((f64::from_bits(values[28]))
                    * (f64::from_bits(
                        ((f64::from_bits(0x3ff0000000000000u64)) - (f64::from_bits(values[44])))
                            .to_bits(),
                    )))
                .to_bits();
                pc = 217;
            }
            219 => {
                values[16] = ((((((values[14]).wrapping_mul(values[1])) as i32 as i64 as u64)
                    .wrapping_add(values[6])) as i32 as i64 as u64)
                    .wrapping_add(0x1u64)) as i32 as i64 as u64;
                pc = 218;
            }
            220 => {
                values[23] = ((values[11]).wrapping_add(values[6])) as i32 as i64 as u64;
                pc = 219;
            }
            221 => {
                values[28] = ((f64::from_bits(values[28]))
                    - (f64::from_bits((((values[14]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 220;
            }
            222 => {
                values[17] = ((((values[14]).wrapping_mul(values[1])) as i32 as i64 as u64)
                    .wrapping_add(values[6])) as i32 as i64 as u64;
                pc = 221;
            }
            223 => {
                values[11] = (((((values[14]).wrapping_add(0x1u64)) as i32 as i64 as u64)
                    .wrapping_mul(values[1])) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 222;
            }
            224 => {
                values[44] = ((f64::from_bits(values[44]))
                    - (f64::from_bits((((values[6]) as i64) as f64).to_bits())))
                .to_bits();
                pc = 223;
            }
            225 => {
                values[14] =
                    (float_to_integer(f64::from_bits(values[28]), 32, true)?) as i32 as i64 as u64;
                pc = 224;
            }
            226 => {
                values[6] =
                    (float_to_integer(f64::from_bits(values[44]), 32, true)?) as i32 as i64 as u64;
                pc = 225;
            }
            227 => {
                values[28] = values[42];
                pc = 226;
            }
            228 => {
                pc = if (u64::from((f64::from_bits(values[42])) <= (f64::from_bits(values[28]))))
                    != 0
                {
                    227
                } else {
                    226
                };
            }
            229 => {
                values[28] = values[32];
                pc = 228;
            }
            230 => {
                pc = if (u64::from((f64::from_bits(0x0u64)) <= (f64::from_bits(values[32])))) != 0 {
                    229
                } else {
                    228
                };
            }
            231 => {
                values[28] = 0x0u64;
                pc = 230;
            }
            232 => {
                values[44] = values[43];
                pc = 231;
            }
            233 => {
                pc = if (u64::from((f64::from_bits(values[43])) <= (f64::from_bits(values[44]))))
                    != 0
                {
                    232
                } else {
                    231
                };
            }
            234 => {
                values[44] = values[28];
                pc = 233;
            }
            235 => {
                pc = if (u64::from((f64::from_bits(0x0u64)) <= (f64::from_bits(values[28])))) != 0 {
                    234
                } else {
                    233
                };
            }
            236 => {
                values[44] = 0x0u64;
                pc = 235;
            }
            237 => {
                values[32] = ((f64::from_bits(values[46]))
                    + (f64::from_bits(memory.read_u64((values[26]).wrapping_add(
                        ((0u64.wrapping_sub(0x2u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?)))
                .to_bits();
                pc = 236;
            }
            238 => {
                values[28] = ((f64::from_bits(values[45]))
                    + (f64::from_bits(memory.read_u64((values[26]).wrapping_add(
                        ((0u64.wrapping_sub(0x3u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?)))
                .to_bits();
                pc = 237;
            }
            239 => {
                values[19] = 0x2u64;
                pc = 238;
            }
            240 => {
                values[31] = 0x0u64;
                pc = 239;
            }
            241 => {
                values[26] = (values[0]).wrapping_add(0x1600u64);
                pc = 240;
            }
            242 => {
                values[45] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((((u64::from(memory.read_u32(relocate("STACK_local_res20"))?)) as i32
                            as i64 as u64) as i64) as f64)
                            .to_bits(),
                    )) + (f64::from_bits(0x3ff8000000000000u64)))
                    .to_bits(),
                )) * (f64::from_bits(values[34])))
                .to_bits();
                pc = 241;
            }
            243 => {
                values[5] = 0x0u64;
                pc = 242;
            }
            244 => {
                {
                    let destination = values[27];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            245 => {
                values[21] = relocate("DAT_1401073d8");
                pc = 244;
            }
            246 => {
                values[46] = ((f64::from_bits(
                    ((f64::from_bits(
                        ((((values[18]) as i32 as i64 as u64) as i64) as f64).to_bits(),
                    )) + (f64::from_bits(0x3ff8000000000000u64)))
                    .to_bits(),
                )) * (f64::from_bits(values[37])))
                .to_bits();
                pc = 245;
            }
            247 => {
                {
                    let destination = relocate("STACK_local_res20");
                    let stored = (0x0u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 246;
            }
            248 => {
                values[27] = (values[0]).wrapping_add(0xc8u64);
                pc = 247;
            }
            249 => {
                values[47] = u64::from((0x0u64) as u32);
                pc = 248;
            }
            250 => {
                values[42] = ((f64::from_bits((((values[2]) as i64) as f64).to_bits()))
                    - (f64::from_bits(0x4000000000000000u64)))
                .to_bits();
                pc = 249;
            }
            251 => {
                values[43] = ((f64::from_bits((((values[1]) as i64) as f64).to_bits()))
                    - (f64::from_bits(0x4000000000000000u64)))
                .to_bits();
                pc = 250;
            }
            252 => {
                pc = if (u64::from(((values[26]) as i64) < ((0x140107448u64) as i64))) != 0 {
                    272
                } else {
                    251
                };
            }
            253 => {
                values[27] = (values[27]).wrapping_add((0x8u64).wrapping_mul(8));
                pc = 252;
            }
            254 => {
                {
                    let destination = values[27];
                    let stored = ((f64::from_bits(memory.read_u64(values[27])?))
                        - (f64::from_bits(0x3ff0000000000000u64)))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 253;
            }
            255 => {
                {
                    let destination = (values[27]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[27]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        - (f64::from_bits(0x3ff0000000000000u64)))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 254;
            }
            256 => {
                {
                    let destination = (values[27]).wrapping_add(
                        ((0u64.wrapping_sub(0x3u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[27]).wrapping_add(
                        ((0u64.wrapping_sub(0x3u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        - (f64::from_bits(0x3ff0000000000000u64)))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 255;
            }
            257 => {
                {
                    let destination = (values[27]).wrapping_add(
                        ((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64).wrapping_mul(8),
                    );
                    let stored = ((f64::from_bits(memory.read_u64((values[27]).wrapping_add(
                        ((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64).wrapping_mul(8),
                    ))?))
                        - (f64::from_bits(0x3ff0000000000000u64)))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 256;
            }
            258 => {
                values[26] = (values[26]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 257;
            }
            259 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    269
                } else {
                    258
                };
            }
            260 => {
                values[21] = (values[21]).wrapping_add((0x4u64).wrapping_mul(8));
                pc = 259;
            }
            261 => {
                values[14] =
                    (((values[14]).wrapping_add(0x2u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 260;
            }
            262 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        < (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    267
                } else {
                    261
                };
            }
            263 => {
                values[11] = (values[6]) as i32 as i64 as u64;
                pc = 262;
            }
            264 => {
                values[12] = (values[12]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 263;
            }
            265 => {
                {
                    let destination = values[12];
                    let stored = ((f64::from_bits(
                        ((f64::from_bits((((values[11]) as i64) as f64).to_bits()))
                            * (f64::from_bits(values[34])))
                        .to_bits(),
                    )) * (f64::from_bits(values[42])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 264;
            }
            266 => {
                {
                    let destination = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = ((f64::from_bits(
                        ((f64::from_bits((((values[14]) as i64) as f64).to_bits()))
                            * (f64::from_bits(values[37])))
                        .to_bits(),
                    )) * (f64::from_bits(values[42])))
                    .to_bits();
                    memory.write_u64(destination, stored)?;
                }
                pc = 265;
            }
            267 => {
                values[6] =
                    (((values[11]).wrapping_add(0x2u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 266;
            }
            268 => {
                values[11] =
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 267;
            }
            269 => {
                values[12] = values[21];
                pc = 268;
            }
            270 => {
                values[21] = (values[27]).wrapping_add(
                    ((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64).wrapping_mul(8),
                );
                pc = 269;
            }
            271 => {
                values[14] =
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 270;
            }
            272 => {
                values[42] = memory.read_u64(values[26])?;
                pc = 271;
            }
            273 => {
                values[37] = ((f64::from_bits((((values[2]) as i64) as f64).to_bits()))
                    / (f64::from_bits(0x403c000000000000u64)))
                .to_bits();
                pc = 272;
            }
            274 => {
                values[34] = ((f64::from_bits((((values[1]) as i64) as f64).to_bits()))
                    / (f64::from_bits(0x403c000000000000u64)))
                .to_bits();
                pc = 273;
            }
            275 => {
                values[27] = (values[0]).wrapping_add(0x1608u64);
                pc = 274;
            }
            276 => {
                values[26] = relocate("DAT_140107430");
                pc = 275;
            }
            277 => {
                values[25] = values[18];
                pc = 276;
            }
            278 => {
                values[22] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
                pc = 277;
            }
            279 => {
                values[25] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
                pc = 276;
            }
            280 => {
                values[22] = values[18];
                pc = 279;
            }
            281 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    278
                } else {
                    280
                };
            }
            282 => {
                values[4] = ((u64::from(memory.read_u8((values[0]).wrapping_add(0x34u64))?)) as i8
                    as i64 as u64) as i8 as i64 as u64;
                pc = 281;
            }
            283 => {
                values[18] = 0x0u64;
                pc = 282;
            }
            284 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((0x31u64) as i32 as i64 as u64) as i64)
                            < (((values[1]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (((0x31u64) as i32 as i64 as u64) as i64)
                                < (((values[2]) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    283
                } else {
                    0
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400268a0` (`mspaint.c:37513`).
pub fn fun_1400268a0(
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
    let mut pc: i32 = 126;
    loop {
        match pc {
            0 => {
                {
                    let call_arguments = [relocate("\"Argument not found.\""), values[1]];
                    invoke("FUN_140032060", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140032060"));
            }
            1 => {
                return Ok(Some(values[0]));
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(values[0])?)) as i8 as i64 as u64) as i32 as i64
                        as u64) as i64)
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
                    let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[0]).wrapping_add((0x16u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[0]).wrapping_add((0x14u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[0]).wrapping_add((0x11u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[0]).wrapping_add((0xfu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[0]).wrapping_add((0xeu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[0]).wrapping_add((0xdu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[0]).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = (values[0]).wrapping_add((0xbu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[0]).wrapping_add((0xau64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[0]).wrapping_add((0x9u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = (values[0]).wrapping_add((0x7u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = (values[0]).wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 22;
            }
            24 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = (values[0]).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 25;
            }
            27 => {
                pc = 26;
            }
            28 => {
                return Ok(Some(values[0]));
            }
            29 => {
                {
                    let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = values[0];
                    let stored = (0xdu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 30;
            }
            32 => {
                values[5] = memory.read_u64(
                    ((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))
                        .wrapping_add(0x8u64),
                )?;
                pc = 31;
            }
            33 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 32;
            }
            34 => {
                return Ok(Some(values[0]));
            }
            35 => {
                {
                    let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = values[0];
                    let stored = (0xcu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 36;
            }
            38 => {
                values[5] = memory.read_u64(
                    ((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))
                        .wrapping_add(0x8u64),
                )?;
                pc = 37;
            }
            39 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 38;
            }
            40 => {
                return Ok(Some(values[0]));
            }
            41 => {
                {
                    let destination = values[0];
                    let stored = (0xbu64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 42;
            }
            44 => {
                return Ok(Some(values[0]));
            }
            45 => {
                {
                    let destination = values[0];
                    let stored = (0xau64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 46;
            }
            48 => {
                return Ok(Some(values[0]));
            }
            49 => {
                {
                    let destination = values[0];
                    let stored = (0x9u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 50;
            }
            52 => {
                return Ok(Some(values[0]));
            }
            53 => {
                {
                    let destination = values[0];
                    let stored = (0x8u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 54;
            }
            56 => {
                return Ok(Some(values[0]));
            }
            57 => {
                {
                    let destination = values[0];
                    let stored = (0x7u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((u64::from((values[4]) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 57;
            }
            59 => {
                values[4] = u64::from(
                    (u64::from(
                        memory
                            .read_u32((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?,
                    )) as u32,
                );
                pc = 58;
            }
            60 => {
                return Ok(Some(values[0]));
            }
            61 => {
                {
                    let destination = values[0];
                    let stored = (0x6u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((u64::from((values[4]) as u16)) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 61;
            }
            63 => {
                values[4] = u64::from(
                    (u64::from(
                        memory
                            .read_u16((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?,
                    )) as u16,
                );
                pc = 62;
            }
            64 => {
                return Ok(Some(values[0]));
            }
            65 => {
                {
                    let destination = values[0];
                    let stored = (0x5u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((u64::from((values[4]) as u8)) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 65;
            }
            67 => {
                values[4] = u64::from(
                    (u64::from(
                        memory
                            .read_u8((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?,
                    )) as u8,
                );
                pc = 66;
            }
            68 => {
                return Ok(Some(values[0]));
            }
            69 => {
                {
                    let destination = values[0];
                    let stored = (0x4u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 70;
            }
            72 => {
                return Ok(Some(values[0]));
            }
            73 => {
                {
                    let destination = values[0];
                    let stored = (0x3u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                values[4] =
                    memory.read_u64((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?;
                pc = 74;
            }
            76 => {
                return Ok(Some(values[0]));
            }
            77 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((u64::from((values[4]) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = values[0];
                    let stored = (0x2u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 77;
            }
            79 => {
                values[4] = u64::from(
                    (u64::from(
                        memory
                            .read_u32((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?,
                    )) as u32,
                );
                pc = 78;
            }
            80 => {
                return Ok(Some(values[0]));
            }
            81 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = u64::from((u64::from((values[4]) as u32)) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = values[0];
                    let stored = (0x1u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 81;
            }
            83 => {
                values[4] = u64::from(
                    (u64::from(
                        memory
                            .read_u32((values[1]).wrapping_add((values[3]).wrapping_mul(0x8u64)))?,
                    )) as u32,
                );
                pc = 82;
            }
            84 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0xdu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    27
                };
            }
            85 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0xcu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    39
                } else {
                    84
                };
            }
            86 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0xbu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    43
                } else {
                    85
                };
            }
            87 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0xau64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    47
                } else {
                    86
                };
            }
            88 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x9u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    51
                } else {
                    87
                };
            }
            89 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x8u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    55
                } else {
                    88
                };
            }
            90 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x7u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    59
                } else {
                    89
                };
            }
            91 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x6u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    90
                };
            }
            92 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    67
                } else {
                    91
                };
            }
            93 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    71
                } else {
                    92
                };
            }
            94 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    75
                } else {
                    93
                };
            }
            95 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    79
                } else {
                    94
                };
            }
            96 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    83
                } else {
                    95
                };
            }
            97 => {
                values[8] = (u64::from(((values[6]).wrapping_shr((0x3cu64) as u32)) as u8)) as i32
                    as i64 as u64;
                pc = 96;
            }
            98 => {
                pc = if (u64::from(
                    (((u64::from(((values[6]).wrapping_shr((0x3cu64) as u32)) as u8)) as i32 as i64
                        as u64) as i64)
                        < (((0xeu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    97
                } else {
                    26
                };
            }
            99 => {
                values[1] = ((values[6]) & (0xfffffffffffffffu64))
                    .wrapping_add(memory.read_u64((values[1]).wrapping_add(0x10u64))?);
                pc = 98;
            }
            100 => {
                values[6] = memory.read_u64(
                    (memory.read_u64((values[1]).wrapping_add(0x10u64))?)
                        .wrapping_add((values[2]).wrapping_mul(0x8u64)),
                )?;
                pc = 99;
            }
            101 => {
                pc = if (u64::from((values[2]) < (values[3]))) != 0 {
                    100
                } else {
                    2
                };
            }
            102 => {
                values[3] = memory.read_u64((values[1]).wrapping_add(0x8u64))?;
                pc = 101;
            }
            103 => {
                {
                    let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let destination = (values[0]).wrapping_add((0x16u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[0]).wrapping_add((0x14u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 105;
            }
            107 => {
                {
                    let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 106;
            }
            108 => {
                {
                    let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let destination = (values[0]).wrapping_add((0x11u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = (values[0]).wrapping_add((0xfu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 110;
            }
            112 => {
                {
                    let destination = (values[0]).wrapping_add((0xeu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = (values[0]).wrapping_add((0xdu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 112;
            }
            114 => {
                {
                    let destination = (values[0]).wrapping_add((0xcu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let destination = (values[0]).wrapping_add((0xbu64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 114;
            }
            116 => {
                {
                    let destination = (values[0]).wrapping_add((0xau64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 115;
            }
            117 => {
                {
                    let destination = (values[0]).wrapping_add((0x9u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 116;
            }
            118 => {
                {
                    let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 117;
            }
            119 => {
                {
                    let destination = (values[0]).wrapping_add((0x7u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = (values[0]).wrapping_add((0x6u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 121;
            }
            123 => {
                {
                    let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 123;
            }
            125 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 124;
            }
            126 => {
                {
                    let destination = (values[0]).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 125;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140026da0` (`mspaint.c:37737`).
pub fn fun_140026da0(
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
    ];
    let mut pc: i32 = 16;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let destination = values[1];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                pc = if (u64::from((values[5]) != (values[6]))) != 0 {
                    9
                } else {
                    1
                };
            }
            3 => {
                {
                    let destination = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (memory
                        .read_u64((values[4]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                    .wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (memory
                        .read_u64((values[4]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                    .wrapping_add(
                        (memory.read_u64((values[4]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                            .wrapping_mul(0x2u64),
                    );
                    let stored = ((values[3]) as i16 as i64 as u64) as i16 as i64 as u64;
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 3;
            }
            5 => {
                values[5] = (values[5]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 4;
            }
            6 => {
                values[6] = memory.read_u64(
                    (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[4])?)?, values[4]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (memory.read_u64((values[4]).wrapping_add((0x3u64).wrapping_mul(8)))?)
                        < ((memory
                            .read_u64((values[4]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                        .wrapping_add(0x1u64)),
                )) != 0
                {
                    7
                } else {
                    5
                };
            }
            9 => {
                values[3] = ((u64::from(memory.read_u8(values[5])?)) as i8 as i64 as u64) as i8
                    as i64 as u64;
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (relocate("STACK_local_38"))
                        != (memory.read_u64(
                            (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?),
                )) != 0
                {
                    9
                } else {
                    1
                };
            }
            11 => {
                values[6] = memory.read_u64(
                    (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 10;
            }
            12 => {
                values[5] = relocate("STACK_local_38");
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_48"),
                        relocate("STACK_local_38"),
                        relocate("STACK_local_20"),
                        values[2],
                    ];
                    invoke("FUN_14002cef0", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = u64::from((0xau64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                values[4] = memory.read_u64(values[0])?;
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_78"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002dcb0` (`mspaint.c:43348`).
pub fn fun_14002dcb0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
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
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
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
    let mut pc: i32 = 211;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[0]));
            }
            1 => {
                {
                    let destination = values[0];
                    let stored = (values[42]).wrapping_add(values[38]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from(
                        (u64::from(((values[33]).wrapping_add(values[39])) as u32)) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                values[42] = 0x1u64;
                pc = 2;
            }
            4 => {
                pc = 2;
            }
            5 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from((values[38]) != (values[36]))) != 0)
                            || ((u64::from(
                                ((u64::from(
                                    (((values[48]) as i32 as i64 as u64) as i64)
                                        != (((0x0u64) as i32 as i64 as u64) as i64),
                                )) != 0)
                                    && ((u64::from(
                                        (((values[43]) as i32 as i64 as u64) as i64)
                                            != (((0x0u64) as i32 as i64 as u64) as i64),
                                    )) != 0),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(
                            (((values[32]) as i32 as i64 as u64) as i64)
                                < (((0x5u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[32] = u64::from((0x4u64) as u8);
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (((values[50]) as i32 as i64 as u64) as i64)
                                != (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                (((values[32]) as i32 as i64 as u64) as i64)
                                    == (((0x5u64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(((values[38]) & (0x1u64)) == (0x0u64))) != 0),
                )) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                pc = if (u64::from(
                    ((((values[36]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((((({
                            let numerator = values[36];
                            let denominator = 0xau64;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        }) as i32 as i64 as u64)
                            .wrapping_mul(0xau64)) as i32 as i64
                            as u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    15
                } else {
                    7
                };
            }
            9 => {
                values[37] = values[38];
                pc = 8;
            }
            10 => {
                values[46] = {
                    let numerator = values[36];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 9;
            }
            11 => {
                values[32] = u64::from(
                    ((((values[37]) as i8 as i64 as u64).wrapping_add(
                        (((values[38]) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64) as u8,
                );
                pc = 10;
            }
            12 => {
                values[50] = u64::from(
                    (u64::from(
                        (((values[50])
                            & (u64::from(
                                (((values[32]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            ))) as i32 as i64 as u64)
                            != 0,
                    )) != 0,
                );
                pc = 11;
            }
            13 => {
                values[39] =
                    (((values[39]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 12;
            }
            14 => {
                values[38] = {
                    let numerator = values[37];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 13;
            }
            15 => {
                values[36] = values[46];
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[43]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                let _ = {
                                    values[46] = {
                                        let numerator = values[36];
                                        let denominator = 0xau64;
                                        if denominator == 0 {
                                            return Err(AdapterError::Arithmetic(
                                                "division by zero",
                                            ));
                                        }
                                        numerator.checked_div(denominator).ok_or(
                                            AdapterError::Arithmetic("signed division overflow"),
                                        )?
                                    };
                                    values[46]
                                };
                                {
                                    values[37] = values[38];
                                    values[37]
                                }
                            };
                            u64::from(
                                ((((values[36]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                                    == (((((({
                                        let numerator = values[36];
                                        let denominator = 0xau64;
                                        if denominator == 0 {
                                            return Err(AdapterError::Arithmetic(
                                                "division by zero",
                                            ));
                                        }
                                        numerator.checked_div(denominator).ok_or(
                                            AdapterError::Arithmetic("signed division overflow"),
                                        )?
                                    }) as i32 as i64
                                        as u64)
                                        .wrapping_mul(0xau64))
                                        as i32 as i64
                                        as u64) as i32
                                        as i64 as u64)
                                        as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    15
                } else {
                    7
                };
            }
            17 => {
                pc = if (u64::from(
                    ({
                        let numerator = values[36];
                        let denominator = 0xau64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) < (values[35]),
                )) != 0
                {
                    33
                } else {
                    16
                };
            }
            18 => {
                values[41] = values[36];
                pc = 17;
            }
            19 => {
                values[47] = values[38];
                pc = 18;
            }
            20 => {
                values[37] = {
                    let numerator = values[36];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 19;
            }
            21 => {
                values[35] = {
                    let numerator = values[35];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 20;
            }
            22 => {
                values[32] = u64::from(
                    ((((values[47]) as i8 as i64 as u64).wrapping_add(
                        (((values[38]) as i8 as i64 as u64)
                            .wrapping_mul((0u64.wrapping_sub(0xau64)) as i32 as i64 as u64))
                            as i32 as i64 as u64,
                    )) as i32 as i64 as u64) as u8,
                );
                pc = 21;
            }
            23 => {
                values[39] =
                    (((values[39]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 22;
            }
            24 => {
                values[50] = u64::from((values[30]) != 0);
                pc = 23;
            }
            25 => {
                values[30] = u64::from((values[50]) != 0);
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    (((values[32]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    24
                };
            }
            27 => {
                values[30] = 0u64;
                pc = 26;
            }
            28 => {
                values[46] = values[43];
                pc = 27;
            }
            29 => {
                values[43] = u64::from(
                    (u64::from(
                        ((values[46]).wrapping_shr(
                            (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                        )) as u8,
                    )) != 0,
                );
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    ((((values[41]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == ((((((values[36]) as i32 as i64 as u64).wrapping_mul(0xau64)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[43] = 0u64;
                pc = 30;
            }
            32 => {
                values[38] = {
                    let numerator = values[47];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 31;
            }
            33 => {
                values[36] = values[37];
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    ({
                        let numerator = values[36];
                        let denominator = 0xau64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) < (values[35]),
                )) != 0
                {
                    33
                } else {
                    16
                };
            }
            35 => {
                values[41] = values[36];
                pc = 34;
            }
            36 => {
                values[47] = values[38];
                pc = 35;
            }
            37 => {
                values[37] = {
                    let numerator = values[36];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 36;
            }
            38 => {
                values[35] = {
                    let numerator = values[35];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 37;
            }
            39 => {
                values[43] = u64::from(
                    (u64::from(
                        ((values[46]).wrapping_shr(
                            (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                        )) as u8,
                    )) != 0,
                );
                pc = 38;
            }
            40 => {
                values[32] = u64::from((0x0u64) as u8);
                pc = 39;
            }
            41 => {
                values[39] = (0x0u64) as i32 as i64 as u64;
                pc = 40;
            }
            42 => {
                pc = 123;
            }
            43 => {
                pc = if (u64::from(
                    ((((values[46]) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                values[38] = values[37];
                pc = 43;
            }
            45 => {
                values[46] = values[42];
                pc = 44;
            }
            46 => {
                pc = if (u64::from(
                    (((values[48]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    45
                } else {
                    44
                };
            }
            47 => {
                values[46] = values[32];
                pc = 46;
            }
            48 => {
                values[35] = (values[35]).wrapping_sub(0x1u64);
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    (((values[48]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    47
                };
            }
            50 => {
                values[50] = 1u64;
                pc = 49;
            }
            51 => {
                pc = 123;
            }
            52 => {
                values[50] = u64::from(
                    (u64::from(
                        ((values[38])
                            & (u64::from(
                                ((((0x1u64 as i32).wrapping_shl(
                                    (u64::from(
                                        ((u64::from(
                                            (((values[44]) as i8 as i64 as u64)
                                                .wrapping_sub(0x1u64))
                                                as u32,
                                        )) & (0x3fu64))
                                            as u32,
                                    )) as u32,
                                ) as u64) as i32 as i64 as u64)
                                    .wrapping_sub(0x1u64)) as u32,
                            )))
                            == (0x0u64),
                    )) != 0,
                );
                pc = 51;
            }
            53 => {
                pc = if (u64::from(
                    (u64::from((values[44]) as u32)) < (u64::from((0x3fu64) as u32)),
                )) != 0
                {
                    52
                } else {
                    44
                };
            }
            54 => {
                pc = if (u64::from((u64::from((values[44]) as u32)) < (u64::from((0x2u64) as u32))))
                    != 0
                {
                    50
                } else {
                    53
                };
            }
            55 => {
                values[37] = ((values[47])
                    .wrapping_shr((((values[49]) & (0x3fu64)) as i32 as i64 as u64) as u32))
                    | ((values[34] as i64).wrapping_shl(
                        (((0x40u64).wrapping_sub(((values[49]) & (0x3fu64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as u32,
                    ) as u64);
                pc = 54;
            }
            56 => {
                values[34] = (values[34]).wrapping_add(0x1u64);
                pc = 55;
            }
            57 => {
                pc = if (u64::from((values[47]) < (values[37]))) != 0 {
                    56
                } else {
                    55
                };
            }
            58 => {
                values[47] = ((values[13]).wrapping_mul(values[25])).wrapping_add(values[37]);
                pc = 57;
            }
            59 => {
                values[37] = (((values[14]) as u128 * (values[26]) as u128) >> 64) as u64;
                pc = 58;
            }
            60 => {
                values[26] = values[38];
                pc = 59;
            }
            61 => {
                values[14] = values[47];
                pc = 60;
            }
            62 => {
                values[36] = ((values[41])
                    .wrapping_shr((((values[49]) & (0x3fu64)) as i32 as i64 as u64) as u32))
                    | ((values[31] as i64).wrapping_shl(
                        (((0x40u64).wrapping_sub(((values[49]) & (0x3fu64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as u32,
                    ) as u64);
                pc = 61;
            }
            63 => {
                values[34] = (((values[13]) as u128 * (values[25]) as u128) >> 64) as u64;
                pc = 62;
            }
            64 => {
                values[25] = values[38];
                pc = 63;
            }
            65 => {
                values[13] = values[37];
                pc = 64;
            }
            66 => {
                values[31] = (values[31]).wrapping_add(0x1u64);
                pc = 65;
            }
            67 => {
                pc = if (u64::from((values[41]) < (values[36]))) != 0 {
                    66
                } else {
                    65
                };
            }
            68 => {
                values[41] = ((values[11]).wrapping_mul(values[23])).wrapping_add(values[36]);
                pc = 67;
            }
            69 => {
                values[36] = (((values[12]) as u128 * (values[24]) as u128) >> 64) as u64;
                pc = 68;
            }
            70 => {
                values[24] = values[36];
                pc = 69;
            }
            71 => {
                values[12] = values[47];
                pc = 70;
            }
            72 => {
                values[31] = (((values[11]) as u128 * (values[23]) as u128) >> 64) as u64;
                pc = 71;
            }
            73 => {
                values[23] = values[36];
                pc = 72;
            }
            74 => {
                values[11] = values[37];
                pc = 73;
            }
            75 => {
                values[36] = ((values[38]).wrapping_sub(values[36])).wrapping_sub(0x1u64);
                pc = 74;
            }
            76 => {
                values[35] = ((values[41])
                    .wrapping_shr((((values[49]) & (0x3fu64)) as i32 as i64 as u64) as u32))
                    | ((values[31] as i64).wrapping_shl(
                        (((0x40u64).wrapping_sub(((values[49]) & (0x3fu64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as u32,
                    ) as u64);
                pc = 75;
            }
            77 => {
                values[49] = u64::from(
                    ((((((values[44]) as i8 as i64 as u64).wrapping_sub(
                        (u64::from(
                            ((u64::from(
                                ((values[33]).wrapping_mul(
                                    (0u64.wrapping_sub(0x12934fu64)) as i32 as i64 as u64,
                                )) as u32,
                            ) as u32)
                                .wrapping_shr((0x13u64) as u32) as u64)
                                as u32,
                        )) as i8 as i64 as u64,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x38u64)) as i32 as i64 as u64) as u8,
                );
                pc = 76;
            }
            78 => {
                values[31] = (values[31]).wrapping_add(0x1u64);
                pc = 77;
            }
            79 => {
                pc = if (u64::from((values[41]) < (values[35]))) != 0 {
                    78
                } else {
                    77
                };
            }
            80 => {
                values[41] = ((values[9]).wrapping_mul(values[21])).wrapping_add(values[35]);
                pc = 79;
            }
            81 => {
                values[35] = (((values[10]) as u128 * (values[22]) as u128) >> 64) as u64;
                pc = 80;
            }
            82 => {
                values[22] = (values[38]).wrapping_add(0x2u64);
                pc = 81;
            }
            83 => {
                values[10] = values[47];
                pc = 82;
            }
            84 => {
                values[47] = memory.read_u64(
                    (relocate("DAT_1400e9260")).wrapping_add(
                        (((u64::from((0u64.wrapping_sub(values[33])) as u32)) as i32 as i64
                            as u64)
                            .wrapping_mul(0x10u64))
                        .wrapping_mul(1),
                    ),
                )?;
                pc = 83;
            }
            85 => {
                values[31] = (((values[9]) as u128 * (values[21]) as u128) >> 64) as u64;
                pc = 84;
            }
            86 => {
                values[21] = (values[38]).wrapping_add(0x2u64);
                pc = 85;
            }
            87 => {
                values[9] = values[37];
                pc = 86;
            }
            88 => {
                values[37] = memory.read_u64(
                    (relocate("DAT_1400e9268")).wrapping_add(
                        (((u64::from((0u64.wrapping_sub(values[33])) as u32)) as i32 as i64
                            as u64)
                            .wrapping_mul(0x10u64))
                        .wrapping_mul(1),
                    ),
                )?;
                pc = 87;
            }
            89 => {
                values[33] =
                    u64::from((u64::from(((values[44]).wrapping_add(values[39])) as u32)) as u32);
                pc = 88;
            }
            90 => {
                values[44] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((values[39]).wrapping_mul(
                                    (0u64.wrapping_sub(0xb2efbu64)) as i32 as i64 as u64,
                                )) as i32 as i64 as u64) as u32,
                            ) as u32)
                                .wrapping_shr((0x14u64) as u32) as u64)
                                as u32,
                        ))
                        .wrapping_sub(u64::from(
                            (u64::from(
                                (((0x1u64) as i32 as i64 as u64) as i64)
                                    < ((((0u64.wrapping_sub(values[39])) as i32 as i64 as u64)
                                        as i32 as i64
                                        as u64) as i64),
                            )) as u32,
                        ))) as u32,
                    )) as u32,
                );
                pc = 89;
            }
            91 => {
                pc = 2;
            }
            92 => {
                pc = if (u64::from(
                    ((u64::from((values[38]) != (values[36]))) != 0)
                        && ((u64::from(!((values[50]) != 0))) != 0),
                )) != 0
                {
                    91
                } else {
                    3
                };
            }
            93 => {
                pc = if (u64::from(
                    ({
                        let numerator = values[36];
                        let denominator = 0xau64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) < (values[35]),
                )) != 0
                {
                    100
                } else {
                    92
                };
            }
            94 => {
                values[37] = values[38];
                pc = 93;
            }
            95 => {
                values[46] = {
                    let numerator = values[36];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 94;
            }
            96 => {
                values[35] = {
                    let numerator = values[35];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 95;
            }
            97 => {
                values[39] =
                    (((values[39]).wrapping_add(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 96;
            }
            98 => {
                values[50] = u64::from(
                    (u64::from(
                        (u64::from((0x4u64) as u32))
                            < (u64::from(
                                (u64::from(
                                    ((((values[37]) as i32 as i64 as u64).wrapping_add(
                                        (((values[38]) as i32 as i64 as u64).wrapping_mul(
                                            (0u64.wrapping_sub(0xau64)) as i32 as i64 as u64,
                                        )) as i32 as i64
                                            as u64,
                                    )) as i32 as i64 as u64)
                                        as u32,
                                )) as u32,
                            )),
                    )) != 0,
                );
                pc = 97;
            }
            99 => {
                values[38] = {
                    let numerator = values[37];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 98;
            }
            100 => {
                values[36] = values[46];
                pc = 99;
            }
            101 => {
                pc = if (u64::from(
                    ({
                        let numerator = values[36];
                        let denominator = 0xau64;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                    }) < (values[35]),
                )) != 0
                {
                    100
                } else {
                    92
                };
            }
            102 => {
                values[37] = values[38];
                pc = 101;
            }
            103 => {
                values[46] = {
                    let numerator = values[36];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 102;
            }
            104 => {
                values[35] = {
                    let numerator = values[35];
                    let denominator = 0xau64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 103;
            }
            105 => {
                values[36] = values[47];
                pc = 104;
            }
            106 => {
                values[35] = values[46];
                pc = 105;
            }
            107 => {
                values[50] = u64::from(
                    (u64::from(
                        (u64::from((0x31u64) as u32))
                            < (u64::from(
                                (u64::from(
                                    ((((values[37]) as i32 as i64 as u64).wrapping_add(
                                        (((values[38]) as i32 as i64 as u64).wrapping_mul(
                                            (0u64.wrapping_sub(0x64u64)) as i32 as i64 as u64,
                                        )) as i32 as i64
                                            as u64,
                                    )) as i32 as i64 as u64)
                                        as u32,
                                )) as u32,
                            )),
                    )) != 0,
                );
                pc = 106;
            }
            108 => {
                values[38] = ((((values[37]).wrapping_sub(values[31]))
                    .wrapping_shr((0x1u64) as u32))
                .wrapping_add(values[31]))
                .wrapping_shr((0x6u64) as u32);
                pc = 107;
            }
            109 => {
                values[39] = (0x2u64) as i32 as i64 as u64;
                pc = 108;
            }
            110 => {
                values[31] =
                    (((0x47ae147ae147ae15u64) as u128 * (values[29]) as u128) >> 64) as u64;
                pc = 109;
            }
            111 => {
                values[29] = values[37];
                pc = 110;
            }
            112 => {
                pc = if (u64::from((values[47]) < (values[46]))) != 0 {
                    111
                } else {
                    104
                };
            }
            113 => {
                values[50] = 0u64;
                pc = 112;
            }
            114 => {
                values[47] = ((((values[36]).wrapping_sub(values[34]))
                    .wrapping_shr((0x1u64) as u32))
                .wrapping_add(values[34]))
                .wrapping_shr((0x6u64) as u32);
                pc = 113;
            }
            115 => {
                values[46] = ((((values[35]).wrapping_sub(values[31]))
                    .wrapping_shr((0x1u64) as u32))
                .wrapping_add(values[31]))
                .wrapping_shr((0x6u64) as u32);
                pc = 114;
            }
            116 => {
                values[34] =
                    (((0x47ae147ae147ae15u64) as u128 * (values[28]) as u128) >> 64) as u64;
                pc = 115;
            }
            117 => {
                values[28] = values[36];
                pc = 116;
            }
            118 => {
                values[31] =
                    (((0x47ae147ae147ae15u64) as u128 * (values[27]) as u128) >> 64) as u64;
                pc = 117;
            }
            119 => {
                values[27] = values[35];
                pc = 118;
            }
            120 => {
                pc = 41;
            }
            121 => {
                pc = if (u64::from(
                    (((values[50]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    120
                } else {
                    119
                };
            }
            122 => {
                values[38] = values[37];
                pc = 121;
            }
            123 => {
                values[39] = (0x0u64) as i32 as i64 as u64;
                pc = 122;
            }
            124 => {
                values[50] = u64::from(
                    (u64::from(
                        (u64::from((values[33]) as u32)) <= (u64::from((values[44]) as u32)),
                    )) != 0,
                );
                pc = 123;
            }
            125 => {
                pc = if (u64::from(
                    (((values[39]) as i32 as i64 as u64) as i64)
                        == ((((((values[38]) as i32 as i64 as u64).wrapping_mul(0x5u64)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    130
                } else {
                    124
                };
            }
            126 => {
                values[45] = values[44];
                pc = 125;
            }
            127 => {
                values[41] = values[38];
                pc = 126;
            }
            128 => {
                values[39] = ((values[41]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 127;
            }
            129 => {
                values[38] = {
                    let numerator = values[41];
                    let denominator = 0x5u64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 128;
            }
            130 => {
                values[44] = u64::from(
                    ((((values[45]) as i32 as i64 as u64).wrapping_add(0x1u64)) as i32 as i64
                        as u64) as u32,
                );
                pc = 129;
            }
            131 => {
                pc = 44;
            }
            132 => {
                values[35] = (values[35]).wrapping_sub(u64::from(
                    (u64::from((values[33]) as u32)) <= (u64::from((values[44]) as u32)),
                ));
                pc = 131;
            }
            133 => {
                pc = if (u64::from(
                    (((values[39]) as i32 as i64 as u64) as i64)
                        == ((((((values[47]) as i32 as i64 as u64).wrapping_mul(0x5u64)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    138
                } else {
                    132
                };
            }
            134 => {
                values[38] = values[47];
                pc = 133;
            }
            135 => {
                values[39] = ((values[38]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 134;
            }
            136 => {
                values[47] = {
                    let numerator = values[38];
                    let denominator = 0x5u64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 135;
            }
            137 => {
                values[41] = values[44];
                pc = 136;
            }
            138 => {
                values[44] = u64::from(
                    ((((values[41]) as i32 as i64 as u64).wrapping_add(0x1u64)) as i32 as i64
                        as u64) as u32,
                );
                pc = 137;
            }
            139 => {
                pc = if (u64::from(
                    ((((values[47]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((((({
                            let numerator = values[47];
                            let denominator = 0x5u64;
                            if denominator == 0 {
                                return Err(AdapterError::Arithmetic("division by zero"));
                            }
                            numerator
                                .checked_div(denominator)
                                .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                        }) as i32 as i64 as u64)
                            .wrapping_mul(0x5u64)) as i32 as i64
                            as u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    138
                } else {
                    132
                };
            }
            140 => {
                values[41] = values[42];
                pc = 139;
            }
            141 => {
                values[38] = {
                    let numerator = values[47];
                    let denominator = 0x5u64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 140;
            }
            142 => {
                values[46] =
                    u64::from((u64::from((values[33]) as u32)) <= (u64::from((values[44]) as u32)));
                pc = 131;
            }
            143 => {
                pc = if (u64::from(
                    (((values[39]) as i32 as i64 as u64) as i64)
                        == ((((((values[47]) as i32 as i64 as u64).wrapping_mul(0x5u64)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    148
                } else {
                    142
                };
            }
            144 => {
                values[46] = values[44];
                pc = 143;
            }
            145 => {
                values[38] = values[47];
                pc = 144;
            }
            146 => {
                values[39] = ((values[38]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 145;
            }
            147 => {
                values[47] = {
                    let numerator = values[38];
                    let denominator = 0x5u64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 146;
            }
            148 => {
                values[44] = u64::from(
                    ((((values[46]) as i32 as i64 as u64).wrapping_add(0x1u64)) as i32 as i64
                        as u64) as u32,
                );
                pc = 147;
            }
            149 => {
                pc = if (u64::from(
                    ((((values[40]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == ((((((values[38]) as i32 as i64 as u64).wrapping_mul(0x5u64)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    148
                } else {
                    142
                };
            }
            150 => {
                values[46] = values[42];
                pc = 149;
            }
            151 => {
                values[38] = {
                    let numerator = values[40];
                    let denominator = 0x5u64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 150;
            }
            152 => {
                pc = if (u64::from(
                    (((values[48]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    141
                } else {
                    151
                };
            }
            153 => {
                values[44] = u64::from((0x0u64) as u32);
                pc = 152;
            }
            154 => {
                pc = if (u64::from(
                    ((((values[38]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != ((((((values[41]) as i32 as i64 as u64).wrapping_mul(0x5u64)) as i32
                            as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    153
                } else {
                    130
                };
            }
            155 => {
                values[45] = values[42];
                pc = 154;
            }
            156 => {
                values[41] = {
                    let numerator = values[38];
                    let denominator = 0x5u64;
                    if denominator == 0 {
                        return Err(AdapterError::Arithmetic("division by zero"));
                    }
                    numerator
                        .checked_div(denominator)
                        .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                };
                pc = 155;
            }
            157 => {
                pc = 44;
            }
            158 => {
                pc = if (u64::from(
                    (u64::from((0x15u64) as u32)) < (u64::from((values[33]) as u32)),
                )) != 0
                {
                    157
                } else {
                    156
                };
            }
            159 => {
                values[37] = ((values[41])
                    .wrapping_shr((((values[32]) & (0x3fu64)) as i32 as i64 as u64) as u32))
                    | ((values[31] as i64).wrapping_shl(
                        (((0x40u64).wrapping_sub(((values[32]) & (0x3fu64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as u32,
                    ) as u64);
                pc = 158;
            }
            160 => {
                values[31] = (values[31]).wrapping_add(0x1u64);
                pc = 159;
            }
            161 => {
                pc = if (u64::from((values[41]) < (values[37]))) != 0 {
                    160
                } else {
                    159
                };
            }
            162 => {
                values[41] = ((values[7]).wrapping_mul(values[19])).wrapping_add(values[37]);
                pc = 161;
            }
            163 => {
                values[37] = (((values[8]) as u128 * (values[20]) as u128) >> 64) as u64;
                pc = 162;
            }
            164 => {
                values[20] = values[38];
                pc = 163;
            }
            165 => {
                values[8] = values[37];
                pc = 164;
            }
            166 => {
                values[36] = ((values[41])
                    .wrapping_shr((((values[32]) & (0x3fu64)) as i32 as i64 as u64) as u32))
                    | ((values[34] as i64).wrapping_shl(
                        (((0x40u64).wrapping_sub(((values[32]) & (0x3fu64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as u32,
                    ) as u64);
                pc = 165;
            }
            167 => {
                values[31] = (((values[7]) as u128 * (values[19]) as u128) >> 64) as u64;
                pc = 166;
            }
            168 => {
                values[19] = values[38];
                pc = 167;
            }
            169 => {
                values[7] = memory.read_u64(
                    (relocate("DAT_1400e8028")).wrapping_add((values[31]).wrapping_mul(1)),
                )?;
                pc = 168;
            }
            170 => {
                values[34] = (values[34]).wrapping_add(0x1u64);
                pc = 169;
            }
            171 => {
                pc = if (u64::from((values[41]) < (values[36]))) != 0 {
                    170
                } else {
                    169
                };
            }
            172 => {
                values[41] = ((values[5]).wrapping_mul(values[17])).wrapping_add(values[36]);
                pc = 171;
            }
            173 => {
                values[36] = (((values[6]) as u128 * (values[18]) as u128) >> 64) as u64;
                pc = 172;
            }
            174 => {
                values[18] = values[40];
                pc = 173;
            }
            175 => {
                values[6] = values[37];
                pc = 174;
            }
            176 => {
                values[34] = (((values[5]) as u128 * (values[17]) as u128) >> 64) as u64;
                pc = 175;
            }
            177 => {
                values[17] = values[40];
                pc = 176;
            }
            178 => {
                values[5] = memory.read_u64(
                    (relocate("DAT_1400e8028")).wrapping_add((values[31]).wrapping_mul(1)),
                )?;
                pc = 177;
            }
            179 => {
                values[35] = ((values[41])
                    .wrapping_shr((((values[32]) & (0x3fu64)) as i32 as i64 as u64) as u32))
                    | ((values[34] as i64).wrapping_shl(
                        (((0x40u64).wrapping_sub(((values[32]) & (0x3fu64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as u32,
                    ) as u64);
                pc = 178;
            }
            180 => {
                values[40] = ((values[38]).wrapping_sub(values[36])).wrapping_sub(0x1u64);
                pc = 179;
            }
            181 => {
                values[34] = (values[34]).wrapping_add(0x1u64);
                pc = 180;
            }
            182 => {
                pc = if (u64::from((values[41]) < (values[35]))) != 0 {
                    181
                } else {
                    180
                };
            }
            183 => {
                values[41] = ((values[3]).wrapping_mul(values[15])).wrapping_add(values[35]);
                pc = 182;
            }
            184 => {
                values[35] = (((values[4]) as u128 * (values[16]) as u128) >> 64) as u64;
                pc = 183;
            }
            185 => {
                values[16] = values[47];
                pc = 184;
            }
            186 => {
                values[4] = values[37];
                pc = 185;
            }
            187 => {
                values[37] = memory.read_u64(
                    (relocate("DAT_1400e8020")).wrapping_add((values[31]).wrapping_mul(1)),
                )?;
                pc = 186;
            }
            188 => {
                values[34] = (((values[3]) as u128 * (values[15]) as u128) >> 64) as u64;
                pc = 187;
            }
            189 => {
                values[15] = values[47];
                pc = 188;
            }
            190 => {
                values[3] = memory.read_u64(
                    (relocate("DAT_1400e8028")).wrapping_add((values[31]).wrapping_mul(1)),
                )?;
                pc = 189;
            }
            191 => {
                values[32] = u64::from(
                    ((((((((u64::from(
                        ((u64::from(((values[33]).wrapping_mul(0x12934fu64)) as u32) as u32)
                            .wrapping_shr((0x13u64) as u32) as u64) as u32,
                    )) as i8 as i64 as u64)
                        .wrapping_sub((values[39]) as i8 as i64 as u64))
                        as i32 as i64 as u64)
                        .wrapping_add(0x3au64)) as i32 as i64 as u64)
                        .wrapping_add((values[33]) as i8 as i64 as u64)) as i32
                        as i64 as u64) as u8,
                );
                pc = 190;
            }
            192 => {
                values[31] = (values[33]).wrapping_mul(0x10u64);
                pc = 191;
            }
            193 => {
                values[33] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((values[39]).wrapping_mul(0x13441u64)) as i32 as i64 as u64)
                                    as u32,
                            ) as u32)
                                .wrapping_shr((0x12u64) as u32) as u64)
                                as u32,
                        ))
                        .wrapping_sub(u64::from(
                            (u64::from(
                                (((0x3u64) as i32 as i64 as u64) as i64)
                                    < (((values[39]) as i32 as i64 as u64) as i64),
                            )) as u32,
                        ))) as u32,
                    )) as u32,
                );
                pc = 192;
            }
            194 => {
                values[47] = (values[38]).wrapping_add(0x2u64);
                pc = 193;
            }
            195 => {
                pc = if (u64::from(
                    (((values[39]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    90
                } else {
                    194
                };
            }
            196 => {
                values[50] = 0u64;
                pc = 195;
            }
            197 => {
                values[46] = 0x0u64;
                pc = 196;
            }
            198 => {
                values[36] = values[42];
                pc = 197;
            }
            199 => {
                values[32] = u64::from((0x0u64) as u8);
                pc = 198;
            }
            200 => {
                values[32] = u64::from((0x1u64) as u8);
                pc = 197;
            }
            201 => {
                values[36] = 0x1u64;
                pc = 200;
            }
            202 => {
                pc = if (u64::from(
                    ((u64::from((values[1]) == (0x0u64))) != 0)
                        && ((u64::from(
                            (u64::from((0x1u64) as u32)) < (u64::from((values[2]) as u32)),
                        )) != 0),
                )) != 0
                {
                    199
                } else {
                    201
                };
            }
            203 => {
                values[38] = (values[38]).wrapping_mul(0x4u64);
                pc = 202;
            }
            204 => {
                values[48] = u64::from(
                    ((((!(u64::from((values[38]) as u8))) as i32 as i64 as u64) & (0x1u64)) as i32
                        as i64 as u64) as u8,
                );
                pc = 203;
            }
            205 => {
                values[42] = 0x0u64;
                pc = 204;
            }
            206 => {
                values[38] = (values[1]) | (0x10000000000000u64);
                pc = 205;
            }
            207 => {
                pc = if (u64::from((u64::from((values[2]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    206
                } else {
                    205
                };
            }
            208 => {
                values[38] = values[1];
                pc = 207;
            }
            209 => {
                values[39] =
                    (u64::from(((values[2]).wrapping_sub(0x435u64)) as u32)) as i32 as i64 as u64;
                pc = 208;
            }
            210 => {
                pc = if (u64::from((u64::from((values[2]) as u32)) != (u64::from((0x0u64) as u32))))
                    != 0
                {
                    209
                } else {
                    208
                };
            }
            211 => {
                values[39] =
                    ((0u64.wrapping_sub(0x434u64)) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 210;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b0bdc` (`mspaint.c:155443`).
pub fn fun_1400b0bdc(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    ];
    let mut pc: i32 = 61;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                {
                    let destination = relocate("DAT_140179090");
                    let stored =
                        (memory.read_u64(relocate("DAT_140179090"))?) & (0xffffffffffffff7fu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((values[10] as u32).wrapping_shr((0x15u64) as u32) as u64)
                                        as u32,
                                )) & (0x1u64)) as u32,
                            )) as u32,
                        )) != (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            (((values[8]).wrapping_shr((0x13u64) as u32)) & (0x1u64)) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let destination = relocate("DAT_140179090");
                    let stored =
                        (memory.read_u64(relocate("DAT_140179090"))?) & (0xffffffffffffffbfu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                pc = if (u64::from(
                    (((0x1u64) as i32 as i64 as u64) as i64)
                        < (((u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("_DAT_1401cfa50"))?)) as u32,
                            )) as u8,
                        )) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                {
                    let destination = relocate("DAT_140179090");
                    let stored = (!((u64::from(
                        ((u64::from(((values[11]).wrapping_shr((0x10u64) as u32)) as u32))
                            & (0x6u64)) as u32,
                    )) | (0x1000029u64)))
                        & (memory.read_u64(relocate("DAT_140179090"))?);
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = relocate("_DAT_1401cfa50");
                    let stored = u64::from(
                        (u64::from(((u64::from((values[11]) as u32)) & (0x400ffu64)) as u32))
                            as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((values[10] as u32).wrapping_shr((0x13u64) as u32) as u64)
                                        as u32,
                                )) & (0x1u64)) as u32,
                            )) as u32,
                        )) != (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            (((((values[5]) & (0xe0u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                == (((0xe0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    6
                } else {
                    2
                };
            }
            8 => {
                {
                    let destination = relocate("DAT_140179090");
                    let stored =
                        (memory.read_u64(relocate("DAT_140179090"))?) & (0xfffffffffeffffffu64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                ((values[9] as u32).wrapping_shr((0x17u64) as u32) as u64) as u32,
                            )) & (0x1u64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let destination = relocate("DAT_140179090");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("DAT_14017909c");
                    let stored = u64::from((values[4]) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = u64::from(
                    (u64::from((u64::from(memory.read_u8(relocate("DAT_14017909c"))?)) as u8))
                        as u32,
                );
                pc = 11;
            }
            13 => {
                values[6] = (memory.read_u64(relocate("DAT_140179090"))?) & (0xffffffffffffffd9u64);
                pc = 12;
            }
            14 => {
                {
                    let destination = relocate("_DAT_140179098");
                    let stored = u64::from((0x6u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                {
                    let destination = relocate("DAT_14017909c");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u8(relocate("DAT_14017909c"))?)) as u8,
                        )) | (0x68u64)) as i32 as i64 as u64) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from((u64::from(((values[7]) & (0xd0030000u64)) as u32)) as u32))
                            == (u64::from((0xd0030000u64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            (((((values[5]) & (0xe0u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                                as i64)
                                == (((0xe0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    15
                } else {
                    11
                };
            }
            17 => {
                values[4] = u64::from(
                    (((u64::from((u64::from(memory.read_u8(relocate("DAT_14017909c"))?)) as u8))
                        | (0x28u64)) as i32 as i64 as u64) as u32,
                );
                pc = 16;
            }
            18 => {
                values[6] = (memory.read_u64(relocate("DAT_140179090"))?) & (0xfffffffffffffffdu64);
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("_DAT_140179098");
                    let stored = u64::from((0x5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                pc = if (u64::from(
                    (u64::from((u64::from(((values[7]) & (0x20u64)) as u32)) as u32))
                        != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    19
                } else {
                    11
                };
            }
            21 => {
                values[4] = u64::from(
                    (((u64::from((u64::from(memory.read_u8(relocate("DAT_14017909c"))?)) as u8))
                        | (0x8u64)) as i32 as i64 as u64) as u32,
                );
                pc = 20;
            }
            22 => {
                values[6] = memory.read_u64(relocate("DAT_140179090"))?;
                pc = 21;
            }
            23 => {
                {
                    let destination = relocate("_DAT_140179098");
                    let stored = u64::from((0x3u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((values[4] as u32).wrapping_shr((0x1cu64) as u32) as u64)
                                        as u32,
                                )) & (0x1u64)) as u32,
                            )) as u32,
                        )) != (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && (({
                            let _ = {
                                values[5] = u64::from((u64::from((values[8]) as u8)) as u8);
                                values[5]
                            };
                            u64::from(
                                (((((values[5]) & (0x6u64)) as i32 as i64 as u64) as i32 as i64
                                    as u64) as i64)
                                    == (((0x6u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    23
                } else {
                    2
                };
            }
            25 => {
                values[8] = ((values[12]) & (values[8])) & (0xffffffffu64);
                pc = 24;
            }
            26 => {
                values[8] = {
                    let call_arguments = [0x0u64];
                    invoke("xinuse", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                ((values[4] as u32).wrapping_shr((0x1bu64) as u32) as u64) as u32,
                            )) & (0x1u64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    26
                } else {
                    0
                };
            }
            28 => {
                {
                    let destination = relocate("DAT_140179090");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                values[8] = (memory.read_u64(relocate("DAT_140179090"))?) & (0xffffffffffffffeeu64);
                pc = 28;
            }
            30 => {
                {
                    let destination = relocate("DAT_14017909c");
                    let stored = u64::from((0x6u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("_DAT_140179098");
                    let stored = u64::from((0x2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                ((values[4] as u32).wrapping_shr((0x14u64) as u32) as u64) as u32,
                            )) & (0x1u64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    31
                } else {
                    28
                };
            }
            33 => {
                values[8] = (memory.read_u64(relocate("DAT_140179090"))?) & (0xfffffffffffffffeu64);
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("DAT_14017909c");
                    let stored = u64::from((0x2u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("_DAT_140179098");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 34;
            }
            36 => {
                values[11] = u64::from(
                    (u64::from(memory.read_u32((values[3]).wrapping_add(0x4u64))?)) as u32,
                );
                pc = 35;
            }
            37 => {
                values[3] = {
                    let call_arguments = [0x24u64];
                    invoke("cpuid", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    (((0x23u64) as i32 as i64 as u64) as i64)
                        < ((((u64::from(memory.read_u32(values[0])?)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    35
                };
            }
            39 => {
                values[10] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[3]).wrapping_add(0x8u64))?)) as u32,
                    )) as u32,
                );
                pc = 38;
            }
            40 => {
                values[3] = {
                    let call_arguments = [0x7u64];
                    invoke(
                        "cpuid_Extended_Feature_Enumeration_info",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 39;
            }
            41 => {
                pc = if (u64::from(
                    (((0x0u64) as i32 as i64 as u64) as i64)
                        < ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    38
                };
            }
            42 => {
                {
                    let destination = relocate("DAT_1401cfa54");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("DAT_1401cfa54"))?)) as u32,
                            )) | (0x2u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                ((values[7] as u32).wrapping_shr((0x9u64) as u32) as u64) as u32,
                            )) & (0x1u64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                values[9] = u64::from(
                    ((u64::from(
                        memory.read_u32((values[2]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64) as u32,
                );
                pc = 43;
            }
            45 => {
                values[7] = u64::from(
                    ((u64::from(
                        memory.read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64) as u32,
                );
                pc = 44;
            }
            46 => {
                values[2] = {
                    let call_arguments = [0x7u64];
                    invoke(
                        "cpuid_Extended_Feature_Enumeration_info",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 45;
            }
            47 => {
                pc = if (u64::from(
                    (((0x6u64) as i32 as i64 as u64) as i64)
                        < ((((u64::from(memory.read_u32(values[0])?)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    35
                };
            }
            48 => {
                values[11] = 0x0u64;
                pc = 47;
            }
            49 => {
                values[10] = u64::from((0x0u64) as u32);
                pc = 48;
            }
            50 => {
                values[9] = u64::from((0x0u64) as u32);
                pc = 49;
            }
            51 => {
                values[7] = u64::from((0x0u64) as u32);
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("DAT_1401cfa54");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("DAT_1401cfa54"))?)) as u32,
                            )) | (0x1u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from(
                                (u64::from((values[7]) as u32)) == (u64::from((0x106c0u64) as u32)),
                            )) != 0)
                                || ((u64::from(
                                    (u64::from((values[7]) as u32))
                                        == (u64::from((0x20660u64) as u32)),
                                )) != 0),
                        )) != 0)
                            || ((u64::from(
                                (u64::from((values[7]) as u32)) == (u64::from((0x20670u64) as u32)),
                            )) != 0),
                    )) != 0)
                        || ((u64::from(
                            ((u64::from(
                                (u64::from(
                                    (u64::from(((values[7]).wrapping_sub(0x30650u64)) as u32))
                                        as u32,
                                )) < (u64::from((0x21u64) as u32)),
                            )) != 0)
                                && ((u64::from(
                                    (((0x100010001u64).wrapping_shr(
                                        ((u64::from(((values[7]).wrapping_sub(0x30650u64)) as u32))
                                            & (0x3fu64))
                                            as u32,
                                    )) & (0x1u64))
                                        != (0x0u64),
                                )) != 0),
                        )) != 0),
                )) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                {
                    let destination = relocate("_DAT_1401790a8");
                    let stored = 0xffffffffffffffffu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("_DAT_1401790a0");
                    let stored = 0x8000u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                values[7] = u64::from(
                    (u64::from(
                        ((u64::from((u64::from(memory.read_u32(values[1])?)) as u32))
                            & (0xfff3ff0u64)) as u32,
                    )) as u32,
                );
                pc = 55;
            }
            57 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((((u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?,
                            )) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x49656e69u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                ((((u64::from(memory.read_u32(
                                    (values[0]).wrapping_add((0x3u64).wrapping_mul(4)),
                                )?)) as i32 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    == (((0x6c65746eu64) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(
                                memory
                                    .read_u32((values[0]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x756e6547u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    56
                } else {
                    51
                };
            }
            58 => {
                values[4] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[1]).wrapping_add((0x3u64).wrapping_mul(4)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 57;
            }
            59 => {
                values[1] = {
                    let call_arguments = [0x1u64];
                    invoke("cpuid_Version_info", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                values[0] = {
                    let call_arguments = [0x0u64];
                    invoke("cpuid_basic_info", &call_arguments, memory)
                };
                pc = 59;
            }
            61 => {
                values[12] = relocate("CONTEXT_in_XCR0");
                pc = 60;
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
    ("FUN_14000d4c0", 1, fun_14000d4c0),
    ("FUN_14000d9e0", 3, fun_14000d9e0),
    ("FUN_1400268a0", 3, fun_1400268a0),
    ("FUN_140026da0", 3, fun_140026da0),
    ("FUN_14002dcb0", 3, fun_14002dcb0),
    ("FUN_1400b0bdc", 0, fun_1400b0bdc),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_14000d9e0",
        &[("STACK_local_res20", 32), ("STACK_local_108", -264)],
    ),
    (
        "FUN_140026da0",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
            ("STACK_local_20", -32),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_14000d9e0",
        &[("STACK_local_res20", 4), ("STACK_local_108", 8)],
    ),
    (
        "FUN_140026da0",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 4),
            ("STACK_local_48", 16),
            ("STACK_local_38", 24),
            ("STACK_local_20", 8),
        ],
    ),
];
