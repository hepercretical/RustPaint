//! Checked translations generated from original bodies by batch 011.
//! Resolve `STACK_local_*` to caller-owned scratch; sizes are in `SCRATCH_LAYOUTS`.
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
/// Translation of `FUN_140001010` (`mspaint.c:5124`).
pub fn fun_140001010(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    {
        let destination = relocate("DAT_1401d0510");
        let stored = invoke("operator_new", &[0x30u64], memory);
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = memory.read_u64(relocate("DAT_1401d0510"))?;
        let stored = memory.read_u64(relocate("DAT_1401d0510"))?;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("DAT_1401d0510"))?).wrapping_add(0x8u64);
        let stored = memory.read_u64(relocate("DAT_1401d0510"))?;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("DAT_1401d0510"))?).wrapping_add(0x10u64);
        let stored = memory.read_u64(relocate("DAT_1401d0510"))?;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (memory.read_u64(relocate("DAT_1401d0510"))?).wrapping_add(0x18u64);
        let stored = u64::from((0x101u64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    invoke("atexit", &[relocate("FUN_1400df750")], memory);
    Ok(None)
}

/// Translation of `FUN_140001bc0` (`mspaint.c:5757`).
pub fn fun_140001bc0(
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
    {
        let destination = values[0];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = u64::from((0xaabbccddu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[0]).wrapping_add(0xcu64);
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0xcu64))?)) as u32))
                as u32,
        );
        memory.write_u32(destination, stored as u32)?;
    }
    values[2] = memory.read_u64((values[1]).wrapping_add(0x10u64))?;
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
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
    Ok(Some(values[0]))
}

/// Translation of `FUN_140002770` (`mspaint.c:6398`).
pub fn fun_140002770(
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
    let values = [
        arguments[0],
        (arguments[1]) as i32 as i64 as u64,
        arguments[2],
    ];
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = values[0];
            let stored = (values[1]) as i32 as i64 as u64;
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(values[0]));
    }
    invoke("FUN_140002540", &[values[1], values[2]], memory);
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400031b0` (`mspaint.c:7173`).
pub fn fun_1400031b0(
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
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x28u64;
    }
    invoke(
        "FUN_1400028d0",
        &[values[2], relocate("STACK_local_res8")],
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

/// Translation of `FUN_140003e90` (`mspaint.c:8029`).
pub fn fun_140003e90(
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
    values[1] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = relocate("DAT_00000010");
    }
    invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x10u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140004150` (`mspaint.c:8208`).
pub fn fun_140004150(
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

/// Translation of `FUN_1400045c0` (`mspaint.c:8505`).
pub fn fun_1400045c0(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64((values[0]).wrapping_add(0x18u64))?,
            ((u64::from(memory.read_u32((values[0]).wrapping_add(0x20u64))?)) as i32 as i64 as u64)
                .wrapping_add(memory.read_u64((values[0]).wrapping_add(0x10u64))?),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140004ba0` (`mspaint.c:8906`).
pub fn fun_140004ba0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], arguments[3], 0u64];
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        values[4] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x30u64),
            )?],
            memory,
        );
        return Ok(Some(values[4]));
    }
    {
        let destination = values[3];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x8007000eu64))
}

/// Translation of `FUN_140004bd0` (`mspaint.c:8923`).
pub fn fun_140004bd0(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        values[1] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x38u64),
            )?],
            memory,
        );
        return Ok(Some(values[1]));
    }
    Ok(Some(0x8007000eu64))
}

/// Translation of `FUN_140004bf0` (`mspaint.c:8939`).
pub fn fun_140004bf0(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        values[1] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x40u64),
            )?],
            memory,
        );
        return Ok(Some(values[1]));
    }
    Ok(Some(0x8007000eu64))
}

/// Translation of `FUN_140004d80` (`mspaint.c:9014`).
pub fn fun_140004d80(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    values[1] = invoke("FUN_140004e70", &[relocate("STACK_local_res8")], memory);
    values[0] =
        ((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32 as i64 as u64;
    invoke("RoTransformError", &[values[0], 0x0u64, 0x0u64], memory);
    if u64::from(
        (u64::from(
            (((values[0]) as i32 as i64 as u64) as i64)
                != ((((0u64.wrapping_sub(0x7ffefef8u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                    as i64),
        )) != 0
            && (u64::from(
                (((values[0]) as i32 as i64 as u64) as i64)
                    != ((((0u64.wrapping_sub(0x7ff8f946u64)) as i32 as i64 as u64) as i32 as i64
                        as u64) as i64),
            )) != 0,
    ) != 0
    {
        return Ok(Some(u64::from(
            (u64::from(
                (((values[0]) as i32 as i64 as u64) as i64)
                    != ((((0u64.wrapping_sub(0x76fdffffu64)) as i32 as i64 as u64) as i32 as i64
                        as u64) as i64),
            )) != 0,
        )));
    }
    Ok(Some(0u64))
}

/// Translation of `FUN_1400056f0` (`mspaint.c:9539`).
pub fn fun_1400056f0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "std::_Xlength_error",
        &[relocate("s_string_too_long_140105720")],
        memory,
    );
    Err(AdapterError::Halted("std::_Xlength_error"))
}

/// Translation of `FUN_140005800` (`mspaint.c:9603`).
pub fn fun_140005800(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_140005790", &[relocate("STACK_local_28")], memory);
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_28"), relocate("DAT_14016ab38")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140006680` (`mspaint.c:10434`).
pub fn fun_140006680(
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
    Ok(None)
}

/// Translation of `FUN_140006790` (`mspaint.c:10510`).
pub fn fun_140006790(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64((values[0]).wrapping_add(0x8u64))?).wrapping_add(0x8u64),
            )?,
            (values[0]).wrapping_add(0x8u64),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400067b0` (`mspaint.c:10521`).
pub fn fun_1400067b0(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64((values[0]).wrapping_add(0x8u64))?).wrapping_add(0x10u64),
            )?,
            (values[0]).wrapping_add(0x8u64),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400067d0` (`mspaint.c:10532`).
pub fn fun_1400067d0(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64((values[0]).wrapping_add(0x8u64))?).wrapping_add(0x18u64),
            )?,
            (values[0]).wrapping_add(0x8u64),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140006920` (`mspaint.c:10654`).
pub fn fun_140006920(
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
    values[4] = invoke(
        "FUN_1400063d0",
        &[(values[0]).wrapping_add(0x8u64), relocate("STACK_local_28")],
        memory,
    );
    values[3] = memory.read_u64((values[4]).wrapping_add((0x1u64).wrapping_mul(8)))?;
    values[2] = u64::from(
        (u64::from(
            (u64::from(memory.read_u32((values[4]).wrapping_add((0x2u64).wrapping_mul(8)))?))
                as u32,
        )) as u32,
    );
    {
        let destination = values[1];
        let stored = memory.read_u64(values[4])?;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = values[3];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = u64::from((values[2]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400091c0` (`mspaint.c:12675`).
pub fn fun_1400091c0(
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
    values[2] = memory.read_u64(values[0])?;
    values[4] = 0x0u64;
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = ((u64::from(memory.read_u32((values[2]).wrapping_add(0x60u64))?)) as i32 as i64
            as u64) as i32 as i64 as u64;
        memory.write_u32(destination, stored as u32)?;
    }
    if u64::from(
        ((((u64::from(
            memory.read_u32((relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)))?,
        )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = (0x1u64) as i32 as i64 as u64;
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add(0x60u64);
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
    }
    values[1] = (values[2]).wrapping_add(0x50u64);
    if u64::from((relocate("STACK_local_20")) != (values[1])) != 0 {
        values[4] = memory.read_u64(values[1])?;
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_20");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    invoke(
        "ReleaseSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from(((values[4]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_140009eb0",
            &[
                relocate("STACK_local_20"),
                values[2],
                relocate("STACK_local_28"),
            ],
            memory,
        );
        values[5] = memory.read_u64(relocate("STACK_local_20"))?;
    }
    if u64::from(((values[5]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
    }
    values[2] = memory.read_u64(values[0])?;
    values[3] = (invoke("FUN_140003ae0", &[values[2]], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(u64::from(
        (u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0,
    )))
}

/// Translation of `FUN_1400092f0` (`mspaint.c:12751`).
pub fn fun_1400092f0(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    values[1] = invoke("FUN_140003ae0", &[values[2]], memory);
    if u64::from(
        ((((values[1]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (values[2]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64),
                )?,
                (values[2]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64),
            ],
            memory,
        );
        values[1] &= 0xffffffffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140009560` (`mspaint.c:12905`).
pub fn fun_140009560(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    values[3] = invoke(
        "FUN_140009d60",
        &[values[2], relocate("STACK_local_res8")],
        memory,
    );
    {
        let destination = values[1];
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[3])?)) as u32)) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400095c0` (`mspaint.c:12949`).
pub fn fun_1400095c0(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    values[1] = invoke("FUN_140003ae0", &[values[2]], memory);
    if u64::from(
        ((((values[1]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (values[2]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64),
                )?,
                (values[2]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64),
            ],
            memory,
        );
        values[1] &= 0xffffffffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400096b0` (`mspaint.c:13025`).
pub fn fun_1400096b0(
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
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[3]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[3]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = u64::from(
            (u64::from((u64::from(memory.read_u8((values[3]).wrapping_add(0x68u64))?)) as u8))
                as u8,
        );
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[3]).wrapping_add(0x48u64)],
            memory,
        );
        {
            let destination = values[1];
            let stored = u64::from((values[2]) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140009d30", &[values[3]], memory);
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_20"), relocate("STACK_local_38")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_20"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140009760` (`mspaint.c:13060`).
pub fn fun_140009760(
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
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000048");
    }
    invoke("AcquireSRWLockExclusive", &[values[3]], memory);
    values[2] = (values[0]).wrapping_add(0x40u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000050");
    }
    values[2] = memory.read_u64(values[2])?;
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
    invoke("ReleaseSRWLockExclusive", &[values[3]], memory);
    {
        let destination = values[1];
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400099b0` (`mspaint.c:13195`).
pub fn fun_1400099b0(
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
    let mut values = [arguments[0], arguments[1], arguments[2], arguments[3], 0u64];
    invoke(
        "FUN_140004ee0",
        &[
            relocate("STACK_local_10"),
            (values[0]).wrapping_add(0x48u64),
            values[2],
            values[3],
            values[0],
        ],
        memory,
    );
    values[4] = invoke("FUN_14000a980", &[relocate("STACK_local_20")], memory);
    invoke(
        "FUN_14000a9b0",
        &[(values[0]).wrapping_add(0x38u64), values[4]],
        memory,
    );
    invoke(
        "__ExceptionPtrDestroy",
        &[relocate("STACK_local_20")],
        memory,
    );
    values[4] = invoke(
        "FUN_14000a950",
        &[
            relocate("STACK_local_20"),
            (values[0]).wrapping_add(0x38u64),
        ],
        memory,
    );
    invoke("FUN_14000a940", &[values[4]], memory);
    Err(AdapterError::Halted("FUN_14000a940"))
}

/// Translation of `FUN_140009a40` (`mspaint.c:13225`).
pub fn fun_140009a40(
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
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke("BasePaint::Image::~Image", &[values[1]], memory);
        invoke("FUN_1400b022c", &[values[1], 0x58u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140009b30` (`mspaint.c:13275`).
pub fn fun_140009b30(
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
    if u64::from((values[0]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x10u64))?,
                values[0],
                0x1u64,
            ],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140009e00` (`mspaint.c:13450`).
pub fn fun_140009e00(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x20u64))?) != (0x0u64)) != 0 {
        invoke(
            "GlobalFree",
            &[memory.read_u64((values[0]).wrapping_add(0x20u64))?],
            memory,
        );
    }
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x30u64))?) != (0x0u64)) != 0 {
        invoke(
            "GlobalFree",
            &[memory.read_u64((values[0]).wrapping_add(0x30u64))?],
            memory,
        );
    }
    invoke(
        "FreePropVariantArray",
        &[0x64u64, (values[0]).wrapping_add(0x38u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u8((values[0]).wrapping_add(0xa28u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_140017f70", &[], memory);
    }
    invoke(
        "FUN_140017d00",
        &[(values[0]).wrapping_add(0x9b8u64)],
        memory,
    );
    if u64::from(
        (u64::from(
            ((((u64::from(memory.read_u8((values[0]).wrapping_add(0x9b0u64))?)) as i8 as i64 as u64)
                as i32 as i64 as u64) as i64)
                != (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0
            && (u64::from(
                ((memory.read_u64((values[0]).wrapping_add(0x9a0u64))?) as i64)
                    != ((0x0u64) as i64),
            )) != 0,
    ) != 0
    {
        invoke("FUN_140006970", &[], memory);
    }
    if u64::from(
        ((memory.read_u64((values[0]).wrapping_add(0x998u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0xa60u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140009eb0` (`mspaint.c:13480`).
pub fn fun_140009eb0(
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
    values[6] = (values[1]).wrapping_add(0x10u64);
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x1039u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[6],
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

/// Translation of `FUN_14000a0c0` (`mspaint.c:13605`).
pub fn fun_14000a0c0(
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
    invoke("FUN_140009a80", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000a3c0` (`mspaint.c:13776`).
pub fn fun_14000a3c0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_14000a4a0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a4a0"))
}

/// Translation of `FUN_14000a4a0` (`mspaint.c:13820`).
pub fn fun_14000a4a0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    values[0] = invoke("FUN_140004e70", &[relocate("STACK_local_res8")], memory);
    values[1] = u64::from((invoke("FUN_140002190", &[values[0]], memory)) as u32);
    invoke("RoFailFastWithErrorContext", &[values[1]], memory);
    invoke("abort", &[], memory);
    Err(AdapterError::Halted("abort"))
}

/// Translation of `FUN_14000a550` (`mspaint.c:13879`).
pub fn fun_14000a550(
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
    values[2] = u64::from((invoke("GetLastError", &[], memory)) as u32);
    values[3] = u64::from((invoke("FUN_14000a540", &[values[2]], memory)) as u32);
    values[1] = invoke(
        "FUN_140001b50",
        &[relocate("STACK_local_res10"), values[3]],
        memory,
    );
    invoke(
        "FUN_140002540",
        &[
            u64::from((u64::from(memory.read_u32(values[1])?)) as u32),
            values[0],
        ],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_14000a5c0` (`mspaint.c:13914`).
pub fn fun_14000a5c0(
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
    values[2] = (invoke(
        "FUN_14000a630",
        &[
            values[1],
            memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?,
            memory.read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(8)))?,
        ],
        memory,
    )) as i8 as i64 as u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "INDIRECT_CALL",
            &[memory
                .read_u64(memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?)?],
            memory,
        );
    }
    if u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[values[1]], memory);
    }
    invoke("FUN_1400b022c", &[values[1], 0x18u64], memory);
    Ok(None)
}

/// Translation of `FUN_14000a610` (`mspaint.c:13934`).
pub fn fun_14000a610(
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
        "INDIRECT_CALL",
        &[memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000c240` (`mspaint.c:15594`).
pub fn fun_14000c240(
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
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x146bu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }

    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");
    if u64::from(
        ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x14u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            < (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140002540",
            &[
                (u64::from(memory.read_u32((values[0]).wrapping_add(0x14u64))?)) as i32 as i64
                    as u64,
                relocate("STACK_local_38"),
            ],
            memory,
        );
        return Err(AdapterError::Halted("FUN_140002540"));
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    if u64::from(
        ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            != (((0x2u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x3cu64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        values[1] = (invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(
                    memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?,
                )?)
                .wrapping_add(0x40u64),
            )?],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                < (((values[1]) as i32 as i64 as u64) as i64),
        ) != 0
        {
            return Ok(None);
        }
        invoke(
            "FUN_140002540",
            &[values[1], relocate("STACK_local_38")],
            memory,
        );
        return Err(AdapterError::Halted("FUN_140002540"));
    }
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x1404u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_14000aa50",
        &[relocate("STACK_local_20"), relocate("STACK_local_38")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_20"), relocate("DAT_14016a960")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_14000c570` (`mspaint.c:15762`).
pub fn fun_14000c570(
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
    values[2] = memory.read_u64(values[0])?;
    values[4] = 0x0u64;
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = ((u64::from(memory.read_u32((values[2]).wrapping_add(0x60u64))?)) as i32 as i64
            as u64) as i32 as i64 as u64;
        memory.write_u32(destination, stored as u32)?;
    }
    if u64::from(
        ((((u64::from(
            memory.read_u32((relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)))?,
        )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = (0x1u64) as i32 as i64 as u64;
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add(0x60u64);
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
    }
    values[1] = (values[2]).wrapping_add(0x50u64);
    if u64::from((relocate("STACK_local_20")) != (values[1])) != 0 {
        values[4] = memory.read_u64(values[1])?;
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_20");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    invoke(
        "ReleaseSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from(((values[4]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_14000cc70",
            &[
                relocate("STACK_local_20"),
                values[2],
                relocate("STACK_local_28"),
            ],
            memory,
        );
        values[5] = memory.read_u64(relocate("STACK_local_20"))?;
    }
    if u64::from(((values[5]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
    }
    values[2] = memory.read_u64(values[0])?;
    values[3] = (invoke("FUN_140003ae0", &[values[2]], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(u64::from(
        (u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0,
    )))
}

/// Translation of `FUN_14000c7e0` (`mspaint.c:15946`).
pub fn fun_14000c7e0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[1]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[1]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[1]).wrapping_add(0x48u64)],
            memory,
        );
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140009d30", &[values[1]], memory);
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_20"), relocate("STACK_local_38")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_20"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_14000cc70` (`mspaint.c:16204`).
pub fn fun_14000cc70(
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
    values[6] = (values[1]).wrapping_add(0x10u64);
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xfc7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[6],
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

/// Translation of `FUN_14000ce50` (`mspaint.c:16314`).
pub fn fun_14000ce50(
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
    invoke("FUN_14000ca50", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000d150` (`mspaint.c:16507`).
pub fn fun_14000d150(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x10u64))?,
            values[1],
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14000d170` (`mspaint.c:16524`).
pub fn fun_14000d170(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?,
            values[1],
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14000d2d0` (`mspaint.c:16647`).
pub fn fun_14000d2d0(
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
    values[1] = (values[0]).wrapping_add(0x10u64);
    values[2] = invoke("FUN_140003ae0", &[values[1]], memory);
    if u64::from(
        ((((values[2]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[0]).wrapping_add(0x18u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?,
                values[1],
                0x1u64,
            ],
            memory,
        );
        values[2] &= 0xffffffffu64;
    }
    Ok(Some(values[2]))
}

/// Translation of `FUN_14000d3f0` (`mspaint.c:16750`).
pub fn fun_14000d3f0(
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
        values[0] = relocate("DAT_00000010");
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

/// Translation of `FUN_14000e8b0` (`mspaint.c:17571`).
pub fn fun_14000e8b0(
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
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        if u64::from(
            (u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64))) != 0
                && (u64::from(
                    (memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                        != (0x0u64),
                )) != 0,
        ) != 0
        {
            invoke(
                "free",
                &[memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?],
                memory,
            );
        }
        {
            let destination = (values[1]).wrapping_add(0x34u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x3u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x4u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x5u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[1]).wrapping_add((0x6u64).wrapping_mul(8));
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke("FUN_1400b022c", &[values[1], 0x2344u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14000f7d0` (`mspaint.c:18372`).
pub fn fun_14000f7d0(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x18u64))?) != (0x0u64)) != 0 {
        values[1] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x18u64))?)?)
                    .wrapping_add(0x30u64),
            )?],
            memory,
        );
        return Ok(Some(values[1]));
    }
    Ok(Some(0x8007000eu64))
}

/// Translation of `FUN_14000f800` (`mspaint.c:18388`).
pub fn fun_14000f800(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x18u64))?) != (0x0u64)) != 0 {
        values[1] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x18u64))?)?)
                    .wrapping_add(0x38u64),
            )?],
            memory,
        );
        return Ok(Some(values[1]));
    }
    Ok(Some(0x8007000eu64))
}

/// Translation of `FUN_14000f820` (`mspaint.c:18404`).
pub fn fun_14000f820(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x18u64))?) != (0x0u64)) != 0 {
        values[1] = invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x18u64))?)?)
                    .wrapping_add(0x40u64),
            )?],
            memory,
        );
        return Ok(Some(values[1]));
    }
    Ok(Some(0x8007000eu64))
}

/// Translation of `FUN_140011310` (`mspaint.c:19690`).
pub fn fun_140011310(
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
        "INDIRECT_CALL",
        &[memory.read_u64((values[0]).wrapping_add(0x10u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140011c70` (`mspaint.c:20195`).
pub fn fun_140011c70(
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
    values[2]=u64::from((invoke("FUN_140011d80",&[values[0],values[1],relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.Windows.ImplementationLibrary.1.0.250325.1\\\\include\\\\wil\\\\resource.h\"")],memory)) as u32);
    if u64::from(
        (((0x0u64) as i32 as i64 as u64) as i64)
            < ((((values[2]) as i32 as i64 as u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = u64::from(
            (u64::from(((u64::from(((values[2]) & (0xffffu64)) as u32)) | (0x80070000u64)) as u32))
                as u32,
        );
    }
    invoke(
        "FUN_1400131e0",
        &[relocate("STACK_local_18"), values[2]],
        memory,
    );
    invoke("FUN_140012f00",&[values[0],(values[1]) & (0xffffffffu64),relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.Windows.ImplementationLibrary.1.0.250325.1\\\\include\\\\wil\\\\resource.h\"")],memory);
    Err(AdapterError::Halted("FUN_140012f00"))
}

/// Translation of `FUN_140011cf0` (`mspaint.c:20216`).
pub fn fun_140011cf0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_140011c70", &[], memory);
    Err(AdapterError::Halted("FUN_140011c70"))
}

/// Translation of `FUN_1400123f0` (`mspaint.c:20736`).
pub fn fun_1400123f0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 7 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 7,
            available: arguments.len(),
        }));
    }
    let mut values = [
        arguments[0],
        arguments[1],
        arguments[2],
        arguments[3],
        arguments[4],
        arguments[5],
        u64::from((arguments[6]) as u32),
        0u64,
        0u64,
    ];
    values[1] &= 0xffffffffu64;
    {
        let destination = relocate("STACK_local_18");
        let stored = u64::from((values[6]) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = invoke("FUN_140012f40", &[], memory);

    invoke(
        "FUN_1400123e0",
        &[
            values[0],
            (values[1]) & (0xffffffffu64),
            values[2],
            values[3],
            values[4],
            values[5],
            relocate("STACK_local_18"),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140012b50` (`mspaint.c:21069`).
pub fn fun_140012b50(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if u64::from(((memory.read_u64(relocate("DAT_1401cfc10"))?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(relocate("DAT_1401cfc10"))?],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140012f00` (`mspaint.c:21292`).
pub fn fun_140012f00(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_140012c80", &[], memory);
    Err(AdapterError::Halted("FUN_140012c80"))
}

/// Translation of `FUN_1400148e0` (`mspaint.c:22593`).
pub fn fun_1400148e0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "std::_Xlength_error",
        &[relocate("\"vector too long\"")],
        memory,
    );
    Err(AdapterError::Halted("std::_Xlength_error"))
}

/// Translation of `FUN_1400180d0` (`mspaint.c:25427`).
pub fn fun_1400180d0(
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
    values[1] = memory.read_u64((values[0]).wrapping_add(0x10u64))?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke("BasePaint::Image::~Image", &[values[1]], memory);
        invoke("FUN_1400b022c", &[values[1], 0x58u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140018e80` (`mspaint.c:26288`).
pub fn fun_140018e80(
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
    values[3] = invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x18u64))?,
            values[0],
            relocate("STACK_local_18"),
            values[1],
        ],
        memory,
    );
    if u64::from(
        (u64::from(
            ((memory.read_u64(
                (memory.read_u64((values[3]).wrapping_add((0x2u64).wrapping_mul(4)))?)
                    .wrapping_add(0x8u64),
            )?) as i64)
                == ((memory.read_u64(
                    (memory.read_u64((values[2]).wrapping_add((0x2u64).wrapping_mul(4)))?)
                        .wrapping_add(0x8u64),
                )?) as i64),
        )) != 0
            && (u64::from(
                ((((u64::from(memory.read_u32(values[3])?)) as i32 as i64 as u64) as i32 as i64
                    as u64) as i64)
                    == ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32
                        as i64 as u64) as i64),
            )) != 0,
    ) != 0
    {
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140018f30` (`mspaint.c:26337`).
pub fn fun_140018f30(
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
        u64::from((arguments[1]) as u32),
        arguments[2],
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    {
        let destination = values[0];
        let stored = 0x0u64;
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
        let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = 0xfu64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[0];
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    values[5] = memory.read_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?;
    if u64::from((values[5]) != (0x0u64)) != 0 {
        if u64::from((0x7fffffffu64) < (values[5])) != 0 {
            invoke("FUN_140018a60", &[], memory);
            return Err(AdapterError::Halted("FUN_140018a60"));
        }
        values[5] = invoke(
            "FUN_1400b1660",
            &[
                values[1],
                memory.read_u64(values[2])?,
                values[5],
                0x0u64,
                0x0u64,
            ],
            memory,
        );
        if u64::from(
            (((((values[5]).wrapping_shr(0x20u64 as u32)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64)
                != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140018ef0",
                &[(values[5]).wrapping_shr(0x20u64 as u32)],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140018ef0"));
        }
        values[3] = memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?;
        values[7] = (values[5]) as i32 as i64 as u64;
        if u64::from((values[3]) < (values[7])) != 0 {
            values[8] = (values[7]).wrapping_sub(values[3]);
            values[4] = memory.read_u64((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?;
            if u64::from(((values[4]).wrapping_sub(values[3])) < (values[8])) != 0 {
                invoke(
                    "FUN_140018aa0",
                    &[values[0], values[8], values[4], values[8]],
                    memory,
                );
            } else {
                {
                    let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                    let stored = values[7];
                    memory.write_u64(destination, stored)?;
                }
                values[6] = values[0];
                if u64::from((0xfu64) < (values[4])) != 0 {
                    values[6] = memory.read_u64(values[0])?;
                }
                invoke(
                    "memset",
                    &[(values[6]).wrapping_add(values[3]), 0x0u64, values[8]],
                    memory,
                );
                {
                    let destination = ((values[6]).wrapping_add(values[3])).wrapping_add(values[8]);
                    let stored = u64::from((0x0u64) as u8);
                    memory.write_u8(destination, stored as u8)?;
                }
            }
        } else {
            {
                let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
                let stored = values[7];
                memory.write_u64(destination, stored)?;
            }
            values[6] = values[0];
            if u64::from(
                (0xfu64) < (memory.read_u64((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?),
            ) != 0
            {
                values[6] = memory.read_u64(values[0])?;
            }
            {
                let destination = (values[7]).wrapping_add(values[6]);
                let stored = u64::from((0x0u64) as u8);
                memory.write_u8(destination, stored as u8)?;
            }
        }
        values[6] = values[0];
        if u64::from(
            (0xfu64) < (memory.read_u64((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?),
        ) != 0
        {
            values[6] = memory.read_u64(values[0])?;
        }
        values[5] = invoke(
            "FUN_1400b1660",
            &[
                values[1],
                memory.read_u64(values[2])?,
                u64::from(
                    (u64::from(
                        memory.read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                    )) as u32,
                ),
                values[6],
                (values[5]) as i32 as i64 as u64,
            ],
            memory,
        );
        if u64::from(
            (((((values[5]).wrapping_shr(0x20u64 as u32)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64)
                != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140018ef0",
                &[(values[5]).wrapping_shr(0x20u64 as u32)],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140018ef0"));
        }
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400199f0` (`mspaint.c:26840`).
pub fn fun_1400199f0(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64, 0u64, 0u64, 0u64];
    {
        let destination = values[0];
        let stored = 0x0u64;
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
        let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[2] = memory.read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(8)))?;
    if u64::from((0x7u64) < (memory.read_u64((values[1]).wrapping_add((0x3u64).wrapping_mul(8)))?))
        != 0
    {
        values[1] = memory.read_u64(values[1])?;
    }
    if u64::from((0x7ffffffffffffffeu64) < (values[2])) != 0 {
        invoke("FUN_1400056f0", &[], memory);
        return Err(AdapterError::Halted("FUN_1400056f0"));
    }
    if u64::from((values[2]) < (0x8u64)) != 0 {
        {
            let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = values[2];
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
            let stored = 0x7u64;
            memory.write_u64(destination, stored)?;
        }
        values[3] = memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?;
        {
            let destination = values[0];
            let stored = memory.read_u64(values[1])?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    values[4] = (values[2]) | (0x7u64);
    if u64::from((values[4]) < (0x7fffffffffffffffu64)) != 0 {
        if u64::from((values[4]) < (0xau64)) != 0 {
            values[4] = 0xau64;
        }
        values[6] = (values[4]).wrapping_add(0x1u64);
        if u64::from((0x7fffffffffffffffu64) < (values[6])) != 0 {
            invoke("FUN_140005800", &[], memory);
            return Err(AdapterError::Halted("FUN_140005800"));
        }
    } else {
        values[6] = 0x7fffffffffffffffu64;
        values[4] = 0x7ffffffffffffffeu64;
    }
    values[5] = invoke("FUN_140005710", &[(values[6]).wrapping_mul(0x2u64)], memory);
    {
        let destination = values[0];
        let stored = values[5];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = values[4];
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "memcpy",
        &[
            values[5],
            values[1],
            ((values[2]).wrapping_mul(0x2u64)).wrapping_add(0x2u64),
        ],
        memory,
    );
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001a0b0` (`mspaint.c:27180`).
pub fn fun_14001a0b0(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    {
        let destination = values[0];
        let stored = relocate("CFileException::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "ATL::CStringT<>::~CStringT<>",
        &[(values[0]).wrapping_add((0x3u64).wrapping_mul(8))],
        memory,
    );
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x20u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001ba30` (`mspaint.c:28311`).
pub fn fun_14001ba30(
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
            relocate("CAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = (values[2]).wrapping_add(0x10u64);
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
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
            relocate("CRefAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001bc30` (`mspaint.c:28407`).
pub fn fun_14001bc30(
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
        values[1] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        {
            let destination = values[2];
            let stored = relocate("CAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x7u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14001c740` (`mspaint.c:29236`).
pub fn fun_14001c740(
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
    values[2] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000040");
    }
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

/// Translation of `FUN_14001d460` (`mspaint.c:30182`).
pub fn fun_14001d460(
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
    values[2] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000020");
    }
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

/// Translation of `FUN_14001d750` (`mspaint.c:30365`).
pub fn fun_14001d750(
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
    if u64::from(
        (u64::from(((memory.read_u64(values[1])?) as i64) == ((0x5eb4e5fa3c3f9be5u64) as i64)))
            != 0
            && (u64::from(
                ((memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i64)
                    == ((0u64.wrapping_sub(0x7de4ed7644c3a743u64)) as i64),
            )) != 0,
    ) != 0
    {
        values[2] = (values[0]).wrapping_add(0x10u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[2] = 0x0u64;
        }
        return Ok(Some(values[2]));
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[0];
        memory.write_u64(destination, stored)?;
    }
    let _ = values[1];
    values[2] = invoke("FUN_14001d7a0", &[relocate("STACK_local_18")], memory);
    Ok(Some(values[2]))
}

/// Translation of `FUN_14001dad0` (`mspaint.c:30552`).
pub fn fun_14001dad0(
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
    invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            relocate("DAT_140179110"),
        ],
        memory,
    );
    Ok(Some(0x1u64))
}

/// Translation of `FUN_14001dda0` (`mspaint.c:30726`).
pub fn fun_14001dda0(
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
    values[6] = (values[0]).wrapping_add(0x8u64);
    values[3] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[6] = relocate("DAT_00000018");
        values[3] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[3])?).wrapping_add(0x50u64))?)) as i32
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
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
        values[5] = (memory.read_u64(values[3])?).wrapping_add(0x8u64);
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

/// Translation of `FUN_14001de60` (`mspaint.c:30777`).
pub fn fun_14001de60(
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
    values[4] = (values[0]).wrapping_add(0x8u64);
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000018");
        values[2] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[2])?).wrapping_add(0x50u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = (values[0]).wrapping_add(0x20u64);
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
        values[3] = relocate("DAT_00000028");
    }
    {
        let destination = values[1];
        let stored = u64::from(
            (u64::from(
                ((memory.read_u64(values[3])?) as i64) != ((memory.read_u64(values[2])?) as i64),
            )) != 0,
        );
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001df10` (`mspaint.c:30818`).
pub fn fun_14001df10(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add(0x8u64);
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000018");
        values[2] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[2])?).wrapping_add(0x50u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000028");
    }
    values[3] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000030");
    }
    if u64::from((memory.read_u64(values[2])?) == (memory.read_u64(values[3])?)) != 0 {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0xcccu64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = memory.read_u64(memory.read_u64(values[2])?)?;
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

/// Translation of `FUN_14001e1c0` (`mspaint.c:30988`).
pub fn fun_14001e1c0(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add(0x8u64);
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000018");
        values[2] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[2])?).wrapping_add(0x50u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000028");
    }
    values[3] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000030");
    }
    if u64::from((memory.read_u64(values[2])?) == (memory.read_u64(values[3])?)) != 0 {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x612u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = memory.read_u64(memory.read_u64(values[2])?)?;
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

/// Translation of `FUN_14001f590` (`mspaint.c:32054`).
pub fn fun_14001f590(
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
    values[6] = (values[1]).wrapping_add(0x10u64);
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x4c7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[6],
            memory.read_u64(values[2])?,
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

/// Translation of `FUN_1400222b0` (`mspaint.c:33839`).
pub fn fun_1400222b0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "std::_Xlength_error",
        &[relocate("\"map/set too long\"")],
        memory,
    );
    Err(AdapterError::Halted("std::_Xlength_error"))
}

/// Translation of `FUN_140022c80` (`mspaint.c:34355`).
pub fn fun_140022c80(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = relocate("DAT_00000028");
    }
    invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x58u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140024700` (`mspaint.c:35755`).
pub fn fun_140024700(
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
    if u64::from((memory.read_u64(values[0])?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x10u64))?],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_140024c90` (`mspaint.c:36136`).
pub fn fun_140024c90(
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
    values[1] = (values[0]).wrapping_add(0x8u64);
    values[2] = invoke("FUN_140003ae0", &[values[1]], memory);
    if u64::from(
        ((((values[2]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[0]).wrapping_add(0x10u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?,
                values[1],
                0x1u64,
            ],
            memory,
        );
        values[2] &= 0xffffffffu64;
    }
    Ok(Some(values[2]))
}

/// Translation of `FUN_140024e80` (`mspaint.c:36280`).
pub fn fun_140024e80(
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
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x6872u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[1] = (invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x18u64))?)?)
                .wrapping_add(0xa8u64),
        )?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140002540",
            &[values[1], relocate("STACK_local_28")],
            memory,
        );
        return Err(AdapterError::Halted("FUN_140002540"));
    }
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x6882u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[1] = (invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x18u64))?)?)
                .wrapping_add(0xb0u64),
        )?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
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
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[1], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140026e70` (`mspaint.c:37774`).
pub fn fun_140026e70(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "_invoke_watson",
        &[0x0u64, 0x0u64, 0x0u64, 0x0u64, 0x0u64],
        memory,
    );
    Err(AdapterError::Halted("_invoke_watson"))
}

/// Translation of `FUN_14002c610` (`mspaint.c:42192`).
pub fn fun_14002c610(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "FUN_140032060",
        &[relocate("\"Width is not an integer.\"")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140032060"))
}

/// Translation of `FUN_14002c750` (`mspaint.c:42267`).
pub fn fun_14002c750(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "FUN_140032060",
        &[relocate("\"Precision is not an integer.\"")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140032060"))
}

/// Translation of `FUN_140031bd0` (`mspaint.c:46348`).
pub fn fun_140031bd0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_140031b60", &[relocate("STACK_local_28")], memory);
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_28"), relocate("DAT_14016ad30")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140032060` (`mspaint.c:46572`).
pub fn fun_140032060(
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
    invoke(
        "FUN_140031fb0",
        &[relocate("STACK_local_28"), values[0]],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_28"), relocate("DAT_14016ad98")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140032490` (`mspaint.c:46785`).
pub fn fun_140032490(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_1400321e0", &[], memory);
    Err(AdapterError::Halted("FUN_1400321e0"))
}

/// Translation of `FUN_140032520` (`mspaint.c:46812`).
pub fn fun_140032520(
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
    invoke("FUN_1400324c0",&[values[0],0x138eu64,relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.Windows.ImplementationLibrary.1.0.250325.1\\\\include\\\\wil\\\\resource.h\"")],memory);
    Err(AdapterError::Halted("FUN_1400324c0"))
}

/// Translation of `FUN_140033c90` (`mspaint.c:48161`).
pub fn fun_140033c90(
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
    values[1] = (values[0]).wrapping_add(0x68u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = relocate("DAT_00000078");
    }
    if u64::from((memory.read_u64(values[1])?) == (0x0u64)) != 0 {
        invoke("std::_Xbad_function_call", &[], memory);
        return Err(AdapterError::Halted("std::_Xbad_function_call"));
    }
    invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x10u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140036cf0` (`mspaint.c:50848`).
pub fn fun_140036cf0(
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
        "FUN_140034d90",
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

/// Translation of `FUN_1400385a0` (`mspaint.c:52392`).
pub fn fun_1400385a0(
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
        "INDIRECT_CALL",
        &[memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400385d0` (`mspaint.c:52411`).
pub fn fun_1400385d0(
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
    values[1] =
        (invoke("_Mtx_lock", &[(values[0]).wrapping_add(0x20u64)], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("std::_Throw_Cpp_error", &[0x5u64], memory);
        return Err(AdapterError::Halted("std::_Throw_Cpp_error"));
    }
    if u64::from(
        ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x6cu64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x7fffffffu64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[0]).wrapping_add(0x6cu64);
            let stored = u64::from((0x7ffffffeu64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        invoke("std::_Throw_Cpp_error", &[0x6u64], memory);
        return Err(AdapterError::Halted("std::_Throw_Cpp_error"));
    }
    values[2] = u64::from(
        (u64::from(
            ((((u64::from(memory.read_u32((values[0]).wrapping_add(0x8u64))?)) as i32 as i64 as u64)
                as i32 as i64 as u64) as i64)
                != (((0x2u64) as i32 as i64 as u64) as i64),
        )) != 0,
    );
    if values[2] != 0 {
        {
            let destination = (values[0]).wrapping_add(0x8u64);
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
    }
    invoke("_Mtx_unlock", &[(values[0]).wrapping_add(0x20u64)], memory);
    Ok(Some(u64::from((values[2]) != 0)))
}

/// Translation of `FUN_140038890` (`mspaint.c:52543`).
pub fn fun_140038890(
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
    let mut values = [arguments[0], (arguments[1]) as i8 as i64 as u64, 0u64];
    values[2] = memory.read_u64((values[0]).wrapping_add(0x40u64))?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x20u64))?,
                values[2],
                u64::from((values[2]) != ((values[0]).wrapping_add(0x8u64))),
            ],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add(0x40u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_1400b022c", &[values[0], 0x48u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140038ea0` (`mspaint.c:52869`).
pub fn fun_140038ea0(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?,
            (values[0]).wrapping_add(0x10u64),
            0x0u64,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140039d40` (`mspaint.c:53787`).
pub fn fun_140039d40(
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
    values[0] = memory.read_u64(values[0])?;
    if u64::from((values[0]) != (0x0u64)) != 0 {
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
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14003aee0` (`mspaint.c:54854`).
pub fn fun_14003aee0(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64, 0u64, 0u64];
    {
        let destination = values[0];
        let stored = memory.read_u64(values[1])?;
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
        let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = ((memory.read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(8)))?)
        .wrapping_sub(memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?)
        as i64)
        .wrapping_shr(0x3u64 as u32) as u64;
    if u64::from((values[4]) != (0x0u64)) != 0 {
        if u64::from((0x1fffffffffffffffu64) < (values[4])) != 0 {
            invoke("FUN_1400148e0", &[], memory);
            return Err(AdapterError::Halted("FUN_1400148e0"));
        }
        values[3] = invoke("FUN_140005710", &[(values[4]).wrapping_mul(0x8u64)], memory);
        {
            let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
            let stored = (values[3]).wrapping_add((values[4]).wrapping_mul(0x8u64));
            memory.write_u64(destination, stored)?;
        }
        values[2] = memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?;
        values[5] = (memory.read_u64((values[1]).wrapping_add((0x2u64).wrapping_mul(8)))?)
            .wrapping_sub(memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?);
        invoke(
            "memmove",
            &[
                values[2],
                memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?,
                values[5],
            ],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = (values[2]).wrapping_add(
                ((values[5] as i64).wrapping_shr(0x3u64 as u32) as u64).wrapping_mul(0x8u64),
            );
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003b170` (`mspaint.c:55002`).
pub fn fun_14003b170(
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
    values[2] = (invoke("FUN_140002190", &[], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_14003b160", &[], memory);
    }
    values[1] = invoke(
        "FUN_14000a950",
        &[relocate("STACK_local_18"), (values[0]).wrapping_add(0x8u64)],
        memory,
    );
    invoke("FUN_14000a940", &[values[1]], memory);
    Err(AdapterError::Halted("FUN_14000a940"))
}

/// Translation of `FUN_14003b2d0` (`mspaint.c:55082`).
pub fn fun_14003b2d0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_14003b220", &[relocate("STACK_local_28")], memory);
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_28"), relocate("DAT_14016ae90")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140041620` (`mspaint.c:60780`).
pub fn fun_140041620(
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
        "FUN_14003d360",
        &[values[4], relocate("STACK_local_res8"), 0x0u64, 0x1u64],
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

/// Translation of `FUN_1400419f0` (`mspaint.c:61023`).
pub fn fun_1400419f0(
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
    values[3] = (values[0]).wrapping_add(0x38u64);
    values[2] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000048");
        values[2] = 0x38u64;
    }
    invoke(
        "FUN_1400072f0",
        &[
            values[2],
            relocate("STACK_local_res8"),
            u64::from((u64::from(memory.read_u32(values[3])?)) as u32),
        ],
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

/// Translation of `FUN_140041a50` (`mspaint.c:61046`).
pub fn fun_140041a50(
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
    values[2] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000040");
    }
    values[2] = memory.read_u64(values[2])?;
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

/// Translation of `FUN_140041aa0` (`mspaint.c:61068`).
pub fn fun_140041aa0(
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
    values[2] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000038");
    }
    values[2] = memory.read_u64(values[2])?;
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

/// Translation of `FUN_140042990` (`mspaint.c:61938`).
pub fn fun_140042990(
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
    if u64::from(
        (u64::from(((memory.read_u64(values[1])?) as i64) == ((0x5525d83d1759a6aau64) as i64)))
            != 0
            && (u64::from(
                ((memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i64)
                    == ((0u64.wrapping_sub(0x288e6b8b63c15f5au64)) as i64),
            )) != 0,
    ) != 0
    {
        values[2] = (values[0]).wrapping_add(0x10u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[2] = 0x0u64;
        }
        return Ok(Some(values[2]));
    }
    {
        let destination = relocate("STACK_local_18");
        let stored = values[0];
        memory.write_u64(destination, stored)?;
    }
    let _ = values[1];
    values[2] = invoke("FUN_1400429e0", &[relocate("STACK_local_18")], memory);
    Ok(Some(values[2]))
}

/// Translation of `FUN_140045d40` (`mspaint.c:64810`).
pub fn fun_140045d40(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    {
        let destination = values[0];
        let stored = relocate("CPrintingDialog::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke("CDialog::~CDialog", &[values[0]], memory);
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x130u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140047b60` (`mspaint.c:66288`).
pub fn fun_140047b60(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    {
        let destination = values[0];
        let stored = relocate("CCommonDialog::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke("CDialog::~CDialog", &[values[0]], memory);
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x240u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140048000` (`mspaint.c:66527`).
pub fn fun_140048000(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    {
        let destination = values[0];
        let stored = relocate("CCommonDialog::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke("CDialog::~CDialog", &[values[0]], memory);
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x130u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14004ac40` (`mspaint.c:68782`).
pub fn fun_14004ac40(
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
    values[2] = u64::from(
        (invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x68u64),
                )?,
                memory.read_u64(values[3])?,
                0xcu64,
            ],
            memory,
        )) as u8,
    );
    {
        let destination = values[1];
        let stored = u64::from((values[2]) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004ad40` (`mspaint.c:68857`).
pub fn fun_14004ad40(
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
    values[2] = u64::from(
        (invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x100u64),
            )?],
            memory,
        )) as u16,
    );
    {
        let destination = values[1];
        let stored = u64::from((values[2]) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004ada0` (`mspaint.c:68891`).
pub fn fun_14004ada0(
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
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000028");
    }
    values[2] = memory.read_u64(values[2])?;
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

/// Translation of `FUN_14004b8d0` (`mspaint.c:69464`).
pub fn fun_14004b8d0(
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
    values[2] = memory.read_u64((values[0]).wrapping_add(0x50u64))?;
    {
        let destination = values[1];
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

/// Translation of `FUN_14004ce20` (`mspaint.c:70593`).
pub fn fun_14004ce20(
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
        let destination = values[0];
        let stored = relocate("CNarratorBridge::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke("FUN_14004cd30", &[], memory);
    invoke("CWnd::~CWnd", &[values[0]], memory);
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        if u64::from(((values[1]) & (0x4u64)) == (0x0u64)) != 0 {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0xf8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14004d150` (`mspaint.c:70777`).
pub fn fun_14004d150(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = invoke("operator_new", &[0x218u64], memory);
    if u64::from((values[0]) != (0x0u64)) != 0 {
        invoke("memset", &[values[0], 0x0u64, 0x218u64], memory);
        invoke("CPreviewView::CPreviewView", &[values[0]], memory);
        {
            let destination = values[0];
            let stored = relocate("CPaintPreviewView::vftable");
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(values[0]));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004d1b0` (`mspaint.c:70794`).
pub fn fun_14004d1b0(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    invoke("CPreviewView::~CPreviewView", &[values[0]], memory);
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x218u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14004d370` (`mspaint.c:70914`).
pub fn fun_14004d370(
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
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?,
            values[1],
            0x1u64,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140051b70` (`mspaint.c:74315`).
pub fn fun_140051b70(
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
    values[2] = invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x48u64))?)?)
                .wrapping_add(0x28u64),
        )?],
        memory,
    );
    if u64::from(
        (u64::from(
            (u64::from(
                (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64),
                )) != 0
                    && (u64::from(
                        ((((u64::from(memory.read_u32(values[1])?)) as i32 as i64 as u64) as i32
                            as i64 as u64) as i64)
                            < ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64)
                                as i32 as i64 as u64) as i64),
                    )) != 0,
            )) != 0
                && (u64::from(
                    ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                        as i64)
                        < ((((u64::from(
                            memory.read_u32((values[1]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                        )) as i32 as i64 as u64) as i32 as i64 as u64)
                            as i64),
                )) != 0,
        )) != 0
            && (u64::from(
                ((((u64::from(memory.read_u32((values[1]).wrapping_add((0x1u64).wrapping_mul(4)))?))
                    as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                    < ((((u64::from(
                        memory.read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64) as i32 as i64 as u64) as i64),
            )) != 0,
    ) != 0
    {
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140051f00` (`mspaint.c:74572`).
pub fn fun_140051f00(
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
        invoke(
            "FUN_1400a3d60",
            &[(values[1]).wrapping_add(0x1e0u64)],
            memory,
        );
        return Ok(None);
    }
    invoke("FUN_1400a3d60", &[0x1e0u64], memory);
    Ok(None)
}

/// Translation of `FUN_140052fa0` (`mspaint.c:75441`).
pub fn fun_140052fa0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64];
    values[1] = invoke("FUN_14005c3d0", &[relocate("STACK_local_res10")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x2c50u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[0] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[1])?,
            0x0u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[0]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[0], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_1400534b0` (`mspaint.c:75709`).
pub fn fun_1400534b0(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        invoke("FUN_14001aa10", &[0x0u64, 0x1u64, values[1]], memory);
        return Ok(None);
    }
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) == (u64::from((0x1u64) as u32)),
    ) != 0
    {
        invoke("FUN_14001aa10", &[0x1u64, 0x0u64, values[1]], memory);
        return Ok(None);
    }
    invoke("FUN_14000a4a0", &[], memory);
    Err(AdapterError::Halted("FUN_14000a4a0"))
}

/// Translation of `FUN_140053970` (`mspaint.c:75939`).
pub fn fun_140053970(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64];
    values[1] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res10")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x70fcu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[0] = (invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x180u64))?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[0]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[0], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140053bf0` (`mspaint.c:76081`).
pub fn fun_140053bf0(
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
    values[4] = invoke("FUN_14004e8c0", &[relocate("STACK_local_28")], memory);
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x6e2du64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[4])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[4])?,
            memory.read_u64(values[2])?,
            values[1],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(((memory.read_u64(relocate("STACK_local_28"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_28")], memory);
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_20")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140053c80` (`mspaint.c:76111`).
pub fn fun_140053c80(
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
    values[2] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res10")], memory);
    invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x38u64))?,
            memory.read_u64(values[2])?,
            memory.read_u64(values[1])?,
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
    Ok(None)
}

/// Translation of `FUN_140053cc0` (`mspaint.c:76129`).
pub fn fun_140053cc0(
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
        0u64,
    ];
    values[3] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res18")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x7016u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x110u64))?,
            memory.read_u64(values[3])?,
            values[1],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res18")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res18")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140053d40` (`mspaint.c:76158`).
pub fn fun_140053d40(
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
        0u64,
    ];
    values[3] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res18")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x7038u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x120u64))?,
            memory.read_u64(values[3])?,
            values[1],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res18")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res18")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140053dc0` (`mspaint.c:76187`).
pub fn fun_140053dc0(
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
        0u64,
    ];
    values[3] = invoke("FUN_14004e8c0", &[relocate("STACK_local_38")], memory);
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x709cu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x150u64))?,
            memory.read_u64(values[3])?,
            values[1],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(((memory.read_u64(relocate("STACK_local_38"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_38")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_30")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140053e40` (`mspaint.c:76216`).
pub fn fun_140053e40(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64];
    values[1] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res10")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x70acu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[0] = (invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x158u64))?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[0]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[0], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140053eb0` (`mspaint.c:76245`).
pub fn fun_140053eb0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64];
    values[1] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res10")], memory);
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x70bcu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[0] = (invoke(
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x160u64))?],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[0]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            ((memory
                .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
                as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[0], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140054010` (`mspaint.c:76318`).
pub fn fun_140054010(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    values[0] = invoke("FUN_14004e8c0", &[relocate("STACK_local_res10")], memory);
    values[1] = invoke("FUN_140053b70", &[values[0]], memory);
    if u64::from(
        ((memory
            .read_u64((relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8)))?)
            as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        values[1] &= 0xffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400540f0` (`mspaint.c:76373`).
pub fn fun_1400540f0(
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
        u64::from((arguments[1]) as u8),
        0u64,
        0u64,
        0u64,
        0u64,
    ];
    values[3] = invoke("FUN_14004e8c0", &[relocate("STACK_local_28")], memory);
    {
        let destination = (relocate("STACK_local_20")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x705au64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[3])?)?).wrapping_add(0x130u64))?,
            memory.read_u64(values[3])?,
            values[1],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(((memory.read_u64(relocate("STACK_local_28"))?) as i64) != ((0x0u64) as i64))
            != 0
        {
            invoke("FUN_140001b30", &[relocate("STACK_local_28")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[2], relocate("STACK_local_20")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140055060` (`mspaint.c:77132`).
pub fn fun_140055060(
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
    values[2] = (values[0]).wrapping_add(0xe8u64);
    values[3] = (invoke("FUN_140001a20", &[values[2]], memory)) as i8 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from((values[2]) != (values[1])) != 0 {
            if u64::from(((memory.read_u64(values[2])?) as i64) != ((0x0u64) as i64)) != 0 {
                invoke("FUN_140001b30", &[values[2]], memory);
            }
            values[1] = memory.read_u64(values[1])?;
            {
                let destination = values[2];
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
        }
        invoke(
            "FUN_1400572d0",
            &[
                (values[0]).wrapping_add(0x28u64),
                relocate("PTR_s_AiInformation_1401121a0"),
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400561d0` (`mspaint.c:78482`).
pub fn fun_1400561d0(
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
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[3]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[3]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[3]).wrapping_add(0x68u64))?)) as u32))
                as u32,
        );
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[3]).wrapping_add(0x48u64)],
            memory,
        );
        {
            let destination = values[1];
            let stored = u64::from((values[2]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140009d30", &[values[3]], memory);
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_20"), relocate("STACK_local_38")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_20"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140056410` (`mspaint.c:78607`).
pub fn fun_140056410(
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
    values[2] = memory.read_u64(values[0])?;
    values[4] = 0x0u64;
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = ((u64::from(memory.read_u32((values[2]).wrapping_add(0x60u64))?)) as i32 as i64
            as u64) as i32 as i64 as u64;
        memory.write_u32(destination, stored as u32)?;
    }
    if u64::from(
        ((((u64::from(
            memory.read_u32((relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)))?,
        )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = (0x1u64) as i32 as i64 as u64;
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add(0x60u64);
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
    }
    values[1] = (values[2]).wrapping_add(0x50u64);
    if u64::from((relocate("STACK_local_20")) != (values[1])) != 0 {
        values[4] = memory.read_u64(values[1])?;
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_20");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    invoke(
        "ReleaseSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from(((values[4]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_1400592b0",
            &[
                relocate("STACK_local_20"),
                values[2],
                relocate("STACK_local_28"),
            ],
            memory,
        );
        values[5] = memory.read_u64(relocate("STACK_local_20"))?;
    }
    if u64::from(((values[5]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
    }
    values[2] = memory.read_u64(values[0])?;
    values[3] = (invoke("FUN_140058bc0", &[values[2]], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(u64::from(
        (u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0,
    )))
}

/// Translation of `FUN_140056560` (`mspaint.c:78698`).
pub fn fun_140056560(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    values[1] = invoke("FUN_140058bc0", &[values[2]], memory);
    if u64::from(
        ((((values[1]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (values[2]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64),
                )?,
                (values[2]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64),
            ],
            memory,
        );
        values[1] &= 0xffffffffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400567d0` (`mspaint.c:78852`).
pub fn fun_1400567d0(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    values[3] = invoke(
        "FUN_140058c70",
        &[values[2], relocate("STACK_local_res8")],
        memory,
    );
    {
        let destination = values[1];
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[3])?)) as u32)) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140056820` (`mspaint.c:78887`).
pub fn fun_140056820(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    values[1] = invoke("FUN_140058bc0", &[values[2]], memory);
    if u64::from(
        ((((values[1]) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (values[2]).wrapping_add((0u64.wrapping_sub(0x8u64)) as i32 as i64 as u64),
                )?,
                (values[2]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64),
            ],
            memory,
        );
        values[1] &= 0xffffffffu64;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140056910` (`mspaint.c:78963`).
pub fn fun_140056910(
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
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[3]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[3]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = memory.read_u64((values[3]).wrapping_add(0x68u64))?;
        {
            let destination = (values[3]).wrapping_add(0x68u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[3]).wrapping_add(0x48u64)],
            memory,
        );
        {
            let destination = values[1];
            let stored = values[2];
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140058c40", &[values[3]], memory);
    {
        let destination = (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_30"), relocate("STACK_local_48")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_30"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400569d0` (`mspaint.c:79000`).
pub fn fun_1400569d0(
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
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000048");
    }
    invoke("AcquireSRWLockExclusive", &[values[3]], memory);
    values[2] = (values[0]).wrapping_add(0x40u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000050");
    }
    values[2] = memory.read_u64(values[2])?;
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
    invoke("ReleaseSRWLockExclusive", &[values[3]], memory);
    {
        let destination = values[1];
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140056c20` (`mspaint.c:79135`).
pub fn fun_140056c20(
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
    let mut values = [arguments[0], arguments[1], arguments[2], arguments[3], 0u64];
    invoke(
        "FUN_140004ee0",
        &[
            relocate("STACK_local_10"),
            (values[0]).wrapping_add(0x48u64),
            values[2],
            values[3],
            values[0],
        ],
        memory,
    );
    values[4] = invoke("FUN_14000a980", &[relocate("STACK_local_20")], memory);
    invoke(
        "FUN_14000a9b0",
        &[(values[0]).wrapping_add(0x38u64), values[4]],
        memory,
    );
    invoke(
        "__ExceptionPtrDestroy",
        &[relocate("STACK_local_20")],
        memory,
    );
    values[4] = invoke(
        "FUN_14000a950",
        &[
            relocate("STACK_local_20"),
            (values[0]).wrapping_add(0x38u64),
        ],
        memory,
    );
    invoke("FUN_14000a940", &[values[4]], memory);
    Err(AdapterError::Halted("FUN_14000a940"))
}

/// Translation of `FUN_140056e30` (`mspaint.c:79292`).
pub fn fun_140056e30(
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
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[3]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[3]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = u64::from(
            (u64::from((u64::from(memory.read_u8((values[3]).wrapping_add(0x68u64))?)) as u8))
                as u8,
        );
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[3]).wrapping_add(0x48u64)],
            memory,
        );
        {
            let destination = values[1];
            let stored = u64::from((values[2]) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140058c40", &[values[3]], memory);
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_20"), relocate("STACK_local_38")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_20"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140057030` (`mspaint.c:79399`).
pub fn fun_140057030(
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
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "Paint::AI::ImageObjectEncoding::~ImageObjectEncoding",
            &[values[1]],
            memory,
        );
        invoke("FUN_1400b022c", &[values[1], 0x8u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140057880` (`mspaint.c:79875`).
pub fn fun_140057880(
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
        "INDIRECT_CALL",
        &[memory
            .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?],
        memory,
    );
    Ok(Some(0x1u64))
}

/// Translation of `FUN_140057f80` (`mspaint.c:80260`).
pub fn fun_140057f80(
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
    if u64::from(
        ((((u64::from(memory.read_u8((values[0]).wrapping_add((0x4u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(8));
            let stored = u64::from((0x0u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        invoke(
            "FUN_1400399e0",
            &[
                memory.read_u64(values[0])?,
                memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
            ],
            memory,
        );
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x190u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

        values[1] = (invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(
                    memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?,
                )?)
                .wrapping_add(0x30u64),
            )?],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[1]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140002540",
                &[values[1], relocate("STACK_local_28")],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140002540"));
        }
    }
    invoke("FUN_140057460", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_140058190` (`mspaint.c:80426`).
pub fn fun_140058190(
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
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[3] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[3]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[3]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = memory.read_u64((values[3]).wrapping_add(0x68u64))?;
        {
            let destination = (values[3]).wrapping_add(0x68u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[3]).wrapping_add(0x48u64)],
            memory,
        );
        {
            let destination = values[1];
            let stored = values[2];
            memory.write_u64(destination, stored)?;
        }
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140009d30", &[values[3]], memory);
    {
        let destination = (relocate("STACK_local_48")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_30"), relocate("STACK_local_48")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_30"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400592b0` (`mspaint.c:81347`).
pub fn fun_1400592b0(
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
    values[6] = (values[1]).wrapping_add(0x10u64);
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x1039u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[6],
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

/// Translation of `FUN_140059a40` (`mspaint.c:81770`).
pub fn fun_140059a40(
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

/// Translation of `FUN_140059a70` (`mspaint.c:81782`).
pub fn fun_140059a70(
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
    invoke("FUN_140057ed0", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140059aa0` (`mspaint.c:81794`).
pub fn fun_140059aa0(
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
    invoke("FUN_140009a80", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14005a270` (`mspaint.c:82237`).
pub fn fun_14005a270(
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
    invoke("FUN_140057ed0", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14005b480` (`mspaint.c:83263`).
pub fn fun_14005b480(
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
    values[4] = 0x0u64;
    values[3] = invoke("operator_new", &[0x48u64], memory);
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
            let destination = (values[3]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[3]).wrapping_add(0xcu64);
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[3];
            let stored = relocate("std::_Ref_count_obj2<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x3u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[3]).wrapping_add((0x4u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "__ExceptionPtrCopy",
            &[
                (values[3]).wrapping_add((0x3u64).wrapping_mul(8)),
                values[1],
            ],
            memory,
        );
        invoke(
            "FUN_14003aee0",
            &[
                (values[3]).wrapping_add((0x5u64).wrapping_mul(8)),
                values[2],
            ],
            memory,
        );
        values[4] = values[3];
    }
    {
        let destination = values[0];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[0];
        let stored = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = values[4];
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14005d5f0` (`mspaint.c:85062`).
pub fn fun_14005d5f0(
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
    values[1] = memory.read_u64((values[0]).wrapping_add(0x78u64))?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x20u64))?,
                values[1],
                u64::from((values[1]) != ((values[0]).wrapping_add(0x40u64))),
            ],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add(0x78u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    invoke(
        "FUN_140017c60",
        &[(values[0]).wrapping_add(0x20u64)],
        memory,
    );
    invoke("FUN_1400054d0", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_140061210` (`mspaint.c:87788`).
pub fn fun_140061210(
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
    values[1] = (invoke("FUN_140061080", &[], memory)) as i8 as i64 as u64;
    {
        let destination = (values[0]).wrapping_add(0x2c0u64);
        let stored = (values[1]) as i8 as i64 as u64;
        memory.write_u8(destination, stored as u8)?;
    }
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_14006e890", &[], memory);
        {
            let destination = (values[0]).wrapping_add(0x2c3u64);
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8(destination, stored as u8)?;
        }
        invoke(
            "AfxThrowFileException",
            &[
                0x6u64,
                (0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64,
                0x0u64,
            ],
            memory,
        );
        return Err(AdapterError::Halted("AfxThrowFileException"));
    }
    Ok(None)
}

/// Translation of `FUN_140062150` (`mspaint.c:88493`).
pub fn fun_140062150(
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
    invoke(
        "COleServerDoc::OnDeactivateUI",
        &[values[0], values[1]],
        memory,
    );
    values[2] = (invoke(
        "CPaintTool::Id",
        &[memory.read_u64(relocate("DAT_1401d06e0"))?],
        memory,
    )) as i32 as i64 as u64;
    invoke(
        "FUN_1400512b0",
        &[
            relocate("DAT_1401d06a0"),
            relocate("STACK_local_res18"),
            values[2],
        ],
        memory,
    );
    if u64::from(
        ((memory
            .read_u64((relocate("STACK_local_res18")).wrapping_add((0x0u64).wrapping_mul(8)))?)
            as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[relocate("STACK_local_res18")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140063dc0` (`mspaint.c:90085`).
pub fn fun_140063dc0(
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
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x7e4u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.ApplicationModel.DataTransfer.h\"");

        values[1] = (invoke(
            "INDIRECT_CALL",
            &[memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x30u64))?],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[1]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140002540",
                &[values[1], relocate("STACK_local_28")],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140002540"));
        }
    }
    if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[values[0]], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140063e30` (`mspaint.c:90117`).
pub fn fun_140063e30(
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
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x5d5u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

        values[1] = (invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x1d8u64),
                )?,
                memory.read_u64(values[0])?,
                0x2u64,
            ],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[1]) as i32 as i64 as u64) as i64) < (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140002540",
                &[values[1], relocate("STACK_local_28")],
                memory,
            );
            return Err(AdapterError::Halted("FUN_140002540"));
        }
    }
    if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[values[0]], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140065540` (`mspaint.c:91540`).
pub fn fun_140065540(
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
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
        values[5] = (memory.read_u64(values[3])?).wrapping_add(0x8u64);
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

/// Translation of `FUN_140065600` (`mspaint.c:91591`).
pub fn fun_140065600(
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
    values[4] = (values[0]).wrapping_add(0x8u64);
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000018");
        values[2] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[2])?).wrapping_add(0x28u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = (values[0]).wrapping_add(0x20u64);
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
        values[3] = relocate("DAT_00000028");
    }
    {
        let destination = values[1];
        let stored = u64::from(
            (u64::from(
                ((memory.read_u64(values[3])?) as i64) != ((memory.read_u64(values[2])?) as i64),
            )) != 0,
        );
        memory.write_u8(destination, stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400656b0` (`mspaint.c:91632`).
pub fn fun_1400656b0(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[4] = (values[0]).wrapping_add(0x8u64);
    values[2] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = relocate("DAT_00000018");
        values[2] = relocate("DAT_00000020");
    }
    if u64::from(
        ((((u64::from(memory.read_u32((memory.read_u64(values[2])?).wrapping_add(0x28u64))?)) as i32
            as i64 as u64) as i32 as i64 as u64) as i64)
            != ((((u64::from(memory.read_u32(values[4])?)) as i32 as i64 as u64) as i32 as i64
                as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000028");
    }
    values[3] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000030");
    }
    if u64::from((memory.read_u64(values[2])?) == (memory.read_u64(values[3])?)) != 0 {
        {
            let destination = (relocate("STACK_local_58")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x612u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

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
    values[2] = memory.read_u64(memory.read_u64(values[2])?)?;
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

/// Translation of `FUN_140065990` (`mspaint.c:91826`).
pub fn fun_140065990(
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
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x8u64))?)?)
                    .wrapping_add(0x110u64),
            )?,
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            memory.read_u64(values[1])?,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400661d0` (`mspaint.c:92310`).
pub fn fun_1400661d0(
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
    let mut values = [arguments[0], u64::from((arguments[1]) as u32), 0u64];
    {
        let destination = values[0];
        let stored = relocate("CDocument::CDocumentAdapter::vftable");
        memory.write_u64(destination, stored)?;
    }
    values[2] = memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
                values[2],
                0x1u64,
            ],
            memory,
        );
        {
            let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    {
        let destination = values[0];
        let stored = relocate("ATL::IDocument::vftable");
        memory.write_u64(destination, stored)?;
    }
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140066340` (`mspaint.c:92388`).
pub fn fun_140066340(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x178u64),
            )?],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140066360` (`mspaint.c:92402`).
pub fn fun_140066360(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x190u64),
            )?],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140066380` (`mspaint.c:92416`).
pub fn fun_140066380(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) == (0x0u64)) != 0 {
        return Ok(Some(0x0u64));
    }
    values[1] = invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                .wrapping_add(0x170u64),
        )?],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400663b0` (`mspaint.c:92432`).
pub fn fun_1400663b0(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x180u64),
            )?],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_1400663d0` (`mspaint.c:92446`).
pub fn fun_1400663d0(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) == (0x0u64)) != 0 {
        return Ok(Some(0x0u64));
    }
    values[1] = invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                .wrapping_add(0x188u64),
        )?],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400663f0` (`mspaint.c:92462`).
pub fn fun_1400663f0(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) == (0x0u64)) != 0 {
        return Ok(Some(0x0u64));
    }
    values[1] = invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                .wrapping_add(0x198u64),
        )?],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_140066410` (`mspaint.c:92478`).
pub fn fun_140066410(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                    .wrapping_add(0x1a0u64),
            )?],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140066430` (`mspaint.c:92492`).
pub fn fun_140066430(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add(0x10u64))?) == (0x0u64)) != 0 {
        return Ok(Some(0x0u64));
    }
    values[1] = invoke(
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x10u64))?)?)
                .wrapping_add(0x1a8u64),
        )?],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_140066470` (`mspaint.c:92532`).
pub fn fun_140066470(
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
    values[1] = (values[0]).wrapping_add(0x118u64);
    if u64::from(((memory.read_u64(values[1])?) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = invoke("operator_new", &[0x18u64], memory);
        values[3] = 0x0u64;
        if u64::from((values[2]) != (0x0u64)) != 0 {
            {
                let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = values[2];
                let stored = relocate("ATL::IDocument::vftable");
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
                let stored = u64::from((0x0u64) as u32);
                memory.write_u32(destination, stored as u32)?;
            }
            {
                let destination = values[2];
                let stored = relocate("CDocument::CDocumentAdapter::vftable");
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
                let stored = values[0];
                memory.write_u64(destination, stored)?;
            }
            values[3] = values[2];
        }
        {
            let destination = values[1];
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
    }
    if u64::from((memory.read_u64(values[1])?) == (0x0u64)) != 0 {
        return Ok(Some(0x0u64));
    }
    invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x8u64))?,
        ],
        memory,
    );
    Ok(Some(memory.read_u64(values[1])?))
}

/// Translation of `FUN_14006a1d0` (`mspaint.c:95596`).
pub fn fun_14006a1d0(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    invoke(
        "CSingleDocTemplate::~CSingleDocTemplate",
        &[values[0]],
        memory,
    );
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0xf8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14006aaa0` (`mspaint.c:96090`).
pub fn fun_14006aaa0(
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
        if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
            return Ok(Some(memory.read_u64((values[1]).wrapping_add(0x40u64))?));
        }
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14006b110` (`mspaint.c:96362`).
pub fn fun_14006b110(
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
        let destination = values[0];
        let stored = relocate("IPaintListener::vftable");
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "IPaintAppServices::IPaintAppServices",
        &[(values[0]).wrapping_add((0x1u64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "IPaintImageUtilities::IPaintImageUtilities",
        &[(values[0]).wrapping_add((0x2u64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "IDirectXResourceManager::IDirectXResourceManager",
        &[(values[0]).wrapping_add((0x3u64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "ITextServices::ITextServices",
        &[(values[0]).wrapping_add((0x4u64).wrapping_mul(8))],
        memory,
    );
    {
        let destination = (values[0]).wrapping_add((0x5u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x6u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x7u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[0];
        let stored = relocate("CPaintToolManager::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = relocate("CPaintToolManager::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x2u64).wrapping_mul(8));
        let stored = relocate("CPaintToolManager::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = relocate("CPaintToolManager::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x4u64).wrapping_mul(8));
        let stored = relocate("CPaintToolManager::vftable");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x8u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x9u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xau64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xbu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xcu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xdu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = invoke("operator_new", &[0x38u64], memory);
    {
        let destination = values[1];
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x8u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x10u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x18u64);
        let stored = u64::from((0x101u64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xcu64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xeu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xfu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = invoke("operator_new", &[0x28u64], memory);
    {
        let destination = values[1];
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x8u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x10u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x18u64);
        let stored = u64::from((0x101u64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    {
        let destination = (values[0]).wrapping_add((0xeu64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x11u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = invoke("operator_new", &[0x30u64], memory);
    {
        let destination = values[1];
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x8u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x10u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x18u64);
        let stored = u64::from((0x101u64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x10u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x13u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    values[1] = invoke("operator_new", &[0x18u64], memory);
    {
        let destination = values[1];
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x8u64);
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x12u64).wrapping_mul(8));
        let stored = values[1];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x14u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x15u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x16u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x17u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x18u64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x19u64).wrapping_mul(8));
        let stored = u64::from((0x0u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[0]).wrapping_add(0xccu64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1au64).wrapping_mul(8));
        let stored = 0xffffffffffffffffu64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1bu64).wrapping_mul(8));
        let stored = u64::from((0x0u64) as u16);
        memory.write_u16(destination, stored as u16)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1cu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1du64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1eu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[0]).wrapping_add((0x1fu64).wrapping_mul(8));
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14006f5d0` (`mspaint.c:99630`).
pub fn fun_14006f5d0(
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
        (((((u64::from((u64::from(memory.read_u16((values[0]).wrapping_add(0x338u64))?)) as u16))
            & (0x8u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            (((((u64::from(
                (u64::from(memory.read_u16((values[0]).wrapping_add(0x338u64))?)) as u16,
            )) & (0x10u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] = memory.read_u64((values[0]).wrapping_add(0x690u64))?;
            values[2] = invoke("GetTickCount64", &[], memory);
            if u64::from(((values[2]).wrapping_sub(values[3])) < (0xea61u64)) != 0 {
                {
                    let destination = (values[0]).wrapping_add(0x338u64);
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16((values[0]).wrapping_add(0x338u64))?))
                                as u16,
                        )) & (0xfff7u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                return Ok(None);
            }
        }
        if u64::from(
            ((((u64::from(memory.read_u8((values[0]).wrapping_add(0x33du64))?)) as i8 as i64 as u64)
                as i32 as i64 as u64) as i64)
                != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            if u64::from(
                (((((u64::from(
                    (u64::from(memory.read_u8((values[0]).wrapping_add(0x338u64))?)) as u8,
                )) & (0x10u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                    as i64)
                    == (((0x0u64) as i32 as i64 as u64) as i64),
            ) != 0
            {
                values[3] = memory.read_u64((values[0]).wrapping_add(0x6a0u64))?;
            } else {
                values[3] = memory.read_u64((values[0]).wrapping_add(0x698u64))?;
            }
            if u64::from(((values[3]) as i64) == ((0x0u64) as i64)) != 0 {
                values[4] = relocate("L\"\"");
            } else {
                values[4] = memory.read_u64((values[3]).wrapping_add(0x10u64))?;
            }
            values[1] = (invoke("AfxMessageBox", &[values[4], 0x2010u64, 0x0u64], memory)) as i32
                as i64 as u64;
            if u64::from(
                (((values[1]) as i32 as i64 as u64) as i64)
                    == (((0x1u64) as i32 as i64 as u64) as i64),
            ) != 0
            {
                {
                    let destination = (values[0]).wrapping_add(0x338u64);
                    let stored = u64::from(
                        (((u64::from(
                            (u64::from(memory.read_u16((values[0]).wrapping_add(0x338u64))?))
                                as u16,
                        )) & (0xffe7u64)) as i32 as i64 as u64) as u16,
                    );
                    memory.write_u16(destination, stored as u16)?;
                }
                values[2] = invoke("GetTickCount64", &[], memory);
                {
                    let destination = (values[0]).wrapping_add(0x690u64);
                    let stored = values[2];
                    memory.write_u64(destination, stored)?;
                }
            }
        }
    }
    Ok(None)
}

/// Translation of `FUN_14006f690` (`mspaint.c:99675`).
pub fn fun_14006f690(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64, 0u64, 0u64];
    values[0] = memory.read_u64(relocate("DAT_1401d0b70"))?;
    {
        let destination = (relocate("STACK_local_res10")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = memory.read_u64(relocate("DAT_1401d0b70"))?;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((memory.read_u64(relocate("DAT_1401d0b70"))?) != (0x0u64)) != 0 {
        invoke(
            "INDIRECT_CALL",
            &[
                memory.read_u64(
                    (memory.read_u64(memory.read_u64(relocate("DAT_1401d0b70"))?)?)
                        .wrapping_add(0x8u64),
                )?,
                memory.read_u64(relocate("DAT_1401d0b70"))?,
            ],
            memory,
        );
    }
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x605u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[1] = (invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x1f0u64))?,
            values[0],
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(((values[0]) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[relocate("STACK_local_res10")], memory);
        }
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[1], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140076ec0` (`mspaint.c:104634`).
pub fn fun_140076ec0(
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
            relocate("CAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = (values[2]).wrapping_add(0x10u64);
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
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
            relocate("CRefAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400771d0` (`mspaint.c:104752`).
pub fn fun_1400771d0(
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
    values[3] = invoke(
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
    if u64::from(((values[3]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = invoke(
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
        if u64::from(((values[3]) as i64) == ((0x0u64) as i64)) != 0 {
            return Ok(Some(u64::from((0x0u64) as u8)));
        }
        values[4] = memory.read_u64((values[3]).wrapping_add(0x10u64))?;
        if u64::from(((values[0]).wrapping_add(0x10u64)) != (values[4])) != 0 {
            values[2] = (values[4]).wrapping_add((0x2u64).wrapping_mul(8));
            if u64::from(
                (0x7u64) < (memory.read_u64((values[4]).wrapping_add((0x3u64).wrapping_mul(8)))?),
            ) != 0
            {
                values[4] = memory.read_u64(values[4])?;
            }
            invoke(
                "FUN_140014910",
                &[
                    (values[0]).wrapping_add(0x10u64),
                    values[4],
                    memory.read_u64(values[2])?,
                ],
                memory,
            );
            return Ok(Some(u64::from((0x1u64) as u8)));
        }
    } else {
        values[4] = (values[3]).wrapping_add(0x10u64);
        if u64::from(((values[0]).wrapping_add(0x10u64)) != (values[4])) != 0 {
            if u64::from((0x7u64) < (memory.read_u64((values[3]).wrapping_add(0x28u64))?)) != 0 {
                values[4] = memory.read_u64(values[4])?;
            }
            invoke(
                "FUN_140014910",
                &[
                    (values[0]).wrapping_add(0x10u64),
                    values[4],
                    memory.read_u64((values[3]).wrapping_add(0x20u64))?,
                ],
                memory,
            );
        }
    }
    Ok(Some(u64::from((0x1u64) as u8)))
}

/// Translation of `FUN_140077290` (`mspaint.c:104791`).
pub fn fun_140077290(
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
    if u64::from(((values[2]) as i64) == ((0x0u64) as i64)) != 0 {
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
        if u64::from(((values[2]) as i64) == ((0x0u64) as i64)) != 0 {
            return Ok(Some(u64::from((0x0u64) as u8)));
        }
        values[2] = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
    } else {
        values[2] = (values[2]).wrapping_add(0x10u64);
    }
    values[3] = (values[0]).wrapping_add(0x10u64);
    if u64::from((values[2]) != (values[3])) != 0 {
        if u64::from((0x7u64) < (memory.read_u64((values[0]).wrapping_add(0x28u64))?)) != 0 {
            values[3] = memory.read_u64(values[3])?;
        }
        invoke(
            "FUN_140014910",
            &[
                values[2],
                values[3],
                memory.read_u64((values[0]).wrapping_add(0x20u64))?,
            ],
            memory,
        );
    }
    Ok(Some(u64::from((0x1u64) as u8)))
}

/// Translation of `FUN_1400773c0` (`mspaint.c:104848`).
pub fn fun_1400773c0(
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
            relocate("CAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = (values[2]).wrapping_add(0x10u64);
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
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
            relocate("CRefAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400774e0` (`mspaint.c:104896`).
pub fn fun_1400774e0(
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
            relocate("CAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = (values[2]).wrapping_add(0x10u64);
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
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
            relocate("CRefAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140077600` (`mspaint.c:104944`).
pub fn fun_140077600(
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
            relocate("CAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = (values[2]).wrapping_add(0x10u64);
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
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
            relocate("CRefAttributeValue<>::RTTI_Type_Descriptor"),
            0x0u64,
        ],
        memory,
    );
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        {
            let destination = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                    as u32,
            );
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(Some(0x1u64));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140077690` (`mspaint.c:104966`).
pub fn fun_140077690(
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
    values[2] = memory.read_u64(values[0])?;
    values[4] = 0x0u64;
    {
        let destination = relocate("STACK_local_20");
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = ((u64::from(memory.read_u32((values[2]).wrapping_add(0x60u64))?)) as i32 as i64
            as u64) as i32 as i64 as u64;
        memory.write_u32(destination, stored as u32)?;
    }
    if u64::from(
        ((((u64::from(
            memory.read_u32((relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4)))?,
        )) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = (0x1u64) as i32 as i64 as u64;
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add(0x60u64);
            let stored = u64::from((0x1u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
    }
    values[1] = (values[2]).wrapping_add(0x50u64);
    if u64::from((relocate("STACK_local_20")) != (values[1])) != 0 {
        values[4] = memory.read_u64(values[1])?;
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = relocate("STACK_local_20");
            let stored = values[4];
            memory.write_u64(destination, stored)?;
        }
    }
    invoke(
        "ReleaseSRWLockExclusive",
        &[(values[2]).wrapping_add(0x48u64)],
        memory,
    );
    values[5] = 0x0u64;
    if u64::from(((values[4]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_14007a140",
            &[
                relocate("STACK_local_20"),
                values[2],
                relocate("STACK_local_28"),
            ],
            memory,
        );
        values[5] = memory.read_u64(relocate("STACK_local_20"))?;
    }
    if u64::from(((values[5]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_20")], memory);
    }
    values[2] = memory.read_u64(values[0])?;
    values[3] = (invoke("FUN_140058bc0", &[values[2]], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let destination = (values[2]).wrapping_add(0x8u64);
            let stored = 0x1u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(Some(u64::from(
        (u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0,
    )))
}

/// Translation of `FUN_140077900` (`mspaint.c:105150`).
pub fn fun_140077900(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke(
        "AcquireSRWLockExclusive",
        &[(values[1]).wrapping_add(0x48u64)],
        memory,
    );
    if u64::from(
        ((((u64::from(memory.read_u32((values[1]).wrapping_add(0x60u64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x1u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "ReleaseSRWLockExclusive",
            &[(values[1]).wrapping_add(0x48u64)],
            memory,
        );
        return Ok(Some(0x0u64));
    }
    invoke("FUN_140058c40", &[values[1]], memory);
    {
        let destination = (relocate("STACK_local_38")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x15b7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    invoke(
        "FUN_14000a9d0",
        &[relocate("STACK_local_20"), relocate("STACK_local_38")],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_20"), relocate("DAT_14016a840")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140077af0` (`mspaint.c:105254`).
pub fn fun_140077af0(
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
        values[1] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        {
            let destination = values[2];
            let stored = relocate("CAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x3u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140077b40` (`mspaint.c:105273`).
pub fn fun_140077b40(
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
        values[1] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        {
            let destination = values[2];
            let stored = relocate("CAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x3u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140077b90` (`mspaint.c:105292`).
pub fn fun_140077b90(
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
        values[1] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        {
            let destination = values[2];
            let stored = relocate("CAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x3u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_1400782c0` (`mspaint.c:105744`).
pub fn fun_1400782c0(
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
    values[1] = invoke("operator_new", &[0x30u64], memory);
    if u64::from((values[1]) != (0x0u64)) != 0 {
        values[2] = invoke(
            "FUN_1400199f0",
            &[
                relocate("STACK_local_28"),
                (values[0]).wrapping_add(0x10u64),
            ],
            memory,
        );
        {
            let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x6u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = values[1];
            let stored = relocate("CAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        invoke(
            "FUN_1400199f0",
            &[
                (values[1]).wrapping_add((0x2u64).wrapping_mul(8)),
                values[2],
            ],
            memory,
        );
        invoke("FUN_1400054d0", &[values[2]], memory);
        return Ok(Some(values[1]));
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140078410` (`mspaint.c:105810`).
pub fn fun_140078410(
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
        values[1] = u64::from(
            (u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32))
                as u32,
        );
        {
            let destination = values[2];
            let stored = relocate("CAttributeValue<>::vftable");
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x1u64).wrapping_mul(8));
            let stored = u64::from((0x3u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        {
            let destination = (values[2]).wrapping_add((0x2u64).wrapping_mul(8));
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14007a140` (`mspaint.c:107391`).
pub fn fun_14007a140(
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
    values[6] = (values[1]).wrapping_add(0x10u64);
    {
        let destination = (relocate("STACK_local_30")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xfc7u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.h\"");

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[0])?)?).wrapping_add(0x18u64))?,
            memory.read_u64(values[0])?,
            values[6],
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

/// Translation of `FUN_14007a4f0` (`mspaint.c:107618`).
pub fn fun_14007a4f0(
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
    {
        let destination = values[1];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
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
            let destination = (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x541u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

        invoke(
            "FUN_14001fd10",
            &[relocate("STACK_local_38"), relocate("STACK_local_50")],
            memory,
        );
        invoke(
            "_CxxThrowException",
            &[relocate("STACK_local_38"), relocate("DAT_14016a7e0")],
            memory,
        );
        return Err(AdapterError::Halted("_CxxThrowException"));
    }
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000028");
    }
    values[3] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000030");
    }
    if u64::from((memory.read_u64(values[2])?) == (memory.read_u64(values[3])?)) != 0 {
        {
            let destination = (relocate("STACK_local_50")).wrapping_add((0x0u64).wrapping_mul(4));
            let stored = u64::from((0x612u64) as u32);
            memory.write_u32(destination, stored as u32)?;
        }
        let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Windows.Foundation.Collections.h\"");

        invoke(
            "FUN_14001fd50",
            &[relocate("STACK_local_20"), relocate("STACK_local_50")],
            memory,
        );
        invoke(
            "_CxxThrowException",
            &[relocate("STACK_local_20"), relocate("DAT_14016a660")],
            memory,
        );
        return Err(AdapterError::Halted("_CxxThrowException"));
    }
    values[4] = invoke(
        "FUN_140005fd0",
        &[memory.read_u64(memory.read_u64(values[2])?)?],
        memory,
    );
    {
        let destination = values[1];
        let stored = values[4];
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14007aa80` (`mspaint.c:107954`).
pub fn fun_14007aa80(
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

/// Translation of `FUN_14007acf0` (`mspaint.c:108099`).
pub fn fun_14007acf0(
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

/// Translation of `FUN_14007b4a0` (`mspaint.c:108584`).
pub fn fun_14007b4a0(
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
    invoke("FUN_14000ca50", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14007b4d0` (`mspaint.c:108596`).
pub fn fun_14007b4d0(
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
    invoke("FUN_1400789c0", &[relocate("STACK_local_18")], memory);
    {
        let destination = values[1];
        let stored =
            memory.read_u64((relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8)))?;
        memory.write_u64(destination, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14007b7f0` (`mspaint.c:108765`).
pub fn fun_14007b7f0(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64((values[0]).wrapping_add(0x18u64))?,
            memory.read_u64((values[0]).wrapping_add(0x10u64))?,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14007b800` (`mspaint.c:108776`).
pub fn fun_14007b800(
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
    values[2] = memory.read_u64((values[1]).wrapping_add((0x7u64).wrapping_mul(8)))?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        if u64::from((values[2]) == (values[1])) != 0 {
            values[3] = invoke(
                "INDIRECT_CALL",
                &[
                    memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x8u64))?,
                    values[2],
                    values[0],
                ],
                memory,
            );
            {
                let destination = (values[0]).wrapping_add(0x38u64);
                let stored = values[3];
                memory.write_u64(destination, stored)?;
            }
            values[2] = memory.read_u64((values[1]).wrapping_add((0x7u64).wrapping_mul(8)))?;
            if u64::from((values[2]) != (0x0u64)) != 0 {
                invoke(
                    "INDIRECT_CALL",
                    &[
                        memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x20u64))?,
                        values[2],
                        u64::from((values[2]) != (values[1])),
                    ],
                    memory,
                );
                {
                    let destination = (values[1]).wrapping_add((0x7u64).wrapping_mul(8));
                    let stored = 0x0u64;
                    memory.write_u64(destination, stored)?;
                }
                return Ok(None);
            }
        } else {
            {
                let destination = (values[0]).wrapping_add(0x38u64);
                let stored = values[2];
                memory.write_u64(destination, stored)?;
            }
            {
                let destination = (values[1]).wrapping_add((0x7u64).wrapping_mul(8));
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
    }
    Ok(None)
}

/// Translation of `FUN_14007bf40` (`mspaint.c:109126`).
pub fn fun_14007bf40(
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

/// Translation of `FUN_14007c020` (`mspaint.c:109183`).
pub fn fun_14007c020(
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

/// Translation of `FUN_14007c0f0` (`mspaint.c:109248`).
pub fn fun_14007c0f0(
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
    {
        let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u8((values[0]).wrapping_add(0x8u64))?)) as u8)) as u8,
        );
        memory.write_u8(destination, stored as u8)?;
    }
    values[2] = memory.read_u64((values[0]).wrapping_add(0x10u64))?;
    {
        let destination = (values[1]).wrapping_add((0x2u64).wrapping_mul(8));
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

/// Translation of `FUN_14007c140` (`mspaint.c:109274`).
pub fn fun_14007c140(
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
    invoke("FUN_14006c970", &[(values[0]).wrapping_add(0x8u64)], memory);
    {
        let destination = (relocate("STACK_local_res8")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x3u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    invoke(
        "FUN_14006ca90",
        &[
            (values[0]).wrapping_add(0x8u64),
            relocate("STACK_local_res8"),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14007c190` (`mspaint.c:109303`).
pub fn fun_14007c190(
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

/// Translation of `FUN_14007c210` (`mspaint.c:109353`).
pub fn fun_14007c210(
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

/// Translation of `FUN_14007c380` (`mspaint.c:109480`).
pub fn fun_14007c380(
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
    let mut values = [arguments[0], u64::from((arguments[1]) as u32), 0u64];
    values[2] = values[1];
    invoke(
        "FUN_1400131e0",
        &[relocate("STACK_local_18"), 0x8000ffffu64],
        memory,
    );
    invoke("FUN_140012f00",&[values[0],(values[2]) & (0xffffffffu64),relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\packages\\\\Microsoft.Windows.ImplementationLibrary.1.0.250325.1\\\\include\\\\wil\\\\resource.h\"")],memory);
    Err(AdapterError::Halted("FUN_140012f00"))
}

/// Translation of `FUN_14007c3e0` (`mspaint.c:109496`).
pub fn fun_14007c3e0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_14007c380", &[], memory);
    Err(AdapterError::Halted("FUN_14007c380"))
}

/// Translation of `FUN_14007c620` (`mspaint.c:109613`).
pub fn fun_14007c620(
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
    invoke(
        "FUN_1400324c0",
        &[
            values[0],
            0x534u64,
            relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\pbrush.cpp\""),
        ],
        memory,
    );
    Err(AdapterError::Halted("FUN_1400324c0"))
}

/// Translation of `FUN_14007c650` (`mspaint.c:109622`).
pub fn fun_14007c650(
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
    invoke(
        "FUN_1400324c0",
        &[
            values[0],
            values[1],
            relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\pbrush.cpp\""),
        ],
        memory,
    );
    Err(AdapterError::Halted("FUN_1400324c0"))
}

/// Translation of `FUN_14007cfc0` (`mspaint.c:110104`).
pub fn fun_14007cfc0(
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
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    invoke("CView::~CView", &[values[0]], memory);
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        if u64::from(
            (u64::from((u64::from(((values[1]) & (0x4u64)) as u32)) as u32))
                == (u64::from((0x0u64) as u32)),
        ) != 0
        {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x110u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14007dd30` (`mspaint.c:110802`).
pub fn fun_14007dd30(
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
    if u64::from((values[2]) != (0x0u64)) != 0 {
        if u64::from(
            ((memory.read_u64((values[2]).wrapping_add((0x5u64).wrapping_mul(8)))?) as i64)
                != ((0x0u64) as i64),
        ) != 0
        {
            invoke("FUN_14007d910", &[], memory);
            {
                let destination = (values[2]).wrapping_add((0x5u64).wrapping_mul(8));
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        values[3] = memory.read_u64(
            (memory.read_u64((memory.read_u64(values[2])?).wrapping_add(0x130u64))?)
                .wrapping_add(0x20u64),
        )?;
        if u64::from((values[3]) != (0x0u64)) != 0 {
            invoke("DeleteDC", &[values[3]], memory);
            {
                let destination = (memory
                    .read_u64((memory.read_u64(values[2])?).wrapping_add(0x130u64))?)
                .wrapping_add(0x20u64);
                let stored = 0x0u64;
                memory.write_u64(destination, stored)?;
            }
        }
        {
            let destination = values[2];
            let stored = memory.read_u64((values[0]).wrapping_add(0x108u64))?;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = (values[0]).wrapping_add(0x100u64);
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
    }
    Ok(None)
}

/// Translation of `FUN_140080920` (`mspaint.c:112981`).
pub fn fun_140080920(
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
    invoke("TracePrint", &[], memory);
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
    invoke("FUN_140045ed0", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_140083a60` (`mspaint.c:115203`).
pub fn fun_140083a60(
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
        "INDIRECT_CALL",
        &[
            memory.read_u64(
                (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0xe8u64))?)?)
                    .wrapping_add(0x2e8u64),
            )?,
            memory.read_u64((values[0]).wrapping_add(0xe8u64))?,
            0x0u64,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140084940` (`mspaint.c:115914`).
pub fn fun_140084940(
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
    invoke(
        "FUN_140084970",
        &[values[0], relocate("STACK_local_res10"), 0x0u64],
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
    Ok(None)
}

/// Translation of `FUN_140084b60` (`mspaint.c:116023`).
pub fn fun_140084b60(
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
    values[1] = invoke("operator_new", &[0x510u64], memory);
    {
        let destination = (values[1]).wrapping_add((0x1u64).wrapping_mul(8));
        let stored = relocate("FUN_1400c8c20");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = values[1];
        let stored = relocate("FUN_1400c8c30");
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add((0x3u64).wrapping_mul(8));
        let stored = values[0];
        memory.write_u64(destination, stored)?;
    }
    {
        let destination = (values[1]).wrapping_add((0x4u64).wrapping_mul(8));
        let stored = u64::from((0x10002u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = (values[1]).wrapping_add(0x11u64);
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8(destination, stored as u8)?;
    }
    invoke("FUN_1400c8c30", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140084f70` (`mspaint.c:116285`).
pub fn fun_140084f70(
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
    invoke(
        "INDIRECT_CALL",
        &[
            memory.read_u64((memory.read_u64(values[1])?).wrapping_add(0x8u64))?,
            values[1],
            0x0u64,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140085520` (`mspaint.c:116616`).
pub fn fun_140085520(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64, 0u64];
    values[0] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);

    invoke(
        "Paint::Telemetry::TraceLogger::SendCommandEvent",
        &[values[0], 0x1du64, 0x15u64, relocate("STACK_local_48")],
        memory,
    );
    invoke("FUN_140032c20", &[relocate("STACK_local_res10")], memory);
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

/// Translation of `FUN_140085e80` (`mspaint.c:117096`).
pub fn fun_140085e80(
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
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x29dfu64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }
    {
        let destination = values[0];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\PaintUI.h\"");

    values[2] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[1])?)?).wrapping_add(0x30u64))?,
            memory.read_u64(values[1])?,
            values[0],
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

/// Translation of `FUN_14008d170` (`mspaint.c:122250`).
pub fn fun_14008d170(
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
        "INDIRECT_CALL",
        &[memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x110u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140090290` (`mspaint.c:124721`).
pub fn fun_140090290(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("_CxxThrowException", &[0x0u64, 0x0u64], memory);
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_1400902a0` (`mspaint.c:124730`).
pub fn fun_1400902a0(
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
    invoke(
        "FUN_14008ff50",
        &[relocate("STACK_local_d8"), values[0]],
        memory,
    );
    invoke(
        "_CxxThrowException",
        &[relocate("STACK_local_d8"), relocate("DAT_14016af08")],
        memory,
    );
    Err(AdapterError::Halted("_CxxThrowException"))
}

/// Translation of `FUN_140092da0` (`mspaint.c:126826`).
pub fn fun_140092da0(
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
    invoke(
        "PostMessageW",
        &[
            memory.read_u64((values[0]).wrapping_add(0x40u64))?,
            0x10u64,
            0x0u64,
            0x0u64,
        ],
        memory,
    );
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Input.h\"");
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0x705u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x48u64))?,
            memory.read_u64(values[2])?,
            0x1u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140092e10` (`mspaint.c:126852`).
pub fn fun_140092e10(
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
    invoke(
        "PostMessageW",
        &[
            memory.read_u64((values[0]).wrapping_add(0x40u64))?,
            0x10u64,
            0x0u64,
            0x0u64,
        ],
        memory,
    );
    let _ = relocate("\"D:\\\\a\\\\_work\\\\1\\\\s\\\\src\\\\paintapp\\\\x64\\\\Release\\\\Generated Files\\\\winrt\\\\Microsoft.UI.Xaml.Input.h\"");
    {
        let destination = (relocate("STACK_local_28")).wrapping_add((0x0u64).wrapping_mul(4));
        let stored = u64::from((0xdb9u64) as u32);
        memory.write_u32(destination, stored as u32)?;
    }

    values[3] = (invoke(
        "INDIRECT_CALL",
        &[
            memory
                .read_u64((memory.read_u64(memory.read_u64(values[2])?)?).wrapping_add(0x48u64))?,
            memory.read_u64(values[2])?,
            0x1u64,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            < (((values[3]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        return Ok(None);
    }
    invoke(
        "FUN_140002540",
        &[values[3], relocate("STACK_local_28")],
        memory,
    );
    Err(AdapterError::Halted("FUN_140002540"))
}

/// Translation of `FUN_140093420` (`mspaint.c:127157`).
pub fn fun_140093420(
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
        let destination = values[0];
        let stored = 0x0u64;
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[0]) != (values[1])) != 0 {
        values[3] = memory.read_u64(values[1])?;
        {
            let destination = values[1];
            let stored = 0x0u64;
            memory.write_u64(destination, stored)?;
        }
        {
            let destination = values[0];
            let stored = values[3];
            memory.write_u64(destination, stored)?;
        }
    }
    {
        let destination = (relocate("STACK_local_18")).wrapping_add((0x0u64).wrapping_mul(8));
        let stored = values[2];
        memory.write_u64(destination, stored)?;
    }
    if u64::from((values[1]) != (relocate("STACK_local_18"))) != 0 {
        if u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[1]], memory);
        }
        {
            let destination = values[1];
            let stored = values[2];
            memory.write_u64(destination, stored)?;
        }
        return Ok(None);
    }
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[relocate("STACK_local_18")], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140093514` (`mspaint.c:127216`).
pub fn fun_140093514(
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
        "INDIRECT_CALL",
        &[memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x428u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140093524` (`mspaint.c:127227`).
pub fn fun_140093524(
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
        "INDIRECT_CALL",
        &[memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x4e8u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140094f40` (`mspaint.c:128516`).
pub fn fun_140094f40(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke(
        "std::_Xlength_error",
        &[relocate("\"deque<T> too long\"")],
        memory,
    );
    Err(AdapterError::Halted("std::_Xlength_error"))
}

/// Translation of `FUN_140095da0` (`mspaint.c:129303`).
pub fn fun_140095da0(
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
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x70u64))?)?)
                .wrapping_add(0x38u64),
        )?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140095e40` (`mspaint.c:129343`).
pub fn fun_140095e40(
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
        "INDIRECT_CALL",
        &[memory.read_u64(
            (memory.read_u64(memory.read_u64((values[0]).wrapping_add(0x70u64))?)?)
                .wrapping_add(0x48u64),
        )?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140096070` (`mspaint.c:129485`).
pub fn fun_140096070(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
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

/// Translation of `FUN_1400961c0` (`mspaint.c:129601`).
pub fn fun_1400961c0(
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
    values[2] = (values[0]).wrapping_add(0x50u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000078");
    }
    values[2] = memory.read_u64(values[2])?;
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

pub type Handler = fn(
    &[u64],
    &mut MemoryImage,
    &mut RelocateCall<'_>,
    &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError>;
pub const HANDLERS: &[(&str, usize, Handler)] = &[
    ("FUN_140001010", 0, fun_140001010),
    ("FUN_140001bc0", 2, fun_140001bc0),
    ("FUN_140002770", 3, fun_140002770),
    ("FUN_1400031b0", 2, fun_1400031b0),
    ("FUN_140003e90", 1, fun_140003e90),
    ("FUN_140004150", 2, fun_140004150),
    ("FUN_1400045c0", 1, fun_1400045c0),
    ("FUN_140004ba0", 4, fun_140004ba0),
    ("FUN_140004bd0", 1, fun_140004bd0),
    ("FUN_140004bf0", 1, fun_140004bf0),
    ("FUN_140004d80", 0, fun_140004d80),
    ("FUN_1400056f0", 0, fun_1400056f0),
    ("FUN_140005800", 0, fun_140005800),
    ("FUN_140006680", 1, fun_140006680),
    ("FUN_140006790", 1, fun_140006790),
    ("FUN_1400067b0", 1, fun_1400067b0),
    ("FUN_1400067d0", 1, fun_1400067d0),
    ("FUN_140006920", 2, fun_140006920),
    ("FUN_1400091c0", 1, fun_1400091c0),
    ("FUN_1400092f0", 1, fun_1400092f0),
    ("FUN_140009560", 2, fun_140009560),
    ("FUN_1400095c0", 1, fun_1400095c0),
    ("FUN_1400096b0", 2, fun_1400096b0),
    ("FUN_140009760", 2, fun_140009760),
    ("FUN_1400099b0", 4, fun_1400099b0),
    ("FUN_140009a40", 1, fun_140009a40),
    ("FUN_140009b30", 1, fun_140009b30),
    ("FUN_140009e00", 2, fun_140009e00),
    ("FUN_140009eb0", 3, fun_140009eb0),
    ("FUN_14000a0c0", 2, fun_14000a0c0),
    ("FUN_14000a3c0", 0, fun_14000a3c0),
    ("FUN_14000a4a0", 0, fun_14000a4a0),
    ("FUN_14000a550", 1, fun_14000a550),
    ("FUN_14000a5c0", 2, fun_14000a5c0),
    ("FUN_14000a610", 1, fun_14000a610),
    ("FUN_14000c240", 1, fun_14000c240),
    ("FUN_14000c570", 1, fun_14000c570),
    ("FUN_14000c7e0", 1, fun_14000c7e0),
    ("FUN_14000cc70", 3, fun_14000cc70),
    ("FUN_14000ce50", 2, fun_14000ce50),
    ("FUN_14000d150", 1, fun_14000d150),
    ("FUN_14000d170", 1, fun_14000d170),
    ("FUN_14000d2d0", 1, fun_14000d2d0),
    ("FUN_14000d3f0", 2, fun_14000d3f0),
    ("FUN_14000e8b0", 1, fun_14000e8b0),
    ("FUN_14000f7d0", 1, fun_14000f7d0),
    ("FUN_14000f800", 1, fun_14000f800),
    ("FUN_14000f820", 1, fun_14000f820),
    ("FUN_140011310", 1, fun_140011310),
    ("FUN_140011c70", 2, fun_140011c70),
    ("FUN_140011cf0", 0, fun_140011cf0),
    ("FUN_1400123f0", 7, fun_1400123f0),
    ("FUN_140012b50", 0, fun_140012b50),
    ("FUN_140012f00", 0, fun_140012f00),
    ("FUN_1400148e0", 0, fun_1400148e0),
    ("FUN_1400180d0", 1, fun_1400180d0),
    ("FUN_140018e80", 3, fun_140018e80),
    ("FUN_140018f30", 3, fun_140018f30),
    ("FUN_1400199f0", 2, fun_1400199f0),
    ("FUN_14001a0b0", 2, fun_14001a0b0),
    ("FUN_14001ba30", 2, fun_14001ba30),
    ("FUN_14001bc30", 1, fun_14001bc30),
    ("FUN_14001c740", 2, fun_14001c740),
    ("FUN_14001d460", 2, fun_14001d460),
    ("FUN_14001d750", 2, fun_14001d750),
    ("FUN_14001dad0", 1, fun_14001dad0),
    ("FUN_14001dda0", 2, fun_14001dda0),
    ("FUN_14001de60", 2, fun_14001de60),
    ("FUN_14001df10", 2, fun_14001df10),
    ("FUN_14001e1c0", 2, fun_14001e1c0),
    ("FUN_14001f590", 3, fun_14001f590),
    ("FUN_1400222b0", 0, fun_1400222b0),
    ("FUN_140022c80", 1, fun_140022c80),
    ("FUN_140024700", 1, fun_140024700),
    ("FUN_140024c90", 1, fun_140024c90),
    ("FUN_140024e80", 1, fun_140024e80),
    ("FUN_140026e70", 0, fun_140026e70),
    ("FUN_14002c610", 0, fun_14002c610),
    ("FUN_14002c750", 0, fun_14002c750),
    ("FUN_140031bd0", 0, fun_140031bd0),
    ("FUN_140032060", 1, fun_140032060),
    ("FUN_140032490", 0, fun_140032490),
    ("FUN_140032520", 1, fun_140032520),
    ("FUN_140033c90", 1, fun_140033c90),
    ("FUN_140036cf0", 2, fun_140036cf0),
    ("FUN_1400385a0", 1, fun_1400385a0),
    ("FUN_1400385d0", 1, fun_1400385d0),
    ("FUN_140038890", 2, fun_140038890),
    ("FUN_140038ea0", 1, fun_140038ea0),
    ("FUN_140039d40", 1, fun_140039d40),
    ("FUN_14003aee0", 2, fun_14003aee0),
    ("FUN_14003b170", 1, fun_14003b170),
    ("FUN_14003b2d0", 0, fun_14003b2d0),
    ("FUN_140041620", 2, fun_140041620),
    ("FUN_1400419f0", 2, fun_1400419f0),
    ("FUN_140041a50", 2, fun_140041a50),
    ("FUN_140041aa0", 2, fun_140041aa0),
    ("FUN_140042990", 2, fun_140042990),
    ("FUN_140045d40", 2, fun_140045d40),
    ("FUN_140047b60", 2, fun_140047b60),
    ("FUN_140048000", 2, fun_140048000),
    ("FUN_14004ac40", 2, fun_14004ac40),
    ("FUN_14004ad40", 2, fun_14004ad40),
    ("FUN_14004ada0", 2, fun_14004ada0),
    ("FUN_14004b8d0", 2, fun_14004b8d0),
    ("FUN_14004ce20", 2, fun_14004ce20),
    ("FUN_14004d150", 0, fun_14004d150),
    ("FUN_14004d1b0", 2, fun_14004d1b0),
    ("FUN_14004d370", 2, fun_14004d370),
    ("FUN_140051b70", 2, fun_140051b70),
    ("FUN_140051f00", 0, fun_140051f00),
    ("FUN_140052fa0", 0, fun_140052fa0),
    ("FUN_1400534b0", 2, fun_1400534b0),
    ("FUN_140053970", 0, fun_140053970),
    ("FUN_140053bf0", 3, fun_140053bf0),
    ("FUN_140053c80", 2, fun_140053c80),
    ("FUN_140053cc0", 2, fun_140053cc0),
    ("FUN_140053d40", 2, fun_140053d40),
    ("FUN_140053dc0", 2, fun_140053dc0),
    ("FUN_140053e40", 0, fun_140053e40),
    ("FUN_140053eb0", 0, fun_140053eb0),
    ("FUN_140054010", 0, fun_140054010),
    ("FUN_1400540f0", 2, fun_1400540f0),
    ("FUN_140055060", 2, fun_140055060),
    ("FUN_1400561d0", 2, fun_1400561d0),
    ("FUN_140056410", 1, fun_140056410),
    ("FUN_140056560", 1, fun_140056560),
    ("FUN_1400567d0", 2, fun_1400567d0),
    ("FUN_140056820", 1, fun_140056820),
    ("FUN_140056910", 2, fun_140056910),
    ("FUN_1400569d0", 2, fun_1400569d0),
    ("FUN_140056c20", 4, fun_140056c20),
    ("FUN_140056e30", 2, fun_140056e30),
    ("FUN_140057030", 1, fun_140057030),
    ("FUN_140057880", 1, fun_140057880),
    ("FUN_140057f80", 1, fun_140057f80),
    ("FUN_140058190", 2, fun_140058190),
    ("FUN_1400592b0", 3, fun_1400592b0),
    ("FUN_140059a40", 2, fun_140059a40),
    ("FUN_140059a70", 2, fun_140059a70),
    ("FUN_140059aa0", 2, fun_140059aa0),
    ("FUN_14005a270", 2, fun_14005a270),
    ("FUN_14005b480", 3, fun_14005b480),
    ("FUN_14005d5f0", 1, fun_14005d5f0),
    ("FUN_140061210", 1, fun_140061210),
    ("FUN_140062150", 2, fun_140062150),
    ("FUN_140063dc0", 1, fun_140063dc0),
    ("FUN_140063e30", 1, fun_140063e30),
    ("FUN_140065540", 2, fun_140065540),
    ("FUN_140065600", 2, fun_140065600),
    ("FUN_1400656b0", 2, fun_1400656b0),
    ("FUN_140065990", 2, fun_140065990),
    ("FUN_1400661d0", 2, fun_1400661d0),
    ("FUN_140066340", 1, fun_140066340),
    ("FUN_140066360", 1, fun_140066360),
    ("FUN_140066380", 1, fun_140066380),
    ("FUN_1400663b0", 1, fun_1400663b0),
    ("FUN_1400663d0", 1, fun_1400663d0),
    ("FUN_1400663f0", 1, fun_1400663f0),
    ("FUN_140066410", 1, fun_140066410),
    ("FUN_140066430", 1, fun_140066430),
    ("FUN_140066470", 1, fun_140066470),
    ("FUN_14006a1d0", 2, fun_14006a1d0),
    ("FUN_14006aaa0", 0, fun_14006aaa0),
    ("FUN_14006b110", 1, fun_14006b110),
    ("FUN_14006f5d0", 1, fun_14006f5d0),
    ("FUN_14006f690", 0, fun_14006f690),
    ("FUN_140076ec0", 2, fun_140076ec0),
    ("FUN_1400771d0", 2, fun_1400771d0),
    ("FUN_140077290", 2, fun_140077290),
    ("FUN_1400773c0", 2, fun_1400773c0),
    ("FUN_1400774e0", 2, fun_1400774e0),
    ("FUN_140077600", 2, fun_140077600),
    ("FUN_140077690", 1, fun_140077690),
    ("FUN_140077900", 1, fun_140077900),
    ("FUN_140077af0", 1, fun_140077af0),
    ("FUN_140077b40", 1, fun_140077b40),
    ("FUN_140077b90", 1, fun_140077b90),
    ("FUN_1400782c0", 1, fun_1400782c0),
    ("FUN_140078410", 1, fun_140078410),
    ("FUN_14007a140", 3, fun_14007a140),
    ("FUN_14007a4f0", 2, fun_14007a4f0),
    ("FUN_14007aa80", 2, fun_14007aa80),
    ("FUN_14007acf0", 2, fun_14007acf0),
    ("FUN_14007b4a0", 2, fun_14007b4a0),
    ("FUN_14007b4d0", 2, fun_14007b4d0),
    ("FUN_14007b7f0", 1, fun_14007b7f0),
    ("FUN_14007b800", 2, fun_14007b800),
    ("FUN_14007bf40", 2, fun_14007bf40),
    ("FUN_14007c020", 2, fun_14007c020),
    ("FUN_14007c0f0", 2, fun_14007c0f0),
    ("FUN_14007c140", 1, fun_14007c140),
    ("FUN_14007c190", 2, fun_14007c190),
    ("FUN_14007c210", 2, fun_14007c210),
    ("FUN_14007c380", 2, fun_14007c380),
    ("FUN_14007c3e0", 0, fun_14007c3e0),
    ("FUN_14007c620", 1, fun_14007c620),
    ("FUN_14007c650", 2, fun_14007c650),
    ("FUN_14007cfc0", 2, fun_14007cfc0),
    ("FUN_14007dd30", 3, fun_14007dd30),
    ("FUN_140080920", 1, fun_140080920),
    ("FUN_140083a60", 1, fun_140083a60),
    ("FUN_140084940", 1, fun_140084940),
    ("FUN_140084b60", 1, fun_140084b60),
    ("FUN_140084f70", 2, fun_140084f70),
    ("FUN_140085520", 0, fun_140085520),
    ("FUN_140085e80", 2, fun_140085e80),
    ("FUN_14008d170", 1, fun_14008d170),
    ("FUN_140090290", 0, fun_140090290),
    ("FUN_1400902a0", 1, fun_1400902a0),
    ("FUN_140092da0", 3, fun_140092da0),
    ("FUN_140092e10", 3, fun_140092e10),
    ("FUN_140093420", 2, fun_140093420),
    ("FUN_140093514", 1, fun_140093514),
    ("FUN_140093524", 1, fun_140093524),
    ("FUN_140094f40", 0, fun_140094f40),
    ("FUN_140095da0", 1, fun_140095da0),
    ("FUN_140095e40", 1, fun_140095e40),
    ("FUN_140096070", 1, fun_140096070),
    ("FUN_1400961c0", 2, fun_1400961c0),
];
/// Scratch requirements by original function; regions must remain live through the call.
pub const SCRATCH_LAYOUTS: &[(&str, &[(&str, usize)])] = &[
    ("FUN_1400031b0", &[("STACK_local_res8", 32)]),
    ("FUN_140004d80", &[("STACK_local_res8", 32)]),
    ("FUN_140005800", &[("STACK_local_28", 40)]),
    ("FUN_140006920", &[("STACK_local_28", 32)]),
    (
        "FUN_1400091c0",
        &[("STACK_local_28", 8), ("STACK_local_20", 8)],
    ),
    ("FUN_140009560", &[("STACK_local_res8", 32)]),
    (
        "FUN_1400096b0",
        &[("STACK_local_38", 8), ("STACK_local_20", 24)],
    ),
    (
        "FUN_1400099b0",
        &[("STACK_local_20", 16), ("STACK_local_10", 8)],
    ),
    ("FUN_140009eb0", &[("STACK_local_30", 8)]),
    ("FUN_14000a0c0", &[("STACK_local_18", 16)]),
    ("FUN_14000a4a0", &[("STACK_local_res8", 32)]),
    ("FUN_14000a550", &[("STACK_local_res10", 24)]),
    (
        "FUN_14000c240",
        &[("STACK_local_38", 8), ("STACK_local_20", 32)],
    ),
    (
        "FUN_14000c570",
        &[("STACK_local_28", 8), ("STACK_local_20", 8)],
    ),
    (
        "FUN_14000c7e0",
        &[("STACK_local_38", 8), ("STACK_local_20", 24)],
    ),
    ("FUN_14000cc70", &[("STACK_local_30", 8)]),
    ("FUN_14000ce50", &[("STACK_local_18", 16)]),
    ("FUN_140011c70", &[("STACK_local_18", 16)]),
    ("FUN_1400123f0", &[("STACK_local_18", 4)]),
    ("FUN_140018e80", &[("STACK_local_18", 16)]),
    ("FUN_14001d750", &[("STACK_local_18", 8)]),
    (
        "FUN_14001dda0",
        &[("STACK_local_38", 8), ("STACK_local_20", 32)],
    ),
    (
        "FUN_14001de60",
        &[("STACK_local_38", 8), ("STACK_local_20", 32)],
    ),
    (
        "FUN_14001df10",
        &[
            ("STACK_local_58", 8),
            ("STACK_local_40", 24),
            ("STACK_local_28", 32),
        ],
    ),
    (
        "FUN_14001e1c0",
        &[
            ("STACK_local_58", 8),
            ("STACK_local_40", 24),
            ("STACK_local_28", 32),
        ],
    ),
    ("FUN_14001f590", &[("STACK_local_30", 8)]),
    ("FUN_140024e80", &[("STACK_local_28", 8)]),
    ("FUN_140031bd0", &[("STACK_local_28", 40)]),
    ("FUN_140032060", &[("STACK_local_28", 40)]),
    ("FUN_140036cf0", &[("STACK_local_res8", 8)]),
    ("FUN_14003b170", &[("STACK_local_18", 24)]),
    ("FUN_14003b2d0", &[("STACK_local_28", 40)]),
    ("FUN_140041620", &[("STACK_local_res8", 8)]),
    ("FUN_1400419f0", &[("STACK_local_res8", 32)]),
    ("FUN_140042990", &[("STACK_local_18", 8)]),
    (
        "FUN_140052fa0",
        &[("STACK_local_res10", 24), ("STACK_local_28", 8)],
    ),
    (
        "FUN_140053970",
        &[("STACK_local_res10", 24), ("STACK_local_28", 8)],
    ),
    (
        "FUN_140053bf0",
        &[("STACK_local_20", 8), ("STACK_local_28", 8)],
    ),
    ("FUN_140053c80", &[("STACK_local_res10", 24)]),
    (
        "FUN_140053cc0",
        &[("STACK_local_res18", 16), ("STACK_local_28", 8)],
    ),
    (
        "FUN_140053d40",
        &[("STACK_local_res18", 16), ("STACK_local_28", 8)],
    ),
    (
        "FUN_140053dc0",
        &[("STACK_local_30", 8), ("STACK_local_38", 8)],
    ),
    (
        "FUN_140053e40",
        &[("STACK_local_res10", 24), ("STACK_local_28", 8)],
    ),
    (
        "FUN_140053eb0",
        &[("STACK_local_res10", 24), ("STACK_local_28", 8)],
    ),
    ("FUN_140054010", &[("STACK_local_res10", 24)]),
    (
        "FUN_1400540f0",
        &[("STACK_local_20", 8), ("STACK_local_28", 8)],
    ),
    (
        "FUN_1400561d0",
        &[("STACK_local_38", 8), ("STACK_local_20", 24)],
    ),
    (
        "FUN_140056410",
        &[("STACK_local_28", 8), ("STACK_local_20", 8)],
    ),
    ("FUN_1400567d0", &[("STACK_local_res8", 32)]),
    (
        "FUN_140056910",
        &[("STACK_local_48", 8), ("STACK_local_30", 24)],
    ),
    (
        "FUN_140056c20",
        &[("STACK_local_20", 16), ("STACK_local_10", 8)],
    ),
    (
        "FUN_140056e30",
        &[("STACK_local_38", 8), ("STACK_local_20", 24)],
    ),
    ("FUN_140057f80", &[("STACK_local_28", 8)]),
    (
        "FUN_140058190",
        &[("STACK_local_48", 8), ("STACK_local_30", 24)],
    ),
    ("FUN_1400592b0", &[("STACK_local_30", 8)]),
    ("FUN_140059a70", &[("STACK_local_18", 16)]),
    ("FUN_140059aa0", &[("STACK_local_18", 16)]),
    ("FUN_14005a270", &[("STACK_local_18", 16)]),
    ("FUN_140062150", &[("STACK_local_res18", 16)]),
    ("FUN_140063dc0", &[("STACK_local_28", 8)]),
    ("FUN_140063e30", &[("STACK_local_28", 8)]),
    (
        "FUN_140065540",
        &[("STACK_local_38", 8), ("STACK_local_20", 32)],
    ),
    (
        "FUN_140065600",
        &[("STACK_local_38", 8), ("STACK_local_20", 32)],
    ),
    (
        "FUN_1400656b0",
        &[
            ("STACK_local_58", 8),
            ("STACK_local_40", 24),
            ("STACK_local_28", 32),
        ],
    ),
    (
        "FUN_14006f690",
        &[("STACK_local_res10", 24), ("STACK_local_28", 8)],
    ),
    (
        "FUN_140077690",
        &[("STACK_local_28", 8), ("STACK_local_20", 8)],
    ),
    (
        "FUN_140077900",
        &[("STACK_local_38", 8), ("STACK_local_20", 24)],
    ),
    ("FUN_1400782c0", &[("STACK_local_28", 32)]),
    ("FUN_14007a140", &[("STACK_local_30", 8)]),
    (
        "FUN_14007a4f0",
        &[
            ("STACK_local_50", 8),
            ("STACK_local_38", 24),
            ("STACK_local_20", 24),
        ],
    ),
    ("FUN_14007b4a0", &[("STACK_local_18", 16)]),
    ("FUN_14007b4d0", &[("STACK_local_18", 16)]),
    ("FUN_14007c140", &[("STACK_local_res8", 32)]),
    ("FUN_14007c380", &[("STACK_local_18", 24)]),
    ("FUN_140080920", &[("STACK_local_res10", 24)]),
    ("FUN_140084940", &[("STACK_local_res10", 24)]),
    (
        "FUN_140085520",
        &[("STACK_local_res10", 24), ("STACK_local_48", 56)],
    ),
    ("FUN_140085e80", &[("STACK_local_28", 8)]),
    ("FUN_1400902a0", &[("STACK_local_d8", 216)]),
    ("FUN_140092da0", &[("STACK_local_28", 8)]),
    ("FUN_140092e10", &[("STACK_local_28", 8)]),
    ("FUN_140093420", &[("STACK_local_18", 16)]),
];
