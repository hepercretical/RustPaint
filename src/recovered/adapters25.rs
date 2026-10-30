//! Checked translations generated from original bodies by batch 025.
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
/// Translation of `FUN_1400034e0` (`mspaint.c:7436`).
pub fn fun_1400034e0(
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
    let mut pc: i32 = 61;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_1400041f0", &call_arguments, memory)
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
                    let call_arguments = [values[2]];
                    invoke("ReleaseSRWLockExclusive", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_1400041f0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[7] = values[3];
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1u64).wrapping_mul(8))];
                    invoke("ReleaseSRWLockExclusive", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                values[6] = values[7];
                pc = 8;
            }
            10 => {
                {
                    let destination = values[0];
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                pc = if (u64::from((values[0]) != (relocate("STACK_local_48")))) != 0 {
                    11
                } else {
                    8
                };
            }
            13 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                values[3] = memory.read_u64(values[0])?;
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1u64).wrapping_mul(8))];
                    invoke("AcquireSRWLockExclusive", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                pc = if (values[11]) != 0 { 15 } else { 5 };
            }
            17 => {
                values[6] = values[7];
                pc = 16;
            }
            18 => {
                values[11] = u64::from((u64::from((values[5]) == (values[1]))) != 0);
                pc = 17;
            }
            19 => {
                values[5] = {
                    let call_arguments =
                        [memory.read_u64((values[3]).wrapping_add((0x2u64).wrapping_mul(4)))?];
                    invoke("EncodePointer", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                pc = 16;
            }
            21 => {
                pc = if (u64::from((values[10]) != (memory.read_u64(relocate("STACK_local_38"))?)))
                    != 0
                {
                    37
                } else {
                    20
                };
            }
            22 => {
                values[0] = memory.read_u64(relocate("STACK_local_30"))?;
                pc = 21;
            }
            23 => {
                values[10] = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 22;
            }
            24 => {
                values[12] =
                    (((values[12]) as i32 as i64 as u64).wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 23;
            }
            25 => {
                values[9] = (values[9]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 24;
            }
            26 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    26
                } else {
                    25
                };
            }
            28 => {
                {
                    let destination = (values[9]).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                values[4] = memory.read_u64((values[10]).wrapping_add((0x0u64).wrapping_mul(1)))?;
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [values[9]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                pc = if (u64::from(
                    (memory.read_u64((values[9]).wrapping_add((0x0u64).wrapping_mul(1)))?)
                        != (0x0u64),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                pc = if (u64::from((values[9]) != (values[10]))) != 0 {
                    31
                } else {
                    25
                };
            }
            33 => {
                pc = 20;
            }
            34 => {
                pc = if (u64::from(
                    ((((values[12]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    32
                };
            }
            35 => {
                values[0] = memory.read_u64(relocate("STACK_local_30"))?;
                pc = 34;
            }
            36 => {
                values[11] = 1u64;
                pc = 23;
            }
            37 => {
                pc = if (u64::from(
                    ((values[11]) != 0)
                        || (({
                            let _ = {
                                values[5] = {
                                    let call_arguments = [memory.read_u64(
                                        (values[10]).wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?];
                                    invoke("EncodePointer", &call_arguments, memory)
                                };
                                values[5]
                            };
                            u64::from((values[1]) != (values[5]))
                        }) != 0),
                )) != 0
                {
                    35
                } else {
                    36
                };
            }
            38 => {
                pc = if (u64::from((values[10]) != (memory.read_u64(relocate("STACK_local_38"))?)))
                    != 0
                {
                    37
                } else {
                    5
                };
            }
            39 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (values[3]).wrapping_add(
                        (((u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[3]).wrapping_add((0x1u64).wrapping_mul(4)),
                                )?)) as u32,
                            )) as u32,
                        ))
                        .wrapping_add(0x1u64))
                        .wrapping_mul(0x2u64))
                        .wrapping_mul(4),
                    );
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                values[10] = (values[3]).wrapping_add((0x2u64).wrapping_mul(4));
                pc = 40;
            }
            42 => {
                values[3] = memory.read_u64(values[0])?;
                pc = 41;
            }
            43 => {
                values[9] = (values[6]).wrapping_add((0x2u64).wrapping_mul(4));
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [
                        (values[6]).wrapping_add((0x2u64).wrapping_mul(4)),
                        0x0u64,
                        (values[8]).wrapping_shl((0x3u64) as u32),
                    ];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[6]).wrapping_add((0x1u64).wrapping_mul(4));
                    let stored = u64::from((values[8]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = values[6];
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 45;
            }
            47 => {
                values[6] = {
                    let call_arguments = [((values[12]).wrapping_mul(0x8u64)).wrapping_add(0x8u64)];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                pc = if (u64::from((u64::from((values[8]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    19
                } else {
                    47
                };
            }
            49 => {
                values[11] = 0u64;
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
                values[12] = values[8];
                pc = 50;
            }
            52 => {
                values[8] = u64::from(
                    (u64::from(
                        ((u64::from(
                            (u64::from(
                                memory
                                    .read_u32((values[3]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                            )) as u32,
                        ))
                        .wrapping_sub(0x1u64)) as u32,
                    )) as u32,
                );
                pc = 51;
            }
            53 => {
                pc = 3;
            }
            54 => {
                pc = if (u64::from((values[3]) == (0x0u64))) != 0 {
                    53
                } else {
                    52
                };
            }
            55 => {
                values[3] = memory.read_u64(values[0])?;
                pc = 54;
            }
            56 => {
                {
                    let call_arguments = [values[2]];
                    invoke("AcquireSRWLockExclusive", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                values[2] = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
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
                values[7] = 0x0u64;
                pc = 60;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400045e0` (`mspaint.c:8516`).
pub fn fun_1400045e0(
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
    ];
    let mut pc: i32 = 127;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 0;
            }
            2 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                values[3] = ((values[7]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 2;
            }
            4 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 3;
            }
            5 => {
                return Ok(Some(0x0u64));
            }
            6 => {
                pc = 22;
            }
            7 => {
                return Ok(Some(0x0u64));
            }
            8 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 7;
            }
            9 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = ((values[6]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 9;
            }
            11 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 10;
            }
            12 => {
                pc = if (u64::from(((values[6]) as i64) < ((0x0u64) as i64))) != 0 {
                    11
                } else {
                    6
                };
            }
            13 => {
                values[7] = values[6];
                pc = 12;
            }
            14 => {
                pc = 5;
            }
            15 => {
                pc = if (values[10]) != 0 { 14 } else { 13 };
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                values[6] = values[7];
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                pc = if (values[10]) != 0 { 18 } else { 16 };
            }
            20 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 19;
            }
            21 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 20;
            }
            22 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i64)
                        < ((values[7]) as i64),
                )) != 0
                {
                    6
                } else {
                    4
                };
            }
            24 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 23;
            }
            25 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                return Ok(Some(0x0u64));
            }
            28 => {
                pc = 44;
            }
            29 => {
                return Ok(Some(0x0u64));
            }
            30 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 29;
            }
            31 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 30;
            }
            32 => {
                values[3] = ((values[6]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 31;
            }
            33 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 32;
            }
            34 => {
                pc = if (u64::from(((values[6]) as i64) < ((0x0u64) as i64))) != 0 {
                    33
                } else {
                    28
                };
            }
            35 => {
                values[7] = values[6];
                pc = 34;
            }
            36 => {
                pc = 27;
            }
            37 => {
                pc = if (values[10]) != 0 { 36 } else { 35 };
            }
            38 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 37;
            }
            39 => {
                values[6] = values[7];
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                pc = if (values[10]) != 0 { 40 } else { 38 };
            }
            42 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 41;
            }
            43 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 42;
            }
            44 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    (((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i64)
                        < ((values[7]) as i64),
                )) != 0
                {
                    28
                } else {
                    4
                };
            }
            46 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 45;
            }
            47 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x38u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    (((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i64)
                        < ((values[7]) as i64),
                )) != 0
                {
                    59
                } else {
                    4
                };
            }
            50 => {
                return Ok(Some(0x0u64));
            }
            51 => {
                pc = if (values[10]) != 0 { 50 } else { 49 };
            }
            52 => {
                values[7] = values[6];
                pc = 51;
            }
            53 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 52;
            }
            54 => {
                values[6] = values[7];
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                pc = if (values[10]) != 0 { 55 } else { 53 };
            }
            57 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 56;
            }
            58 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 57;
            }
            59 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 58;
            }
            60 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 49;
            }
            61 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                return Ok(Some(values[8]));
            }
            64 => {
                values[8] = {
                    let call_arguments = [values[8], values[2]];
                    invoke("FUN_140004c10", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                values[8] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(values[0])?).wrapping_add((0x3u64).wrapping_mul(8)),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                return Ok(Some(values[8]));
            }
            67 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[0])?)?,
                        values[0],
                        values[1],
                        values[2],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    67
                } else {
                    65
                };
            }
            69 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_1401054f0"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    69
                } else {
                    62
                };
            }
            71 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_1401057b0"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 70;
            }
            72 => {
                return Ok(Some(values[8]));
            }
            73 => {
                values[8] = 0x0u64;
                pc = 72;
            }
            74 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    73
                } else {
                    72
                };
            }
            75 => {
                values[8] = 0x8007000eu64;
                pc = 74;
            }
            76 => {
                {
                    let destination = values[2];
                    let stored = memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((values[9]).wrapping_mul(0x2u64)).wrapping_add(0x8u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 77;
            }
            79 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                values[3] = ((values[9]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 79;
            }
            81 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 80;
            }
            82 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            83 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x8u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            84 => {
                {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FUN_140004880", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 84;
            }
            86 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 85;
            }
            87 => {
                values[3] = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 86;
            }
            88 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 87;
            }
            89 => {
                pc = 108;
            }
            90 => {
                values[9] = values[5];
                pc = 89;
            }
            91 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 90;
            }
            92 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((values[5]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 91;
            }
            93 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 92;
            }
            94 => {
                pc = 88;
            }
            95 => {
                pc = if (u64::from(((values[5]) as i64) < ((0x0u64) as i64))) != 0 {
                    94
                } else {
                    93
                };
            }
            96 => {
                pc = 76;
            }
            97 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
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
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((((u64::from(
                        memory.read_u32(
                            (memory.read_u64(
                                (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)
                            .wrapping_add((0x3u64).wrapping_mul(8)),
                        )?,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 98;
            }
            100 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 99;
            }
            101 => {
                pc = if (values[10]) != 0 { 100 } else { 95 };
            }
            102 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 101;
            }
            103 => {
                values[5] = values[9];
                pc = 102;
            }
            104 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = ((memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_shr((0x1u64) as u32))
                        | (0x8000000000000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                pc = if (values[10]) != 0 { 104 } else { 102 };
            }
            106 => {
                values[10] = u64::from((u64::from((values[9]) == (values[5]))) != 0);
                pc = 105;
            }
            107 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 106;
            }
            108 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 107;
            }
            109 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add(0x1cu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 89;
            }
            110 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((values[9]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                {
                    let destination = memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    let stored = relocate("winrt::impl::weak_ref<1,1>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::weak_source<1,1>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 114;
            }
            116 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 115;
            }
            117 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 116;
            }
            118 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) == (0x0u64),
                )) != 0
                {
                    82
                } else {
                    117
                };
            }
            119 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = {
                        let call_arguments = [0x20u64, relocate("PTR_1400e4d48")];
                        invoke("FUN_1400b0234", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                pc = if (u64::from(((values[9]) as i64) < ((0x0u64) as i64))) != 0 {
                    81
                } else {
                    119
                };
            }
            121 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 120;
            }
            122 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    121
                } else {
                    71
                };
            }
            123 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105490"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 122;
            }
            124 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    123
                };
            }
            125 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105420"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 124;
            }
            126 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    125
                };
            }
            127 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105248"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 126;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14000a6c0` (`mspaint.c:13976`).
pub fn fun_14000a6c0(
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
    let mut pc: i32 = 56;
    loop {
        match pc {
            0 => {
                return Ok(Some(
                    (((((((values[14]).wrapping_shr((0x8u64) as u32)) << 8) as i64 >> 8) as u64)
                        & (0xffffffffffffffu64))
                        .wrapping_shl((0x8u64) as u32))
                        | ((0x1u64) & (0xffu64)),
                ));
            }
            1 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_14000a550", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a550"));
            }
            2 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x22d2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 3;
            }
            5 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    0
                };
            }
            6 => {
                values[14] = (((values[10]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[7]) & (0xffffffffu64));
                pc = 5;
            }
            7 => {
                values[10] = u64::from(
                    (u64::from(((values[4]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 6;
            }
            8 => {
                values[7] = ((values[4]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 7;
            }
            9 => {
                values[4] = {
                    let call_arguments = [relocate("FUN_14000a520"), values[1], 0x0u64];
                    invoke("TrySubmitThreadpoolCallback", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_14000a550", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14000a550"));
            }
            11 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x22d2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    0
                };
            }
            15 => {
                values[14] = (((values[12]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[7]) & (0xffffffffu64));
                pc = 14;
            }
            16 => {
                values[12] = u64::from(
                    (u64::from(((values[3]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 15;
            }
            17 => {
                values[7] = ((values[3]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 16;
            }
            18 => {
                values[3] = {
                    let call_arguments = [relocate("FUN_14000a5c0"), values[9], 0x0u64];
                    invoke("TrySubmitThreadpoolCallback", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                values[9] = 0x0u64;
                pc = 19;
            }
            21 => {
                {
                    let destination = (values[9]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            22 => {
                {
                    let destination = (values[9]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    24
                } else {
                    22
                };
            }
            26 => {
                {
                    let destination = values[9];
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                values[5] = memory.read_u64(values[0])?;
                pc = 26;
            }
            28 => {
                pc = if (u64::from((values[9]) == (0x0u64))) != 0 {
                    20
                } else {
                    27
                };
            }
            29 => {
                values[9] = {
                    let call_arguments = [0x18u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                pc = 33;
            }
            31 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                (((memory.read_u64(relocate("STACK_local_30"))?) as i32 as i64
                                    as u64)
                                    .wrapping_sub(0x3u64)) as u32,
                            )) & (0xfffffffdu64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    30
                } else {
                    29
                };
            }
            32 => {
                return Ok(Some(values[11]));
            }
            33 => {
                values[11] = {
                    let call_arguments = [values[0], values[1], values[2]];
                    invoke("FUN_14000a630", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    33
                } else {
                    29
                };
            }
            35 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    34
                };
            }
            36 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    29
                };
            }
            37 => {
                pc = 33;
            }
            38 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    37
                } else {
                    36
                };
            }
            39 => {
                values[8] = ({
                    let call_arguments = [relocate("STACK_local_28"), relocate("STACK_local_30")];
                    invoke("CoGetApartmentType", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    39
                };
            }
            41 => {
                return Ok(Some((values[9]) & (0xffffffffffffff00u64)));
            }
            42 => {
                pc = if (values[6]) != 0 { 41 } else { 40 };
            }
            43 => {
                values[9] = {
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                pc = if (u64::from(
                    ((values[13]) != 0)
                        && ((u64::from(
                            (memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    43
                } else {
                    42
                };
            }
            45 => {
                values[9] = memory.read_u64(relocate("STACK_local_30"))?;
                pc = 44;
            }
            46 => {
                values[6] = 1u64;
                pc = 45;
            }
            47 => {
                values[6] = 0u64;
                pc = 45;
            }
            48 => {
                pc = 46;
            }
            49 => {
                pc = if (u64::from(
                    (memory.read_u64(values[0])?)
                        == (memory.read_u64(relocate("STACK_local_30"))?),
                )) != 0
                {
                    48
                } else {
                    47
                };
            }
            50 => {
                values[13] = 1u64;
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [relocate("DAT_140105fb0"), relocate("STACK_local_30")];
                    invoke("CoGetObjectContext", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                pc = if (u64::from((memory.read_u64(values[0])?) == (0x0u64))) != 0 {
                    46
                } else {
                    52
                };
            }
            54 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_30"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                values[13] = 0u64;
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400136f0` (`mspaint.c:21621`).
pub fn fun_1400136f0(
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
    let mut pc: i32 = 53;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[1]));
            }
            1 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_130"))?];
                    invoke("CoTaskMemFree", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_130"))?) != (0x0u64)))
                    != 0
                {
                    1
                } else {
                    0
                };
            }
            3 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[1]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[1]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
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
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("std::basic_ios<>::~basic_ios<>", &call_arguments, memory)
                };
                pc = 2;
            }
            9 => {
                {
                    let call_arguments = [relocate("STACK_local_108")];
                    invoke(
                        "std::basic_iostream<>::~basic_iostream<>",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [relocate("STACK_local_110")];
                    invoke("FUN_140014640", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let destination = ((relocate("STACK_local_128")).wrapping_add(
                        (u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    (relocate("STACK_local_128"))
                                        .wrapping_add((0x0u64).wrapping_mul(8)),
                                )?)
                                .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64,
                    ))
                    .wrapping_add((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64);
                    let stored = ((((u64::from(
                        memory.read_u32(
                            (memory.read_u64(
                                (relocate("STACK_local_128"))
                                    .wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)
                            .wrapping_add((0x4u64).wrapping_mul(1)),
                        )?,
                    )) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x98u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (relocate("STACK_local_128")).wrapping_add(
                        (u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    (relocate("STACK_local_128"))
                                        .wrapping_add((0x0u64).wrapping_mul(8)),
                                )?)
                                .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64,
                    );
                    let stored = relocate("std::basic_stringstream<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let call_arguments = [values[1], values[5], values[7]];
                    invoke("FUN_140014910", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                pc = if (u64::from(((values[5]) as i64) != ((0x0u64) as i64))) != 0 {
                    13
                } else {
                    12
                };
            }
            15 => {
                values[7] = (((((u64::from(
                    memory.read_u32(memory.read_u64(relocate("STACK_local_c0"))?)?,
                )) as i32 as i64 as u64)
                    .wrapping_mul(0x2u64))
                .wrapping_sub(values[5]))
                .wrapping_add(memory.read_u64(memory.read_u64(relocate("STACK_local_d8"))?)?)
                    as i64)
                    .wrapping_shr((0x1u64) as u32) as u64;
                pc = 14;
            }
            16 => {
                values[5] = memory.read_u64(memory.read_u64(relocate("STACK_local_f8"))?)?;
                pc = 15;
            }
            17 => {
                values[5] = 0x0u64;
                pc = 14;
            }
            18 => {
                values[7] = 0x0u64;
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    (u64::from(memory.read_u32(relocate("STACK_local_a0"))?))
                                        as u32,
                                )) & (0x4u64)) as u32,
                            )) as u32,
                        )) == (u64::from((0x0u64) as u32)),
                    )) != 0)
                        && ((u64::from(
                            ((memory.read_u64(memory.read_u64(relocate("STACK_local_d8"))?)?)
                                as i64)
                                != ((0x0u64) as i64),
                        )) != 0),
                )) != 0
                {
                    16
                } else {
                    18
                };
            }
            20 => {
                values[7] = ((values[6]).wrapping_sub(values[5]) as i64)
                    .wrapping_shr((0x1u64) as u32) as u64;
                pc = 14;
            }
            21 => {
                values[6] = memory.read_u64(relocate("STACK_local_a8"))?;
                pc = 20;
            }
            22 => {
                pc = if (u64::from((values[6]) < (memory.read_u64(relocate("STACK_local_a8"))?)))
                    != 0
                {
                    21
                } else {
                    20
                };
            }
            23 => {
                values[5] = memory.read_u64(memory.read_u64(relocate("STACK_local_f0"))?)?;
                pc = 22;
            }
            24 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((((u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_a0"))?)) as u32,
                            )) as u8,
                        )) & (0x22u64)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            == (((0x2u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[6] = memory
                                    .read_u64(memory.read_u64(relocate("STACK_local_d0"))?)?;
                                values[6]
                            };
                            u64::from((values[6]) == (0x0u64))
                        }) != 0),
                )) != 0
                {
                    19
                } else {
                    23
                };
            }
            25 => {
                {
                    let destination = relocate("STACK_uStack_148");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = u64::from((0xau64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = (values[1]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[1]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let call_arguments = [
                        values[4],
                        u64::from(
                            (u64::from(
                                memory.read_u32(
                                    ((values[0]).wrapping_add((0x1u64).wrapping_mul(16)))
                                        .wrapping_add((0x0u64).wrapping_mul(1)),
                                )?,
                            )) as u32,
                        ),
                    ];
                    invoke("std::basic_ostream<>::operator<<", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                values[4] = {
                    let call_arguments = [values[3], relocate("DAT_140107f08"), 0x1u64];
                    invoke("FUN_140014ae0", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                values[3] = {
                    let call_arguments = [
                        (relocate("STACK_local_128")).wrapping_add((0x2u64).wrapping_mul(8)),
                        memory.read_u64(relocate("STACK_local_130"))?,
                    ];
                    invoke("FUN_140013be0", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                {
                    let destination = relocate("STACK_local_a0");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = relocate("STACK_local_110");
                    let stored = relocate("std::basic_stringbuf<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [relocate("STACK_local_110")];
                    invoke(
                        "std::basic_streambuf<>::basic_streambuf<>",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 38;
            }
            40 => {
                {
                    let destination = ((relocate("STACK_local_128")).wrapping_add(
                        (u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    (relocate("STACK_local_128"))
                                        .wrapping_add((0x0u64).wrapping_mul(8)),
                                )?)
                                .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64,
                    ))
                    .wrapping_add((0u64.wrapping_sub(0x4u64)) as i32 as i64 as u64);
                    let stored = ((((u64::from(
                        memory.read_u32(
                            (memory.read_u64(
                                (relocate("STACK_local_128"))
                                    .wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)
                            .wrapping_add((0x4u64).wrapping_mul(1)),
                        )?,
                    )) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x98u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let destination = (relocate("STACK_local_128")).wrapping_add(
                        (u64::from(
                            memory.read_u32(
                                (memory.read_u64(
                                    (relocate("STACK_local_128"))
                                        .wrapping_add((0x0u64).wrapping_mul(8)),
                                )?)
                                .wrapping_add((0x4u64).wrapping_mul(1)),
                            )?,
                        )) as i32 as i64 as u64,
                    );
                    let stored = relocate("std::basic_stringstream<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [relocate("STACK_local_128"), relocate("STACK_local_110")];
                    invoke(
                        "std::basic_iostream<>::basic_iostream<>",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = u64::from((0x2u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("std::basic_ios<>::basic_ios<>", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                {
                    let destination =
                        (relocate("STACK_local_128")).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = relocate("DAT_140107fa0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination =
                        (relocate("STACK_local_128")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = relocate("DAT_140107fa8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let call_arguments = [relocate("STACK_local_128"), 0x0u64, 0xf8u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                pc = if (u64::from(
                    (((values[2]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    7
                } else {
                    47
                };
            }
            49 => {
                values[2] = ({
                    let call_arguments = [values[0], relocate("STACK_local_130")];
                    invoke("StringFromCLSID", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_178"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140022030` (`mspaint.c:33675`).
pub fn fun_140022030(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
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
        arguments[2],
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
                pc = 82;
            }
            1 => {
                values[3] = ((u64::from(
                    memory.read_u8(
                        (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x18u64),
                    )?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 0;
            }
            2 => {
                values[9] = memory.read_u64(
                    (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        .wrapping_add(0x8u64),
                )?;
                pc = 1;
            }
            3 => {
                {
                    let destination = (memory.read_u64(
                        (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x8u64),
                    )?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[5]).wrapping_add(0x18u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[8]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            7 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let destination = (memory.read_u64(values[0])?).wrapping_add(0x8u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            10 => {
                {
                    let destination = (values[6]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            12 => {
                {
                    let destination = values[6];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    (values[8])
                        == (memory.read_u64((values[6]).wrapping_add((0x2u64).wrapping_mul(8)))?),
                )) != 0
                {
                    10
                } else {
                    12
                };
            }
            14 => {
                values[6] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 13;
            }
            15 => {
                pc = if (u64::from(
                    (values[8])
                        == (memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x8u64))?),
                )) != 0
                {
                    8
                } else {
                    14
                };
            }
            16 => {
                {
                    let destination = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (memory
                        .read_u64((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                    .wrapping_add(0x8u64);
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8(
                            (memory
                                .read_u64((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                            .wrapping_add(0x19u64),
                        )?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    17
                } else {
                    16
                };
            }
            19 => {
                {
                    let destination = values[8];
                    let stored =
                        memory.read_u64((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                values[10] = memory.read_u64(values[8])?;
                pc = 19;
            }
            21 => {
                values[8] = memory.read_u64(
                    (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        .wrapping_add(0x8u64),
                )?;
                pc = 20;
            }
            22 => {
                {
                    let destination = (memory.read_u64(
                        (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x8u64),
                    )?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = (memory
                        .read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 22;
            }
            24 => {
                values[9] = values[8];
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = values[10];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = (memory.read_u64(values[0])?).wrapping_add(0x8u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = values[9];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            29 => {
                {
                    let destination = (values[9]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            30 => {
                pc = if (u64::from((values[8]) == (memory.read_u64(values[9])?))) != 0 {
                    28
                } else {
                    29
                };
            }
            31 => {
                values[9] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (values[8])
                        == (memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x8u64))?),
                )) != 0
                {
                    27
                } else {
                    31
                };
            }
            33 => {
                {
                    let destination = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = (memory.read_u64(values[10])?).wrapping_add(0x8u64);
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8((memory.read_u64(values[10])?).wrapping_add(0x19u64))?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    34
                } else {
                    33
                };
            }
            36 => {
                {
                    let destination = (values[8]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[10])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                pc = if (u64::from((values[9]) == (values[10]))) != 0 {
                    36
                } else {
                    23
                };
            }
            38 => {
                values[10] = memory.read_u64((values[8]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[5]).wrapping_add(0x18u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    38
                };
            }
            40 => {
                values[5] = memory.read_u64(
                    (memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        .wrapping_add((0x2u64).wrapping_mul(8)),
                )?;
                pc = 39;
            }
            41 => {
                values[9] = memory.read_u64(
                    (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        .wrapping_add(0x8u64),
                )?;
                pc = 1;
            }
            42 => {
                {
                    let destination = (memory.read_u64(
                        (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x8u64),
                    )?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = (values[5]).wrapping_add(0x18u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (values[8]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 43;
            }
            45 => {
                pc = 6;
            }
            46 => {
                {
                    let destination = values[10];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let destination = (memory.read_u64(values[0])?).wrapping_add(0x8u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = values[7];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            49 => {
                {
                    let destination = (values[7]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            50 => {
                pc = if (u64::from((values[8]) == (memory.read_u64(values[7])?))) != 0 {
                    48
                } else {
                    49
                };
            }
            51 => {
                values[7] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 50;
            }
            52 => {
                pc = if (u64::from(
                    (values[8])
                        == (memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x8u64))?),
                )) != 0
                {
                    47
                } else {
                    51
                };
            }
            53 => {
                {
                    let destination = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (memory.read_u64(values[10])?).wrapping_add(0x8u64);
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8((memory.read_u64(values[10])?).wrapping_add(0x19u64))?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    54
                } else {
                    53
                };
            }
            56 => {
                {
                    let destination = (values[8]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[10])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
                values[10] = memory.read_u64((values[8]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 56;
            }
            58 => {
                values[8] = memory.read_u64(
                    (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        .wrapping_add(0x8u64),
                )?;
                pc = 57;
            }
            59 => {
                {
                    let destination = (memory.read_u64(
                        (memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x8u64),
                    )?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = (memory
                        .read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 59;
            }
            61 => {
                values[9] = values[8];
                pc = 60;
            }
            62 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = (memory.read_u64(values[0])?).wrapping_add(0x8u64);
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = (values[7]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            66 => {
                {
                    let destination = values[7];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            67 => {
                pc = if (u64::from(
                    (values[8])
                        == (memory.read_u64((values[7]).wrapping_add((0x2u64).wrapping_mul(8)))?),
                )) != 0
                {
                    65
                } else {
                    66
                };
            }
            68 => {
                values[7] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    (values[8])
                        == (memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x8u64))?),
                )) != 0
                {
                    64
                } else {
                    68
                };
            }
            70 => {
                {
                    let destination = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = (memory
                        .read_u64((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                    .wrapping_add(0x8u64);
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8(
                            (memory
                                .read_u64((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                            .wrapping_add(0x19u64),
                        )?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    71
                } else {
                    70
                };
            }
            73 => {
                {
                    let destination = values[8];
                    let stored =
                        memory.read_u64((values[10]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                pc = if (u64::from((values[9]) == (values[10]))) != 0 {
                    73
                } else {
                    60
                };
            }
            75 => {
                values[10] = memory.read_u64(values[8])?;
                pc = 74;
            }
            76 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[5]).wrapping_add(0x18u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    75
                } else {
                    44
                };
            }
            77 => {
                pc = if (u64::from((values[8]) == (values[5]))) != 0 {
                    40
                } else {
                    76
                };
            }
            78 => {
                values[5] = memory.read_u64(
                    memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                )?;
                pc = 77;
            }
            79 => {
                values[8] = memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 78;
            }
            80 => {
                return Ok(Some(values[2]));
            }
            81 => {
                {
                    let destination = (memory
                        .read_u64((values[4]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                    .wrapping_add(0x18u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 80;
            }
            82 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    81
                } else {
                    79
                };
            }
            83 => {
                values[9] = values[2];
                pc = 82;
            }
            84 => {
                values[3] = ((u64::from(
                    memory.read_u8(
                        (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                            .wrapping_add(0x18u64),
                    )?,
                )) as i8 as i64 as u64) as i8 as i64 as u64;
                pc = 83;
            }
            85 => {
                {
                    let destination = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                pc = if (u64::from(
                    (values[9])
                        == (memory.read_u64((values[4]).wrapping_add((0x2u64).wrapping_mul(8)))?),
                )) != 0
                {
                    85
                } else {
                    84
                };
            }
            87 => {
                {
                    let destination = (values[9]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = values[4];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            89 => {
                pc = if (u64::from((values[9]) == (memory.read_u64(values[4])?))) != 0 {
                    88
                } else {
                    84
                };
            }
            90 => {
                {
                    let destination = values[9];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    87
                } else {
                    90
                };
            }
            92 => {
                return Ok(Some(values[2]));
            }
            93 => {
                {
                    let destination = (values[2]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 92;
            }
            94 => {
                {
                    let destination = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = (values[4]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = values[4];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                pc = if (u64::from((values[9]) == (values[4]))) != 0 {
                    96
                } else {
                    91
                };
            }
            98 => {
                {
                    let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                values[9] = memory.read_u64(values[1])?;
                pc = 98;
            }
            100 => {
                values[4] = memory.read_u64(values[0])?;
                pc = 99;
            }
            101 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (memory
                        .read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                    .wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140025500` (`mspaint.c:36613`).
pub fn fun_140025500(
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
    ];
    let mut pc: i32 = 102;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 0;
            }
            2 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                values[3] = ((values[7]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 2;
            }
            4 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 3;
            }
            5 => {
                pc = 21;
            }
            6 => {
                return Ok(Some(0x0u64));
            }
            7 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 6;
            }
            8 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 7;
            }
            9 => {
                values[3] = ((values[6]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 8;
            }
            10 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 9;
            }
            11 => {
                pc = if (u64::from(((values[6]) as i64) < ((0x0u64) as i64))) != 0 {
                    10
                } else {
                    5
                };
            }
            12 => {
                values[7] = values[6];
                pc = 11;
            }
            13 => {
                pc = 0;
            }
            14 => {
                pc = if (values[10]) != 0 { 13 } else { 12 };
            }
            15 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 14;
            }
            16 => {
                values[6] = values[7];
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                pc = if (values[10]) != 0 { 17 } else { 15 };
            }
            19 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 18;
            }
            20 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 19;
            }
            21 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            22 => {
                pc = if (u64::from(((values[7]) as i64) < ((0x0u64) as i64))) != 0 {
                    4
                } else {
                    5
                };
            }
            23 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 22;
            }
            24 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                pc = if (u64::from(!((values[10]) != 0))) != 0 {
                    36
                } else {
                    0
                };
            }
            27 => {
                values[7] = values[6];
                pc = 26;
            }
            28 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 27;
            }
            29 => {
                values[6] = values[7];
                pc = 28;
            }
            30 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                pc = if (values[10]) != 0 { 30 } else { 28 };
            }
            32 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 31;
            }
            33 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 32;
            }
            34 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 33;
            }
            35 => {
                pc = 4;
            }
            36 => {
                pc = if (u64::from(((values[7]) as i64) < ((0x0u64) as i64))) != 0 {
                    35
                } else {
                    34
                };
            }
            37 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 36;
            }
            38 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                return Ok(Some(values[8]));
            }
            41 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[0])?)?,
                        values[0],
                        values[1],
                        values[2],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                return Ok(Some(values[8]));
            }
            43 => {
                values[8] = {
                    let call_arguments = [values[8], values[2]];
                    invoke("FUN_140004c10", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                values[8] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(values[0])?).wrapping_add((0x3u64).wrapping_mul(8)),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    41
                };
            }
            46 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_1401054f0"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 45;
            }
            47 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    39
                };
            }
            48 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_1401057b0"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 47;
            }
            49 => {
                return Ok(Some(values[8]));
            }
            50 => {
                values[8] = 0x0u64;
                pc = 49;
            }
            51 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    50
                } else {
                    49
                };
            }
            52 => {
                values[8] = 0x8007000eu64;
                pc = 51;
            }
            53 => {
                {
                    let destination = values[2];
                    let stored = memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((values[9]).wrapping_mul(0x2u64)).wrapping_add(0x8u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 54;
            }
            56 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 55;
            }
            57 => {
                values[3] = ((values[9]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 56;
            }
            58 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 57;
            }
            59 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            60 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x8u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            61 => {
                {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FUN_140004880", &call_arguments, memory)
                };
                pc = 60;
            }
            62 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 61;
            }
            63 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 62;
            }
            64 => {
                values[3] = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 63;
            }
            65 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 64;
            }
            66 => {
                pc = 85;
            }
            67 => {
                values[9] = values[5];
                pc = 66;
            }
            68 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 67;
            }
            69 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((values[5]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 68;
            }
            70 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 69;
            }
            71 => {
                pc = 65;
            }
            72 => {
                pc = if (u64::from(((values[5]) as i64) < ((0x0u64) as i64))) != 0 {
                    71
                } else {
                    70
                };
            }
            73 => {
                pc = 53;
            }
            74 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 74;
            }
            76 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((((u64::from(
                        memory.read_u32(
                            (memory.read_u64(
                                (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)
                            .wrapping_add((0x3u64).wrapping_mul(8)),
                        )?,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 75;
            }
            77 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 76;
            }
            78 => {
                pc = if (values[10]) != 0 { 77 } else { 72 };
            }
            79 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 78;
            }
            80 => {
                values[5] = values[9];
                pc = 79;
            }
            81 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = ((memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_shr((0x1u64) as u32))
                        | (0x8000000000000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                pc = if (values[10]) != 0 { 81 } else { 79 };
            }
            83 => {
                values[10] = u64::from((u64::from((values[9]) == (values[5]))) != 0);
                pc = 82;
            }
            84 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 83;
            }
            85 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 84;
            }
            86 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add(0x1cu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 66;
            }
            87 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((values[9]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    let stored = relocate("winrt::impl::weak_ref<1,1>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::weak_source<1,1>::vftable");
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
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 91;
            }
            93 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 92;
            }
            94 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 93;
            }
            95 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) == (0x0u64),
                )) != 0
                {
                    59
                } else {
                    94
                };
            }
            96 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = {
                        let call_arguments = [0x20u64, relocate("PTR_1400e4d48")];
                        invoke("FUN_1400b0234", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                pc = if (u64::from(((values[9]) as i64) < ((0x0u64) as i64))) != 0 {
                    58
                } else {
                    96
                };
            }
            98 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 97;
            }
            99 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    98
                } else {
                    48
                };
            }
            100 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105490"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 99;
            }
            101 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    100
                };
            }
            102 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105248"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 101;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140027140` (`mspaint.c:37913`).
pub fn fun_140027140(
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
    ];
    let mut pc: i32 = 48;
    loop {
        match pc {
            0 => {
                pc = 27;
            }
            1 => {
                return Ok(Some(values[0]));
            }
            2 => {
                pc = if (values[12]) != 0 { 1 } else { 0 };
            }
            3 => {
                values[7] = values[0];
                pc = 2;
            }
            4 => {
                values[12] = u64::from((u64::from((values[7]) == (values[0]))) != 0);
                pc = 3;
            }
            5 => {
                pc = 24;
            }
            6 => {
                values[8] = u64::from((0x3u64) as u8);
                pc = 5;
            }
            7 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x5eu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    6
                } else {
                    4
                };
            }
            8 => {
                pc = 24;
            }
            9 => {
                values[8] = u64::from((0x2u64) as u8);
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x3eu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    9
                } else {
                    7
                };
            }
            11 => {
                return Ok(Some(values[10]));
            }
            12 => {
                {
                    let destination = (memory.read_u64(values[2])?).wrapping_add(0x9u64);
                    let stored = u64::from((values[8]) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (memory.read_u64(values[2])?).wrapping_add(0xeu64);
                    let stored = ((values[11]) as i8 as i64 as u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 12;
            }
            14 => {
                values[10] = (values[7]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [
                        values[9],
                        0x0u64,
                        ((((memory.read_u64(values[2])?).wrapping_add(0x14u64))
                            .wrapping_sub(values[9]) as i64)
                            .wrapping_shr((0x1u64) as u32) as u64)
                            .wrapping_mul(0x2u64),
                    ];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                values[9] = (((values[11]).wrapping_mul(0x2u64)).wrapping_add(values[5]))
                    .wrapping_add(0x10u64);
                pc = 15;
            }
            17 => {
                {
                    let call_arguments = [
                        (values[5]).wrapping_add(0x10u64),
                        values[0],
                        (values[11]).wrapping_mul(0x2u64),
                    ];
                    invoke("memmove", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                values[5] = memory.read_u64(values[2])?;
                pc = 17;
            }
            19 => {
                {
                    let call_arguments = [relocate("\"Invalid fill (too long).\"")];
                    invoke("FUN_140032060", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140032060"));
            }
            20 => {
                pc = if (u64::from((0x2u64) < (values[11]))) != 0 {
                    19
                } else {
                    18
                };
            }
            21 => {
                values[11] = ((values[7]).wrapping_sub(values[0]) as i64)
                    .wrapping_shr((0x1u64) as u32) as u64;
                pc = 20;
            }
            22 => {
                {
                    let call_arguments = [relocate("\"invalid fill character \\'{\\'\"")];
                    invoke("FUN_140032060", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140032060"));
            }
            23 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        == (((0x7bu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                pc = if (u64::from((values[7]) != (values[0]))) != 0 {
                    23
                } else {
                    12
                };
            }
            25 => {
                values[8] = u64::from((0x1u64) as u8);
                pc = 24;
            }
            26 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x3cu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    25
                } else {
                    10
                };
            }
            27 => {
                values[4] =
                    u64::from((u64::from((u64::from(memory.read_u16(values[7])?)) as u16)) as u16);
                pc = 26;
            }
            28 => {
                values[7] = (values[0]).wrapping_add((values[6]).wrapping_mul(2));
                pc = 27;
            }
            29 => {
                pc = if (u64::from(
                    ((values[0]).wrapping_add((values[6]).wrapping_mul(2))) != (values[1]),
                )) != 0
                {
                    28
                } else {
                    27
                };
            }
            30 => {
                values[7] = values[0];
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [relocate("\"Invalid format string.\"")];
                    invoke("FUN_140032060", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140032060"));
            }
            32 => {
                pc = if (u64::from(
                    ((u64::from(!((values[12]) != 0))) != 0)
                        || (({
                            let _ = {
                                values[6] = ((((values[7]).wrapping_sub(values[0]) as i64)
                                    .wrapping_shr((0x1u64) as u32)
                                    as u64) as i32
                                    as i64 as u64)
                                    as i32 as i64
                                    as u64;
                                values[6]
                            };
                            u64::from(
                                (((values[6]) as i32 as i64 as u64) as i64)
                                    < (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    31
                } else {
                    30
                };
            }
            33 => {
                values[12] = 1u64;
                pc = 32;
            }
            34 => {
                values[7] = (values[0]).wrapping_add((0x2u64).wrapping_mul(2));
                pc = 33;
            }
            35 => {
                pc = 42;
            }
            36 => {
                pc = if (u64::from(
                    (((0x3ffu64) as i32 as i64 as u64) as i64)
                        < (((u64::from(
                            (((u64::from((u64::from(memory.read_u16(values[10])?)) as u16))
                                .wrapping_add(0x2400u64)) as i32 as i64
                                as u64) as u16,
                        )) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                pc = 32;
            }
            38 => {
                values[7] = values[1];
                pc = 37;
            }
            39 => {
                values[12] = 0u64;
                pc = 38;
            }
            40 => {
                pc = if (u64::from((values[10]) == (values[1]))) != 0 {
                    39
                } else {
                    36
                };
            }
            41 => {
                pc = 32;
            }
            42 => {
                values[12] = 0u64;
                pc = 41;
            }
            43 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        < (((0xe000u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    42
                } else {
                    33
                };
            }
            44 => {
                pc = if (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        < (((0xdc00u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    43
                };
            }
            45 => {
                pc = if (u64::from(
                    (((0xd7ffu64) as i32 as i64 as u64) as i64)
                        < (((values[3]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    33
                };
            }
            46 => {
                values[7] = values[10];
                pc = 45;
            }
            47 => {
                values[10] = (values[0]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 46;
            }
            48 => {
                values[3] =
                    u64::from((u64::from((u64::from(memory.read_u16(values[0])?)) as u16)) as u16);
                pc = 47;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14002fba0` (`mspaint.c:44769`).
pub fn fun_14002fba0(
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
        0u64,
        0u64,
        0u64,
    ];
    let mut pc: i32 = 85;
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
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 0;
            }
            4 => {
                {
                    let destination = values[0];
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                values[1] = (values[1]).wrapping_add((values[13]).wrapping_mul(1));
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [values[1], values[14], values[13]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                pc = 2;
            }
            8 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[13]) as i64),
                )) != 0
                {
                    7
                } else {
                    6
                };
            }
            9 => {
                values[13] = (values[15]).wrapping_sub(values[14]);
                pc = 8;
            }
            10 => {
                pc = if (u64::from(!((values[6]) != 0))) != 0 {
                    9
                } else {
                    4
                };
            }
            11 => {
                values[1] = (values[1]).wrapping_add((values[13]).wrapping_mul(1));
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [values[1], relocate("STACK_local_d8"), values[13]];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    (((values[2]).wrapping_sub(values[1])) as i64) < ((values[13]) as i64),
                )) != 0
                {
                    2
                } else {
                    12
                };
            }
            14 => {
                values[13] = (values[11]).wrapping_sub(relocate("STACK_local_d8"));
                pc = 13;
            }
            15 => {
                values[11] = (values[11]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                );
                pc = 14;
            }
            16 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(memory.read_u8((values[11]).wrapping_add(
                            ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                        ))?)) as u8,
                    )) as i32 as i64 as u64) as i64)
                        == (((0x2eu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    15
                } else {
                    14
                };
            }
            17 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x30u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    19
                } else {
                    16
                };
            }
            18 => {
                values[11] = (values[11]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                );
                pc = 17;
            }
            19 => {
                values[5] = (u64::from(
                    (u64::from(memory.read_u8((values[11]).wrapping_add(
                        ((0u64.wrapping_sub(0x2u64)) as i32 as i64 as u64).wrapping_mul(1),
                    ))?)) as u8,
                )) as i8 as i64 as u64;
                pc = 18;
            }
            20 => {
                values[5] = (u64::from(
                    (u64::from(memory.read_u8((values[11]).wrapping_add(
                        ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(1),
                    ))?)) as u8,
                )) as i8 as i64 as u64;
                pc = 17;
            }
            21 => {
                pc = if (u64::from(
                    (((0x0u64) as i32 as i64 as u64) as i64)
                        < (((values[8]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    14
                };
            }
            22 => {
                values[14] = 0x0u64;
                pc = 21;
            }
            23 => {
                values[11] = memory.read_u64(values[10])?;
                pc = 22;
            }
            24 => {
                values[15] = 0x0u64;
                pc = 23;
            }
            25 => {
                values[10] = {
                    let call_arguments = [
                        relocate("STACK_local_e8"),
                        relocate("STACK_local_d8"),
                        relocate("STACK_local_63"),
                        u64::from(
                            ((float_to_integer(
                                f64::from_bits(
                                    ((f32::from_bits((values[3]) as u32)) as f64).to_bits(),
                                ),
                                64,
                                false,
                            )?)
                            .wrapping_shr(
                                (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                            )) as u32,
                        ),
                    ];
                    invoke("FUN_14002ec80", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 25;
            }
            27 => {
                values[7] = (0x6u64) as i32 as i64 as u64;
                pc = 26;
            }
            28 => {
                pc = 21;
            }
            29 => {
                values[14] = 0x0u64;
                pc = 28;
            }
            30 => {
                values[15] = 0x0u64;
                pc = 29;
            }
            31 => {
                values[11] = relocate("STACK_local_63");
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (((0x3b9ac9ffu64) as i32 as i64 as u64) as i64)
                        < (((values[8]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    26
                };
            }
            33 => {
                values[7] = (values[8]) as i32 as i64 as u64;
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    33
                };
            }
            35 => {
                values[8] = (0x25u64) as i32 as i64 as u64;
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    (((0x25u64) as i32 as i64 as u64) as i64)
                        < (((values[8]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                values[8] = (((((values[7]).wrapping_sub(values[8])) as i32 as i64 as u64)
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 36;
            }
            38 => {
                values[6] = 1u64;
                pc = 37;
            }
            39 => {
                values[14] = values[11];
                pc = 21;
            }
            40 => {
                values[11] = {
                    let call_arguments = [
                        relocate("STACK_local_d8"),
                        memory.read_u64(relocate("STACK_local_e8"))?,
                        0x65u64,
                    ];
                    invoke("thunk_FUN_1400b1150", &call_arguments, memory)
                };
                pc = 39;
            }
            41 => {
                values[15] = memory.read_u64(relocate("STACK_local_e8"))?;
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_e8"),
                        relocate("STACK_local_d8"),
                        relocate("STACK_local_63"),
                        u64::from(
                            ((float_to_integer(
                                f64::from_bits(
                                    ((f32::from_bits((values[3]) as u32)) as f64).to_bits(),
                                ),
                                64,
                                false,
                            )?)
                            .wrapping_shr(
                                (((0x0u64).wrapping_mul(0x8u64)) as i32 as i64 as u64) as u32,
                            )) as u32,
                        ),
                    ];
                    invoke("FUN_14002f400", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = (values[7]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                values[7] = (0x6u64) as i32 as i64 as u64;
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_uStack_e0");
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 41;
            }
            46 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = relocate("STACK_local_63");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                pc = 43;
            }
            48 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x3b9aca00u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                values[7] = (values[8]) as i32 as i64 as u64;
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    44
                } else {
                    49
                };
            }
            51 => {
                values[8] = (0x6fu64) as i32 as i64 as u64;
                pc = 50;
            }
            52 => {
                pc = if (u64::from(
                    (((0x6fu64) as i32 as i64 as u64) as i64)
                        < (((((values[7])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    51
                } else {
                    50
                };
            }
            53 => {
                values[8] = (((values[7])
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 52;
            }
            54 => {
                values[6] = 0u64;
                pc = 53;
            }
            55 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[8]) as i32 as i64 as u64) as i64)
                            < (((values[7]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((0u64.wrapping_sub(0x5u64)) as i32 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                < (((values[8]) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    38
                } else {
                    54
                };
            }
            56 => {
                values[8] = ((((((values[9]).wrapping_sub(values[12]) as i64)
                    .wrapping_shr((0x2u64) as u32) as u64) as i32
                    as i64 as u64)
                    .wrapping_add((0u64.wrapping_sub(0x5u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 55;
            }
            57 => {
                pc = if (u64::from(
                    ((u64::from(
                        (values[9])
                            != (((values[12]).wrapping_add((values[4]).wrapping_mul(4)))
                                .wrapping_add((0x5u64).wrapping_mul(4))),
                    )) != 0)
                        && ((u64::from(
                            (u64::from(
                                (float_to_integer(
                                    (f32::from_bits(
                                        (u64::from((u64::from(memory.read_u32(values[9])?)) as u32))
                                            as u32,
                                    )) as f64,
                                    32,
                                    false,
                                )?) as u32,
                            )) < (u64::from(
                                (float_to_integer(
                                    (f32::from_bits((values[3]) as u32)) as f64,
                                    32,
                                    false,
                                )?) as u32,
                            )),
                        )) != 0),
                )) != 0
                {
                    58
                } else {
                    56
                };
            }
            58 => {
                let _ = {
                    values[9] = (values[9]).wrapping_add((0x1u64).wrapping_mul(4));
                    values[9]
                };
                pc = 57;
            }
            59 => {
                let _ = {
                    values[9] = values[12];
                    values[9]
                };
                pc = 57;
            }
            60 => {
                values[7] = (values[4]) as i32 as i64 as u64;
                pc = 59;
            }
            61 => {
                values[12] = (relocate("DAT_140103f40")).wrapping_add(
                    ((({
                        let numerator = (((((values[4]).wrapping_add(0xau64)) as i32 as i64 as u64)
                            .wrapping_mul(
                                ((values[4])
                                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                                    as i32 as i64 as u64,
                            )) as i32 as i64 as u64) as i32;
                        let denominator = (0x2u64) as i32;
                        if denominator == 0 {
                            return Err(AdapterError::Arithmetic("division by zero"));
                        }
                        numerator
                            .checked_div(denominator)
                            .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                            as u64
                    }) as i32 as i64 as u64)
                        .wrapping_mul(0x4u64))
                    .wrapping_mul(1),
                );
                pc = 60;
            }
            62 => {
                values[4] = (0x6u64) as i32 as i64 as u64;
                pc = 61;
            }
            63 => {
                values[4] = (0x27u64) as i32 as i64 as u64;
                pc = 59;
            }
            64 => {
                pc = if (u64::from(
                    (((0x27u64) as i32 as i64 as u64) as i64)
                        < (((values[7]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    59
                };
            }
            65 => {
                values[4] = (values[7]) as i32 as i64 as u64;
                pc = 64;
            }
            66 => {
                values[12] = relocate("DAT_140104040");
                pc = 65;
            }
            67 => {
                values[7] = (values[4]) as i32 as i64 as u64;
                pc = 66;
            }
            68 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0xf4240u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    67
                } else {
                    66
                };
            }
            69 => {
                values[7] = (0xf4240u64) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                pc = 61;
            }
            71 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[4]) as i32 as i64 as u64) as i64)
                            < (((0xf4240u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (((values[4]) as i32 as i64 as u64) as i64)
                                < (((0x8u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    70
                } else {
                    69
                };
            }
            72 => {
                pc = 61;
            }
            73 => {
                values[4] = (0x1u64) as i32 as i64 as u64;
                pc = 72;
            }
            74 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    73
                } else {
                    71
                };
            }
            75 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    62
                } else {
                    74
                };
            }
            76 => {
                return Ok(Some(values[0]));
            }
            77 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 76;
            }
            78 => {
                {
                    let destination = values[0];
                    let stored = (values[1]).wrapping_add((0x1u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 77;
            }
            79 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x30u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 78;
            }
            80 => {
                return Ok(Some(values[0]));
            }
            81 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((0x84u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = values[0];
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                pc = if (u64::from((values[1]) == (values[2]))) != 0 {
                    82
                } else {
                    79
                };
            }
            84 => {
                pc = if (u64::from(
                    ((f32::from_bits((values[3]) as u32)) as f64) == (f64::from_bits(0x0u64)),
                )) != 0
                {
                    83
                } else {
                    75
                };
            }
            85 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_118"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140042b30` (`mspaint.c:62021`).
pub fn fun_140042b30(
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
    ];
    let mut pc: i32 = 50;
    loop {
        match pc {
            0 => {
                pc = 28;
            }
            1 => {
                pc = if (u64::from((values[10]) != (values[2]))) != 0 {
                    19
                } else {
                    0
                };
            }
            2 => {
                {
                    let destination = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            3 => {
                {
                    let destination = values[12];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                pc = 15;
            }
            5 => {
                pc = 29;
            }
            6 => {
                pc = if (u64::from((values[10]) == (values[2]))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                values[11] = values[10];
                pc = 6;
            }
            8 => {
                pc = 3;
            }
            9 => {
                pc = if (u64::from((values[11]) == (values[6]))) != 0 {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let destination = (values[0]).wrapping_add(0x10u64);
                    let stored = (memory.read_u64((values[0]).wrapping_add(0x10u64))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [values[11], 0x28u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [(values[11]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[11]).wrapping_add((0x2u64).wrapping_mul(8)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                {
                    let call_arguments = [(values[11]).wrapping_add((0x3u64).wrapping_mul(8))];
                    invoke("FUN_140018220", &call_arguments, memory)
                };
                pc = 13;
            }
            15 => {
                values[10] = memory.read_u64(values[11])?;
                pc = 14;
            }
            16 => {
                values[11] = values[10];
                pc = 4;
            }
            17 => {
                values[6] = memory.read_u64((values[12]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 16;
            }
            18 => {
                values[12] = (values[3]).wrapping_add(
                    ((values[7]) & (memory.read_u64((values[0]).wrapping_add(0x30u64))?))
                        .wrapping_mul(0x10u64),
                );
                pc = 17;
            }
            19 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("FUN_1400445c0", &call_arguments, memory)
                };
                pc = 18;
            }
            20 => {
                {
                    let destination = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            21 => {
                values[8] = values[4];
                pc = 20;
            }
            22 => {
                {
                    let destination = values[12];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                pc = if (u64::from((values[6]) == (values[1]))) != 0 {
                    22
                } else {
                    20
                };
            }
            24 => {
                values[8] = values[5];
                pc = 23;
            }
            25 => {
                pc = 40;
            }
            26 => {
                return Ok(Some(values[2]));
            }
            27 => {
                {
                    let destination = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = values[5];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = values[12];
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                pc = if (u64::from((values[6]) == (values[1]))) != 0 {
                    29
                } else {
                    28
                };
            }
            31 => {
                pc = if (u64::from((values[10]) == (values[2]))) != 0 {
                    30
                } else {
                    25
                };
            }
            32 => {
                values[9] = values[10];
                pc = 31;
            }
            33 => {
                pc = 24;
            }
            34 => {
                pc = if (u64::from((values[9]) == (values[11]))) != 0 {
                    33
                } else {
                    32
                };
            }
            35 => {
                {
                    let destination = (values[0]).wrapping_add(0x10u64);
                    let stored = (memory.read_u64((values[0]).wrapping_add(0x10u64))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [values[9], 0x28u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                {
                    let call_arguments = [(values[9]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[9]).wrapping_add((0x2u64).wrapping_mul(8)))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    37
                } else {
                    36
                };
            }
            39 => {
                {
                    let call_arguments = [(values[9]).wrapping_add((0x3u64).wrapping_mul(8))];
                    invoke("FUN_140018220", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                values[10] = memory.read_u64(values[9])?;
                pc = 39;
            }
            41 => {
                values[9] = values[1];
                pc = 25;
            }
            42 => {
                values[11] =
                    memory.read_u64((values[12]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 41;
            }
            43 => {
                values[6] = memory.read_u64(values[12])?;
                pc = 42;
            }
            44 => {
                values[12] = (((memory.read_u64((values[0]).wrapping_add(0x30u64))?)
                    & (values[7]))
                    .wrapping_mul(0x10u64))
                .wrapping_add(values[3]);
                pc = 43;
            }
            45 => {
                values[7] = {
                    let call_arguments = [
                        values[0],
                        (values[1]).wrapping_add((0x2u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_1400445c0", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                values[5] = memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 45;
            }
            47 => {
                values[4] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
                pc = 46;
            }
            48 => {
                values[3] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
                pc = 47;
            }
            49 => {
                return Ok(Some(values[2]));
            }
            50 => {
                pc = if (u64::from((values[1]) == (values[2]))) != 0 {
                    49
                } else {
                    48
                };
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140048ab0` (`mspaint.c:67108`).
pub fn fun_140048ab0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [
        0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64,
    ];
    let mut pc: i32 = 114;
    loop {
        match pc {
            0 => {
                pc = 50;
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
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = 45;
            }
            4 => {
                {
                    let call_arguments = [relocate("STACK_local_60")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                {
                    let call_arguments = [relocate("STACK_local_40")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 5;
            }
            7 => {
                {
                    let call_arguments = [relocate("DAT_1401d0d00"), relocate("STACK_local_98")];
                    invoke("FUN_1400199f0", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_98"),
                        values[10],
                        ((values[1]).wrapping_sub(values[10]) as i64).wrapping_shr((0x1u64) as u32)
                            as u64,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
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
                    let destination = relocate("STACK_uStack_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                pc = 19;
            }
            14 => {
                pc = 12;
            }
            15 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[2]) as i32 as i64 as u64) as i64)
                            == (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[3] = (values[10]).wrapping_add(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                        .wrapping_mul(2),
                                );
                                values[3]
                            };
                            u64::from(
                                (((values[2]) as i32 as i64 as u64) as i64)
                                    == (((0x2fu64) as i32 as i64 as u64) as i64),
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
                values[2] = ((u64::from(memory.read_u16((values[10]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                ))?)) as i16 as i64 as u64) as i16 as i64 as u64;
                pc = 15;
            }
            17 => {
                pc = 12;
            }
            18 => {
                pc = if (u64::from((values[7]) == (values[10]))) != 0 {
                    17
                } else {
                    16
                };
            }
            19 => {
                values[10] = values[3];
                pc = 18;
            }
            20 => {
                pc = if (u64::from((values[7]) != (values[1]))) != 0 {
                    24
                } else {
                    19
                };
            }
            21 => {
                values[7] = (values[7]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 20;
            }
            22 => {
                pc = 19;
            }
            23 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[7])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u16(values[7])?)) as i16 as i64 as u64) as i32
                                as i64 as u64) as i64)
                                != (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    22
                } else {
                    21
                };
            }
            24 => {
                values[3] = values[1];
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[7]) != (values[1]))) != 0 {
                    24
                } else {
                    12
                };
            }
            26 => {
                values[10] = values[1];
                pc = 25;
            }
            27 => {
                values[7] = {
                    let call_arguments = [values[6], values[1]];
                    invoke("FUN_140019de0", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                values[1] = (values[6]).wrapping_add(
                    (memory.read_u64(relocate("STACK_local_30"))?).wrapping_mul(0x2u64),
                );
                pc = 27;
            }
            29 => {
                values[6] = memory.read_u64(relocate("STACK_local_40"))?;
                pc = 28;
            }
            30 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_28"))?))) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                values[6] = relocate("STACK_local_40");
                pc = 30;
            }
            32 => {
                pc = if (u64::from((0x1u64) < ((values[1]).wrapping_sub(values[10])))) != 0 {
                    31
                } else {
                    2
                };
            }
            33 => {
                pc = 39;
            }
            34 => {
                pc = 32;
            }
            35 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[2]) as i32 as i64 as u64) as i64)
                            == (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[3] = (values[10]).wrapping_add(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                        .wrapping_mul(2),
                                );
                                values[3]
                            };
                            u64::from(
                                (((values[2]) as i32 as i64 as u64) as i64)
                                    == (((0x2fu64) as i32 as i64 as u64) as i64),
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
                values[2] = ((u64::from(memory.read_u16((values[10]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                ))?)) as i16 as i64 as u64) as i16 as i64 as u64;
                pc = 35;
            }
            37 => {
                pc = 32;
            }
            38 => {
                pc = if (u64::from((values[7]) == (values[10]))) != 0 {
                    37
                } else {
                    36
                };
            }
            39 => {
                values[10] = values[3];
                pc = 38;
            }
            40 => {
                pc = if ({
                    let _ = {
                        values[7] = (values[7]).wrapping_add((0x1u64).wrapping_mul(2));
                        values[7]
                    };
                    u64::from((values[7]) != (values[1]))
                }) != 0
                {
                    43
                } else {
                    39
                };
            }
            41 => {
                pc = 39;
            }
            42 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[7])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u16(values[7])?)) as i16 as i64 as u64) as i32
                                as i64 as u64) as i64)
                                != (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    41
                } else {
                    40
                };
            }
            43 => {
                values[3] = values[1];
                pc = 42;
            }
            44 => {
                return Ok(Some(relocate("DAT_1401d0d00")));
            }
            45 => {
                {
                    let call_arguments = [relocate("STACK_local_78")];
                    invoke("FUN_14004b0f0", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0d00"),
                        relocate("L\"mspaint.exe\""),
                        0xbu64,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                {
                    let destination = relocate("DAT_1401d0d18");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("_DAT_1401d0d10");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("uRam00000001401d0d08");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("DAT_1401d0d00");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                pc = if (u64::from(
                    (((memory.read_u64(relocate("STACK_pWStack_70"))?)
                        .wrapping_sub(memory.read_u64(relocate("STACK_local_78"))?)
                        as i64)
                        .wrapping_shr((0x1u64) as u32) as u64)
                        < (0x2800u64),
                )) != 0
                {
                    105
                } else {
                    50
                };
            }
            52 => {
                {
                    let call_arguments =
                        [relocate("STACK_local_78"), (values[9]).wrapping_mul(0x2u64)];
                    invoke("FUN_14004af60", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                pc = 32;
            }
            54 => {
                pc = 43;
            }
            55 => {
                pc = if (u64::from((values[7]) != (values[1]))) != 0 {
                    54
                } else {
                    53
                };
            }
            56 => {
                values[10] = values[1];
                pc = 55;
            }
            57 => {
                values[7] = {
                    let call_arguments = [values[6], values[1]];
                    invoke("FUN_140019de0", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                values[1] = (values[6]).wrapping_add(
                    (memory.read_u64(relocate("STACK_local_30"))?).wrapping_mul(0x2u64),
                );
                pc = 57;
            }
            59 => {
                values[6] = memory.read_u64(relocate("STACK_local_40"))?;
                pc = 58;
            }
            60 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_28"))?))) != 0
                {
                    59
                } else {
                    58
                };
            }
            61 => {
                values[6] = relocate("STACK_local_40");
                pc = 60;
            }
            62 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_40"),
                        values[6],
                        memory.read_u64(relocate("STACK_local_50"))?,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = relocate("STACK_uStack_38");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination = relocate("STACK_local_40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                values[6] = memory.read_u64(relocate("STACK_local_60"))?;
                pc = 66;
            }
            68 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_uStack_48"))?))) != 0
                {
                    67
                } else {
                    66
                };
            }
            69 => {
                values[6] = relocate("STACK_local_60");
                pc = 68;
            }
            70 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination =
                        (relocate("STACK_local_60")).wrapping_add((values[9]).wrapping_mul(0x2u64));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 69;
            }
            74 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_60"),
                        memory.read_u64(relocate("STACK_local_78"))?,
                        (values[9]).wrapping_mul(0x2u64),
                    ];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            76 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination = (values[6]).wrapping_add((values[9]).wrapping_mul(0x2u64));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 69;
            }
            78 => {
                {
                    let call_arguments = [values[6], values[0], (values[9]).wrapping_mul(0x2u64)];
                    invoke("memcpy", &call_arguments, memory)
                };
                pc = 77;
            }
            79 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                values[6] = {
                    let call_arguments = [(values[8]).wrapping_mul(0x2u64)];
                    invoke("FUN_140005710", &call_arguments, memory)
                };
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140005800", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140005800"));
            }
            84 => {
                pc = if (u64::from((0x7fffffffffffffffu64) < (values[8]))) != 0 {
                    83
                } else {
                    82
                };
            }
            85 => {
                values[8] = (values[5]).wrapping_add(0x1u64);
                pc = 84;
            }
            86 => {
                values[5] = 0xau64;
                pc = 85;
            }
            87 => {
                pc = if (u64::from((values[5]) < (0xau64))) != 0 {
                    86
                } else {
                    85
                };
            }
            88 => {
                values[5] = 0x7ffffffffffffffeu64;
                pc = 82;
            }
            89 => {
                values[8] = 0x7fffffffffffffffu64;
                pc = 88;
            }
            90 => {
                pc = if (u64::from((values[5]) < (0x7fffffffffffffffu64))) != 0 {
                    87
                } else {
                    89
                };
            }
            91 => {
                values[5] = (values[9]) | (0x7u64);
                pc = 90;
            }
            92 => {
                pc = if (u64::from((values[9]) < (0x8u64))) != 0 {
                    76
                } else {
                    91
                };
            }
            93 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400056f0", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_1400056f0"));
            }
            94 => {
                pc = if (u64::from((0x7ffffffffffffffeu64) < (values[9]))) != 0 {
                    93
                } else {
                    92
                };
            }
            95 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_local_78"))?)
                        == (memory.read_u64(relocate("STACK_pWStack_70"))?),
                )) != 0
                {
                    72
                } else {
                    94
                };
            }
            96 => {
                {
                    let destination = relocate("STACK_uStack_48");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_50");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = relocate("STACK_uStack_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                pc = if (u64::from((values[4]) < (values[9]))) != 0 {
                    99
                } else {
                    52
                };
            }
            101 => {
                values[9] = ((memory.read_u64(relocate("STACK_pWStack_70"))?)
                    .wrapping_sub(memory.read_u64(relocate("STACK_local_78"))?)
                    as i64)
                    .wrapping_shr((0x1u64) as u32) as u64;
                pc = 100;
            }
            102 => {
                pc = 50;
            }
            103 => {
                pc = if (u64::from((u64::from((values[4]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    102
                } else {
                    101
                };
            }
            104 => {
                values[0] = memory.read_u64(relocate("STACK_local_78"))?;
                pc = 103;
            }
            105 => {
                values[4] = u64::from(
                    ({
                        let call_arguments = [
                            0x0u64,
                            memory.read_u64(relocate("STACK_local_78"))?,
                            u64::from(
                                (((memory.read_u64(relocate("STACK_pWStack_70"))?)
                                    .wrapping_sub(memory.read_u64(relocate("STACK_local_78"))?)
                                    as i64)
                                    .wrapping_shr((0x1u64) as u32)
                                    as u64) as u32,
                            ),
                        ];
                        invoke("GetModuleFileNameW", &call_arguments, memory)
                    }) as u32,
                );
                pc = 104;
            }
            106 => {
                {
                    let destination = relocate("STACK_pWStack_70");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 105;
            }
            107 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_78"))?,
                        0x0u64,
                        0x208u64,
                    ];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 106;
            }
            108 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                values[0] = (memory.read_u64(relocate("STACK_local_78"))?)
                    .wrapping_add((0x104u64).wrapping_mul(2));
                pc = 108;
            }
            110 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = {
                        let call_arguments = [0x208u64];
                        invoke("FUN_140005710", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                {
                    let destination = relocate("STACK_pWStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
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
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_c8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140058810` (`mspaint.c:80752`).
pub fn fun_140058810(
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
    let mut pc: i32 = 75;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[2]];
                    invoke("ReleaseSRWLockExclusive", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [relocate("STACK_local_98")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    2
                } else {
                    1
                };
            }
            4 => {
                {
                    let destination = values[3];
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 1;
            }
            5 => {
                {
                    let call_arguments = [values[3]];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 4;
            }
            6 => {
                pc = if (u64::from((memory.read_u64(values[3])?) != (0x0u64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                pc = if (u64::from((values[3]) == (relocate("STACK_local_98")))) != 0 {
                    3
                } else {
                    6
                };
            }
            8 => {
                values[3] = (values[0]).wrapping_add(0x50u64);
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [relocate("STACK_local_50")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) != (0x0u64),
                )) != 0
                {
                    9
                } else {
                    8
                };
            }
            11 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64((values[6]).wrapping_add((0x0u64).wrapping_mul(1)))?)
                            .wrapping_add(0x8u64),
                        )?,
                        values[6],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 10;
            }
            12 => {
                pc = 8;
            }
            13 => {
                pc = if (u64::from((values[6]) == (0x0u64))) != 0 {
                    12
                } else {
                    11
                };
            }
            14 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 13;
            }
            15 => {
                values[6] = memory.read_u64(values[1])?;
                pc = 14;
            }
            16 => {
                values[6] = 0x0u64;
                pc = 10;
            }
            17 => {
                {
                    let call_arguments = [relocate("STACK_local_70")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 16;
            }
            18 => {
                pc = if (u64::from(((values[5]).wrapping_shr(0)) != (0x0u64))) != 0 {
                    17
                } else {
                    10
                };
            }
            19 => {
                values[6] = 0x0u64;
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            22 => {
                values[6] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 21;
            }
            23 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_90"))?)
                        .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = relocate("winrt::impl::delegate<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 25;
            }
            27 => {
                {
                    let destination = ((memory.read_u64(relocate("STACK_local_90"))?)
                        .wrapping_add((0x4u64).wrapping_mul(8)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = memory.read_u64(relocate("STACK_pcStack_80"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = ((memory.read_u64(relocate("STACK_local_90"))?)
                        .wrapping_add((0x3u64).wrapping_mul(8)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = memory.read_u64(
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = ((memory.read_u64(relocate("STACK_local_90"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (values[5]).wrapping_shr(0);
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let destination = ((memory.read_u64(relocate("STACK_local_90"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 29;
            }
            31 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) == (0x0u64))) != 0
                {
                    20
                } else {
                    30
                };
            }
            32 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = {
                        let call_arguments = [0x28u64];
                        invoke("operator_new", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_pvStack_60");
                    let stored = memory.read_u64(relocate("STACK_pcStack_80"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = memory.read_u64(
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_pcStack_80");
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_3c"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_uStack_40"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_uStack_44"))?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(memory.read_u32(relocate("STACK_local_48"))?)) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            37 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored = (values[5]).wrapping_shr(0);
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination =
                        (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?) == (0x0u64),
                )) != 0
                {
                    15
                } else {
                    38
                };
            }
            40 => {
                {
                    let stored = memory.read_u64(
                        (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?;
                    values[5] = stored;
                }
                pc = 39;
            }
            41 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_48"),
                        memory.read_u64(values[1])?,
                        relocate("STACK_local_50"),
                    ];
                    invoke("RoGetAgileReference", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                {
                    let destination =
                        (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                {
                    let destination = relocate("STACK_uStack_3c");
                    let stored = u64::from((0xd04b3895u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_uStack_40");
                    let stored = u64::from((0x91746593u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_uStack_44");
                    let stored = u64::from((0x54fa6248u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = u64::from((0xf800670fu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 45;
            }
            47 => {
                pc = 8;
            }
            48 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64((values[6]).wrapping_add((0x0u64).wrapping_mul(1)))?)
                            .wrapping_add(0x8u64),
                        )?,
                        values[6],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    48
                } else {
                    47
                };
            }
            50 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                values[6] = memory.read_u64(values[1])?;
                pc = 50;
            }
            52 => {
                {
                    let call_arguments = [relocate("STACK_local_58")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 51;
            }
            53 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_58"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    52
                } else {
                    46
                };
            }
            54 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[4])?)?,
                        values[4],
                        relocate("DAT_1401057b0"),
                        relocate("STACK_local_58"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
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
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    55
                } else {
                    46
                };
            }
            57 => {
                values[4] = memory.read_u64(values[1])?;
                pc = 56;
            }
            58 => {
                return Ok(None);
            }
            59 => {
                {
                    let call_arguments = [values[1], values[0], relocate("STACK_local_50")];
                    invoke("FUN_140009eb0", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                return Ok(None);
            }
            61 => {
                pc = if (u64::from(((memory.read_u64(values[1])?) as i64) == ((0x0u64) as i64)))
                    != 0
                {
                    60
                } else {
                    59
                };
            }
            62 => {
                {
                    let call_arguments = [values[2]];
                    invoke("ReleaseSRWLockExclusive", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x60u64))?)) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    62
                } else {
                    57
                };
            }
            64 => {
                {
                    let destination = ((relocate("STACK_local_50"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        ((u64::from(memory.read_u32((values[0]).wrapping_add(0x60u64))?)) as i32
                            as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination = (values[0]).wrapping_add(0x64u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let call_arguments = [relocate("STACK_local_70"), relocate("DAT_14016a900")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            67 => {
                {
                    let call_arguments = [relocate("STACK_local_70"), relocate("STACK_local_88")];
                    invoke("FUN_14000a900", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            69 => {
                {
                    let destination = relocate("STACK_pcStack_80");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 68;
            }
            70 => {
                {
                    let destination = ((relocate("STACK_local_88"))
                        .wrapping_add((0x0u64).wrapping_mul(1)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x155au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 69;
            }
            71 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8((values[0]).wrapping_add(0x64u64))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    70
                } else {
                    65
                };
            }
            72 => {
                {
                    let call_arguments = [values[2]];
                    invoke("AcquireSRWLockExclusive", &call_arguments, memory)
                };
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                values[2] = (values[0]).wrapping_add(0x48u64);
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_b8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 74;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14005a920` (`mspaint.c:82640`).
pub fn fun_14005a920(
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
    ];
    let mut pc: i32 = 127;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x0u64));
            }
            1 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 0;
            }
            2 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 1;
            }
            3 => {
                values[3] = ((values[7]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 2;
            }
            4 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 3;
            }
            5 => {
                return Ok(Some(0x0u64));
            }
            6 => {
                pc = 22;
            }
            7 => {
                return Ok(Some(0x0u64));
            }
            8 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 7;
            }
            9 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 8;
            }
            10 => {
                values[3] = ((values[6]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 9;
            }
            11 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 10;
            }
            12 => {
                pc = if (u64::from(((values[6]) as i64) < ((0x0u64) as i64))) != 0 {
                    11
                } else {
                    6
                };
            }
            13 => {
                values[7] = values[6];
                pc = 12;
            }
            14 => {
                pc = 5;
            }
            15 => {
                pc = if (values[10]) != 0 { 14 } else { 13 };
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                values[6] = values[7];
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                pc = if (values[10]) != 0 { 18 } else { 16 };
            }
            20 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 19;
            }
            21 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 20;
            }
            22 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 21;
            }
            23 => {
                pc = if (u64::from(
                    (((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i64)
                        < ((values[7]) as i64),
                )) != 0
                {
                    6
                } else {
                    4
                };
            }
            24 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 23;
            }
            25 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                return Ok(Some(0x0u64));
            }
            28 => {
                pc = 44;
            }
            29 => {
                return Ok(Some(0x0u64));
            }
            30 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 29;
            }
            31 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 30;
            }
            32 => {
                values[3] = ((values[6]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 31;
            }
            33 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 32;
            }
            34 => {
                pc = if (u64::from(((values[6]) as i64) < ((0x0u64) as i64))) != 0 {
                    33
                } else {
                    28
                };
            }
            35 => {
                values[7] = values[6];
                pc = 34;
            }
            36 => {
                pc = 27;
            }
            37 => {
                pc = if (values[10]) != 0 { 36 } else { 35 };
            }
            38 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 37;
            }
            39 => {
                values[6] = values[7];
                pc = 38;
            }
            40 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                pc = if (values[10]) != 0 { 40 } else { 38 };
            }
            42 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 41;
            }
            43 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 42;
            }
            44 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 43;
            }
            45 => {
                pc = if (u64::from(
                    (((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i64)
                        < ((values[7]) as i64),
                )) != 0
                {
                    28
                } else {
                    4
                };
            }
            46 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 45;
            }
            47 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 46;
            }
            48 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x38u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    (((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i64)
                        < ((values[7]) as i64),
                )) != 0
                {
                    59
                } else {
                    4
                };
            }
            50 => {
                return Ok(Some(0x0u64));
            }
            51 => {
                pc = if (values[10]) != 0 { 50 } else { 49 };
            }
            52 => {
                values[7] = values[6];
                pc = 51;
            }
            53 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 52;
            }
            54 => {
                values[6] = values[7];
                pc = 53;
            }
            55 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = (values[7]).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                pc = if (values[10]) != 0 { 55 } else { 53 };
            }
            57 => {
                values[10] =
                    u64::from((u64::from(((values[7]) as i64) == ((values[6]) as i64))) != 0);
                pc = 56;
            }
            58 => {
                values[6] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 57;
            }
            59 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 58;
            }
            60 => {
                values[7] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 49;
            }
            61 => {
                {
                    let destination = values[2];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                return Ok(Some(values[8]));
            }
            64 => {
                values[8] = {
                    let call_arguments = [values[8], values[2]];
                    invoke("FUN_140004c10", &call_arguments, memory)
                };
                pc = 63;
            }
            65 => {
                values[8] = {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(values[0])?).wrapping_add((0x3u64).wrapping_mul(8)),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 64;
            }
            66 => {
                return Ok(Some(values[8]));
            }
            67 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[0])?)?,
                        values[0],
                        values[1],
                        values[2],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    67
                } else {
                    65
                };
            }
            69 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_1401054f0"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 68;
            }
            70 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    69
                } else {
                    62
                };
            }
            71 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_1401057b0"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 70;
            }
            72 => {
                return Ok(Some(values[8]));
            }
            73 => {
                values[8] = 0x0u64;
                pc = 72;
            }
            74 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    73
                } else {
                    72
                };
            }
            75 => {
                values[8] = 0x8007000eu64;
                pc = 74;
            }
            76 => {
                {
                    let destination = values[2];
                    let stored = memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 75;
            }
            77 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((values[9]).wrapping_mul(0x2u64)).wrapping_add(0x8u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 77;
            }
            79 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                values[3] = ((values[9]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 79;
            }
            81 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 80;
            }
            82 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            83 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x8u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            84 => {
                {
                    let call_arguments = [relocate("STACK_local_38")];
                    invoke("FUN_140004880", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 84;
            }
            86 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 85;
            }
            87 => {
                values[3] = ((values[5]).wrapping_mul(0x2u64)).wrapping_add(0x18u64);
                pc = 86;
            }
            88 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 87;
            }
            89 => {
                pc = 108;
            }
            90 => {
                values[9] = values[5];
                pc = 89;
            }
            91 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 90;
            }
            92 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((values[5]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 91;
            }
            93 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 92;
            }
            94 => {
                pc = 88;
            }
            95 => {
                pc = if (u64::from(((values[5]) as i64) < ((0x0u64) as i64))) != 0 {
                    94
                } else {
                    93
                };
            }
            96 => {
                pc = 76;
            }
            97 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
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
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((((u64::from(
                        memory.read_u32(
                            (memory.read_u64(
                                (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)
                            .wrapping_add((0x3u64).wrapping_mul(8)),
                        )?,
                    )) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 98;
            }
            100 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 99;
            }
            101 => {
                pc = if (values[10]) != 0 { 100 } else { 95 };
            }
            102 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 101;
            }
            103 => {
                values[5] = values[9];
                pc = 102;
            }
            104 => {
                {
                    let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = ((memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_shr((0x1u64) as u32))
                        | (0x8000000000000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                pc = if (values[10]) != 0 { 104 } else { 102 };
            }
            106 => {
                values[10] = u64::from((u64::from((values[9]) == (values[5]))) != 0);
                pc = 105;
            }
            107 => {
                values[5] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 106;
            }
            108 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 107;
            }
            109 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add(0x1cu64);
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 89;
            }
            110 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = ((values[9]) as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 109;
            }
            111 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                {
                    let destination = memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?;
                    let stored = relocate("winrt::impl::weak_ref<1,1>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::weak_source<1,1>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 112;
            }
            114 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 113;
            }
            115 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 114;
            }
            116 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 115;
            }
            117 => {
                values[8] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 116;
            }
            118 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) == (0x0u64),
                )) != 0
                {
                    82
                } else {
                    117
                };
            }
            119 => {
                {
                    let destination =
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = {
                        let call_arguments = [0x20u64, relocate("PTR_1400e4d48")];
                        invoke("FUN_1400b0234", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                pc = if (u64::from(((values[9]) as i64) < ((0x0u64) as i64))) != 0 {
                    81
                } else {
                    119
                };
            }
            121 => {
                values[9] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 120;
            }
            122 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    121
                } else {
                    71
                };
            }
            123 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105490"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 122;
            }
            124 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    48
                } else {
                    123
                };
            }
            125 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105420"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 124;
            }
            126 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    26
                } else {
                    125
                };
            }
            127 => {
                values[4] = ({
                    let call_arguments = [values[1], relocate("DAT_140105248"), 0x10u64];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 126;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14005c890` (`mspaint.c:84317`).
pub fn fun_14005c890(
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
    let mut pc: i32 = 295;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[17]));
            }
            1 => {
                values[17] &= 0xffffffffffffff00u64;
                pc = 0;
            }
            2 => {
                {
                    let call_arguments = [values[9], 0x58u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            3 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_60"))?];
                    invoke("BasePaint::Image::~Image", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_60"))?) != (0x0u64))) != 0
                {
                    3
                } else {
                    0
                };
            }
            5 => {
                values[9] = memory.read_u64(relocate("STACK_local_60"))?;
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
                    ((memory.read_u64(relocate("STACK_local_50"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                values[17] = 0x0u64;
                pc = 7;
            }
            9 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_58"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_58"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 7;
            }
            10 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) != (0x0u64))) != 0
                {
                    9
                } else {
                    7
                };
            }
            11 => {
                values[17] = 0x0u64;
                pc = 10;
            }
            12 => {
                {
                    let call_arguments = [values[18], 0x0u64, values[8]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                values[18] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            15 => {
                pc = if (u64::from(
                    (((((values[14]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    14
                } else {
                    11
                };
            }
            16 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
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
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                values[14] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 18;
            }
            20 => {
                values[5] = (memory.read_u64(relocate("STACK_local_80"))?).wrapping_add(0x18u64);
                pc = 19;
            }
            21 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            22 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_80"))?) != (0x0u64))) != 0
                {
                    21
                } else {
                    11
                };
            }
            23 => {
                values[8] = memory.read_u64(relocate("STACK_local_80"))?;
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [
                        values[23],
                        relocate("STACK_local_80"),
                        relocate("STACK_local_130"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendErrorEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [relocate("STACK_local_80"), relocate("STACK_local_158")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored = 0x1cu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = relocate("L\"PerceptibleWatermarkedBitmap\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = relocate("STACK_local_130");
                    memory.write_u64(destination, stored)?;
                }
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[27])?)?,
                        values[27],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 10;
            }
            31 => {
                pc = if (u64::from((values[27]) != (0x0u64))) != 0 {
                    30
                } else {
                    10
                };
            }
            32 => {
                {
                    let call_arguments = [values[28]];
                    invoke("GdipDeleteGraphics", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                values[17] = 0x1u64;
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [
                        values[23],
                        0x21u64,
                        relocate("STACK_local_48"),
                        relocate("STACK_local_130"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendPerformanceCompleteEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[24])?)?,
                        values[24],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 35;
            }
            37 => {
                pc = if (u64::from((values[24]) != (0x0u64))) != 0 {
                    36
                } else {
                    35
                };
            }
            38 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_158"))?;
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                values[24] = memory.read_u64(memory.read_u64(relocate("STACK_local_158"))?)?;
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                values[27] = values[25];
                pc = 32;
            }
            42 => {
                values[17] = 0x0u64;
                pc = 41;
            }
            43 => {
                values[28] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 42;
            }
            44 => {
                {
                    let call_arguments = [values[18], 0x0u64, values[8]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                values[18] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            47 => {
                pc = if (u64::from(
                    (((((values[14]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    42
                };
            }
            48 => {
                values[28] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 47;
            }
            49 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    45
                } else {
                    48
                };
            }
            50 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 49;
            }
            51 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 50;
            }
            52 => {
                values[14] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 51;
            }
            53 => {
                values[5] = (memory.read_u64(relocate("STACK_local_80"))?).wrapping_add(0x18u64);
                pc = 52;
            }
            54 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 53;
            }
            55 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_80"))?) != (0x0u64))) != 0
                {
                    54
                } else {
                    42
                };
            }
            56 => {
                values[8] = memory.read_u64(relocate("STACK_local_80"))?;
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [
                        values[23],
                        relocate("STACK_local_80"),
                        relocate("STACK_local_130"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendErrorEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 56;
            }
            58 => {
                {
                    let call_arguments = [relocate("STACK_local_80"), relocate("STACK_local_158")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 57;
            }
            59 => {
                {
                    let destination = relocate("STACK_local_150");
                    let stored = 0x1fu64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = relocate("L\"PerceptibleWatermarkOwnedBitmap\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = relocate("STACK_local_130");
                    memory.write_u64(destination, stored)?;
                }
                pc = 61;
            }
            63 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    62
                };
            }
            64 => {
                {
                    let destination = (values[25]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 63;
            }
            65 => {
                values[27] = 0x0u64;
                pc = 64;
            }
            66 => {
                values[14] = ((u64::from(
                    memory.read_u32((values[25]).wrapping_add((0x2u64).wrapping_mul(8)))?,
                )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 65;
            }
            67 => {
                {
                    let destination = relocate("STACK_pcStack_70");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_pcStack_70")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[14]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                values[14] = (values[15]) as i32 as i64 as u64;
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    68
                } else {
                    67
                };
            }
            70 => {
                values[15] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_90"))?,
                        values[26],
                        0x0u64,
                        0x0u64,
                    ];
                    invoke("GdipDrawImageRectI", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 69;
            }
            71 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = memory.read_u64(relocate("STACK_local_90"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                {
                    let destination = relocate("STACK_local_178");
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                memory
                                    .read_u32((relocate("STACK_local_88")).wrapping_add(0x0u64))?,
                            )) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 71;
            }
            73 => {
                {
                    let destination = relocate("STACK_local_170");
                    let stored = u64::from((values[7]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                values[26] = 0x0u64;
                pc = 73;
            }
            75 => {
                values[26] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_58"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8)),
                )?;
                pc = 73;
            }
            76 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_58"))?) == (0x0u64))) != 0
                {
                    74
                } else {
                    75
                };
            }
            77 => {
                values[28] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 76;
            }
            78 => {
                values[14] = ((memory.read_u64(relocate("STACK_pcStack_70"))?) as i32 as i64 as u64)
                    as i32 as i64 as u64;
                pc = 77;
            }
            79 => {
                {
                    let destination = (relocate("STACK_pcStack_70")).wrapping_add(0x0u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 78;
            }
            80 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64((values[25]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("GdipGetImageGraphicsContext", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 77;
            }
            81 => {
                pc = if (u64::from((values[25]) == (0x0u64))) != 0 {
                    79
                } else {
                    80
                };
            }
            82 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = values[25];
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = relocate("STACK_pcStack_70");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 84;
            }
            86 => {
                values[25] = {
                    let call_arguments = [
                        values[24],
                        (memory.read_u64(relocate("STACK_local_88"))?) & (0xffffffffu64),
                        (memory.read_u64(relocate("STACK_local_90"))?) & (0xffffffffu64),
                        0xe200bu64,
                    ];
                    invoke("FUN_14003bb40", &call_arguments, memory)
                };
                pc = 85;
            }
            87 => {
                {
                    let destination = (values[24]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (values[24]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = values[24];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 88;
            }
            90 => {
                pc = if (u64::from((values[24]) != (0x0u64))) != 0 {
                    89
                } else {
                    85
                };
            }
            91 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_90")).wrapping_add(0x0u64))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 90;
            }
            92 => {
                values[24] = {
                    let call_arguments = [];
                    invoke("GdipAlloc", &call_arguments, memory)
                };
                pc = 91;
            }
            93 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[14]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 92;
            }
            94 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    93
                } else {
                    92
                };
            }
            95 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(relocate("STACK_local_58"))?)
                                .wrapping_add((0x1u64).wrapping_mul(8)),
                        )?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("GdipGetImageHeight", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_90"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = (values[24]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (values[14]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 96;
            }
            98 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    97
                } else {
                    96
                };
            }
            99 => {
                values[10] = memory.read_u64(relocate("STACK_local_58"))?;
                pc = 98;
            }
            100 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(relocate("STACK_local_58"))?)
                                .wrapping_add((0x1u64).wrapping_mul(8)),
                        )?,
                        relocate("STACK_local_88"),
                    ];
                    invoke("GdipGetImageWidth", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 99;
            }
            101 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_88"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("STACK_local_58"))?) == (0x0u64))) != 0)
                        || (({
                            let _ = {
                                let _ = {
                                    values[14] =
                                        ((u64::from(
                                            memory.read_u32(
                                                (memory.read_u64(relocate("STACK_local_58"))?)
                                                    .wrapping_add((0x2u64).wrapping_mul(8)),
                                            )?,
                                        )) as i32 as i64
                                            as u64) as i32
                                            as i64 as u64;
                                    values[14]
                                };
                                {
                                    let destination = (memory
                                        .read_u64(relocate("STACK_local_58"))?)
                                    .wrapping_add((0x2u64).wrapping_mul(8));
                                    let stored = u64::from((0x0u64) as u32);
                                    memory.write_u32(destination, stored as u32)?;
                                    stored
                                }
                            };
                            u64::from(
                                (((values[14]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    29
                } else {
                    101
                };
            }
            103 => {
                values[24] = memory.read_u64(relocate("STACK_local_58"))?;
                pc = 102;
            }
            104 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_60"))?];
                    invoke("BasePaint::Image::Bitmap", &call_arguments, memory)
                };
                pc = 103;
            }
            105 => {
                {
                    let destination = relocate("STACK_local_58");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 104;
            }
            106 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("STACK_local_60"))?) == (0x0u64))) != 0)
                        || (({
                            let _ = {
                                values[11] = u64::from(
                                    ({
                                        let call_arguments = [
                                            memory.read_u64(relocate("STACK_local_60"))?,
                                            relocate("DAT_1401d06b8"),
                                            relocate("STACK_local_50"),
                                        ];
                                        invoke(
                                            "Paint::AI::AddPerceptibleWatermark",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[11]
                            };
                            u64::from(!((values[11]) != 0))
                        }) != 0),
                )) != 0
                {
                    8
                } else {
                    105
                };
            }
            107 => {
                {
                    let call_arguments = [];
                    invoke(
                        "Paint::AI::GetPerceptibleWatermarkSvg",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 106;
            }
            108 => {
                {
                    let call_arguments = [relocate("STACK_local_60"), relocate("DAT_1401d06b8")];
                    invoke("BasePaint::Image::Create", &call_arguments, memory)
                };
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                {
                    let call_arguments = [
                        values[23],
                        relocate("STACK_local_48"),
                        0x21u64,
                        relocate("STACK_local_130"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendPerformanceStartEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 109;
            }
            111 => {
                {
                    let destination = relocate("STACK_local_f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                values[23] = {
                    let call_arguments = [];
                    invoke(
                        "Paint::Telemetry::TraceLogger::GetInstance",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 111;
            }
            113 => {
                values[25] = 0x0u64;
                pc = 112;
            }
            114 => {
                pc = 1;
            }
            115 => {
                pc = if (u64::from(
                    ((((values[16]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    114
                } else {
                    113
                };
            }
            116 => {
                values[17] = (((values[22]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[12]) & (0xffffffffu64));
                pc = 115;
            }
            117 => {
                values[22] = u64::from(
                    (u64::from(((values[1]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 116;
            }
            118 => {
                values[12] = ((values[1]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 117;
            }
            119 => {
                values[1] = {
                    let call_arguments = [values[18], 0x0u64, values[8]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 118;
            }
            120 => {
                values[18] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 119;
            }
            121 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            122 => {
                pc = if (u64::from(
                    ((((values[13]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    121
                } else {
                    115
                };
            }
            123 => {
                pc = if (u64::from(
                    (u64::from((values[13]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    120
                } else {
                    122
                };
            }
            124 => {
                values[17] = values[13];
                pc = 123;
            }
            125 => {
                values[13] =
                    u64::from((((values[14]).wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32);
                pc = 124;
            }
            126 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 125;
            }
            127 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 126;
            }
            128 => {
                values[14] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 127;
            }
            129 => {
                values[5] = (memory.read_u64(relocate("STACK_local_80"))?).wrapping_add(0x18u64);
                pc = 128;
            }
            130 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 129;
            }
            131 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_80"))?) != (0x0u64))) != 0
                {
                    130
                } else {
                    115
                };
            }
            132 => {
                values[8] = memory.read_u64(relocate("STACK_local_80"))?;
                pc = 131;
            }
            133 => {
                values[17] = (((values[21]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[12]) & (0xffffffffu64));
                pc = 132;
            }
            134 => {
                values[21] = u64::from(
                    (u64::from(((values[2]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 133;
            }
            135 => {
                values[12] = ((values[2]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 134;
            }
            136 => {
                values[2] = {
                    let call_arguments = [values[18], 0x0u64, values[25]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 135;
            }
            137 => {
                values[18] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 136;
            }
            138 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            139 => {
                pc = if (u64::from(
                    ((((values[13]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    138
                } else {
                    132
                };
            }
            140 => {
                pc = if (u64::from(
                    (u64::from((values[13]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    137
                } else {
                    139
                };
            }
            141 => {
                values[17] = values[13];
                pc = 140;
            }
            142 => {
                values[13] =
                    u64::from((((values[14]).wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32);
                pc = 141;
            }
            143 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 142;
            }
            144 => {
                {
                    let destination = values[28];
                    let stored = ((((u64::from(memory.read_u32(values[28])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 143;
            }
            145 => {
                values[14] = ((u64::from(memory.read_u32(values[28])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 144;
            }
            146 => {
                values[28] = (memory.read_u64(relocate("STACK_local_78"))?).wrapping_add(0x3u64);
                pc = 145;
            }
            147 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 146;
            }
            148 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_78"))?) != (0x0u64))) != 0
                {
                    147
                } else {
                    132
                };
            }
            149 => {
                values[25] = memory.read_u64(relocate("STACK_local_78"))?;
                pc = 148;
            }
            150 => {
                values[17] = (((values[20]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[12]) & (0xffffffffu64));
                pc = 149;
            }
            151 => {
                values[20] = u64::from(
                    (u64::from(((values[3]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 150;
            }
            152 => {
                values[12] = ((values[3]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 151;
            }
            153 => {
                values[3] = {
                    let call_arguments = [values[18], 0x0u64, values[25]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 152;
            }
            154 => {
                values[18] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 153;
            }
            155 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            156 => {
                pc = if (u64::from(
                    ((((values[13]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    155
                } else {
                    149
                };
            }
            157 => {
                pc = if (u64::from(
                    (u64::from((values[13]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    154
                } else {
                    156
                };
            }
            158 => {
                values[17] = values[13];
                pc = 157;
            }
            159 => {
                values[13] =
                    u64::from((((values[14]).wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32);
                pc = 158;
            }
            160 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 159;
            }
            161 => {
                {
                    let destination = values[28];
                    let stored = ((((u64::from(memory.read_u32(values[28])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 160;
            }
            162 => {
                values[14] = ((u64::from(memory.read_u32(values[28])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 161;
            }
            163 => {
                values[28] = (values[25]).wrapping_add((0x3u64).wrapping_mul(8));
                pc = 162;
            }
            164 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 163;
            }
            165 => {
                pc = if (u64::from((values[25]) != (0x0u64))) != 0 {
                    164
                } else {
                    149
                };
            }
            166 => {
                values[17] = (((values[19]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[12]) & (0xffffffffu64));
                pc = 165;
            }
            167 => {
                values[19] = u64::from(
                    (u64::from(((values[4]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 166;
            }
            168 => {
                values[12] = ((values[4]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 167;
            }
            169 => {
                values[4] = {
                    let call_arguments = [values[18], 0x0u64, values[28]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 168;
            }
            170 => {
                values[18] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 169;
            }
            171 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            172 => {
                pc = if (u64::from(
                    (((((values[14]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    171
                } else {
                    165
                };
            }
            173 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    170
                } else {
                    172
                };
            }
            174 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 173;
            }
            175 => {
                {
                    let destination = values[27];
                    let stored = ((((u64::from(memory.read_u32(values[27])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 174;
            }
            176 => {
                values[14] = ((u64::from(memory.read_u32(values[27])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 175;
            }
            177 => {
                values[27] = (memory.read_u64(relocate("STACK_local_88"))?)
                    .wrapping_add((0x3u64).wrapping_mul(8));
                pc = 176;
            }
            178 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 177;
            }
            179 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_88"))?) != (0x0u64))) != 0
                {
                    178
                } else {
                    165
                };
            }
            180 => {
                values[17] = values[16];
                pc = 179;
            }
            181 => {
                values[28] = memory.read_u64(relocate("STACK_local_88"))?;
                pc = 180;
            }
            182 => {
                values[16] = {
                    let call_arguments = [
                        relocate("STACK_local_88"),
                        relocate("STACK_local_90"),
                        relocate("STACK_local_78"),
                        relocate("STACK_local_80"),
                    ];
                    invoke("FUN_140045460", &call_arguments, memory)
                };
                pc = 181;
            }
            183 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_88"),
                        relocate("STACK_local_168"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 183;
            }
            185 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 184;
            }
            186 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 185;
            }
            187 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 182;
            }
            188 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 187;
            }
            189 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 188;
            }
            190 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_148")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            191 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    190
                } else {
                    189
                };
            }
            192 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_130"))?,
                        relocate("STACK_local_88"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 191;
            }
            193 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 192;
            }
            194 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 193;
            }
            195 => {
                {
                    let destination =
                        (relocate("STACK_local_148")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 194;
            }
            196 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 195;
            }
            197 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    186
                } else {
                    196
                };
            }
            198 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 197;
            }
            199 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 198;
            }
            200 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 199;
            }
            201 => {
                {
                    let destination = relocate("STACK_local_160");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 200;
            }
            202 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = relocate("STACK_local_130");
                    memory.write_u64(destination, stored)?;
                }
                pc = 201;
            }
            203 => {
                {
                    let destination = relocate("STACK_local_130");
                    let stored = relocate("STACK_local_128");
                    memory.write_u64(destination, stored)?;
                }
                pc = 202;
            }
            204 => {
                {
                    let destination = relocate("STACK_local_118");
                    let stored = relocate("L\"PerceptibleWatermarkPopupConfirmationTitle\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 203;
            }
            205 => {
                {
                    let destination = relocate("STACK_local_124");
                    let stored = u64::from((0x2au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 204;
            }
            206 => {
                {
                    let destination = relocate("STACK_local_128");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 205;
            }
            207 => {
                values[6] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 206;
            }
            208 => {
                values[25] = memory.read_u64(relocate("STACK_local_90"))?;
                pc = 207;
            }
            209 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_90"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 208;
            }
            210 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 209;
            }
            211 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 210;
            }
            212 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 211;
            }
            213 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 208;
            }
            214 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 213;
            }
            215 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 214;
            }
            216 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_148")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            217 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    216
                } else {
                    215
                };
            }
            218 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_b0"))?,
                        relocate("STACK_local_90"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 217;
            }
            219 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 218;
            }
            220 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 219;
            }
            221 => {
                {
                    let destination =
                        (relocate("STACK_local_148")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 220;
            }
            222 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 221;
            }
            223 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    212
                } else {
                    222
                };
            }
            224 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 223;
            }
            225 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 224;
            }
            226 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 225;
            }
            227 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 226;
            }
            228 => {
                {
                    let destination = relocate("STACK_local_88");
                    let stored = relocate("STACK_local_b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 227;
            }
            229 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("STACK_local_a8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 228;
            }
            230 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = relocate("L\"PerceptibleWatermarkPopupConfirmationContent\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 229;
            }
            231 => {
                {
                    let destination = relocate("STACK_local_a4");
                    let stored = u64::from((0x2cu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 230;
            }
            232 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 231;
            }
            233 => {
                values[6] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 232;
            }
            234 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_78"),
                        relocate("STACK_local_90"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 233;
            }
            235 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 234;
            }
            236 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 235;
            }
            237 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 236;
            }
            238 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 233;
            }
            239 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 238;
            }
            240 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 239;
            }
            241 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_148")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            242 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    241
                } else {
                    240
                };
            }
            243 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_d0"))?,
                        relocate("STACK_local_78"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 242;
            }
            244 => {
                {
                    let destination = relocate("STACK_local_138");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            245 => {
                {
                    let destination = relocate("STACK_local_140");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 244;
            }
            246 => {
                {
                    let destination =
                        (relocate("STACK_local_148")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x5783u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 245;
            }
            247 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 246;
            }
            248 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    237
                } else {
                    247
                };
            }
            249 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 248;
            }
            250 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 249;
            }
            251 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 250;
            }
            252 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 251;
            }
            253 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = relocate("STACK_local_d0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 252;
            }
            254 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = relocate("STACK_local_c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 253;
            }
            255 => {
                {
                    let destination = relocate("STACK_local_b8");
                    let stored = relocate("L\"PerceptibleWatermarkPopupConfirmationYes\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 254;
            }
            256 => {
                {
                    let destination = relocate("STACK_local_c4");
                    let stored = u64::from((0x28u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 255;
            }
            257 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 256;
            }
            258 => {
                values[6] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 257;
            }
            259 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_80"),
                        relocate("STACK_local_78"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 258;
            }
            260 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 259;
            }
            261 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 260;
            }
            262 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 261;
            }
            263 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 258;
            }
            264 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 263;
            }
            265 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 264;
            }
            266 => {
                {
                    let call_arguments = [values[14], relocate("STACK_local_78")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            267 => {
                pc = if (u64::from(
                    (((values[14]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    266
                } else {
                    265
                };
            }
            268 => {
                values[14] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_f0"))?,
                        relocate("STACK_local_80"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 267;
            }
            269 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 268;
            }
            270 => {
                {
                    let destination = relocate("STACK_pcStack_70");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 269;
            }
            271 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = ((((values[17]) as i32 as i64 as u64) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 270;
            }
            272 => {
                values[17] =
                    (memory.read_u64(relocate("STACK_local_78"))?).wrapping_shr((0x20u64) as u32);
                pc = 271;
            }
            273 => {
                {
                    let destination = relocate("STACK_local_80");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 272;
            }
            274 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    262
                } else {
                    273
                };
            }
            275 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 274;
            }
            276 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 275;
            }
            277 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 276;
            }
            278 => {
                {
                    let destination = relocate("STACK_local_168");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 277;
            }
            279 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = relocate("STACK_local_f0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 278;
            }
            280 => {
                {
                    let destination = relocate("STACK_local_f0");
                    let stored = relocate("STACK_local_e8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 279;
            }
            281 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = relocate("L\"PerceptibleWatermarkPopupConfirmationNo\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 280;
            }
            282 => {
                {
                    let destination = relocate("STACK_local_e4");
                    let stored = u64::from((0x27u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 281;
            }
            283 => {
                {
                    let destination = relocate("STACK_local_e8");
                    let stored = u64::from((0x1u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 282;
            }
            284 => {
                pc = if (u64::from(
                    ((((values[16]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    283
                } else {
                    113
                };
            }
            285 => {
                pc = if (u64::from(
                    ((((values[16]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    1
                } else {
                    284
                };
            }
            286 => {
                values[6] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 285;
            }
            287 => {
                values[17] = {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 286;
            }
            288 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_90"))?) != (0x0u64))) != 0
                {
                    287
                } else {
                    286
                };
            }
            289 => {
                values[17] = values[16];
                pc = 288;
            }
            290 => {
                values[16] = {
                    let call_arguments = [relocate("STACK_local_90")];
                    invoke("FUN_14005d0a0", &call_arguments, memory)
                };
                pc = 289;
            }
            291 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b78"))?)?)
                            .wrapping_add((0x2u64).wrapping_mul(4)),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 290;
            }
            292 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401d0b78"))?) != (0x0u64))) != 0
                {
                    291
                } else {
                    290
                };
            }
            293 => {
                {
                    let destination = relocate("STACK_local_158");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 292;
            }
            294 => {
                {
                    let destination = relocate("STACK_local_90");
                    let stored = memory.read_u64(relocate("DAT_1401d0b78"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 293;
            }
            295 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_198"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 294;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140065c80` (`mspaint.c:91993`).
pub fn fun_140065c80(
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
    ];
    let mut pc: i32 = 115;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                pc = 47;
            }
            2 => {
                values[8] = values[3];
                pc = 1;
            }
            3 => {
                {
                    let destination = values[18];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 2;
            }
            4 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination = (values[15]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination = values[7];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 6;
            }
            8 => {
                values[7] = memory.read_u64((values[15]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 7;
            }
            9 => {
                {
                    let destination = values[6];
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                values[6] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 9;
            }
            11 => {
                {
                    let destination = values[5];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                values[5] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 11;
            }
            13 => {
                pc = if (u64::from((memory.read_u64(values[18])?) != (values[15]))) != 0 {
                    45
                } else {
                    12
                };
            }
            14 => {
                pc = 47;
            }
            15 => {
                values[8] = values[3];
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 15;
            }
            17 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[18];
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[14]).wrapping_add(0x8u64);
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = values[6];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                values[6] = memory.read_u64((values[14]).wrapping_add(0x8u64))?;
                pc = 19;
            }
            21 => {
                {
                    let destination = values[15];
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                values[15] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 21;
            }
            23 => {
                {
                    let destination = values[18];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 22;
            }
            24 => {
                values[18] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 23;
            }
            25 => {
                values[14] = memory.read_u64(values[15])?;
                pc = 24;
            }
            26 => {
                pc = if (u64::from(!((values[19]) != 0))) != 0 {
                    25
                } else {
                    13
                };
            }
            27 => {
                values[19] = 0u64;
                pc = 26;
            }
            28 => {
                values[19] = u64::from(
                    (u64::from(
                        (((values[10]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0,
                );
                pc = 26;
            }
            29 => {
                values[10] = ({
                    let call_arguments = [values[17], values[13]];
                    invoke("FUN_14003bc20", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    27
                } else {
                    29
                };
            }
            31 => {
                values[19] = 1u64;
                pc = 26;
            }
            32 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((values[10]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    30
                } else {
                    31
                };
            }
            33 => {
                values[17] = relocate("DAT_140105708");
                pc = 32;
            }
            34 => {
                values[16] = (0x0u64) as i32 as i64 as u64;
                pc = 33;
            }
            35 => {
                values[17] = memory.read_u64((values[14]).wrapping_add(0x10u64))?;
                pc = 32;
            }
            36 => {
                values[16] = ((u64::from(memory.read_u32((values[14]).wrapping_add(0x4u64))?))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 35;
            }
            37 => {
                pc = if (u64::from(((values[14]) as i64) == ((0x0u64) as i64))) != 0 {
                    34
                } else {
                    36
                };
            }
            38 => {
                values[14] = memory.read_u64((values[8]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 37;
            }
            39 => {
                values[13] = relocate("DAT_140105708");
                pc = 38;
            }
            40 => {
                values[10] = (0x0u64) as i32 as i64 as u64;
                pc = 39;
            }
            41 => {
                values[13] = memory.read_u64((values[14]).wrapping_add(0x10u64))?;
                pc = 38;
            }
            42 => {
                values[10] = ((u64::from(memory.read_u32((values[14]).wrapping_add(0x4u64))?))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 41;
            }
            43 => {
                pc = if (u64::from(((values[14]) as i64) == ((0x0u64) as i64))) != 0 {
                    40
                } else {
                    42
                };
            }
            44 => {
                values[14] =
                    memory.read_u64((values[15]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 43;
            }
            45 => {
                values[15] =
                    memory.read_u64((values[15]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 44;
            }
            46 => {
                pc = if (u64::from((memory.read_u64(values[18])?) != (values[15]))) != 0 {
                    45
                } else {
                    12
                };
            }
            47 => {
                pc = 81;
            }
            48 => {
                values[8] = values[3];
                pc = 47;
            }
            49 => {
                {
                    let destination = (values[18]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = (values[8]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (values[15]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[6];
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = values[7];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                values[7] = memory.read_u64((values[15]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 53;
            }
            55 => {
                {
                    let destination = values[6];
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                values[6] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 55;
            }
            57 => {
                {
                    let destination = values[5];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                values[5] = memory.read_u64((values[8]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 57;
            }
            59 => {
                pc = if (u64::from((values[15]) != (values[8]))) != 0 {
                    58
                } else {
                    49
                };
            }
            60 => {
                values[15] = memory.read_u64(values[15])?;
                pc = 59;
            }
            61 => {
                pc = 46;
            }
            62 => {
                pc = if (values[19]) != 0 { 61 } else { 60 };
            }
            63 => {
                values[19] = 0u64;
                pc = 62;
            }
            64 => {
                values[19] = u64::from(
                    (u64::from(
                        (((values[10]) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0,
                );
                pc = 62;
            }
            65 => {
                values[10] = ({
                    let call_arguments = [values[17], values[13]];
                    invoke("FUN_14003bc20", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 64;
            }
            66 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    65
                };
            }
            67 => {
                values[19] = 1u64;
                pc = 62;
            }
            68 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((values[10]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    66
                } else {
                    67
                };
            }
            69 => {
                values[17] = relocate("DAT_140105708");
                pc = 68;
            }
            70 => {
                values[16] = (0x0u64) as i32 as i64 as u64;
                pc = 69;
            }
            71 => {
                values[17] = memory.read_u64((values[14]).wrapping_add(0x10u64))?;
                pc = 68;
            }
            72 => {
                values[16] = ((u64::from(memory.read_u32((values[14]).wrapping_add(0x4u64))?))
                    as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 71;
            }
            73 => {
                pc = if (u64::from(((values[14]) as i64) == ((0x0u64) as i64))) != 0 {
                    70
                } else {
                    72
                };
            }
            74 => {
                values[13] = relocate("DAT_140105708");
                pc = 73;
            }
            75 => {
                values[10] = (0x0u64) as i32 as i64 as u64;
                pc = 74;
            }
            76 => {
                values[13] = memory.read_u64((values[4]).wrapping_add(0x10u64))?;
                pc = 73;
            }
            77 => {
                values[10] = ((u64::from(memory.read_u32((values[4]).wrapping_add(0x4u64))?)) as i32
                    as i64 as u64) as i32 as i64 as u64;
                pc = 76;
            }
            78 => {
                pc = if (u64::from(((values[4]) as i64) == ((0x0u64) as i64))) != 0 {
                    75
                } else {
                    77
                };
            }
            79 => {
                values[4] = memory.read_u64((values[15]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 78;
            }
            80 => {
                values[15] =
                    memory.read_u64((values[18]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 79;
            }
            81 => {
                pc = 99;
            }
            82 => {
                values[8] = values[3];
                pc = 81;
            }
            83 => {
                {
                    let destination = (values[18]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let destination = values[18];
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 83;
            }
            85 => {
                pc = 80;
            }
            86 => {
                pc = if (u64::from((memory.read_u64(values[18])?) != (values[9]))) != 0 {
                    85
                } else {
                    84
                };
            }
            87 => {
                values[18] = (memory.read_u64(relocate("DAT_1401d0e68"))?).wrapping_add(
                    ((values[11]) & (memory.read_u64(relocate("DAT_1401d0e80"))?))
                        .wrapping_mul(0x10u64),
                );
                pc = 86;
            }
            88 => {
                pc = if (u64::from(
                    (values[12])
                        < ((u64::from(
                            (u64::from(memory.read_u32((values[14]).wrapping_add(0x4u64))?)) as u32,
                        ))
                        .wrapping_mul(0x2u64)),
                )) != 0
                {
                    91
                } else {
                    87
                };
            }
            89 => {
                values[11] = ((values[11])
                    ^ (u64::from((u64::from(memory.read_u8(values[2])?)) as u8)))
                .wrapping_mul(0x100000001b3u64);
                pc = 88;
            }
            90 => {
                values[12] = (values[12]).wrapping_add(0x1u64);
                pc = 89;
            }
            91 => {
                values[2] =
                    (memory.read_u64((values[14]).wrapping_add(0x10u64))?).wrapping_add(values[12]);
                pc = 90;
            }
            92 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[14]).wrapping_add(0x4u64))?)) as u32,
                    )) != (0x0u64),
                )) != 0
                {
                    91
                } else {
                    87
                };
            }
            93 => {
                values[12] = 0x0u64;
                pc = 92;
            }
            94 => {
                pc = if (u64::from(((values[14]) as i64) != ((0x0u64) as i64))) != 0 {
                    93
                } else {
                    87
                };
            }
            95 => {
                values[3] = memory.read_u64(values[8])?;
                pc = 94;
            }
            96 => {
                values[11] = 0xcbf29ce484222325u64;
                pc = 95;
            }
            97 => {
                values[14] = memory.read_u64((values[8]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                pc = 96;
            }
            98 => {
                return Ok(None);
            }
            99 => {
                pc = if (u64::from((values[8]) == (values[9]))) != 0 {
                    98
                } else {
                    97
                };
            }
            100 => {
                values[8] = memory.read_u64(memory.read_u64(relocate("DAT_1401d0e58"))?)?;
                pc = 47;
            }
            101 => {
                {
                    let destination = relocate("DAT_1401d0e88");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 100;
            }
            102 => {
                {
                    let destination = relocate("DAT_1401d0e80");
                    let stored = (values[14]).wrapping_sub(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let call_arguments =
                        [relocate("DAT_1401d0e68"), (values[14]).wrapping_mul(0x2u64)];
                    invoke("FUN_140044150", &call_arguments, memory)
                };
                pc = 102;
            }
            104 => {
                values[14] = ((0x1u64 as i32).wrapping_shl(
                    (u64::from(
                        ((u64::from(
                            (((values[14]) as i8 as i64 as u64).wrapping_add(0x1u64)) as u32,
                        )) & (0x3fu64)) as u32,
                    )) as u32,
                ) as u64) as i32 as i64 as u64;
                pc = 103;
            }
            105 => {
                pc = if (u64::from(((values[11]).wrapping_shr((values[14]) as u32)) == (0x0u64)))
                    != 0
                {
                    106
                } else {
                    104
                };
            }
            106 => {
                let _ = {
                    values[14] =
                        (values[14]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    values[14]
                };
                pc = 105;
            }
            107 => {
                pc = if (u64::from((values[11]) != (0x0u64))) != 0 {
                    105
                } else {
                    104
                };
            }
            108 => {
                values[14] = 0x3fu64;
                pc = 107;
            }
            109 => {
                values[11] = ((values[1]).wrapping_sub(0x1u64)) | (0x1u64);
                pc = 108;
            }
            110 => {
                {
                    let call_arguments = [relocate("\"invalid hash bucket count\"")];
                    invoke("std::_Xlength_error", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xlength_error"));
            }
            111 => {
                pc = if (u64::from(
                    (((0x1u64 as i32).wrapping_shl(
                        (((u64::from((values[14]) as u8)) & (0x3fu64)) as i32 as i64 as u64) as u32,
                    ) as u64) as i32 as i64 as u64)
                        < (values[1]),
                )) != 0
                {
                    110
                } else {
                    109
                };
            }
            112 => {
                pc = if (u64::from(
                    ((0xfffffffffffffffu64).wrapping_shr((values[14]) as u32)) == (0x0u64),
                )) != 0
                {
                    113
                } else {
                    111
                };
            }
            113 => {
                let _ = {
                    values[14] =
                        (values[14]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    values[14]
                };
                pc = 112;
            }
            114 => {
                let _ = {
                    values[14] = 0x3fu64;
                    values[14]
                };
                pc = 112;
            }
            115 => {
                values[9] = memory.read_u64(relocate("DAT_1401d0e58"))?;
                pc = 114;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140067520` (`mspaint.c:93448`).
pub fn fun_140067520(
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
    ];
    let mut pc: i32 = 36;
    loop {
        match pc {
            0 => {
                return Ok(Some((values[4]) as i32 as i64 as u64));
            }
            1 => {
                {
                    let destination = (values[1]).wrapping_add((0x40u64).wrapping_mul(1));
                    let stored = relocate("u_MSPaintApp_1401790e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                return Ok(Some((0x0u64) as i32 as i64 as u64));
            }
            3 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    2
                } else {
                    1
                };
            }
            4 => {
                values[5] = ({
                    let call_arguments = [relocate("STACK_local_88")];
                    invoke("AfxRegisterClass", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 3;
            }
            5 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = {
                        let call_arguments = [values[2], relocate("DAT_00000002")];
                        invoke("LoadIconW", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x40u64).wrapping_mul(1));
                    let stored = relocate("u_MSPaintApp_1401790e0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 5;
            }
            7 => {
                {
                    let destination =
                        (relocate("STACK_local_88")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(
                                    memory.read_u32(
                                        (relocate("STACK_local_88"))
                                            .wrapping_add((0x0u64).wrapping_mul(1)),
                                    )?,
                                )) as u32,
                            )) & (0xfffffffcu64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 6;
            }
            8 => {
                {
                    let call_arguments = [
                        values[2],
                        memory.read_u64((values[1]).wrapping_add((0x40u64).wrapping_mul(1)))?,
                        relocate("STACK_local_88"),
                    ];
                    invoke("GetClassInfoW", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    8
                } else {
                    1
                };
            }
            10 => {
                values[6] = ({
                    let call_arguments = [
                        values[2],
                        relocate("u_MSPaintApp_1401790e0"),
                        relocate("STACK_local_88"),
                    ];
                    invoke("GetClassInfoW", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 9;
            }
            11 => {
                values[2] = memory.read_u64((values[7]).wrapping_add((0x10u64).wrapping_mul(1)))?;
                pc = 10;
            }
            12 => {
                values[7] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[1]).wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = (values[8]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                {
                    let destination = (values[1]).wrapping_add((0x24u64).wrapping_mul(1));
                    let stored = (values[5]) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 13;
            }
            15 => {
                values[5] = ((((u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0x8u64).wrapping_mul(1)),
                )?)) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 14;
            }
            16 => {
                {
                    let destination = (values[1]).wrapping_add((0x2cu64).wrapping_mul(1));
                    let stored = ((u64::from(memory.read_u32(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(1)),
                    )?)) as i32 as i64 as u64) as i32 as i64
                        as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 15;
            }
            17 => {
                values[8] = ((((u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0xcu64).wrapping_mul(1)),
                )?)) as i32 as i64 as u64)
                    .wrapping_sub(
                        (u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x4u64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64,
                    )) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[1]).wrapping_add((0x28u64).wrapping_mul(1));
                    let stored = ((u64::from(memory.read_u32(
                        (relocate("STACK_local_38")).wrapping_add((0x4u64).wrapping_mul(1)),
                    )?)) as i32 as i64 as u64) as i32 as i64
                        as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("_DAT_1401d08ec");
                    let stored = u64::from(
                        ((u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0xcu64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = relocate("iRam00000001401d08e8");
                    let stored = u64::from(
                        ((u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x8u64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("_DAT_1401d08e4");
                    let stored = u64::from(
                        ((u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x4u64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("_DAT_1401d08e0");
                    let stored = u64::from(
                        ((u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(1)),
                        )?)) as i32 as i64 as u64) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                {
                    let destination = (values[1]).wrapping_add((0x28u64).wrapping_mul(1));
                    let stored = u64::from((0x80000000u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 14;
            }
            24 => {
                values[5] =
                    (u64::from((0u64.wrapping_sub(0x80000000u64)) as u32)) as i32 as i64 as u64;
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[1]).wrapping_add((0x2cu64).wrapping_mul(1));
                    let stored = u64::from((0x80000000u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                values[8] =
                    (u64::from((0u64.wrapping_sub(0x80000000u64)) as u32)) as i32 as i64 as u64;
                pc = 25;
            }
            27 => {
                pc = if (values[3]) != 0 { 22 } else { 26 };
            }
            28 => {
                values[3] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_38")];
                        invoke("EnsureWindowIsVisibleOnScreen", &call_arguments, memory)
                    }) != 0,
                );
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [relocate("STACK_local_38"), relocate("DAT_1401d08e0")];
                    invoke("CopyRect", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    (((values[4]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    29
                } else {
                    0
                };
            }
            31 => {
                {
                    let destination = (values[1]).wrapping_add((0x48u64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(
                                    (values[1]).wrapping_add((0x48u64).wrapping_mul(1)),
                                )?)) as u32,
                            )) & (0xeffffdffu64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 30;
            }
            32 => {
                values[4] = ({
                    let call_arguments = [values[0], values[1]];
                    invoke("CFrameWnd::PreCreateWindow", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 31;
            }
            33 => {
                {
                    let destination = (values[1]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let call_arguments =
                        [memory.read_u64((values[1]).wrapping_add((0x10u64).wrapping_mul(1)))?];
                    invoke("DestroyMenu", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                pc = if (u64::from(
                    (memory.read_u64((values[1]).wrapping_add((0x10u64).wrapping_mul(1)))?)
                        != (0x0u64),
                )) != 0
                {
                    34
                } else {
                    32
                };
            }
            36 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_a8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 35;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14006a430` (`mspaint.c:95725`).
pub fn fun_14006a430(
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
    let mut pc: i32 = 46;
    loop {
        match pc {
            0 => {
                return Ok(Some((values[11]) & (0xffffffffffffff00u64)));
            }
            1 => {
                {
                    let destination = (values[1]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 0;
            }
            2 => {
                values[11] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x20u64))?,
                        values[5],
                        u64::from((values[5]) != (values[1])),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 1;
            }
            3 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    2
                } else {
                    0
                };
            }
            4 => {
                values[5] = memory.read_u64((values[1]).wrapping_add((0x7u64).wrapping_mul(8)))?;
                pc = 3;
            }
            5 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 4;
            }
            6 => {
                values[11] = (((values[13]) & (0xffffffffu64)).wrapping_shl((0x20u64) as u32))
                    | ((values[8]) & (0xffffffffu64));
                pc = 5;
            }
            7 => {
                values[13] = u64::from(
                    (u64::from(((values[2]).wrapping_shr((0x20u64) as u32)) as u32)) as u32,
                );
                pc = 6;
            }
            8 => {
                values[8] = ((values[2]) as i32 as i64 as u64) as i32 as i64 as u64;
                pc = 7;
            }
            9 => {
                values[2] = {
                    let call_arguments = [values[12], 0x0u64, values[6]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 8;
            }
            10 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            12 => {
                pc = if (u64::from(
                    ((((values[7]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    11
                } else {
                    5
                };
            }
            13 => {
                pc = if (u64::from((u64::from((values[7]) as u32)) == (u64::from((0x0u64) as u32))))
                    != 0
                {
                    10
                } else {
                    12
                };
            }
            14 => {
                values[11] = values[7];
                pc = 13;
            }
            15 => {
                values[7] =
                    u64::from((((values[4]).wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32);
                pc = 14;
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 16;
            }
            18 => {
                values[4] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 17;
            }
            19 => {
                values[3] = (values[6]).wrapping_add(0x18u64);
                pc = 18;
            }
            20 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 19;
            }
            21 => {
                pc = if (u64::from((values[6]) != (0x0u64))) != 0 {
                    20
                } else {
                    4
                };
            }
            22 => {
                values[6] = memory.read_u64(values[0])?;
                pc = 21;
            }
            23 => {
                values[11] = {
                    let call_arguments = [values[9]];
                    invoke("FUN_1400c85c0", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let destination = (values[9]).wrapping_add(0x11u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[9]).wrapping_add((0xcu64).wrapping_mul(8));
                    let stored = u64::from((0x10002u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = (values[1]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(
                            memory.read_u64((values[1]).wrapping_add((0x7u64).wrapping_mul(8)))?,
                        )?)
                        .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                pc = 25;
            }
            29 => {
                pc = if (u64::from(
                    (memory.read_u64((values[1]).wrapping_add((0x7u64).wrapping_mul(8)))?)
                        == (0x0u64),
                )) != 0
                {
                    28
                } else {
                    27
                };
            }
            30 => {
                {
                    let destination = (values[9]).wrapping_add((0xbu64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                values[10] = {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                {
                    let destination = (values[9]).wrapping_add((0xbu64).wrapping_mul(8));
                    let stored = values[5];
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            33 => {
                pc = if (u64::from((values[5]) == (values[1]))) != 0 {
                    31
                } else {
                    32
                };
            }
            34 => {
                pc = if (u64::from((values[5]) != (0x0u64))) != 0 {
                    33
                } else {
                    25
                };
            }
            35 => {
                {
                    let destination = relocate("STACK_local_48");
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                values[5] = memory.read_u64((values[1]).wrapping_add((0x7u64).wrapping_mul(8)))?;
                pc = 35;
            }
            37 => {
                {
                    let destination = (values[9]).wrapping_add((0xbu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 36;
            }
            38 => {
                {
                    let destination = (values[9]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = values[0];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                values[10] = memory.read_u64(values[0])?;
                pc = 39;
            }
            41 => {
                {
                    let destination = values[9];
                    let stored = relocate("FUN_1400c85c0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                {
                    let destination = (values[9]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("FUN_1400c85b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 41;
            }
            43 => {
                values[9] = {
                    let call_arguments = [0x180u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
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
                {
                    let destination = relocate("STACK_local_20");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14006d6e0` (`mspaint.c:98085`).
pub fn fun_14006d6e0(
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
    ];
    let mut pc: i32 = 388;
    loop {
        match pc {
            0 => {
                return Ok(Some(0x1u64));
            }
            1 => {
                {
                    let call_arguments = [values[11], 0x0u64, values[20]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[11] = {
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
                return Ok(Some(0x1u64));
            }
            5 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((((values[8])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    4
                } else {
                    3
                };
            }
            6 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    2
                };
            }
            7 => {
                values[20] = memory.read_u64(relocate("STACK_local_3d8"))?;
                pc = 6;
            }
            8 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 7;
            }
            9 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 8;
            }
            10 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 9;
            }
            11 => {
                values[1] = (memory.read_u64(relocate("STACK_local_3d8"))?).wrapping_add(0x18u64);
                pc = 10;
            }
            12 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 11;
            }
            13 => {
                return Ok(Some(0x1u64));
            }
            14 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_3d8"))?) == (0x0u64)))
                    != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                {
                    let call_arguments = [];
                    invoke("COleObjectFactory::RegisterAll", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                {
                    let call_arguments = [];
                    invoke("TraceLaunchWithAutomation", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                pc = 2;
            }
            18 => {
                pc = 3;
            }
            19 => {
                return Ok(Some(0x1u64));
            }
            20 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((((values[8])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    19
                } else {
                    18
                };
            }
            21 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    20
                } else {
                    17
                };
            }
            22 => {
                values[20] = memory.read_u64(relocate("STACK_local_3d8"))?;
                pc = 21;
            }
            23 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 22;
            }
            24 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 23;
            }
            25 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 24;
            }
            26 => {
                values[1] = (memory.read_u64(relocate("STACK_local_3d8"))?).wrapping_add(0x18u64);
                pc = 25;
            }
            27 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 26;
            }
            28 => {
                return Ok(Some(0x1u64));
            }
            29 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_3d8"))?) == (0x0u64)))
                    != 0
                {
                    28
                } else {
                    27
                };
            }
            30 => {
                {
                    let call_arguments = [relocate("STACK_local_2a8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_2e0"))?)?)
                            .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(relocate("STACK_local_2d8"))?)) as i8 as i64
                            as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    32
                } else {
                    30
                };
            }
            34 => {
                pc = 3;
            }
            35 => {
                return Ok(Some(0x0u64));
            }
            36 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((((values[8])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                return Ok(Some(0x0u64));
            }
            38 => {
                {
                    let call_arguments = [values[11], 0x0u64, values[20]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                values[11] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    39
                } else {
                    36
                };
            }
            41 => {
                values[20] = memory.read_u64(relocate("STACK_local_3d8"))?;
                pc = 40;
            }
            42 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 41;
            }
            43 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 42;
            }
            44 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 43;
            }
            45 => {
                values[1] = (memory.read_u64(relocate("STACK_local_3d8"))?).wrapping_add(0x18u64);
                pc = 44;
            }
            46 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 45;
            }
            47 => {
                return Ok(Some(0x0u64));
            }
            48 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_3d8"))?) == (0x0u64)))
                    != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                {
                    let call_arguments = [relocate("STACK_local_2a8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_2e0"))?)?)
                            .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(relocate("STACK_local_2d8"))?)) as i8 as i64
                            as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    51
                } else {
                    49
                };
            }
            53 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_14006f5d0", &call_arguments, memory)
                };
                pc = 52;
            }
            54 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16((values[0]).wrapping_add((0x338u64).wrapping_mul(1)))?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    53
                } else {
                    33
                };
            }
            55 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory
                                .read_u64((values[0]).wrapping_add((0x40u64).wrapping_mul(1)))?)
                            .wrapping_add(0x40u64),
                        )?,
                        0x111u64,
                        0xc35cu64,
                        0x0u64,
                    ];
                    invoke("PostMessageW", &call_arguments, memory)
                };
                pc = 54;
            }
            56 => {
                {
                    let call_arguments = [values[19]];
                    invoke("SysFreeString", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                pc = if (u64::from((values[19]) != (0x0u64))) != 0 {
                    56
                } else {
                    55
                };
            }
            58 => {
                values[19] = values[24];
                pc = 57;
            }
            59 => {
                {
                    let call_arguments = [
                        values[2],
                        memory.read_u64(
                            (relocate("STACK_local_3b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                    ];
                    invoke("FUN_140024fc0", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                pc = if (u64::from((values[2]) != (relocate("STACK_local_3b8")))) != 0 {
                    59
                } else {
                    57
                };
            }
            61 => {
                values[19] = memory.read_u64(
                    (relocate("STACK_local_3b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                pc = 60;
            }
            62 => {
                values[2] = (values[23]).wrapping_add(0x18u64);
                pc = 61;
            }
            63 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_3d0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            64 => {
                values[7] = ({
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr")];
                    invoke("FUN_140032520", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 63;
            }
            65 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_3b8")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) == (0x0u64),
                )) != 0
                {
                    64
                } else {
                    62
                };
            }
            66 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_368")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[22]) & (0xffffffffu64)))
                        | (0x2000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                {
                    let destination =
                        (relocate("STACK_local_3b8")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = {
                        let call_arguments = [memory
                            .read_u64((values[0]).wrapping_add((0x368u64).wrapping_mul(1)))?];
                        invoke("SysAllocString", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 66;
            }
            68 => {
                values[23] = memory.read_u64(
                    (memory.read_u64((values[0]).wrapping_add((0x40u64).wrapping_mul(1)))?)
                        .wrapping_add(0x1d8u64),
                )?;
                pc = 67;
            }
            69 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory.read_u8((values[0]).wrapping_add((0x360u64).wrapping_mul(1)))?,
                        )) as u8,
                    )) as i32 as i64 as u64) as i64)
                        != (((u64::from((0x0u64) as u8)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    68
                } else {
                    54
                };
            }
            70 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_2e0"))?)?)
                            .wrapping_add(0x10u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [];
                    invoke("std::_Xbad_function_call", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xbad_function_call"));
            }
            72 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2e0"))?) == (0x0u64)))
                    != 0
                {
                    71
                } else {
                    70
                };
            }
            73 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_2d8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    72
                } else {
                    69
                };
            }
            74 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x158u64))?,
                        values[0],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                pc = if (values[5]) != 0 { 74 } else { 73 };
            }
            76 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add((0x40u64).wrapping_mul(1)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    75
                } else {
                    54
                };
            }
            77 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_14006fa00", &call_arguments, memory)
                };
                pc = 76;
            }
            78 => {
                values[5] = 1u64;
                pc = 77;
            }
            79 => {
                pc = 107;
            }
            80 => {
                pc = if (u64::from((values[11]) != (0xffffffffffffffffu64))) != 0 {
                    79
                } else {
                    78
                };
            }
            81 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 80;
            }
            82 => {
                {
                    let call_arguments = [relocate("STACK_local_368")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 81;
            }
            83 => {
                values[11] = {
                    let call_arguments = [memory.read_u64(values[3])?, relocate("STACK_local_288")];
                    invoke("FindFirstFileW", &call_arguments, memory)
                };
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [values[3], relocate("STACK_local_368")];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 83;
            }
            85 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((values[5]) != 0)
                            && (({
                                let _ = {
                                    values[8] = ({
                                        let call_arguments = [
                                            memory.read_u64(
                                                (memory.read_u64(values[19])?)
                                                    .wrapping_add(0xd8u64),
                                            )?,
                                            values[19],
                                            relocate("STACK_local_368"),
                                            0x4u64,
                                        ];
                                        invoke("INDIRECT_CALL", &call_arguments, memory)
                                    }) as i32 as i64
                                        as u64;
                                    values[8]
                                };
                                u64::from(
                                    (((values[8]) as i32 as i64 as u64) as i64)
                                        != (((0x0u64) as i32 as i64 as u64) as i64),
                                )
                            }) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                values[5] = u64::from(
                                    ({
                                        let call_arguments = [relocate("STACK_local_368")];
                                        invoke(
                                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                            &call_arguments,
                                            memory,
                                        )
                                    }) != 0,
                                );
                                values[5]
                            };
                            u64::from(!((values[5]) != 0))
                        }) != 0),
                )) != 0
                {
                    84
                } else {
                    83
                };
            }
            86 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [relocate("STACK_local_368")];
                        invoke(
                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 85;
            }
            87 => {
                {
                    let call_arguments = [relocate("STACK_local_2c8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 86;
            }
            88 => {
                {
                    let call_arguments = [relocate("STACK_local_368"), values[10]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 87;
            }
            89 => {
                values[10] = memory.read_u64(values[10])?;
                pc = 88;
            }
            90 => {
                pc = if (u64::from(
                    (0x7u64)
                        < (memory.read_u64((values[10]).wrapping_add((0xcu64).wrapping_mul(2)))?),
                )) != 0
                {
                    89
                } else {
                    88
                };
            }
            91 => {
                values[10] = {
                    let call_arguments = [relocate("STACK_local_360"), relocate("STACK_local_2c8")];
                    invoke("FUN_140019ef0", &call_arguments, memory)
                };
                pc = 90;
            }
            92 => {
                values[22] = u64::from((u64::from(((values[21]) | (0xd800u64)) as u32)) as u32);
                pc = 91;
            }
            93 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 92;
            }
            94 => {
                {
                    let destination = relocate("STACK_pwStack_348");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 93;
            }
            95 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory.read_u64(values[3])?)
                                .wrapping_add((values[23]).wrapping_mul(0x2u64)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    99
                } else {
                    97
                };
            }
            99 => {
                values[23] = (values[23]).wrapping_add(0x1u64);
                pc = 98;
            }
            100 => {
                values[23] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 99;
            }
            101 => {
                {
                    let call_arguments = [
                        values[3],
                        (relocate("STACK_local_288")).wrapping_add((0x2cu64).wrapping_mul(1)),
                    ];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 78;
            }
            102 => {
                {
                    let call_arguments = [
                        values[3],
                        ((values[8]).wrapping_add(0x1u64)) as i32 as i64 as u64,
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::ReleaseBuffer",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 101;
            }
            103 => {
                {
                    let call_arguments = [
                        values[3],
                        ((values[8]).wrapping_add(0x1u64)) as i32 as i64 as u64,
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetBuffer",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 102;
            }
            104 => {
                values[8] = ({
                    let call_arguments = [values[3], 0x3au64];
                    invoke("ATL::CStringT<>::ReverseFind", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 103;
            }
            105 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64
                            as u64) as i64),
                )) != 0
                {
                    104
                } else {
                    103
                };
            }
            106 => {
                values[8] = ({
                    let call_arguments = [values[3], 0x5cu64];
                    invoke("ATL::CStringT<>::ReverseFind", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 105;
            }
            107 => {
                {
                    let call_arguments = [values[11]];
                    invoke("FindClose", &call_arguments, memory)
                };
                pc = 106;
            }
            108 => {
                pc = if (u64::from((values[11]) == (0xffffffffffffffffu64))) != 0 {
                    100
                } else {
                    107
                };
            }
            109 => {
                values[11] = {
                    let call_arguments = [memory.read_u64(values[3])?, relocate("STACK_local_288")];
                    invoke("FindFirstFileW", &call_arguments, memory)
                };
                pc = 108;
            }
            110 => {
                pc = if (u64::from(!((values[6]) != 0))) != 0 {
                    109
                } else {
                    77
                };
            }
            111 => {
                values[6] = u64::from(
                    ({
                        let call_arguments = [values[3]];
                        invoke(
                            "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                            &call_arguments,
                            memory,
                        )
                    }) != 0,
                );
                pc = 110;
            }
            112 => {
                values[3] = (values[0]).wrapping_add((0x340u64).wrapping_mul(1));
                pc = 111;
            }
            113 => {
                values[5] = 0u64;
                pc = 112;
            }
            114 => {
                pc = 39;
            }
            115 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            116 => {
                return Ok(Some(0x0u64));
            }
            117 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((((values[8])
                            .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                            as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0
                {
                    116
                } else {
                    115
                };
            }
            118 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    117
                } else {
                    114
                };
            }
            119 => {
                values[20] = memory.read_u64(relocate("STACK_local_3d8"))?;
                pc = 118;
            }
            120 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 119;
            }
            121 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 120;
            }
            122 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 121;
            }
            123 => {
                values[1] = (memory.read_u64(relocate("STACK_local_3d8"))?).wrapping_add(0x18u64);
                pc = 122;
            }
            124 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 123;
            }
            125 => {
                return Ok(Some(0x0u64));
            }
            126 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_3d8"))?) == (0x0u64)))
                    != 0
                {
                    125
                } else {
                    124
                };
            }
            127 => {
                {
                    let call_arguments = [relocate("STACK_local_2a8")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 126;
            }
            128 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 127;
            }
            129 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_2e0"))?)?)
                            .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 128;
            }
            130 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(relocate("STACK_local_2d8"))?)) as i8 as i64
                            as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    129
                } else {
                    127
                };
            }
            131 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((u64::from(
                            (u64::from(
                                memory.read_u8(
                                    (values[0]).wrapping_add((0x360u64).wrapping_mul(1)),
                                )?,
                            )) as u8,
                        )) as i32 as i64 as u64) as i64)
                            != (((u64::from((0x0u64) as u8)) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((u64::from(
                                (({
                                    let _ = {
                                        values[5] = u64::from(
                                            ({
                                                let call_arguments = [(values[0])
                                                    .wrapping_add((0x370u64).wrapping_mul(1))];
                                                invoke(
                                                    "ATL::CSimpleStringT<wchar_t,1>::IsEmpty",
                                                    &call_arguments,
                                                    memory,
                                                )
                                            }) != 0,
                                        );
                                        values[5]
                                    };
                                    u64::from(
                                        ((values[5]) != 0)
                                            || (({
                                                let _ = {
                                                    values[5] = u64::from(
                                                        ({
                                                            let call_arguments = [(values[0])
                                                                .wrapping_add(
                                                                    (0x368u64).wrapping_mul(1),
                                                                )];
                                                            invoke("ATL::CSimpleStringT<wchar_t,1>::IsEmpty",&call_arguments,memory)
                                                        }) != 0,
                                                    );
                                                    values[5]
                                                };
                                                values[5]
                                            }) != 0),
                                    )
                                }) != 0)
                                    || (({
                                        let _ = {
                                            values[9] = ({
                                                let call_arguments = [
                                                    memory.read_u64((values[0]).wrapping_add(
                                                        (0x370u64).wrapping_mul(1),
                                                    ))?,
                                                    relocate("STACK_local_2c8"),
                                                ];
                                                invoke("CLSIDFromString", &call_arguments, memory)
                                            })
                                                as i32
                                                as i64
                                                as u64;
                                            values[9]
                                        };
                                        u64::from(
                                            (((values[9]) as i32 as i64 as u64) as i64)
                                                != (((0x0u64) as i32 as i64 as u64) as i64),
                                        )
                                    }) != 0),
                            )) != 0)
                                || (({
                                    let _ = {
                                        values[8] = ({
                                            let call_arguments = [
                                                relocate("STACK_local_2c8"),
                                                relocate("DAT_1400e4cc0"),
                                                0x10u64,
                                            ];
                                            invoke("memcmp", &call_arguments, memory)
                                        })
                                            as i32
                                            as i64
                                            as u64;
                                        values[8]
                                    };
                                    u64::from(
                                        (((values[8]) as i32 as i64 as u64) as i64)
                                            != (((0x0u64) as i32 as i64 as u64) as i64),
                                    )
                                }) != 0),
                        )) != 0),
                )) != 0
                {
                    130
                } else {
                    113
                };
            }
            132 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x288u64).wrapping_mul(1)))?
                    ];
                    invoke("RecoveryHandler::StartRecovery", &call_arguments, memory)
                };
                pc = 131;
            }
            133 => {
                {
                    let call_arguments = [values[11], 0x0u64, values[20]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 132;
            }
            134 => {
                values[11] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 133;
            }
            135 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            136 => {
                pc = if (u64::from(
                    (((((values[8]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    135
                } else {
                    132
                };
            }
            137 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    134
                } else {
                    136
                };
            }
            138 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 137;
            }
            139 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 138;
            }
            140 => {
                values[8] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 139;
            }
            141 => {
                values[1] = (memory.read_u64(relocate("STACK_local_368"))?).wrapping_add(0x18u64);
                pc = 140;
            }
            142 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 141;
            }
            143 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_368"))?) != (0x0u64)))
                    != 0
                {
                    142
                } else {
                    132
                };
            }
            144 => {
                values[20] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 143;
            }
            145 => {
                {
                    let call_arguments = [
                        values[17],
                        relocate("STACK_local_368"),
                        relocate("STACK_local_360"),
                    ];
                    invoke(
                        "Paint::Telemetry::TraceLogger::SendTraceEvent",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 144;
            }
            146 => {
                {
                    let call_arguments = [relocate("STACK_local_368"), relocate("STACK_local_3d0")];
                    invoke("FUN_140005310", &call_arguments, memory)
                };
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = values[0];
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_local_3c8");
                    let stored = 0x8u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = relocate("L\"Recovery\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                {
                    let destination = relocate("STACK_local_328");
                    let stored = relocate("STACK_local_360");
                    memory.write_u64(destination, stored)?;
                }
                pc = 149;
            }
            151 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 150;
            }
            152 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = relocate("STACK_local_360");
                    memory.write_u64(destination, stored)?;
                }
                pc = 151;
            }
            153 => {
                values[17] = {
                    let call_arguments = [];
                    invoke(
                        "Paint::Telemetry::TraceLogger::GetInstance",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 152;
            }
            154 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_378"))?)?)
                            .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 131;
            }
            155 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u8(relocate("STACK_local_370"))?)) as i8 as i64
                            as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            (memory.read_u64(relocate("STACK_local_378"))?) != (0x0u64),
                        )) != 0),
                )) != 0
                {
                    154
                } else {
                    131
                };
            }
            156 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 155;
            }
            157 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 156;
            }
            158 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_2e0"))?)?)
                            .wrapping_add(0x20u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 157;
            }
            159 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64)))
                    != 0
                {
                    158
                } else {
                    156
                };
            }
            160 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_2d8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    159
                } else {
                    155
                };
            }
            161 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = (0x1u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 155;
            }
            162 => {
                {
                    let call_arguments = [relocate("STACK_local_318")];
                    invoke("FUN_14007b800", &call_arguments, memory)
                };
                pc = 161;
            }
            163 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 162;
            }
            164 => {
                {
                    let call_arguments = [relocate("STACK_local_318")];
                    invoke("FUN_14007b800", &call_arguments, memory)
                };
                pc = 155;
            }
            165 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 164;
            }
            166 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_2e0"))?)?)
                                .wrapping_add(0x20u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_2e0"))?,
                        u64::from(
                            (memory.read_u64(relocate("STACK_local_2e0"))?)
                                != (relocate("STACK_local_318")),
                        ),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 165;
            }
            167 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64)))
                    != 0
                {
                    166
                } else {
                    164
                };
            }
            168 => {
                pc = if (u64::from((relocate("STACK_local_318")) != (values[18]))) != 0 {
                    167
                } else {
                    155
                };
            }
            169 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u8(relocate("STACK_local_2d8"))?)) as i8 as i64
                        as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    163
                } else {
                    168
                };
            }
            170 => {
                pc = if (u64::from(
                    ((((memory.read_u64((values[18]).wrapping_add((0x8u64).wrapping_mul(8)))?) as i8
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    160
                } else {
                    169
                };
            }
            171 => {
                values[18] = {
                    let call_arguments = [values[0]];
                    invoke("FUN_14006b770", &call_arguments, memory)
                };
                pc = 170;
            }
            172 => {
                pc = if (values[5]) != 0 { 153 } else { 171 };
            }
            173 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 172;
            }
            174 => {
                values[5] = u64::from(
                    ({
                        let call_arguments = [values[14], relocate("STACK_local_360")];
                        invoke("RestartHandler::IsRestart", &call_arguments, memory)
                    }) != 0,
                );
                pc = 173;
            }
            175 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 174;
            }
            176 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory
                                .read_u64((values[0]).wrapping_add((0x90u64).wrapping_mul(1)))?)
                            .wrapping_add((values[23]).wrapping_mul(0x2u64)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    177
                } else {
                    175
                };
            }
            177 => {
                values[23] = (values[23]).wrapping_add(0x1u64);
                pc = 176;
            }
            178 => {
                values[23] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 177;
            }
            179 => {
                {
                    let destination = relocate("STACK_pwStack_348");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 178;
            }
            180 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 179;
            }
            181 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 180;
            }
            182 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 181;
            }
            183 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x280u64).wrapping_mul(1)))?;
                pc = 182;
            }
            184 => {
                {
                    let destination = relocate("STACK_local_2d8");
                    let stored = (0x0u64) as i8 as i64 as u64;
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 183;
            }
            185 => {
                {
                    let destination = (values[0]).wrapping_add((0x6e8u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 184;
            }
            186 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    185
                } else {
                    184
                };
            }
            187 => {
                values[8] = ({
                    let call_arguments = [
                        values[25],
                        (values[0]).wrapping_add((0x6e8u64).wrapping_mul(1)),
                    ];
                    invoke("FUN_140010af0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 186;
            }
            188 => {
                values[25] = u64::from(
                    ({
                        let call_arguments = [values[0], 0x0u64];
                        invoke("FUN_140075fe0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 187;
            }
            189 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x288u64).wrapping_mul(1)))?,
                        memory.read_u64((values[0]).wrapping_add((0x290u64).wrapping_mul(1)))?,
                    ];
                    invoke(
                        "RecoveryHandler::SetRecoveryCallback",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 188;
            }
            190 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[4])?)?,
                        values[4],
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 189;
            }
            191 => {
                pc = if (u64::from((values[4]) != (0x0u64))) != 0 {
                    190
                } else {
                    189
                };
            }
            192 => {
                {
                    let destination = (values[0]).wrapping_add((0x290u64).wrapping_mul(1));
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 191;
            }
            193 => {
                values[4] =
                    memory.read_u64((values[0]).wrapping_add((0x290u64).wrapping_mul(1)))?;
                pc = 192;
            }
            194 => {
                values[22] = u64::from((u64::from(((values[21]) | (0x1800u64)) as u32)) as u32);
                pc = 193;
            }
            195 => {
                values[13] = values[16];
                pc = 194;
            }
            196 => {
                {
                    let destination = (values[16]).wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 195;
            }
            197 => {
                {
                    let destination = (values[16]).wrapping_add((0x38u64).wrapping_mul(1));
                    let stored = 0x7u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 196;
            }
            198 => {
                {
                    let destination = (values[16]).wrapping_add((0x30u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 197;
            }
            199 => {
                {
                    let destination = (values[16]).wrapping_add((0x28u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 198;
            }
            200 => {
                {
                    let destination = (values[16]).wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 199;
            }
            201 => {
                {
                    let call_arguments = [(values[16]).wrapping_add((0x18u64).wrapping_mul(1))];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 200;
            }
            202 => {
                {
                    let destination = (values[16]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 201;
            }
            203 => {
                {
                    let destination = values[16];
                    let stored = relocate("PaintRecoveryCallback::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 202;
            }
            204 => {
                {
                    let destination = (values[16]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 203;
            }
            205 => {
                pc = if (u64::from((values[16]) != (0x0u64))) != 0 {
                    204
                } else {
                    194
                };
            }
            206 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = values[16];
                    memory.write_u64(destination, stored)?;
                }
                pc = 205;
            }
            207 => {
                values[13] = values[24];
                pc = 206;
            }
            208 => {
                values[16] = {
                    let call_arguments = [0x40u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 207;
            }
            209 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x280u64).wrapping_mul(1)))?
                    ];
                    invoke("RestartHandler::RegisterRestart", &call_arguments, memory)
                };
                pc = 208;
            }
            210 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x280u64).wrapping_mul(1)))?,
                        relocate("STACK_local_2a8"),
                    ];
                    invoke(
                        "RestartHandler::SetRestartParameter",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 209;
            }
            211 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x288u64).wrapping_mul(1)))?
                    ];
                    invoke("RecoveryHandler::GetRecoveryKey", &call_arguments, memory)
                };
                pc = 210;
            }
            212 => {
                {
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x288u64).wrapping_mul(1)))?
                    ];
                    invoke(
                        "RecoveryHandler::RegisterForRecovery",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 211;
            }
            213 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400794f0", &call_arguments, memory)
                };
                pc = 212;
            }
            214 => {
                pc = if (u64::from(((values[23]) as i64) != ((0x0u64) as i64))) != 0 {
                    213
                } else {
                    212
                };
            }
            215 => {
                {
                    let destination = (values[0]).wrapping_add((0x288u64).wrapping_mul(1));
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 214;
            }
            216 => {
                values[23] =
                    memory.read_u64((values[0]).wrapping_add((0x288u64).wrapping_mul(1)))?;
                pc = 215;
            }
            217 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 216;
            }
            218 => {
                values[21] = u64::from((u64::from(((values[21]) & (0xfffffbffu64)) as u32)) as u32);
                pc = 217;
            }
            219 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((values[21] as u32).wrapping_shr((0xau64) as u32) as u64) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    218
                } else {
                    216
                };
            }
            220 => {
                values[13] = {
                    let call_arguments = [
                        values[15],
                        values[14],
                        relocate("STACK_local_360"),
                        0x5u64,
                        0x12cu64,
                    ];
                    invoke("RecoveryHandler::RecoveryHandler", &call_arguments, memory)
                };
                pc = 219;
            }
            221 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_368")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[22]) & (0xffffffffu64)))
                        | (0x600u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 220;
            }
            222 => {
                values[21] = u64::from((u64::from(((values[22]) | (0x600u64)) as u32)) as u32);
                pc = 221;
            }
            223 => {
                {
                    let call_arguments =
                        [relocate("STACK_local_360"), relocate("L\"Paint\""), 0x5u64];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 222;
            }
            224 => {
                {
                    let destination = relocate("STACK_pwStack_348");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 223;
            }
            225 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 224;
            }
            226 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 225;
            }
            227 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 226;
            }
            228 => {
                {
                    let destination = (values[15]).wrapping_add((0x40u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 227;
            }
            229 => {
                {
                    let destination = (values[15]).wrapping_add((0x38u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 228;
            }
            230 => {
                {
                    let destination = (values[15]).wrapping_add((0x30u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 229;
            }
            231 => {
                {
                    let destination = (values[15]).wrapping_add((0x28u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 230;
            }
            232 => {
                {
                    let destination = (values[15]).wrapping_add((0x20u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 231;
            }
            233 => {
                {
                    let destination = (values[15]).wrapping_add((0x18u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 232;
            }
            234 => {
                {
                    let destination = (values[15]).wrapping_add((0x10u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 233;
            }
            235 => {
                {
                    let destination = (values[15]).wrapping_add((0x8u64).wrapping_mul(1));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 234;
            }
            236 => {
                {
                    let destination = values[15];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 235;
            }
            237 => {
                pc = if (u64::from((values[15]) != (0x0u64))) != 0 {
                    236
                } else {
                    219
                };
            }
            238 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = values[15];
                    memory.write_u64(destination, stored)?;
                }
                pc = 237;
            }
            239 => {
                values[13] = values[24];
                pc = 238;
            }
            240 => {
                values[15] = {
                    let call_arguments = [0x48u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 239;
            }
            241 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x280u64).wrapping_mul(1)))?;
                pc = 240;
            }
            242 => {
                {
                    let call_arguments = [values[23], 0x40u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 241;
            }
            243 => {
                {
                    let call_arguments = [values[23]];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 242;
            }
            244 => {
                {
                    let call_arguments = [(values[23]).wrapping_add(0x20u64)];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 243;
            }
            245 => {
                pc = if (u64::from(((values[23]) as i64) != ((0x0u64) as i64))) != 0 {
                    244
                } else {
                    241
                };
            }
            246 => {
                {
                    let destination = (values[0]).wrapping_add((0x280u64).wrapping_mul(1));
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 245;
            }
            247 => {
                values[23] =
                    memory.read_u64((values[0]).wrapping_add((0x280u64).wrapping_mul(1)))?;
                pc = 246;
            }
            248 => {
                values[21] = u64::from((u64::from(((values[22]) | (0x200u64)) as u32)) as u32);
                pc = 247;
            }
            249 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 248;
            }
            250 => {
                values[22] = u64::from((u64::from(((values[22]) & (0xfffffeffu64)) as u32)) as u32);
                pc = 249;
            }
            251 => {
                pc = if (u64::from(
                    (u64::from(
                        (u64::from(
                            ((values[22] as u32).wrapping_shr((0x8u64) as u32) as u64) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    250
                } else {
                    248
                };
            }
            252 => {
                values[13] = {
                    let call_arguments = [values[14], relocate("STACK_local_360")];
                    invoke("RestartHandler::RestartHandler", &call_arguments, memory)
                };
                pc = 251;
            }
            253 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_368")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[21]) & (0xffffffffu64)))
                        | (0x110u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 252;
            }
            254 => {
                values[22] = u64::from((u64::from(((values[21]) | (0x110u64)) as u32)) as u32);
                pc = 253;
            }
            255 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_360"),
                        relocate("L\"/restart\""),
                        0x8u64,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 254;
            }
            256 => {
                {
                    let destination = relocate("STACK_pwStack_348");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 255;
            }
            257 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 256;
            }
            258 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 257;
            }
            259 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 258;
            }
            260 => {
                pc = if (u64::from((values[14]) != (0x0u64))) != 0 {
                    259
                } else {
                    251
                };
            }
            261 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 260;
            }
            262 => {
                values[13] = values[24];
                pc = 261;
            }
            263 => {
                values[14] = {
                    let call_arguments = [0x40u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 262;
            }
            264 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    263
                } else {
                    16
                };
            }
            265 => {
                pc = 16;
            }
            266 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    265
                } else {
                    15
                };
            }
            267 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory.read_u8((values[0]).wrapping_add((0x33au64).wrapping_mul(1)))?,
                        )) as u8,
                    )) as i32 as i64 as u64) as i64)
                        == (((u64::from((0x0u64) as u8)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    264
                } else {
                    266
                };
            }
            268 => {
                values[8] = ({
                    let call_arguments = [values[0]];
                    invoke("CWinApp::RunAutomated", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 267;
            }
            269 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory.read_u8((values[0]).wrapping_add((0x33au64).wrapping_mul(1)))?,
                        )) as u8,
                    )) as i32 as i64 as u64) as i64)
                        == (((u64::from((0x0u64) as u8)) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    268
                } else {
                    267
                };
            }
            270 => {
                {
                    let call_arguments = [];
                    invoke("TraceLaunchWithEmbedding", &call_arguments, memory)
                };
                pc = 269;
            }
            271 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    270
                } else {
                    269
                };
            }
            272 => {
                {
                    let destination = (values[0]).wrapping_add((0x33au64).wrapping_mul(1));
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(
                                (((values[7]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )) as u8,
                        )) as u8,
                    );
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 271;
            }
            273 => {
                values[7] = ({
                    let call_arguments = [values[0]];
                    invoke("CWinApp::RunEmbedded", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 272;
            }
            274 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x2a8u64).wrapping_mul(1)),
                        relocate("DAT_140106808"),
                        values[19],
                        0x1u64,
                        0x0u64,
                    ];
                    invoke(
                        "COleTemplateServer::ConnectTemplate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 273;
            }
            275 => {
                {
                    let call_arguments = [values[0], values[19]];
                    invoke("CWinApp::AddDocTemplate", &call_arguments, memory)
                };
                pc = 274;
            }
            276 => {
                {
                    let call_arguments = [values[19], 0x5u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("CDocTemplate::SetServerInfo", &call_arguments, memory)
                };
                pc = 275;
            }
            277 => {
                values[19] = values[13];
                pc = 276;
            }
            278 => {
                {
                    let destination = values[13];
                    let stored = relocate("PaintSingleDocTemplate::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 277;
            }
            279 => {
                {
                    let call_arguments = [
                        values[13],
                        0x2u64,
                        relocate("PTR_s_CPBDoc_140115050"),
                        relocate("PTR_s_CPBFrame_1401158b0"),
                        relocate("PTR_s_CPBView_1401193a0"),
                    ];
                    invoke(
                        "CSingleDocTemplate::CSingleDocTemplate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 278;
            }
            280 => {
                {
                    let call_arguments = [values[13], 0x0u64, 0xf8u64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 279;
            }
            281 => {
                pc = if (u64::from((values[13]) != (0x0u64))) != 0 {
                    280
                } else {
                    276
                };
            }
            282 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = values[13];
                    memory.write_u64(destination, stored)?;
                }
                pc = 281;
            }
            283 => {
                values[19] = values[24];
                pc = 282;
            }
            284 => {
                values[13] = {
                    let call_arguments = [0xf8u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 283;
            }
            285 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_14008ba00", &call_arguments, memory)
                };
                pc = 284;
            }
            286 => {
                {
                    let call_arguments = [values[0]];
                    invoke("FUN_1400752f0", &call_arguments, memory)
                };
                pc = 285;
            }
            287 => {
                {
                    let destination = (memory
                        .read_u64((values[12]).wrapping_add((0x80u64).wrapping_mul(1)))?)
                    .wrapping_add(0x48u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 286;
            }
            288 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[12]).wrapping_add((0x80u64).wrapping_mul(1)))?)
                        as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    287
                } else {
                    286
                };
            }
            289 => {
                values[12] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 288;
            }
            290 => {
                {
                    let call_arguments = [];
                    invoke("terminate", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("terminate"));
            }
            291 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    290
                } else {
                    289
                };
            }
            292 => {
                values[7] = ({
                    let call_arguments = [];
                    invoke("AfxOleInit", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 291;
            }
            293 => {
                {
                    let call_arguments = [values[11], 0x0u64, values[20]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 292;
            }
            294 => {
                values[11] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 293;
            }
            295 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            296 => {
                pc = if (u64::from(
                    (((((values[7]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    295
                } else {
                    292
                };
            }
            297 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    294
                } else {
                    296
                };
            }
            298 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 297;
            }
            299 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 298;
            }
            300 => {
                values[7] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 299;
            }
            301 => {
                values[1] = (memory.read_u64(relocate("STACK_local_368"))?).wrapping_add(0x18u64);
                pc = 300;
            }
            302 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 301;
            }
            303 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_368"))?) != (0x0u64)))
                    != 0
                {
                    302
                } else {
                    292
                };
            }
            304 => {
                values[20] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 303;
            }
            305 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x6a0u64).wrapping_mul(1))];
                    invoke("FUN_140001d80", &call_arguments, memory)
                };
                pc = 304;
            }
            306 => {
                values[22] = u64::from((u64::from(((values[21]) | (0x10u64)) as u32)) as u32);
                pc = 305;
            }
            307 => {
                values[21] = u64::from((u64::from(((values[21]) | (0x21u64)) as u32)) as u32);
                pc = 306;
            }
            308 => {
                {
                    let call_arguments = [0x0u64, relocate("STACK_local_368")];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 307;
            }
            309 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[23];
                    memory.write_u64(destination, stored)?;
                }
                pc = 308;
            }
            310 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 309;
            }
            311 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 310;
            }
            312 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 306;
            }
            313 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 312;
            }
            314 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 313;
            }
            315 => {
                values[21] = u64::from((u64::from(((values[21]) | (0xe1u64)) as u32)) as u32);
                pc = 314;
            }
            316 => {
                {
                    let call_arguments = [values[7], relocate("STACK_local_3d0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            317 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    316
                } else {
                    315
                };
            }
            318 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_360"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 317;
            }
            319 => {
                {
                    let destination = relocate("STACK_local_3c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 318;
            }
            320 => {
                {
                    let destination = relocate("STACK_local_3c8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 319;
            }
            321 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_3d0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 320;
            }
            322 => {
                {
                    let destination = (relocate("STACK_local_3d0")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_3d0"))?)
                                .wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 321;
            }
            323 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 322;
            }
            324 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    311
                } else {
                    323
                };
            }
            325 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 324;
            }
            326 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 325;
            }
            327 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 326;
            }
            328 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 327;
            }
            329 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = relocate("STACK_local_360");
                    memory.write_u64(destination, stored)?;
                }
                pc = 328;
            }
            330 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = relocate("STACK_pCStack_358");
                    memory.write_u64(destination, stored)?;
                }
                pc = 329;
            }
            331 => {
                {
                    let destination = relocate("STACK_pwStack_348");
                    let stored = relocate("L\"LowMemoryError\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 330;
            }
            332 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = 0xe00000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 331;
            }
            333 => {
                values[23] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 332;
            }
            334 => {
                {
                    let call_arguments = [values[11], 0x0u64, values[20]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 333;
            }
            335 => {
                values[11] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 334;
            }
            336 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            337 => {
                pc = if (u64::from(
                    (((((values[7]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    336
                } else {
                    333
                };
            }
            338 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    335
                } else {
                    337
                };
            }
            339 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 338;
            }
            340 => {
                {
                    let destination = values[1];
                    let stored = ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 339;
            }
            341 => {
                values[7] = ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 340;
            }
            342 => {
                values[1] = (memory.read_u64(relocate("STACK_local_368"))?).wrapping_add(0x18u64);
                pc = 341;
            }
            343 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 342;
            }
            344 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_368"))?) != (0x0u64)))
                    != 0
                {
                    343
                } else {
                    333
                };
            }
            345 => {
                values[20] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 344;
            }
            346 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x698u64).wrapping_mul(1))];
                    invoke("FUN_140001d80", &call_arguments, memory)
                };
                pc = 345;
            }
            347 => {
                values[21] = u64::from((0x2u64) as u32);
                pc = 346;
            }
            348 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_368"),
                        relocate("STACK_local_3d0"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 347;
            }
            349 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[23];
                    memory.write_u64(destination, stored)?;
                }
                pc = 348;
            }
            350 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 349;
            }
            351 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 350;
            }
            352 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 346;
            }
            353 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 352;
            }
            354 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 353;
            }
            355 => {
                values[21] = u64::from((0xeu64) as u32);
                pc = 354;
            }
            356 => {
                pc = 63;
            }
            357 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    356
                } else {
                    355
                };
            }
            358 => {
                values[7] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_360"))?,
                        relocate("STACK_local_368"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 357;
            }
            359 => {
                {
                    let destination = relocate("STACK_local_3c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 358;
            }
            360 => {
                {
                    let destination = relocate("STACK_local_3c8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 359;
            }
            361 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_3d0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 360;
            }
            362 => {
                {
                    let destination = (relocate("STACK_local_3d0")).wrapping_add(0x4u64);
                    let stored = u64::from(
                        (u64::from(
                            ((memory.read_u64(relocate("STACK_local_3d0"))?)
                                .wrapping_shr((0x20u64) as u32)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 361;
            }
            363 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 362;
            }
            364 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    351
                } else {
                    363
                };
            }
            365 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 364;
            }
            366 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 365;
            }
            367 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 366;
            }
            368 => {
                {
                    let destination = ((relocate("STACK_local_2c8"))
                        .wrapping_add((0x0u64).wrapping_mul(16)))
                    .wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 367;
            }
            369 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = relocate("STACK_local_360");
                    memory.write_u64(destination, stored)?;
                }
                pc = 368;
            }
            370 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = relocate("STACK_pCStack_358");
                    memory.write_u64(destination, stored)?;
                }
                pc = 369;
            }
            371 => {
                {
                    let destination = relocate("STACK_pwStack_348");
                    let stored = relocate("L\"NoMemoryError\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 370;
            }
            372 => {
                {
                    let destination = relocate("STACK_pCStack_358");
                    let stored = 0xd00000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 371;
            }
            373 => {
                values[23] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 372;
            }
            374 => {
                {
                    let call_arguments = [0x8001u64];
                    invoke("SetErrorMode", &call_arguments, memory)
                };
                pc = 373;
            }
            375 => {
                {
                    let call_arguments = [];
                    invoke("PGS::PGSInitialize", &call_arguments, memory)
                };
                pc = 374;
            }
            376 => {
                {
                    let destination = (values[0]).wrapping_add((0xd8u64).wrapping_mul(1));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 375;
            }
            377 => {
                values[10] = {
                    let call_arguments = [relocate("L\"Paint\"")];
                    invoke("_wcsdup", &call_arguments, memory)
                };
                pc = 376;
            }
            378 => {
                {
                    let call_arguments =
                        [memory.read_u64((values[0]).wrapping_add((0xd8u64).wrapping_mul(1)))?];
                    invoke("free", &call_arguments, memory)
                };
                pc = 377;
            }
            379 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0xd8u64).wrapping_mul(1)))?)
                        != (0x0u64),
                )) != 0
                {
                    378
                } else {
                    377
                };
            }
            380 => {
                {
                    let call_arguments = [
                        values[0],
                        relocate("L\"Microsoft\\\\Windows\\\\CurrentVersion\\\\Applets\""),
                    ];
                    invoke("CWinApp::SetRegistryKey", &call_arguments, memory)
                };
                pc = 379;
            }
            381 => {
                {
                    let call_arguments = [relocate("STACK_local_3d8")];
                    invoke("FUN_14006a220", &call_arguments, memory)
                };
                pc = 380;
            }
            382 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add((0x658u64).wrapping_mul(1)),
                        relocate("STACK_local_3d8"),
                    ];
                    invoke("FUN_14004baf0", &call_arguments, memory)
                };
                pc = 381;
            }
            383 => {
                {
                    let call_arguments = [];
                    invoke("TraceLaunch", &call_arguments, memory)
                };
                pc = 382;
            }
            384 => {
                {
                    let call_arguments = [];
                    invoke("PaintTraceLoggingInitialize", &call_arguments, memory)
                };
                pc = 383;
            }
            385 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_368"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 384;
            }
            386 => {
                values[8] = (0x0u64) as i32 as i64 as u64;
                pc = 385;
            }
            387 => {
                values[24] = 0x0u64;
                pc = 386;
            }
            388 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_408"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 387;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140070390` (`mspaint.c:100278`).
pub fn fun_140070390(
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
        (arguments[1]) as i32 as i64 as u64,
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
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
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
    let mut pc: i32 = 588;
    loop {
        match pc {
            0 => {
                pc = 6;
            }
            1 => {
                pc = 149;
            }
            2 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[7]) as i32 as i64 as u64) as i64)
                            == (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || (({
                            let _ = {
                                values[18] = (values[30]).wrapping_add(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                        .wrapping_mul(2),
                                );
                                values[18]
                            };
                            u64::from(
                                (((values[7]) as i32 as i64 as u64) as i64)
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
                values[7] = ((u64::from(memory.read_u16((values[30]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                ))?)) as i16 as i64 as u64) as i16 as i64 as u64;
                pc = 2;
            }
            4 => {
                pc = 149;
            }
            5 => {
                pc = if (u64::from((values[31]) == (values[30]))) != 0 {
                    4
                } else {
                    3
                };
            }
            6 => {
                values[30] = values[18];
                pc = 5;
            }
            7 => {
                pc = if (u64::from((values[31]) != (values[39]))) != 0 {
                    11
                } else {
                    6
                };
            }
            8 => {
                values[31] = (values[31]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 7;
            }
            9 => {
                pc = 6;
            }
            10 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[31])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u16(values[31])?)) as i16 as i64 as u64)
                                as i32 as i64 as u64) as i64)
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
                values[18] = values[39];
                pc = 10;
            }
            12 => {
                return Ok(Some(0x0u64));
            }
            13 => {
                {
                    let call_arguments = [relocate("STACK_local_358")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 12;
            }
            14 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_358"))?) != (0x0u64)))
                    != 0
                {
                    13
                } else {
                    12
                };
            }
            15 => {
                {
                    let call_arguments = [values[27], 0x0u64, values[15]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                values[27] = {
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
                    (((((values[21]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
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
                    (((values[21]) as i32 as i64 as u64) as i64)
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
                    let destination = values[37];
                    let stored = ((((u64::from(memory.read_u32(values[37])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 20;
            }
            22 => {
                values[21] = ((u64::from(memory.read_u32(values[37])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 21;
            }
            23 => {
                values[37] = (values[15]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                pc = if (u64::from((values[15]) != (0x0u64))) != 0 {
                    24
                } else {
                    14
                };
            }
            26 => {
                {
                    let call_arguments = [relocate("STACK_local_2d0")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 25;
            }
            27 => {
                {
                    let call_arguments = [relocate("STACK_local_248")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 26;
            }
            28 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_290"))?];
                    invoke("LocalFree", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_290"))?) != (0x0u64)))
                    != 0
                {
                    28
                } else {
                    27
                };
            }
            30 => {
                {
                    let destination = relocate("STACK_local_328");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 29;
            }
            31 => {
                {
                    let destination = relocate("STACK_puStack_330");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 30;
            }
            32 => {
                {
                    let destination = relocate("STACK_local_338");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            33 => {
                {
                    let call_arguments = [values[41]];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            35 => {
                pc = if (u64::from(
                    ((u64::from(
                        (0xfffu64)
                            < (({
                                let numerator = ((memory.read_u64(relocate("STACK_local_328"))?)
                                    .wrapping_sub(memory.read_u64(relocate("STACK_local_338"))?))
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
                            .wrapping_mul(0x18u64)),
                    )) != 0)
                        && (({
                            let _ = {
                                values[41] = memory.read_u64(
                                    (memory.read_u64(relocate("STACK_local_338"))?).wrapping_add(
                                        (0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64,
                                    ),
                                )?;
                                values[41]
                            };
                            u64::from(
                                ((0x1fu64) as i64)
                                    < ((((memory.read_u64(relocate("STACK_local_338"))?)
                                        .wrapping_sub(values[41]))
                                    .wrapping_sub(0x8u64))
                                        as i64),
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
                values[41] = memory.read_u64(relocate("STACK_local_338"))?;
                pc = 35;
            }
            37 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_338"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    36
                } else {
                    29
                };
            }
            38 => {
                {
                    let destination = relocate("STACK_local_308");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 37;
            }
            39 => {
                {
                    let destination = relocate("STACK_pCStack_310");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 38;
            }
            40 => {
                {
                    let destination = relocate("STACK_local_318");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 39;
            }
            41 => {
                {
                    let call_arguments = [values[25]];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            43 => {
                pc = if (u64::from(
                    ((u64::from(
                        (0xfffu64)
                            < (((memory.read_u64(relocate("STACK_local_308"))?)
                                .wrapping_sub(memory.read_u64(relocate("STACK_local_318"))?))
                                & (0xfffffffffffffff8u64)),
                    )) != 0)
                        && (({
                            let _ = {
                                values[25] = memory.read_u64(
                                    (memory.read_u64(relocate("STACK_local_318"))?).wrapping_add(
                                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                            .wrapping_mul(1),
                                    ),
                                )?;
                                values[25]
                            };
                            u64::from(
                                (0x1fu64)
                                    < ((memory.read_u64(relocate("STACK_local_318"))?)
                                        .wrapping_add(
                                            (((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                                .wrapping_sub(values[25]))
                                            .wrapping_mul(1),
                                        )),
                            )
                        }) != 0),
                )) != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                values[25] = memory.read_u64(relocate("STACK_local_318"))?;
                pc = 43;
            }
            45 => {
                pc = if (u64::from((values[25]) != (values[16]))) != 0 {
                    47
                } else {
                    44
                };
            }
            46 => {
                let _ = {
                    values[25] = (values[25]).wrapping_add((0x8u64).wrapping_mul(1));
                    values[25]
                };
                pc = 45;
            }
            47 => {
                {
                    let call_arguments = [values[25]];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_318"))?) != (0x0u64)))
                    != 0
                {
                    45
                } else {
                    37
                };
            }
            49 => {
                values[25] = memory.read_u64(relocate("STACK_local_318"))?;
                pc = 48;
            }
            50 => {
                values[16] = memory.read_u64(relocate("STACK_pCStack_310"))?;
                pc = 49;
            }
            51 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = relocate("STACK_pCStack_2f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = relocate("STACK_local_300");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let call_arguments = [];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 53;
            }
            55 => {
                {
                    let call_arguments = [0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64];
                    invoke("_invoke_watson", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_invoke_watson"));
            }
            56 => {
                pc = if (u64::from(
                    ((u64::from(
                        (0xfffu64)
                            < (((memory.read_u64(relocate("STACK_local_2f0"))?)
                                .wrapping_sub(memory.read_u64(relocate("STACK_local_300"))?))
                                & (0xfffffffffffffff8u64)),
                    )) != 0)
                        && ((u64::from(
                            (0x1fu64)
                                < ((memory.read_u64(relocate("STACK_local_300"))?).wrapping_add(
                                    (((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64)
                                        .wrapping_sub(
                                            memory.read_u64(
                                                (memory.read_u64(relocate("STACK_local_300"))?)
                                                    .wrapping_add(
                                                        ((0u64.wrapping_sub(0x8u64)) as i32 as i64
                                                            as u64)
                                                            .wrapping_mul(1),
                                                    ),
                                            )?,
                                        ))
                                    .wrapping_mul(1),
                                )),
                        )) != 0),
                )) != 0
                {
                    55
                } else {
                    54
                };
            }
            57 => {
                pc = if (u64::from((values[25]) != (values[16]))) != 0 {
                    59
                } else {
                    56
                };
            }
            58 => {
                let _ = {
                    values[25] = (values[25]).wrapping_add((0x8u64).wrapping_mul(1));
                    values[25]
                };
                pc = 57;
            }
            59 => {
                {
                    let call_arguments = [values[25]];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 58;
            }
            60 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_300"))?) != (0x0u64)))
                    != 0
                {
                    57
                } else {
                    50
                };
            }
            61 => {
                values[25] = memory.read_u64(relocate("STACK_local_300"))?;
                pc = 60;
            }
            62 => {
                values[16] = memory.read_u64(relocate("STACK_pCStack_2f8"))?;
                pc = 61;
            }
            63 => {
                {
                    let call_arguments =
                        [memory.read_u64((memory.read_u64(values[43])?).wrapping_add(0x8u64))?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 62;
            }
            64 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    63
                } else {
                    62
                };
            }
            65 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 64;
            }
            66 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 65;
            }
            67 => {
                values[21] = ((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 66;
            }
            68 => {
                values[5] = (values[43]).wrapping_add(0xcu64);
                pc = 67;
            }
            69 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 68;
            }
            70 => {
                {
                    let call_arguments = [memory.read_u64(
                        memory.read_u64(memory.read_u64(relocate("STACK_local_398"))?)?,
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                pc = if (u64::from(
                    ((((values[41]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    70
                } else {
                    62
                };
            }
            72 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 71;
            }
            73 => {
                {
                    let destination = values[6];
                    let stored = ((((memory.read_u64(values[6])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 72;
            }
            74 => {
                values[41] = memory.read_u64(values[6])?;
                pc = 73;
            }
            75 => {
                values[6] = (memory.read_u64(relocate("STACK_local_398"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 74;
            }
            76 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 75;
            }
            77 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_398"))?) != (0x0u64)))
                    != 0
                {
                    76
                } else {
                    62
                };
            }
            78 => {
                values[43] = memory.read_u64(relocate("STACK_local_398"))?;
                pc = 77;
            }
            79 => {
                {
                    let call_arguments = [relocate("STACK_local_2d8")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 78;
            }
            80 => {
                {
                    let destination = relocate("STACK_local_2b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = relocate("STACK_puStack_2c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = relocate("STACK_local_2c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let call_arguments = [];
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
                    ((u64::from(
                        (0xfffu64)
                            < (((memory.read_u64(relocate("STACK_local_2b8"))?)
                                .wrapping_sub(memory.read_u64(relocate("STACK_local_2c8"))?))
                                & (0xfffffffffffffff0u64)),
                    )) != 0)
                        && ((u64::from(
                            ((0x1fu64) as i64)
                                < ((((memory.read_u64(relocate("STACK_local_2c8"))?).wrapping_sub(
                                    memory.read_u64(
                                        (memory.read_u64(relocate("STACK_local_2c8"))?)
                                            .wrapping_add(
                                                (0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64,
                                            ),
                                    )?,
                                ))
                                .wrapping_sub(0x8u64)) as i64),
                        )) != 0),
                )) != 0
                {
                    84
                } else {
                    83
                };
            }
            86 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_2c8"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    85
                } else {
                    79
                };
            }
            87 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 86;
            }
            88 => {
                {
                    let call_arguments = [relocate("STACK_local_288")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 87;
            }
            89 => {
                {
                    let call_arguments = [relocate("STACK_local_2e0")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 88;
            }
            90 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_2e0"))?) != (0x0u64)))
                    != 0
                {
                    89
                } else {
                    88
                };
            }
            91 => {
                {
                    let call_arguments = [relocate("STACK_local_340")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 90;
            }
            92 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_340"))?) != (0x0u64)))
                    != 0
                {
                    91
                } else {
                    90
                };
            }
            93 => {
                {
                    let call_arguments = [memory.read_u64(relocate("STACK_local_360"))?];
                    invoke("CoTaskMemFree", &call_arguments, memory)
                };
                pc = 92;
            }
            94 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_360"))?) != (0x0u64)))
                    != 0
                {
                    93
                } else {
                    92
                };
            }
            95 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_370"))?;
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 94;
            }
            96 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520")];
                    invoke("FUN_140075fe0", &call_arguments, memory)
                };
                pc = 95;
            }
            97 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520")];
                    invoke("FUN_140075d90", &call_arguments, memory)
                };
                pc = 95;
            }
            98 => {
                {
                    let destination = relocate("_DAT_1401d07ba");
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(relocate("_DAT_1401d07ba"))?)) as u16,
                        ))
                        .wrapping_add(0x1u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 97;
            }
            99 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[21]) as i32 as i64 as u64) as i64)
                            < (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            ((((u64::from(
                                memory.read_u16(
                                    (relocate("STACK_local_238"))
                                        .wrapping_add((0x0u64).wrapping_mul(2)),
                                )?,
                            )) as i16 as i64 as u64) as i32 as i64
                                as u64) as i64)
                                == (((0x0u64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    96
                } else {
                    98
                };
            }
            100 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_358"))?,
                        relocate("STACK_local_238"),
                    ];
                    invoke("FUN_1400101e0", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 99;
            }
            101 => {
                {
                    let call_arguments = [relocate("STACK_local_238"), 0x0u64, 0x1fau64];
                    invoke("memset", &call_arguments, memory)
                };
                pc = 100;
            }
            102 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_3a0"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = memory.read_u64(
                        ((memory.read_u64(relocate("STACK_local_338"))?).wrapping_add(0x10u64))
                            .wrapping_add((values[32]).wrapping_mul(0x18u64)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 101;
            }
            103 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_local_3a0"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 102;
            }
            104 => {
                {
                    let destination = memory.read_u64(relocate("STACK_local_3a0"))?;
                    let stored = memory.read_u64(values[9])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 103;
            }
            105 => {
                values[8] = memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 104;
            }
            106 => {
                values[9] = (memory.read_u64(relocate("STACK_local_338"))?)
                    .wrapping_add((values[32]).wrapping_mul(0x18u64));
                pc = 105;
            }
            107 => {
                pc = if (u64::from(
                    (values[32])
                        < ({
                            let numerator = ((memory.read_u64(relocate("STACK_puStack_330"))?)
                                .wrapping_sub(memory.read_u64(relocate("STACK_local_338"))?))
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
                    106
                } else {
                    101
                };
            }
            108 => {
                values[32] = values[42];
                pc = 107;
            }
            109 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_350")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[42]) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 108;
            }
            110 => {
                values[42] = u64::from(
                    ((((memory.read_u64(relocate("STACK_local_350"))?) as i32 as i64 as u64)
                        .wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32,
                );
                pc = 109;
            }
            111 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x5aeu64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            112 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    111
                } else {
                    110
                };
            }
            113 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        relocate("STACK_local_350"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 112;
            }
            114 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_350"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 113;
            }
            115 => {
                {
                    let call_arguments =
                        [values[25], memory.read_u64(relocate("STACK_local_360"))?];
                    invoke("ATL::CStringT<>::operator=", &call_arguments, memory)
                };
                pc = 114;
            }
            116 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x5a9u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            117 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    116
                } else {
                    115
                };
            }
            118 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_340"))?)?)
                                .wrapping_add(0x28u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_340"))?,
                        0x80058000u64,
                        relocate("STACK_local_360"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 117;
            }
            119 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 118;
            }
            120 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x5a7u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            121 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    120
                } else {
                    119
                };
            }
            122 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0xa0u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        relocate("STACK_local_340"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 121;
            }
            123 => {
                {
                    let destination = relocate("STACK_local_340");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 122;
            }
            124 => {
                pc = if (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < (((values[21]) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    123
                } else {
                    90
                };
            }
            125 => {
                values[21] = ({
                    let call_arguments = [values[10], values[43]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 124;
            }
            126 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[23])?).wrapping_add(0xf8u64))?,
                        values[23],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 125;
            }
            127 => {
                pc = if (u64::from((values[23]) != (0x0u64))) != 0 {
                    126
                } else {
                    125
                };
            }
            128 => {
                values[23] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 127;
            }
            129 => {
                values[10] = memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                        .wrapping_add(0x18u64),
                )?;
                pc = 128;
            }
            130 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x5a3u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            131 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    130
                } else {
                    129
                };
            }
            132 => {
                values[43] = memory.read_u64(relocate("STACK_local_358"))?;
                pc = 131;
            }
            133 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0xe8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        memory.read_u64(relocate("STACK_local_2e0"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 132;
            }
            134 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x5a2u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            135 => {
                pc = if (u64::from(
                    (((values[20]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    134
                } else {
                    133
                };
            }
            136 => {
                values[20] = ({
                    let call_arguments = [
                        relocate("L\"prop:System.Security.EncryptionOwners\""),
                        relocate("DAT_140117168"),
                        relocate("STACK_local_2e0"),
                    ];
                    invoke(
                        "PSGetPropertyDescriptionListFromString",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 135;
            }
            137 => {
                {
                    let destination = relocate("STACK_local_2e0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let call_arguments = [relocate("STACK_local_3d0")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 137;
            }
            139 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x59fu64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            140 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    139
                } else {
                    138
                };
            }
            141 => {
                values[21] = ({
                    let call_arguments = [values[10], values[43], values[34]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 140;
            }
            142 => {
                values[34] = memory.read_u64(relocate("STACK_local_3d0"))?;
                pc = 141;
            }
            143 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_3b8"))?))) != 0
                {
                    142
                } else {
                    141
                };
            }
            144 => {
                values[34] = relocate("STACK_local_3d0");
                pc = 143;
            }
            145 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_3d0"),
                        values[30],
                        ((values[39]).wrapping_sub(values[30]) as i64).wrapping_shr((0x1u64) as u32)
                            as u64,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 144;
            }
            146 => {
                {
                    let destination = relocate("STACK_local_3b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 145;
            }
            147 => {
                {
                    let destination = relocate("STACK_local_3c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 146;
            }
            148 => {
                {
                    let destination = relocate("STACK_uStack_3c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 147;
            }
            149 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 148;
            }
            150 => {
                pc = if (u64::from((values[31]) == (values[39]))) != 0 {
                    149
                } else {
                    11
                };
            }
            151 => {
                values[30] = values[39];
                pc = 150;
            }
            152 => {
                values[31] = {
                    let call_arguments = [values[34], values[39]];
                    invoke("FUN_140019de0", &call_arguments, memory)
                };
                pc = 151;
            }
            153 => {
                values[39] = (values[34]).wrapping_add(
                    (memory.read_u64(relocate("STACK_local_258"))?).wrapping_mul(0x2u64),
                );
                pc = 152;
            }
            154 => {
                values[34] = memory.read_u64(relocate("STACK_local_268"))?;
                pc = 153;
            }
            155 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_250"))?))) != 0
                {
                    154
                } else {
                    153
                };
            }
            156 => {
                values[34] = relocate("STACK_local_268");
                pc = 155;
            }
            157 => {
                values[10] = memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                        .wrapping_add(0x78u64),
                )?;
                pc = 156;
            }
            158 => {
                values[43] = memory.read_u64(relocate("STACK_local_358"))?;
                pc = 157;
            }
            159 => {
                {
                    let call_arguments = [relocate("STACK_local_368")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 158;
            }
            160 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 159;
            }
            161 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_360"))?) != (0x0u64)))
                    != 0
                {
                    160
                } else {
                    159
                };
            }
            162 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x59cu64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            163 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    162
                } else {
                    161
                };
            }
            164 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x58u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        memory.read_u64(relocate("STACK_local_360"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 163;
            }
            165 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x59bu64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            166 => {
                pc = if (u64::from(
                    (((values[20]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    165
                } else {
                    164
                };
            }
            167 => {
                values[20] = ({
                    let call_arguments = [
                        values[33],
                        0x0u64,
                        relocate("DAT_140117178"),
                        relocate("STACK_local_360"),
                    ];
                    invoke("SHCreateItemFromParsingName", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 166;
            }
            168 => {
                values[33] = memory.read_u64(relocate("STACK_local_288"))?;
                pc = 167;
            }
            169 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_270"))?))) != 0
                {
                    168
                } else {
                    167
                };
            }
            170 => {
                values[33] = relocate("STACK_local_288");
                pc = 169;
            }
            171 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 170;
            }
            172 => {
                {
                    let call_arguments = [relocate("STACK_local_2b0")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 171;
            }
            173 => {
                {
                    let call_arguments = [relocate("STACK_local_288")];
                    invoke("FUN_14007c400", &call_arguments, memory)
                };
                pc = 172;
            }
            174 => {
                {
                    let call_arguments = [relocate("STACK_local_2b0")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 173;
            }
            175 => {
                {
                    let destination = relocate("STACK_pwStack_298");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 174;
            }
            176 => {
                {
                    let destination = relocate("STACK_local_2a0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 175;
            }
            177 => {
                {
                    let destination = relocate("STACK_uStack_2a8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 176;
            }
            178 => {
                {
                    let destination = relocate("STACK_local_2b0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 177;
            }
            179 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory.read_u64(relocate("STACK_local_368"))?)
                                .wrapping_add(((values[41]).wrapping_mul(0x2u64)).wrapping_mul(1)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    180
                } else {
                    178
                };
            }
            180 => {
                values[41] = (values[41]).wrapping_add(0x1u64);
                pc = 179;
            }
            181 => {
                values[41] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 180;
            }
            182 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), relocate("STACK_local_368")];
                    invoke("FUN_14006feb0", &call_arguments, memory)
                };
                pc = 181;
            }
            183 => {
                pc = 199;
            }
            184 => {
                values[42] = u64::from(
                    (u64::from(
                        ((u64::from(
                            ((u64::from(
                                ((u64::from(
                                    (memory.read_u64(relocate("STACK_local_2a0"))?) as u32,
                                )) & (0x10u64)) as u32,
                            )) | (0x20u64)) as u32,
                        ) as u32)
                            .wrapping_shr((0x4u64) as u32) as u64) as u32,
                    )) as u32,
                );
                pc = 183;
            }
            185 => {
                pc = if (u64::from(
                    ((u64::from(
                        (u64::from(
                            (u64::from(
                                ((u64::from(
                                    ((u64::from(
                                        (memory.read_u64(relocate("STACK_local_2a0"))?) as u32,
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
                                            (relocate("STACK_local_2a0")).wrapping_add(0x4u64),
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
                                                (relocate("STACK_local_2a0")).wrapping_add(0x4u64),
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
                    184
                } else {
                    182
                };
            }
            186 => {
                pc = 158;
            }
            187 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 186;
            }
            188 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_360"))?) != (0x0u64)))
                    != 0
                {
                    187
                } else {
                    186
                };
            }
            189 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x594u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            190 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    189
                } else {
                    188
                };
            }
            191 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x60u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        memory.read_u64(relocate("STACK_local_360"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 190;
            }
            192 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x593u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            193 => {
                pc = if (u64::from(
                    (((values[20]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    192
                } else {
                    191
                };
            }
            194 => {
                values[20] = ({
                    let call_arguments = [
                        values[33],
                        0x0u64,
                        relocate("DAT_140117178"),
                        relocate("STACK_local_360"),
                    ];
                    invoke("SHCreateItemFromParsingName", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 193;
            }
            195 => {
                values[33] = memory.read_u64(relocate("STACK_local_288"))?;
                pc = 194;
            }
            196 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_270"))?))) != 0
                {
                    195
                } else {
                    194
                };
            }
            197 => {
                values[33] = relocate("STACK_local_288");
                pc = 196;
            }
            198 => {
                {
                    let destination = relocate("STACK_local_360");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 197;
            }
            199 => {
                pc = if (u64::from(
                    (u64::from((values[42]) as u32)) == (u64::from((0x3u64) as u32)),
                )) != 0
                {
                    198
                } else {
                    182
                };
            }
            200 => {
                values[42] = u64::from((0x1u64) as u32);
                pc = 199;
            }
            201 => {
                pc = 199;
            }
            202 => {
                pc = if (u64::from(!((values[45]) != 0))) != 0 {
                    201
                } else {
                    200
                };
            }
            203 => {
                values[42] = u64::from((0x0u64) as u32);
                pc = 202;
            }
            204 => {
                values[45] = u64::from(
                    (u64::from((u64::from((values[42]) as u32)) == (u64::from((0x35u64) as u32))))
                        != 0,
                );
                pc = 203;
            }
            205 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            (u64::from((values[42]) as u32)) != (u64::from((0x40u64) as u32)),
                        )) != 0)
                            && ((u64::from(
                                (u64::from((values[42]) as u32)) != (u64::from((0x2u64) as u32)),
                            )) != 0),
                    )) != 0)
                        && ((u64::from(
                            (u64::from((values[42]) as u32)) != (u64::from((0x3u64) as u32)),
                        )) != 0),
                )) != 0
                {
                    204
                } else {
                    200
                };
            }
            206 => {
                pc = 203;
            }
            207 => {
                values[45] = u64::from(
                    (u64::from((u64::from((values[42]) as u32)) == (u64::from((0x10bu64) as u32))))
                        != 0,
                );
                pc = 206;
            }
            208 => {
                pc = if (u64::from(
                    (u64::from((values[42]) as u32)) != (u64::from((0x7bu64) as u32)),
                )) != 0
                {
                    207
                } else {
                    200
                };
            }
            209 => {
                pc = if (u64::from(
                    (u64::from((values[42]) as u32)) < (u64::from((0x41u64) as u32)),
                )) != 0
                {
                    205
                } else {
                    208
                };
            }
            210 => {
                pc = if (u64::from(
                    (u64::from((values[42]) as u32)) == (u64::from((0x0u64) as u32)),
                )) != 0
                {
                    185
                } else {
                    209
                };
            }
            211 => {
                values[42] = u64::from(
                    ({
                        let call_arguments = [
                            values[33],
                            relocate("STACK_local_2b0"),
                            0x3u64,
                            0xffffffffu64,
                        ];
                        invoke("FUN_1400b173c", &call_arguments, memory)
                    }) as u32,
                );
                pc = 210;
            }
            212 => {
                values[33] = memory.read_u64(relocate("STACK_local_288"))?;
                pc = 211;
            }
            213 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_270"))?))) != 0
                {
                    212
                } else {
                    211
                };
            }
            214 => {
                values[33] = relocate("STACK_local_288");
                pc = 213;
            }
            215 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_288"),
                        values[34],
                        ((values[39]).wrapping_sub(values[34]) as i64).wrapping_shr((0x1u64) as u32)
                            as u64,
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 214;
            }
            216 => {
                {
                    let destination = relocate("STACK_local_270");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 215;
            }
            217 => {
                {
                    let destination = relocate("STACK_local_278");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 216;
            }
            218 => {
                {
                    let destination = relocate("STACK_uStack_280");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 217;
            }
            219 => {
                {
                    let destination = relocate("STACK_local_288");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 218;
            }
            220 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[31])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            == (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        || ((u64::from(
                            ((((u64::from(memory.read_u16(values[31])?)) as i16 as i64 as u64)
                                as i32 as i64 as u64) as i64)
                                == (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    224
                } else {
                    219
                };
            }
            221 => {
                values[31] = (values[39]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                );
                pc = 220;
            }
            222 => {
                pc = 219;
            }
            223 => {
                pc = if (u64::from((values[30]) == (values[39]))) != 0 {
                    222
                } else {
                    221
                };
            }
            224 => {
                values[39] = values[31];
                pc = 223;
            }
            225 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((values[7]) as i32 as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[39] = (values[39]).wrapping_add(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64)
                                        .wrapping_mul(2),
                                );
                                values[39]
                            };
                            u64::from(
                                (((values[7]) as i32 as i64 as u64) as i64)
                                    != (((0x2fu64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    229
                } else {
                    224
                };
            }
            226 => {
                values[31] = values[39];
                pc = 225;
            }
            227 => {
                values[7] = ((u64::from(memory.read_u16((values[39]).wrapping_add(
                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64).wrapping_mul(2),
                ))?)) as i16 as i64 as u64) as i16 as i64 as u64;
                pc = 226;
            }
            228 => {
                pc = 219;
            }
            229 => {
                pc = if (u64::from((values[30]) == (values[39]))) != 0 {
                    228
                } else {
                    227
                };
            }
            230 => {
                pc = if (u64::from((values[30]) != (values[39]))) != 0 {
                    233
                } else {
                    229
                };
            }
            231 => {
                values[30] = (values[30]).wrapping_add((0x1u64).wrapping_mul(2));
                pc = 230;
            }
            232 => {
                pc = 229;
            }
            233 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((u64::from(memory.read_u16(values[30])?)) as i16 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            != (((0x5cu64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && ((u64::from(
                            ((((u64::from(memory.read_u16(values[30])?)) as i16 as i64 as u64)
                                as i32 as i64 as u64) as i64)
                                != (((0x2fu64) as i32 as i64 as u64) as i64),
                        )) != 0),
                )) != 0
                {
                    232
                } else {
                    231
                };
            }
            234 => {
                pc = if (u64::from((values[30]) != (values[39]))) != 0 {
                    233
                } else {
                    219
                };
            }
            235 => {
                values[30] = {
                    let call_arguments = [values[34], values[39]];
                    invoke("FUN_140019de0", &call_arguments, memory)
                };
                pc = 234;
            }
            236 => {
                values[39] = (values[34]).wrapping_add(
                    (memory.read_u64(relocate("STACK_local_258"))?).wrapping_mul(0x2u64),
                );
                pc = 235;
            }
            237 => {
                values[34] = memory.read_u64(relocate("STACK_local_268"))?;
                pc = 236;
            }
            238 => {
                pc = if (u64::from((0x7u64) < (memory.read_u64(relocate("STACK_local_250"))?))) != 0
                {
                    237
                } else {
                    236
                };
            }
            239 => {
                values[34] = relocate("STACK_local_268");
                pc = 238;
            }
            240 => {
                {
                    let call_arguments = [relocate("STACK_local_268")];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 239;
            }
            241 => {
                {
                    let destination = relocate("STACK_local_250");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 240;
            }
            242 => {
                {
                    let destination = relocate("STACK_local_258");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 241;
            }
            243 => {
                {
                    let destination = relocate("STACK_uStack_260");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 242;
            }
            244 => {
                {
                    let destination = relocate("STACK_local_268");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 243;
            }
            245 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u16(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_378"))?)?)
                                .wrapping_add((values[41]).wrapping_mul(0x2u64)),
                        )?,
                    )) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    246
                } else {
                    244
                };
            }
            246 => {
                values[41] = (values[41]).wrapping_add(0x1u64);
                pc = 245;
            }
            247 => {
                values[41] = (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64;
                pc = 246;
            }
            248 => {
                values[25] = memory.read_u64(relocate("STACK_local_378"))?;
                pc = 247;
            }
            249 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x58au64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            250 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((((values[32]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[21] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(
                                                memory.read_u64(relocate("STACK_local_358"))?,
                                            )?)
                                            .wrapping_add(0x28u64),
                                        )?,
                                        memory.read_u64(relocate("STACK_local_358"))?,
                                        (values[32]) & (0xffffffffu64),
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[21]
                            };
                            u64::from(
                                (((values[21]) as i32 as i64 as u64) as i64)
                                    < (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    249
                } else {
                    248
                };
            }
            251 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x586u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            252 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    251
                } else {
                    248
                };
            }
            253 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x28u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        values[42],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 252;
            }
            254 => {
                pc = if (u64::from(
                    ((u64::from((u64::from((values[42]) as u32)) == (u64::from((0x0u64) as u32))))
                        != 0)
                        || ((u64::from(
                            ({
                                let numerator = ((memory
                                    .read_u64(relocate("STACK_puStack_330"))?)
                                .wrapping_sub(memory.read_u64(relocate("STACK_local_338"))?))
                                    as i64;
                                let denominator = (0x18u64) as i64;
                                if denominator == 0 {
                                    return Err(AdapterError::Arithmetic("division by zero"));
                                }
                                numerator
                                    .checked_div(denominator)
                                    .ok_or(AdapterError::Arithmetic("signed division overflow"))?
                                    as u64
                            }) < (values[42]),
                        )) != 0),
                )) != 0
                {
                    250
                } else {
                    253
                };
            }
            255 => {
                values[42] = u64::from(
                    ({
                        let call_arguments = [
                            relocate("CONTEXT_unaff_retaddr"),
                            relocate("STACK_local_338"),
                            memory.read_u64(relocate("STACK_local_3a0"))?,
                        ];
                        invoke("FUN_1400701c0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 254;
            }
            256 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x581u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            257 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    256
                } else {
                    255
                };
            }
            258 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x20u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        (((memory.read_u64(relocate("STACK_puStack_2c0"))?)
                            .wrapping_sub(memory.read_u64(relocate("STACK_local_2c8"))?)
                            as i64)
                            .wrapping_shr((0x4u64) as u32) as u64)
                            & (0xffffffffu64),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 257;
            }
            259 => {
                pc = if (u64::from(
                    (values[38])
                        < ({
                            let numerator = ((memory.read_u64(relocate("STACK_puStack_330"))?)
                                .wrapping_sub(memory.read_u64(relocate("STACK_local_338"))?))
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
                    266
                } else {
                    258
                };
            }
            260 => {
                values[38] = (values[38]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 259;
            }
            261 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_2c8"),
                        memory.read_u64(relocate("STACK_puStack_2c0"))?,
                    ];
                    invoke("FUN_140079ab0", &call_arguments, memory)
                };
                pc = 260;
            }
            262 => {
                values[38] = (values[38]).wrapping_add((0x1u64).wrapping_mul(1));
                pc = 259;
            }
            263 => {
                {
                    let destination = relocate("STACK_puStack_2c0");
                    let stored = (memory.read_u64(relocate("STACK_puStack_2c0"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 262;
            }
            264 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_2c0"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = memory.read_u64(
                        (memory.read_u64(relocate("STACK_local_300"))?)
                            .wrapping_add(((values[38]).wrapping_mul(0x8u64)).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 263;
            }
            265 => {
                {
                    let destination = memory.read_u64(relocate("STACK_puStack_2c0"))?;
                    let stored = memory.read_u64(
                        (memory.read_u64(relocate("STACK_local_318"))?)
                            .wrapping_add(((values[38]).wrapping_mul(0x8u64)).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 264;
            }
            266 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_puStack_2c0"))?)
                        == (memory.read_u64(relocate("STACK_local_2b8"))?),
                )) != 0
                {
                    261
                } else {
                    265
                };
            }
            267 => {
                {
                    let destination = relocate("STACK_local_2b8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 259;
            }
            268 => {
                {
                    let destination = relocate("STACK_puStack_2c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 267;
            }
            269 => {
                {
                    let destination = relocate("STACK_local_2c8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 268;
            }
            270 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_338"),
                        memory.read_u64(relocate("STACK_puStack_330"))?,
                        relocate("DAT_1401d04f0"),
                    ];
                    invoke("FUN_1400798c0", &call_arguments, memory)
                };
                pc = 269;
            }
            271 => {
                {
                    let destination = relocate("STACK_puStack_330");
                    let stored = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x3u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 269;
            }
            272 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = (((u64::from(
                        (u64::from(memory.read_u32(
                            (relocate("DAT_1401d0500")).wrapping_add((0x4u64).wrapping_mul(1)),
                        )?)) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((u64::from(
                            (u64::from(
                                (u64::from(memory.read_u8(relocate("DAT_1401d0500"))?)) as u8,
                            )) as u32,
                        )) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 271;
            }
            273 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_puStack_330"))?).wrapping_add(0xcu64);
                    let stored = u64::from((values[14]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 272;
            }
            274 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = u64::from((values[13]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 273;
            }
            275 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_puStack_330"))?).wrapping_add(0x4u64);
                    let stored = u64::from((values[12]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 274;
            }
            276 => {
                {
                    let destination = memory.read_u64(relocate("STACK_puStack_330"))?;
                    let stored = u64::from(
                        (u64::from(
                            (u64::from(memory.read_u32(relocate("_DAT_1401d04f0"))?)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 275;
            }
            277 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_puStack_330"))?)
                        == (memory.read_u64(relocate("STACK_local_328"))?),
                )) != 0
                {
                    270
                } else {
                    276
                };
            }
            278 => {
                values[12] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32(relocate("uRam00000001401d04f4"))?)) as u32,
                    )) as u32,
                );
                pc = 277;
            }
            279 => {
                values[13] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32(relocate("uRam00000001401d04f8"))?)) as u32,
                    )) as u32,
                );
                pc = 278;
            }
            280 => {
                values[14] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32(relocate("uRam00000001401d04fc"))?)) as u32,
                    )) as u32,
                );
                pc = 279;
            }
            281 => {
                {
                    let call_arguments = [relocate("DAT_1401d04e8")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 280;
            }
            282 => {
                {
                    let destination =
                        (relocate("DAT_1401d0500")).wrapping_add((0x4u64).wrapping_mul(1));
                    let stored = u64::from(
                        (((((memory.read_word(
                            (relocate("DAT_1401d0500")).wrapping_add((0x5u64).wrapping_mul(1)),
                            3,
                        )?) & 0xffffffu64)
                            & (0xffffffu64))
                            .wrapping_shl((0x8u64) as u32))
                            | ((0x1u64) & (0xffu64))) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 281;
            }
            283 => {
                {
                    let destination =
                        (relocate("DAT_1401d0500")).wrapping_add((0x0u64).wrapping_mul(1));
                    let stored = u64::from((0x26200au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 282;
            }
            284 => {
                {
                    let destination = relocate("uRam00000001401d04fc");
                    let stored = u64::from((0x759ffab0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 283;
            }
            285 => {
                {
                    let destination = relocate("uRam00000001401d04f8");
                    let stored = u64::from((0xc96acba9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 284;
            }
            286 => {
                {
                    let destination = relocate("uRam00000001401d04f4");
                    let stored = u64::from((0x44149b51u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 285;
            }
            287 => {
                {
                    let destination = relocate("_DAT_1401d04f0");
                    let stored = u64::from((0x3654cecau64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 286;
            }
            288 => {
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
                                (u64::from(memory.read_u32(relocate("DAT_1401d04e8"))?)) as u32,
                            )) as u32,
                        )),
                    )) != 0)
                        && (({
                            let _ = {
                                let call_arguments = [relocate("DAT_1401d04e8")];
                                invoke("FUN_1400b03b8", &call_arguments, memory)
                            };
                            u64::from(
                                (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("DAT_1401d04e8"))?))
                                            as u32,
                                    )) as u32,
                                )) == (u64::from(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32,
                                )),
                            )
                        }) != 0),
                )) != 0
                {
                    287
                } else {
                    280
                };
            }
            289 => {
                {
                    let call_arguments = [relocate("STACK_local_3d8")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 288;
            }
            290 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_300"),
                        memory.read_u64(relocate("STACK_pCStack_2f8"))?,
                        values[29],
                    ];
                    invoke("FUN_140079e30", &call_arguments, memory)
                };
                pc = 289;
            }
            291 => {
                {
                    let destination = relocate("STACK_pCStack_2f8");
                    let stored = (memory.read_u64(relocate("STACK_pCStack_2f8"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 289;
            }
            292 => {
                {
                    let call_arguments =
                        [memory.read_u64(relocate("STACK_pCStack_2f8"))?, values[29]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 291;
            }
            293 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_pCStack_2f8"))?)
                        == (memory.read_u64(relocate("STACK_local_2f0"))?),
                )) != 0
                {
                    290
                } else {
                    292
                };
            }
            294 => {
                values[29] = {
                    let call_arguments = [relocate("STACK_local_3d8"), relocate("L\"*.paint\"")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 293;
            }
            295 => {
                {
                    let call_arguments = [relocate("STACK_local_368")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 294;
            }
            296 => {
                {
                    let call_arguments = [relocate("STACK_local_3d8")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 295;
            }
            297 => {
                {
                    let call_arguments = [relocate("STACK_local_340")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 296;
            }
            298 => {
                {
                    let call_arguments = [relocate("STACK_local_348")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 297;
            }
            299 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 298;
            }
            300 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_318"),
                        memory.read_u64(relocate("STACK_pCStack_310"))?,
                        relocate("STACK_local_360"),
                    ];
                    invoke("FUN_140079e30", &call_arguments, memory)
                };
                pc = 299;
            }
            301 => {
                {
                    let destination = relocate("STACK_pCStack_310");
                    let stored = (memory.read_u64(relocate("STACK_pCStack_310"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 299;
            }
            302 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_pCStack_310"))?,
                        relocate("STACK_local_360"),
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 301;
            }
            303 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_pCStack_310"))?)
                        == (memory.read_u64(relocate("STACK_local_308"))?),
                )) != 0
                {
                    300
                } else {
                    302
                };
            }
            304 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_360"),
                        memory.read_u64(relocate("STACK_local_348"))?,
                        values[22],
                        values[37],
                        values[21],
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::Concatenate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 303;
            }
            305 => {
                values[22] = ({
                    let call_arguments = [relocate("STACK_local_348")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 304;
            }
            306 => {
                values[37] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 305;
            }
            307 => {
                values[21] = ({
                    let call_arguments = [relocate("STACK_local_368")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 306;
            }
            308 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_350")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[42]) & (0xffffffffu64)))
                        | (0x18c00u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 307;
            }
            309 => {
                {
                    let call_arguments = [relocate("STACK_local_360"), values[26]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 308;
            }
            310 => {
                values[26] = {
                    let call_arguments = [relocate("STACK_local_348")];
                    invoke("ATL::CStringT<>::GetManager", &call_arguments, memory)
                };
                pc = 309;
            }
            311 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_348"),
                        memory.read_u64(relocate("STACK_local_340"))?,
                        values[22],
                        values[35],
                        values[21],
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::Concatenate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 310;
            }
            312 => {
                values[22] = ({
                    let call_arguments = [relocate("STACK_local_340")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 311;
            }
            313 => {
                values[35] = memory.read_u64(values[28])?;
                pc = 312;
            }
            314 => {
                values[21] = ({
                    let call_arguments = [values[28]];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 313;
            }
            315 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_350")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[42]) & (0xffffffffu64)))
                        | (0x8c00u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 314;
            }
            316 => {
                {
                    let call_arguments = [relocate("STACK_local_348"), values[26]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 315;
            }
            317 => {
                values[26] = {
                    let call_arguments = [relocate("STACK_local_340")];
                    invoke("ATL::CStringT<>::GetManager", &call_arguments, memory)
                };
                pc = 316;
            }
            318 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_340"),
                        memory.read_u64(relocate("STACK_local_2d8"))?,
                        values[21],
                        relocate("L\" (\""),
                        0x2u64,
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::Concatenate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 317;
            }
            319 => {
                values[21] = ({
                    let call_arguments = [relocate("STACK_local_2d8")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 318;
            }
            320 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_350")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[42]) & (0xffffffffu64)))
                        | (0x8800u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 319;
            }
            321 => {
                {
                    let call_arguments = [relocate("STACK_local_340"), values[26]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 320;
            }
            322 => {
                values[26] = {
                    let call_arguments = [relocate("STACK_local_2d8")];
                    invoke("ATL::CStringT<>::GetManager", &call_arguments, memory)
                };
                pc = 321;
            }
            323 => {
                values[28] = {
                    let call_arguments = [relocate("STACK_local_3d8"), relocate("L\"*.paint\"")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 322;
            }
            324 => {
                {
                    let call_arguments = [relocate("STACK_local_368"), relocate("\")\"")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 323;
            }
            325 => {
                {
                    let call_arguments = [values[27], 0x0u64, values[40]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 324;
            }
            326 => {
                values[27] = {
                    let call_arguments = [];
                    invoke("GetProcessHeap", &call_arguments, memory)
                };
                pc = 325;
            }
            327 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            328 => {
                pc = if (u64::from(
                    (((((values[21]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    327
                } else {
                    324
                };
            }
            329 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    326
                } else {
                    328
                };
            }
            330 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 329;
            }
            331 => {
                {
                    let destination = values[37];
                    let stored = ((((u64::from(memory.read_u32(values[37])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 330;
            }
            332 => {
                values[21] = ((u64::from(memory.read_u32(values[37])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 331;
            }
            333 => {
                values[37] = (values[40]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 332;
            }
            334 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 333;
            }
            335 => {
                pc = if (u64::from((values[37]) != (0x0u64))) != 0 {
                    334
                } else {
                    324
                };
            }
            336 => {
                {
                    let call_arguments = [relocate("STACK_local_2d8"), values[35]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 335;
            }
            337 => {
                values[40] = values[38];
                pc = 336;
            }
            338 => {
                values[35] = relocate("L\"\"");
                pc = 337;
            }
            339 => {
                values[40] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 336;
            }
            340 => {
                values[35] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_368"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                )?;
                pc = 339;
            }
            341 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_368"))?) == (0x0u64)))
                    != 0
                {
                    338
                } else {
                    340
                };
            }
            342 => {
                values[38] = 0x0u64;
                pc = 341;
            }
            343 => {
                values[37] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 342;
            }
            344 => {
                values[42] = u64::from((u64::from(((values[42]) | (0x1000u64)) as u32)) as u32);
                pc = 343;
            }
            345 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_368"),
                        relocate("STACK_local_3d8"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 344;
            }
            346 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[41];
                    memory.write_u64(destination, stored)?;
                }
                pc = 345;
            }
            347 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 346;
            }
            348 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 347;
            }
            349 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 343;
            }
            350 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 349;
            }
            351 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 350;
            }
            352 => {
                values[42] = u64::from((u64::from(((values[42]) | (0x7000u64)) as u32)) as u32);
                pc = 351;
            }
            353 => {
                {
                    let call_arguments = [values[21], relocate("STACK_local_3d0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            354 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    353
                } else {
                    352
                };
            }
            355 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_2b0"))?,
                        relocate("STACK_local_368"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 354;
            }
            356 => {
                {
                    let destination = relocate("STACK_local_3c0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 355;
            }
            357 => {
                {
                    let destination = relocate("STACK_uStack_3c8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 356;
            }
            358 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_3d0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 357;
            }
            359 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 358;
            }
            360 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    348
                } else {
                    359
                };
            }
            361 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 360;
            }
            362 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 361;
            }
            363 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 362;
            }
            364 => {
                {
                    let destination = relocate("STACK_local_3b0");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 363;
            }
            365 => {
                {
                    let destination = relocate("STACK_local_3d8");
                    let stored = relocate("STACK_local_2b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 364;
            }
            366 => {
                {
                    let destination = relocate("STACK_local_2b0");
                    let stored = relocate("STACK_uStack_2a8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 365;
            }
            367 => {
                {
                    let destination = relocate("STACK_pwStack_298");
                    let stored = relocate("L\"PaintProjectFileFormat\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 366;
            }
            368 => {
                {
                    let destination = relocate("STACK_uStack_2a8");
                    let stored = 0x1600000001u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 367;
            }
            369 => {
                pc = if ({
                    let _ = {
                        let _ = {
                            values[17] = memory.read_u64(relocate("STACK_local_290"))?;
                            values[17]
                        };
                        {
                            values[41] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                            values[41]
                        }
                    };
                    u64::from(
                        (u64::from((u64::from((values[44]) as u32)) as u32))
                            < (u64::from(
                                (u64::from(
                                    (u64::from(
                                        memory.read_u32(
                                            (relocate("STACK_local_320"))
                                                .wrapping_add((0x0u64).wrapping_mul(4)),
                                        )?,
                                    )) as u32,
                                )) as u32,
                            )),
                    )
                }) != 0
                {
                    399
                } else {
                    368
                };
            }
            370 => {
                let _ = {
                    values[44] =
                        u64::from(((u64::from((values[44]) as u32)).wrapping_add(0x1u64)) as u32);
                    values[44]
                };
                pc = 369;
            }
            371 => {
                {
                    let call_arguments = [relocate("STACK_local_350")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 370;
            }
            372 => {
                {
                    let call_arguments = [relocate("STACK_local_348")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 371;
            }
            373 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_338"),
                        memory.read_u64(relocate("STACK_puStack_330"))?,
                        ((values[17])
                            .wrapping_add(((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1)))
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                    ];
                    invoke("FUN_1400796c0", &call_arguments, memory)
                };
                pc = 372;
            }
            374 => {
                {
                    let destination = relocate("STACK_puStack_330");
                    let stored = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x3u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 372;
            }
            375 => {
                {
                    let destination =
                        (memory.read_u64(relocate("STACK_puStack_330"))?).wrapping_add(0x14u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 374;
            }
            376 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = u64::from((0x26200au64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 375;
            }
            377 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 376;
            }
            378 => {
                {
                    let destination = memory.read_u64(relocate("STACK_puStack_330"))?;
                    let stored = memory.read_u64(
                        ((values[17])
                            .wrapping_add(((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1)))
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                    )?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 377;
            }
            379 => {
                values[11] = memory.read_u64(
                    ((values[17])
                        .wrapping_add(((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1)))
                    .wrapping_add((0x18u64).wrapping_mul(1)),
                )?;
                pc = 378;
            }
            380 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_puStack_330"))?)
                        == (memory.read_u64(relocate("STACK_local_328"))?),
                )) != 0
                {
                    373
                } else {
                    379
                };
            }
            381 => {
                values[32] = ((memory.read_u64(relocate("STACK_pCStack_310"))?)
                    .wrapping_sub(memory.read_u64(relocate("STACK_local_318"))?)
                    as i64)
                    .wrapping_shr((0x3u64) as u32) as u64;
                pc = 380;
            }
            382 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    381
                } else {
                    380
                };
            }
            383 => {
                values[21] = ({
                    let call_arguments = [
                        ((values[17])
                            .wrapping_add(((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1)))
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                        relocate("DAT_1400e4bf0"),
                        0x10u64,
                    ];
                    invoke("memcmp", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 382;
            }
            384 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_300"),
                        memory.read_u64(relocate("STACK_pCStack_2f8"))?,
                        relocate("STACK_local_348"),
                    ];
                    invoke("FUN_140079e30", &call_arguments, memory)
                };
                pc = 383;
            }
            385 => {
                {
                    let destination = relocate("STACK_pCStack_2f8");
                    let stored = (memory.read_u64(relocate("STACK_pCStack_2f8"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 383;
            }
            386 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_pCStack_2f8"))?,
                        relocate("STACK_local_348"),
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 385;
            }
            387 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_pCStack_2f8"))?)
                        == (memory.read_u64(relocate("STACK_local_2f0"))?),
                )) != 0
                {
                    384
                } else {
                    386
                };
            }
            388 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_318"),
                        memory.read_u64(relocate("STACK_pCStack_310"))?,
                        relocate("STACK_local_350"),
                    ];
                    invoke("FUN_140079e30", &call_arguments, memory)
                };
                pc = 387;
            }
            389 => {
                {
                    let destination = relocate("STACK_pCStack_310");
                    let stored = (memory.read_u64(relocate("STACK_pCStack_310"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 387;
            }
            390 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_pCStack_310"))?,
                        relocate("STACK_local_350"),
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 389;
            }
            391 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_pCStack_310"))?)
                        == (memory.read_u64(relocate("STACK_local_308"))?),
                )) != 0
                {
                    388
                } else {
                    390
                };
            }
            392 => {
                {
                    let call_arguments = [relocate("STACK_local_350"), relocate("L\")\"")];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 391;
            }
            393 => {
                {
                    let call_arguments = [relocate("STACK_local_350"), relocate("STACK_local_348")];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 392;
            }
            394 => {
                {
                    let call_arguments = [relocate("STACK_local_350"), relocate("L\" (\"")];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 393;
            }
            395 => {
                {
                    let call_arguments = [relocate("STACK_local_348")];
                    invoke("ATL::CStringT<>::MakeLower", &call_arguments, memory)
                };
                pc = 394;
            }
            396 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_348"),
                        memory.read_u64(
                            ((values[17]).wrapping_add(
                                ((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1),
                            ))
                            .wrapping_add((0x38u64).wrapping_mul(1)),
                        )?,
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 395;
            }
            397 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_350"),
                        memory.read_u64(
                            ((values[17]).wrapping_add(
                                ((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1),
                            ))
                            .wrapping_add((0x30u64).wrapping_mul(1)),
                        )?,
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 396;
            }
            398 => {
                pc = if (u64::from(
                    ((u64::from(
                        ((u64::from(
                            ((u64::from(
                                (((values[19]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )) != 0)
                                && (({
                                    let _ = {
                                        values[21] = ({
                                            let call_arguments = [
                                                ((values[17]).wrapping_add(
                                                    ((values[44]).wrapping_mul(0x68u64))
                                                        .wrapping_mul(1),
                                                ))
                                                .wrapping_add((0x10u64).wrapping_mul(1)),
                                                relocate("DAT_1400e4c50"),
                                                0x10u64,
                                            ];
                                            invoke("memcmp", &call_arguments, memory)
                                        })
                                            as i32
                                            as i64
                                            as u64;
                                        values[21]
                                    };
                                    u64::from(
                                        (((values[21]) as i32 as i64 as u64) as i64)
                                            != (((0x0u64) as i32 as i64 as u64) as i64),
                                    )
                                }) != 0),
                        )) != 0)
                            && (({
                                let _ = {
                                    values[21] = ({
                                        let call_arguments = [
                                            ((values[17]).wrapping_add(
                                                ((values[44]).wrapping_mul(0x68u64))
                                                    .wrapping_mul(1),
                                            ))
                                            .wrapping_add((0x10u64).wrapping_mul(1)),
                                            relocate("DAT_1400e4c80"),
                                            0x10u64,
                                        ];
                                        invoke("memcmp", &call_arguments, memory)
                                    }) as i32
                                        as i64
                                        as u64;
                                    values[21]
                                };
                                u64::from(
                                    (((values[21]) as i32 as i64 as u64) as i64)
                                        != (((0x0u64) as i32 as i64 as u64) as i64),
                                )
                            }) != 0),
                    )) != 0)
                        && (({
                            let _ = {
                                values[21] = ({
                                    let call_arguments = [
                                        ((values[17]).wrapping_add(
                                            ((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1),
                                        ))
                                        .wrapping_add((0x10u64).wrapping_mul(1)),
                                        relocate("DAT_1400e4c00"),
                                        0x10u64,
                                    ];
                                    invoke("memcmp", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[21]
                            };
                            u64::from(
                                (((values[21]) as i32 as i64 as u64) as i64)
                                    != (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    397
                } else {
                    370
                };
            }
            399 => {
                values[19] = ({
                    let call_arguments = [
                        values[8],
                        ((memory.read_u64(relocate("STACK_local_290"))?)
                            .wrapping_add(((values[44]).wrapping_mul(0x68u64)).wrapping_mul(1)))
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                    ];
                    invoke("FUN_140015560", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 398;
            }
            400 => {
                let _ = {
                    values[44] = (values[37]) & (0xffffffffu64);
                    values[44]
                };
                pc = 369;
            }
            401 => {
                values[32] = (values[37]) & (0xffffffffu64);
                pc = 400;
            }
            402 => {
                pc = if (u64::from((values[43]) != (relocate("DAT_1401d0c70")))) != 0 {
                    482
                } else {
                    401
                };
            }
            403 => {
                values[43] = (values[43]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 402;
            }
            404 => {
                values[43] = (values[43]).wrapping_add((0x2u64).wrapping_mul(8));
                pc = 402;
            }
            405 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 404;
            }
            406 => {
                values[37] = 0x0u64;
                pc = 405;
            }
            407 => {
                {
                    let call_arguments = [values[27], 0x0u64, values[37]];
                    invoke("HeapFree", &call_arguments, memory)
                };
                pc = 406;
            }
            408 => {
                values[27] = {
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
                    (((((values[21]).wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
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
                    (((values[21]) as i32 as i64 as u64) as i64)
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
                    let destination = values[38];
                    let stored = ((((u64::from(memory.read_u32(values[38])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 412;
            }
            414 => {
                values[21] = ((u64::from(memory.read_u32(values[38])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 413;
            }
            415 => {
                values[38] = (values[37]).wrapping_add((0x18u64).wrapping_mul(1));
                pc = 414;
            }
            416 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 415;
            }
            417 => {
                pc = 403;
            }
            418 => {
                pc = if (u64::from((values[37]) == (0x0u64))) != 0 {
                    417
                } else {
                    416
                };
            }
            419 => {
                {
                    let call_arguments = [relocate("STACK_local_340")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 418;
            }
            420 => {
                {
                    let call_arguments = [relocate("STACK_local_348")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 419;
            }
            421 => {
                {
                    let call_arguments = [relocate("STACK_local_338")];
                    invoke("FUN_1400798c0", &call_arguments, memory)
                };
                pc = 420;
            }
            422 => {
                {
                    let destination = relocate("STACK_puStack_330");
                    let stored = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x3u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 420;
            }
            423 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x2u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[9]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 422;
            }
            424 => {
                {
                    let destination = (memory.read_u64(relocate("STACK_puStack_330"))?)
                        .wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 423;
            }
            425 => {
                {
                    let destination = memory.read_u64(relocate("STACK_puStack_330"))?;
                    let stored = memory.read_u64(values[9])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 424;
            }
            426 => {
                values[11] = memory.read_u64((values[9]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 425;
            }
            427 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_puStack_330"))?)
                        == (memory.read_u64(relocate("STACK_local_328"))?),
                )) != 0
                {
                    421
                } else {
                    426
                };
            }
            428 => {
                values[9] = memory.read_u64(values[43])?;
                pc = 427;
            }
            429 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_300"),
                        memory.read_u64(relocate("STACK_pCStack_2f8"))?,
                        relocate("STACK_local_348"),
                    ];
                    invoke("FUN_140079e30", &call_arguments, memory)
                };
                pc = 428;
            }
            430 => {
                {
                    let destination = relocate("STACK_pCStack_2f8");
                    let stored = (memory.read_u64(relocate("STACK_pCStack_2f8"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 428;
            }
            431 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_pCStack_2f8"))?,
                        relocate("STACK_local_348"),
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 430;
            }
            432 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_pCStack_2f8"))?)
                        == (memory.read_u64(relocate("STACK_local_2f0"))?),
                )) != 0
                {
                    429
                } else {
                    431
                };
            }
            433 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_318"),
                        memory.read_u64(relocate("STACK_pCStack_310"))?,
                        relocate("STACK_local_340"),
                    ];
                    invoke("FUN_140079e30", &call_arguments, memory)
                };
                pc = 432;
            }
            434 => {
                {
                    let destination = relocate("STACK_pCStack_310");
                    let stored = (memory.read_u64(relocate("STACK_pCStack_310"))?)
                        .wrapping_add((0x8u64).wrapping_mul(1));
                    memory.write_u64(destination, stored)?;
                }
                pc = 432;
            }
            435 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_pCStack_310"))?,
                        relocate("STACK_local_340"),
                    ];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 434;
            }
            436 => {
                pc = if (u64::from(
                    (memory.read_u64(relocate("STACK_pCStack_310"))?)
                        == (memory.read_u64(relocate("STACK_local_308"))?),
                )) != 0
                {
                    433
                } else {
                    435
                };
            }
            437 => {
                {
                    let call_arguments = [relocate("STACK_local_348"), 0x0u64];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 436;
            }
            438 => {
                {
                    let call_arguments = [relocate("STACK_local_3d8")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 437;
            }
            439 => {
                {
                    let call_arguments = [relocate("STACK_local_360")];
                    invoke("ATL::CStringT<>::~CStringT<>", &call_arguments, memory)
                };
                pc = 438;
            }
            440 => {
                values[42] = u64::from((u64::from(((values[42]) | (0x40000u64)) as u32)) as u32);
                pc = 439;
            }
            441 => {
                {
                    let call_arguments = [relocate("STACK_local_348"), relocate("STACK_local_360")];
                    invoke("ATL::CStringT<>::operator+=", &call_arguments, memory)
                };
                pc = 440;
            }
            442 => {
                {
                    let call_arguments = [
                        relocate("STACK_local_360"),
                        relocate("STACK_local_2e8"),
                        0x1u64,
                        memory.read_u64(values[25])?,
                        values[21],
                    ];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::Concatenate",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 441;
            }
            443 => {
                values[21] = ({
                    let call_arguments = [values[25]];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 442;
            }
            444 => {
                {
                    let destination =
                        (relocate("STACK_local_2e8")).wrapping_add((0x0u64).wrapping_mul(2));
                    let stored = u64::from((0x3bu64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 443;
            }
            445 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = ((((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_350")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((values[42]) & (0xffffffffu64)))
                        | (0x60000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 444;
            }
            446 => {
                {
                    let call_arguments = [relocate("STACK_local_360"), values[26]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 445;
            }
            447 => {
                values[26] = {
                    let call_arguments = [values[25]];
                    invoke("ATL::CStringT<>::GetManager", &call_arguments, memory)
                };
                pc = 446;
            }
            448 => {
                values[25] = {
                    let call_arguments = [relocate("STACK_local_3d8"), relocate("L\"*.dib\"")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 447;
            }
            449 => {
                {
                    let call_arguments = [relocate("STACK_local_348"), relocate("L\"*.bmp\"")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 448;
            }
            450 => {
                {
                    let call_arguments = [relocate("STACK_local_340"), values[35]];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 449;
            }
            451 => {
                values[35] = relocate("L\"\"");
                pc = 450;
            }
            452 => {
                values[35] = 0x0u64;
                pc = 450;
            }
            453 => {
                values[35] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_368"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                )?;
                pc = 450;
            }
            454 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_368"))?) == (0x0u64)))
                    != 0
                {
                    452
                } else {
                    453
                };
            }
            455 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_368"))?) == (0x0u64)))
                    != 0
                {
                    451
                } else {
                    454
                };
            }
            456 => {
                values[37] = memory.read_u64(relocate("STACK_local_368"))?;
                pc = 455;
            }
            457 => {
                values[42] = u64::from((u64::from(((values[42]) | (0x80000u64)) as u32)) as u32);
                pc = 456;
            }
            458 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_368"),
                        relocate("STACK_local_350"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 457;
            }
            459 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[41];
                    memory.write_u64(destination, stored)?;
                }
                pc = 458;
            }
            460 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 459;
            }
            461 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 460;
            }
            462 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 456;
            }
            463 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 462;
            }
            464 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 463;
            }
            465 => {
                values[42] = u64::from((u64::from(((values[42]) | (0x380000u64)) as u32)) as u32);
                pc = 464;
            }
            466 => {
                pc = 508;
            }
            467 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    466
                } else {
                    465
                };
            }
            468 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_2b0"))?,
                        relocate("STACK_local_368"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 467;
            }
            469 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = values[37];
                    memory.write_u64(destination, stored)?;
                }
                pc = 468;
            }
            470 => {
                {
                    let destination = relocate("STACK_local_3c0");
                    let stored = values[37];
                    memory.write_u64(destination, stored)?;
                }
                pc = 469;
            }
            471 => {
                {
                    let destination = relocate("STACK_uStack_3c8");
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 470;
            }
            472 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_3d0")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 471;
            }
            473 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    461
                } else {
                    472
                };
            }
            474 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 473;
            }
            475 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 474;
            }
            476 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 475;
            }
            477 => {
                {
                    let destination = relocate("STACK_local_3b0");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 476;
            }
            478 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored = relocate("STACK_local_2b0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 477;
            }
            479 => {
                {
                    let destination = relocate("STACK_local_2b0");
                    let stored =
                        memory.read_u64((values[43]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 478;
            }
            480 => {
                pc = if (u64::from(
                    (((values[19]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    403
                } else {
                    479
                };
            }
            481 => {
                values[41] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 480;
            }
            482 => {
                values[19] = ({
                    let call_arguments = [values[8], memory.read_u64(values[43])?];
                    invoke("FUN_140015560", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 481;
            }
            483 => {
                {
                    let destination = relocate("STACK_plStack_388");
                    let stored = memory.read_u64(relocate("STACK_local_398"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 402;
            }
            484 => {
                {
                    let destination = relocate("STACK_local_390");
                    let stored = values[8];
                    memory.write_u64(destination, stored)?;
                }
                pc = 483;
            }
            485 => {
                values[43] = relocate("DAT_1401d0c30");
                pc = 484;
            }
            486 => {
                {
                    let destination = relocate("STACK_local_398");
                    let stored = memory.read_u64((values[41]).wrapping_add(0x2a0u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 485;
            }
            487 => {
                values[8] = memory.read_u64((values[41]).wrapping_add(0x298u64))?;
                pc = 486;
            }
            488 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 487;
            }
            489 => {
                {
                    let destination = values[5];
                    let stored = ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 488;
            }
            490 => {
                values[5] =
                    (memory.read_u64((values[41]).wrapping_add(0x2a0u64))?).wrapping_add(0x8u64);
                pc = 489;
            }
            491 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 490;
            }
            492 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[41]).wrapping_add(0x2a0u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    491
                } else {
                    487
                };
            }
            493 => {
                values[41] =
                    memory.read_u64((values[24]).wrapping_add((0xe8u64).wrapping_mul(1)))?;
                pc = 492;
            }
            494 => {
                values[24] = {
                    let call_arguments = [values[38]];
                    invoke("CFrameWnd::GetActiveView", &call_arguments, memory)
                };
                pc = 493;
            }
            495 => {
                values[38] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[23])?).wrapping_add(0xf8u64))?,
                        values[23],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 494;
            }
            496 => {
                pc = if (u64::from((values[23]) != (0x0u64))) != 0 {
                    495
                } else {
                    494
                };
            }
            497 => {
                values[38] = values[37];
                pc = 496;
            }
            498 => {
                values[23] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 497;
            }
            499 => {
                {
                    let destination = relocate("STACK_local_2f0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 498;
            }
            500 => {
                {
                    let destination = relocate("STACK_pCStack_2f8");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 499;
            }
            501 => {
                {
                    let destination = relocate("STACK_local_300");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 500;
            }
            502 => {
                {
                    let destination = relocate("STACK_local_308");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 501;
            }
            503 => {
                {
                    let destination = relocate("STACK_pCStack_310");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 502;
            }
            504 => {
                {
                    let destination = relocate("STACK_local_318");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 503;
            }
            505 => {
                {
                    let destination = relocate("STACK_local_328");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 504;
            }
            506 => {
                {
                    let destination = relocate("STACK_puStack_330");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 505;
            }
            507 => {
                {
                    let destination = relocate("STACK_local_338");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 506;
            }
            508 => {
                {
                    let call_arguments = [values[21], relocate("STACK_local_3d0")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            509 => {
                values[21] = ({
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr")];
                    invoke("FUN_14007c620", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 508;
            }
            510 => {
                {
                    let call_arguments = [0x8000ffffu64];
                    invoke("FUN_1400122b0", &call_arguments, memory)
                };
                pc = 509;
            }
            511 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    510
                } else {
                    507
                };
            }
            512 => {
                values[21] = ({
                    let call_arguments = [relocate("STACK_local_320"), relocate("STACK_local_290")];
                    invoke("GetGdiplusEncoders", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 511;
            }
            513 => {
                {
                    let destination = relocate("STACK_local_290");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 512;
            }
            514 => {
                {
                    let destination =
                        (relocate("STACK_local_320")).wrapping_add((0x0u64).wrapping_mul(4));
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 513;
            }
            515 => {
                values[37] = 0x0u64;
                pc = 514;
            }
            516 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x52cu64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            517 => {
                pc = if (u64::from(
                    ((u64::from(
                        (((0x1u64) as i32 as i64 as u64) as i64)
                            < (((values[21]) as i32 as i64 as u64) as i64),
                    )) != 0)
                        && (({
                            let _ = {
                                values[21] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(
                                                memory.read_u64(relocate("STACK_local_358"))?,
                                            )?)
                                            .wrapping_add(0xb0u64),
                                        )?,
                                        memory.read_u64(relocate("STACK_local_358"))?,
                                        (memory.read_u64(relocate("STACK_local_2d0"))?)
                                            .wrapping_add(0x2u64),
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                }) as i32 as i64
                                    as u64;
                                values[21]
                            };
                            u64::from(
                                (((values[21]) as i32 as i64 as u64) as i64)
                                    < (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    516
                } else {
                    515
                };
            }
            518 => {
                values[21] = ({
                    let call_arguments = [relocate("STACK_local_2d0")];
                    invoke(
                        "ATL::CSimpleStringT<wchar_t,1>::GetLength",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 517;
            }
            519 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[43])?).wrapping_add(0xd8u64))?,
                        values[43],
                        relocate("STACK_local_248"),
                        0x3u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 518;
            }
            520 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[43])?).wrapping_add(0xd8u64))?,
                        values[43],
                        relocate("STACK_local_2d0"),
                        0x4u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 519;
            }
            521 => {
                pc = if (u64::from(
                    ((u64::from((memory.read_u64(relocate("STACK_local_368"))?) != (0x0u64))) != 0)
                        && (({
                            let _ = {
                                values[43] = {
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(
                                                memory.read_u64(relocate("DAT_1401d05d8"))?,
                                            )?)
                                            .wrapping_add(0x38u64),
                                        )?,
                                        memory.read_u64(relocate("DAT_1401d05d8"))?,
                                        relocate("STACK_local_368"),
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                };
                                values[43]
                            };
                            u64::from((values[43]) != (0x0u64))
                        }) != 0),
                )) != 0
                {
                    520
                } else {
                    518
                };
            }
            522 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = {
                        let call_arguments = [memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401d05d8"))?)?)
                                .wrapping_add(0x30u64),
                        )?];
                        invoke("INDIRECT_CALL", &call_arguments, memory)
                    };
                    memory.write_u64(destination, stored)?;
                }
                pc = 521;
            }
            523 => {
                {
                    let call_arguments = [relocate("STACK_local_248")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 522;
            }
            524 => {
                {
                    let call_arguments = [relocate("STACK_local_2d0")];
                    invoke("ATL::CStringT<>::CStringT<>", &call_arguments, memory)
                };
                pc = 523;
            }
            525 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x524u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            526 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    525
                } else {
                    524
                };
            }
            527 => {
                values[21] = ({
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                            .wrapping_add(0x48u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 526;
            }
            528 => {
                {
                    let destination = relocate("STACK_local_2e4");
                    let stored = u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32(relocate("STACK_local_2e4"))?)) as u32,
                            )) | (0x842u64)) as u32,
                        )) as u32,
                    );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 527;
            }
            529 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x520u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            530 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    529
                } else {
                    528
                };
            }
            531 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x50u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        relocate("STACK_local_2e4"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 530;
            }
            532 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x51bu64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            533 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    532
                } else {
                    531
                };
            }
            534 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_358"))?)?)
                                .wrapping_add(0x88u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_358"))?,
                        values[36],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 533;
            }
            535 => {
                values[36] = relocate("DAT_140105708");
                pc = 534;
            }
            536 => {
                values[36] = memory.read_u64(
                    (memory.read_u64(relocate("STACK_local_3a8"))?)
                        .wrapping_add((0x10u64).wrapping_mul(1)),
                )?;
                pc = 534;
            }
            537 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_3a8"))?) != (0x0u64)))
                    != 0
                {
                    536
                } else {
                    534
                };
            }
            538 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_3a8"))?) == (0x0u64)))
                    != 0
                {
                    535
                } else {
                    537
                };
            }
            539 => {
                values[42] = u64::from((u64::from(((values[42]) | (0x400000u64)) as u32)) as u32);
                pc = 538;
            }
            540 => {
                values[15] = memory.read_u64(relocate("STACK_local_3a8"))?;
                pc = 539;
            }
            541 => {
                values[42] = u64::from((0x800000u64) as u32);
                pc = 540;
            }
            542 => {
                {
                    let call_arguments = [
                        0x0u64,
                        relocate("STACK_local_3a8"),
                        relocate("STACK_local_368"),
                    ];
                    invoke("FUN_1400325e0", &call_arguments, memory)
                };
                pc = 541;
            }
            543 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = values[41];
                    memory.write_u64(destination, stored)?;
                }
                pc = 542;
            }
            544 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 543;
            }
            545 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 544;
            }
            546 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 540;
            }
            547 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored = (memory.read_u64(relocate("_DAT_1401cfcd8"))?)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 546;
            }
            548 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 547;
            }
            549 => {
                values[42] = u64::from((0x3800000u64) as u32);
                pc = 548;
            }
            550 => {
                {
                    let destination = relocate("STACK_local_3a8");
                    let stored = memory.read_u64(relocate("STACK_local_368"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 549;
            }
            551 => {
                {
                    let call_arguments = [values[21], relocate("STACK_local_390")];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            552 => {
                pc = if (u64::from(
                    (((values[21]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    551
                } else {
                    550
                };
            }
            553 => {
                values[21] = ({
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("DAT_1401cfcd0"))?)?)
                                .wrapping_add(0x30u64),
                        )?,
                        memory.read_u64(relocate("DAT_1401cfcd0"))?,
                        memory.read_u64(relocate("STACK_local_3d0"))?,
                        relocate("STACK_local_368"),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 552;
            }
            554 => {
                {
                    let destination = relocate("STACK_local_380");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 553;
            }
            555 => {
                {
                    let destination = relocate("STACK_plStack_388");
                    let stored = 0x140105810u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 554;
            }
            556 => {
                {
                    let destination = relocate("STACK_local_390");
                    let stored = (((u64::from(
                        (u64::from(
                            memory.read_u32((relocate("STACK_local_390")).wrapping_add(0x4u64))?,
                        )) as u32,
                    )) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x5783u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 555;
            }
            557 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 556;
            }
            558 => {
                pc = if (u64::from((memory.read_u64(relocate("DAT_1401cfcd0"))?) == (0x0u64))) != 0
                {
                    545
                } else {
                    557
                };
            }
            559 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 558;
            }
            560 => {
                {
                    let destination = relocate("_DAT_1401cfcd8");
                    let stored =
                        (memory.read_u64(relocate("_DAT_1401cfcd8"))?).wrapping_add(0x1u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 559;
            }
            561 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 560;
            }
            562 => {
                {
                    let destination = relocate("STACK_local_398");
                    let stored = relocate("DAT_1401cfcd8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 561;
            }
            563 => {
                {
                    let destination = relocate("STACK_local_368");
                    let stored = relocate("STACK_local_3d0");
                    memory.write_u64(destination, stored)?;
                }
                pc = 562;
            }
            564 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 563;
            }
            565 => {
                {
                    let destination = relocate("STACK_local_3b8");
                    let stored = values[35];
                    memory.write_u64(destination, stored)?;
                }
                pc = 563;
            }
            566 => {
                {
                    let destination = relocate("STACK_local_3d0");
                    let stored = relocate("STACK_uStack_3c8");
                    memory.write_u64(destination, stored)?;
                }
                pc = 565;
            }
            567 => {
                {
                    let destination = relocate("STACK_uStack_3c8");
                    let stored = ((((values[32]) as i32 as i64 as u64) & (0xffffffffu64))
                        .wrapping_shl((0x20u64) as u32))
                        | ((0x1u64) & (0xffffffffu64));
                    memory.write_u64(destination, stored)?;
                }
                pc = 566;
            }
            568 => {
                {
                    let call_arguments = [];
                    invoke("abort", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("abort"));
            }
            569 => {
                pc = if (u64::from(
                    (((u64::from(
                        (u64::from(
                            memory
                                .read_u16((values[35]).wrapping_add(
                                    ((values[32]) & (0xffffffffu64)).wrapping_mul(2),
                                ))?,
                        )) as u16,
                    )) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    568
                } else {
                    567
                };
            }
            570 => {
                pc = if (u64::from(
                    ((((values[32]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    564
                } else {
                    569
                };
            }
            571 => {
                pc =
                    if (u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16(
                                (values[35]).wrapping_add((values[32]).wrapping_mul(2)),
                            )?)) as u16,
                        )) as i32 as i64 as u64) as i64)
                            != (((0x0u64) as i32 as i64 as u64) as i64),
                    )) != 0
                    {
                        572
                    } else {
                        570
                    };
            }
            572 => {
                values[32] = (values[32]).wrapping_add(0x1u64);
                pc = 571;
            }
            573 => {
                values[32] = 0xffffffffffffffffu64;
                pc = 572;
            }
            574 => {
                values[35] = relocate("L\"SaveAs\"");
                pc = 573;
            }
            575 => {
                pc = if (u64::from(
                    (((values[1]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    574
                } else {
                    573
                };
            }
            576 => {
                values[35] = relocate("L\"SaveCopyAs\"");
                pc = 575;
            }
            577 => {
                {
                    let call_arguments = [relocate("CONTEXT_unaff_retaddr"), 0x519u64];
                    invoke("FUN_14007c650", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_14007c650"));
            }
            578 => {
                pc = if (u64::from(
                    (((values[20]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    577
                } else {
                    576
                };
            }
            579 => {
                values[41] = memory.read_u64(relocate("_DAT_1401cfcd8"))?;
                pc = 578;
            }
            580 => {
                values[20] = ({
                    let call_arguments = [
                        relocate("DAT_1400e58f8"),
                        0x0u64,
                        0x3u64,
                        relocate("DAT_140117188"),
                        relocate("STACK_local_358"),
                    ];
                    invoke("CoCreateInstance", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 579;
            }
            581 => {
                {
                    let destination = relocate("STACK_local_378");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 580;
            }
            582 => {
                {
                    let destination = relocate("STACK_local_3a0");
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                pc = 581;
            }
            583 => {
                {
                    let destination = relocate("STACK_local_358");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 582;
            }
            584 => {
                {
                    let destination = values[4];
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 583;
            }
            585 => {
                {
                    let destination = relocate("STACK_local_350");
                    let stored =
                        (memory.read_u64(relocate("STACK_local_350"))?) & (0xffffffff00000000u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 584;
            }
            586 => {
                values[36] = 0x0u64;
                pc = 585;
            }
            587 => {
                {
                    let destination = relocate("STACK_local_370");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 586;
            }
            588 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_408"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 587;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_140097aa0` (`mspaint.c:131178`).
pub fn fun_140097aa0(
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
    let mut pc: i32 = 60;
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
                    let call_arguments = [relocate("STACK_local_30")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 0;
            }
            3 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64))) != 0
                {
                    2
                } else {
                    0
                };
            }
            4 => {
                {
                    let destination = values[1];
                    let stored = values[11];
                    memory.write_u64(destination, stored)?;
                }
                pc = 3;
            }
            5 => {
                values[11] = 0x0u64;
                pc = 4;
            }
            6 => {
                pc = if (u64::from((values[12]) == (0x0u64))) != 0 {
                    5
                } else {
                    4
                };
            }
            7 => {
                values[11] = (values[12]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 6;
            }
            8 => {
                values[12] = values[10];
                pc = 7;
            }
            9 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 8;
            }
            10 => {
                {
                    let destination = values[10];
                    let stored = relocate("winrt::impl::heap_implements<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 9;
            }
            11 => {
                {
                    let destination = (values[10]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored =
                        memory.read_u64((values[11]).wrapping_add((0x2u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                {
                    let destination = (values[10]).wrapping_add((0x6u64).wrapping_mul(8));
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 11;
            }
            13 => {
                {
                    let destination = (values[10]).wrapping_add((0x5u64).wrapping_mul(8));
                    let stored = memory.read_u64(values[11])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 12;
            }
            14 => {
                values[2] = memory.read_u64((values[11]).wrapping_add((0x1u64).wrapping_mul(8)))?;
                pc = 13;
            }
            15 => {
                {
                    let call_arguments = [memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_30"))?)?)
                            .wrapping_add(0x8u64),
                    )?];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 14;
            }
            16 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64))) != 0
                {
                    15
                } else {
                    14
                };
            }
            17 => {
                {
                    let destination = (values[10]).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_30"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                {
                    let destination = (values[10]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = relocate("Paint::D2DBitmap::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 17;
            }
            19 => {
                {
                    let destination = values[10];
                    let stored = relocate("Paint::D2DBitmap::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let destination = (values[10]).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x1u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 19;
            }
            21 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            22 => {
                {
                    let destination = relocate("DAT_1401cfae8");
                    let stored = ((((u64::from(memory.read_u32(relocate("DAT_1401cfae8"))?)) as i32
                        as i64 as u64)
                        .wrapping_add(0x1u64)) as i32 as i64
                        as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 21;
            }
            23 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 22;
            }
            24 => {
                {
                    let destination = values[10];
                    let stored = relocate("winrt::impl::producers_base<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 23;
            }
            25 => {
                {
                    let destination = (values[10]).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = relocate("winrt::impl::produce<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    25
                } else {
                    7
                };
            }
            27 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                values[12] = 0x0u64;
                pc = 27;
            }
            29 => {
                values[10] = {
                    let call_arguments = [0x40u64];
                    invoke("operator_new", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_30"))?) == (0x0u64))) != 0
                {
                    1
                } else {
                    29
                };
            }
            31 => {
                values[11] = memory.read_u64(relocate("STACK_local_38"))?;
                pc = 30;
            }
            32 => {
                {
                    let call_arguments = [values[3]];
                    invoke("SelectionTool::D2DSelectionMask", &call_arguments, memory)
                };
                pc = 31;
            }
            33 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 31;
            }
            34 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = relocate("DAT_1401d0e30");
                    memory.write_u64(destination, stored)?;
                }
                pc = 33;
            }
            35 => {
                {
                    let call_arguments = [memory.read_u64((values[2]).wrapping_add(0x40u64))?];
                    invoke("SelectionTool::D2DSelectionMask", &call_arguments, memory)
                };
                pc = 31;
            }
            36 => {
                pc = if (values[6]) != 0 { 34 } else { 35 };
            }
            37 => {
                values[6] = u64::from(
                    ({
                        let call_arguments = [values[9]];
                        invoke("PGS::PGSRect::IsRectEmpty", &call_arguments, memory)
                    }) != 0,
                );
                pc = 36;
            }
            38 => {
                values[9] = {
                    let call_arguments = [];
                    invoke("PGS::PGSRect::Empty", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                values[9] = {
                    let call_arguments = [memory.read_u64((values[2]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::GetTrackerRect", &call_arguments, memory)
                };
                pc = 37;
            }
            40 => {
                pc = if (u64::from(
                    ((u64::from((values[4]) == (0x0u64))) != 0)
                        || (({
                            let _ = {
                                values[5] = ({
                                    let call_arguments = [
                                        memory.read_u64(
                                            (memory.read_u64(values[4])?).wrapping_add(0x68u64),
                                        )?,
                                        values[4],
                                        0x17u64,
                                    ];
                                    invoke("INDIRECT_CALL", &call_arguments, memory)
                                }) as i8 as i64 as u64;
                                values[5]
                            };
                            u64::from(
                                (((values[5]) as i32 as i64 as u64) as i64)
                                    == (((0x0u64) as i32 as i64 as u64) as i64),
                            )
                        }) != 0),
                )) != 0
                {
                    38
                } else {
                    39
                };
            }
            41 => {
                {
                    let call_arguments = [relocate("STACK_local_48")];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 31;
            }
            42 => {
                pc = if (u64::from(
                    (memory.read_u64(
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8)),
                    )?) != (0x0u64),
                )) != 0
                {
                    41
                } else {
                    31
                };
            }
            43 => {
                {
                    let destination = values[8];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = memory.read_u64(values[8])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = relocate("STACK_local_38");
                    let stored = relocate("DAT_1401d0e30");
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                values[8] = {
                    let call_arguments = [values[4]];
                    invoke("MaskBasedSelectionTool::D2DMask", &call_arguments, memory)
                };
                pc = 45;
            }
            47 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    40
                } else {
                    46
                };
            }
            48 => {
                values[4] = memory.read_u64((values[2]).wrapping_add(0x40u64))?;
                pc = 47;
            }
            49 => {
                values[5] = ({
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x68u64))?,
                        values[3],
                        0x1fu64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 48;
            }
            50 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x2f0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    32
                } else {
                    49
                };
            }
            51 => {
                values[3] = memory.read_u64((values[2]).wrapping_add(0x40u64))?;
                pc = 50;
            }
            52 => {
                values[7] = ({
                    let call_arguments = [memory.read_u64((values[2]).wrapping_add(0x40u64))?];
                    invoke("CPaintTool::Id", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 51;
            }
            53 => {
                {
                    let call_arguments = [relocate("DAT_1401d0e28")];
                    invoke("_Init_thread_footer", &call_arguments, memory)
                };
                pc = 52;
            }
            54 => {
                {
                    let destination = relocate("_DAT_1401d0e40");
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("uRam00000001401d0e38");
                    let stored = 0x3f80000000000000u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                {
                    let destination = relocate("_DAT_1401d0e30");
                    let stored = 0x3f800000u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 55;
            }
            57 => {
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
                                (u64::from(memory.read_u32(relocate("DAT_1401d0e28"))?)) as u32,
                            )) as u32,
                        )),
                    )) != 0)
                        && (({
                            let _ = {
                                let call_arguments = [relocate("DAT_1401d0e28")];
                                invoke("FUN_1400b03b8", &call_arguments, memory)
                            };
                            u64::from(
                                (u64::from(
                                    (u64::from(
                                        (u64::from(memory.read_u32(relocate("DAT_1401d0e28"))?))
                                            as u32,
                                    )) as u32,
                                )) == (u64::from(
                                    ((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as u32,
                                )),
                            )
                        }) != 0),
                )) != 0
                {
                    56
                } else {
                    52
                };
            }
            58 => {
                {
                    let destination =
                        (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8));
                    let stored = values[1];
                    memory.write_u64(destination, stored)?;
                }
                pc = 57;
            }
            59 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x30u64))?;
                pc = 58;
            }
            60 => {
                {
                    let destination = relocate("STACK_local_28");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_68"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_14009dc50` (`mspaint.c:136056`).
pub fn fun_14009dc50(
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
    let mut pc: i32 = 68;
    loop {
        match pc {
            0 => {
                return Ok(Some(values[12]));
            }
            1 => {
                {
                    let call_arguments =
                        [(relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(8))];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                values[12] = 0x1u64;
                pc = 1;
            }
            3 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0x8u64))?,
                        values[7],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    3
                } else {
                    2
                };
            }
            5 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 4;
            }
            6 => {
                {
                    let destination = values[3];
                    let stored = ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 5;
            }
            7 => {
                values[5] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 6;
            }
            8 => {
                values[3] = (values[7]).wrapping_add(0xcu64);
                pc = 7;
            }
            9 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 8;
            }
            10 => {
                {
                    let call_arguments = [memory.read_u64(memory.read_u64(values[7])?)?, values[7]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    ((((values[6]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    10
                } else {
                    2
                };
            }
            12 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 11;
            }
            13 => {
                {
                    let destination = values[4];
                    let stored = ((((memory.read_u64(values[4])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 12;
            }
            14 => {
                values[6] = memory.read_u64(values[4])?;
                pc = 13;
            }
            15 => {
                values[4] = (values[7]).wrapping_add((0x1u64).wrapping_mul(8));
                pc = 14;
            }
            16 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 15;
            }
            17 => {
                pc = if (u64::from((values[7]) != (0x0u64))) != 0 {
                    16
                } else {
                    2
                };
            }
            18 => {
                values[7] = (((u64::from(
                    (u64::from(memory.read_u32(relocate("STACK_uStack_cc"))?)) as u32,
                )) & (0xffffffffu64))
                    .wrapping_shl((0x20u64) as u32))
                    | ((u64::from(
                        (u64::from(memory.read_u32(relocate("STACK_local_d0"))?)) as u32,
                    )) & (0xffffffffu64));
                pc = 17;
            }
            19 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("Value::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 18;
            }
            20 => {
                {
                    let call_arguments = [relocate("STACK_local_a0")];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 19;
            }
            21 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("CAttributeValue<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 20;
            }
            22 => {
                {
                    let call_arguments = [values[8], relocate("STACK_local_b0")];
                    invoke("CAttribute::SetValue", &call_arguments, memory)
                };
                pc = 21;
            }
            23 => {
                {
                    let call_arguments = [values[10]];
                    invoke("FUN_1400054d0", &call_arguments, memory)
                };
                pc = 22;
            }
            24 => {
                {
                    let call_arguments = [relocate("STACK_local_a0"), values[10]];
                    invoke("FUN_1400199f0", &call_arguments, memory)
                };
                pc = 23;
            }
            25 => {
                {
                    let destination = relocate("STACK_local_c0");
                    let stored = values[10];
                    memory.write_u64(destination, stored)?;
                }
                pc = 24;
            }
            26 => {
                {
                    let destination = relocate("STACK_local_b0");
                    let stored = relocate("CAttributeValue<>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 25;
            }
            27 => {
                {
                    let destination = relocate("STACK_local_a8");
                    let stored = u64::from((0x6u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 26;
            }
            28 => {
                values[10] = {
                    let call_arguments = [
                        relocate("STACK_local_80"),
                        (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_1400199f0", &call_arguments, memory)
                };
                pc = 27;
            }
            29 => {
                values[8] = memory.read_u64(relocate("STACK_local_d8"))?;
                pc = 28;
            }
            30 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0b30"),
                        (relocate("STACK_local_d8")) as i32 as i64 as u64,
                    ];
                    invoke("CAttributeStore::Get", &call_arguments, memory)
                };
                pc = 29;
            }
            31 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(relocate("STACK_local_b8"))?)?)
                                .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(relocate("STACK_local_b8"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    (((values[5]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    30
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
                values[5] = ((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32
                    as i64 as u64;
                pc = 34;
            }
            36 => {
                values[3] = (memory.read_u64(relocate("STACK_local_b8"))?).wrapping_add(0xcu64);
                pc = 35;
            }
            37 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 36;
            }
            38 => {
                {
                    let call_arguments = [
                        memory.read_u64(
                            memory.read_u64(memory.read_u64(relocate("STACK_local_b8"))?)?,
                        )?,
                        memory.read_u64(relocate("STACK_local_b8"))?,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 37;
            }
            39 => {
                pc = if (u64::from(
                    ((((values[6]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    38
                } else {
                    30
                };
            }
            40 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 39;
            }
            41 => {
                {
                    let destination = values[7];
                    let stored = ((((memory.read_u64(values[7])?) as i32 as i64 as u64)
                        .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                        as i32 as i64 as u64) as i32 as i64 as u64;
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 40;
            }
            42 => {
                values[6] = memory.read_u64(values[7])?;
                pc = 41;
            }
            43 => {
                values[7] = (memory.read_u64(relocate("STACK_local_b8"))?)
                    .wrapping_add((0x1u64).wrapping_mul(8));
                pc = 42;
            }
            44 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 43;
            }
            45 => {
                pc = if (u64::from((memory.read_u64(relocate("STACK_local_b8"))?) != (0x0u64))) != 0
                {
                    44
                } else {
                    30
                };
            }
            46 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = relocate("Value::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let call_arguments = [
                        memory.read_u64(relocate("STACK_local_c0"))?,
                        relocate("STACK_local_d8"),
                    ];
                    invoke("CAttribute::SetValue", &call_arguments, memory)
                };
                pc = 46;
            }
            48 => {
                {
                    let destination = relocate("STACK_local_c8");
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
                pc = 47;
            }
            49 => {
                {
                    let destination = relocate("STACK_local_d8");
                    let stored = relocate("CAttributeValue<double,22>::vftable");
                    memory.write_u64(destination, stored)?;
                }
                pc = 48;
            }
            50 => {
                {
                    let destination = relocate("STACK_local_d0");
                    let stored = u64::from((0x16u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let call_arguments = [
                        relocate("DAT_1401d0b30"),
                        (relocate("STACK_local_c0")) as i32 as i64 as u64,
                    ];
                    invoke("CAttributeStore::Get", &call_arguments, memory)
                };
                pc = 50;
            }
            52 => {
                pc = 1;
            }
            53 => {
                values[12] = 0x0u64;
                pc = 52;
            }
            54 => {
                pc = if (u64::from(
                    ((u64::from(
                        (f64::from_bits(values[2]))
                            < (f64::from_bits(memory.read_u64(relocate("STACK_local_60"))?)),
                    )) != 0)
                        || ((u64::from(
                            (f64::from_bits(memory.read_u64(
                                (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)) < (f64::from_bits(values[2])),
                        )) != 0),
                )) != 0
                {
                    53
                } else {
                    51
                };
            }
            55 => {
                pc = if (values[9]) != 0 { 54 } else { 51 };
            }
            56 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [
                            relocate("DAT_1401d0b30"),
                            0x55u64,
                            relocate("STACK_local_60"),
                            relocate("STACK_local_58"),
                        ];
                        invoke("CAttributeStore::GetRange", &call_arguments, memory)
                    }) != 0,
                );
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [
                        (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(8)),
                        values[11],
                        values[13],
                    ];
                    invoke("FUN_1400053f0", &call_arguments, memory)
                };
                pc = 56;
            }
            58 => {
                values[11] = relocate("DAT_140105708");
                pc = 57;
            }
            59 => {
                values[13] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[6]).wrapping_add(0x4u64))?)) as u32,
                    )) as u32,
                );
                pc = 57;
            }
            60 => {
                values[11] = memory.read_u64((values[6]).wrapping_add(0x10u64))?;
                pc = 59;
            }
            61 => {
                pc = if (u64::from(((values[6]) as i64) == ((0x0u64) as i64))) != 0 {
                    58
                } else {
                    60
                };
            }
            62 => {
                values[6] = memory.read_u64(values[1])?;
                pc = 61;
            }
            63 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x4u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x3u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 63;
            }
            65 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 64;
            }
            66 => {
                {
                    let destination =
                        (relocate("STACK_local_58")).wrapping_add((0x1u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 65;
            }
            67 => {
                values[13] = u64::from((0x0u64) as u32);
                pc = 66;
            }
            68 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStack_f8"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 67;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400b2750` (`mspaint.c:158319`).
pub fn fun_1400b2750(
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
    ];
    let mut pc: i32 = 130;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x180u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0xcau64))?)) as i16
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
                    let call_arguments = [(values[0]).wrapping_add((0x11u64).wrapping_mul(8))];
                    invoke("FUN_140015d00", &call_arguments, memory)
                };
                pc = 2;
            }
            4 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x2u64).wrapping_mul(8))];
                    invoke("FUN_140008c50", &call_arguments, memory)
                };
                pc = 3;
            }
            5 => {
                pc = 4;
            }
            6 => {
                pc = 4;
            }
            7 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 6;
            }
            8 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1au64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    7
                } else {
                    6
                };
            }
            9 => {
                pc = if (u64::from(((values[3]) as i64) == ((0x1u64) as i64))) != 0 {
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
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 10;
            }
            12 => {
                values[3] = memory.read_u64(values[4])?;
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
                    let destination = relocate("STACK_local_98");
                    let stored = values[4];
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
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 16;
            }
            18 => {
                values[3] = memory.read_u64(values[4])?;
                pc = 17;
            }
            19 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 18;
            }
            20 => {
                values[4] = memory.read_u64((values[0]).wrapping_add((0x1bu64).wrapping_mul(8)))?;
                pc = 19;
            }
            21 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1bu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    20
                } else {
                    8
                };
            }
            22 => {
                pc = 4;
            }
            23 => {
                return Ok(None);
            }
            24 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    23
                } else {
                    22
                };
            }
            25 => {
                values[10] = ({
                    let call_arguments = [];
                    invoke("FUN_1400091c0", &call_arguments, memory)
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
                    let destination = values[1];
                    let stored = u64::from((0x0u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 26;
            }
            28 => {
                {
                    let destination = (values[0]).wrapping_add((0x24u64).wrapping_mul(8));
                    let stored = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    memory.write_u64(destination, stored)?;
                }
                pc = 27;
            }
            29 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1fu64).wrapping_mul(8))];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 28;
            }
            30 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1fu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    29
                } else {
                    28
                };
            }
            31 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x20u64).wrapping_mul(8))];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 30;
            }
            32 => {
                pc = if (u64::from(
                    ((memory.read_u64(relocate("STACK_local_60"))?) as i64) != ((0x0u64) as i64),
                )) != 0
                {
                    31
                } else {
                    30
                };
            }
            33 => {
                {
                    let destination = relocate("STACK_local_60");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x20u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 32;
            }
            34 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(memory.read_u64(values[2])?)?)?,
                        memory.read_u64(relocate("STACK_local_68"))?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 33;
            }
            35 => {
                {
                    let destination = relocate("STACK_local_68");
                    let stored = memory.read_u64(values[2])?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 34;
            }
            36 => {
                pc = if (u64::from(((memory.read_u64(values[2])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    35
                } else {
                    33
                };
            }
            37 => {
                {
                    let destination = (values[0]).wrapping_add((0xfu64).wrapping_mul(8));
                    let stored = u64::from((values[9]) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 36;
            }
            38 => {
                values[9] = u64::from(
                    ({
                        let call_arguments = [
                            values[3],
                            values[2],
                            0x2u64,
                            (values[0]).wrapping_add((0x21u64).wrapping_mul(8)),
                        ];
                        invoke("FUN_140017480", &call_arguments, memory)
                    }) as u8,
                );
                pc = 37;
            }
            39 => {
                {
                    let call_arguments = [
                        values[2],
                        (values[0]).wrapping_add((0x20u64).wrapping_mul(8)),
                        (values[3]).wrapping_add(0x20u64),
                        (values[3]).wrapping_add(0x28u64),
                        (values[3]).wrapping_add(0x30u64),
                        (((values[3]) as i32 as i64 as u64).wrapping_add(0x38u64)) as i32 as i64
                            as u64,
                        (((u64::from(
                            (u64::from(
                                memory.read_u32(relocate("STACK_in_stack_ffffffffffffff1c"))?,
                            )) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl((0x20u64) as u32))
                            | ((0x64u64) & (0xffffffffu64)),
                    ];
                    invoke("GdiplusHelpers::LoadStreamImage", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                values[3] = memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?;
                pc = 39;
            }
            41 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 40;
            }
            42 => {
                values[2] = (values[0]).wrapping_add((0x23u64).wrapping_mul(8));
                pc = 41;
            }
            43 => {
                {
                    let destination = (values[0]).wrapping_add((0x22u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 42;
            }
            44 => {
                {
                    let destination = (values[0]).wrapping_add((0x21u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[0]).wrapping_add((0x20u64).wrapping_mul(8));
                    let stored = memory.read_u64(relocate("STACK_local_70"))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let destination = relocate("STACK_local_70");
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x25u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 45;
            }
            47 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x26u64).wrapping_mul(8)),
                    ];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            48 => {
                pc = if (u64::from(
                    (((values[11]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    47
                } else {
                    46
                };
            }
            49 => {
                values[11] = ({
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add((0x1fu64).wrapping_mul(8)))?,
                        relocate("DAT_140108230"),
                        (values[0]).wrapping_add((0x25u64).wrapping_mul(8)),
                    ];
                    invoke(
                        "CreateStreamOverRandomAccessStream",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 48;
            }
            50 => {
                {
                    let destination = (values[0]).wrapping_add((0x28u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 49;
            }
            51 => {
                {
                    let destination = (values[0]).wrapping_add((0x27u64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 50;
            }
            52 => {
                {
                    let destination = (values[0]).wrapping_add((0x26u64).wrapping_mul(8));
                    let stored = u64::from((0xaa9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 51;
            }
            53 => {
                {
                    let destination = (values[0]).wrapping_add((0x25u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[0]).wrapping_add((0x1fu64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 53;
            }
            55 => {
                {
                    let destination = relocate("STACK_local_78");
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 54;
            }
            56 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x29u64).wrapping_mul(8)))?;
                pc = 55;
            }
            57 => {
                {
                    let call_arguments = [
                        values[11],
                        (values[0]).wrapping_add((0x2au64).wrapping_mul(8)),
                    ];
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
                        memory.read_u64(memory.read_u64(values[12])?)?,
                        values[12],
                        relocate("PTR_140108388"),
                        (values[0]).wrapping_add((0x29u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 58;
            }
            60 => {
                {
                    let destination = (values[0]).wrapping_add((0x2cu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                {
                    let destination = (values[0]).wrapping_add((0x2bu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 60;
            }
            62 => {
                {
                    let destination = (values[0]).wrapping_add((0x2au64).wrapping_mul(8));
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 61;
            }
            63 => {
                {
                    let destination = (values[0]).wrapping_add((0x29u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 62;
            }
            64 => {
                pc = if (u64::from((values[12]) != (0x0u64))) != 0 {
                    63
                } else {
                    54
                };
            }
            65 => {
                values[12] =
                    memory.read_u64((values[0]).wrapping_add((0x12u64).wrapping_mul(8)))?;
                pc = 64;
            }
            66 => {
                values[14] = 0x0u64;
                pc = 65;
            }
            67 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1au64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    67
                } else {
                    66
                };
            }
            69 => {
                pc = if (u64::from(((values[3]) as i64) == ((0x1u64) as i64))) != 0 {
                    74
                } else {
                    68
                };
            }
            70 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 69;
            }
            71 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 70;
            }
            72 => {
                values[3] = memory.read_u64(values[4])?;
                pc = 71;
            }
            73 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 72;
            }
            74 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 73;
            }
            75 => {
                {
                    let destination = relocate("STACK_local_98");
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
                pc = 69;
            }
            76 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 75;
            }
            77 => {
                {
                    let destination = values[4];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 76;
            }
            78 => {
                values[3] = memory.read_u64(values[4])?;
                pc = 77;
            }
            79 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 78;
            }
            80 => {
                values[4] = memory.read_u64((values[0]).wrapping_add((0x1bu64).wrapping_mul(8)))?;
                pc = 79;
            }
            81 => {
                pc = if (u64::from(
                    (memory.read_u64((values[0]).wrapping_add((0x1bu64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0
                {
                    80
                } else {
                    68
                };
            }
            82 => {
                {
                    let call_arguments = [(values[0]).wrapping_add((0x1bu64).wrapping_mul(8))];
                    invoke("FUN_140008b20", &call_arguments, memory)
                };
                pc = 81;
            }
            83 => {
                pc = 4;
            }
            84 => {
                pc = 28;
            }
            85 => {
                {
                    let destination = (values[0]).wrapping_add((0xfu64).wrapping_mul(8));
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 84;
            }
            86 => {
                pc = 66;
            }
            87 => {
                {
                    let destination = (values[3]).wrapping_add(0xa4cu64);
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (values[3]).wrapping_add(0xa44u64);
                    let stored =
                        memory.read_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                values[14] =
                    memory.read_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)))?;
                pc = 88;
            }
            90 => {
                {
                    let destination = (values[3]).wrapping_add(0xa40u64);
                    let stored = u64::from((values[8]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination = (values[3]).wrapping_add(0xa3cu64);
                    let stored = u64::from((values[7]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 90;
            }
            92 => {
                {
                    let destination = (values[3]).wrapping_add(0xa38u64);
                    let stored = u64::from((values[6]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 91;
            }
            93 => {
                {
                    let destination = (values[3]).wrapping_add(0xa34u64);
                    let stored =
                        u64::from(
                            (u64::from(
                                (u64::from(memory.read_u32(
                                    (values[0]).wrapping_add((0x14u64).wrapping_mul(8)),
                                )?)) as u32,
                            )) as u32,
                        );
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 92;
            }
            94 => {
                values[8] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0xacu64))?)) as u32,
                    )) as u32,
                );
                pc = 93;
            }
            95 => {
                values[7] = u64::from(
                    (u64::from(
                        (u64::from(
                            memory.read_u32((values[0]).wrapping_add((0x15u64).wrapping_mul(8)))?,
                        )) as u32,
                    )) as u32,
                );
                pc = 94;
            }
            96 => {
                values[6] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0xa4u64))?)) as u32,
                    )) as u32,
                );
                pc = 95;
            }
            97 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    96
                } else {
                    85
                };
            }
            98 => {
                values[10] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[12])?)?,
                        values[12],
                        (values[0]).wrapping_add((0x13u64).wrapping_mul(8)),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 97;
            }
            99 => {
                values[12] = {
                    let call_arguments = [values[3]];
                    invoke("FUN_140017300", &call_arguments, memory)
                };
                pc = 98;
            }
            100 => {
                pc = 82;
            }
            101 => {
                return Ok(None);
            }
            102 => {
                pc = if (u64::from(
                    (((values[10]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    101
                } else {
                    100
                };
            }
            103 => {
                values[10] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add((0x1bu64).wrapping_mul(8)),
                        values[0],
                    ];
                    invoke("FUN_140009170", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 102;
            }
            104 => {
                {
                    let destination = values[1];
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 103;
            }
            105 => {
                {
                    let destination = (values[0]).wrapping_add((0x1eu64).wrapping_mul(8));
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 104;
            }
            106 => {
                {
                    let destination = (values[0]).wrapping_add((0x1du64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 105;
            }
            107 => {
                {
                    let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
                    let stored = values[14];
                    memory.write_u64(destination, stored)?;
                }
                pc = 106;
            }
            108 => {
                {
                    let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 107;
            }
            109 => {
                {
                    let call_arguments = [relocate("STACK_local_b0"), relocate("DAT_14016a960")];
                    invoke("_CxxThrowException", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("_CxxThrowException"));
            }
            110 => {
                {
                    let call_arguments = [relocate("STACK_local_b0")];
                    invoke("FUN_14000aa50", &call_arguments, memory)
                };
                pc = 109;
            }
            111 => {
                {
                    let destination = (values[0]).wrapping_add((0x2fu64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                {
                    let destination = (values[0]).wrapping_add((0x2eu64).wrapping_mul(8));
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 111;
            }
            113 => {
                {
                    let destination = (values[0]).wrapping_add((0x2du64).wrapping_mul(8));
                    let stored = u64::from((0x1628u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 112;
            }
            114 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u32((values[0]).wrapping_add((0xeu64).wrapping_mul(8)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    113
                } else {
                    108
                };
            }
            115 => {
                values[14] = {
                    let call_arguments = [
                        values[13],
                        (values[0]).wrapping_add((0x1au64).wrapping_mul(8)),
                    ];
                    invoke(
                        "Paint::Provenance::ProvenanceHelper::InsertImageIngredientAsync",
                        &call_arguments,
                        memory,
                    )
                };
                pc = 114;
            }
            116 => {
                values[13] = {
                    let call_arguments = [values[3]];
                    invoke("FUN_140017300", &call_arguments, memory)
                };
                pc = 115;
            }
            117 => {
                pc = if (u64::from(
                    ((((u64::from(
                        memory.read_u8((values[0]).wrapping_add((0x18u64).wrapping_mul(8)))?,
                    )) as i8 as i64 as u64) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    116
                } else {
                    99
                };
            }
            118 => {
                values[3] = memory.read_u64((values[0]).wrapping_add((0x11u64).wrapping_mul(8)))?;
                pc = 117;
            }
            119 => {
                return Ok(None);
            }
            120 => {
                {
                    let call_arguments = [values[5]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 119;
            }
            121 => {
                values[5] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 120;
            }
            122 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        == (((0x1u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    121
                };
            }
            123 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    5
                } else {
                    122
                };
            }
            124 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    21
                } else {
                    123
                };
            }
            125 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    82
                } else {
                    124
                };
            }
            126 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    83
                } else {
                    125
                };
            }
            127 => {
                pc = if (u64::from(
                    (((values[15]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    118
                } else {
                    126
                };
            }
            128 => {
                values[15] = (u64::from((u64::from(memory.read_u16(values[1])?)) as u16)) as i32
                    as i64 as u64;
                pc = 127;
            }
            129 => {
                values[1] = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
                pc = 128;
            }
            130 => {
                {
                    let destination = relocate("STACK_local_30");
                    let stored = (memory.read_u64(relocate("DAT_140179040"))?)
                        ^ (relocate("STACK_auStackY_118"));
                    memory.write_u64(destination, stored)?;
                }
                pc = 129;
            }
            _ => unreachable!("invalid recovered control-flow block"),
        }
    }
}

/// Translation of `FUN_1400c85c0` (`mspaint.c:173833`).
pub fn fun_1400c85c0(
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
    ];
    let mut pc: i32 = 150;
    loop {
        match pc {
            0 => {
                return Ok(None);
            }
            1 => {
                {
                    let call_arguments = [values[0], 0x180u64];
                    invoke("FUN_1400b022c", &call_arguments, memory)
                };
                pc = 0;
            }
            2 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u16((values[0]).wrapping_add(0x62u64))?)) as i16
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
                    let call_arguments = [(values[0]).wrapping_add(0x18u64)];
                    invoke("FUN_14006a580", &call_arguments, memory)
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
                    ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    6
                } else {
                    5
                };
            }
            8 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 7;
            }
            9 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0xa0u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    8
                } else {
                    7
                };
            }
            10 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 9;
            }
            11 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0xa8u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    10
                } else {
                    9
                };
            }
            12 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 11;
            }
            13 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0xb0u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    12
                } else {
                    11
                };
            }
            14 => {
                pc = 76;
            }
            15 => {
                pc = 3;
            }
            16 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 15;
            }
            17 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    16
                } else {
                    15
                };
            }
            18 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 17;
            }
            19 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x78u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    18
                } else {
                    17
                };
            }
            20 => {
                pc = if (u64::from(((values[4]) as i64) == ((0x1u64) as i64))) != 0 {
                    25
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
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 21;
            }
            23 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 22;
            }
            24 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 23;
            }
            25 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 24;
            }
            26 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 20;
            }
            27 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 26;
            }
            28 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 27;
            }
            29 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 28;
            }
            30 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x80u64))?;
                pc = 29;
            }
            31 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x80u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    30
                } else {
                    19
                };
            }
            32 => {
                pc = 3;
            }
            33 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 32;
            }
            34 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    33
                } else {
                    32
                };
            }
            35 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 34;
            }
            36 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0xa0u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    35
                } else {
                    34
                };
            }
            37 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 36;
            }
            38 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0xa8u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    37
                } else {
                    36
                };
            }
            39 => {
                {
                    let call_arguments = [(values[0]).wrapping_add(0xb0u64)];
                    invoke("FUN_140006970", &call_arguments, memory)
                };
                pc = 38;
            }
            40 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0xb0u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    39
                } else {
                    38
                };
            }
            41 => {
                {
                    let call_arguments = [(values[0]).wrapping_add(0x138u64)];
                    invoke("FUN_14000cf80", &call_arguments, memory)
                };
                pc = 40;
            }
            42 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(memory.read_u64(values[1])?)?)?,
                        memory.read_u64(values[1])?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 41;
            }
            43 => {
                pc = if (u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    42
                } else {
                    41
                };
            }
            44 => {
                {
                    let call_arguments = [relocate("DAT_1401d0520"), 0x2u64, 0x3f4u64, 0x0u64];
                    invoke("FUN_14006f460", &call_arguments, memory)
                };
                pc = 43;
            }
            45 => {
                {
                    let destination = (values[0]).wrapping_add(0x170u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 44;
            }
            46 => {
                {
                    let call_arguments = [0x1u64];
                    invoke("FUN_140033a00", &call_arguments, memory)
                };
                pc = 41;
            }
            47 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    46
                } else {
                    41
                };
            }
            48 => {
                {
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(memory.read_u64(values[1])?)?)?,
                        memory.read_u64(values[1])?,
                        0x1u64,
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 47;
            }
            49 => {
                pc = if (u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)))
                    != 0
                {
                    48
                } else {
                    47
                };
            }
            50 => {
                {
                    let call_arguments = [values[15], values[14]];
                    invoke("FUN_1400177a0", &call_arguments, memory)
                };
                pc = 49;
            }
            51 => {
                values[15] = u64::from(
                    ({
                        let call_arguments = [values[4], 0x1u64];
                        invoke("FUN_1400153f0", &call_arguments, memory)
                    }) as u32,
                );
                pc = 50;
            }
            52 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    51
                } else {
                    49
                };
            }
            53 => {
                values[6] = ({
                    let call_arguments = [
                        values[4],
                        values[1],
                        0x2u64,
                        (values[0]).wrapping_add(0x140u64),
                    ];
                    invoke("FUN_140017480", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 52;
            }
            54 => {
                {
                    let destination = (values[4]).wrapping_add(0xa30u64);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 53;
            }
            55 => {
                pc = if (u64::from(((memory.read_u64(values[1])?) as i64) == ((0x0u64) as i64)))
                    != 0
                {
                    45
                } else {
                    54
                };
            }
            56 => {
                {
                    let call_arguments = [
                        values[1],
                        (values[0]).wrapping_add(0xb0u64),
                        (values[4]).wrapping_add(0x20u64),
                        (values[4]).wrapping_add(0x28u64),
                        (values[4]).wrapping_add(0x30u64),
                        (((values[4]) as i32 as i64 as u64).wrapping_add(0x38u64)) as i32 as i64
                            as u64,
                        (((u64::from(
                            (u64::from(
                                memory.read_u32(relocate("STACK_in_stack_fffffffffffffedc"))?,
                            )) as u32,
                        )) & (0xffffffffu64))
                            .wrapping_shl((0x20u64) as u32))
                            | ((0x64u64) & (0xffffffffu64)),
                    ];
                    invoke("GdiplusHelpers::LoadStreamImage", &call_arguments, memory)
                };
                pc = 55;
            }
            57 => {
                {
                    let destination = values[1];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 56;
            }
            58 => {
                values[1] = (values[0]).wrapping_add(0x150u64);
                pc = 57;
            }
            59 => {
                {
                    let destination = (values[0]).wrapping_add(0x148u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 58;
            }
            60 => {
                {
                    let destination = (values[0]).wrapping_add(0x140u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 59;
            }
            61 => {
                values[4] =
                    memory.read_u64((values[14]).wrapping_add((0x298u64).wrapping_mul(1)))?;
                pc = 60;
            }
            62 => {
                {
                    let call_arguments =
                        [memory.read_u64((values[13]).wrapping_add((0x8u64).wrapping_mul(1)))?];
                    invoke("CCmdTarget::BeginWaitCursor", &call_arguments, memory)
                };
                pc = 61;
            }
            63 => {
                values[13] = {
                    let call_arguments = [];
                    invoke("AfxGetModuleState", &call_arguments, memory)
                };
                pc = 62;
            }
            64 => {
                values[14] =
                    memory.read_u64((values[12]).wrapping_add((0xe8u64).wrapping_mul(1)))?;
                pc = 63;
            }
            65 => {
                pc = if (u64::from((values[12]) != (0x0u64))) != 0 {
                    64
                } else {
                    63
                };
            }
            66 => {
                values[12] = {
                    let call_arguments = [values[11]];
                    invoke("CFrameWnd::GetActiveView", &call_arguments, memory)
                };
                pc = 65;
            }
            67 => {
                values[11] = {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[10])?).wrapping_add(0xf8u64))?,
                        values[10],
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 66;
            }
            68 => {
                pc = if (u64::from((values[10]) != (0x0u64))) != 0 {
                    67
                } else {
                    66
                };
            }
            69 => {
                values[11] = values[14];
                pc = 68;
            }
            70 => {
                values[10] = {
                    let call_arguments = [];
                    invoke("AfxGetThread", &call_arguments, memory)
                };
                pc = 69;
            }
            71 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            72 => {
                pc = if (u64::from(
                    ((((u64::from(memory.read_u32((values[0]).wrapping_add(0xc0u64))?)) as i32
                        as i64 as u64) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    71
                } else {
                    70
                };
            }
            73 => {
                {
                    let destination = (values[0]).wrapping_add(0x130u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 72;
            }
            74 => {
                {
                    let destination = (values[0]).wrapping_add(0x128u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 73;
            }
            75 => {
                {
                    let destination = (values[0]).wrapping_add(0x120u64);
                    let stored = u64::from((0x240fu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 74;
            }
            76 => {
                values[14] = 0x0u64;
                pc = 75;
            }
            77 => {
                return Ok(None);
            }
            78 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    77
                } else {
                    76
                };
            }
            79 => {
                values[6] = ({
                    let call_arguments = [(values[0]).wrapping_add(0xb8u64), values[0]];
                    invoke("FUN_140032550", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 78;
            }
            80 => {
                {
                    let destination = (values[0]).wrapping_add(0x60u64);
                    let stored = u64::from((0x6u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 79;
            }
            81 => {
                {
                    let destination = (values[0]).wrapping_add(0xc0u64);
                    let stored = u64::from((0x0u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 80;
            }
            82 => {
                {
                    let destination = (values[0]).wrapping_add(0xb8u64);
                    let stored = (values[0]).wrapping_add(0x68u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 81;
            }
            83 => {
                {
                    let destination = (values[0]).wrapping_add(0xb0u64);
                    let stored = memory.read_u64((values[0]).wrapping_add(0x100u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 82;
            }
            84 => {
                {
                    let call_arguments = [values[8], (values[0]).wrapping_add(0x108u64)];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            85 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    84
                } else {
                    83
                };
            }
            86 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64((values[0]).wrapping_add(0xa8u64))?,
                        relocate("DAT_140108230"),
                        (values[0]).wrapping_add(0x100u64),
                    ];
                    invoke(
                        "CreateStreamOverRandomAccessStream",
                        &call_arguments,
                        memory,
                    )
                }) as i32 as i64 as u64;
                pc = 85;
            }
            87 => {
                {
                    let destination = (values[0]).wrapping_add(0x118u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 86;
            }
            88 => {
                {
                    let destination = (values[0]).wrapping_add(0x110u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 87;
            }
            89 => {
                {
                    let destination = (values[0]).wrapping_add(0x108u64);
                    let stored = u64::from((0xaa9u64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 88;
            }
            90 => {
                {
                    let destination = (values[0]).wrapping_add(0x100u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 89;
            }
            91 => {
                {
                    let destination = (values[0]).wrapping_add(0xa8u64);
                    let stored = values[9];
                    memory.write_u64(destination, stored)?;
                }
                pc = 90;
            }
            92 => {
                values[9] = memory.read_u64((values[0]).wrapping_add(0xe0u64))?;
                pc = 91;
            }
            93 => {
                {
                    let call_arguments = [values[8], (values[0]).wrapping_add(0xe8u64)];
                    invoke("FUN_140002540", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            94 => {
                pc = if (u64::from(
                    (((values[8]) as i32 as i64 as u64) as i64)
                        < (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    93
                } else {
                    92
                };
            }
            95 => {
                values[8] = ({
                    let call_arguments = [
                        memory.read_u64(memory.read_u64(values[3])?)?,
                        values[3],
                        relocate("PTR_140108388"),
                        (values[0]).wrapping_add(0xe0u64),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 94;
            }
            96 => {
                {
                    let destination = (values[0]).wrapping_add(0xf8u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 95;
            }
            97 => {
                {
                    let destination = (values[0]).wrapping_add(0xf0u64);
                    let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
                    memory.write_u64(destination, stored)?;
                }
                pc = 96;
            }
            98 => {
                {
                    let destination = (values[0]).wrapping_add(0xe8u64);
                    let stored = u64::from((0x84bu64) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 97;
            }
            99 => {
                {
                    let destination = (values[0]).wrapping_add(0xe0u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 98;
            }
            100 => {
                pc = if (u64::from((values[3]) != (0x0u64))) != 0 {
                    99
                } else {
                    91
                };
            }
            101 => {
                values[3] = memory.read_u64((values[0]).wrapping_add(0xa0u64))?;
                pc = 100;
            }
            102 => {
                {
                    let call_arguments = [];
                    invoke("FUN_140001b30", &call_arguments, memory)
                };
                pc = 101;
            }
            103 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x78u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    102
                } else {
                    101
                };
            }
            104 => {
                pc = if (u64::from(((values[4]) as i64) == ((0x1u64) as i64))) != 0 {
                    109
                } else {
                    103
                };
            }
            105 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 104;
            }
            106 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 105;
            }
            107 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 106;
            }
            108 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 107;
            }
            109 => {
                {
                    let call_arguments = [];
                    invoke("_Thrd_yield", &call_arguments, memory)
                };
                pc = 108;
            }
            110 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 104;
            }
            111 => {
                {
                    let destination = values[2];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 110;
            }
            112 => {
                values[4] = memory.read_u64(values[2])?;
                pc = 111;
            }
            113 => {
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                pc = 112;
            }
            114 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x80u64))?;
                pc = 113;
            }
            115 => {
                pc = if (u64::from(
                    ((memory.read_u64((values[0]).wrapping_add(0x80u64))?) as i64)
                        != ((0x0u64) as i64),
                )) != 0
                {
                    114
                } else {
                    103
                };
            }
            116 => {
                {
                    let call_arguments = [
                        (values[0]).wrapping_add(0x80u64),
                        (values[0]).wrapping_add(0xa0u64),
                    ];
                    invoke("FUN_140062e90", &call_arguments, memory)
                };
                pc = 115;
            }
            117 => {
                values[9] = 0x0u64;
                pc = 116;
            }
            118 => {
                pc = 3;
            }
            119 => {
                pc = 117;
            }
            120 => {
                return Ok(None);
            }
            121 => {
                pc = if (u64::from(
                    (((values[6]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    120
                } else {
                    119
                };
            }
            122 => {
                values[6] = ({
                    let call_arguments = [(values[0]).wrapping_add(0x80u64), values[0]];
                    invoke("thunk_FUN_140065160", &call_arguments, memory)
                }) as i8 as i64 as u64;
                pc = 121;
            }
            123 => {
                {
                    let destination = (values[0]).wrapping_add(0x60u64);
                    let stored = u64::from((0x4u64) as u16);
                    memory.write_u16(destination, stored as u16)?;
                }
                pc = 122;
            }
            124 => {
                {
                    let destination = (values[0]).wrapping_add(0x98u64);
                    let stored = u64::from((0x1u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
                pc = 123;
            }
            125 => {
                {
                    let destination = (values[0]).wrapping_add(0x90u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 124;
            }
            126 => {
                {
                    let destination = (values[0]).wrapping_add(0x88u64);
                    let stored = (values[0]).wrapping_add(0x78u64);
                    memory.write_u64(destination, stored)?;
                }
                pc = 125;
            }
            127 => {
                {
                    let destination = (values[0]).wrapping_add(0x80u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 126;
            }
            128 => {
                {
                    let call_arguments = [
                        memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x10u64))?,
                        values[2],
                        (values[0]).wrapping_add(0x78u64),
                    ];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 127;
            }
            129 => {
                {
                    let call_arguments = [];
                    invoke("std::_Xbad_function_call", &call_arguments, memory)
                };
                return Err(AdapterError::Halted("std::_Xbad_function_call"));
            }
            130 => {
                pc = if (u64::from((values[2]) == (0x0u64))) != 0 {
                    129
                } else {
                    128
                };
            }
            131 => {
                values[2] = memory.read_u64((values[0]).wrapping_add(0x58u64))?;
                pc = 130;
            }
            132 => {
                {
                    let destination = (values[0]).wrapping_add(0x70u64);
                    let stored = u64::from((values[15]) as u32);
                    memory.write_u32(destination, stored as u32)?;
                }
                pc = 131;
            }
            133 => {
                values[15] = u64::from(
                    (u64::from(
                        (u64::from(memory.read_u32((values[0]).wrapping_add(0xdcu64))?)) as u32,
                    )) as u32,
                );
                pc = 132;
            }
            134 => {
                values[15] = u64::from((0x1u64) as u32);
                pc = 132;
            }
            135 => {
                pc = if (u64::from(
                    (((values[7]) as i32 as i64 as u64) as i64)
                        == (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    133
                } else {
                    134
                };
            }
            136 => {
                values[7] = ({
                    let call_arguments = [
                        (values[0]).wrapping_add(0xdcu64),
                        (values[0]).wrapping_add(0xd8u64),
                    ];
                    invoke("CoGetApartmentType", &call_arguments, memory)
                }) as i32 as i64 as u64;
                pc = 135;
            }
            137 => {
                {
                    let destination = (values[0]).wrapping_add(0x68u64);
                    let stored = memory.read_u64((values[0]).wrapping_add(0xd0u64))?;
                    memory.write_u64(destination, stored)?;
                }
                pc = 136;
            }
            138 => {
                {
                    let call_arguments =
                        [relocate("DAT_140105fb0"), (values[0]).wrapping_add(0xd0u64)];
                    invoke("CoGetObjectContext", &call_arguments, memory)
                };
                pc = 137;
            }
            139 => {
                {
                    let destination = (values[0]).wrapping_add(0xd0u64);
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                pc = 138;
            }
            140 => {
                return Ok(None);
            }
            141 => {
                {
                    let call_arguments = [values[5]];
                    invoke("INDIRECT_CALL", &call_arguments, memory)
                };
                pc = 140;
            }
            142 => {
                values[5] = {
                    let call_arguments = [0x3u64];
                    invoke("swi", &call_arguments, memory)
                };
                pc = 141;
            }
            143 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0xffffu64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    4
                } else {
                    142
                };
            }
            144 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x7u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    13
                } else {
                    143
                };
            }
            145 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x6u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    14
                } else {
                    144
                };
            }
            146 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x5u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    31
                } else {
                    145
                };
            }
            147 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x4u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    117
                } else {
                    146
                };
            }
            148 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x3u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    118
                } else {
                    147
                };
            }
            149 => {
                pc = if (u64::from(
                    (((values[16]) as i32 as i64 as u64) as i64)
                        == (((0x2u64) as i32 as i64 as u64) as i64),
                )) != 0
                {
                    139
                } else {
                    148
                };
            }
            150 => {
                values[16] = (u64::from(
                    (u64::from(memory.read_u16((values[0]).wrapping_add(0x60u64))?)) as u16,
                )) as i32 as i64 as u64;
                pc = 149;
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
    ("FUN_1400034e0", 2, fun_1400034e0),
    ("FUN_1400045e0", 3, fun_1400045e0),
    ("FUN_14000a6c0", 3, fun_14000a6c0),
    ("FUN_1400136f0", 2, fun_1400136f0),
    ("FUN_140022030", 3, fun_140022030),
    ("FUN_140025500", 3, fun_140025500),
    ("FUN_140027140", 3, fun_140027140),
    ("FUN_14002fba0", 5, fun_14002fba0),
    ("FUN_140042b30", 3, fun_140042b30),
    ("FUN_140048ab0", 0, fun_140048ab0),
    ("FUN_140058810", 2, fun_140058810),
    ("FUN_14005a920", 3, fun_14005a920),
    ("FUN_14005c890", 1, fun_14005c890),
    ("FUN_140065c80", 2, fun_140065c80),
    ("FUN_140067520", 2, fun_140067520),
    ("FUN_14006a430", 2, fun_14006a430),
    ("FUN_14006d6e0", 1, fun_14006d6e0),
    ("FUN_140070390", 5, fun_140070390),
    ("FUN_140097aa0", 2, fun_140097aa0),
    ("FUN_14009dc50", 3, fun_14009dc50),
    ("FUN_1400b2750", 1, fun_1400b2750),
    ("FUN_1400c85c0", 1, fun_1400c85c0),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_1400034e0",
        &[
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
        ],
    ),
    ("FUN_1400045e0", &[("STACK_local_38", -56)]),
    (
        "FUN_14000a6c0",
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
        "FUN_1400136f0",
        &[
            ("STACK_auStack_178", -376),
            ("STACK_local_158", -344),
            ("STACK_local_150", -336),
            ("STACK_uStack_148", -328),
            ("STACK_local_138", -312),
            ("STACK_local_130", -304),
            ("STACK_local_128", -296),
            ("STACK_local_110", -272),
            ("STACK_local_108", -264),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c0", -192),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_90", -144),
            ("STACK_local_28", -40),
        ],
    ),
    ("FUN_140025500", &[("STACK_local_38", -56)]),
    (
        "FUN_14002fba0",
        &[
            ("STACK_auStack_118", -280),
            ("STACK_local_f8", -248),
            ("STACK_local_e8", -232),
            ("STACK_uStack_e0", -224),
            ("STACK_local_d8", -216),
            ("STACK_local_63", -99),
            ("STACK_local_58", -88),
        ],
    ),
    (
        "FUN_140048ab0",
        &[
            ("STACK_auStack_c8", -200),
            ("STACK_local_98", -152),
            ("STACK_uStack_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_pWStack_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_uStack_58", -88),
            ("STACK_local_50", -80),
            ("STACK_uStack_48", -72),
            ("STACK_local_40", -64),
            ("STACK_uStack_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_140058810",
        &[
            ("STACK_auStack_b8", -184),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_pcStack_80", -128),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_pvStack_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_uStack_44", -68),
            ("STACK_uStack_40", -64),
            ("STACK_uStack_3c", -60),
            ("STACK_local_38", -56),
        ],
    ),
    ("FUN_14005a920", &[("STACK_local_38", -56)]),
    (
        "FUN_14005c890",
        &[
            ("STACK_auStack_198", -408),
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
            ("STACK_local_124", -292),
            ("STACK_local_118", -280),
            ("STACK_local_f8", -248),
            ("STACK_local_f0", -240),
            ("STACK_local_e8", -232),
            ("STACK_local_e4", -228),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_local_c8", -200),
            ("STACK_local_c4", -196),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_a8", -168),
            ("STACK_local_a4", -164),
            ("STACK_local_98", -152),
            ("STACK_local_90", -144),
            ("STACK_local_88", -136),
            ("STACK_local_80", -128),
            ("STACK_local_78", -120),
            ("STACK_pcStack_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140067520",
        &[
            ("STACK_auStack_a8", -168),
            ("STACK_local_88", -136),
            ("STACK_local_38", -56),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14006a430",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_14006d6e0",
        &[
            ("STACK_auStackY_408", -1032),
            ("STACK_local_3d8", -984),
            ("STACK_local_3d0", -976),
            ("STACK_local_3c8", -968),
            ("STACK_local_3c0", -960),
            ("STACK_local_3b8", -952),
            ("STACK_local_378", -888),
            ("STACK_local_370", -880),
            ("STACK_local_368", -872),
            ("STACK_local_360", -864),
            ("STACK_pCStack_358", -856),
            ("STACK_local_350", -848),
            ("STACK_pwStack_348", -840),
            ("STACK_local_328", -808),
            ("STACK_local_318", -792),
            ("STACK_local_2e0", -736),
            ("STACK_local_2d8", -728),
            ("STACK_local_2c8", -712),
            ("STACK_local_2a8", -680),
            ("STACK_local_288", -648),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140070390",
        &[
            ("STACK_auStackY_408", -1032),
            ("STACK_local_3d8", -984),
            ("STACK_local_3d0", -976),
            ("STACK_uStack_3c8", -968),
            ("STACK_local_3c0", -960),
            ("STACK_local_3b8", -952),
            ("STACK_local_3b0", -944),
            ("STACK_local_3a8", -936),
            ("STACK_local_3a0", -928),
            ("STACK_local_398", -920),
            ("STACK_local_390", -912),
            ("STACK_plStack_388", -904),
            ("STACK_local_380", -896),
            ("STACK_local_378", -888),
            ("STACK_local_370", -880),
            ("STACK_local_368", -872),
            ("STACK_local_360", -864),
            ("STACK_local_358", -856),
            ("STACK_local_350", -848),
            ("STACK_local_348", -840),
            ("STACK_local_340", -832),
            ("STACK_local_338", -824),
            ("STACK_puStack_330", -816),
            ("STACK_local_328", -808),
            ("STACK_local_320", -800),
            ("STACK_local_318", -792),
            ("STACK_pCStack_310", -784),
            ("STACK_local_308", -776),
            ("STACK_local_300", -768),
            ("STACK_pCStack_2f8", -760),
            ("STACK_local_2f0", -752),
            ("STACK_local_2e8", -744),
            ("STACK_local_2e4", -740),
            ("STACK_local_2e0", -736),
            ("STACK_local_2d8", -728),
            ("STACK_local_2d0", -720),
            ("STACK_local_2c8", -712),
            ("STACK_puStack_2c0", -704),
            ("STACK_local_2b8", -696),
            ("STACK_local_2b0", -688),
            ("STACK_uStack_2a8", -680),
            ("STACK_local_2a0", -672),
            ("STACK_pwStack_298", -664),
            ("STACK_local_290", -656),
            ("STACK_local_288", -648),
            ("STACK_uStack_280", -640),
            ("STACK_local_278", -632),
            ("STACK_local_270", -624),
            ("STACK_local_268", -616),
            ("STACK_uStack_260", -608),
            ("STACK_local_258", -600),
            ("STACK_local_250", -592),
            ("STACK_local_248", -584),
            ("STACK_local_238", -568),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140097aa0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14009dc50",
        &[
            ("STACK_auStack_f8", -248),
            ("STACK_local_d8", -216),
            ("STACK_local_d0", -208),
            ("STACK_uStack_cc", -204),
            ("STACK_local_c8", -200),
            ("STACK_local_c0", -192),
            ("STACK_local_b8", -184),
            ("STACK_local_b0", -176),
            ("STACK_local_a8", -168),
            ("STACK_local_a0", -160),
            ("STACK_local_80", -128),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_30", -48),
        ],
    ),
    (
        "FUN_1400b2750",
        &[
            ("STACK_in_stack_ffffffffffffff1c", -228),
            ("STACK_auStackY_118", -280),
            ("STACK_local_b0", -176),
            ("STACK_local_98", -152),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_30", -48),
        ],
    ),
    (
        "FUN_1400c85c0",
        &[("STACK_in_stack_fffffffffffffedc", -292)],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_1400034e0",
        &[
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
        ],
    ),
    ("FUN_1400045e0", &[("STACK_local_38", 16)]),
    (
        "FUN_14000a6c0",
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
        "FUN_1400136f0",
        &[
            ("STACK_auStack_178", 32),
            ("STACK_local_158", 4),
            ("STACK_local_150", 8),
            ("STACK_uStack_148", 8),
            ("STACK_local_138", 8),
            ("STACK_local_130", 8),
            ("STACK_local_128", 24),
            ("STACK_local_110", 8),
            ("STACK_local_108", 16),
            ("STACK_local_f8", 8),
            ("STACK_local_f0", 8),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_a8", 8),
            ("STACK_local_a0", 4),
            ("STACK_local_90", 104),
            ("STACK_local_28", 8),
        ],
    ),
    ("FUN_140025500", &[("STACK_local_38", 16)]),
    (
        "FUN_14002fba0",
        &[
            ("STACK_auStack_118", 32),
            ("STACK_local_f8", 4),
            ("STACK_local_e8", 8),
            ("STACK_uStack_e0", 4),
            ("STACK_local_d8", 117),
            ("STACK_local_63", 11),
            ("STACK_local_58", 8),
        ],
    ),
    (
        "FUN_140048ab0",
        &[
            ("STACK_auStack_c8", 48),
            ("STACK_local_98", 8),
            ("STACK_uStack_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_pWStack_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_uStack_58", 8),
            ("STACK_local_50", 8),
            ("STACK_uStack_48", 8),
            ("STACK_local_40", 8),
            ("STACK_uStack_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_140058810",
        &[
            ("STACK_auStack_b8", 32),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_pcStack_80", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_pvStack_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 4),
            ("STACK_uStack_44", 4),
            ("STACK_uStack_40", 4),
            ("STACK_uStack_3c", 4),
            ("STACK_local_38", 8),
        ],
    ),
    ("FUN_14005a920", &[("STACK_local_38", 16)]),
    (
        "FUN_14005c890",
        &[
            ("STACK_auStack_198", 32),
            ("STACK_local_178", 4),
            ("STACK_local_170", 4),
            ("STACK_local_168", 8),
            ("STACK_local_160", 8),
            ("STACK_local_158", 8),
            ("STACK_local_150", 8),
            ("STACK_local_148", 8),
            ("STACK_local_140", 8),
            ("STACK_local_138", 8),
            ("STACK_local_130", 8),
            ("STACK_local_128", 4),
            ("STACK_local_124", 4),
            ("STACK_local_118", 8),
            ("STACK_local_f8", 8),
            ("STACK_local_f0", 8),
            ("STACK_local_e8", 4),
            ("STACK_local_e4", 4),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 8),
            ("STACK_local_c8", 4),
            ("STACK_local_c4", 4),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 4),
            ("STACK_local_a4", 4),
            ("STACK_local_98", 8),
            ("STACK_local_90", 8),
            ("STACK_local_88", 8),
            ("STACK_local_80", 8),
            ("STACK_local_78", 8),
            ("STACK_pcStack_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 16),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140067520",
        &[
            ("STACK_auStack_a8", 32),
            ("STACK_local_88", 72),
            ("STACK_local_38", 16),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14006a430",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_20", 8),
        ],
    ),
    (
        "FUN_14006d6e0",
        &[
            ("STACK_auStackY_408", 32),
            ("STACK_local_3d8", 8),
            ("STACK_local_3d0", 8),
            ("STACK_local_3c8", 8),
            ("STACK_local_3c0", 8),
            ("STACK_local_3b8", 64),
            ("STACK_local_378", 8),
            ("STACK_local_370", 1),
            ("STACK_local_368", 8),
            ("STACK_local_360", 8),
            ("STACK_pCStack_358", 8),
            ("STACK_local_350", 8),
            ("STACK_pwStack_348", 8),
            ("STACK_local_328", 8),
            ("STACK_local_318", 56),
            ("STACK_local_2e0", 8),
            ("STACK_local_2d8", 1),
            ("STACK_local_2c8", 32),
            ("STACK_local_2a8", 32),
            ("STACK_local_288", 592),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140070390",
        &[
            ("STACK_auStackY_408", 32),
            ("STACK_local_3d8", 8),
            ("STACK_local_3d0", 8),
            ("STACK_uStack_3c8", 8),
            ("STACK_local_3c0", 8),
            ("STACK_local_3b8", 8),
            ("STACK_local_3b0", 8),
            ("STACK_local_3a8", 8),
            ("STACK_local_3a0", 8),
            ("STACK_local_398", 8),
            ("STACK_local_390", 8),
            ("STACK_plStack_388", 8),
            ("STACK_local_380", 8),
            ("STACK_local_378", 8),
            ("STACK_local_370", 8),
            ("STACK_local_368", 8),
            ("STACK_local_360", 8),
            ("STACK_local_358", 8),
            ("STACK_local_350", 8),
            ("STACK_local_348", 8),
            ("STACK_local_340", 8),
            ("STACK_local_338", 8),
            ("STACK_puStack_330", 8),
            ("STACK_local_328", 8),
            ("STACK_local_320", 8),
            ("STACK_local_318", 8),
            ("STACK_pCStack_310", 8),
            ("STACK_local_308", 8),
            ("STACK_local_300", 8),
            ("STACK_pCStack_2f8", 8),
            ("STACK_local_2f0", 8),
            ("STACK_local_2e8", 4),
            ("STACK_local_2e4", 4),
            ("STACK_local_2e0", 8),
            ("STACK_local_2d8", 8),
            ("STACK_local_2d0", 8),
            ("STACK_local_2c8", 8),
            ("STACK_puStack_2c0", 8),
            ("STACK_local_2b8", 8),
            ("STACK_local_2b0", 8),
            ("STACK_uStack_2a8", 8),
            ("STACK_local_2a0", 8),
            ("STACK_pwStack_298", 8),
            ("STACK_local_290", 8),
            ("STACK_local_288", 8),
            ("STACK_uStack_280", 8),
            ("STACK_local_278", 8),
            ("STACK_local_270", 8),
            ("STACK_local_268", 8),
            ("STACK_uStack_260", 8),
            ("STACK_local_258", 8),
            ("STACK_local_250", 8),
            ("STACK_local_248", 16),
            ("STACK_local_238", 512),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140097aa0",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 16),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14009dc50",
        &[
            ("STACK_auStack_f8", 32),
            ("STACK_local_d8", 8),
            ("STACK_local_d0", 4),
            ("STACK_uStack_cc", 4),
            ("STACK_local_c8", 8),
            ("STACK_local_c0", 8),
            ("STACK_local_b8", 8),
            ("STACK_local_b0", 8),
            ("STACK_local_a8", 4),
            ("STACK_local_a0", 32),
            ("STACK_local_80", 32),
            ("STACK_local_60", 8),
            ("STACK_local_58", 40),
            ("STACK_local_30", 8),
        ],
    ),
    (
        "FUN_1400b2750",
        &[
            ("STACK_in_stack_ffffffffffffff1c", 4),
            ("STACK_auStackY_118", 32),
            ("STACK_local_b0", 24),
            ("STACK_local_98", 8),
            ("STACK_local_78", 8),
            ("STACK_local_70", 8),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_30", 8),
        ],
    ),
    ("FUN_1400c85c0", &[("STACK_in_stack_fffffffffffffedc", 4)]),
];
