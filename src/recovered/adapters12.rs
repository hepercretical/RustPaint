//! Checked translations generated from original bodies by batch 012.
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
}
impl From<MemoryFault> for AdapterError {
    fn from(fault: MemoryFault) -> Self {
        Self::Memory(fault)
    }
}
use super::call_support::{MemoryImage, MissingArguments, RecordCall, RelocateCall};
/// Translation of `FUN_140001df0` (`mspaint.c:5920`).
pub fn fun_140001df0(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = values[0];
        let stored = relocate("std::exception::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__std_exception_copy",
        &[relocate("STACK_local_28")],
        memory,
    );
    Ok(Some(values[0]))
}

/// Translation of `FUN_140001e50` (`mspaint.c:5942`).
pub fn fun_140001e50(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = values[0];
        let stored = relocate("std::exception::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("\"length\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__std_exception_copy",
        &[relocate("STACK_local_28")],
        memory,
    );
    {
        let destination = values[0];
        let stored = relocate("std::invalid_argument::vftable");
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140002ca0` (`mspaint.c:6740`).
pub fn fun_140002ca0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64];
    if u64::from(
        ((memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i64)
            == ((0xau64) as i64),
    ) != 0
    {
        values[4] = (memory.read_u64(memory.read_u64(values[2])?)?)
            .wrapping_add(0u64.wrapping_sub(0x756f72676b636142u64));
        if u64::from(((values[4]) as i64) == ((0x0u64) as i64)) != 0 {
            values[4] = (u64::from(
                (u64::from(memory.read_u16(
                    (memory.read_u64(values[2])?).wrapping_add((0x1u64).wrapping_mul(8)),
                )?)) as u16,
            ))
            .wrapping_sub(0x646eu64);
        }
        if u64::from(((values[4]) as i64) == ((0x0u64) as i64)) != 0 {
            invoke("FUN_1400029b0", &[values[0], 0x1u64], memory);
            return Ok(None);
        }
    }
    if u64::from(
        ((memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i64)
            == ((0x13u64) as i64),
    ) != 0
    {
        values[3] = (invoke(
            "memcmp",
            &[
                memory.read_u64(values[2])?,
                relocate("\"IsBackgroundVisible\""),
                0x13u64,
            ],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke("FUN_1400029b0", &[values[0], 0x0u64, 0x1u64], memory);
            return Ok(None);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140002fc0` (`mspaint.c:6998`).
pub fn fun_140002fc0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x48u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400036a0",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140003830` (`mspaint.c:7623`).
pub fn fun_140003830(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64, 0u64];
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x38u64))?,
            values[1],
        ],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from((values[4]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                values[4],
            ],
            memory,
        );
        values[5] = values[4];
        {
            let destination = relocate("STACK_local_50");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    {
        let destination = (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xb8eu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_40");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Data.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[5],
            memory.read_u64(values[2])?,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((values[5]) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_50")], memory);
        }
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_48")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140003b60` (`mspaint.c:7832`).
pub fn fun_140003b60(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x20u64))?,
            values[0],
            relocate("STACK_local_28"),
        ],
        memory,
    );
    if u64::from(
        (u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            )) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[2];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    } else {
        {
            let destination = values[1];
            let stored = u64::from(
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        values[3] = invoke(
            "CoTaskMemAlloc",
            &[(u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            ))
            .wrapping_shl(0x4u64 as u32)],
            memory,
        );
        {
            let destination = values[2];
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        if u64::from((values[3]) == (0x0u64)) != 0 {
            return Ok(Some(0x8007000eu64));
        }
        invoke(
            "memmove",
            &[
                values[3],
                memory.read_u64(relocate("STACK_local_20"))?,
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                ))
                .wrapping_shl(0x4u64 as u32),
            ],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400066d0` (`mspaint.c:10463`).
pub fn fun_1400066d0(
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
    let values = [arguments[0], u64::from((arguments[1]) as u8)];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored = relocate("Paint::Utilities::CanvasModifiedEvent::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = u64::from((0x1eu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((values[1]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
            values[0],
            relocate("STACK_local_38"),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140006730` (`mspaint.c:10487`).
pub fn fun_140006730(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored = relocate("Paint::Utilities::OperationEvent::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = u64::from((0x14u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
            values[0],
            relocate("STACK_local_38"),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400097e0` (`mspaint.c:13089`).
pub fn fun_1400097e0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_140009f20",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_140009eb0",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_14000a160` (`mspaint.c:13649`).
pub fn fun_14000a160(
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
    {
        let destination = relocate("STACK_local_res8");
        let stored = memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_res8"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_res8"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    {
        let destination = relocate("STACK_local_res10");
        let stored = relocate("STACK_local_res8");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_res18");
        let stored = invoke("operator_new", &[0x30u64], memory);
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(relocate("STACK_local_res8"))?;
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?)
            .wrapping_add((0x1u64).wrapping_mul(8));
        let stored = relocate("FUN_1400b2180");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = memory.read_u64(relocate("STACK_local_res18"))?;
        let stored = relocate("FUN_1400b2190");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_res8");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?)
            .wrapping_add((0x3u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?)
            .wrapping_add((0x4u64).wrapping_mul(8));
        let stored = u64::from((0x10002u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?).wrapping_add(0x11u64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    invoke(
        "FUN_1400b2190",
        &[memory.read_u64(relocate("STACK_local_res18"))?],
        memory,
    );
    if u64::from((memory.read_u64(relocate("STACK_local_res8"))?) != (0x0u64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_res8")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_14000ab00` (`mspaint.c:14236`).
pub fn fun_14000ab00(
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
    {
        let destination = relocate("STACK_local_18");
        let stored =
            (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_458"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[0];
        let stored = relocate("CGdiObject::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke("CGdiObject::DeleteObject", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_14000abc0` (`mspaint.c:14269`).
pub fn fun_14000abc0(
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
    let mut values = [arguments[0], 0u64, 0u64, 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "GdipCloneImage",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[0]).wrapping_add(0x10u64);
            let stored = (values[2]) as i32 as i64 as u64;
            memory.write_u32(destination, stored as u32)?;
        }
    }
    values[3] = invoke("GdipAlloc", &[], memory);
    if u64::from((values[3]) != (0x0u64)) != 0 {
        {
            let destination = values[3];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[1] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        {
            let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = values[3];
            let stored = relocate("Gdiplus::Image::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14000b480` (`mspaint.c:14783`).
pub fn fun_14000b480(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x48u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400036a0",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000b9d0` (`mspaint.c:15111`).
pub fn fun_14000b9d0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = memory.read_u64((values[0]).wrapping_add(0x10u64))?;
    values[3] = 0x0u64;
    if u64::from((values[2]) == (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_38");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    } else {
        {
            let destination = relocate("STACK_local_30");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x18u64))?,
                values[2],
                relocate("DAT_140106dc0"),
                relocate("STACK_local_30"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_30"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            values[3] = (memory.read_u64(relocate("STACK_local_30"))?)
                .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
            {
                let destination = relocate("STACK_local_30");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        {
            let destination = relocate("STACK_local_38");
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        if u64::from(((memory.read_u64(relocate("STACK_local_30"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_30")], memory);
        }
    }
    if u64::from(((values[3]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((values[0]).wrapping_add(0x18u64))?,
                ((u64::from(memory.read_u32((values[0]).wrapping_add(0x20u64))?)) as i32 as i64
                    as u64)
                    .wrapping_add(values[3]),
                u64::from((u64::from(memory.read_u32(values[1])?)) as u32),
            ],
            memory,
        );
        invoke("FUN_140004100", &[relocate("STACK_local_38")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_14000baa0` (`mspaint.c:15151`).
pub fn fun_14000baa0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = memory.read_u64((values[0]).wrapping_add(0x10u64))?;
    values[3] = 0x0u64;
    if u64::from((values[2]) == (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_38");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    } else {
        {
            let destination = relocate("STACK_local_30");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x18u64))?,
                values[2],
                relocate("DAT_140106dc0"),
                relocate("STACK_local_30"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_30"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            values[3] = (memory.read_u64(relocate("STACK_local_30"))?)
                .wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
            {
                let destination = relocate("STACK_local_30");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        {
            let destination = relocate("STACK_local_38");
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        if u64::from(((memory.read_u64(relocate("STACK_local_30"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_30")], memory);
        }
    }
    if u64::from(((values[3]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((values[0]).wrapping_add(0x18u64))?,
                ((u64::from(memory.read_u32((values[0]).wrapping_add(0x20u64))?)) as i32 as i64
                    as u64)
                    .wrapping_add(values[3]),
                memory.read_u64(values[1])?,
            ],
            memory,
        );
        invoke("FUN_140004100", &[relocate("STACK_local_38")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_14000beb0` (`mspaint.c:15382`).
pub fn fun_14000beb0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Dispatching.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x1b6u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14000c880` (`mspaint.c:15978`).
pub fn fun_14000c880(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_14000cce0",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_14000cc70",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_14000d0a0` (`mspaint.c:16462`).
pub fn fun_14000d0a0(
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
    ];
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = 0x0u64;
    values[6] = (values[0]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[6] = values[4];
    }
    values[5] = values[4];
    if u64::from(
        ((values[6]) as i64) != (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[5] = values[6];
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64((values[5]).wrapping_add(0x10u64))?).wrapping_add(0x20u64),
            )?,
            (values[5]).wrapping_add(0x10u64),
            relocate("STACK_local_38"),
        ],
        memory,
    );
    if u64::from(
        (u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            )) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[2];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    } else {
        {
            let destination = values[1];
            let stored = u64::from(
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        values[3] = invoke(
            "CoTaskMemAlloc",
            &[(u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            ))
            .wrapping_shl(0x4u64 as u32)],
            memory,
        );
        {
            let destination = values[2];
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        if u64::from((values[3]) == (0x0u64)) != 0 {
            values[4] = 0x8007000eu64;
        } else {
            invoke(
                "memmove",
                &[
                    values[3],
                    memory.read_u64(relocate("STACK_local_30"))?,
                    (u64::from(
                        (u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                        )?)) as u32,
                    ))
                    .wrapping_shl(0x4u64 as u32),
                ],
                memory,
            );
        }
    }
    Ok(Some(values[4]))
}

/// Translation of `FUN_140017d00` (`mspaint.c:25241`).
pub fn fun_140017d00(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_140108298"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140018690",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140018220` (`mspaint.c:25510`).
pub fn fun_140018220(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_140105598"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140018690",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140018550` (`mspaint.c:25741`).
pub fn fun_140018550(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((values[0]).wrapping_add(0x18u64))?,
            memory.read_u64((values[0]).wrapping_add(0x10u64))?,
            relocate("STACK_local_18"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140018730` (`mspaint.c:25873`).
pub fn fun_140018730(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke("FUN_1400186c0", &[relocate("STACK_local_28")], memory);
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_28"), relocate("DAT_14016ab98")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400188e0` (`mspaint.c:25967`).
pub fn fun_1400188e0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Storage.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x283u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140018a60` (`mspaint.c:26056`).
pub fn fun_140018a60(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = invoke("FUN_140018a40", &[relocate("STACK_local_48")], memory);
    {
        let destination = relocate("STACK_local_58");
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[0])?)) as u32)) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_54");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[0]).wrapping_add((0x1u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_50");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[0]).wrapping_add((0x2u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_4c");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[0]).wrapping_add((0x3u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_140018bf0",
        &[relocate("STACK_local_38"), relocate("STACK_local_58")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_38"), relocate("DAT_14016abe0")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140018ef0` (`mspaint.c:26314`).
pub fn fun_140018ef0(
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
    let mut values = [u64::from((arguments[0]) as u32), 0u64];
    values[1] = invoke(
        "FUN_140018ed0",
        &[relocate("STACK_local_48"), values[0]],
        memory,
    );
    {
        let destination = relocate("STACK_local_58");
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[1])?)) as u32)) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_54");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x1u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_50");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x2u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_4c");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x3u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_140018bf0",
        &[relocate("STACK_local_38"), relocate("STACK_local_58")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_38"), relocate("DAT_14016abe0")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140019d80` (`mspaint.c:27031`).
pub fn fun_140019d80(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_c8");
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[1])?)) as u32)) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_c4");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x1u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_c0");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x2u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_uStack_bc");
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x3u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_140019900",
        &[relocate("STACK_local_b8"), relocate("\"exists\"")],
        memory,
    );
    invoke(
        "FUN_140019650",
        &[
            relocate("STACK_local_98"),
            relocate("STACK_local_b8"),
            values[2],
            relocate("STACK_local_c8"),
        ],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_98"), relocate("DAT_14016acd0")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_14001a000` (`mspaint.c:27155`).
pub fn fun_14001a000(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = invoke("GdipAlloc", &[0x18u64], memory);
    if u64::from((values[3]) != (0x0u64)) != 0 {
        {
            let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = values[3];
            let stored = relocate("Gdiplus::Bitmap::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[2] = u64::from(
            (invoke(
                "GdipCreateBitmapFromHBITMAP",
                &[values[0], values[1], relocate("STACK_local_18")],
                memory,
            )) as u32,
        );
        {
            let destination = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[2]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(values[3]))
}

/// Translation of `FUN_14001c070` (`mspaint.c:28775`).
pub fn fun_14001c070(
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
    ];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[5] = (values[0]).wrapping_add((0u64.wrapping_sub(0x40u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[4] = u64::from(
        (invoke(
            "FUN_14001ed80",
            &[values[5], relocate("STACK_local_18")],
            memory,
        )) as u8,
    );
    {
        let destination = values[3];
        let stored = u64::from((values[4]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001c320` (`mspaint.c:29000`).
pub fn fun_14001c320(
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
    ];
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_10");
        let stored = u64::from((values[1]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_14001ea90",
        &[values[3], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001c6f0` (`mspaint.c:29212`).
pub fn fun_14001c6f0(
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
    ];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[5] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[4] = u64::from(
        (invoke(
            "FUN_14001ed80",
            &[values[5], relocate("STACK_local_18")],
            memory,
        )) as u8,
    );
    {
        let destination = values[3];
        let stored = u64::from((values[4]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d250` (`mspaint.c:30054`).
pub fn fun_14001d250(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14001e5b0",
        &[values[2], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d2d0` (`mspaint.c:30091`).
pub fn fun_14001d2d0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14001f000",
        &[values[3], values[1], relocate("STACK_local_18")],
        memory,
    );
    invoke(
        "FUN_14001f1a0",
        &[values[3], 0x1u64, (values[1]) & (0xffffffffu64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d340` (`mspaint.c:30114`).
pub fn fun_14001d340(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14001e7c0",
        &[values[3], values[1], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d5f0` (`mspaint.c:30294`).
pub fn fun_14001d5f0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add(0x40u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = invoke(
        "FUN_14001e900",
        &[
            values[4],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(values[3])?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001db10` (`mspaint.c:30564`).
pub fn fun_14001db10(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64, 0u64];
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = 0x0u64;
    values[5] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = values[4];
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[5])?).wrapping_add(0x20u64))?,
            values[5],
            relocate("STACK_local_38"),
        ],
        memory,
    );
    if u64::from(
        (u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            )) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[2];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    } else {
        {
            let destination = values[1];
            let stored = u64::from(
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        values[3] = invoke(
            "CoTaskMemAlloc",
            &[(u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            ))
            .wrapping_shl(0x4u64 as u32)],
            memory,
        );
        {
            let destination = values[2];
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        if u64::from((values[3]) == (0x0u64)) != 0 {
            values[4] = 0x8007000eu64;
        } else {
            invoke(
                "memmove",
                &[
                    values[3],
                    memory.read_u64(relocate("STACK_local_30"))?,
                    (u64::from(
                        (u64::from(memory.read_u32(
                            (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4)),
                        )?)) as u32,
                    ))
                    .wrapping_shl(0x4u64 as u32),
                ],
                memory,
            );
        }
    }
    Ok(Some(values[4]))
}

/// Translation of `FUN_14001ff10` (`mspaint.c:32543`).
pub fn fun_14001ff10(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140109460"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140020720` (`mspaint.c:32961`).
pub fn fun_140020720(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140109508"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140022cf0` (`mspaint.c:34387`).
pub fn fun_140022cf0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140022970",
        &[values[2], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140022d40` (`mspaint.c:34409`).
pub fn fun_140022d40(
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
    values[4] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000020");
    }
    values[2] = memory.read_u64(values[4])?;
    {
        let destination = (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = (values[2]).wrapping_add(0xe8u64);
    values[3] = (invoke(
        "FUN_140001a20",
        &[values[1], relocate("STACK_local_18")],
        memory,
    )) as i8 as i64 as u64;
    values[4] =
        memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((values[1]) != (relocate("STACK_local_18"))) != 0 {
            if u64::from((memory.read_u64(values[1])?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[values[1]], memory);
            }
            values[4] = memory
                .read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
            {
                let destination = values[1];
                let stored = memory.read_u64(
                    (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?;
                memory.write_u64(destination, stored)?;
            }
            if u64::from(
                (memory.read_u64(
                    (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                )?) != (0x0u64),
            ) != 0
            {
                invoke(
                    "INDIRECT_CALL",
                    &[
                        memory.read_u64(
                            (memory.read_u64(memory.read_u64(
                                (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                            )?)?)
                            .wrapping_add(0x8u64),
                        )?,
                        memory.read_u64(
                            (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)),
                        )?,
                    ],
                    memory,
                );
            }
        }
        invoke(
            "FUN_1400572d0",
            &[
                (values[2]).wrapping_add(0x28u64),
                relocate("PTR_s_AiInformation_1401121a0"),
            ],
            memory,
        );
    }
    if u64::from((values[4]) != (0x0u64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140023010` (`mspaint.c:34549`).
pub fn fun_140023010(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000018");
    }
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x455bu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x98u64))?,
            memory.read_u64(values[3])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = memory.read_u64(relocate("STACK_local_20"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_38")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140023640` (`mspaint.c:34881`).
pub fn fun_140023640(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = values[0];
        memory.write_u64(destination, stored)?;
    }
    invoke("FUN_14000bf40", &[], memory);
    values[1] = memory.read_u64(values[1])?;
    {
        let destination = (values[0]).wrapping_add(0x8u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?],
            memory,
        );
    }
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x6832u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?)
                    .wrapping_add(0x88u64),
            )?,
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            0x0u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140031f40` (`mspaint.c:46515`).
pub fn fun_140031f40(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = values[0];
        let stored = relocate("std::exception::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__std_exception_copy",
        &[relocate("STACK_local_28")],
        memory,
    );
    {
        let destination = values[0];
        let stored = relocate("std::runtime_error::vftable");
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140031fb0` (`mspaint.c:46538`).
pub fn fun_140031fb0(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = values[0];
        let stored = relocate("std::exception::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__std_exception_copy",
        &[relocate("STACK_local_28")],
        memory,
    );
    {
        let destination = values[0];
        let stored = relocate("std::format_error::vftable");
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140032550` (`mspaint.c:46825`).
pub fn fun_140032550(
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
    values[2] = memory.read_u64(values[0])?;
    {
        let destination = relocate("STACK_local_18");
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_18"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_18"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = u64::from(
            (u64::from(
                (memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?) as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    values[3] = invoke(
        "FUN_14000a6c0",
        &[
            relocate("STACK_local_18"),
            values[1],
            (values[0]).wrapping_add((0x1u64).wrapping_mul(8)),
        ],
        memory,
    );
    if u64::from((memory.read_u64(relocate("STACK_local_18"))?) != (0x0u64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        values[3] &= 0xffu64;
    }
    Ok(Some(values[3]))
}

/// Translation of `FUN_140033700` (`mspaint.c:47842`).
pub fn fun_140033700(
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
    {
        let destination = relocate("STACK_local_res8");
        let stored = memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_res8"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_res8"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    {
        let destination = relocate("STACK_local_res10");
        let stored = relocate("STACK_local_res8");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_res18");
        let stored = invoke("operator_new", &[0x70u64], memory);
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(relocate("STACK_local_res8"))?;
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?)
            .wrapping_add((0x1u64).wrapping_mul(8));
        let stored = relocate("FUN_1400b67b0");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = memory.read_u64(relocate("STACK_local_res18"))?;
        let stored = relocate("FUN_1400b67c0");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_res8");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?)
            .wrapping_add((0x3u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?)
            .wrapping_add((0x4u64).wrapping_mul(8));
        let stored = u64::from((0x10002u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_res18"))?).wrapping_add(0x11u64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    invoke(
        "FUN_1400b67c0",
        &[memory.read_u64(relocate("STACK_local_res18"))?],
        memory,
    );
    if u64::from((memory.read_u64(relocate("STACK_local_res8"))?) != (0x0u64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_res8")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140033970` (`mspaint.c:47967`).
pub fn fun_140033970(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.ApplicationModel.DataTransfer.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xe3u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140036bf0` (`mspaint.c:50778`).
pub fn fun_140036bf0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x50u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x70u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400036a0",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140039d90` (`mspaint.c:53809`).
pub fn fun_140039d90(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add((0x7u64).wrapping_mul(8)))?) == (0x0u64))
        != 0
    {
        invoke("std::_Xbad_function_call", &[], memory);
        return Err(AdapterError::Halted("std::_Xbad_function_call"));
    }
    invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory
                .read_u64(memory.read_u64((values[0]).wrapping_add((0x7u64).wrapping_mul(8)))?)?)
            .wrapping_add(0x10u64),
        )?],
        memory,
    );
    values[1] = memory.read_u64((values[0]).wrapping_add((0x7u64).wrapping_mul(8)))?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x20u64))?,
                values[1],
                u64::from((values[1]) != (values[0])),
            ],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add((0x7u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    invoke("FUN_1400b022c", &[values[0], 0x40u64], memory);
    Ok(None)
}

/// Translation of `FUN_14003b220` (`mspaint.c:55048`).
pub fn fun_14003b220(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = values[0];
        let stored = relocate("std::exception::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("\"This function cannot be called on a default constructed task\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__std_exception_copy",
        &[relocate("STACK_local_28")],
        memory,
    );
    {
        let destination = values[0];
        let stored = relocate("Concurrency::invalid_operation::vftable");
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003ba30` (`mspaint.c:55566`).
pub fn fun_14003ba30(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_14010e2f8"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14003bb40` (`mspaint.c:55616`).
pub fn fun_14003bb40(
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
    ];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[0];
        let stored = relocate("Gdiplus::Bitmap::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = relocate("STACK_local_18");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = u64::from(
        (invoke(
            "GdipCreateBitmapFromScan0",
            &[values[1], values[2], 0x0u64],
            memory,
        )) as u32,
    );
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = u64::from((values[3]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = memory.read_u64(relocate("STACK_local_18"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003c5b0` (`mspaint.c:56362`).
pub fn fun_14003c5b0(
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
    let mut values = [arguments[0], 0u64, 0u64, 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
    values[1] = u64::from(
        (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x14u64))?)) as u32)) as u32,
    );
    values[2] = u64::from(
        (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32)) as u32,
    );
    invoke(
        "FUN_1400072f0",
        &[
            (values[3]).wrapping_add(0x38u64),
            relocate("STACK_local_18"),
            values[1],
        ],
        memory,
    );
    invoke(
        "FUN_140040b60",
        &[values[3], relocate("STACK_local_18"), values[1], values[2]],
        memory,
    );
    if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64)) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_1400415b0` (`mspaint.c:60758`).
pub fn fun_1400415b0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400036a0",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140041680` (`mspaint.c:60807`).
pub fn fun_140041680(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003ea10",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400416d0` (`mspaint.c:60829`).
pub fn fun_1400416d0(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003e7b0",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140041720` (`mspaint.c:60851`).
pub fn fun_140041720(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003d0f0",
        &[
            (values[2]).wrapping_add(0x38u64),
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    if u64::from(
        ((((u64::from(
            memory.read_u32((relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4)))?,
        )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140040b60",
            &[
                values[2],
                relocate("STACK_local_28"),
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as i32 as i64 as u64,
                (((u64::from(memory.read_u32(
                    (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as i32 as i64 as u64)
                    .wrapping_add((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64))
                    as i32 as i64 as u64,
            ],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140041830` (`mspaint.c:60905`).
pub fn fun_140041830(
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
    ];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003d0f0",
        &[
            (values[3]).wrapping_add(0x38u64),
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    invoke(
        "FUN_140040b60",
        &[
            values[3],
            relocate("STACK_local_28"),
            u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            ),
            values[2],
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400418b0` (`mspaint.c:60929`).
pub fn fun_1400418b0(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003e420",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140041900` (`mspaint.c:60951`).
pub fn fun_140041900(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003e0e0",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400419a0` (`mspaint.c:61003`).
pub fn fun_1400419a0(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14003cf00",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140042010` (`mspaint.c:61355`).
pub fn fun_140042010(
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
    ];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[5] = (values[0]).wrapping_add((0u64.wrapping_sub(0x40u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[4] = u64::from(
        (invoke(
            "FUN_140043ff0",
            &[values[5], relocate("STACK_local_18")],
            memory,
        )) as u8,
    );
    {
        let destination = values[3];
        let stored = u64::from((values[4]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140042060` (`mspaint.c:61377`).
pub fn fun_140042060(
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
    ];
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_10");
        let stored = u64::from((values[1]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_140043df0",
        &[values[3], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400423b0` (`mspaint.c:61566`).
pub fn fun_1400423b0(
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
    ];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[5] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[4] = u64::from(
        (invoke(
            "FUN_140043ff0",
            &[values[5], relocate("STACK_local_18")],
            memory,
        )) as u8,
    );
    {
        let destination = values[3];
        let stored = u64::from((values[4]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400426f0` (`mspaint.c:61786`).
pub fn fun_1400426f0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140043910",
        &[values[2], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140042770` (`mspaint.c:61823`).
pub fn fun_140042770(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14001f000",
        &[values[3], values[1], relocate("STACK_local_18")],
        memory,
    );
    invoke(
        "FUN_1400442c0",
        &[values[3], 0x1u64, (values[1]) & (0xffffffffu64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400427e0` (`mspaint.c:61846`).
pub fn fun_1400427e0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140043b20",
        &[values[3], values[1], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140042840` (`mspaint.c:61868`).
pub fn fun_140042840(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add(0x40u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x50u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = invoke(
        "FUN_140043c60",
        &[
            values[4],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(values[3])?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400488c0` (`mspaint.c:67013`).
pub fn fun_1400488c0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64, 0u64];
    {
        let destination = relocate("STACK_local_40");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x38u64))?,
            values[1],
        ],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from((values[4]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                values[4],
            ],
            memory,
        );
        values[5] = values[4];
        {
            let destination = relocate("STACK_local_40");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x10d1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[5],
            u64::from((u64::from(memory.read_u32(values[2])?)) as u32),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((values[5]) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_40")], memory);
        }
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_38")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14004ab10` (`mspaint.c:68694`).
pub fn fun_14004ab10(
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
    values[3] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000020");
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x68u64))?,
            memory.read_u64(values[3])?,
            0x4u64,
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "INDIRECT_CALL",
            &[memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x58u64))?],
            memory,
        );
    }
    {
        let destination = values[1];
        let stored = u64::from(
            (u64::from(
                (((values[2]) as i32 as i64 as u64) as i64)
                    != (((0x0u64) as i32 as i64 as u64) as i64),
            )) != 0,
        );
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004b850` (`mspaint.c:69438`).
pub fn fun_14004b850(
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
    let values = [arguments[0], arguments[1]];
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x50u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke(
        "FUN_14000f600",
        &[(values[0]).wrapping_add(0x30u64)],
        memory,
    );
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x18u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140001b30",
            &[(values[0]).wrapping_add(0x18u64)],
            memory,
        );
    }
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x10u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140001b30",
            &[(values[0]).wrapping_add(0x10u64)],
            memory,
        );
    }
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140001b30", &[(values[0]).wrapping_add(0x8u64)], memory);
    }
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x58u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14004cd30` (`mspaint.c:70542`).
pub fn fun_14004cd30(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_98"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64((values[0]).wrapping_add(0xe8u64))?;
    if u64::from(
        (u64::from((values[1]) != (0x0u64))) != 0
            && (u64::from(
                ((memory.read_u64((values[0]).wrapping_add(0xf0u64))?) as i64) != ((0x0u64) as i64),
            )) != 0,
    ) != 0
    {
        {
            let destination = relocate("STACK_local_78");
            let stored = relocate("STACK_local_58");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_70");
            let stored = u64::from((0x4u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_58");
            let stored = 0x49502a8ec12bcd8eu64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_uStack_50");
            let stored = 0xeb581d112536e78au64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_48");
            let stored = 0x4bc228acca774feau64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_uStack_40");
            let stored = 0xa3106c6decacca94u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_38");
            let stored = u64::from((0x608d3df8u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_uStack_34");
            let stored = u64::from((0x4aa78128u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_uStack_30");
            let stored = u64::from((0x5ef528a4u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_uStack_2c");
            let stored = u64::from((0x91722649u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_28");
            let stored = u64::from((0x8763404fu64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_uStack_24");
            let stored = u64::from((0x452aa1bdu64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_uStack_20");
            let stored = u64::from((0x13fc489u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_uStack_1c");
            let stored = u64::from((0x63883d3u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x48u64))?,
                values[1],
                memory.read_u64((values[0]).wrapping_add(0xf0u64))?,
                0xfffffffcu64,
                0x0u64,
            ],
            memory,
        );
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0xe8u64))?)?)
                    .wrapping_add(0x10u64),
            )?],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add(0xe8u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    if u64::from((memory.read_u64((values[0]).wrapping_add(0xf0u64))?) != (0x0u64)) != 0 {
        invoke(
            "DestroyWindow",
            &[memory.read_u64((values[0]).wrapping_add(0xf0u64))?],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add(0xf0u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(None)
}

/// Translation of `FUN_140053010` (`mspaint.c:75470`).
pub fn fun_140053010(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_res8");
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_res8"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_res8"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    invoke(
        "FUN_140003c90",
        &[
            (values[0]).wrapping_add(0x88u64),
            values[1],
            relocate("STACK_local_res8"),
        ],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_140053730` (`mspaint.c:75845`).
pub fn fun_140053730(
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
    values[6] = invoke("AfxGetThread", &[], memory);
    values[7] = 0x0u64;
    if u64::from((values[6]) != (0x0u64)) != 0 {
        values[7] = invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[6])?).wrapping_add(0xf8u64))?,
                values[6],
            ],
            memory,
        );
    }
    invoke(
        "FUN_1400a3ec0",
        &[(values[7]).wrapping_add(0x1e0u64), 0x1u64],
        memory,
    );
    values[8] = invoke("FUN_14004e8c0", &[relocate("STACK_local_48")], memory);
    values[2] = u64::from(
        (invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(
                        (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64),
                    )?)
                    .wrapping_add(0xa8u64),
                )?,
                (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64),
            ],
            memory,
        )) as u32,
    );
    values[3] = u64::from((invoke("FromNative", &[values[2]], memory)) as u32);
    values[2] = u64::from(
        (invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(
                        (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64),
                    )?)
                    .wrapping_add(0x98u64),
                )?,
                (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64),
            ],
            memory,
        )) as u32,
    );
    values[4] = u64::from((invoke("FromNative", &[values[2]], memory)) as u32);
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x706bu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[5] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[8])?)?).wrapping_add(0x138u64))?,
            memory.read_u64(values[8])?,
            values[4],
            values[3],
            values[1],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[5]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_48")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[5], relocate("STACK_local_38")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400539e0` (`mspaint.c:75968`).
pub fn fun_1400539e0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64];
    values[1] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res10")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x70ecu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[0] = (invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x178u64))?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        (((values[0]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140002540",
            &[values[0], relocate("STACK_local_28")],
            memory,
        );
        return Err(AdapterError::Halted("FUN_140002540"));
    }
    if u64::from(
        ((memory
            .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
            as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
    }
    values[2] = invoke("AfxGetThread", &[], memory);
    if u64::from((values[2]) != (0x0u64)) != 0 {
        values[3] = invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0xf8u64))?,
                values[2],
            ],
            memory,
        );
        if u64::from(((values[3]) as i64) != ((0x0u64) as i64)) != 0 {
            invoke(
                "FUN_1400a3ec0",
                &[(values[3]).wrapping_add(0x1e0u64), 0x0u64],
                memory,
            );
            return Ok(None);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140055f80` (`mspaint.c:78282`).
pub fn fun_140055f80(
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
    if u64::from(
        ((((memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i8 as i64
            as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x0u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        values[2] = memory.read_u64(values[0])?;
        {
            let destination = (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[1] = (values[2]).wrapping_add(0xe8u64);
        values[4] = (invoke(
            "FUN_140001a20",
            &[values[1], relocate("STACK_local_18")],
            memory,
        )) as i8 as i64 as u64;
        values[3] =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        if u64::from(
            (((values[4]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            if u64::from((values[1]) != (relocate("STACK_local_18"))) != 0 {
                if u64::from((memory.read_u64(values[1])?) != (0x0u64)) != 0 {
                    invoke("FUN_140001b30", &[values[1]], memory);
                }
                {
                    let destination = values[1];
                    let stored = values[3];
                    memory.write_u64(destination, stored)?;
                }
                if u64::from((values[3]) != (0x0u64)) != 0 {
                    invoke(
                        "INDIRECT_CALL",
                        &[
                            memory.read_u64((memory.read_u64(values[3])?).wrapping_add(0x8u64))?,
                            values[3],
                        ],
                        memory,
                    );
                }
            }
            invoke(
                "FUN_1400572d0",
                &[
                    (values[2]).wrapping_add(0x28u64),
                    relocate("PTR_s_AiInformation_1401121a0"),
                ],
                memory,
            );
        }
        if u64::from((values[3]) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140056280` (`mspaint.c:78517`).
pub fn fun_140056280(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140058810",
        &[values[2], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140056a50` (`mspaint.c:79029`).
pub fn fun_140056a50(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_140059320",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_1400592b0",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140056ee0` (`mspaint.c:79327`).
pub fn fun_140056ee0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_140009f20",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_1400592b0",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400573d0` (`mspaint.c:79609`).
pub fn fun_1400573d0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_140111ac8"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_14005b538",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140058250` (`mspaint.c:80463`).
pub fn fun_140058250(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_140059320",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_140009eb0",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400583a0` (`mspaint.c:80537`).
pub fn fun_1400583a0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_48");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_40");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__ExceptionPtrCopy",
        &[relocate("STACK_local_48"), values[1]],
        memory,
    );
    values[2] = u64::from(
        (invoke(
            "FUN_140059100",
            &[
                values[0],
                relocate("STACK_local_48"),
                relocate("STACK_local_38"),
            ],
            memory,
        )) as u8,
    );
    invoke("FUN_140018130", &[relocate("STACK_uStack_30")], memory);
    invoke("__ExceptionPtrDestroy", &[values[1]], memory);
    Ok(Some(u64::from((values[2]) as u8)))
}

/// Translation of `FUN_14005bb70` (`mspaint.c:83617`).
pub fn fun_14005bb70(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140112130"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14005c340` (`mspaint.c:84024`).
pub fn fun_14005c340(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x2c97u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14005ff50` (`mspaint.c:86907`).
pub fn fun_14005ff50(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    values[0] = invoke("AfxGetThread", &[], memory);
    if u64::from((values[0]) != (0x0u64)) != 0 {
        values[1] = invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0xf8u64))?,
                values[0],
            ],
            memory,
        );
        if u64::from((values[1]) != (0x0u64)) != 0 {
            invoke(
                "INDIRECT_CALL",
                &[
                    memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x358u64))?,
                    values[1],
                    0x1u64,
                ],
                memory,
            );
            return Ok(None);
        }
    }
    Ok(None)
}

/// Translation of `FUN_1400604d0` (`mspaint.c:87148`).
pub fn fun_1400604d0(
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
    values[1] = invoke("operator_new", &[0xf0u64], memory);
    if u64::from((values[1]) == (0x0u64)) != 0 {
        values[2] = 0x0u64;
    } else {
        invoke("memset", &[values[1], 0x0u64, 0xf0u64], memory);
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_uStack_10");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[2] = invoke(
            "FUN_14008cab0",
            &[values[1], values[0], relocate("STACK_local_18")],
            memory,
        );
    }
    invoke("FUN_14005ffa0", &[values[0]], memory);
    Ok(Some(values[2]))
}

/// Translation of `FUN_140060540` (`mspaint.c:87172`).
pub fn fun_140060540(
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
    values[2] = invoke(
        "COleServerDoc::OnGetLinkedItem",
        &[values[0], values[1]],
        memory,
    );
    if u64::from((values[2]) == (0x0u64)) != 0 {
        values[3] = invoke("operator_new", &[0xf0u64], memory);
        if u64::from((values[3]) != (0x0u64)) != 0 {
            invoke("memset", &[values[3], 0x0u64, 0xf0u64], memory);
            {
                let destination = relocate("STACK_local_18");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = relocate("STACK_uStack_10");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "FUN_14008cab0",
                &[values[3], values[0], relocate("STACK_local_18")],
                memory,
            );
        }
    }
    Ok(None)
}

/// Translation of `FUN_1400622c0` (`mspaint.c:88561`).
pub fn fun_1400622c0(
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
    values[6] = invoke("operator_new", &[0x18u64], memory);
    if u64::from((values[6]) != (0x0u64)) != 0 {
        values[3] = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x1u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        values[4] = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x2u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        values[5] = u64::from(
            (u64::from(
                (u64::from(memory.read_u32((values[1]).wrapping_add((0x3u64).wrapping_mul(4)))?))
                    as u32,
            )) as u32,
        );
        {
            let destination = values[6];
            let stored =
                u64::from((u64::from((u64::from(memory.read_u32(values[1])?)) as u32)) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[6]).wrapping_add((0x1u64).wrapping_mul(4));
            let stored = u64::from((values[3]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[6]).wrapping_add((0x2u64).wrapping_mul(4));
            let stored = u64::from((values[4]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[6]).wrapping_add((0x3u64).wrapping_mul(4));
            let stored = u64::from((values[5]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[6]).wrapping_add((0x4u64).wrapping_mul(4));
            let stored = memory.read_u64((values[1]).wrapping_add((0x4u64).wrapping_mul(4)))?;
            memory.write_u64(destination, stored)?;
        }
    }
    values[2] = memory.read_u64((values[0]).wrapping_add(0x2c8u64))?;
    {
        let destination = (values[0]).wrapping_add(0x2c8u64);
        let stored = values[6];
        memory.write_u64(destination, stored)?;
    }
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_1400b022c", &[values[2], 0x18u64], memory);
    }
    values[7] = invoke("AfxGetThread", &[], memory);
    if u64::from((values[7]) != (0x0u64)) != 0 {
        values[8] = invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[7])?).wrapping_add(0xf8u64))?,
                values[7],
            ],
            memory,
        );
        if u64::from((values[8]) != (0x0u64)) != 0 {
            values[9] = 0xe10bu64;
            if u64::from(
                (((u64::from((u64::from(memory.read_u8(relocate("DAT_1401d085a"))?)) as u8)) as i32
                    as i64 as u64) as i64)
                    == (((0x0u64) as i32 as i64 as u64) as i64),
            ) != 0
            {
                values[9] = 0xe104u64;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory.read_u64((memory.read_u64(values[8])?).wrapping_add(0x28u64))?,
                    values[8],
                    values[9],
                    0x0u64,
                    0x0u64,
                    0x0u64,
                ],
                memory,
            );
        }
    }
    Ok(None)
}

/// Translation of `FUN_140062de0` (`mspaint.c:89182`).
pub fn fun_140062de0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = invoke("operator_new", &[0x4c0u64], memory);
        memory.write_u64(destination, stored)?;
    }
    {
        let destination =
            (memory.read_u64(relocate("STACK_local_28"))?).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = relocate("FUN_1400c4950");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = memory.read_u64(relocate("STACK_local_28"))?;
        let stored = relocate("FUN_1400c4960");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination =
            (memory.read_u64(relocate("STACK_local_28"))?).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = values[0];
        memory.write_u64(destination, stored)?;
    }
    values[3] = memory.read_u64(values[2])?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination =
            (memory.read_u64(relocate("STACK_local_28"))?).wrapping_add((0x4u64).wrapping_mul(8));
        let stored = values[3];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination =
            (memory.read_u64(relocate("STACK_local_28"))?).wrapping_add((0x5u64).wrapping_mul(8));
        let stored = u64::from((0x10002u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = values[1];
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = (memory.read_u64(relocate("STACK_local_28"))?).wrapping_add(0x11u64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    invoke("FUN_1400c4960", &[], memory);
    if u64::from(((memory.read_u64(values[2])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[values[2]], memory);
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140063ee0` (`mspaint.c:90158`).
pub fn fun_140063ee0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_1401145c8"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140018690",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140064c20` (`mspaint.c:90966`).
pub fn fun_140064c20(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x28u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400659d0",
        &[values[3], values[1], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140065c30` (`mspaint.c:91978`).
pub fn fun_140065c30(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((values[0]).wrapping_add(0x18u64))?,
            memory.read_u64((values[0]).wrapping_add(0x10u64))?,
            relocate("STACK_local_20"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140066aa0` (`mspaint.c:92835`).
pub fn fun_140066aa0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.ApplicationModel.DataTransfer.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xb8du64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x58u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140066d80` (`mspaint.c:92993`).
pub fn fun_140066d80(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.ApplicationModel.DataTransfer.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xb7bu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x50u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400697f0` (`mspaint.c:95014`).
pub fn fun_1400697f0(
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
    values[2] = memory.read_u64(values[0])?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        invoke(
            "FUN_14000a2e0",
            &[(values[2]).wrapping_add((0x4u64).wrapping_mul(8))],
            memory,
        );
        if u64::from((values[2]) != (relocate("STACK_local_28"))) != 0 {
            if u64::from(((memory.read_u64(values[2])?) as i64) != ((0x0u64) as i64)) != 0 {
                invoke("FUN_140001b30", &[values[2]], memory);
            }
            {
                let destination = values[2];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        values[1] = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
        if u64::from((values[1]) != (relocate("STACK_local_20"))) != 0 {
            if u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)) != 0 {
                invoke("FUN_140006970", &[values[1]], memory);
            }
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        values[1] = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
        if u64::from((values[1]) != (relocate("STACK_local_18"))) != 0 {
            if u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)) != 0 {
                invoke("FUN_140006970", &[values[1]], memory);
            }
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from(
            ((memory.read_u64((values[2]).wrapping_add((0x4u64).wrapping_mul(8)))?) as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140001b30",
                &[(values[2]).wrapping_add((0x4u64).wrapping_mul(8))],
                memory,
            );
        }
        if u64::from(
            (memory.read_u64((values[2]).wrapping_add((0x3u64).wrapping_mul(8)))?) != (0x0u64),
        ) != 0
        {
            invoke(
                "SysFreeString",
                &[memory.read_u64((values[2]).wrapping_add((0x3u64).wrapping_mul(8)))?],
                memory,
            );
        }
        if u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140006970", &[values[1]], memory);
        }
        if u64::from(
            ((memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140006970", &[], memory);
        }
        if u64::from(((memory.read_u64(values[2])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[2]], memory);
        }
        invoke("FUN_1400b022c", &[values[2], 0x28u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_1400698f0` (`mspaint.c:95071`).
pub fn fun_1400698f0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_140115220"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140018690",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140069980` (`mspaint.c:95100`).
pub fn fun_140069980(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_140110990"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140069ed8",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140069a10` (`mspaint.c:95129`).
pub fn fun_140069a10(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_1401151c8"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140069ec8",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140069aa0` (`mspaint.c:95158`).
pub fn fun_140069aa0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_1401151d8"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140069eb8",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140069b30` (`mspaint.c:95187`).
pub fn fun_140069b30(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_1401151b8"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140069ea8",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_140069bc0` (`mspaint.c:95216`).
pub fn fun_140069bc0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_1401151b8"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140069e98",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_14006b370` (`mspaint.c:96460`).
pub fn fun_14006b370(
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
    let mut values = [arguments[0], 0u64, 0u64];
    values[1] = memory.read_u64((values[0]).wrapping_add(0xf8u64))?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "BasePaint::DirectX::Helpers::TextureStore::~TextureStore",
            &[values[1]],
            memory,
        );
        invoke("FUN_1400b022c", &[values[1], 0x90u64], memory);
    }
    values[2] = memory.read_u64((values[0]).wrapping_add(0xf0u64))?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        invoke(
            "BasePaint::DirectX::Helpers::ShaderFactory::~ShaderFactory",
            &[values[2]],
            memory,
        );
        invoke("FUN_1400b022c", &[values[2], 0xd0u64], memory);
    }
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0xe8u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0xe0u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke(
        "FUN_1400573d0",
        &[(values[0]).wrapping_add(0xb8u64)],
        memory,
    );
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0xa0u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_1400041f0", &[], memory);
    }
    invoke(
        "FUN_140006890",
        &[(values[0]).wrapping_add(0x90u64)],
        memory,
    );
    invoke(
        "FUN_140034240",
        &[(values[0]).wrapping_add(0x80u64)],
        memory,
    );
    invoke(
        "FUN_140078760",
        &[(values[0]).wrapping_add(0x70u64)],
        memory,
    );
    invoke(
        "FUN_1400786f0",
        &[(values[0]).wrapping_add(0x60u64)],
        memory,
    );
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x58u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140001b30",
            &[(values[0]).wrapping_add(0x58u64)],
            memory,
        );
    }
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x28u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_1400041f0",
            &[(values[0]).wrapping_add(0x28u64)],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400779a0` (`mspaint.c:105182`).
pub fn fun_1400779a0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_14000cce0",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_14007a140",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140077ea0` (`mspaint.c:105507`).
pub fn fun_140077ea0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[2] = (values[4]).wrapping_add(0x48u64);
    {
        let destination = relocate("STACK_local_68");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[2]], memory);
    if u64::from(
        ((((u64::from(memory.read_u8((values[4]).wrapping_add(0x64u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[4]).wrapping_add(0x64u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = relocate("STACK_local_68");
            let stored = ((u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_68")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            ))
            .wrapping_shl(0x20u64 as u32))
                | (u64::from(
                    ((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                        as u64) as u32,
                ));
            memory.write_u64(destination, stored)?;
        }
        if u64::from(
            ((((u64::from(memory.read_u32((values[4]).wrapping_add(0x60u64))?)) as i32 as i64
                as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = invoke(
                "FUN_14007a1b0",
                &[relocate("STACK_local_68"), relocate("STACK_local_30")],
                memory,
            );
            values[5] = (values[4]).wrapping_add(0x50u64);
            if u64::from((values[5]) != (values[3])) != 0 {
                if u64::from(((memory.read_u64(values[5])?) as i64) != ((0x0u64) as i64)) != 0 {
                    invoke("FUN_140001b30", &[values[5]], memory);
                }
                values[4] = memory.read_u64(values[3])?;
                {
                    let destination = values[3];
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                {
                    let destination = values[5];
                    let stored = values[4];
                    memory.write_u64(destination, stored)?;
                }
            }
            if u64::from((memory.read_u64(relocate("STACK_local_68"))?) != (0x0u64)) != 0 {
                invoke("FUN_140001b30", &[relocate("STACK_local_68")], memory);
            }
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
        } else {
            invoke("ReleaseSRWLockExclusive", &[values[2]], memory);
            if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
                invoke(
                    "FUN_1400592b0",
                    &[
                        relocate("STACK_local_30"),
                        values[4],
                        relocate("STACK_local_68"),
                    ],
                    memory,
                );
            }
        }
        return Ok(Some(0x0u64));
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x155au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000a900",
        &[relocate("STACK_local_48"), relocate("STACK_local_60")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_48"), relocate("DAT_14016a900")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400786a0` (`mspaint.c:105947`).
pub fn fun_1400786a0(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_res8");
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_res8"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_res8"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    invoke(
        "FUN_140003c90",
        &[values[0], values[1], relocate("STACK_local_res8")],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_140079120` (`mspaint.c:106620`).
pub fn fun_140079120(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x28u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14007b320",
        &[values[3], values[1], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14007beb0` (`mspaint.c:109096`).
pub fn fun_14007beb0(
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
    {
        let destination = relocate("STACK_local_res18");
        let stored = relocate("STACK_local_48");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_48");
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_40");
        let stored = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_40"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_40"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("STACK_local_48");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored = relocate("L\"BitmapShareTarget\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x11u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140005310",
        &[relocate("STACK_local_res10"), relocate("STACK_local_58")],
        memory,
    );
    invoke(
        "FUN_14006a430",
        &[relocate("STACK_local_res10"), relocate("STACK_local_48")],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14007bf90` (`mspaint.c:109153`).
pub fn fun_14007bf90(
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
    {
        let destination = relocate("STACK_local_res18");
        let stored = relocate("STACK_local_48");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_48");
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_40");
        let stored = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("STACK_local_40"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_40"))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("STACK_local_48");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored = relocate("L\"StorageItemShareTarget\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_50");
        let stored = 0x16u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140005310",
        &[relocate("STACK_local_res10"), relocate("STACK_local_58")],
        memory,
    );
    invoke(
        "FUN_14006a430",
        &[relocate("STACK_local_res10"), relocate("STACK_local_48")],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14007c7f0` (`mspaint.c:109708`).
pub fn fun_14007c7f0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.ApplicationModel.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x272u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x38u64))?,
            memory.read_u64(values[1])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14007ca00` (`mspaint.c:109814`).
pub fn fun_14007ca00(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140117510"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14007ecc0` (`mspaint.c:111558`).
pub fn fun_14007ecc0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    values[0] = (invoke("FUN_140007c00", &[0xe12bu64], memory)) as i8 as i64 as u64;
    if u64::from(
        (((values[0]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[1] = (invoke(
            "CPaintTool::Id",
            &[memory.read_u64(relocate("DAT_1401d06e0"))?],
            memory,
        )) as i32 as i64 as u64;
        invoke(
            "FUN_1400512b0",
            &[
                relocate("DAT_1401d06a0"),
                relocate("STACK_local_res10"),
                values[1],
            ],
            memory,
        );
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
        {
            let destination = relocate("STACK_local_20");
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_10");
            let stored = relocate("L\"UndoAnnouncement\"");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_28");
            let stored = relocate("STACK_local_20");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_1c");
            let stored = u64::from((0x10u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
        invoke("FUN_1400947a0", &[], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14007efa0` (`mspaint.c:111697`).
pub fn fun_14007efa0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("TraceDelete", &[], memory);
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"DeleteAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x12u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    invoke("FUN_14000c0d0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_14007f000` (`mspaint.c:111719`).
pub fn fun_14007f000(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"CopyAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x10u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    invoke("FUN_140032a80", &[relocate("STACK_local_res10")], memory);
    if u64::from(
        ((memory
            .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
            as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_14007f070` (`mspaint.c:111744`).
pub fn fun_14007f070(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"PasteAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x11u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    invoke("FUN_140032df0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_14007f240` (`mspaint.c:111856`).
pub fn fun_14007f240(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"InverSelectionAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x1au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    values[0] = memory.read_u64(relocate("DAT_1401d06e0"))?;
    values[1] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                    .wrapping_add(0x68u64),
            )?,
            memory.read_u64(relocate("DAT_1401d06e0"))?,
            0x16u64,
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("CPaintTool::OnInvertSelection", &[values[0]], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140084410` (`mspaint.c:115660`).
pub fn fun_140084410(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64];
    values[0] = memory.read_u64(relocate("DAT_1401d06e0"))?;
    values[1] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                    .wrapping_add(0x68u64),
            )?,
            memory.read_u64(relocate("DAT_1401d06e0"))?,
            0x0u64,
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = (invoke("CPaintTool::Id", &[values[0]], memory)) as i32 as i64 as u64;
        invoke(
            "FUN_1400512b0",
            &[
                relocate("DAT_1401d06a0"),
                relocate("STACK_local_res10"),
                values[2],
            ],
            memory,
        );
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"RotateAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x12u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    invoke("FUN_14000c380", &[0x4056800000000000u64], memory);
    Ok(None)
}

/// Translation of `FUN_1400844c0` (`mspaint.c:115695`).
pub fn fun_1400844c0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64];
    values[0] = memory.read_u64(relocate("DAT_1401d06e0"))?;
    values[1] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                    .wrapping_add(0x68u64),
            )?,
            memory.read_u64(relocate("DAT_1401d06e0"))?,
            0x0u64,
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = (invoke("CPaintTool::Id", &[values[0]], memory)) as i32 as i64 as u64;
        invoke(
            "FUN_1400512b0",
            &[
                relocate("DAT_1401d06a0"),
                relocate("STACK_local_res10"),
                values[2],
            ],
            memory,
        );
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"RotateAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x12u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    invoke("FUN_14000c380", &[0x4070e00000000000u64], memory);
    Ok(None)
}

/// Translation of `FUN_140084570` (`mspaint.c:115730`).
pub fn fun_140084570(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64];
    values[0] = memory.read_u64(relocate("DAT_1401d06e0"))?;
    values[1] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                    .wrapping_add(0x68u64),
            )?,
            memory.read_u64(relocate("DAT_1401d06e0"))?,
            0x0u64,
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = (invoke("CPaintTool::Id", &[values[0]], memory)) as i32 as i64 as u64;
        invoke(
            "FUN_1400512b0",
            &[
                relocate("DAT_1401d06a0"),
                relocate("STACK_local_res10"),
                values[2],
            ],
            memory,
        );
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = u64::from((0x1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = relocate("L\"RotateAnnouncement\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_1c");
        let stored = u64::from((0x12u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke("FUN_14004ce90", &[relocate("STACK_local_28")], memory);
    invoke("FUN_14000c380", &[0x4066800000000000u64], memory);
    Ok(None)
}

/// Translation of `FUN_1400853c0` (`mspaint.c:116526`).
pub fn fun_1400853c0(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140084280",
        &[
            (values[0]).wrapping_add(0x10u64),
            values[1],
            relocate("STACK_local_18"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140085410` (`mspaint.c:116543`).
pub fn fun_140085410(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140082bb0",
        &[
            (values[0]).wrapping_add(0x10u64),
            values[1],
            relocate("STACK_local_18"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140085700` (`mspaint.c:116722`).
pub fn fun_140085700(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140118880"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140085980` (`mspaint.c:116847`).
pub fn fun_140085980(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_1401188d8"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140085c00` (`mspaint.c:116972`).
pub fn fun_140085c00(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140118920"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140086060` (`mspaint.c:117195`).
pub fn fun_140086060(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_140118980"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140088b00` (`mspaint.c:119125`).
pub fn fun_140088b00(
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
    ];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = u64::from(
        (invoke(
            "GetDlgItemInt",
            &[values[0], values[1], relocate("STACK_local_18"), 0x0u64],
            memory,
        )) as u32,
    );
    if u64::from(
        (u64::from(
            ((((u64::from(
                memory.read_u32(
                    (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?,
            )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                != (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0
            && (u64::from((u64::from((values[3]) as u32)) != (u64::from((0x0u64) as u32)))) != 0,
    ) != 0
    {
        {
            let destination = values[2];
            let stored = u64::from((values[3]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        values[4] = invoke("GetDlgItem", &[values[0], 0x438u64], memory);
        invoke("InvalidateRect", &[values[4], 0x0u64, 0x1u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140092970` (`mspaint.c:126617`).
pub fn fun_140092970(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xd6u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x80u64))?,
            memory.read_u64(values[0])?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140093230` (`mspaint.c:127067`).
pub fn fun_140093230(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_14011a2f0"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140093524",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_1400932c0` (`mspaint.c:127096`).
pub fn fun_1400932c0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        {
            let destination = relocate("STACK_local_18");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x18u64))?,
                values[1],
                relocate("DAT_14011a2f0"),
                relocate("STACK_local_18"),
            ],
            memory,
        );
        if u64::from(((memory.read_u64(relocate("STACK_local_18"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke(
                "FUN_140093514",
                &[
                    memory.read_u64(relocate("STACK_local_18"))?,
                    memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                ],
                memory,
            );
            invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
        }
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
    }
    Ok(None)
}

/// Translation of `FUN_1400939a0` (`mspaint.c:127466`).
pub fn fun_1400939a0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_14011a418"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140096440` (`mspaint.c:129767`).
pub fn fun_140096440(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000008");
    }
    values[1] = invoke("FUN_140003ae0", &[values[2]], memory);
    if u64::from(
        ((((values[1]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
                values[2],
                0x1u64,
            ],
            memory,
        );
        values[1] &= 0xffffffffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140096930` (`mspaint.c:130078`).
pub fn fun_140096930(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_88"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = (values[2]).wrapping_add(0x68u64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = (relocate("STACK_local_60")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = relocate("STACK_local_60");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140095700",
        &[
            values[2],
            relocate("STACK_local_20"),
            relocate("STACK_local_60"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140096c40` (`mspaint.c:130261`).
pub fn fun_140096c40(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("DAT_1401d07c6");
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = u64::from(
        (invoke(
            "FUN_140095520",
            &[values[4], relocate("STACK_local_18")],
            memory,
        )) as u8,
    );
    {
        let destination = values[2];
        let stored = u64::from((values[3]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140096cd0` (`mspaint.c:130301`).
pub fn fun_140096cd0(
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
    let mut values = [arguments[0], (arguments[1]) as i32 as i64 as u64, 0u64];
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) == (((0x2u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = (values[0]).wrapping_add(0x50u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[2] = relocate("DAT_00000070");
        }
        invoke(
            "INDIRECT_CALL",
            &[memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x20u64))?],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140096d70` (`mspaint.c:130365`).
pub fn fun_140096d70(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000008");
    }
    values[1] = invoke("FUN_140003ae0", &[values[2]], memory);
    if u64::from(
        ((((values[1]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
                values[2],
                0x1u64,
            ],
            memory,
        );
        values[1] &= 0xffffffffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140097040` (`mspaint.c:130575`).
pub fn fun_140097040(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = (values[3]).wrapping_add(0xa2u64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = (relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140095190",
        &[
            values[3],
            relocate("STACK_local_18"),
            relocate("STACK_local_res10"),
            0x0u64,
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140097090` (`mspaint.c:130596`).
pub fn fun_140097090(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = (values[3]).wrapping_add(0xa2u64);
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = (relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140095190",
        &[
            values[3],
            relocate("STACK_local_18"),
            relocate("STACK_local_res10"),
            0x1u64,
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140097230` (`mspaint.c:130707`).
pub fn fun_140097230(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64, 0u64];
    {
        let destination = relocate("STACK_local_40");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64((values[1]).wrapping_add(0x8u64))?).wrapping_add(0x38u64),
        )?],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from((values[4]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[4])?).wrapping_add(0x8u64))?,
                values[4],
            ],
            memory,
        );
        values[5] = values[4];
        {
            let destination = relocate("STACK_local_40");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x10d1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[5],
            memory.read_u64(values[2])?,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((values[5]) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_40")], memory);
        }
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_38")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140097300` (`mspaint.c:130746`).
pub fn fun_140097300(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[0]) == (0x0u64)) != 0 {
        values[0] = relocate("DAT_00000008");
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x20u64))?,
            values[0],
            relocate("STACK_local_28"),
        ],
        memory,
    );
    if u64::from(
        (u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            )) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[2];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    } else {
        {
            let destination = values[1];
            let stored = u64::from(
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        values[3] = invoke(
            "CoTaskMemAlloc",
            &[(u64::from(
                (u64::from(memory.read_u32(
                    (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                )?)) as u32,
            ))
            .wrapping_shl(0x4u64 as u32)],
            memory,
        );
        {
            let destination = values[2];
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        if u64::from((values[3]) == (0x0u64)) != 0 {
            return Ok(Some(0x8007000eu64));
        }
        invoke(
            "memmove",
            &[
                values[3],
                memory.read_u64(relocate("STACK_local_20"))?,
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                ))
                .wrapping_shl(0x4u64 as u32),
            ],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140097450` (`mspaint.c:130830`).
pub fn fun_140097450(
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
    let values = [arguments[0], arguments[1], arguments[2], arguments[3]];
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            ((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as i32 as i64 as u64)
                .wrapping_add(values[1]),
            values[2],
            memory.read_u64(values[3])?,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140097490` (`mspaint.c:130855`).
pub fn fun_140097490(
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
    if u64::from((values[0]) == (0x0u64)) != 0 {
        values[0] = relocate("DAT_00000008");
    }
    values[2] = u64::from(
        (invoke(
            "INDIRECT_CALL",
            &[memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x10u64))?],
            memory,
        )) as u32,
    );
    {
        let destination = values[1];
        let stored = u64::from((values[2]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140097f80` (`mspaint.c:131435`).
pub fn fun_140097f80(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x40u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140097dc0",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140097ff0` (`mspaint.c:131459`).
pub fn fun_140097ff0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140097710",
        &[values[2], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140098130` (`mspaint.c:131535`).
pub fn fun_140098130(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[3] = invoke(
        "FUN_140097aa0",
        &[values[4], relocate("STACK_local_res8")],
        memory,
    );
    values[2] = memory.read_u64(values[3])?;
    {
        let destination = values[3];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[1];
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    if u64::from(((memory.read_u64(relocate("STACK_local_res8"))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res8")], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140098190` (`mspaint.c:131562`).
pub fn fun_140098190(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
    }
    values[2] = memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0xe8u64))?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
                values[2],
            ],
            memory,
        );
    }
    {
        let destination = values[1];
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400981f0` (`mspaint.c:131582`).
pub fn fun_1400981f0(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
    }
    invoke(
        "FUN_140052570",
        &[memory.read_u64(values[2])?, relocate("STACK_local_res8")],
        memory,
    );
    {
        let destination = values[1];
        let stored = memory
            .read_u64((relocate("STACK_local_res8")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140098240` (`mspaint.c:131602`).
pub fn fun_140098240(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000038");
    }
    {
        let destination = (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x44d9u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x58u64))?,
            memory.read_u64(values[3])?,
            relocate("STACK_local_28"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = u64::from(
                (u64::from((memory.read_u64(relocate("STACK_local_28"))?) as u32)) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(4));
            let stored = u64::from(
                (u64::from(
                    (u64::from(memory.read_u32((relocate("STACK_local_28")).wrapping_add(0x4u64))?))
                        as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400983b0` (`mspaint.c:131690`).
pub fn fun_1400983b0(
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
    {
        let destination = (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xd001u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = (invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[1], relocate("STACK_local_20")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140099670` (`mspaint.c:132572`).
pub fn fun_140099670(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = invoke(
        "FUN_1400985c0",
        &[
            values[4],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(values[3])?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099700` (`mspaint.c:132612`).
pub fn fun_140099700(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x38u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140097dc0",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099770` (`mspaint.c:132637`).
pub fn fun_140099770(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x58u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000068");
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[3]) != (relocate("STACK_local_20"))) != 0 {
        if u64::from((memory.read_u64(values[3])?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[values[3]], memory);
        }
        {
            let destination = values[3];
            let stored = values[1];
            memory.write_u64(destination, stored)?;
        }
        if u64::from((values[1]) != (0x0u64)) != 0 {
            invoke(
                "INDIRECT_CALL",
                &[
                    memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?,
                    values[1],
                ],
                memory,
            );
        }
    }
    values[2] = (values[0]).wrapping_add(0x40u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x50u64;
    }
    invoke(
        "FUN_140099560",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099820` (`mspaint.c:132673`).
pub fn fun_140099820(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140098890",
        &[
            values[3],
            relocate("STACK_local_20"),
            relocate("STACK_local_28"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099880` (`mspaint.c:132698`).
pub fn fun_140099880(
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
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x0u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    values[3] = (values[0]).wrapping_add(0x58u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000068");
    }
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x533bu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[3])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[1];
            let stored = u64::from(
                (u64::from(
                    (u64::from(memory.read_u32(
                        (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4)),
                    )?)) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_38")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140099ae0` (`mspaint.c:132868`).
pub fn fun_140099ae0(
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
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    values[2] = invoke(
        "FUN_140098760",
        &[values[3], relocate("STACK_local_res8")],
        memory,
    );
    {
        let destination = values[1];
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099c30` (`mspaint.c:132947`).
pub fn fun_140099c30(
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
    {
        let destination = (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xd027u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            memory.read_u64(values[1])?,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14009a190` (`mspaint.c:133236`).
pub fn fun_14009a190(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140099e50",
        &[
            values[3],
            relocate("STACK_local_20"),
            relocate("STACK_local_28"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14009b180` (`mspaint.c:134023`).
pub fn fun_14009b180(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14009b1d0",
        &[
            (values[0]).wrapping_add(0x10u64),
            values[1],
            relocate("STACK_local_18"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14009bd70` (`mspaint.c:134588`).
pub fn fun_14009bd70(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "FUN_140001df0",
        &[relocate("STACK_local_28"), relocate("\"Not implemented\"")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_28"), relocate("DAT_14016aec0")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_14009bf20` (`mspaint.c:134682`).
pub fn fun_14009bf20(
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
    let values = [arguments[0], arguments[1]];
    if u64::from(
        ((((u64::from(memory.read_u32((values[1]).wrapping_add(0x8u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            != (((0xau64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(None);
    }
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x50u64))?) == (0x0u64)) != 0 {
        invoke("std::_Xbad_function_call", &[], memory);
        return Err(AdapterError::Halted("std::_Xbad_function_call"));
    }
    invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x50u64))?)?)
                .wrapping_add(0x10u64),
        )?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400a10d0` (`mspaint.c:138472`).
pub fn fun_1400a10d0(
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
    let values = [arguments[0]];
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?) as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[values[0]], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_1400a3320` (`mspaint.c:140003`).
pub fn fun_1400a3320(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "Paint::AI::PaintAIManager::AIFactory::GetCloudCreditsManager",
        &[memory.read_u64((values[0]).wrapping_add(0x68u64))?],
        memory,
    );
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\AI.h\"");
        memory.write_u64(destination, stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32 as i64 as u64)
            as i64)
            == (((0x2u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x54u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        values[3] = (invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)
                    .wrapping_add(0x48u64),
            )?],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140002540",
                &[values[3], relocate("STACK_local_38")],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140002540"));
        }
    } else {
        {
            let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x64u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        values[3] = (invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)
                    .wrapping_add(0x50u64),
            )?],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140002540",
                &[values[3], relocate("STACK_local_38")],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140002540"));
        }
    }
    if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_1400a41f0` (`mspaint.c:140784`).
pub fn fun_1400a41f0(
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
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_78"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_58");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "FUN_140001a20",
        &[
            (values[0]).wrapping_add(0xe8u64),
            relocate("STACK_local_58"),
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from((memory.read_u64(relocate("STACK_local_58"))?) != (0x0u64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_58")], memory);
    }
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = relocate("STACK_local_30");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[2] = memory.read_u64((values[0]).wrapping_add(0xe8u64))?;
        {
            let destination = (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x22u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_48");
            let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Hosting.h\"");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_40");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[4] = (invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x30u64))?,
                values[2],
                relocate("STACK_local_30"),
            ],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[4]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140002540",
                &[values[4], relocate("STACK_local_50")],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140002540"));
        }
        {
            let destination = relocate("STACK_local_58");
            let stored = memory.read_u64(relocate("STACK_local_30"))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_30");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        values[3] = (invoke(
            "FUN_140001a20",
            &[relocate("STACK_local_58"), relocate("STACK_local_30")],
            memory,
        )) as i8 as i64 as u64;
        if u64::from((memory.read_u64(relocate("STACK_local_30"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_30")], memory);
        }
        if u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            {
                let destination = relocate("STACK_local_30");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            {
                let destination =
                    (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(4));
                let stored = u64::from((0x2506u64) as u32);
                memory.write_u32(destination, stored as u32)?;
            }
            {
                let destination = relocate("STACK_local_48");
                let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.h\"");
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = relocate("STACK_local_40");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            values[4] = (invoke(
                "INDIRECT_CALL",
                &[
                    memory.read_u64(
                        (memory.read_u64(memory.read_u64(relocate("STACK_local_58"))?)?)
                            .wrapping_add(0x368u64),
                    )?,
                    memory.read_u64(relocate("STACK_local_58"))?,
                    relocate("STACK_local_30"),
                ],
                memory,
            )) as i32 as i64 as u64;
            if u64::from(
                (((values[4]) as i32 as i64 as u64) as i64)
                    < (((0x0u64) as i32 as i64 as u64) as i64),
            ) != 0
            {
                invoke(
                    "FUN_140002540",
                    &[values[4], relocate("STACK_local_50")],
                    memory,
                );
                return Err(AdapterError::Halted("FUN_140002540"));
            }
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_30"))?;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            if u64::from((memory.read_u64(relocate("STACK_local_58"))?) == (0x0u64)) != 0 {
                return Ok(Some(values[1]));
            }
        }
        invoke("FUN_140001b30", &[relocate("STACK_local_58")], memory);
    } else {
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400a4ff0` (`mspaint.c:141475`).
pub fn fun_1400a4ff0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14009e430",
        &[values[0], relocate("STACK_local_18")],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a50f0` (`mspaint.c:141545`).
pub fn fun_1400a50f0(
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
    ];
    {
        let destination = relocate("STACK_local_28");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[3];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[6] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[6] = 0x0u64;
    }
    {
        let destination = relocate("STACK_local_48");
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[5] = invoke(
        "FUN_14009e2c0",
        &[
            values[6],
            relocate("STACK_local_38"),
            relocate("STACK_local_30"),
            values[2],
        ],
        memory,
    );
    values[4] = memory.read_u64(values[5])?;
    {
        let destination = values[5];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[3];
        let stored = values[4];
        memory.write_u64(destination, stored)?;
    }
    if u64::from(((memory.read_u64(relocate("STACK_local_38"))?) as i64) != ((0x0u64) as i64)) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_38")], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5400` (`mspaint.c:141736`).
pub fn fun_1400a5400(
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
        (arguments[1]) as i16 as i64 as u64,
        0u64,
        0u64,
    ];
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x4a1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\XamlHost.cpp\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000020");
    }
    values[2] = (invoke(
        "PostMessageW",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x40u64))?,
            0x111u64,
            u64::from((((values[1]).wrapping_add(0xe110u64)) as i32 as i64 as u64) as u16),
            0x0u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(Some(0x0u64));
    }
    invoke("FUN_14000a550", &[relocate("STACK_local_28")], memory);
    Err(AdapterError::Halted("FUN_14000a550"))
}

/// Translation of `FUN_1400a5480` (`mspaint.c:141766`).
pub fn fun_1400a5480(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_58"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140075b40",
        &[values[0], relocate("STACK_local_38")],
        memory,
    );
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x25au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64(relocate("STACK_local_38"))?)?)
                    .wrapping_add(0x40u64),
            )?,
            memory.read_u64(relocate("STACK_local_38"))?,
            relocate("STACK_local_18"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_38"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_38")], memory);
        }
        {
            let destination = values[1];
            let stored = memory.read_u64(relocate("STACK_local_18"))?;
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400a5940` (`mspaint.c:142048`).
pub fn fun_1400a5940(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x90u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0xa0u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400a4e20",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a59b0` (`mspaint.c:142072`).
pub fn fun_1400a59b0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                        .wrapping_add(0x110u64),
                )?,
                memory.read_u64(relocate("DAT_1401d06e0"))?,
                0x6u64,
            ],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5ae0` (`mspaint.c:142139`).
pub fn fun_1400a5ae0(
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
    values[1] = (values[0]).wrapping_add(0x3e4u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = relocate("DAT_000003f4");
    }
    if u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                        .wrapping_add(0x110u64),
                )?,
                memory.read_u64(relocate("DAT_1401d06e0"))?,
                u64::from((u64::from(memory.read_u32(values[1])?)) as u32),
            ],
            memory,
        );
    }
    {
        let destination = values[1];
        let stored = u64::from((0x0u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6550` (`mspaint.c:142722`).
pub fn fun_1400a6550(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if u64::from((memory.read_u64(relocate("DAT_1401d06e0"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("DAT_1401d06e0"))?)?)
                        .wrapping_add(0x110u64),
                )?,
                memory.read_u64(relocate("DAT_1401d06e0"))?,
                0x1u64,
            ],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6990` (`mspaint.c:142953`).
pub fn fun_1400a6990(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x48u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x58u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400a4e20",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6a00` (`mspaint.c:142975`).
pub fn fun_1400a6a00(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_res8")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = values[0];
        memory.write_u64(destination, stored)?;
    }
    values[3] = invoke(
        "FUN_14009d3e0",
        &[values[0], relocate("STACK_local_res8")],
        memory,
    );
    values[2] = memory.read_u64(values[3])?;
    {
        let destination = values[3];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[1];
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    if u64::from(
        ((memory.read_u64((relocate("STACK_local_res8")).wrapping_add((0x0u64).wrapping_mul(8)))?)
            as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res8")], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6a50` (`mspaint.c:142998`).
pub fn fun_1400a6a50(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x40u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400a4e20",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6de0` (`mspaint.c:143205`).
pub fn fun_1400a6de0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x28u64;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400a4e20",
        &[
            values[3],
            relocate("STACK_local_28"),
            relocate("STACK_local_20"),
        ],
        memory,
    );
    {
        let destination = values[2];
        let stored = memory.read_u64(relocate("STACK_local_28"))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a70c0` (`mspaint.c:143354`).
pub fn fun_1400a70c0(
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
    values[2] = invoke("operator_new", &[0x18u64], memory);
    if u64::from((values[2]) != (0x0u64)) != 0 {
        values[1] = memory.read_u64((values[0]).wrapping_add(0x10u64))?;
        {
            let destination = values[2];
            let stored = relocate("CRefAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = values[1];
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x3u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_1400a7110` (`mspaint.c:143373`).
pub fn fun_1400a7110(
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
    values[2] = invoke(
        "__RTDynamicCast",
        &[
            values[1],
            0x0u64,
            relocate("Value::RTTI_Type_Descriptor"),
            relocate("CRefAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
            let stored = u64::from(
                (u64::from(
                    (u64::from(
                        memory.read_u32(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?,
                    )) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    values[2] = invoke(
        "__RTDynamicCast",
        &[
            values[1],
            0x0u64,
            relocate("Value::RTTI_Type_Descriptor"),
            relocate("CAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = (values[2]).wrapping_add(0x10u64);
            let stored = u64::from(
                (u64::from(
                    (u64::from(
                        memory.read_u32(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?,
                    )) as u32,
                )) as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a7620` (`mspaint.c:143618`).
pub fn fun_1400a7620(
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
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_10");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x10d1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            0x0u64,
            0x0u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(((memory.read_u64(relocate("STACK_local_30"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_30")], memory);
        }
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[1], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400a76a0` (`mspaint.c:143649`).
pub fn fun_1400a76a0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x10d1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            0x0u64,
            u64::from((u64::from(memory.read_u32(values[2])?)) as u32),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400a7700` (`mspaint.c:143675`).
pub fn fun_1400a7700(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x10d1u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            0x0u64,
            u64::from((u64::from(memory.read_u8(values[2])?)) as u8),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(Some(0x1u64));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400a85d0` (`mspaint.c:144711`).
pub fn fun_1400a85d0(
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
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_48");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_uStack_40");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "__ExceptionPtrCopy",
        &[relocate("STACK_local_48"), values[1]],
        memory,
    );
    values[2] = u64::from(
        (invoke(
            "FUN_1400a8ee0",
            &[
                values[0],
                relocate("STACK_local_48"),
                relocate("STACK_local_38"),
            ],
            memory,
        )) as u8,
    );
    invoke("FUN_140018130", &[relocate("STACK_uStack_30")], memory);
    invoke("__ExceptionPtrDestroy", &[values[1]], memory);
    Ok(Some(u64::from((values[2]) as u8)))
}

/// Translation of `FUN_1400a9090` (`mspaint.c:145310`).
pub fn fun_1400a9090(
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
        u64::from((arguments[1]) as u32),
        arguments[2],
        arguments[3],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    values[8] = (values[0]).wrapping_add(0x8u64);
    values[4] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[8] = relocate("DAT_00000018");
        values[4] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[4])?).wrapping_add(0x28u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[8])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_38");
            let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_30");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_14001fd10",
            &[relocate("STACK_local_28"), relocate("STACK_local_40")],
            memory,
        );
        invoke(
            "_CxxThrowException",
            &[relocate("STACK_local_28"), relocate("DAT_14016a7e0")],
            memory,
        );
        return Err(AdapterError::Halted("_CxxThrowException"));
    }
    values[4] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000028");
    }
    values[5] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = relocate("DAT_00000030");
    }
    values[6] = ((memory.read_u64(values[5])?).wrapping_sub(memory.read_u64(values[4])?) as i64)
        .wrapping_shr(0x2u64 as u32) as u64;
    values[7] = (values[6]) & (0xffffffffu64);
    if u64::from(
        (u64::from((values[1]) as u32)) < (u64::from((u64::from((values[6]) as u32)) as u32)),
    ) != 0
    {
        values[7] = values[1];
    }
    if u64::from(
        ((((values[7]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "memmove",
            &[
                values[2],
                memory.read_u64(values[4])?,
                (values[7]).wrapping_mul(0x4u64),
            ],
            memory,
        );
    }
    {
        let destination = values[4];
        let stored = (memory.read_u64(values[4])?).wrapping_add((values[7]).wrapping_mul(0x4u64));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[3];
        let stored = ((values[7]) as i32 as i64 as u64) as i32 as i64 as u64;
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a9190` (`mspaint.c:145362`).
pub fn fun_1400a9190(
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
    values[6] = (values[0]).wrapping_add(0x8u64);
    values[3] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[6] = relocate("DAT_00000018");
        values[3] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[3])?).wrapping_add(0x28u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[6])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_30");
            let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_28");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_14001fd10",
            &[relocate("STACK_local_20"), relocate("STACK_local_38")],
            memory,
        );
        invoke(
            "_CxxThrowException",
            &[relocate("STACK_local_20"), relocate("DAT_14016a7e0")],
            memory,
        );
        return Err(AdapterError::Halted("_CxxThrowException"));
    }
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000028");
    }
    values[4] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000030");
    }
    values[2] = memory.read_u64(values[4])?;
    values[7] =
        u64::from((u64::from(((memory.read_u64(values[3])?) as i64) == ((values[2]) as i64))) != 0);
    if u64::from((values[7]) == 0) != 0 {
        values[5] = (memory.read_u64(values[3])?).wrapping_add(0x4u64);
        {
            let destination = values[3];
            let stored = values[5];
            memory.write_u64(destination, stored)?;
        }
        values[7] = u64::from((u64::from(((values[5]) as i64) == ((values[2]) as i64))) != 0);
    }
    {
        let destination = values[1];
        let stored = u64::from((u64::from((values[7]) == 0)) != 0);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a9250` (`mspaint.c:145413`).
pub fn fun_1400a9250(
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
    values[5] = (values[0]).wrapping_add(0x8u64);
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[5] = relocate("DAT_00000018");
        values[2] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[2])?).wrapping_add(0x28u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[5])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_50");
            let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_48");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_14001fd10",
            &[relocate("STACK_local_40"), relocate("STACK_local_58")],
            memory,
        );
        invoke(
            "_CxxThrowException",
            &[relocate("STACK_local_40"), relocate("DAT_14016a7e0")],
            memory,
        );
        return Err(AdapterError::Halted("_CxxThrowException"));
    }
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000028");
    }
    values[4] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000030");
    }
    if u64::from((memory.read_u64(values[3])?) == (memory.read_u64(values[4])?)) != 0 {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x612u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = relocate("STACK_local_50");
            let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_48");
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_14001fd50",
            &[relocate("STACK_local_28"), relocate("STACK_local_58")],
            memory,
        );
        invoke(
            "_CxxThrowException",
            &[relocate("STACK_local_28"), relocate("DAT_14016a660")],
            memory,
        );
        return Err(AdapterError::Halted("_CxxThrowException"));
    }
    {
        let destination = values[1];
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32(memory.read_u64(values[3])?)?)) as u32)) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a95e0` (`mspaint.c:145623`).
pub fn fun_1400a95e0(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_38"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_1400a9630",
        &[
            (values[0]).wrapping_add(0x10u64),
            values[1],
            relocate("STACK_local_18"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a9820` (`mspaint.c:145742`).
pub fn fun_1400a9820(
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
    {
        let destination = relocate("STACK_local_res8");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = (invoke(
        "FUN_140001a20",
        &[
            (memory.read_u64(
                (memory.read_u64((values[0]).wrapping_add(0x10u64))?).wrapping_add(0x20u64),
            )?)
            .wrapping_add(0xe8u64),
            relocate("STACK_local_res8"),
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(((memory.read_u64(relocate("STACK_local_res8"))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res8")], memory);
    }
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_14009de70",
            &[memory.read_u64((values[0]).wrapping_add(0x10u64))?],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400aa740` (`mspaint.c:146692`).
pub fn fun_1400aa740(
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
    let values = [arguments[0], arguments[1], u64::from((arguments[2]) as u32)];
    {
        let destination = relocate("STACK_local_10");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_48"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((values[2]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((values[0]).wrapping_add(0x18u64))?,
            memory.read_u64((values[0]).wrapping_add(0x10u64))?,
            relocate("STACK_local_20"),
            relocate("STACK_local_18"),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ab3e0` (`mspaint.c:147374`).
pub fn fun_1400ab3e0(
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
    let values = [arguments[0], (arguments[1]) as i8 as i64 as u64];
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x18u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140001b30", &[(values[0]).wrapping_add(0x8u64)], memory);
    }
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_1400b022c", &[values[0], 0x20u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_1400ab470` (`mspaint.c:147411`).
pub fn fun_1400ab470(
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
    values[2] = invoke("operator_new", &[0x20u64], memory);
    {
        let destination = values[2];
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64(destination, stored)?;
    }
    values[1] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
    {
        let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?],
            memory,
        );
    }
    {
        let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    values[1] = memory.read_u64((values[0]).wrapping_add(0x18u64))?;
    {
        let destination = (values[2]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?],
            memory,
        );
    }
    Ok(Some(values[2]))
}

/// Translation of `FUN_1400ab530` (`mspaint.c:147469`).
pub fn fun_1400ab530(
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
    {
        let destination = values[1];
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64(destination, stored)?;
    }
    values[2] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
    {
        let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[2]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?],
            memory,
        );
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400acec0` (`mspaint.c:148710`).
pub fn fun_1400acec0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_14011cef8"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400ad430` (`mspaint.c:148966`).
pub fn fun_1400ad430(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_14011d028"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400ad730` (`mspaint.c:149121`).
pub fn fun_1400ad730(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_14011cf90"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400adbe0` (`mspaint.c:149350`).
pub fn fun_1400adbe0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64];
    {
        let destination = relocate("STACK_local_18");
        let stored = (memory.read_u64(relocate("DAT_140179040"))?) ^ (relocate("STACK_auStack_68"));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_40")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x19cdu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = relocate("STACK_local_38");
        let stored=relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\base.h\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_30");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = relocate("STACK_local_28");
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[2])?,
            relocate("STACK_local_20"),
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) == (0x0u64)) != 0 {
            {
                let destination = values[1];
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        } else {
            {
                let destination = relocate("STACK_local_28");
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            invoke(
                "INDIRECT_CALL",
                &[
                    memory
                        .read_u64(memory.read_u64(memory.read_u64(relocate("STACK_local_20"))?)?)?,
                    memory.read_u64(relocate("STACK_local_20"))?,
                    relocate("DAT_14011d018"),
                    relocate("STACK_local_28"),
                ],
                memory,
            );
            {
                let destination = values[1];
                let stored = memory.read_u64(relocate("STACK_local_28"))?;
                memory.write_u64(destination, stored)?;
            }
        }
        if u64::from((memory.read_u64(relocate("STACK_local_20"))?) != (0x0u64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_40")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400b0234` (`mspaint.c:154719`).
pub fn fun_1400b0234(
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
    let values = [arguments[0]];
    invoke("operator_new", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_1400b0470` (`mspaint.c:154874`).
pub fn fun_1400b0470(
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
    let mut values = [(arguments[0]) as i32 as i64 as u64, 0u64];
    if u64::from(
        (((values[0]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = relocate("DAT_1401cf480");
            let stored = (0x1u64) as i8 as i64 as u64;
            memory.write_u8(destination, stored as u8)?;
        }
    }
    invoke("FUN_1400b0bdc", &[], memory);
    values[1] = (invoke("FUN_14009b370", &[], memory)) as i8 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[1] = (invoke("FUN_14009b370", &[], memory)) as i8 as i64 as u64;
        if u64::from(
            (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            return Ok(Some(u64::from((0x1u64) as u8)));
        }
        invoke("FUN_14009b370", &[0x0u64], memory);
    }
    Ok(Some(u64::from((0x0u64) as u8)))
}

/// Translation of `FUN_1400b0fb4` (`mspaint.c:155639`).
pub fn fun_1400b0fb4(
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
    let values = [arguments[0]];
    invoke(
        "DeleteCriticalSection",
        &[(values[0]).wrapping_add(0x28u64)],
        memory,
    );
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x50u64))?) != (0x0u64)) != 0 {
        invoke(
            "free",
            &[memory.read_u64((values[0]).wrapping_add(0x50u64))?],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add(0x50u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    {
        let destination = (values[0]).wrapping_add(0x58u64);
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    Ok(None)
}

/// Translation of `FUN_1400b1638` (`mspaint.c:156913`).
pub fn fun_1400b1638(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64];
    values[0] = u64::from((invoke("___lc_codepage_func", &[], memory)) as u32);
    values[2] = 0xfde9u64;
    if u64::from((u64::from((values[0]) as u32)) != (u64::from((0xfde9u64) as u32))) != 0 {
        values[1] = (invoke("AreFileApisANSI", &[], memory)) as i32 as i64 as u64;
        values[2] = u64::from(
            (((values[1]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
        );
    }
    Ok(Some(values[2]))
}

/// Translation of `FUN_1400b1c0c` (`mspaint.c:157266`).
pub fn fun_1400b1c0c(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("abort", &[], memory);
    Err(AdapterError::Halted("abort"))
}

/// Translation of `FUN_1400cce40` (`mspaint.c:176925`).
pub fn fun_1400cce40(
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
    invoke(
        "ATL::CStringT<>::CStringT<>",
        &[(values[1]).wrapping_add(0x38u64)],
        memory,
    );
    values[2] = memory.read_u64((values[1]).wrapping_add(0x30u64))?;
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x28u64))?,
            values[2],
            (values[1]).wrapping_add(0x40u64),
            0x200u64,
            0x0u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("ATL::CStringT<>::Format",&[(values[1]).wrapping_add(0x38u64),relocate("L\"%Ts (%Ts:%d)\""),relocate("L\"Exception thrown in destructor\""),relocate("L\"C:\\\\Program Files\\\\Microsoft Visual Studio\\\\2022\\\\Enterprise\\\\VC\\\\Tools\\\\MSVC\\\\14.44.35207\\\\atlmfc\\\\include\\\\afxwin1.inl\""),0x4du64],memory);
    } else {
        invoke("ATL::CStringT<>::Format",&[(values[1]).wrapping_add(0x38u64),relocate("L\"%Ts (%Ts:%d)\\n%Ts\""),relocate("L\"Exception thrown in destructor\""),relocate("L\"C:\\\\Program Files\\\\Microsoft Visual Studio\\\\2022\\\\Enterprise\\\\VC\\\\Tools\\\\MSVC\\\\14.44.35207\\\\atlmfc\\\\include\\\\afxwin1.inl\""),0x4du64,(values[1]).wrapping_add(0x40u64)],memory);
    }
    invoke(
        "AfxMessageBox",
        &[
            memory.read_u64((values[1]).wrapping_add(0x38u64))?,
            0x0u64,
            0x0u64,
        ],
        memory,
    );
    invoke(
        "ATL::CStringT<>::~CStringT<>",
        &[(values[1]).wrapping_add(0x38u64)],
        memory,
    );
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
            values[2],
            0x1u64,
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cd0f0` (`mspaint.c:176978`).
pub fn fun_1400cd0f0(
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
    invoke(
        "ATL::CStringT<>::CStringT<>",
        &[(values[1]).wrapping_add(0x38u64)],
        memory,
    );
    values[2] = memory.read_u64((values[1]).wrapping_add(0x30u64))?;
    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x28u64))?,
            values[2],
            (values[1]).wrapping_add(0x40u64),
            0x200u64,
            0x0u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("ATL::CStringT<>::Format",&[(values[1]).wrapping_add(0x38u64),relocate("L\"%Ts (%Ts:%d)\""),relocate("L\"Exception thrown in destructor\""),relocate("L\"C:\\\\Program Files\\\\Microsoft Visual Studio\\\\2022\\\\Enterprise\\\\VC\\\\Tools\\\\MSVC\\\\14.44.35207\\\\atlmfc\\\\include\\\\afxwin2.inl\""),0x445u64],memory);
    } else {
        invoke("ATL::CStringT<>::Format",&[(values[1]).wrapping_add(0x38u64),relocate("L\"%Ts (%Ts:%d)\\n%Ts\""),relocate("L\"Exception thrown in destructor\""),relocate("L\"C:\\\\Program Files\\\\Microsoft Visual Studio\\\\2022\\\\Enterprise\\\\VC\\\\Tools\\\\MSVC\\\\14.44.35207\\\\atlmfc\\\\include\\\\afxwin2.inl\""),0x445u64,(values[1]).wrapping_add(0x40u64)],memory);
    }
    invoke(
        "AfxMessageBox",
        &[
            memory.read_u64((values[1]).wrapping_add(0x38u64))?,
            0x0u64,
            0x0u64,
        ],
        memory,
    );
    invoke(
        "ATL::CStringT<>::~CStringT<>",
        &[(values[1]).wrapping_add(0x38u64)],
        memory,
    );
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
            values[2],
            0x1u64,
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cddbc` (`mspaint.c:177257`).
pub fn fun_1400cddbc(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    values[3] = invoke(
        "FUN_1400069f0",
        &[
            memory.read_u64((values[1]).wrapping_add(0x78u64))?,
            (values[1]).wrapping_add(0x58u64),
        ],
        memory,
    );
    invoke(
        "FUN_1400053b0",
        &[
            (values[1]).wrapping_add(0xa8u64),
            relocate("L\"CopyAlternateStreams\""),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[values[2], (values[1]).wrapping_add(0xa8u64), values[3]],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0xa8u64)],
        memory,
    );
    invoke(
        "FUN_140001bc0",
        &[
            (values[1]).wrapping_add(0x80u64),
            memory.read_u64((values[1]).wrapping_add(0x78u64))?,
        ],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[(values[1]).wrapping_add(0x80u64), relocate("DAT_14016aad0")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400cf65c` (`mspaint.c:177835`).
pub fn fun_1400cf65c(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64, 0u64];
    values[2] = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
    values[4] = (invoke("FUN_14003b150", &[values[2]], memory)) as i8 as i64 as u64;
    if u64::from(
        (((values[4]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[3] = invoke(
            "FUN_14000a980",
            &[(values[1]).wrapping_add(0x20u64)],
            memory,
        );
        invoke("FUN_14003af90", &[values[2], values[3]], memory);
        invoke(
            "__ExceptionPtrDestroy",
            &[(values[1]).wrapping_add(0x20u64)],
            memory,
        );
    }
    values[3] = invoke(
        "FUN_14003b1a0",
        &[(values[2]).wrapping_add(0x10u64)],
        memory,
    );
    invoke("FUN_14003b170", &[values[3]], memory);
    Err(AdapterError::Halted("FUN_14003b170"))
}

/// Translation of `FUN_1400d1538` (`mspaint.c:178723`).
pub fn fun_1400d1538(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_14000a870", &[], memory);
    invoke("_CxxThrowException", &[0x0u64, 0x0u64], memory);
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400d28c4` (`mspaint.c:179098`).
pub fn fun_1400d28c4(
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
    let values = [arguments[0], arguments[1]];
    {
        let destination = (values[1]).wrapping_add(0xc8u64);
        let stored = memory.read_u64((values[1]).wrapping_add(0x38u64))?;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "COleStreamFile::Abort",
        &[(values[1]).wrapping_add(0x100u64)],
        memory,
    );
    invoke(
        "COleDocument::CommitItems",
        &[
            memory.read_u64((values[1]).wrapping_add(0x30u64))?,
            0x0u64,
            0x0u64,
        ],
        memory,
    );
    invoke(
        "CArchive::Abort",
        &[(values[1]).wrapping_add(0x50u64)],
        memory,
    );
    invoke("AfxThrowLastCleanup", &[], memory);
    invoke("_CxxThrowException", &[0x0u64, 0x0u64], memory);
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400d2966` (`mspaint.c:179114`).
pub fn fun_1400d2966(
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
    let values = [arguments[0], arguments[1]];
    {
        let destination = (values[1]).wrapping_add(0x48u64);
        let stored = memory.read_u64((values[1]).wrapping_add(0x38u64))?;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "COleStreamFile::Abort",
        &[(values[1]).wrapping_add(0xe0u64)],
        memory,
    );
    invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x30u64))?)?)
                .wrapping_add(0xf8u64),
        )?],
        memory,
    );
    invoke(
        "CArchive::Abort",
        &[(values[1]).wrapping_add(0x50u64)],
        memory,
    );
    invoke("AfxThrowLastCleanup", &[], memory);
    invoke("_CxxThrowException", &[0x0u64, 0x0u64], memory);
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400d6168` (`mspaint.c:180537`).
pub fn fun_1400d6168(
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
    let values = [arguments[0], arguments[1]];
    {
        let destination = (values[1]).wrapping_add(0x48u64);
        let stored = memory.read_u64((values[1]).wrapping_add(0x38u64))?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64((values[1]).wrapping_add(0x30u64))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x30u64))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
    }
    invoke("AfxThrowLastCleanup", &[], memory);
    invoke("_CxxThrowException", &[0x0u64, 0x0u64], memory);
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400d65d0` (`mspaint.c:180691`).
pub fn fun_1400d65d0(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        values[4] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x20u64))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
        invoke(
            "FUN_14008c9d0",
            &[
                values[2],
                memory.read_u64((values[1]).wrapping_add(0x48u64))?,
                relocate("L\"std::exception: %hs\""),
                values[4],
            ],
            memory,
        );
    }
    values[3] = ((u64::from(memory.read_u32((values[1]).wrapping_add(0x50u64))?)) as i32 as i64
        as u64) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[3] = ((0u64.wrapping_sub(0x7ff8fdc2u64)) as i32 as i64 as u64) as i32 as i64 as u64;
    }
    {
        let destination = (values[1]).wrapping_add(0x50u64);
        let stored = (values[3]) as i32 as i64 as u64;
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d66b2` (`mspaint.c:180741`).
pub fn fun_1400d66b2(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x98u64))?;
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        values[4] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x30u64))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
        invoke(
            "FUN_14008c9d0",
            &[
                values[2],
                memory.read_u64((values[1]).wrapping_add(0xa0u64))?,
                relocate("L\"std::exception: %hs\""),
                values[4],
            ],
            memory,
        );
    }
    {
        let destination = (values[1]).wrapping_add(0x48u64);
        let stored = u64::from((0x8007000eu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    values[3] = u64::from((invoke("FUN_140012f40", &[0x8007000eu64], memory)) as u32);
    {
        let destination = (values[1]).wrapping_add(0x4cu64);
        let stored = u64::from((values[3]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x50u64);
        let stored = u64::from((0x0u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d67da` (`mspaint.c:180800`).
pub fn fun_1400d67da(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x98u64))?;
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        values[4] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x38u64))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
        invoke(
            "FUN_14008c9d0",
            &[
                values[2],
                memory.read_u64((values[1]).wrapping_add(0xa0u64))?,
                relocate("L\"std::exception: %hs\""),
                values[4],
            ],
            memory,
        );
    }
    {
        let destination = (values[1]).wrapping_add(0x48u64);
        let stored = u64::from((0x8007000eu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    values[3] = u64::from((invoke("FUN_140012f40", &[0x8007000eu64], memory)) as u32);
    {
        let destination = (values[1]).wrapping_add(0x4cu64);
        let stored = u64::from((values[3]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x50u64);
        let stored = u64::from((0x0u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d684b` (`mspaint.c:180823`).
pub fn fun_1400d684b(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x98u64))?;
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        values[4] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x40u64))?)?)
                    .wrapping_add(0x8u64),
            )?],
            memory,
        );
        invoke(
            "FUN_14008c9d0",
            &[
                values[2],
                memory.read_u64((values[1]).wrapping_add(0xa0u64))?,
                relocate("L\"std::exception: %hs\""),
                values[4],
            ],
            memory,
        );
    }
    {
        let destination = (values[1]).wrapping_add(0x48u64);
        let stored = u64::from((0x8007023eu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    values[3] = u64::from((invoke("FUN_140012f40", &[0x8007023eu64], memory)) as u32);
    {
        let destination = (values[1]).wrapping_add(0x4cu64);
        let stored = u64::from((values[3]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x50u64);
        let stored = u64::from((0x0u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d853e` (`mspaint.c:181432`).
pub fn fun_1400d853e(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add((0x4u64).wrapping_mul(8));
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("FUN_14000a3c0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a3c0"))
}

/// Translation of `FUN_1400d9d3b` (`mspaint.c:182529`).
pub fn fun_1400d9d3b(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x38u64))?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x14u64);
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("FUN_14000a3c0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a3c0"))
}

/// Translation of `FUN_1400d9db3` (`mspaint.c:182561`).
pub fn fun_1400d9db3(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x28u64))?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add((0x4u64).wrapping_mul(8));
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("FUN_14000a3c0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a3c0"))
}

/// Translation of `FUN_1400da0a8` (`mspaint.c:182724`).
pub fn fun_1400da0a8(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x28u64))?;
    {
        let destination = (values[2]).wrapping_add(0x170u64);
        let stored = u64::from((0x186u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x178u64);
        let stored = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\Layer.cpp\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x180u64);
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_14000aa50",
        &[
            (values[1]).wrapping_add(0xb8u64),
            (values[2]).wrapping_add(0x170u64),
        ],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[(values[1]).wrapping_add(0xb8u64), relocate("DAT_14016a960")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400db6b3` (`mspaint.c:183917`).
pub fn fun_1400db6b3(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add((0x5u64).wrapping_mul(8));
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("FUN_14000a3c0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a3c0"))
}

/// Translation of `FUN_1400dbcfa` (`mspaint.c:184207`).
pub fn fun_1400dbcfa(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x38u64))?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add((0x9u64).wrapping_mul(8));
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("FUN_14000a3c0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a3c0"))
}

/// Translation of `FUN_1400dc2cb` (`mspaint.c:184534`).
pub fn fun_1400dc2cb(
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
    let values = [arguments[0], arguments[1]];
    {
        let destination = (values[1]).wrapping_add(0x138u64);
        let stored = (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x160u64);
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x20u64);
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x138u64))?)?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0x92u64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[1]).wrapping_add(0x140u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x190u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x148u64);
            let stored = 0x190u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = memory.read_u64((values[1]).wrapping_add(0x140u64))?;
            let stored = memory.read_u64((values[1]).wrapping_add(0x148u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x168u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x180u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x170u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x160u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x20u64);
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x170u64))?)?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x178u64);
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = memory.read_u64((values[1]).wrapping_add(0x168u64))?;
            let stored = memory.read_u64((values[1]).wrapping_add(0x178u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x180u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x190u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x188u64);
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x180u64))?)?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x190u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x180u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x198u64);
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x190u64))?)?;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0x198u64))?,
                memory.read_u64((values[1]).wrapping_add(0x188u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400dcce3` (`mspaint.c:185044`).
pub fn fun_1400dcce3(
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
    values[3] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    values[2] = memory.read_u64((values[1]).wrapping_add(0x48u64))?;
    {
        let destination = (values[2]).wrapping_add(0x1b8u64);
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x88u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x290u64);
        let stored = relocate("L\"InstallCodecAndRetry\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x298u64);
        let stored = 0x14u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[2]).wrapping_add(0x1c0u64),
            (values[2]).wrapping_add(0x290u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[3],
            (values[2]).wrapping_add(0x1c0u64),
            (values[2]).wrapping_add(0x1b8u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[2]).wrapping_add(0x1c0u64)],
        memory,
    );
    invoke(
        "Paint::PlatformHelpers::CodecInfo::ProductId",
        &[(values[2]).wrapping_add(0xa0u64)],
        memory,
    );
    invoke("FUN_140064070", &[], memory);
    invoke(
        "FUN_140002130",
        &[(values[2]).wrapping_add(0x1c8u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400dd34e` (`mspaint.c:185400`).
pub fn fun_1400dd34e(
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
    values[3] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    values[2] = memory.read_u64((values[1]).wrapping_add(0x50u64))?;
    {
        let destination = (values[2]).wrapping_add(0x218u64);
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0xd8u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x4a8u64);
        let stored = relocate("L\"OnShareStorageItems\"");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add(0x4b0u64);
        let stored = 0x13u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[2]).wrapping_add(0x220u64),
            (values[2]).wrapping_add(0x4a8u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[3],
            (values[2]).wrapping_add(0x220u64),
            (values[2]).wrapping_add(0x218u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[2]).wrapping_add(0x220u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400dd40a` (`mspaint.c:185430`).
pub fn fun_1400dd40a(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x50u64))?;
    {
        let destination = values[2];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[2]).wrapping_add((0x5u64).wrapping_mul(8));
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("FUN_14000a3c0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a3c0"))
}

/// Translation of `FUN_1400dd490` (`mspaint.c:185466`).
pub fn fun_1400dd490(
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
    let values = [arguments[0], arguments[1]];
    {
        let destination = (values[1]).wrapping_add(0x88u64);
        let stored = (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x68u64);
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x20u64);
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x88u64))?)?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0xa2u64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[1]).wrapping_add(0x90u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x80u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x98u64);
            let stored = 0x560u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = memory.read_u64((values[1]).wrapping_add(0x90u64))?;
            let stored = memory.read_u64((values[1]).wrapping_add(0x98u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xb8u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x70u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xc0u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x68u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0x20u64);
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xc0u64))?)?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xc8u64);
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = memory.read_u64((values[1]).wrapping_add(0xb8u64))?;
            let stored = memory.read_u64((values[1]).wrapping_add(0xc8u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xd0u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x80u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xd8u64);
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xd0u64))?)?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xe0u64);
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x70u64);
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add(0xe8u64);
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xe0u64))?)?;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0xe8u64))?,
                memory.read_u64((values[1]).wrapping_add(0xd8u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

pub type Handler = fn(
    &[u64],
    &mut MemoryImage,
    &mut RelocateCall<'_>,
    &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError>;
pub const HANDLERS: &[(&str, usize, Handler)] = &[
    ("FUN_140001df0", 2, fun_140001df0),
    ("FUN_140001e50", 1, fun_140001e50),
    ("FUN_140002ca0", 3, fun_140002ca0),
    ("FUN_140002fc0", 3, fun_140002fc0),
    ("FUN_140003830", 3, fun_140003830),
    ("FUN_140003b60", 3, fun_140003b60),
    ("FUN_1400066d0", 2, fun_1400066d0),
    ("FUN_140006730", 2, fun_140006730),
    ("FUN_1400097e0", 2, fun_1400097e0),
    ("FUN_14000a160", 1, fun_14000a160),
    ("FUN_14000ab00", 1, fun_14000ab00),
    ("FUN_14000abc0", 1, fun_14000abc0),
    ("FUN_14000b480", 3, fun_14000b480),
    ("FUN_14000b9d0", 2, fun_14000b9d0),
    ("FUN_14000baa0", 2, fun_14000baa0),
    ("FUN_14000beb0", 2, fun_14000beb0),
    ("FUN_14000c880", 2, fun_14000c880),
    ("FUN_14000d0a0", 3, fun_14000d0a0),
    ("FUN_140017d00", 1, fun_140017d00),
    ("FUN_140018220", 1, fun_140018220),
    ("FUN_140018550", 3, fun_140018550),
    ("FUN_140018730", 0, fun_140018730),
    ("FUN_1400188e0", 2, fun_1400188e0),
    ("FUN_140018a60", 0, fun_140018a60),
    ("FUN_140018ef0", 1, fun_140018ef0),
    ("FUN_140019d80", 3, fun_140019d80),
    ("FUN_14001a000", 2, fun_14001a000),
    ("FUN_14001c070", 4, fun_14001c070),
    ("FUN_14001c320", 3, fun_14001c320),
    ("FUN_14001c6f0", 4, fun_14001c6f0),
    ("FUN_14001d250", 2, fun_14001d250),
    ("FUN_14001d2d0", 3, fun_14001d2d0),
    ("FUN_14001d340", 3, fun_14001d340),
    ("FUN_14001d5f0", 3, fun_14001d5f0),
    ("FUN_14001db10", 3, fun_14001db10),
    ("FUN_14001ff10", 3, fun_14001ff10),
    ("FUN_140020720", 3, fun_140020720),
    ("FUN_140022cf0", 2, fun_140022cf0),
    ("FUN_140022d40", 1, fun_140022d40),
    ("FUN_140023010", 2, fun_140023010),
    ("FUN_140023640", 2, fun_140023640),
    ("FUN_140031f40", 2, fun_140031f40),
    ("FUN_140031fb0", 2, fun_140031fb0),
    ("FUN_140032550", 2, fun_140032550),
    ("FUN_140033700", 1, fun_140033700),
    ("FUN_140033970", 2, fun_140033970),
    ("FUN_140036bf0", 3, fun_140036bf0),
    ("FUN_140039d90", 1, fun_140039d90),
    ("FUN_14003b220", 1, fun_14003b220),
    ("FUN_14003ba30", 3, fun_14003ba30),
    ("FUN_14003bb40", 3, fun_14003bb40),
    ("FUN_14003c5b0", 1, fun_14003c5b0),
    ("FUN_1400415b0", 3, fun_1400415b0),
    ("FUN_140041680", 2, fun_140041680),
    ("FUN_1400416d0", 2, fun_1400416d0),
    ("FUN_140041720", 2, fun_140041720),
    ("FUN_140041830", 3, fun_140041830),
    ("FUN_1400418b0", 2, fun_1400418b0),
    ("FUN_140041900", 2, fun_140041900),
    ("FUN_1400419a0", 2, fun_1400419a0),
    ("FUN_140042010", 4, fun_140042010),
    ("FUN_140042060", 3, fun_140042060),
    ("FUN_1400423b0", 4, fun_1400423b0),
    ("FUN_1400426f0", 2, fun_1400426f0),
    ("FUN_140042770", 3, fun_140042770),
    ("FUN_1400427e0", 3, fun_1400427e0),
    ("FUN_140042840", 3, fun_140042840),
    ("FUN_1400488c0", 3, fun_1400488c0),
    ("FUN_14004ab10", 2, fun_14004ab10),
    ("FUN_14004b850", 2, fun_14004b850),
    ("FUN_14004cd30", 1, fun_14004cd30),
    ("FUN_140053010", 3, fun_140053010),
    ("FUN_140053730", 2, fun_140053730),
    ("FUN_1400539e0", 0, fun_1400539e0),
    ("FUN_140055f80", 1, fun_140055f80),
    ("FUN_140056280", 2, fun_140056280),
    ("FUN_140056a50", 2, fun_140056a50),
    ("FUN_140056ee0", 2, fun_140056ee0),
    ("FUN_1400573d0", 1, fun_1400573d0),
    ("FUN_140058250", 2, fun_140058250),
    ("FUN_1400583a0", 2, fun_1400583a0),
    ("FUN_14005bb70", 3, fun_14005bb70),
    ("FUN_14005c340", 2, fun_14005c340),
    ("FUN_14005ff50", 0, fun_14005ff50),
    ("FUN_1400604d0", 1, fun_1400604d0),
    ("FUN_140060540", 2, fun_140060540),
    ("FUN_1400622c0", 2, fun_1400622c0),
    ("FUN_140062de0", 3, fun_140062de0),
    ("FUN_140063ee0", 1, fun_140063ee0),
    ("FUN_140064c20", 3, fun_140064c20),
    ("FUN_140065c30", 2, fun_140065c30),
    ("FUN_140066aa0", 2, fun_140066aa0),
    ("FUN_140066d80", 2, fun_140066d80),
    ("FUN_1400697f0", 1, fun_1400697f0),
    ("FUN_1400698f0", 1, fun_1400698f0),
    ("FUN_140069980", 1, fun_140069980),
    ("FUN_140069a10", 1, fun_140069a10),
    ("FUN_140069aa0", 1, fun_140069aa0),
    ("FUN_140069b30", 1, fun_140069b30),
    ("FUN_140069bc0", 1, fun_140069bc0),
    ("FUN_14006b370", 1, fun_14006b370),
    ("FUN_1400779a0", 2, fun_1400779a0),
    ("FUN_140077ea0", 2, fun_140077ea0),
    ("FUN_1400786a0", 3, fun_1400786a0),
    ("FUN_140079120", 3, fun_140079120),
    ("FUN_14007beb0", 1, fun_14007beb0),
    ("FUN_14007bf90", 1, fun_14007bf90),
    ("FUN_14007c7f0", 2, fun_14007c7f0),
    ("FUN_14007ca00", 3, fun_14007ca00),
    ("FUN_14007ecc0", 0, fun_14007ecc0),
    ("FUN_14007efa0", 0, fun_14007efa0),
    ("FUN_14007f000", 0, fun_14007f000),
    ("FUN_14007f070", 0, fun_14007f070),
    ("FUN_14007f240", 0, fun_14007f240),
    ("FUN_140084410", 0, fun_140084410),
    ("FUN_1400844c0", 0, fun_1400844c0),
    ("FUN_140084570", 0, fun_140084570),
    ("FUN_1400853c0", 3, fun_1400853c0),
    ("FUN_140085410", 3, fun_140085410),
    ("FUN_140085700", 3, fun_140085700),
    ("FUN_140085980", 3, fun_140085980),
    ("FUN_140085c00", 3, fun_140085c00),
    ("FUN_140086060", 3, fun_140086060),
    ("FUN_140088b00", 3, fun_140088b00),
    ("FUN_140092970", 3, fun_140092970),
    ("FUN_140093230", 1, fun_140093230),
    ("FUN_1400932c0", 1, fun_1400932c0),
    ("FUN_1400939a0", 3, fun_1400939a0),
    ("FUN_140096440", 1, fun_140096440),
    ("FUN_140096930", 2, fun_140096930),
    ("FUN_140096c40", 3, fun_140096c40),
    ("FUN_140096cd0", 2, fun_140096cd0),
    ("FUN_140096d70", 1, fun_140096d70),
    ("FUN_140097040", 3, fun_140097040),
    ("FUN_140097090", 3, fun_140097090),
    ("FUN_140097230", 3, fun_140097230),
    ("FUN_140097300", 3, fun_140097300),
    ("FUN_140097450", 4, fun_140097450),
    ("FUN_140097490", 2, fun_140097490),
    ("FUN_140097f80", 3, fun_140097f80),
    ("FUN_140097ff0", 2, fun_140097ff0),
    ("FUN_140098130", 2, fun_140098130),
    ("FUN_140098190", 2, fun_140098190),
    ("FUN_1400981f0", 2, fun_1400981f0),
    ("FUN_140098240", 2, fun_140098240),
    ("FUN_1400983b0", 1, fun_1400983b0),
    ("FUN_140099670", 3, fun_140099670),
    ("FUN_140099700", 3, fun_140099700),
    ("FUN_140099770", 2, fun_140099770),
    ("FUN_140099820", 3, fun_140099820),
    ("FUN_140099880", 2, fun_140099880),
    ("FUN_140099ae0", 2, fun_140099ae0),
    ("FUN_140099c30", 2, fun_140099c30),
    ("FUN_14009a190", 3, fun_14009a190),
    ("FUN_14009b180", 3, fun_14009b180),
    ("FUN_14009bd70", 0, fun_14009bd70),
    ("FUN_14009bf20", 2, fun_14009bf20),
    ("FUN_1400a10d0", 1, fun_1400a10d0),
    ("FUN_1400a3320", 3, fun_1400a3320),
    ("FUN_1400a41f0", 2, fun_1400a41f0),
    ("FUN_1400a4ff0", 2, fun_1400a4ff0),
    ("FUN_1400a50f0", 4, fun_1400a50f0),
    ("FUN_1400a5400", 2, fun_1400a5400),
    ("FUN_1400a5480", 2, fun_1400a5480),
    ("FUN_1400a5940", 3, fun_1400a5940),
    ("FUN_1400a59b0", 0, fun_1400a59b0),
    ("FUN_1400a5ae0", 1, fun_1400a5ae0),
    ("FUN_1400a6550", 0, fun_1400a6550),
    ("FUN_1400a6990", 3, fun_1400a6990),
    ("FUN_1400a6a00", 2, fun_1400a6a00),
    ("FUN_1400a6a50", 3, fun_1400a6a50),
    ("FUN_1400a6de0", 3, fun_1400a6de0),
    ("FUN_1400a70c0", 1, fun_1400a70c0),
    ("FUN_1400a7110", 2, fun_1400a7110),
    ("FUN_1400a7620", 1, fun_1400a7620),
    ("FUN_1400a76a0", 3, fun_1400a76a0),
    ("FUN_1400a7700", 3, fun_1400a7700),
    ("FUN_1400a85d0", 2, fun_1400a85d0),
    ("FUN_1400a9090", 4, fun_1400a9090),
    ("FUN_1400a9190", 2, fun_1400a9190),
    ("FUN_1400a9250", 2, fun_1400a9250),
    ("FUN_1400a95e0", 3, fun_1400a95e0),
    ("FUN_1400a9820", 1, fun_1400a9820),
    ("FUN_1400aa740", 3, fun_1400aa740),
    ("FUN_1400ab3e0", 2, fun_1400ab3e0),
    ("FUN_1400ab470", 1, fun_1400ab470),
    ("FUN_1400ab530", 2, fun_1400ab530),
    ("FUN_1400acec0", 3, fun_1400acec0),
    ("FUN_1400ad430", 3, fun_1400ad430),
    ("FUN_1400ad730", 3, fun_1400ad730),
    ("FUN_1400adbe0", 3, fun_1400adbe0),
    ("FUN_1400b0234", 1, fun_1400b0234),
    ("FUN_1400b0470", 1, fun_1400b0470),
    ("FUN_1400b0fb4", 1, fun_1400b0fb4),
    ("FUN_1400b1638", 0, fun_1400b1638),
    ("FUN_1400b1c0c", 0, fun_1400b1c0c),
    ("FUN_1400cce40", 2, fun_1400cce40),
    ("FUN_1400cd0f0", 2, fun_1400cd0f0),
    ("FUN_1400cddbc", 2, fun_1400cddbc),
    ("FUN_1400cf65c", 2, fun_1400cf65c),
    ("FUN_1400d1538", 0, fun_1400d1538),
    ("FUN_1400d28c4", 2, fun_1400d28c4),
    ("FUN_1400d2966", 2, fun_1400d2966),
    ("FUN_1400d6168", 2, fun_1400d6168),
    ("FUN_1400d65d0", 2, fun_1400d65d0),
    ("FUN_1400d66b2", 2, fun_1400d66b2),
    ("FUN_1400d67da", 2, fun_1400d67da),
    ("FUN_1400d684b", 2, fun_1400d684b),
    ("FUN_1400d853e", 2, fun_1400d853e),
    ("FUN_1400d9d3b", 2, fun_1400d9d3b),
    ("FUN_1400d9db3", 2, fun_1400d9db3),
    ("FUN_1400da0a8", 2, fun_1400da0a8),
    ("FUN_1400db6b3", 2, fun_1400db6b3),
    ("FUN_1400dbcfa", 2, fun_1400dbcfa),
    ("FUN_1400dc2cb", 2, fun_1400dc2cb),
    ("FUN_1400dcce3", 2, fun_1400dcce3),
    ("FUN_1400dd34e", 2, fun_1400dd34e),
    ("FUN_1400dd40a", 2, fun_1400dd40a),
    ("FUN_1400dd490", 2, fun_1400dd490),
];
/// Signed byte offsets from a shared caller-selected frame anchor.
pub const FRAME_OFFSETS: &[(&str, &[(&str, i32)])] = &[
    (
        "FUN_140001df0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140001e50",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140002fc0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140003830",
        &[
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140003b60",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400066d0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140006730",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400097e0",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14000a160",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_res10", 16),
            ("STACK_local_res18", 24),
        ],
    ),
    (
        "FUN_14000ab00",
        &[("STACK_auStack_458", -1112), ("STACK_local_18", -24)],
    ),
    (
        "FUN_14000abc0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14000b480",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14000b9d0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14000baa0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14000beb0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14000c880",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14000d0a0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140017d00",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140018220",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140018550",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140018730",
        &[
            ("STACK_local_28", -40),
            ("STACK_uStack_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400188e0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140018a60",
        &[
            ("STACK_local_58", -88),
            ("STACK_uStack_54", -84),
            ("STACK_uStack_50", -80),
            ("STACK_uStack_4c", -76),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140018ef0",
        &[
            ("STACK_local_58", -88),
            ("STACK_uStack_54", -84),
            ("STACK_uStack_50", -80),
            ("STACK_uStack_4c", -76),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
        ],
    ),
    (
        "FUN_140019d80",
        &[
            ("STACK_local_c8", -200),
            ("STACK_uStack_c4", -196),
            ("STACK_uStack_c0", -192),
            ("STACK_uStack_bc", -188),
            ("STACK_local_b8", -184),
            ("STACK_local_98", -152),
        ],
    ),
    (
        "FUN_14001a000",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14001c070",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14001c320",
        &[("STACK_local_18", -24), ("STACK_uStack_10", -16)],
    ),
    (
        "FUN_14001c6f0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14001d250",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14001d2d0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14001d340",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14001d5f0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14001db10",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14001ff10",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140020720",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140022cf0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    ("FUN_140022d40", &[("STACK_local_18", -24)]),
    (
        "FUN_140023010",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140023640",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140031f40",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140031fb0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140032550",
        &[("STACK_local_18", -24), ("STACK_local_10", -16)],
    ),
    (
        "FUN_140033700",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_res10", 16),
            ("STACK_local_res18", 24),
        ],
    ),
    (
        "FUN_140033970",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140036bf0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14003b220",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14003ba30",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14003bb40",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14003c5b0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400415b0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140041680",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400416d0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140041720",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140041830",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400418b0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140041900",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400419a0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140042010",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140042060",
        &[("STACK_local_18", -24), ("STACK_uStack_10", -16)],
    ),
    (
        "FUN_1400423b0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400426f0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140042770",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400427e0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140042840",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400488c0",
        &[
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_14004cd30",
        &[
            ("STACK_auStack_98", -152),
            ("STACK_local_78", -120),
            ("STACK_local_70", -112),
            ("STACK_local_58", -88),
            ("STACK_uStack_50", -80),
            ("STACK_local_48", -72),
            ("STACK_uStack_40", -64),
            ("STACK_local_38", -56),
            ("STACK_uStack_34", -52),
            ("STACK_uStack_30", -48),
            ("STACK_uStack_2c", -44),
            ("STACK_local_28", -40),
            ("STACK_uStack_24", -36),
            ("STACK_uStack_20", -32),
            ("STACK_uStack_1c", -28),
            ("STACK_local_18", -24),
        ],
    ),
    ("FUN_140053010", &[("STACK_local_res8", 8)]),
    (
        "FUN_140053730",
        &[
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400539e0",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    ("FUN_140055f80", &[("STACK_local_18", -24)]),
    (
        "FUN_140056280",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140056a50",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140056ee0",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400573d0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140058250",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400583a0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_uStack_40", -64),
            ("STACK_local_38", -56),
            ("STACK_uStack_30", -48),
            ("STACK_local_28", -40),
            ("STACK_uStack_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14005bb70",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14005c340",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400604d0",
        &[("STACK_local_18", -24), ("STACK_uStack_10", -16)],
    ),
    (
        "FUN_140060540",
        &[("STACK_local_18", -24), ("STACK_uStack_10", -16)],
    ),
    (
        "FUN_140062de0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140063ee0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140064c20",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140065c30",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140066aa0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140066d80",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400697f0",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400698f0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140069980",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140069a10",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140069aa0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140069b30",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140069bc0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400779a0",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140077ea0",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_68", -104),
            ("STACK_local_60", -96),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    ("FUN_1400786a0", &[("STACK_local_res8", 8)]),
    (
        "FUN_140079120",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007beb0",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_res18", 24),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007bf90",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_res18", 24),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007c7f0",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007ca00",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14007ecc0",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007efa0",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007f000",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007f070",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14007f240",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140084410",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400844c0",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140084570",
        &[
            ("STACK_local_res10", 16),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_1c", -28),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400853c0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140085410",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140085700",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140085980",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140085c00",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140086060",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140088b00",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140092970",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140093230",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400932c0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400939a0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140096930",
        &[
            ("STACK_auStack_88", -136),
            ("STACK_local_60", -96),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140096c40",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140097040",
        &[("STACK_local_res10", 16), ("STACK_local_18", -24)],
    ),
    (
        "FUN_140097090",
        &[("STACK_local_res10", 16), ("STACK_local_18", -24)],
    ),
    (
        "FUN_140097230",
        &[
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_140097300",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140097f80",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140097ff0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    ("FUN_140098130", &[("STACK_local_res8", 8)]),
    ("FUN_1400981f0", &[("STACK_local_res8", 8)]),
    (
        "FUN_140098240",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400983b0",
        &[
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_140099670",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140099700",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140099770",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140099820",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_140099880",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    ("FUN_140099ae0", &[("STACK_local_res8", 8)]),
    (
        "FUN_140099c30",
        &[
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_14009a190",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_14009b180",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    ("FUN_14009bd70", &[("STACK_local_28", -40)]),
    (
        "FUN_1400a3320",
        &[
            ("STACK_auStack_58", -88),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a41f0",
        &[
            ("STACK_auStack_78", -120),
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400a4ff0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400a50f0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400a5400",
        &[
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a5480",
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
        "FUN_1400a5940",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a6990",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    ("FUN_1400a6a00", &[("STACK_local_res8", 8)]),
    (
        "FUN_1400a6a50",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a6de0",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a7620",
        &[
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400a76a0",
        &[
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a7700",
        &[
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400a85d0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_48", -72),
            ("STACK_uStack_40", -64),
            ("STACK_local_38", -56),
            ("STACK_uStack_30", -48),
            ("STACK_local_28", -40),
            ("STACK_uStack_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400a9090",
        &[
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400a9190",
        &[
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
        ],
    ),
    (
        "FUN_1400a9250",
        &[
            ("STACK_local_58", -88),
            ("STACK_local_50", -80),
            ("STACK_local_48", -72),
            ("STACK_local_40", -64),
            ("STACK_local_28", -40),
        ],
    ),
    (
        "FUN_1400a95e0",
        &[
            ("STACK_auStack_38", -56),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    ("FUN_1400a9820", &[("STACK_local_res8", 8)]),
    (
        "FUN_1400aa740",
        &[
            ("STACK_auStack_48", -72),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
            ("STACK_local_10", -16),
        ],
    ),
    (
        "FUN_1400acec0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400ad430",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400ad730",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
    (
        "FUN_1400adbe0",
        &[
            ("STACK_auStack_68", -104),
            ("STACK_local_40", -64),
            ("STACK_local_38", -56),
            ("STACK_local_30", -48),
            ("STACK_local_28", -40),
            ("STACK_local_20", -32),
            ("STACK_local_18", -24),
        ],
    ),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    (
        "FUN_140001df0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 1),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140001e50",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 1),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140002fc0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140003830",
        &[
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
        ],
    ),
    (
        "FUN_140003b60",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400066d0",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 4),
            ("STACK_local_28", 8),
            ("STACK_local_20", 1),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140006730",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 4),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400097e0",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14000a160",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_res10", 8),
            ("STACK_local_res18", 8),
        ],
    ),
    (
        "FUN_14000ab00",
        &[("STACK_auStack_458", 1088), ("STACK_local_18", 8)],
    ),
    (
        "FUN_14000abc0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14000b480",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14000b9d0",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14000baa0",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14000beb0",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14000c880",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14000d0a0",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140017d00",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140018220",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140018550",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140018730",
        &[
            ("STACK_local_28", 8),
            ("STACK_uStack_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400188e0",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140018a60",
        &[
            ("STACK_local_58", 4),
            ("STACK_uStack_54", 4),
            ("STACK_uStack_50", 4),
            ("STACK_uStack_4c", 4),
            ("STACK_local_48", 16),
            ("STACK_local_38", 56),
        ],
    ),
    (
        "FUN_140018ef0",
        &[
            ("STACK_local_58", 4),
            ("STACK_uStack_54", 4),
            ("STACK_uStack_50", 4),
            ("STACK_uStack_4c", 4),
            ("STACK_local_48", 16),
            ("STACK_local_38", 56),
        ],
    ),
    (
        "FUN_140019d80",
        &[
            ("STACK_local_c8", 4),
            ("STACK_uStack_c4", 4),
            ("STACK_uStack_c0", 4),
            ("STACK_uStack_bc", 4),
            ("STACK_local_b8", 32),
            ("STACK_local_98", 144),
        ],
    ),
    (
        "FUN_14001a000",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14001c070",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14001c320",
        &[("STACK_local_18", 8), ("STACK_uStack_10", 4)],
    ),
    (
        "FUN_14001c6f0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14001d250",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14001d2d0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14001d340",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14001d5f0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14001db10",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14001ff10",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140020720",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140022cf0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    ("FUN_140022d40", &[("STACK_local_18", 16)]),
    (
        "FUN_140023010",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140023640",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140031f40",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 1),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140031fb0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 1),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140032550",
        &[("STACK_local_18", 8), ("STACK_local_10", 4)],
    ),
    (
        "FUN_140033700",
        &[
            ("STACK_local_res8", 8),
            ("STACK_local_res10", 8),
            ("STACK_local_res18", 8),
        ],
    ),
    (
        "FUN_140033970",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140036bf0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14003b220",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 1),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14003ba30",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14003bb40",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14003c5b0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400415b0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140041680",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400416d0",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140041720",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140041830",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400418b0",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140041900",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400419a0",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140042010",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140042060",
        &[("STACK_local_18", 8), ("STACK_uStack_10", 4)],
    ),
    (
        "FUN_1400423b0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400426f0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140042770",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400427e0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140042840",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400488c0",
        &[
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_14004cd30",
        &[
            ("STACK_auStack_98", 32),
            ("STACK_local_78", 8),
            ("STACK_local_70", 4),
            ("STACK_local_58", 8),
            ("STACK_uStack_50", 8),
            ("STACK_local_48", 8),
            ("STACK_uStack_40", 8),
            ("STACK_local_38", 4),
            ("STACK_uStack_34", 4),
            ("STACK_uStack_30", 4),
            ("STACK_uStack_2c", 4),
            ("STACK_local_28", 4),
            ("STACK_uStack_24", 4),
            ("STACK_uStack_20", 4),
            ("STACK_uStack_1c", 4),
            ("STACK_local_18", 8),
        ],
    ),
    ("FUN_140053010", &[("STACK_local_res8", 8)]),
    (
        "FUN_140053730",
        &[
            ("STACK_local_48", 16),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400539e0",
        &[
            ("STACK_local_res10", 24),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    ("FUN_140055f80", &[("STACK_local_18", 16)]),
    (
        "FUN_140056280",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140056a50",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140056ee0",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400573d0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140058250",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400583a0",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 8),
            ("STACK_uStack_40", 8),
            ("STACK_local_38", 8),
            ("STACK_uStack_30", 8),
            ("STACK_local_28", 8),
            ("STACK_uStack_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14005bb70",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14005c340",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400604d0",
        &[("STACK_local_18", 8), ("STACK_uStack_10", 8)],
    ),
    (
        "FUN_140060540",
        &[("STACK_local_18", 8), ("STACK_uStack_10", 8)],
    ),
    (
        "FUN_140062de0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140063ee0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140064c20",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140065c30",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140066aa0",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140066d80",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400697f0",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 16),
        ],
    ),
    (
        "FUN_1400698f0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140069980",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140069a10",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140069aa0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140069b30",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140069bc0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400779a0",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140077ea0",
        &[
            ("STACK_auStack_88", 32),
            ("STACK_local_68", 8),
            ("STACK_local_60", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 24),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    ("FUN_1400786a0", &[("STACK_local_res8", 8)]),
    (
        "FUN_140079120",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007beb0",
        &[
            ("STACK_local_res10", 8),
            ("STACK_local_res18", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007bf90",
        &[
            ("STACK_local_res10", 8),
            ("STACK_local_res18", 8),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007c7f0",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007ca00",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14007ecc0",
        &[
            ("STACK_local_res10", 24),
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007efa0",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007f000",
        &[
            ("STACK_local_res10", 24),
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007f070",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14007f240",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140084410",
        &[
            ("STACK_local_res10", 24),
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400844c0",
        &[
            ("STACK_local_res10", 24),
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140084570",
        &[
            ("STACK_local_res10", 24),
            ("STACK_local_28", 8),
            ("STACK_local_20", 4),
            ("STACK_local_1c", 4),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400853c0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140085410",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140085700",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140085980",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140085c00",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140086060",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140088b00",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140092970",
        &[
            ("STACK_auStack_58", 40),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140093230",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400932c0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400939a0",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140096930",
        &[
            ("STACK_auStack_88", 40),
            ("STACK_local_60", 56),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140096c40",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140097040",
        &[("STACK_local_res10", 24), ("STACK_local_18", 16)],
    ),
    (
        "FUN_140097090",
        &[("STACK_local_res10", 24), ("STACK_local_18", 16)],
    ),
    (
        "FUN_140097230",
        &[
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_140097300",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140097f80",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140097ff0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    ("FUN_140098130", &[("STACK_local_res8", 8)]),
    ("FUN_1400981f0", &[("STACK_local_res8", 32)]),
    (
        "FUN_140098240",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400983b0",
        &[
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_140099670",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140099700",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140099770",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140099820",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_140099880",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    ("FUN_140099ae0", &[("STACK_local_res8", 32)]),
    (
        "FUN_140099c30",
        &[
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_14009a190",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_14009b180",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    ("FUN_14009bd70", &[("STACK_local_28", 40)]),
    (
        "FUN_1400a3320",
        &[
            ("STACK_auStack_58", 32),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a41f0",
        &[
            ("STACK_auStack_78", 32),
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400a4ff0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400a50f0",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 4),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
        ],
    ),
    (
        "FUN_1400a5400",
        &[
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a5480",
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
        "FUN_1400a5940",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a6990",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    ("FUN_1400a6a00", &[("STACK_local_res8", 32)]),
    (
        "FUN_1400a6a50",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a6de0",
        &[
            ("STACK_auStack_48", 32),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a7620",
        &[
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400a76a0",
        &[
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a7700",
        &[
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400a85d0",
        &[
            ("STACK_auStack_68", 32),
            ("STACK_local_48", 8),
            ("STACK_uStack_40", 8),
            ("STACK_local_38", 8),
            ("STACK_uStack_30", 8),
            ("STACK_local_28", 8),
            ("STACK_uStack_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400a9090",
        &[
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 32),
        ],
    ),
    (
        "FUN_1400a9190",
        &[
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 32),
        ],
    ),
    (
        "FUN_1400a9250",
        &[
            ("STACK_local_58", 8),
            ("STACK_local_50", 8),
            ("STACK_local_48", 8),
            ("STACK_local_40", 24),
            ("STACK_local_28", 40),
        ],
    ),
    (
        "FUN_1400a95e0",
        &[
            ("STACK_auStack_38", 32),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    ("FUN_1400a9820", &[("STACK_local_res8", 8)]),
    (
        "FUN_1400aa740",
        &[
            ("STACK_auStack_48", 40),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
            ("STACK_local_10", 8),
        ],
    ),
    (
        "FUN_1400acec0",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400ad430",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400ad730",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
    (
        "FUN_1400adbe0",
        &[
            ("STACK_auStack_68", 40),
            ("STACK_local_40", 8),
            ("STACK_local_38", 8),
            ("STACK_local_30", 8),
            ("STACK_local_28", 8),
            ("STACK_local_20", 8),
            ("STACK_local_18", 8),
        ],
    ),
];
