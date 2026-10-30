//! Checked translations generated from original bodies by batch 028.
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
/// Translation of `FUN_14008df70` (`mspaint.c:122961`).
pub fn fun_14008df70(
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
    let mut pc: i32 = 151;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                pc = 73;
            }
            2 => {
                pc = 37;
            }
            3 => {
                pc = if (u64::from((values[13]) == (memory.read_u64(relocate("STACK_local_48"))?)))
                    != 0
                {
                    2
                } else {
                    1
                };
            }
            4 => {
                values[13] = memory.read_u64(values[13])?;
                pc = 3;
            }
            5 => {
                values[14] = memory.read_u64(values[14])?;
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[14]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[13]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                return Ok(None);
            }
            8 => {
                {
                    let destination = values[4];
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = (values[6]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = values[6];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                values[4] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[14]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = (memory
                        .read_u64((values[14]).wrapping_add((0x6u64).wrapping_mul(8)))?)
                    .wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[6]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[6] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [relocate("\"list too long\"")];
                    invoke("std::_Xlength_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xlength_error"));
            }
            18 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[14]).wrapping_add((0x6u64).wrapping_mul(8)))?)
                        as i64)
                        == ((0xaaaaaaaaaaaaaaau64) as i64),
                )) != 0
                {
                    17
                } else {
                    16
                };
            }
            19 => {
                values[7] = memory.read_u64(memory.read_u64(relocate("STACK_local_78"))?)?;
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = (values[14]).wrapping_add((0x5u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_48"))?, 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 7;
            }
            22 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    25
                } else {
                    21
                };
            }
            23 => {
                values[4] = values[8];
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [values[4], 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                values[8] = memory.read_u64(values[4])?;
                pc = 24;
            }
            26 => {
                values[4] = memory.read_u64(memory.read_u64(relocate("STACK_local_48"))?)?;
                pc = 22;
            }
            27 => {
                {
                    let destination = memory.read_u64(
                        (memory.read_u64(relocate("STACK_local_48"))?)
                            .wrapping_add((0x1u64).wrapping_mul(8)),
                    )?;
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = (values[11]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = (memory
                        .read_u64((values[11]).wrapping_add((0x6u64).wrapping_mul(8)))?)
                    .wrapping_sub(values[15]);
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                pc = if (u64::from((values[7]) != (values[6]))) != 0 {
                    33
                } else {
                    28
                };
            }
            30 => {
                values[14] = values[7];
                pc = 29;
            }
            31 => {
                values[15] = (values[15]).wrapping_add(0x1u64);
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [values[14], 0x18u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                values[7] = memory.read_u64(values[14])?;
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[6]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = values[7];
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                values[7] = memory.read_u64((values[14]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 35;
            }
            37 => {
                pc = if (u64::from((values[14]) != (values[6]))) != 0 {
                    36
                } else {
                    27
                };
            }
            38 => {
                pc = 27;
            }
            39 => {
                {
                    let destination = (values[11]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = (memory
                        .read_u64((values[11]).wrapping_add((0x6u64).wrapping_mul(8)))?)
                    .wrapping_add(memory.read_u64(relocate("STACK_uStack_60"))?);
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_58"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_58"))?;
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = values[4];
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = (values[6]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                values[4] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 43;
            }
            45 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_uStack_60"))?) != (0x0u64)))
                    != 0
                {
                    44
                } else {
                    38
                };
            }
            46 => {
                values[7] = memory.read_u64(values[16])?;
                pc = 45;
            }
            47 => {
                values[6] = 0x0u64;
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            49 => {
                pc = if ({
                    let _ = {
                        let _ = {
                            let destination = relocate("STACK_uStack_60");
                            let stored = values[15];
                            memory.write_u64(destination, stored)?;
                            stored
                        };
                        {
                            let destination = relocate("STACK_local_58");
                            let stored = values[14];
                            memory.write_u64(destination, stored)?;
                            stored
                        }
                    };
                    u64::from((values[13]) != (values[7]))
                }) != 0
                {
                    57
                } else {
                    48
                };
            }
            50 => {
                let _ = {
                    values[13] = memory.read_u64(values[13])?;
                    values[13]
                };
                pc = 49;
            }
            51 => {
                values[14] = values[12];
                pc = 50;
            }
            52 => {
                values[15] = (values[15]).wrapping_add(0x1u64);
                pc = 51;
            }
            53 => {
                {
                    let destination = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = values[14];
                    let stored = values[12];
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[12]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[13]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                values[12] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                let _ = {
                    values[13] = memory.read_u64(values[13])?;
                    values[13]
                };
                pc = 49;
            }
            59 => {
                values[15] = memory.read_u64(relocate("STACK_uStack_60"))?;
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_plStack_50");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                values[14] = values[6];
                pc = 60;
            }
            62 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (values[6]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[13]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                values[6] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                pc = if (u64::from((values[13]) == (memory.read_u64(relocate("STACK_local_48"))?)))
                    != 0
                {
                    47
                } else {
                    66
                };
            }
            68 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("STACK_plStack_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                pc = 6;
            }
            73 => {
                pc = if (u64::from((values[14]) != (values[6]))) != 0 {
                    72
                } else {
                    71
                };
            }
            74 => {
                pc = if (u64::from((values[13]) != (memory.read_u64(relocate("STACK_local_48"))?)))
                    != 0
                {
                    73
                } else {
                    37
                };
            }
            75 => {
                values[14] = memory.read_u64(values[6])?;
                pc = 74;
            }
            76 => {
                values[6] = memory.read_u64(values[16])?;
                pc = 75;
            }
            77 => {
                values[13] = memory.read_u64(memory.read_u64(relocate("STACK_local_48"))?)?;
                pc = 76;
            }
            78 => {
                pc = if (u64::from((values[16]) != (relocate("STACK_local_48")))) != 0 {
                    77
                } else {
                    27
                };
            }
            79 => {
                values[16] = (values[11]).wrapping_add((0x5u64).wrapping_mul(8));
                pc = 78;
            }
            80 => {
                values[7] = memory.read_u64(relocate("STACK_local_48"))?;
                pc = 79;
            }
            81 => {
                values[11] = {
                    let call_arguments = [values[0], relocate("STACK_local_78"), values[7]];
                    invoke("FUN_140022030", &call_arguments, memory)
                };
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = memory.read_u64(relocate("STACK_uStack_60"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = memory.read_u64(relocate("STACK_local_68"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = (values[7]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[7]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = values[7];
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (values[7]).wrapping_add((0x5u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = (values[10]).wrapping_add(0x8u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = values[10];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                values[10] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 90;
            }
            92 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = (values[7]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = (values[7]).wrapping_add((0x5u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = (values[7]).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = (values[1]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 94;
            }
            96 => {
                values[7] = {
                    let call_arguments = [0x38u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400222b0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_1400222b0"));
            }
            100 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        == (0x492492492492492u64),
                )) != 0
                {
                    99
                } else {
                    98
                };
            }
            101 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8((values[11]).wrapping_add(0x19u64))?)) as i8
                            as i64 as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            (((values[1]) as i32 as i64 as u64) as i64)
                                < ((((u64::from(memory.read_u32(
                                    (values[11]).wrapping_add((0x4u64).wrapping_mul(8)),
                                )?)) as i32 as i64 as u64)
                                    as i32 as i64 as u64)
                                    as i64),
                        )) != 0),
                )) != 0
                {
                    100
                } else {
                    80
                };
            }
            102 => {
                pc = if ({
                    let _ = {
                        values[5] = values[9];
                        values[5]
                    };
                    u64::from(
                        (((values[3]) as i32 as i64 as u64) as i64)
                            == (((0x0u64) as i32 as i64 as u64) as i64),
                    )
                }) != 0
                {
                    112
                } else {
                    101
                };
            }
            103 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                values[9] = memory.read_u64(values[9])?;
                pc = 103;
            }
            105 => {
                values[3] = ((u64::from(
                    memory.read_u8((memory.read_u64(values[9])?).wrapping_add(0x19u64))?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 104;
            }
            106 => {
                values[9] = (values[5]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 105;
            }
            107 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_uStack_60")).wrapping_add(0x4u64))?,
                        )) as u32,
                    ))
                    .wrapping_shl((0x20u64) as u32);
                    memory.write_u64(destination, stored)?;
                }
                pc = 106;
            }
            108 => {
                values[11] = values[5];
                pc = 105;
            }
            109 => {
                values[9] = values[5];
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_uStack_60")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[5]).wrapping_add((0x4u64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((values[1]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    107
                } else {
                    110
                };
            }
            112 => {
                {
                    let destination = (relocate("STACK_uStack_60")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_uStack_60"))?)
                                .wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            114 => {
                values[11] = values[16];
                pc = 113;
            }
            115 => {
                values[3] = ((u64::from(memory.read_u8((values[9]).wrapping_add(0x19u64))?)) as i8
                    as i64 as u64) as i8 as i64 as u64;
                pc = 114;
            }
            116 => {
                {
                    let destination = relocate("STACK_uStack_60");
                    let stored = (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_uStack_60")).wrapping_add(0x4u64))?,
                        )) as u32,
                    ))
                    .wrapping_shl((0x20u64) as u32);
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                values[9] = memory.read_u64((values[16]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 116;
            }
            118 => {
                values[16] = memory.read_u64(values[0])?;
                pc = 117;
            }
            119 => {
                {
                    let destination = values[4];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 119;
            }
            121 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 120;
            }
            122 => {
                {
                    let destination = values[8];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 121;
            }
            123 => {
                values[4] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 122;
            }
            124 => {
                {
                    let destination = relocate("STACK_lStack_40");
                    let stored =
                        (memory.read_u64(relocate("STACK_lStack_40"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 123;
            }
            125 => {
                {
                    let destination = (values[8]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 124;
            }
            126 => {
                values[8] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 125;
            }
            127 => {
                {
                    let destination = relocate("STACK_plStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                values[15] = 0x0u64;
                pc = 127;
            }
            129 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("STACK_local_48");
                    memory.write_u64(destination, stored)?;
                }
                pc = 128;
            }
            130 => {
                {
                    let call_arguments = [relocate("\"list too long\"")];
                    invoke("std::_Xlength_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xlength_error"));
            }
            131 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_lStack_40"))?) as i64)
                        == ((0xaaaaaaaaaaaaaaau64) as i64),
                )) != 0
                {
                    130
                } else {
                    129
                };
            }
            132 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 131;
            }
            133 => {
                {
                    let destination = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 132;
            }
            134 => {
                {
                    let destination = values[7];
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                values[7] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 134;
            }
            136 => {
                {
                    let destination = relocate("STACK_lStack_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 135;
            }
            137 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((((u64::from(memory.read_u8((values[14]).wrapping_add(0x19u64))?))
                                as i8 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0)
                            && ((u64::from(
                                ((((memory.read_u64(
                                    (values[14]).wrapping_add((0x4u64).wrapping_mul(8)),
                                )?) as i32 as i64 as u64) as i32
                                    as i64 as u64) as i64)
                                    <= (((values[1]) as i32 as i64 as u64) as i64),
                            )) != 0),
                    )) != 0)
                        && ((u64::from((values[14]) != (values[7]))) != 0),
                )) != 0
                {
                    20
                } else {
                    137
                };
            }
            139 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    146
                } else {
                    138
                };
            }
            140 => {
                values[3] = ((u64::from(memory.read_u8((values[6]).wrapping_add(0x19u64))?)) as i8
                    as i64 as u64) as i8 as i64 as u64;
                pc = 139;
            }
            141 => {
                values[14] = values[13];
                pc = 140;
            }
            142 => {
                values[6] = memory.read_u64(values[6])?;
                pc = 141;
            }
            143 => {
                values[13] = values[14];
                pc = 142;
            }
            144 => {
                values[6] = (values[6]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 143;
            }
            145 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[6]).wrapping_add((0x4u64).wrapping_mul(8)))?) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((values[1]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    144
                } else {
                    142
                };
            }
            146 => {
                values[13] = values[6];
                pc = 145;
            }
            147 => {
                values[14] = values[7];
                pc = 139;
            }
            148 => {
                values[3] = ((u64::from(memory.read_u8((values[6]).wrapping_add(0x19u64))?)) as i8
                    as i64 as u64) as i8 as i64 as u64;
                pc = 147;
            }
            149 => {
                values[6] = memory.read_u64((values[7]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 148;
            }
            150 => {
                values[7] = memory.read_u64(values[0])?;
                pc = 149;
            }
            151 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_98"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 150;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400c4060` (`mspaint.c:170811`).
pub fn fun_1400c4060(
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
    let mut pc: i32 = 240;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x2a0u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x102u64))?)) as i16
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
                    let call_arguments = [(values[0]).wrapping_add((0x10u64).wrapping_mul(8))];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x14u64).wrapping_mul(8))];
                    invoke("FUN_140017c60", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[0]).wrapping_add((0x1fu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[9])?).wrapping_add(0x20u64))?,
                        values[9],
                        u64::from(
                            (values[9]) != ((values[0]).wrapping_add((0x18u64).wrapping_mul(8))),
                        ),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = if (u64::from((values[9]) != (0x0u64))) != 0 {
                    6
                } else {
                    4
                };
            }
            8 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x1fu64).wrapping_mul(8)))?;
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140008c50", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
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
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 14;
            }
            16 => {
                values[1] = (memory
                    .read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                .wrapping_add(0xcu64);
                pc = 15;
            }
            17 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 16;
            }
            18 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(
                        memory.read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?,
                    )?)?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
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
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 21;
            }
            23 => {
                values[1] = (memory
                    .read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                .wrapping_add(0x8u64);
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    24
                } else {
                    10
                };
            }
            26 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    26
                } else {
                    25
                };
            }
            28 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x29u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [
                        values[8],
                        (values[0]).wrapping_add((0x4fu64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            30 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x29u64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x1d8u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x29u64).wrapping_mul(8)))?,
                        0x2u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 30;
            }
            32 => {
                {
                    let destination = (values[0]).wrapping_add((0x51u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = (values[0]).wrapping_add((0x50u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = (values[0]).wrapping_add((0x4fu64).wrapping_mul(8));
                    let stored = u64::from((0x5d5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = (values[0]).wrapping_add((0x2au64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    36
                } else {
                    35
                };
            }
            38 => {
                pc = if (u64::from(((values[10]) as i64) == ((0x1u64) as i64))) != 0 {
                    43
                } else {
                    37
                };
            }
            39 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 38;
            }
            40 => {
                {
                    let destination = values[9];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                values[10] = memory.read_u64(values[9])?;
                pc = 40;
            }
            42 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 41;
            }
            43 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            45 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 44;
            }
            46 => {
                {
                    let destination = values[9];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                values[10] = memory.read_u64(values[9])?;
                pc = 46;
            }
            48 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 47;
            }
            49 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x30u64).wrapping_mul(8)))?;
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x30u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    49
                } else {
                    37
                };
            }
            51 => {
                pc = 9;
            }
            52 => {
                return Ok(None);
            }
            53 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    52
                } else {
                    51
                };
            }
            54 => {
                values[7] = ({
                    let call_arguments = [];
                    invoke("FUN_14000c570", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 53;
            }
            55 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = values[2];
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 55;
            }
            57 => {
                {
                    let destination = (values[0]).wrapping_add((0x3cu64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 57;
            }
            59 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    58
                } else {
                    57
                };
            }
            60 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 59;
            }
            61 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 60;
            }
            62 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 61;
            }
            63 => {
                values[1] = (memory
                    .read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                .wrapping_add(0xcu64);
                pc = 62;
            }
            64 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 63;
            }
            65 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(
                        memory.read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?,
                    )?)?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    65
                } else {
                    57
                };
            }
            67 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 66;
            }
            68 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 67;
            }
            69 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 68;
            }
            70 => {
                values[1] = (memory
                    .read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                .wrapping_add(0x8u64);
                pc = 69;
            }
            71 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 70;
            }
            72 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x27u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    71
                } else {
                    57
                };
            }
            73 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[0]).wrapping_add((0x1fu64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x10u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 72;
            }
            74 => {
                {
                    let call_arguments = [];
                    invoke("std::_Xbad_function_call", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xbad_function_call"));
            }
            75 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1fu64).wrapping_mul(8)))?)
                        == (0x0u64),
                )) != 0
                {
                    74
                } else {
                    73
                };
            }
            76 => {
                {
                    let destination = (values[0]).wrapping_add((0x4du64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x10u64).wrapping_mul(8)))?;
                pc = 76;
            }
            78 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[0]).wrapping_add((0x13u64).wrapping_mul(8)))?),
                )) != 0
                {
                    77
                } else {
                    76
                };
            }
            79 => {
                values[14] = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
                pc = 78;
            }
            80 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 79;
            }
            81 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x28u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    80
                } else {
                    79
                };
            }
            82 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x29u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [
                        values[8],
                        (values[0]).wrapping_add((0x4au64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            84 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    83
                } else {
                    82
                };
            }
            85 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (values[0]).wrapping_add((0x29u64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x1d8u64),
                        )?,
                        memory.read_u64((values[0]).wrapping_add((0x29u64).wrapping_mul(8)))?,
                        0x2u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 84;
            }
            86 => {
                {
                    let destination = (values[0]).wrapping_add((0x4cu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 85;
            }
            87 => {
                {
                    let destination = (values[0]).wrapping_add((0x4bu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (values[0]).wrapping_add((0x4au64).wrapping_mul(8));
                    let stored = u64::from((0x5d5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = (values[0]).wrapping_add((0x2au64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = (values[0]).wrapping_add((0x34u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let call_arguments = [values[12], 0x0u64, values[4]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 90;
            }
            92 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 91;
            }
            93 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            94 => {
                pc = if (u64::from(
                    (((((values[8]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    93
                } else {
                    90
                };
            }
            95 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    92
                } else {
                    94
                };
            }
            96 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 95;
            }
            97 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 96;
            }
            98 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 97;
            }
            99 => {
                values[1] = (values[4]).wrapping_add(0x18u64);
                pc = 98;
            }
            100 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 99;
            }
            101 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    100
                } else {
                    89
                };
            }
            102 => {
                values[4] = memory.read_u64((values[0]).wrapping_add((0x34u64).wrapping_mul(8)))?;
                pc = 101;
            }
            103 => {
                {
                    let call_arguments = [
                        u64::from(
                            (relocate("CONTEXT_IGNORED_FUN_1400c4060_extraout_XMM0_Da_00")) as u32,
                        ),
                        (values[0]).wrapping_add((0x35u64).wrapping_mul(8)),
                        values[11],
                    ];
                    invoke("FUN_140064070", &call_arguments, memory)
                };
                pc = 102;
            }
            104 => {
                values[11] = {
                    let call_arguments =
                        [memory.read_u64((values[0]).wrapping_add((0x3eu64).wrapping_mul(8)))?];
                    invoke(
                        "Paint::PlatformHelpers::CodecInfo::ProductId",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 103;
            }
            105 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    104
                } else {
                    89
                };
            }
            106 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 105;
            }
            107 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x2fu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    106
                } else {
                    105
                };
            }
            108 => {
                pc = if (u64::from(((values[10]) as i64) == ((0x1u64) as i64))) != 0 {
                    113
                } else {
                    107
                };
            }
            109 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 108;
            }
            110 => {
                {
                    let destination = values[9];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                values[10] = memory.read_u64(values[9])?;
                pc = 110;
            }
            112 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 111;
            }
            113 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            115 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 114;
            }
            116 => {
                {
                    let destination = values[9];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 115;
            }
            117 => {
                values[10] = memory.read_u64(values[9])?;
                pc = 116;
            }
            118 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 117;
            }
            119 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x30u64).wrapping_mul(8)))?;
                pc = 118;
            }
            120 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x30u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    119
                } else {
                    107
                };
            }
            121 => {
                values[7] = ({
                    let call_arguments = [(values[0]).wrapping_add((0x30u64).wrapping_mul(8))];
                    invoke("FUN_140008b20", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 120;
            }
            122 => {
                pc = 9;
            }
            123 => {
                pc = 57;
            }
            124 => {
                {
                    let call_arguments =
                        [
                            relocate("DAT_1401d0520"),
                            u64::from(
                                (u64::from(memory.read_u32(
                                    (values[0]).wrapping_add((0x17u64).wrapping_mul(8)),
                                )?)) as u32,
                            ),
                            0x408u64,
                            values[16],
                            values[14],
                        ];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 123;
            }
            125 => {
                values[16] = memory.read_u64(values[16])?;
                pc = 124;
            }
            126 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[0]).wrapping_add((0x13u64).wrapping_mul(8)))?),
                )) != 0
                {
                    125
                } else {
                    124
                };
            }
            127 => {
                values[16] = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
                pc = 126;
            }
            128 => {
                {
                    let destination = (values[0]).wrapping_add((0x25u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 127;
            }
            129 => {
                {
                    let destination = (values[0]).wrapping_add((0x24u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 128;
            }
            130 => {
                {
                    let destination = (values[0]).wrapping_add((0x23u64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 129;
            }
            131 => {
                values[11] = {
                    let call_arguments =
                        [memory.read_u64((values[0]).wrapping_add((0x15u64).wrapping_mul(8)))?];
                    invoke("FUN_140005fd0", &call_arguments, memory)
                };
                pc = 130;
            }
            132 => {
                {
                    let destination = values[14];
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 131;
            }
            133 => {
                values[11] = {
                    let call_arguments = [memory.read_u64(values[3])?];
                    invoke("FUN_140005fd0", &call_arguments, memory)
                };
                pc = 132;
            }
            134 => {
                {
                    let destination = (values[0]).wrapping_add((0x46u64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 133;
            }
            135 => {
                values[14] = (values[0]).wrapping_add((0x22u64).wrapping_mul(8));
                pc = 134;
            }
            136 => {
                pc = 121;
            }
            137 => {
                return Ok(None);
            }
            138 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    137
                } else {
                    136
                };
            }
            139 => {
                values[7] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add((0x30u64).wrapping_mul(8)),
                        values[0],
                    ];
                    invoke("FUN_140009170", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 138;
            }
            140 => {
                {
                    let destination = values[2];
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 139;
            }
            141 => {
                {
                    let destination = (values[0]).wrapping_add((0x33u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 140;
            }
            142 => {
                {
                    let destination = (values[0]).wrapping_add((0x32u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 141;
            }
            143 => {
                {
                    let destination = (values[0]).wrapping_add((0x31u64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 142;
            }
            144 => {
                {
                    let destination = (values[0]).wrapping_add((0x30u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 143;
            }
            145 => {
                {
                    let call_arguments = [relocate("STACK_local_c8"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            146 => {
                {
                    let call_arguments = [relocate("STACK_local_c8")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 145;
            }
            147 => {
                {
                    let destination = (values[0]).wrapping_add((0x49u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = (values[0]).wrapping_add((0x48u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = (values[0]).wrapping_add((0x47u64).wrapping_mul(8));
                    let stored = u64::from((0x1628u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 148;
            }
            150 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    149
                } else {
                    144
                };
            }
            151 => {
                values[11] = {
                    let call_arguments = [
                        values[6],
                        (values[0]).wrapping_add((0x2fu64).wrapping_mul(8)),
                        values[14],
                    ];
                    invoke(
                        "Paint::PlatformHelpers::PackageInstaller::TryInstallCodecAsync",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 150;
            }
            152 => {
                {
                    let destination = (values[0]).wrapping_add((0x2du64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 151;
            }
            153 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 152;
            }
            154 => {
                values[11] = {
                    let call_arguments =
                        [memory.read_u64((values[0]).wrapping_add((0x15u64).wrapping_mul(8)))?];
                    invoke("FUN_140005fd0", &call_arguments, memory)
                };
                pc = 153;
            }
            155 => {
                {
                    let destination = values[14];
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 154;
            }
            156 => {
                values[11] = {
                    let call_arguments = [memory.read_u64(values[3])?];
                    invoke("FUN_140005fd0", &call_arguments, memory)
                };
                pc = 155;
            }
            157 => {
                {
                    let destination = (values[0]).wrapping_add((0x2eu64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                values[14] = (values[0]).wrapping_add((0x2bu64).wrapping_mul(8));
                pc = 157;
            }
            159 => {
                {
                    let destination = (values[0]).wrapping_add((0x2au64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 158;
            }
            160 => {
                {
                    let destination = (values[0]).wrapping_add((0x29u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 159;
            }
            161 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0x8u64))?,
                        values[10],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 160;
            }
            162 => {
                pc = if (u64::from(((values[10]) as i64) != ((0x0u64) as i64))) != 0 {
                    161
                } else {
                    160
                };
            }
            163 => {
                {
                    let call_arguments = [
                        values[8],
                        (values[0]).wrapping_add((0x43u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            164 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    163
                } else {
                    162
                };
            }
            165 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0x1d0u64))?,
                        values[10],
                        0x2u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 164;
            }
            166 => {
                {
                    let destination = (values[0]).wrapping_add((0x45u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 165;
            }
            167 => {
                {
                    let destination = (values[0]).wrapping_add((0x44u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 166;
            }
            168 => {
                {
                    let destination = (values[0]).wrapping_add((0x43u64).wrapping_mul(8));
                    let stored = u64::from((0x5c5u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 167;
            }
            169 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 168;
            }
            170 => {
                pc = if (u64::from(((values[10]) as i64) != ((0x0u64) as i64))) != 0 {
                    169
                } else {
                    168
                };
            }
            171 => {
                {
                    let destination = (values[0]).wrapping_add((0x28u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                values[10] = memory.read_u64(relocate("DAT_1401d0b70"))?;
                pc = 171;
            }
            173 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 172;
            }
            174 => {
                {
                    let destination = (values[0]).wrapping_add((0x26u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("DAT_1401d0ba0"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 173;
            }
            175 => {
                values[6] = memory.read_u64(relocate("DAT_1401d0ba0"))?;
                pc = 174;
            }
            176 => {
                values[10] = memory.read_u64(relocate("DAT_1401d0ba8"))?;
                pc = 175;
            }
            177 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 176;
            }
            178 => {
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
                pc = 177;
            }
            179 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 178;
            }
            180 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("DAT_1401d0ba8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    179
                } else {
                    176
                };
            }
            181 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 180;
            }
            182 => {
                pc = if (u64::from(((values[10]) as i64) == ((0x0u64) as i64))) != 0 {
                    181
                } else {
                    135
                };
            }
            183 => {
                {
                    let destination = (values[0]).wrapping_add((0x21u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 182;
            }
            184 => {
                {
                    let call_arguments = [values[12], 0x0u64, values[4]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 183;
            }
            185 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 184;
            }
            186 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            187 => {
                pc = if (u64::from(
                    (((((values[8]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    186
                } else {
                    183
                };
            }
            188 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    185
                } else {
                    187
                };
            }
            189 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 188;
            }
            190 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 189;
            }
            191 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 190;
            }
            192 => {
                values[1] = (values[4]).wrapping_add(0x18u64);
                pc = 191;
            }
            193 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 192;
            }
            194 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    193
                } else {
                    182
                };
            }
            195 => {
                values[4] = memory.read_u64((values[0]).wrapping_add((0x21u64).wrapping_mul(8)))?;
                pc = 194;
            }
            196 => {
                values[10] = memory.read_u64((values[10]).wrapping_add(0x8u64))?;
                pc = 195;
            }
            197 => {
                values[10] = {
                    let call_arguments = [
                        u64::from(
                            (relocate("CONTEXT_IGNORED_FUN_1400c4060_extraout_XMM0_Da")) as u32,
                        ),
                        (values[0]).wrapping_add((0x40u64).wrapping_mul(8)),
                        values[9],
                        values[15],
                    ];
                    invoke("FUN_140063f70", &call_arguments, memory)
                };
                pc = 196;
            }
            198 => {
                pc = if (u64::from(
                    (values[13])
                        < ((u64::from(
                            (u64::from(memory.read_u32((values[10]).wrapping_add(0x4u64))?)) as u32,
                        ))
                        .wrapping_mul(0x2u64)),
                )) != 0
                {
                    200
                } else {
                    197
                };
            }
            199 => {
                values[13] = (values[13]).wrapping_add(0x1u64);
                pc = 198;
            }
            200 => {
                values[15] = ((values[15])
                    ^ (u64::from(
                        (u64::from(
                            memory.read_u8(
                                (memory.read_u64((values[10]).wrapping_add(0x10u64))?)
                                    .wrapping_add(values[13]),
                            )?,
                        )) as u8,
                    )))
                .wrapping_mul(0x100000001b3u64);
                pc = 199;
            }
            201 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[10]).wrapping_add(0x4u64))?)) as u32,
                    )) != (0x0u64),
                )) != 0
                {
                    200
                } else {
                    197
                };
            }
            202 => {
                pc = if (u64::from(((values[10]) as i64) != ((0x0u64) as i64))) != 0 {
                    201
                } else {
                    197
                };
            }
            203 => {
                values[15] = 0xcbf29ce484222325u64;
                pc = 202;
            }
            204 => {
                values[10] = memory.read_u64(values[9])?;
                pc = 203;
            }
            205 => {
                values[9] = {
                    let call_arguments = [values[3]];
                    invoke(
                        "Paint::PlatformHelpers::CodecInfo::ProductId",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 204;
            }
            206 => {
                {
                    let destination = (values[0]).wrapping_add((0x3eu64).wrapping_mul(8));
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 205;
            }
            207 => {
                values[3] = (values[0]).wrapping_add((0x14u64).wrapping_mul(8));
                pc = 206;
            }
            208 => {
                values[13] = 0x0u64;
                pc = 207;
            }
            209 => {
                {
                    let call_arguments = [relocate("DAT_1401d0e48")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 208;
            }
            210 => {
                {
                    let call_arguments = [relocate("FUN_1400df840")];
                    invoke("atexit", &call_arguments, memory)
                };
                pc = 209;
            }
            211 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0e68"),
                        0x10u64,
                        memory.read_u64(relocate("DAT_1401d0e58"))?,
                    ];
                    invoke("FUN_140044150", &call_arguments, memory)
                };
                pc = 210;
            }
            212 => {
                {
                    let destination = relocate("_DAT_1401d0e50");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("DAT_1401d0e50")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x3f800000u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 211;
            }
            213 => {
                {
                    let destination = relocate("DAT_1401d0e88");
                    let stored = 0x8u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 212;
            }
            214 => {
                {
                    let destination = relocate("DAT_1401d0e80");
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 213;
            }
            215 => {
                {
                    let destination = relocate("uRam00000001401d0e78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 214;
            }
            216 => {
                {
                    let destination = relocate("_DAT_1401d0e70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 215;
            }
            217 => {
                {
                    let destination = relocate("DAT_1401d0e68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 216;
            }
            218 => {
                {
                    let destination =
                        (memory.read_u64(relocate("DAT_1401d0e58"))?).wrapping_add(0x8u64);
                    let stored = memory.read_u64(relocate("DAT_1401d0e58"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 217;
            }
            219 => {
                {
                    let destination = memory.read_u64(relocate("DAT_1401d0e58"))?;
                    let stored = memory.read_u64(relocate("DAT_1401d0e58"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 218;
            }
            220 => {
                {
                    let destination = relocate("DAT_1401d0e58");
                    let stored = {
                        let call_arguments = [0x18u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 219;
            }
            221 => {
                {
                    let destination = relocate("DAT_1401d0e60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 220;
            }
            222 => {
                {
                    let destination = relocate("DAT_1401d0e58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 221;
            }
            223 => {
                {
                    let destination = relocate("_DAT_1401d0e50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 222;
            }
            224 => {
                {
                    let destination = relocate("DAT_1401d0e88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 223;
            }
            225 => {
                {
                    let destination = relocate("DAT_1401d0e80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 224;
            }
            226 => {
                {
                    let destination = relocate("uRam00000001401d0e78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 225;
            }
            227 => {
                {
                    let destination = relocate("_DAT_1401d0e70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 226;
            }
            228 => {
                {
                    let destination = relocate("DAT_1401d0e68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 227;
            }
            229 => {
                pc = if (u64::from(
                    ((u64::from(
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
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("DAT_1401d0e48"))?)) as u32,
                            )) as u32,
                        )),
                    )) != 0)
                        && (({
                            let _ = {
                                let call_arguments = [relocate("DAT_1401d0e48")];
                                invoke("FUN_1400b03b8", &call_arguments, memory)
                            };
                            u64::from(
                                (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("DAT_1401d0e48"))?))
                                            as u32,
                                    )) as u32,
                                )) == (u64::from(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32,
                                )),
                            )
                        }) != 0),
                )) != 0
                {
                    228
                } else {
                    208
                };
            }
            230 => {
                return Ok(None);
            }
            231 => {
                {
                    let call_arguments = [values[5]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 230;
            }
            232 => {
                values[5] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 231;
            }
            233 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    232
                };
            }
            234 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    233
                };
            }
            235 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    50
                } else {
                    234
                };
            }
            236 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    121
                } else {
                    235
                };
            }
            237 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    122
                } else {
                    236
                };
            }
            238 => {
                pc = if (u64::from(
                    (((values[17]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    229
                } else {
                    237
                };
            }
            239 => {
                values[17] = (u64::from((u64::from(memory.read_u16(values[2])?)) as u16)) as i32
                    as i64 as u64;
                pc = 238;
            }
            240 => {
                values[2] = (values[0]).wrapping_add((0x20u64).wrapping_mul(8));
                pc = 239;
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
    ("FUN_14008df70", 3, fun_14008df70),
    ("FUN_1400c4060", 1, fun_1400c4060),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_14008df70",
        &[
            ("STACK_auStack_98", -152),
            ("STACK_local_78", -120),
            ("STACK_plStack_70", -112),
            ("STACK_local_68", -104),
            ("STACK_uStack_60", -96),
            ("STACK_local_58", -88),
            ("STACK_plStack_50", -80),
            ("STACK_local_48", -72),
            ("STACK_lStack_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_1400c4060",
        &[("STACK_local_c8", -200), ("STACK_local_b0", -176)],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_14008df70",
        &[
            ("STACK_auStack_98", 32),
            ("STACK_local_78", 8),
            ("STACK_plStack_70", 8),
            ("STACK_local_68", 8),
            ("STACK_uStack_60", 8),
            ("STACK_local_58", 8),
            ("STACK_plStack_50", 8),
            ("STACK_local_48", 8),
            ("STACK_lStack_40", 8),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_1400c4060",
        &[("STACK_local_c8", 24), ("STACK_local_b0", 8)],
    ),
];
