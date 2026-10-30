//! Restricted translations generated from reviewed original bodies by batch 009.
use super::call_support::{MemoryFault, MemoryImage, MissingArguments, RecordCall, RelocateCall};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterError {
    Memory(MemoryFault),
    Arguments(MissingArguments),
}
impl From<MemoryFault> for AdapterError {
    fn from(e: MemoryFault) -> Self {
        Self::Memory(e)
    }
}
/// Translation of `FUN_140001ee0` (`mspaint.c:5963`).
pub fn fun_140001ee0(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    invoke(
        "__std_exception_destroy",
        &[(values[0]).wrapping_add((0x1u64).wrapping_mul(8))],
        memory,
    );
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140001f30` (`mspaint.c:5976`).
pub fn fun_140001f30(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::invalid_argument::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140001f70` (`mspaint.c:5989`).
pub fn fun_140001f70(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::logic_error::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140001fe0` (`mspaint.c:6013`).
pub fn fun_140001fe0(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::bad_alloc::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140002040` (`mspaint.c:6040`).
pub fn fun_140002040(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    Ok(Some(values[0]))
}

/// Translation of `FUN_140002e60` (`mspaint.c:6858`).
pub fn fun_140002e60(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140002e80` (`mspaint.c:6873`).
pub fn fun_140002e80(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140002ea0` (`mspaint.c:6888`).
pub fn fun_140002ea0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140002f70` (`mspaint.c:6966`).
pub fn fun_140002f70(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140002f90` (`mspaint.c:6981`).
pub fn fun_140002f90(
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
    values[1] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x48u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140003030` (`mspaint.c:7020`).
pub fn fun_140003030(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003050` (`mspaint.c:7035`).
pub fn fun_140003050(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003070` (`mspaint.c:7050`).
pub fn fun_140003070(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003140` (`mspaint.c:7128`).
pub fn fun_140003140(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003160` (`mspaint.c:7143`).
pub fn fun_140003160(
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
    let mut values = [arguments[0], u64::from((arguments[1]) as u8), 0u64];
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    invoke("FUN_1400029b0", &[values[2], 0x0u64, values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140003230` (`mspaint.c:7208`).
pub fn fun_140003230(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003250` (`mspaint.c:7223`).
pub fn fun_140003250(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003270` (`mspaint.c:7238`).
pub fn fun_140003270(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140003340` (`mspaint.c:7316`).
pub fn fun_140003340(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_1400057c0` (`mspaint.c:9590`).
pub fn fun_1400057c0(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::bad_array_new_length::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140009a20` (`mspaint.c:13213`).
pub fn fun_140009a20(
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
    if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_140009e00",
            &[memory.read_u64(values[0])?, 0x1u64],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140009b50` (`mspaint.c:13287`).
pub fn fun_140009b50(
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
        ((memory.read_u64((values[0]).wrapping_add(0x10u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140009e00",
            &[memory.read_u64((values[0]).wrapping_add(0x10u64))?, 0x1u64],
            memory,
        );
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140009b70` (`mspaint.c:13299`).
pub fn fun_140009b70(
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
    values[2] = (invoke(
        "__std_type_info_compare",
        &[(values[1]).wrapping_add(0x8u64), 0x1401796a0u64],
        memory,
    )) as i32 as i64 as u64;
    values[3] = 0x0u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[3] = (values[0]).wrapping_add(0x10u64);
    }
    Ok(Some(values[3]))
}

/// Translation of `FUN_14000a090` (`mspaint.c:13590`).
pub fn fun_14000a090(
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
        ((memory.read_u64((values[0]).wrapping_add((0x3u64).wrapping_mul(8)))?) as i64)
            != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_14000a0f0", &[], memory);
    }
    if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[values[0]], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14000a8a0` (`mspaint.c:14093`).
pub fn fun_14000a8a0(
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
    memory.write_u64(values[0], relocate("Gdiplus::Image::vftable"))?;
    invoke(
        "GdipDisposeImage",
        &[memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?],
        memory,
    );
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        if u64::from(((values[1]) & (0x4u64)) == (0x0u64)) != 0 {
            invoke("GdipFree", &[], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000a950` (`mspaint.c:14136`).
pub fn fun_14000a950(
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
    memory.write_u64(values[0], 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    invoke("__ExceptionPtrCopy", &[values[0], values[1]], memory);
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000a980` (`mspaint.c:14147`).
pub fn fun_14000a980(
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
    memory.write_u64(values[0], 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    invoke("__ExceptionPtrCreate", &[values[0]], memory);
    invoke("__ExceptionPtrCurrentException", &[values[0]], memory);
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000a9b0` (`mspaint.c:14159`).
pub fn fun_14000a9b0(
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
    invoke("__ExceptionPtrAssign", &[values[0], values[1]], memory);
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000aaa0` (`mspaint.c:14218`).
pub fn fun_14000aaa0(
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
    invoke("FUN_14000ab00", &[], memory);
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        if u64::from(((values[1]) & (0x4u64)) == (0x0u64)) != 0 {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000ab50` (`mspaint.c:14250`).
pub fn fun_14000ab50(
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
    memory.write_u64(values[0], relocate("CBitmap::vftable"))?;
    invoke("FUN_14000ab00", &[], memory);
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        if u64::from(((values[1]) & (0x4u64)) == (0x0u64)) != 0 {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000b430` (`mspaint.c:14751`).
pub fn fun_14000b430(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_14000b750", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14000b450` (`mspaint.c:14766`).
pub fn fun_14000b450(
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
    values[1] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x48u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000b4f0` (`mspaint.c:14805`).
pub fn fun_14000b4f0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_14000b750", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14000b9b0` (`mspaint.c:15097`).
pub fn fun_14000b9b0(
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
    if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
        invoke("FUN_140001b30", &[], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000c040` (`mspaint.c:15474`).
pub fn fun_14000c040(
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
        "FUN_14001aa10",
        &[
            values[0],
            0x2u64,
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x8u64))?)) as u32),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14000d060` (`mspaint.c:16429`).
pub fn fun_14000d060(
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
    values[1] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x10u64;
    }
    invoke("FUN_14000d3f0", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14000d080` (`mspaint.c:16444`).
pub fn fun_14000d080(
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
    values[1] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x10u64;
    }
    invoke("FUN_14000d430", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14000f840` (`mspaint.c:18418`).
pub fn fun_14000f840(
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
        "WindowsDeleteString",
        &[memory.read_u64(values[0])?],
        memory,
    );
    memory.write_u64(values[0], 0x0u64)?;
    Ok(None)
}

/// Translation of `FUN_140011980` (`mspaint.c:20003`).
pub fn fun_140011980(
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
    memory.write_u32((values[0]).wrapping_add(0xcu64), 0xc0000001u64 as u32)?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400119b0` (`mspaint.c:20015`).
pub fn fun_1400119b0(
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
        "WindowsDeleteString",
        &[memory.read_u64((values[0]).wrapping_add(0x18u64))?],
        memory,
    );
    memory.write_u64((values[0]).wrapping_add(0x18u64), 0x0u64)?;
    memory.write_u32((values[0]).wrapping_add(0xcu64), 0xc0000001u64 as u32)?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x20u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140011a80` (`mspaint.c:20059`).
pub fn fun_140011a80(
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
    memory.write_u32((values[0]).wrapping_add(0xcu64), 0xc0000001u64 as u32)?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140011d80` (`mspaint.c:20249`).
pub fn fun_140011d80(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
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
        u64::from((arguments[1]) as u32),
        arguments[2],
        arguments[3],
        arguments[4],
        arguments[5],
        0u64,
    ];
    values[6] = u64::from((invoke("GetLastError", &[], memory)) as u32);
    if u64::from((u64::from((values[6]) as u32)) == (u64::from((0x0u64) as u32))) != 0 {
        invoke(
            "FUN_1400123f0",
            &[
                values[0],
                values[1],
                values[2],
                0x0u64,
                0x0u64,
                values[5],
                0x8007029cu64,
            ],
            memory,
        );
        values[6] = u64::from((0x29cu64) as u32);
    }
    Ok(Some(u64::from((values[6]) as u32)))
}

/// Translation of `FUN_140011df0` (`mspaint.c:20265`).
pub fn fun_140011df0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
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
        u64::from((arguments[1]) as u32),
        arguments[2],
        arguments[3],
        arguments[4],
        arguments[5],
        0u64,
    ];
    values[6] = u64::from((invoke("GetLastError", &[], memory)) as u32);
    if u64::from((u64::from((values[6]) as u32)) == (u64::from((0x0u64) as u32))) != 0 {
        invoke(
            "FUN_1400123f0",
            &[
                values[0],
                values[1],
                values[2],
                0x0u64,
                0x0u64,
                values[5],
                0x8007029cu64,
            ],
            memory,
        );
        values[6] = u64::from((0x29cu64) as u32);
    }
    if u64::from(
        (((0x0u64) as i32 as i64 as u64) as i64)
            < ((((values[6]) as i32 as i64 as u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[6] = u64::from(
            (u64::from(((u64::from(((values[6]) & (0xffffu64)) as u32)) | (0x80070000u64)) as u32))
                as u32,
        );
    }
    Ok(Some(u64::from((values[6]) as u32)))
}

/// Translation of `FUN_140011e60` (`mspaint.c:20284`).
pub fn fun_140011e60(
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
    ];
    values[3] = u64::from((invoke("FUN_140011df0", &[], memory)) as u32);
    invoke("FUN_140012f40", &[values[3]], memory);
    invoke(
        "thunk_FUN_140011f80",
        &[values[0], values[1], values[2]],
        memory,
    );
    Ok(Some(u64::from((values[3]) as u32)))
}

/// Translation of `FUN_140012230` (`mspaint.c:20607`).
pub fn fun_140012230(
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
    let mut values = [arguments[0], arguments[1], arguments[2]];
    values[1] &= 0xffffffffu64;
    invoke("FUN_140012f40", &[], memory);
    invoke(
        "thunk_FUN_140011f80",
        &[values[0], (values[1]) & (0xffffffffu64), values[2]],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400123e0` (`mspaint.c:20724`).
pub fn fun_1400123e0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_140012300", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400180b0` (`mspaint.c:25417`).
pub fn fun_1400180b0(
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
        "FUN_140018130",
        &[(values[0]).wrapping_add(0x18u64)],
        memory,
    );
    invoke("FUN_1400181a0", &[(values[0]).wrapping_add(0x8u64)], memory);
    Ok(None)
}

/// Translation of `FUN_140018100` (`mspaint.c:25443`).
pub fn fun_140018100(
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
    values[2] = (invoke(
        "__std_type_info_compare",
        &[(values[1]).wrapping_add(0x8u64), 0x1401799c8u64],
        memory,
    )) as i32 as i64 as u64;
    values[3] = 0x0u64;
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[3] = (values[0]).wrapping_add(0x10u64);
    }
    Ok(Some(values[3]))
}

/// Translation of `FUN_1400186f0` (`mspaint.c:25860`).
pub fn fun_1400186f0(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::bad_optional_access::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140018a40` (`mspaint.c:26046`).
pub fn fun_140018a40(
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
    let values = [arguments[0]];
    memory.write_u32(values[0], 0x16u64 as u32)?;
    memory.write_u64(
        (values[0]).wrapping_add((0x2u64).wrapping_mul(4)),
        relocate("PTR_vftable_1401790b8"),
    )?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140018dd0` (`mspaint.c:26238`).
pub fn fun_140018dd0(
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
    ];
    if u64::from(
        (((values[2]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        memory.write_u32(values[1], 0x0u64 as u32)?;
        memory.write_u64(
            (values[1]).wrapping_add((0x2u64).wrapping_mul(4)),
            relocate("PTR_vftable_1401790b8"),
        )?;
        return Ok(Some(values[1]));
    }
    values[3] = (invoke("std::_Winerror_map", &[values[2]], memory)) as i32 as i64 as u64;
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        memory.write_u32(values[1], values[2] as u32)?;
        memory.write_u64(
            (values[1]).wrapping_add((0x2u64).wrapping_mul(4)),
            relocate("PTR_vftable_1401790c8"),
        )?;
        return Ok(Some(values[1]));
    }
    memory.write_u32(values[1], values[3] as u32)?;
    memory.write_u64(
        (values[1]).wrapping_add((0x2u64).wrapping_mul(4)),
        relocate("PTR_vftable_1401790b8"),
    )?;
    Ok(Some(values[1]))
}

/// Translation of `FUN_140018ed0` (`mspaint.c:26304`).
pub fn fun_140018ed0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.len() < 2 {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 2,
            available: arguments.len(),
        }));
    }
    let values = [arguments[0], u64::from((arguments[1]) as u32)];
    memory.write_u32(values[0], values[1] as u32)?;
    memory.write_u64(
        (values[0]).wrapping_add((0x2u64).wrapping_mul(4)),
        relocate("PTR_vftable_1401790c8"),
    )?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140019980` (`mspaint.c:26810`).
pub fn fun_140019980(
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
        "FUN_14000f600",
        &[(values[0]).wrapping_add((0xdu64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "FUN_1400054d0",
        &[(values[0]).wrapping_add((0x9u64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "FUN_1400054d0",
        &[(values[0]).wrapping_add((0x5u64).wrapping_mul(8))],
        memory,
    );
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    invoke(
        "__std_exception_destroy",
        &[(values[0]).wrapping_add((0x1u64).wrapping_mul(8))],
        memory,
    );
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x88u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140019c40` (`mspaint.c:26966`).
pub fn fun_140019c40(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    invoke(
        "__std_exception_destroy",
        &[(values[0]).wrapping_add((0x1u64).wrapping_mul(8))],
        memory,
    );
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x28u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140019c90` (`mspaint.c:26979`).
pub fn fun_140019c90(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::_System_error::vftable"))?;
    values[2] = memory.read_u64((values[1]).wrapping_add(0x18u64))?;
    values[3] = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
    memory.write_u64(values[0], relocate("std::system_error::vftable"))?;
    memory.write_u64(
        (values[0]).wrapping_add((0x3u64).wrapping_mul(8)),
        values[2],
    )?;
    memory.write_u64(
        (values[0]).wrapping_add((0x4u64).wrapping_mul(8)),
        values[3],
    )?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140019cf0` (`mspaint.c:27000`).
pub fn fun_140019cf0(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::_System_error::vftable"))?;
    values[2] = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
    memory.write_u64(
        (values[0]).wrapping_add((0x3u64).wrapping_mul(8)),
        memory.read_u64((values[1]).wrapping_add(0x18u64))?,
    )?;
    memory.write_u64(
        (values[0]).wrapping_add((0x4u64).wrapping_mul(8)),
        values[2],
    )?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140019d40` (`mspaint.c:27018`).
pub fn fun_140019d40(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::runtime_error::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001a920` (`mspaint.c:27492`).
pub fn fun_14001a920(
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
    memory.write_u64(values[0], relocate("Value::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001bc80` (`mspaint.c:28426`).
pub fn fun_14001bc80(
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
    memory.write_u64(values[0], relocate("Value::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001bcb0` (`mspaint.c:28438`).
pub fn fun_14001bcb0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x48u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bcd0` (`mspaint.c:28453`).
pub fn fun_14001bcd0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x48u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bcf0` (`mspaint.c:28468`).
pub fn fun_14001bcf0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x48u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bdc0` (`mspaint.c:28546`).
pub fn fun_14001bdc0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x48u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bec0` (`mspaint.c:28627`).
pub fn fun_14001bec0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x40u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bee0` (`mspaint.c:28642`).
pub fn fun_14001bee0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x40u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bf00` (`mspaint.c:28657`).
pub fn fun_14001bf00(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x40u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001bfd0` (`mspaint.c:28735`).
pub fn fun_14001bfd0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x40u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c1f0` (`mspaint.c:28877`).
pub fn fun_14001c1f0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c210` (`mspaint.c:28892`).
pub fn fun_14001c210(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c230` (`mspaint.c:28907`).
pub fn fun_14001c230(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c300` (`mspaint.c:28985`).
pub fn fun_14001c300(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x38u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c3e0` (`mspaint.c:29042`).
pub fn fun_14001c3e0(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_14001e3e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001c410` (`mspaint.c:29057`).
pub fn fun_14001c410(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_14001e460", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001c510` (`mspaint.c:29112`).
pub fn fun_14001c510(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_14001e640", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001c8c0` (`mspaint.c:29335`).
pub fn fun_14001c8c0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x30u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c8e0` (`mspaint.c:29350`).
pub fn fun_14001c8e0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x30u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c900` (`mspaint.c:29365`).
pub fn fun_14001c900(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x30u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c9d0` (`mspaint.c:29443`).
pub fn fun_14001c9d0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x30u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001c9f0` (`mspaint.c:29458`).
pub fn fun_14001c9f0(
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
    values[1] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x58u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001cb10` (`mspaint.c:29517`).
pub fn fun_14001cb10(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x28u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004150", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001cb30` (`mspaint.c:29532`).
pub fn fun_14001cb30(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x28u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140004180", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001cb50` (`mspaint.c:29547`).
pub fn fun_14001cb50(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x28u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003b60", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001cc20` (`mspaint.c:29625`).
pub fn fun_14001cc20(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x28u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140003c00", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_14001d1f0` (`mspaint.c:30022`).
pub fn fun_14001d1f0(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_14001e3e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d220` (`mspaint.c:30037`).
pub fn fun_14001d220(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_14001e460", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d2a0` (`mspaint.c:30074`).
pub fn fun_14001d2a0(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_14001e640", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001d5c0` (`mspaint.c:30277`).
pub fn fun_14001d5c0(
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
    values[1] = (values[0]).wrapping_add(0x48u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x58u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400241a0` (`mspaint.c:35524`).
pub fn fun_1400241a0(
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
    values[1] = u64::from((invoke("FUN_1400241d0", &[0x0u64], memory)) as u32);
    memory.write_u32((values[0]).wrapping_add(0x18u64), values[1] as u32)?;
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140025090` (`mspaint.c:36383`).
pub fn fun_140025090(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x20u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140031b90` (`mspaint.c:46335`).
pub fn fun_140031b90(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::bad_cast::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140032020` (`mspaint.c:46559`).
pub fn fun_140032020(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("std::format_error::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140036bc0` (`mspaint.c:50761`).
pub fn fun_140036bc0(
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
    values[1] = (values[0]).wrapping_add(0x50u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x70u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140036c60` (`mspaint.c:50800`).
pub fn fun_140036c60(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140035440", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140038a80` (`mspaint.c:52672`).
pub fn fun_140038a80(
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
        "Concurrency::details::_TaskEventLogger::_LogWorkItemCompleted",
        &[memory.read_u64(values[0])?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140038ec0` (`mspaint.c:52878`).
pub fn fun_140038ec0(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x1c8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003a010` (`mspaint.c:53963`).
pub fn fun_14003a010(
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
        "FUN_14003a4f0",
        &[
            memory.read_u64((values[0]).wrapping_add(0x10u64))?,
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            0xffffffffu64,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003a330` (`mspaint.c:54166`).
pub fn fun_14003a330(
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
    invoke("Concurrency::details::_Release_chore", &[values[0]], memory);
    invoke("FUN_1400b022c", &[values[0], 0x28u64], memory);
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003a360` (`mspaint.c:54176`).
pub fn fun_14003a360(
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
        invoke("Concurrency::details::_Release_chore", &[values[1]], memory);
        invoke("FUN_1400b022c", &[values[1], 0x28u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14003a480` (`mspaint.c:54241`).
pub fn fun_14003a480(
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
    memory.write_u64(
        values[0],
        relocate("Concurrency::details::_TaskProcHandle::vftable"),
    )?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003a620` (`mspaint.c:54327`).
pub fn fun_14003a620(
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
    memory.write_u64(
        values[0],
        relocate("Concurrency::details::_ContinuationTaskHandleBase::vftable"),
    )?;
    invoke(
        "Concurrency::details::_ContextCallback::_Reset",
        &[(values[0]).wrapping_add((0x2u64).wrapping_mul(8))],
        memory,
    );
    memory.write_u64(
        values[0],
        relocate("Concurrency::details::_TaskProcHandle::vftable"),
    )?;
    if u64::from(
        (u64::from((u64::from(((values[1]) & (0x1u64)) as u32)) as u32))
            != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        invoke("FUN_1400b022c", &[values[0], 0x28u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003ae00` (`mspaint.c:54789`).
pub fn fun_14003ae00(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(values[0], relocate("Concurrency::task_canceled::vftable"))?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003aeb0` (`mspaint.c:54842`).
pub fn fun_14003aeb0(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x48u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003b290` (`mspaint.c:55069`).
pub fn fun_14003b290(
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
    memory.write_u64(values[0], relocate("std::exception::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    memory.write_u64(
        values[0],
        relocate("Concurrency::invalid_operation::vftable"),
    )?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003b390` (`mspaint.c:55143`).
pub fn fun_14003b390(
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
    invoke(
        "Concurrency::task_continuation_context::task_continuation_context",
        &[values[0]],
        memory,
    );
    if u64::from(((memory.read_u64(values[0])?) as i64) == ((0x1u64) as i64)) != 0 {
        memory.write_u64(values[0], 0x0u64)?;
        values[1] = u64::from(
            (invoke(
                "Concurrency::details::_ContextCallback::_IsCurrentOriginSTA",
                &[],
                memory,
            )) != 0,
        );
        if values[1] != 0 {
            invoke(
                "Concurrency::details::_ContextCallback::_Capture",
                &[values[0]],
                memory,
            );
        }
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003b4b0` (`mspaint.c:55197`).
pub fn fun_14003b4b0(
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
        "Concurrency::details::_ContextCallback::_Reset",
        &[values[0]],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003b4c0` (`mspaint.c:55206`).
pub fn fun_14003b4c0(
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
        "Concurrency::details::_ContextCallback::_Reset",
        &[values[0]],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003b790` (`mspaint.c:55403`).
pub fn fun_14003b790(
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
    memory.write_u64(
        values[0],
        relocate("Concurrency::details::_RefCounter::vftable"),
    )?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003be50` (`mspaint.c:55856`).
pub fn fun_14003be50(
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
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140004100", &[], memory);
    }
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003c0a0` (`mspaint.c:55988`).
pub fn fun_14003c0a0(
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
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140004100", &[], memory);
    }
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003c340` (`mspaint.c:56169`).
pub fn fun_14003c340(
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
        "FUN_140035440",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            u64::from((u64::from(memory.read_u8((values[0]).wrapping_add(0x10u64))?)) as u8),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003c350` (`mspaint.c:56178`).
pub fn fun_14003c350(
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
        "FUN_1400029b0",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            0x0u64,
            u64::from((u64::from(memory.read_u8((values[0]).wrapping_add(0x10u64))?)) as u8),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003c360` (`mspaint.c:56187`).
pub fn fun_14003c360(
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
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140004100", &[], memory);
    }
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x20u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003c3a0` (`mspaint.c:56201`).
pub fn fun_14003c3a0(
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
        "FUN_14003f260",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32),
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x18u64))?)) as u32),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003c430` (`mspaint.c:56261`).
pub fn fun_14003c430(
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
        ((memory.read_u64((values[0]).wrapping_add(0x20u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140004100", &[(values[0]).wrapping_add(0x8u64)], memory);
    }
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x28u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14003c720` (`mspaint.c:56434`).
pub fn fun_14003c720(
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
        "FUN_14003f6d0",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32),
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x18u64))?)) as u32),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003c780` (`mspaint.c:56461`).
pub fn fun_14003c780(
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
        "FUN_1400368f0",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            (values[0]).wrapping_add(0x10u64),
            memory.read_u64((values[0]).wrapping_add(0x20u64))?,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14003c7c0` (`mspaint.c:56479`).
pub fn fun_14003c7c0(
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
        "FUN_1400356d0",
        &[
            memory.read_u64((values[0]).wrapping_add(0x8u64))?,
            (values[0]).wrapping_add(0x10u64),
            0x0u64,
            0x7u64,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140041580` (`mspaint.c:60741`).
pub fn fun_140041580(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400420a0` (`mspaint.c:61396`).
pub fn fun_1400420a0(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_140043740", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400420d0` (`mspaint.c:61411`).
pub fn fun_1400420d0(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_1400437c0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400421d0` (`mspaint.c:61466`).
pub fn fun_1400421d0(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_1400439a0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140042690` (`mspaint.c:61754`).
pub fn fun_140042690(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_140043740", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400426c0` (`mspaint.c:61769`).
pub fn fun_1400426c0(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_1400437c0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140042740` (`mspaint.c:61806`).
pub fn fun_140042740(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_1400439a0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140043700` (`mspaint.c:62786`).
pub fn fun_140043700(
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
        "FUN_1400356d0",
        &[
            values[1],
            (values[0]).wrapping_add(0x8u64),
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x10u64))?)) as u32),
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0x14u64))?)) as u32),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140047820` (`mspaint.c:66073`).
pub fn fun_140047820(
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
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        if u64::from(((values[1]) & (0x4u64)) == (0x0u64)) != 0 {
            invoke("CNoTrackObject::operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400480a0` (`mspaint.c:66556`).
pub fn fun_1400480a0(
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
        ((memory.read_u64((values[0]).wrapping_add(0x18u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_1400041f0", &[], memory);
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
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140048350` (`mspaint.c:66703`).
pub fn fun_140048350(
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
    values[1] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x48u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004a6e0` (`mspaint.c:68440`).
pub fn fun_14004a6e0(
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
    if u64::from((values[0]) != (values[1])) != 0 {
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[], memory);
        }
        values[2] = memory.read_u64(values[1])?;
        memory.write_u64(values[1], 0x0u64)?;
        memory.write_u64(values[0], values[2])?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14004e1c0` (`mspaint.c:71678`).
pub fn fun_14004e1c0(
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
        ((memory.read_u64((values[0]).wrapping_add(0x48u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke(
        "FUN_140023220",
        &[(values[0]).wrapping_add(0x30u64)],
        memory,
    );
    invoke(
        "FUN_140023220",
        &[(values[0]).wrapping_add(0x18u64)],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_14004f0d0` (`mspaint.c:72480`).
pub fn fun_14004f0d0(
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
        ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke("FUN_14004eff0", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_14004f100` (`mspaint.c:72492`).
pub fn fun_14004f100(
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
        ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke("FUN_14004eff0", &[values[0]], memory);
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x70u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140051bb0` (`mspaint.c:74330`).
pub fn fun_140051bb0(
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
        "CAttributeStore::Get",
        &[relocate("DAT_1401d0b30"), (values[1]) as i32 as i64 as u64],
        memory,
    );
    Ok(Some(values[1]))
}

/// Translation of `FUN_140053060` (`mspaint.c:75485`).
pub fn fun_140053060(
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
        "FUN_1400034e0",
        &[
            (values[0]).wrapping_add(0x88u64),
            memory.read_u64(values[1])?,
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400564e0` (`mspaint.c:78652`).
pub fn fun_1400564e0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140059a40", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140056800` (`mspaint.c:78870`).
pub fn fun_140056800(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_140059a40", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_1400574f0` (`mspaint.c:79676`).
pub fn fun_1400574f0(
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
        ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke("FUN_140008c50", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_140057520` (`mspaint.c:79688`).
pub fn fun_140057520(
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
        ((memory.read_u64((values[0]).wrapping_add(0x68u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke("FUN_140008c50", &[values[0]], memory);
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x70u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400597d0` (`mspaint.c:81647`).
pub fn fun_1400597d0(
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
        "FUN_14000a2e0",
        &[memory.read_u64((values[0]).wrapping_add(0xc0u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140059cb0` (`mspaint.c:81935`).
pub fn fun_140059cb0(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x98u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140059ce0` (`mspaint.c:81947`).
pub fn fun_140059ce0(
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
    memory.write_u64(
        values[0],
        relocate("Concurrency::details::_RefCounter::vftable"),
    )?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0xc8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14005b050` (`mspaint.c:83030`).
pub fn fun_14005b050(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x1c8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140062380` (`mspaint.c:88605`).
pub fn fun_140062380(
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
    values[1] = invoke("FUN_140013610", &[], memory);
    invoke("FUN_1400622c0", &[values[0], values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140062400` (`mspaint.c:88643`).
pub fn fun_140062400(
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
    values[1] = invoke("FUN_140013530", &[], memory);
    invoke("FUN_1400622c0", &[values[0], values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140062520` (`mspaint.c:88722`).
pub fn fun_140062520(
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
    values[1] = invoke("FUN_1400135a0", &[], memory);
    invoke("FUN_1400622c0", &[values[0], values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_1400625a0` (`mspaint.c:88760`).
pub fn fun_1400625a0(
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
    values[1] = invoke("FUN_140013680", &[], memory);
    invoke("FUN_1400622c0", &[values[0], values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140063eb0` (`mspaint.c:90145`).
pub fn fun_140063eb0(
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
        "FUN_140065370",
        &[values[0], memory.read_u64(values[0])?],
        memory,
    );
    invoke(
        "FUN_1400b022c",
        &[memory.read_u64(values[0])?, 0x18u64],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140065960` (`mspaint.c:91797`).
pub fn fun_140065960(
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
        "FUN_14006f720",
        &[relocate("DAT_1401d0520"), memory.read_u64(values[1])?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140066240` (`mspaint.c:92330`).
pub fn fun_140066240(
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
    memory.write_u64(values[0], relocate("ATL::IDocument::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x10u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140066ef0` (`mspaint.c:93083`).
pub fn fun_140066ef0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("AfxTryCleanup", &[], memory);
    Ok(None)
}

/// Translation of `FUN_140069e70` (`mspaint.c:95384`).
pub fn fun_140069e70(
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
        "PostThreadMessageW",
        &[
            u64::from((u64::from(memory.read_u32((values[0]).wrapping_add(0xcu64))?)) as u32),
            0x12u64,
            0x0u64,
            0x0u64,
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14006b2e0` (`mspaint.c:96432`).
pub fn fun_14006b2e0(
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
    invoke("FUN_14006b4f0", &[], memory);
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        if u64::from(((values[1]) & (0x4u64)) == (0x0u64)) != 0 {
            invoke("operator_delete", &[values[0]], memory);
            return Ok(Some(values[0]));
        }
        invoke("_guard_check_icall", &[values[0], 0x708u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14006b4d0` (`mspaint.c:96512`).
pub fn fun_14006b4d0(
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
        "FUN_140018130",
        &[(values[0]).wrapping_add(0x18u64)],
        memory,
    );
    invoke("FUN_140078840", &[(values[0]).wrapping_add(0x8u64)], memory);
    Ok(None)
}

/// Translation of `FUN_140078340` (`mspaint.c:105765`).
pub fn fun_140078340(
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
    memory.write_u64(values[0], relocate("CAttributeValue<>::vftable"))?;
    invoke(
        "FUN_1400054d0",
        &[(values[0]).wrapping_add((0x2u64).wrapping_mul(8))],
        memory,
    );
    memory.write_u64(values[0], relocate("Value::vftable"))?;
    Ok(None)
}

/// Translation of `FUN_140078660` (`mspaint.c:105928`).
pub fn fun_140078660(
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
    if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_1400054d0",
            &[(values[1]).wrapping_add(0x20u64)],
            memory,
        );
        invoke("FUN_1400054d0", &[values[1]], memory);
        invoke("FUN_1400b022c", &[values[1], 0x40u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_140078f60` (`mspaint.c:106489`).
pub fn fun_140078f60(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x28u64;
    }
    invoke("FUN_14007af70", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140079020` (`mspaint.c:106546`).
pub fn fun_140079020(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x28u64;
    }
    invoke("FUN_14007b110", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140079630` (`mspaint.c:106910`).
pub fn fun_140079630(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x78u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14007a8f0` (`mspaint.c:107870`).
pub fn fun_14007a8f0(
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
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_1400b022c", &[values[0], 0x10u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_14007c060` (`mspaint.c:109199`).
pub fn fun_14007c060(
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
        ((memory.read_u64((values[0]).wrapping_add(0x10u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from(
        (((values[1]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140085310` (`mspaint.c:116481`).
pub fn fun_140085310(
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
        ((((u64::from(memory.read_u8((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        memory.write_u8(
            (values[0]).wrapping_add((0x1u64).wrapping_mul(8)),
            0x0u64 as u8,
        )?;
        invoke(
            "CCmdTarget::EndWaitCursor",
            &[memory.read_u64(values[0])?],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_14008ff50` (`mspaint.c:124556`).
pub fn fun_14008ff50(
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
    memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64(values[0], relocate("wil::ResultException::vftable"))?;
    memory.write_u64((values[0]).wrapping_add((0x16u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x17u64).wrapping_mul(8)), 0x0u64)?;
    invoke("FUN_14008f730", &[], memory);
    memory.write_u64((values[0]).wrapping_add((0x18u64).wrapping_mul(8)), 0x0u64)?;
    memory.write_u64((values[0]).wrapping_add((0x19u64).wrapping_mul(8)), 0x0u64)?;
    Ok(Some(values[0]))
}

/// Translation of `FUN_140092e80` (`mspaint.c:126876`).
pub fn fun_140092e80(
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
    values[1] = u64::from(
        (invoke(
            "FUN_1400a3d60",
            &[memory.read_u64((values[0]).wrapping_add(0x1e0u64))?],
            memory,
        )) as u8,
    );
    invoke(
        "FUN_14004cad0",
        &[
            memory.read_u64((values[0]).wrapping_add(0x40u64))?,
            values[1],
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140094910` (`mspaint.c:128209`).
pub fn fun_140094910(
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
    invoke("FUN_140094a60", &[], memory);
    values[1] = memory.read_u64(values[0])?;
    memory.write_u64(values[0], 0x0u64)?;
    invoke("FUN_1400b022c", &[values[1], 0x10u64], memory);
    Ok(None)
}

/// Translation of `FUN_140096010` (`mspaint.c:129438`).
pub fn fun_140096010(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_140097490", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096030` (`mspaint.c:129453`).
pub fn fun_140096030(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_1400974d0", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096050` (`mspaint.c:129468`).
pub fn fun_140096050(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x20u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_140097300", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096210` (`mspaint.c:129621`).
pub fn fun_140096210(
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
    values[1] = (values[0]).wrapping_add(0x58u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x80u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400963e0` (`mspaint.c:129720`).
pub fn fun_1400963e0(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_140097490", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096400` (`mspaint.c:129735`).
pub fn fun_140096400(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_1400974d0", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096420` (`mspaint.c:129750`).
pub fn fun_140096420(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x18u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_140097300", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096ca0` (`mspaint.c:130284`).
pub fn fun_140096ca0(
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
    values[1] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x40u64;
    }
    invoke("FUN_140034000", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140096d10` (`mspaint.c:130318`).
pub fn fun_140096d10(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_140097490", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096d30` (`mspaint.c:130333`).
pub fn fun_140096d30(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_1400974d0", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140096d50` (`mspaint.c:130348`).
pub fn fun_140096d50(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x8u64;
    }
    invoke("FUN_140097300", &[values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140097f50` (`mspaint.c:131418`).
pub fn fun_140097f50(
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
    values[1] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x40u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099640` (`mspaint.c:132555`).
pub fn fun_140099640(
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
    values[1] = (values[0]).wrapping_add(0x40u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x50u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400996d0` (`mspaint.c:132595`).
pub fn fun_1400996d0(
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
    values[1] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x38u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5030` (`mspaint.c:141490`).
pub fn fun_1400a5030(
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
    if u64::from(((values[1]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_140105708");
    } else {
        values[2] = memory.read_u64((values[1]).wrapping_add(0x10u64))?;
    }
    invoke(
        "FUN_14006f720",
        &[relocate("DAT_1401d0520"), values[2]],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5550` (`mspaint.c:141816`).
pub fn fun_1400a5550(
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
    values[1] = (values[0]).wrapping_add(0xa8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0xb8u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a58f0` (`mspaint.c:142019`).
pub fn fun_1400a58f0(
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
    values[3] = u64::from((invoke("FUN_14009d7e0", &[], memory)) as u8);
    memory.write_u8(values[2], values[3] as u8)?;
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5910` (`mspaint.c:142031`).
pub fn fun_1400a5910(
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
    values[1] = (values[0]).wrapping_add(0x90u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0xa0u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5b30` (`mspaint.c:142157`).
pub fn fun_1400a5b30(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_14009d6a0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5c90` (`mspaint.c:142252`).
pub fn fun_1400a5c90(
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
    values[1] = (values[0]).wrapping_add(0x60u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x70u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5e60` (`mspaint.c:142351`).
pub fn fun_1400a5e60(
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
    values[1] = (values[0]).wrapping_add(0x78u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x88u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6c70` (`mspaint.c:143108`).
pub fn fun_1400a6c70(
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
    values[1] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x0u64;
    }
    invoke("FUN_14009c470", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6d80` (`mspaint.c:143171`).
pub fn fun_1400a6d80(
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
    let mut values = [arguments[0], arguments[1], arguments[2], 0u64, 0u64];
    values[4] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[4] = 0x0u64;
    }
    values[3] = u64::from((invoke("FUN_14009c220", &[values[4]], memory)) as u8);
    memory.write_u8(values[2], values[3] as u8)?;
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6db0` (`mspaint.c:143188`).
pub fn fun_1400a6db0(
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
    values[1] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[1] = 0x28u64;
    }
    invoke("FUN_1400034e0", &[values[1]], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a8bc0` (`mspaint.c:145029`).
pub fn fun_1400a8bc0(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x188u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400a98e0` (`mspaint.c:145803`).
pub fn fun_1400a98e0(
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
        "FUN_14009e200",
        &[memory.read_u64((values[0]).wrapping_add(0x8u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400a9a40` (`mspaint.c:145945`).
pub fn fun_1400a9a40(
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
        "FUN_14009e140",
        &[memory.read_u64((values[0]).wrapping_add(0x8u64))?, 0x1u64],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400a9a80` (`mspaint.c:145973`).
pub fn fun_1400a9a80(
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
        "FUN_14009e140",
        &[memory.read_u64((values[0]).wrapping_add(0x8u64))?, 0x0u64],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400aa4e0` (`mspaint.c:146562`).
pub fn fun_1400aa4e0(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0xa0u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400aa510` (`mspaint.c:146574`).
pub fn fun_1400aa510(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x98u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400aa870` (`mspaint.c:146754`).
pub fn fun_1400aa870(
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
        "FUN_14009e090",
        &[memory.read_u64((values[0]).wrapping_add(0x8u64))?],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400ab570` (`mspaint.c:147485`).
pub fn fun_1400ab570(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x1d0u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400ab5a0` (`mspaint.c:147497`).
pub fn fun_1400ab5a0(
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
    memory.write_u64(values[0], relocate("std::_Ref_count_obj2<>::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x1c8u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400b0020` (`mspaint.c:154591`).
pub fn fun_1400b0020(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    invoke("FUN_1400b0af0", &[], memory);
    values[0] = u64::from((invoke("FUN_140002e40", &[], memory)) as u32);
    invoke("_set_new_mode", &[values[0]], memory);
    Ok(None)
}

/// Translation of `FUN_1400b0200` (`mspaint.c:154696`).
pub fn fun_1400b0200(
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
    memory.write_u64(values[0], relocate("type_info::vftable"))?;
    if u64::from(((values[1]) & (0x1u64)) != (0x0u64)) != 0 {
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400b0e80` (`mspaint.c:155555`).
pub fn fun_1400b0e80(
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
        "FUN_1400b0ea0",
        &[
            values[1],
            values[3],
            memory.read_u64((values[3]).wrapping_add(0x38u64))?,
        ],
        memory,
    );
    Ok(Some(0x1u64))
}

/// Translation of `FUN_1400b1d80` (`mspaint.c:157833`).
pub fn fun_1400b1d80(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x98u64),
        (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b1d90", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b2180` (`mspaint.c:158023`).
pub fn fun_1400b2180(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x20u64),
        (memory.read_u64((values[0]).wrapping_add(0x20u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b2190", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b2240` (`mspaint.c:158076`).
pub fn fun_1400b2240(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x7cu64),
        (memory.read_u64((values[0]).wrapping_add(0x7cu64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b2250", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b2740` (`mspaint.c:158306`).
pub fn fun_1400b2740(
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
    memory.write_u64(
        (values[0]).wrapping_add(0xc8u64),
        (memory.read_u64((values[0]).wrapping_add(0xc8u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b2750", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b2bf0` (`mspaint.c:158515`).
pub fn fun_1400b2bf0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0xa0u64),
        (memory.read_u64((values[0]).wrapping_add(0xa0u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b2c00", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b3d90` (`mspaint.c:159214`).
pub fn fun_1400b3d90(
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
    memory.write_u64(
        (values[0]).wrapping_add(0xb8u64),
        (memory.read_u64((values[0]).wrapping_add(0xb8u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b3da0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b41d0` (`mspaint.c:159374`).
pub fn fun_1400b41d0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x88u64),
        (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b41e0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b4850` (`mspaint.c:159607`).
pub fn fun_1400b4850(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x7cu64),
        (memory.read_u64((values[0]).wrapping_add(0x7cu64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b4860", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b4ee0` (`mspaint.c:159916`).
pub fn fun_1400b4ee0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x7cu64),
        (memory.read_u64((values[0]).wrapping_add(0x7cu64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b4ef0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b51b0` (`mspaint.c:160052`).
pub fn fun_1400b51b0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x7cu64),
        (memory.read_u64((values[0]).wrapping_add(0x7cu64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b51c0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b5850` (`mspaint.c:160344`).
pub fn fun_1400b5850(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x7cu64),
        (memory.read_u64((values[0]).wrapping_add(0x7cu64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b5860", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b5ba0` (`mspaint.c:160503`).
pub fn fun_1400b5ba0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x7cu64),
        (memory.read_u64((values[0]).wrapping_add(0x7cu64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b5bb0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b5eb0` (`mspaint.c:160653`).
pub fn fun_1400b5eb0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x90u64),
        (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b5ec0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b64b0` (`mspaint.c:160925`).
pub fn fun_1400b64b0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x14u64),
        (memory.read_u64((values[0]).wrapping_add(0x14u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b64c0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b67b0` (`mspaint.c:161080`).
pub fn fun_1400b67b0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x20u64),
        (memory.read_u64((values[0]).wrapping_add(0x20u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b67c0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b6930` (`mspaint.c:161169`).
pub fn fun_1400b6930(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x98u64),
        (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b6940", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b6f10` (`mspaint.c:161500`).
pub fn fun_1400b6f10(
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
    memory.write_u64(
        (values[0]).wrapping_add(0xd8u64),
        (memory.read_u64((values[0]).wrapping_add(0xd8u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b6f20", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b7900` (`mspaint.c:161961`).
pub fn fun_1400b7900(
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
    memory.write_u64(
        (values[0]).wrapping_add(0xd8u64),
        (memory.read_u64((values[0]).wrapping_add(0xd8u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b7910", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b7ea0` (`mspaint.c:162222`).
pub fn fun_1400b7ea0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x88u64),
        (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b7eb0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b80e0` (`mspaint.c:162339`).
pub fn fun_1400b80e0(
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
    memory.write_u64(
        (values[0]).wrapping_add(0x88u64),
        (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64),
    )?;
    invoke("FUN_1400b80f0", &[], memory);
    Ok(None)
}

pub type Handler = fn(
    &[u64],
    &mut MemoryImage,
    &mut RelocateCall<'_>,
    &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError>;
pub struct Entry {
    pub name: &'static str,
    pub pointers: &'static [bool],
    pub handler: Handler,
}
pub const HANDLERS: &[Entry] = &[
    Entry {
        name: "FUN_140001ee0",
        pointers: &[true, false],
        handler: fun_140001ee0,
    },
    Entry {
        name: "FUN_140001f30",
        pointers: &[true, false],
        handler: fun_140001f30,
    },
    Entry {
        name: "FUN_140001f70",
        pointers: &[true, false],
        handler: fun_140001f70,
    },
    Entry {
        name: "FUN_140001fe0",
        pointers: &[true, false],
        handler: fun_140001fe0,
    },
    Entry {
        name: "FUN_140002040",
        pointers: &[true, false],
        handler: fun_140002040,
    },
    Entry {
        name: "FUN_140002e60",
        pointers: &[false],
        handler: fun_140002e60,
    },
    Entry {
        name: "FUN_140002e80",
        pointers: &[false],
        handler: fun_140002e80,
    },
    Entry {
        name: "FUN_140002ea0",
        pointers: &[false],
        handler: fun_140002ea0,
    },
    Entry {
        name: "FUN_140002f70",
        pointers: &[false],
        handler: fun_140002f70,
    },
    Entry {
        name: "FUN_140002f90",
        pointers: &[false],
        handler: fun_140002f90,
    },
    Entry {
        name: "FUN_140003030",
        pointers: &[false],
        handler: fun_140003030,
    },
    Entry {
        name: "FUN_140003050",
        pointers: &[false],
        handler: fun_140003050,
    },
    Entry {
        name: "FUN_140003070",
        pointers: &[false],
        handler: fun_140003070,
    },
    Entry {
        name: "FUN_140003140",
        pointers: &[false],
        handler: fun_140003140,
    },
    Entry {
        name: "FUN_140003160",
        pointers: &[false, false],
        handler: fun_140003160,
    },
    Entry {
        name: "FUN_140003230",
        pointers: &[false],
        handler: fun_140003230,
    },
    Entry {
        name: "FUN_140003250",
        pointers: &[false],
        handler: fun_140003250,
    },
    Entry {
        name: "FUN_140003270",
        pointers: &[false],
        handler: fun_140003270,
    },
    Entry {
        name: "FUN_140003340",
        pointers: &[false],
        handler: fun_140003340,
    },
    Entry {
        name: "FUN_1400057c0",
        pointers: &[true, false],
        handler: fun_1400057c0,
    },
    Entry {
        name: "FUN_140009a20",
        pointers: &[true],
        handler: fun_140009a20,
    },
    Entry {
        name: "FUN_140009b50",
        pointers: &[true],
        handler: fun_140009b50,
    },
    Entry {
        name: "FUN_140009b70",
        pointers: &[false, false],
        handler: fun_140009b70,
    },
    Entry {
        name: "FUN_14000a090",
        pointers: &[true],
        handler: fun_14000a090,
    },
    Entry {
        name: "FUN_14000a8a0",
        pointers: &[true, false],
        handler: fun_14000a8a0,
    },
    Entry {
        name: "FUN_14000a950",
        pointers: &[true, true],
        handler: fun_14000a950,
    },
    Entry {
        name: "FUN_14000a980",
        pointers: &[true],
        handler: fun_14000a980,
    },
    Entry {
        name: "FUN_14000a9b0",
        pointers: &[true, true],
        handler: fun_14000a9b0,
    },
    Entry {
        name: "FUN_14000aaa0",
        pointers: &[true, false],
        handler: fun_14000aaa0,
    },
    Entry {
        name: "FUN_14000ab50",
        pointers: &[true, false],
        handler: fun_14000ab50,
    },
    Entry {
        name: "FUN_14000b430",
        pointers: &[false],
        handler: fun_14000b430,
    },
    Entry {
        name: "FUN_14000b450",
        pointers: &[false],
        handler: fun_14000b450,
    },
    Entry {
        name: "FUN_14000b4f0",
        pointers: &[false],
        handler: fun_14000b4f0,
    },
    Entry {
        name: "FUN_14000b9b0",
        pointers: &[true],
        handler: fun_14000b9b0,
    },
    Entry {
        name: "FUN_14000c040",
        pointers: &[true],
        handler: fun_14000c040,
    },
    Entry {
        name: "FUN_14000d060",
        pointers: &[false],
        handler: fun_14000d060,
    },
    Entry {
        name: "FUN_14000d080",
        pointers: &[false],
        handler: fun_14000d080,
    },
    Entry {
        name: "FUN_14000f840",
        pointers: &[true],
        handler: fun_14000f840,
    },
    Entry {
        name: "FUN_140011980",
        pointers: &[true, false],
        handler: fun_140011980,
    },
    Entry {
        name: "FUN_1400119b0",
        pointers: &[true, false],
        handler: fun_1400119b0,
    },
    Entry {
        name: "FUN_140011a80",
        pointers: &[true, false],
        handler: fun_140011a80,
    },
    Entry {
        name: "FUN_140011d80",
        pointers: &[false, false, false, false, false, false],
        handler: fun_140011d80,
    },
    Entry {
        name: "FUN_140011df0",
        pointers: &[false, false, false, false, false, false],
        handler: fun_140011df0,
    },
    Entry {
        name: "FUN_140011e60",
        pointers: &[false, false, false],
        handler: fun_140011e60,
    },
    Entry {
        name: "FUN_140012230",
        pointers: &[false, false, false],
        handler: fun_140012230,
    },
    Entry {
        name: "FUN_1400123e0",
        pointers: &[],
        handler: fun_1400123e0,
    },
    Entry {
        name: "FUN_1400180b0",
        pointers: &[false],
        handler: fun_1400180b0,
    },
    Entry {
        name: "FUN_140018100",
        pointers: &[false, false],
        handler: fun_140018100,
    },
    Entry {
        name: "FUN_1400186f0",
        pointers: &[true, false],
        handler: fun_1400186f0,
    },
    Entry {
        name: "FUN_140018a40",
        pointers: &[true],
        handler: fun_140018a40,
    },
    Entry {
        name: "FUN_140018dd0",
        pointers: &[false, true, false],
        handler: fun_140018dd0,
    },
    Entry {
        name: "FUN_140018ed0",
        pointers: &[true, false],
        handler: fun_140018ed0,
    },
    Entry {
        name: "FUN_140019980",
        pointers: &[true, false],
        handler: fun_140019980,
    },
    Entry {
        name: "FUN_140019c40",
        pointers: &[true, false],
        handler: fun_140019c40,
    },
    Entry {
        name: "FUN_140019c90",
        pointers: &[true, true],
        handler: fun_140019c90,
    },
    Entry {
        name: "FUN_140019cf0",
        pointers: &[true, true],
        handler: fun_140019cf0,
    },
    Entry {
        name: "FUN_140019d40",
        pointers: &[true, false],
        handler: fun_140019d40,
    },
    Entry {
        name: "FUN_14001a920",
        pointers: &[true, false],
        handler: fun_14001a920,
    },
    Entry {
        name: "FUN_14001bc80",
        pointers: &[true, false],
        handler: fun_14001bc80,
    },
    Entry {
        name: "FUN_14001bcb0",
        pointers: &[false],
        handler: fun_14001bcb0,
    },
    Entry {
        name: "FUN_14001bcd0",
        pointers: &[false],
        handler: fun_14001bcd0,
    },
    Entry {
        name: "FUN_14001bcf0",
        pointers: &[false],
        handler: fun_14001bcf0,
    },
    Entry {
        name: "FUN_14001bdc0",
        pointers: &[false],
        handler: fun_14001bdc0,
    },
    Entry {
        name: "FUN_14001bec0",
        pointers: &[false],
        handler: fun_14001bec0,
    },
    Entry {
        name: "FUN_14001bee0",
        pointers: &[false],
        handler: fun_14001bee0,
    },
    Entry {
        name: "FUN_14001bf00",
        pointers: &[false],
        handler: fun_14001bf00,
    },
    Entry {
        name: "FUN_14001bfd0",
        pointers: &[false],
        handler: fun_14001bfd0,
    },
    Entry {
        name: "FUN_14001c1f0",
        pointers: &[false],
        handler: fun_14001c1f0,
    },
    Entry {
        name: "FUN_14001c210",
        pointers: &[false],
        handler: fun_14001c210,
    },
    Entry {
        name: "FUN_14001c230",
        pointers: &[false],
        handler: fun_14001c230,
    },
    Entry {
        name: "FUN_14001c300",
        pointers: &[false],
        handler: fun_14001c300,
    },
    Entry {
        name: "FUN_14001c3e0",
        pointers: &[false],
        handler: fun_14001c3e0,
    },
    Entry {
        name: "FUN_14001c410",
        pointers: &[false],
        handler: fun_14001c410,
    },
    Entry {
        name: "FUN_14001c510",
        pointers: &[false],
        handler: fun_14001c510,
    },
    Entry {
        name: "FUN_14001c8c0",
        pointers: &[false],
        handler: fun_14001c8c0,
    },
    Entry {
        name: "FUN_14001c8e0",
        pointers: &[false],
        handler: fun_14001c8e0,
    },
    Entry {
        name: "FUN_14001c900",
        pointers: &[false],
        handler: fun_14001c900,
    },
    Entry {
        name: "FUN_14001c9d0",
        pointers: &[false],
        handler: fun_14001c9d0,
    },
    Entry {
        name: "FUN_14001c9f0",
        pointers: &[false],
        handler: fun_14001c9f0,
    },
    Entry {
        name: "FUN_14001cb10",
        pointers: &[false],
        handler: fun_14001cb10,
    },
    Entry {
        name: "FUN_14001cb30",
        pointers: &[false],
        handler: fun_14001cb30,
    },
    Entry {
        name: "FUN_14001cb50",
        pointers: &[false],
        handler: fun_14001cb50,
    },
    Entry {
        name: "FUN_14001cc20",
        pointers: &[false],
        handler: fun_14001cc20,
    },
    Entry {
        name: "FUN_14001d1f0",
        pointers: &[false],
        handler: fun_14001d1f0,
    },
    Entry {
        name: "FUN_14001d220",
        pointers: &[false],
        handler: fun_14001d220,
    },
    Entry {
        name: "FUN_14001d2a0",
        pointers: &[false],
        handler: fun_14001d2a0,
    },
    Entry {
        name: "FUN_14001d5c0",
        pointers: &[false],
        handler: fun_14001d5c0,
    },
    Entry {
        name: "FUN_1400241a0",
        pointers: &[true],
        handler: fun_1400241a0,
    },
    Entry {
        name: "FUN_140025090",
        pointers: &[true, false],
        handler: fun_140025090,
    },
    Entry {
        name: "FUN_140031b90",
        pointers: &[true, false],
        handler: fun_140031b90,
    },
    Entry {
        name: "FUN_140032020",
        pointers: &[true, false],
        handler: fun_140032020,
    },
    Entry {
        name: "FUN_140036bc0",
        pointers: &[false],
        handler: fun_140036bc0,
    },
    Entry {
        name: "FUN_140036c60",
        pointers: &[false],
        handler: fun_140036c60,
    },
    Entry {
        name: "FUN_140038a80",
        pointers: &[true],
        handler: fun_140038a80,
    },
    Entry {
        name: "FUN_140038ec0",
        pointers: &[true, false],
        handler: fun_140038ec0,
    },
    Entry {
        name: "FUN_14003a010",
        pointers: &[true],
        handler: fun_14003a010,
    },
    Entry {
        name: "FUN_14003a330",
        pointers: &[true],
        handler: fun_14003a330,
    },
    Entry {
        name: "FUN_14003a360",
        pointers: &[true],
        handler: fun_14003a360,
    },
    Entry {
        name: "FUN_14003a480",
        pointers: &[true, false],
        handler: fun_14003a480,
    },
    Entry {
        name: "FUN_14003a620",
        pointers: &[true, false],
        handler: fun_14003a620,
    },
    Entry {
        name: "FUN_14003ae00",
        pointers: &[true, false],
        handler: fun_14003ae00,
    },
    Entry {
        name: "FUN_14003aeb0",
        pointers: &[true, false],
        handler: fun_14003aeb0,
    },
    Entry {
        name: "FUN_14003b290",
        pointers: &[true, false],
        handler: fun_14003b290,
    },
    Entry {
        name: "FUN_14003b390",
        pointers: &[true],
        handler: fun_14003b390,
    },
    Entry {
        name: "FUN_14003b4b0",
        pointers: &[true],
        handler: fun_14003b4b0,
    },
    Entry {
        name: "FUN_14003b4c0",
        pointers: &[true],
        handler: fun_14003b4c0,
    },
    Entry {
        name: "FUN_14003b790",
        pointers: &[true, false],
        handler: fun_14003b790,
    },
    Entry {
        name: "FUN_14003be50",
        pointers: &[true, false],
        handler: fun_14003be50,
    },
    Entry {
        name: "FUN_14003c0a0",
        pointers: &[true, false],
        handler: fun_14003c0a0,
    },
    Entry {
        name: "FUN_14003c340",
        pointers: &[true],
        handler: fun_14003c340,
    },
    Entry {
        name: "FUN_14003c350",
        pointers: &[true],
        handler: fun_14003c350,
    },
    Entry {
        name: "FUN_14003c360",
        pointers: &[true, false],
        handler: fun_14003c360,
    },
    Entry {
        name: "FUN_14003c3a0",
        pointers: &[true],
        handler: fun_14003c3a0,
    },
    Entry {
        name: "FUN_14003c430",
        pointers: &[true, false],
        handler: fun_14003c430,
    },
    Entry {
        name: "FUN_14003c720",
        pointers: &[true],
        handler: fun_14003c720,
    },
    Entry {
        name: "FUN_14003c780",
        pointers: &[true],
        handler: fun_14003c780,
    },
    Entry {
        name: "FUN_14003c7c0",
        pointers: &[true],
        handler: fun_14003c7c0,
    },
    Entry {
        name: "FUN_140041580",
        pointers: &[false],
        handler: fun_140041580,
    },
    Entry {
        name: "FUN_1400420a0",
        pointers: &[false],
        handler: fun_1400420a0,
    },
    Entry {
        name: "FUN_1400420d0",
        pointers: &[false],
        handler: fun_1400420d0,
    },
    Entry {
        name: "FUN_1400421d0",
        pointers: &[false],
        handler: fun_1400421d0,
    },
    Entry {
        name: "FUN_140042690",
        pointers: &[false],
        handler: fun_140042690,
    },
    Entry {
        name: "FUN_1400426c0",
        pointers: &[false],
        handler: fun_1400426c0,
    },
    Entry {
        name: "FUN_140042740",
        pointers: &[false],
        handler: fun_140042740,
    },
    Entry {
        name: "FUN_140043700",
        pointers: &[true, false],
        handler: fun_140043700,
    },
    Entry {
        name: "FUN_140047820",
        pointers: &[true, false],
        handler: fun_140047820,
    },
    Entry {
        name: "FUN_1400480a0",
        pointers: &[true],
        handler: fun_1400480a0,
    },
    Entry {
        name: "FUN_140048350",
        pointers: &[false],
        handler: fun_140048350,
    },
    Entry {
        name: "FUN_14004a6e0",
        pointers: &[true, true],
        handler: fun_14004a6e0,
    },
    Entry {
        name: "FUN_14004e1c0",
        pointers: &[true],
        handler: fun_14004e1c0,
    },
    Entry {
        name: "FUN_14004f0d0",
        pointers: &[true],
        handler: fun_14004f0d0,
    },
    Entry {
        name: "FUN_14004f100",
        pointers: &[true, false],
        handler: fun_14004f100,
    },
    Entry {
        name: "FUN_140051bb0",
        pointers: &[false, false],
        handler: fun_140051bb0,
    },
    Entry {
        name: "FUN_140053060",
        pointers: &[false, true],
        handler: fun_140053060,
    },
    Entry {
        name: "FUN_1400564e0",
        pointers: &[false],
        handler: fun_1400564e0,
    },
    Entry {
        name: "FUN_140056800",
        pointers: &[false],
        handler: fun_140056800,
    },
    Entry {
        name: "FUN_1400574f0",
        pointers: &[true],
        handler: fun_1400574f0,
    },
    Entry {
        name: "FUN_140057520",
        pointers: &[true, false],
        handler: fun_140057520,
    },
    Entry {
        name: "FUN_1400597d0",
        pointers: &[true],
        handler: fun_1400597d0,
    },
    Entry {
        name: "FUN_140059cb0",
        pointers: &[true, false],
        handler: fun_140059cb0,
    },
    Entry {
        name: "FUN_140059ce0",
        pointers: &[true, false],
        handler: fun_140059ce0,
    },
    Entry {
        name: "FUN_14005b050",
        pointers: &[true, false],
        handler: fun_14005b050,
    },
    Entry {
        name: "FUN_140062380",
        pointers: &[false],
        handler: fun_140062380,
    },
    Entry {
        name: "FUN_140062400",
        pointers: &[false],
        handler: fun_140062400,
    },
    Entry {
        name: "FUN_140062520",
        pointers: &[false],
        handler: fun_140062520,
    },
    Entry {
        name: "FUN_1400625a0",
        pointers: &[false],
        handler: fun_1400625a0,
    },
    Entry {
        name: "FUN_140063eb0",
        pointers: &[true],
        handler: fun_140063eb0,
    },
    Entry {
        name: "FUN_140065960",
        pointers: &[false, true],
        handler: fun_140065960,
    },
    Entry {
        name: "FUN_140066240",
        pointers: &[true, false],
        handler: fun_140066240,
    },
    Entry {
        name: "FUN_140066ef0",
        pointers: &[],
        handler: fun_140066ef0,
    },
    Entry {
        name: "FUN_140069e70",
        pointers: &[true],
        handler: fun_140069e70,
    },
    Entry {
        name: "FUN_14006b2e0",
        pointers: &[true, false],
        handler: fun_14006b2e0,
    },
    Entry {
        name: "FUN_14006b4d0",
        pointers: &[false],
        handler: fun_14006b4d0,
    },
    Entry {
        name: "FUN_140078340",
        pointers: &[true],
        handler: fun_140078340,
    },
    Entry {
        name: "FUN_140078660",
        pointers: &[true],
        handler: fun_140078660,
    },
    Entry {
        name: "FUN_140078f60",
        pointers: &[false],
        handler: fun_140078f60,
    },
    Entry {
        name: "FUN_140079020",
        pointers: &[false],
        handler: fun_140079020,
    },
    Entry {
        name: "FUN_140079630",
        pointers: &[true, false],
        handler: fun_140079630,
    },
    Entry {
        name: "FUN_14007a8f0",
        pointers: &[true, false],
        handler: fun_14007a8f0,
    },
    Entry {
        name: "FUN_14007c060",
        pointers: &[true, false],
        handler: fun_14007c060,
    },
    Entry {
        name: "FUN_140085310",
        pointers: &[true],
        handler: fun_140085310,
    },
    Entry {
        name: "FUN_14008ff50",
        pointers: &[true],
        handler: fun_14008ff50,
    },
    Entry {
        name: "FUN_140092e80",
        pointers: &[true],
        handler: fun_140092e80,
    },
    Entry {
        name: "FUN_140094910",
        pointers: &[true],
        handler: fun_140094910,
    },
    Entry {
        name: "FUN_140096010",
        pointers: &[false],
        handler: fun_140096010,
    },
    Entry {
        name: "FUN_140096030",
        pointers: &[false],
        handler: fun_140096030,
    },
    Entry {
        name: "FUN_140096050",
        pointers: &[false],
        handler: fun_140096050,
    },
    Entry {
        name: "FUN_140096210",
        pointers: &[false],
        handler: fun_140096210,
    },
    Entry {
        name: "FUN_1400963e0",
        pointers: &[false],
        handler: fun_1400963e0,
    },
    Entry {
        name: "FUN_140096400",
        pointers: &[false],
        handler: fun_140096400,
    },
    Entry {
        name: "FUN_140096420",
        pointers: &[false],
        handler: fun_140096420,
    },
    Entry {
        name: "FUN_140096ca0",
        pointers: &[false],
        handler: fun_140096ca0,
    },
    Entry {
        name: "FUN_140096d10",
        pointers: &[false],
        handler: fun_140096d10,
    },
    Entry {
        name: "FUN_140096d30",
        pointers: &[false],
        handler: fun_140096d30,
    },
    Entry {
        name: "FUN_140096d50",
        pointers: &[false],
        handler: fun_140096d50,
    },
    Entry {
        name: "FUN_140097f50",
        pointers: &[false],
        handler: fun_140097f50,
    },
    Entry {
        name: "FUN_140099640",
        pointers: &[false],
        handler: fun_140099640,
    },
    Entry {
        name: "FUN_1400996d0",
        pointers: &[false],
        handler: fun_1400996d0,
    },
    Entry {
        name: "FUN_1400a5030",
        pointers: &[false, true],
        handler: fun_1400a5030,
    },
    Entry {
        name: "FUN_1400a5550",
        pointers: &[false],
        handler: fun_1400a5550,
    },
    Entry {
        name: "FUN_1400a58f0",
        pointers: &[false, false, true],
        handler: fun_1400a58f0,
    },
    Entry {
        name: "FUN_1400a5910",
        pointers: &[false],
        handler: fun_1400a5910,
    },
    Entry {
        name: "FUN_1400a5b30",
        pointers: &[false],
        handler: fun_1400a5b30,
    },
    Entry {
        name: "FUN_1400a5c90",
        pointers: &[false],
        handler: fun_1400a5c90,
    },
    Entry {
        name: "FUN_1400a5e60",
        pointers: &[false],
        handler: fun_1400a5e60,
    },
    Entry {
        name: "FUN_1400a6c70",
        pointers: &[false],
        handler: fun_1400a6c70,
    },
    Entry {
        name: "FUN_1400a6d80",
        pointers: &[false, false, true],
        handler: fun_1400a6d80,
    },
    Entry {
        name: "FUN_1400a6db0",
        pointers: &[false],
        handler: fun_1400a6db0,
    },
    Entry {
        name: "FUN_1400a8bc0",
        pointers: &[true, false],
        handler: fun_1400a8bc0,
    },
    Entry {
        name: "FUN_1400a98e0",
        pointers: &[true],
        handler: fun_1400a98e0,
    },
    Entry {
        name: "FUN_1400a9a40",
        pointers: &[true],
        handler: fun_1400a9a40,
    },
    Entry {
        name: "FUN_1400a9a80",
        pointers: &[true],
        handler: fun_1400a9a80,
    },
    Entry {
        name: "FUN_1400aa4e0",
        pointers: &[true, false],
        handler: fun_1400aa4e0,
    },
    Entry {
        name: "FUN_1400aa510",
        pointers: &[true, false],
        handler: fun_1400aa510,
    },
    Entry {
        name: "FUN_1400aa870",
        pointers: &[true],
        handler: fun_1400aa870,
    },
    Entry {
        name: "FUN_1400ab570",
        pointers: &[true, false],
        handler: fun_1400ab570,
    },
    Entry {
        name: "FUN_1400ab5a0",
        pointers: &[true, false],
        handler: fun_1400ab5a0,
    },
    Entry {
        name: "FUN_1400b0020",
        pointers: &[],
        handler: fun_1400b0020,
    },
    Entry {
        name: "FUN_1400b0200",
        pointers: &[true, false],
        handler: fun_1400b0200,
    },
    Entry {
        name: "FUN_1400b0e80",
        pointers: &[false, false, false, true],
        handler: fun_1400b0e80,
    },
    Entry {
        name: "FUN_1400b1d80",
        pointers: &[true],
        handler: fun_1400b1d80,
    },
    Entry {
        name: "FUN_1400b2180",
        pointers: &[true],
        handler: fun_1400b2180,
    },
    Entry {
        name: "FUN_1400b2240",
        pointers: &[true],
        handler: fun_1400b2240,
    },
    Entry {
        name: "FUN_1400b2740",
        pointers: &[true],
        handler: fun_1400b2740,
    },
    Entry {
        name: "FUN_1400b2bf0",
        pointers: &[true],
        handler: fun_1400b2bf0,
    },
    Entry {
        name: "FUN_1400b3d90",
        pointers: &[true],
        handler: fun_1400b3d90,
    },
    Entry {
        name: "FUN_1400b41d0",
        pointers: &[true],
        handler: fun_1400b41d0,
    },
    Entry {
        name: "FUN_1400b4850",
        pointers: &[true],
        handler: fun_1400b4850,
    },
    Entry {
        name: "FUN_1400b4ee0",
        pointers: &[true],
        handler: fun_1400b4ee0,
    },
    Entry {
        name: "FUN_1400b51b0",
        pointers: &[true],
        handler: fun_1400b51b0,
    },
    Entry {
        name: "FUN_1400b5850",
        pointers: &[true],
        handler: fun_1400b5850,
    },
    Entry {
        name: "FUN_1400b5ba0",
        pointers: &[true],
        handler: fun_1400b5ba0,
    },
    Entry {
        name: "FUN_1400b5eb0",
        pointers: &[true],
        handler: fun_1400b5eb0,
    },
    Entry {
        name: "FUN_1400b64b0",
        pointers: &[true],
        handler: fun_1400b64b0,
    },
    Entry {
        name: "FUN_1400b67b0",
        pointers: &[true],
        handler: fun_1400b67b0,
    },
    Entry {
        name: "FUN_1400b6930",
        pointers: &[true],
        handler: fun_1400b6930,
    },
    Entry {
        name: "FUN_1400b6f10",
        pointers: &[true],
        handler: fun_1400b6f10,
    },
    Entry {
        name: "FUN_1400b7900",
        pointers: &[true],
        handler: fun_1400b7900,
    },
    Entry {
        name: "FUN_1400b7ea0",
        pointers: &[true],
        handler: fun_1400b7ea0,
    },
    Entry {
        name: "FUN_1400b80e0",
        pointers: &[true],
        handler: fun_1400b80e0,
    },
];
