//! Checked translations generated from original bodies by batch 010.
use super::adapters9::AdapterError;
use super::call_support::{MemoryImage, MissingArguments, RecordCall, RelocateCall};
/// Translation of `FUN_140001000` (`mspaint.c:5115`).
pub fn fun_140001000(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("atexit", &[relocate("FUN_1400df700")], memory);
    Ok(None)
}

/// Translation of `FUN_140001050` (`mspaint.c:5138`).
pub fn fun_140001050(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("atexit", &[relocate("FUN_1400df7c0")], memory);
    Ok(None)
}

/// Translation of `FUN_140001060` (`mspaint.c:5147`).
pub fn fun_140001060(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_14006aaf0", &[], memory);
    invoke("atexit", &[relocate("FUN_1400df880")], memory);
    Ok(None)
}

/// Translation of `FUN_1400012c0` (`mspaint.c:5249`).
pub fn fun_1400012c0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("atexit", &[relocate("FUN_1400df8c0")], memory);
    Ok(None)
}

/// Translation of `FUN_1400012d0` (`mspaint.c:5258`).
pub fn fun_1400012d0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("atexit", &[relocate("FUN_1400df8e0")], memory);
    Ok(None)
}

/// Translation of `FUN_1400013f0` (`mspaint.c:5313`).
pub fn fun_1400013f0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("FUN_1400b0f0c", &[relocate("DAT_1401cfa60")], memory);
    invoke("atexit", &[relocate("FUN_1400dfa40")], memory);
    Ok(None)
}

/// Translation of `FUN_140001410` (`mspaint.c:5323`).
pub fn fun_140001410(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    invoke("atexit", &[relocate("FUN_1400dfa50")], memory);
    Ok(None)
}

/// Translation of `FUN_140001420` (`mspaint.c:5332`).
pub fn fun_140001420(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = (invoke("AfxInitialize", &[0x0u64, 0xe00u64], memory)) as i32 as i64 as u64;
    {
        let stored = u64::from(((values[0]) as i8 as i64 as u64) as u8);
        memory.write_u8(relocate("DAT_1401cfad1"), stored as u8)?;
    }
    Ok(None)
}

/// Translation of `FUN_140001b80` (`mspaint.c:5740`).
pub fn fun_140001b80(
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
    if u64::from((memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?) != (0x0u64))
        != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    if u64::from((memory.read_u64(values[0])?) != (0x0u64)) != 0 {
        invoke("SysFreeString", &[memory.read_u64(values[0])?], memory);
        {
            let stored = 0x0u64;
            memory.write_u64(values[0], stored)?;
        }
    }
    Ok(None)
}

/// Translation of `FUN_140003190` (`mspaint.c:7158`).
pub fn fun_140003190(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000038");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u8(values[2])?)) as u8)) as u8);
        memory.write_u8(values[1], stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400031f0` (`mspaint.c:7190`).
pub fn fun_1400031f0(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000030");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140003360` (`mspaint.c:7331`).
pub fn fun_140003360(
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
    values[3] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000038");
    }
    if u64::from(
        ((((u64::from(memory.read_u8(values[3])?)) as i8 as i64 as u64) as i32 as i64 as u64)
            as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = u64::from((invoke("FUN_140005f20", &[], memory)) as u32);
    } else {
        values[2] = u64::from(
            (invoke(
                "FromNative",
                &[u64::from(
                    (u64::from(memory.read_u32(relocate("DAT_140179148"))?)) as u32,
                )],
                memory,
            )) as u32,
        );
    }
    {
        let stored = u64::from((values[2]) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400033b0` (`mspaint.c:7353`).
pub fn fun_1400033b0(
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
    values[2] = invoke("FUN_140002080", &[0x13u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"IBac\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"kgro\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x6e0075u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x3cu64), stored as u32)?;
    }
    {
        let stored = u64::from((0x64u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x40u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140004ee0` (`mspaint.c:9080`).
pub fn fun_140004ee0(
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
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    invoke("AcquireSRWLockExclusive", &[values[1]], memory);
    Ok(Some(values[0]))
}

/// Translation of `FUN_140009170` (`mspaint.c:12655`).
pub fn fun_140009170(
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
    values[2] = (values[1]).wrapping_add(0x30u64);
    if u64::from(
        ((values[1]) as i64) == (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = 0x0u64;
    }
    if u64::from(
        ((((u64::from(memory.read_u8((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = values[2];
            memory.write_u64(values[0], stored)?;
        }
        {
            let stored = values[0];
            memory.write_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
        }
        {
            let stored = relocate("FUN_14000a160");
            memory.write_u64(values[2], stored)?;
        }
    }
    invoke("FUN_140009ba0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400092a0` (`mspaint.c:12732`).
pub fn fun_1400092a0(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140106050"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140009590` (`mspaint.c:12923`).
pub fn fun_140009590(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x48u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000060");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140009a80` (`mspaint.c:13241`).
pub fn fun_140009a80(
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
    values[1] = invoke("FUN_140002080", &[0x2du64], memory);
    if u64::from(((values[1]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[2] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x3cu64),
            relocate("L\"on.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x44u64),
            relocate("L\"Asyn\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x4cu64),
            relocate("L\"cOpe\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x54u64),
            relocate("L\"rati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x5cu64),
            relocate("L\"on`1\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x64u64),
            relocate("L\"<Boo\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x6e00610065006cu64;
        memory.write_u64((values[1]).wrapping_add(0x6cu64), stored)?;
    }
    {
        let stored = u64::from((0x3eu64) as u16);
        memory.write_u16((values[1]).wrapping_add(0x74u64), stored as u16)?;
    }
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000a2b0` (`mspaint.c:13718`).
pub fn fun_14000a2b0(
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
    let values = [arguments[0], arguments[1], u64::from((arguments[2]) as u32)];
    {
        let stored = u64::from((values[2]) as u32);
        memory.write_u32(
            (memory.read_u64((values[0]).wrapping_add(0x20u64))?).wrapping_add(0x10u64),
            stored as u32,
        )?;
    }
    invoke("FUN_14000a0f0", &[], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000a580` (`mspaint.c:13896`).
pub fn fun_14000a580(
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
    let mut values = [arguments[0]];
    values[0] = memory.read_u64(values[0])?;
    if u64::from((values[0]) != (0x0u64)) != 0 {
        if u64::from(((memory.read_u64(values[0])?) as i64) != ((0x0u64) as i64)) != 0 {
            invoke("FUN_140001b30", &[values[0]], memory);
        }
        invoke("FUN_1400b022c", &[values[0], 0x18u64], memory);
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_14000b590` (`mspaint.c:14854`).
pub fn fun_14000b590(
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
    values[2] = invoke("FUN_140002080", &[0x1du64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICod\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"ecDo\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"wnlo\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"adHa\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x65006c0064006eu64;
        memory.write_u64((values[2]).wrapping_add(0x4cu64), stored)?;
    }
    {
        let stored = u64::from((0x72u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x54u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14000c540` (`mspaint.c:15742`).
pub fn fun_14000c540(
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
    values[2] = (values[1]).wrapping_add(0x30u64);
    if u64::from(
        ((values[1]) as i64) == (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = 0x0u64;
    }
    if u64::from(
        ((((u64::from(memory.read_u8((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = values[2];
            memory.write_u64(values[0], stored)?;
        }
        {
            let stored = values[0];
            memory.write_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
        }
        {
            let stored = relocate("FUN_14000a160");
            memory.write_u64(values[2], stored)?;
        }
    }
    invoke("FUN_14000caf0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_14000c650` (`mspaint.c:15819`).
pub fn fun_14000c650(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140107040"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14000ca50` (`mspaint.c:16084`).
pub fn fun_14000ca50(
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
    values[1] = invoke("FUN_140002080", &[0x1fu64], memory);
    if u64::from(((values[1]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[2] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x3cu64),
            relocate("L\"on.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x44u64),
            relocate("L\"Asyn\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x74006300410063u64;
        memory.write_u64((values[1]).wrapping_add(0x4cu64), stored)?;
    }
    {
        let stored = u64::from((0x6f0069u64) as u32);
        memory.write_u32((values[1]).wrapping_add(0x54u64), stored as u32)?;
    }
    {
        let stored = u64::from((0x6eu64) as u16);
        memory.write_u16((values[1]).wrapping_add(0x58u64), stored as u16)?;
    }
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14000d1b0` (`mspaint.c:16556`).
pub fn fun_14000d1b0(
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
    values[2] = invoke("FUN_140002080", &[0x12u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ID2D\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"Bitm\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x700061u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x3cu64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14000f390` (`mspaint.c:18089`).
pub fn fun_14000f390(
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
    if u64::from(
        ((((memory.read_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)))?) as i8 as i64
            as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = u64::from((0x0u64) as u8);
            memory.write_u8(
                (values[0]).wrapping_add((0x2u64).wrapping_mul(8)),
                stored as u8,
            )?;
        }
        values[1] = memory.read_u64(values[0])?;
        values[2] = (invoke(
            "GdipBitmapUnlockBits",
            &[
                memory.read_u64((values[1]).wrapping_add(0x8u64))?,
                memory.read_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)))?,
            ],
            memory,
        )) as i32 as i64 as u64;
        if u64::from(
            (((values[2]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            {
                let stored = (values[2]) as i32 as i64 as u64;
                memory.write_u32((values[1]).wrapping_add(0x10u64), stored as u32)?;
            }
        }
    }
    Ok(None)
}

/// Translation of `FUN_140013210` (`mspaint.c:21405`).
pub fn fun_140013210(
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
        invoke("LocalFree", &[memory.read_u64(values[0])?], memory);
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
    }
    Ok(None)
}

/// Translation of `FUN_140014d50` (`mspaint.c:22848`).
pub fn fun_140014d50(
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
    values[2] = invoke("FUN_140002080", &[0x1au64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Plat\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"form\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Help\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"ers.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"IGdi\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"Bitm\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x700061u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x4cu64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140015560` (`mspaint.c:23252`).
pub fn fun_140015560(
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
    if u64::from(
        ((((u64::from(memory.read_u8((values[0]).wrapping_add(0xa28u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = u64::from(
            (invoke(
                "Paint::Provenance::ProvenanceHelper::HasAIContent",
                &[(values[0]).wrapping_add(0x9c8u64)],
                memory,
            )) != 0,
        );
        if u64::from(
            (u64::from(
                (u64::from(
                    (values[2]) != 0
                        && (u64::from(
                            (u64::from(
                                ((memory.read_u64(values[1])?) as i64)
                                    != ((0x44149b513654cecau64) as i64),
                            )) != 0
                                || (u64::from(
                                    ((memory.read_u64(
                                        (values[1]).wrapping_add((0x1u64).wrapping_mul(8)),
                                    )?) as i64)
                                        != ((0x759ffab0c96acba9u64) as i64),
                                )) != 0,
                        )) != 0,
                )) != 0
                    && (u64::from(
                        (u64::from(
                            ((memory.read_u64(values[1])?) as i64)
                                != ((0x11d30728b96b3caeu64) as i64),
                        )) != 0
                            || (u64::from(
                                ((memory
                                    .read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                                    as i64)
                                    != ((0x2ef31ef800007b9du64) as i64),
                            )) != 0,
                    )) != 0,
            )) != 0
                && (u64::from(
                    (u64::from(
                        (u64::from(
                            ((memory.read_u64(values[1])?) as i64)
                                != ((0x11d30728b96b3cafu64) as i64),
                        )) != 0
                            || (u64::from(
                                ((memory
                                    .read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?)
                                    as i64)
                                    != ((0x2ef31ef800007b9du64) as i64),
                            )) != 0,
                    )) != 0
                        && (u64::from(
                            (u64::from(
                                ((memory.read_u64(values[1])?) as i64)
                                    != ((0x11d30728b96b3cb0u64) as i64),
                            )) != 0
                                || (u64::from(
                                    ((memory.read_u64(
                                        (values[1]).wrapping_add((0x1u64).wrapping_mul(8)),
                                    )?) as i64)
                                        != ((0x2ef31ef800007b9du64) as i64),
                                )) != 0,
                        )) != 0,
                )) != 0,
        ) != 0
        {
            return Ok(Some(0x0u64));
        }
    }
    Ok(Some(0x1u64))
}

/// Translation of `FUN_1400172a0` (`mspaint.c:24672`).
pub fn fun_1400172a0(
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
    values[1] = (values[0]).wrapping_add(0x9a0u64);
    if u64::from(
        ((((u64::from(memory.read_u8((values[0]).wrapping_add(0x9b0u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = invoke("FUN_140017300", &[], memory);
        if u64::from(
            ((((u64::from(memory.read_u8((values[0]).wrapping_add(0x9b0u64))?)) as i8 as i64 as u64)
                as i32 as i64 as u64) as i64)
                != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            if u64::from(((memory.read_u64(values[1])?) as i64) != ((0x0u64) as i64)) != 0 {
                invoke("FUN_140006970", &[values[1]], memory);
            }
            {
                let stored = u64::from((0x0u64) as u8);
                memory.write_u8((values[0]).wrapping_add(0x9b0u64), stored as u8)?;
            }
        }
        invoke(
            "Paint::ProjectFile::ProjectFileHandler::ProjectFileHandler",
            &[values[1], relocate("DAT_1401d06b8"), values[2]],
            memory,
        );
        {
            let stored = u64::from((0x1u64) as u8);
            memory.write_u8((values[0]).wrapping_add(0x9b0u64), stored as u8)?;
        }
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140017d90` (`mspaint.c:25267`).
pub fn fun_140017d90(
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
    values[2] = invoke("FUN_140002080", &[0x16u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ILay\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"ersM\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x670061006e0061u64;
        memory.write_u64((values[2]).wrapping_add(0x3cu64), stored)?;
    }
    {
        let stored = u64::from((0x720065u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x44u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140017f70` (`mspaint.c:25356`).
pub fn fun_140017f70(
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
        ((memory.read_u64((values[0]).wrapping_add(0x58u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke("FUN_140001b30", &[], memory);
    }
    invoke(
        "FUN_1400183f0",
        &[
            (values[0]).wrapping_add(0x48u64),
            (values[0]).wrapping_add(0x48u64),
            memory.read_u64(
                (memory.read_u64((values[0]).wrapping_add(0x48u64))?).wrapping_add(0x8u64),
            )?,
        ],
        memory,
    );
    invoke(
        "FUN_1400b022c",
        &[memory.read_u64((values[0]).wrapping_add(0x48u64))?, 0x38u64],
        memory,
    );
    invoke(
        "FUN_14000f600",
        &[(values[0]).wrapping_add(0x28u64)],
        memory,
    );
    invoke("FUN_14000f600", &[(values[0]).wrapping_add(0x8u64)], memory);
    Ok(Some(values[0]))
}

/// Translation of `FUN_1400199e0` (`mspaint.c:26826`).
pub fn fun_1400199e0(
    arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if arguments.is_empty() {
        return Err(AdapterError::Arguments(MissingArguments {
            required: 1,
            available: arguments.len(),
        }));
    }
    let mut values = [arguments[0], 0u64];
    values[1] = (values[0]).wrapping_add(0x68u64);
    if u64::from((0xfu64) < (memory.read_u64((values[0]).wrapping_add(0x80u64))?)) != 0 {
        values[1] = memory.read_u64(values[1])?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140019bb0` (`mspaint.c:26938`).
pub fn fun_140019bb0(
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
        let stored = relocate("std::exception::vftable");
        memory.write_u64(values[0], stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[0]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[0]).wrapping_add((0x2u64).wrapping_mul(8)), stored)?;
    }
    invoke(
        "__std_exception_copy",
        &[(values[1]).wrapping_add(0x8u64)],
        memory,
    );
    {
        let stored = relocate("std::_System_error::vftable");
        memory.write_u64(values[0], stored)?;
    }
    values[2] = u64::from(
        (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0x1cu64))?)) as u32)) as u32,
    );
    values[3] = u64::from(
        (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0x20u64))?)) as u32)) as u32,
    );
    values[4] = u64::from(
        (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0x24u64))?)) as u32)) as u32,
    );
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0x18u64))?)) as u32))
                as u32,
        );
        memory.write_u32(
            (values[0]).wrapping_add((0x3u64).wrapping_mul(8)),
            stored as u32,
        )?;
    }
    {
        let stored = u64::from((values[2]) as u32);
        memory.write_u32((values[0]).wrapping_add(0x1cu64), stored as u32)?;
    }
    {
        let stored = u64::from((values[3]) as u32);
        memory.write_u32(
            (values[0]).wrapping_add((0x4u64).wrapping_mul(8)),
            stored as u32,
        )?;
    }
    {
        let stored = u64::from((values[4]) as u32);
        memory.write_u32((values[0]).wrapping_add(0x24u64), stored as u32)?;
    }
    {
        let stored = relocate("std::filesystem::filesystem_error::vftable");
        memory.write_u64(values[0], stored)?;
    }
    invoke(
        "FUN_1400199f0",
        &[
            (values[0]).wrapping_add((0x5u64).wrapping_mul(8)),
            (values[1]).wrapping_add(0x28u64),
        ],
        memory,
    );
    invoke(
        "FUN_1400199f0",
        &[
            (values[0]).wrapping_add((0x9u64).wrapping_mul(8)),
            (values[1]).wrapping_add(0x48u64),
        ],
        memory,
    );
    invoke(
        "FUN_140019ae0",
        &[
            (values[0]).wrapping_add((0xdu64).wrapping_mul(8)),
            (values[1]).wrapping_add(0x68u64),
        ],
        memory,
    );
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001ac00` (`mspaint.c:27648`).
pub fn fun_14001ac00(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = u64::from(
        (u64::from((u64::from(memory.read_u32(relocate("DAT_140179144"))?)) as u32)) as u32,
    );
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        values[0] = u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179140"))?)) as u32)) as u32,
        );
    }
    invoke("FUN_14001a950", &[0x1fu64, values[0]], memory);
    values[0] = u64::from(
        (u64::from((u64::from(memory.read_u32(relocate("DAT_140179140"))?)) as u32)) as u32,
    );
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        values[0] = u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179144"))?)) as u32)) as u32,
        );
    }
    invoke("FUN_14001a950", &[0x20u64, values[0]], memory);
    invoke("FUN_14001bb70", &[], memory);
    invoke("FUN_14001bb70", &[], memory);
    Ok(None)
}

/// Translation of `FUN_14001d660` (`mspaint.c:30317`).
pub fn fun_14001d660(
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
    values[2] = invoke("FUN_140002080", &[0x45u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Obse\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"rvab\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"leVe\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"ctor\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\"`1<P\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"aint\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x8cu64),
            relocate("L\"UI.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x94u64),
            relocate("L\"olor\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x6d006500740049u64;
        memory.write_u64((values[2]).wrapping_add(0x9cu64), stored)?;
    }
    {
        let stored = u64::from((0x3eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0xa4u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14001e2d0` (`mspaint.c:31045`).
pub fn fun_14001e2d0(
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
    values[2] = invoke("FUN_140002080", &[0x3du64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Iter\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"ator\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"`1<P\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"aint\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\"UI.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"olor\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x6d006500740049u64;
        memory.write_u64((values[2]).wrapping_add(0x8cu64), stored)?;
    }
    {
        let stored = u64::from((0x3eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x94u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14001ef50` (`mspaint.c:31710`).
pub fn fun_14001ef50(
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
    values[1] = invoke("FUN_140002080", &[0x32u64], memory);
    if u64::from(((values[1]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[2] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x5cu64),
            relocate("L\"Iter\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x64u64),
            relocate("L\"ator\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x6cu64),
            relocate("L\"`1<O\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x74u64),
            relocate("L\"bjec\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x3e0074u64) as u32);
        memory.write_u32((values[1]).wrapping_add(0x7cu64), stored as u32)?;
    }
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_14001f480` (`mspaint.c:31963`).
pub fn fun_14001f480(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0xcu64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_0000001c");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001f4a0` (`mspaint.c:31978`).
pub fn fun_14001f4a0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000018");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14001f600` (`mspaint.c:32079`).
pub fn fun_14001f600(
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
    values[1] = invoke("FUN_140002080", &[0x36u64], memory);
    if u64::from(((values[1]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[2] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x5cu64),
            relocate("L\"Vect\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x64u64),
            relocate("L\"orCh\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x6cu64),
            relocate("L\"ange\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x74u64),
            relocate("L\"dEve\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x7200410074006eu64;
        memory.write_u64((values[1]).wrapping_add(0x7cu64), stored)?;
    }
    {
        let stored = u64::from((0x730067u64) as u32);
        memory.write_u32((values[1]).wrapping_add(0x84u64), stored as u32)?;
    }
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140022ba0` (`mspaint.c:34300`).
pub fn fun_140022ba0(
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
    values[2] = invoke("FUN_140002080", &[0x16u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"IAiI\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"nfor\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x6900740061006du64;
        memory.write_u64((values[2]).wrapping_add(0x3cu64), stored)?;
    }
    {
        let stored = u64::from((0x6e006fu64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x44u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140022cc0` (`mspaint.c:34370`).
pub fn fun_140022cc0(
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
        "GenerativeFillTool::CommitContent",
        &[memory.read_u64(values[1])?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400230b0` (`mspaint.c:34582`).
pub fn fun_1400230b0(
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
    values[2] = invoke("FUN_140002080", &[0x1eu64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"IGen\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"erat\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"iveF\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"illM\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x670061006e0061u64;
        memory.write_u64((values[2]).wrapping_add(0x4cu64), stored)?;
    }
    {
        let stored = u64::from((0x720065u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x54u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140024ba0` (`mspaint.c:36075`).
pub fn fun_140024ba0(
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
    values[2] = invoke("FUN_140002080", &[0x26u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    {
        let stored = 0x3600650061007bu64;
        memory.write_u64((values[2]).wrapping_add(0x1cu64), stored)?;
    }
    {
        let stored = 0x62003700380032u64;
        memory.write_u64((values[2]).wrapping_add(0x24u64), stored)?;
    }
    {
        let stored = 0x300030002d0030u64;
        memory.write_u64((values[2]).wrapping_add(0x2cu64), stored)?;
    }
    {
        let stored = 0x31002d00340038u64;
        memory.write_u64((values[2]).wrapping_add(0x34u64), stored)?;
    }
    {
        let stored = 0x2d003200640031u64;
        memory.write_u64((values[2]).wrapping_add(0x3cu64), stored)?;
    }
    {
        let stored = 0x62003300370039u64;
        memory.write_u64((values[2]).wrapping_add(0x44u64), stored)?;
    }
    {
        let stored = 0x6100300030002du64;
        memory.write_u64((values[2]).wrapping_add(0x4cu64), stored)?;
    }
    {
        let stored = 0x30003900630030u64;
        memory.write_u64((values[2]).wrapping_add(0x54u64), stored)?;
    }
    {
        let stored = 0x32006600380036u64;
        memory.write_u64((values[2]).wrapping_add(0x5cu64), stored)?;
    }
    {
        let stored = u64::from((0x7d0065u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x64u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140024db0` (`mspaint.c:36237`).
pub fn fun_140024db0(
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
    values[2] = invoke("FUN_140002080", &[0x26u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    {
        let stored = 0x3500350061007bu64;
        memory.write_u64((values[2]).wrapping_add(0x1cu64), stored)?;
    }
    {
        let stored = 0x36003800610038u64;
        memory.write_u64((values[2]).wrapping_add(0x24u64), stored)?;
    }
    {
        let stored = 0x350061002d0036u64;
        memory.write_u64((values[2]).wrapping_add(0x2cu64), stored)?;
    }
    {
        let stored = 0x31002d00300062u64;
        memory.write_u64((values[2]).wrapping_add(0x34u64), stored)?;
    }
    {
        let stored = 0x2d003200640031u64;
        memory.write_u64((values[2]).wrapping_add(0x3cu64), stored)?;
    }
    {
        let stored = 0x66003800300061u64;
        memory.write_u64((values[2]).wrapping_add(0x44u64), stored)?;
    }
    {
        let stored = 0x6300300030002du64;
        memory.write_u64((values[2]).wrapping_add(0x4cu64), stored)?;
    }
    {
        let stored = 0x37006600340030u64;
        memory.write_u64((values[2]).wrapping_add(0x54u64), stored)?;
    }
    {
        let stored = 0x33006300640032u64;
        memory.write_u64((values[2]).wrapping_add(0x5cu64), stored)?;
    }
    {
        let stored = u64::from((0x7d0063u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x64u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140024fc0` (`mspaint.c:36340`).
pub fn fun_140024fc0(
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
    values[2] = memory.read_u64(values[0])?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        values[3] = u64::from((invoke("GetLastError", &[], memory)) as u32);
        invoke("SysFreeString", &[values[2]], memory);
        invoke("SetLastError", &[values[3]], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(None);
    }
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(None)
}

/// Translation of `FUN_140033540` (`mspaint.c:47642`).
pub fn fun_140033540(
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
    values[2] = (values[1]).wrapping_add(0x30u64);
    if u64::from(
        ((values[1]) as i64) == (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = 0x0u64;
    }
    if u64::from(
        ((((u64::from(memory.read_u8((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = values[2];
            memory.write_u64(values[0], stored)?;
        }
        {
            let stored = values[0];
            memory.write_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
        }
        {
            let stored = relocate("FUN_140033700");
            memory.write_u64(values[2], stored)?;
        }
    }
    invoke("FUN_140033580", &[], memory);
    Ok(None)
}

/// Translation of `FUN_140033bf0` (`mspaint.c:48108`).
pub fn fun_140033bf0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140033c10` (`mspaint.c:48123`).
pub fn fun_140033c10(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x10u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000028");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140033c50` (`mspaint.c:48141`).
pub fn fun_140033c50(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000020");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140033cd0` (`mspaint.c:48180`).
pub fn fun_140033cd0(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000038");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140033d10` (`mspaint.c:48198`).
pub fn fun_140033d10(
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
    values[2] = invoke("FUN_140002080", &[0x25u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"IInf\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"orma\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"tion\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"Mess\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ageW\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ithA\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x6f006900740063u64;
        memory.write_u64((values[2]).wrapping_add(0x5cu64), stored)?;
    }
    {
        let stored = u64::from((0x6eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x64u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140036c90` (`mspaint.c:50815`).
pub fn fun_140036c90(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000050");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u8(values[2])?)) as u8)) as u8);
        memory.write_u8(values[1], stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140036cb0` (`mspaint.c:50830`).
pub fn fun_140036cb0(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000038");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140036ec0` (`mspaint.c:50959`).
pub fn fun_140036ec0(
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
    values[2] = invoke("FUN_140002080", &[0xeu64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x790061004c0049u64;
        memory.write_u64((values[2]).wrapping_add(0x2cu64), stored)?;
    }
    {
        let stored = u64::from((0x720065u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x34u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140037010` (`mspaint.c:51042`).
pub fn fun_140037010(
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
    values[1] = memory.read_u64(values[0])?;
    if u64::from((values[1]) != (0x0u64)) != 0 {
        values[2] = u64::from((invoke("GetLastError", &[], memory)) as u32);
        invoke("DeleteObject", &[values[1]], memory);
        invoke("SetLastError", &[values[2]], memory);
        {
            let stored = 0x0u64;
            memory.write_u64(values[0], stored)?;
        }
        return Ok(Some(values[0]));
    }
    {
        let stored = 0x0u64;
        memory.write_u64(values[0], stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140037380` (`mspaint.c:51217`).
pub fn fun_140037380(
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
    values[2] = memory.read_u64(values[0])?;
    if u64::from((values[2]) != (0x0u64)) != 0 {
        values[3] = u64::from((invoke("GetLastError", &[], memory)) as u32);
        invoke("DeleteObject", &[values[2]], memory);
        invoke("SetLastError", &[values[3]], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(None);
    }
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(None)
}

/// Translation of `FUN_14003bde0` (`mspaint.c:55819`).
pub fn fun_14003bde0(
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
    values[2] = memory.read_u64(values[1])?;
    values[3] = (0x0u64) as i32 as i64 as u64;
    values[4] = relocate("DAT_140105708");
    if u64::from(((values[2]) as i64) == ((0x0u64) as i64)) != 0 {
        values[6] = relocate("DAT_140105708");
        values[5] = (values[3]) as i32 as i64 as u64;
    } else {
        values[6] = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
        values[5] = ((u64::from(memory.read_u32((values[2]).wrapping_add(0x4u64))?)) as i32 as i64
            as u64) as i32 as i64 as u64;
    }
    values[2] = memory.read_u64(values[0])?;
    if u64::from(((values[2]) as i64) != ((0x0u64) as i64)) != 0 {
        values[3] = ((u64::from(memory.read_u32((values[2]).wrapping_add(0x4u64))?)) as i32 as i64
            as u64) as i32 as i64 as u64;
        values[4] = memory.read_u64((values[2]).wrapping_add(0x10u64))?;
    }
    if u64::from(
        (((values[3]) as i32 as i64 as u64) as i64) == (((values[5]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        if u64::from(
            (((values[3]) as i32 as i64 as u64) as i64) != (((0x0u64) as i32 as i64 as u64) as i64),
        ) != 0
        {
            values[3] =
                (invoke("FUN_14003bc20", &[values[4], values[6]], memory)) as i32 as i64 as u64;
            return Ok(Some(u64::from(
                (u64::from(
                    (((values[3]) as i32 as i64 as u64) as i64)
                        != (((0x0u64) as i32 as i64 as u64) as i64),
                )) != 0,
            )));
        }
        return Ok(Some(0u64));
    }
    Ok(Some(1u64))
}

/// Translation of `FUN_14003c730` (`mspaint.c:56444`).
pub fn fun_14003c730(
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
            "DeleteObject",
            &[memory.read_u64((values[0]).wrapping_add(0x20u64))?],
            memory,
        );
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

/// Translation of `FUN_140041950` (`mspaint.c:60971`).
pub fn fun_140041950(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
    }
    {
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140041980` (`mspaint.c:60986`).
pub fn fun_140041980(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000048");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400428b0` (`mspaint.c:61891`).
pub fn fun_1400428b0(
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
    values[2] = invoke("FUN_140002080", &[0x42u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Obse\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"rvab\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"leVe\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"ctor\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\"`1<P\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"aint\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x8cu64),
            relocate("L\"UI.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x94u64),
            relocate("L\"Laye\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x3e0072u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x9cu64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140042dd0` (`mspaint.c:62157`).
pub fn fun_140042dd0(
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
    values[2] = invoke("FUN_140002080", &[0x3au64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Iter\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"ator\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"`1<P\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"aint\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\"UI.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"Laye\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x3e0072u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x8cu64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140042ed0` (`mspaint.c:62219`).
pub fn fun_140042ed0(
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
    values[1] = memory.read_u64((values[0]).wrapping_add(0x8u64))?;
    if u64::from(((values[1]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_140018220",
            &[(values[1]).wrapping_add(0x18u64)],
            memory,
        );
        if u64::from(
            ((memory.read_u64((values[1]).wrapping_add(0x10u64))?) as i64) != ((0x0u64) as i64),
        ) != 0
        {
            invoke(
                "FUN_140001b30",
                &[(values[1]).wrapping_add(0x10u64)],
                memory,
            );
        }
    }
    if u64::from(((memory.read_u64((values[0]).wrapping_add(0x8u64))?) as i64) != ((0x0u64) as i64))
        != 0
    {
        invoke(
            "FUN_1400b022c",
            &[memory.read_u64((values[0]).wrapping_add(0x8u64))?, 0x28u64],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400477f0` (`mspaint.c:66058`).
pub fn fun_1400477f0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = invoke("CNoTrackObject::operator_new", &[0x10u64], memory);
    if u64::from((values[0]) != (0x0u64)) != 0 {
        {
            let stored = relocate("_AFX_WIN_STATE::vftable");
            memory.write_u64(values[0], stored)?;
        }
        return Ok(None);
    }
    Ok(None)
}

/// Translation of `FUN_1400487d0` (`mspaint.c:66954`).
pub fn fun_1400487d0(
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
    values[2] = invoke("FUN_140002080", &[0x23u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ISto\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"reUp\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"date\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"Down\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"load\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"Mana\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x650067u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x5cu64), stored as u32)?;
    }
    {
        let stored = u64::from((0x72u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x60u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14004ab80` (`mspaint.c:68714`).
pub fn fun_14004ab80(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000038");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004aba0` (`mspaint.c:68729`).
pub fn fun_14004aba0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x24u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000034");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004abc0` (`mspaint.c:68744`).
pub fn fun_14004abc0(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x38u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000048");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004ac00` (`mspaint.c:68762`).
pub fn fun_14004ac00(
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
        let stored = 0x0u64;
        memory.write_u64(values[1], stored)?;
    }
    values[3] = (values[0]).wrapping_add(0x30u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000040");
    }
    values[2] = invoke("FUN_140005fd0", &[memory.read_u64(values[3])?], memory);
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004ac80` (`mspaint.c:68799`).
pub fn fun_14004ac80(
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
            "CPaintTool::IsDrawing",
            &[memory.read_u64(values[3])?],
            memory,
        )) != 0,
    );
    {
        let stored = u64::from((values[2]) != 0);
        memory.write_u8(values[1], stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004acc0` (`mspaint.c:68816`).
pub fn fun_14004acc0(
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
            "CPaintTool::IsCanvasResizeEnabled",
            &[memory.read_u64(values[3])?],
            memory,
        )) != 0,
    );
    {
        let stored = u64::from((values[2]) != 0);
        memory.write_u8(values[1], stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004ad80` (`mspaint.c:68874`).
pub fn fun_14004ad80(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x20u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000030");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_14004adf0` (`mspaint.c:68911`).
pub fn fun_14004adf0(
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
    values[2] = invoke("FUN_140002080", &[0x18u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"IToo\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"lInf\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"orma\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"tion\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14004bdf0` (`mspaint.c:69733`).
pub fn fun_14004bdf0(
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
    values[2] = (values[1]).wrapping_add(0x30u64);
    if u64::from(
        ((values[1]) as i64) == (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = 0x0u64;
    }
    if u64::from(
        ((((u64::from(memory.read_u8((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = values[2];
            memory.write_u64(values[0], stored)?;
        }
        {
            let stored = values[0];
            memory.write_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
        }
        {
            let stored = relocate("FUN_140033700");
            memory.write_u64(values[2], stored)?;
        }
    }
    invoke("FUN_14004c0e0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400532e0` (`mspaint.c:75602`).
pub fn fun_1400532e0(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = u64::from(
        (u64::from((u64::from(memory.read_u32(relocate("DAT_140179144"))?)) as u32)) as u32,
    );
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        values[0] = u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179140"))?)) as u32)) as u32,
        );
    }
    Ok(Some(u64::from((values[0]) as u32)))
}

/// Translation of `FUN_140053300` (`mspaint.c:75616`).
pub fn fun_140053300(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = u64::from(
        (u64::from((u64::from(memory.read_u32(relocate("DAT_140179140"))?)) as u32)) as u32,
    );
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) == (u64::from((0x0u64) as u32)),
    ) != 0
    {
        values[0] = u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179144"))?)) as u32)) as u32,
        );
    }
    Ok(Some(u64::from((values[0]) as u32)))
}

/// Translation of `FUN_1400534a0` (`mspaint.c:75700`).
pub fn fun_1400534a0(
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
    invoke(
        "FUN_14001aa10",
        &[
            values[0],
            u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32),
            values[1],
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_140055b10` (`mspaint.c:77824`).
pub fn fun_140055b10(
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
    values[2] = (values[1]).wrapping_add(0x30u64);
    if u64::from(
        ((values[1]) as i64) == (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = 0x0u64;
    }
    if u64::from(
        ((((u64::from(memory.read_u8((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = values[2];
            memory.write_u64(values[0], stored)?;
        }
        {
            let stored = values[0];
            memory.write_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
        }
        {
            let stored = relocate("FUN_140033700");
            memory.write_u64(values[2], stored)?;
        }
    }
    invoke("FUN_140058a40", &[], memory);
    Ok(None)
}

/// Translation of `FUN_140056040` (`mspaint.c:78355`).
pub fn fun_140056040(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140111c48"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400562d0` (`mspaint.c:78537`).
pub fn fun_1400562d0(
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
    values[2] = invoke("FUN_140002080", &[0x3du64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"Asyn\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"cOpe\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"rati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"on`1\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"<Pai\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"ntUI\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\".Ope\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\"rati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"onSt\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x73007500740061u64;
        memory.write_u64((values[2]).wrapping_add(0x8cu64), stored)?;
    }
    {
        let stored = u64::from((0x3eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x94u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140056510` (`mspaint.c:78679`).
pub fn fun_140056510(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140111c18"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140056ca0` (`mspaint.c:79165`).
pub fn fun_140056ca0(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140106050"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400571a0` (`mspaint.c:79467`).
pub fn fun_1400571a0(
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
    values[2] = invoke("FUN_140002080", &[0x1bu64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ITra\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"cker\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"Info\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"rmat\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x6f0069u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x4cu64), stored as u32)?;
    }
    {
        let stored = u64::from((0x6eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x50u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140057280` (`mspaint.c:79522`).
pub fn fun_140057280(
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
        values[1] = memory.read_u64(values[0])?;
        if u64::from((values[1]) != (0x0u64)) != 0 {
            invoke("GdipDeletePath", &[memory.read_u64(values[1])?], memory);
            invoke("GdipFree", &[values[1]], memory);
        }
        invoke("FUN_1400b022c", &[values[0], 0x8u64], memory);
    }
    Ok(None)
}

/// Translation of `FUN_140057ed0` (`mspaint.c:80222`).
pub fn fun_140057ed0(
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
    values[1] = invoke("FUN_140002080", &[0x40u64], memory);
    if u64::from(((values[1]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[2] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[1];
            memory.write_u64(values[0], stored)?;
        }
        return Ok(Some(values[0]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x3cu64),
            relocate("L\"on.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x44u64),
            relocate("L\"Asyn\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x4cu64),
            relocate("L\"cOpe\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x54u64),
            relocate("L\"rati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x5cu64),
            relocate("L\"on`1\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x64u64),
            relocate("L\"<Pla\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x6cu64),
            relocate("L\"tfor\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x74u64),
            relocate("L\"mHel\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x7cu64),
            relocate("L\"pers\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x84u64),
            relocate("L\".IGd\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x8cu64),
            relocate("L\"iBit\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[1]).wrapping_add(0x94u64),
            relocate("L\"map>\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = values[1];
        memory.write_u64(values[0], stored)?;
    }
    Ok(Some(values[0]))
}

/// Translation of `FUN_140058000` (`mspaint.c:80299`).
pub fn fun_140058000(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140111c18"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140059980` (`mspaint.c:81732`).
pub fn fun_140059980(
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
    values[2] = invoke("FUN_140002080", &[0x3du64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
    } else {
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x1cu64),
                relocate("L\"Wind\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x24u64),
                relocate("L\"ows.\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x2cu64),
                relocate("L\"Foun\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x34u64),
                relocate("L\"dati\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x3cu64),
                relocate("L\"on.I\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x44u64),
                relocate("L\"Asyn\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x4cu64),
                relocate("L\"cOpe\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x54u64),
                relocate("L\"rati\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x5cu64),
                relocate("L\"on`1\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x64u64),
                relocate("L\"<Pai\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x6cu64),
                relocate("L\"ntUI\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x74u64),
                relocate("L\".Ope\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x7cu64),
                relocate("L\"rati\""),
                0x4u64,
            ],
            memory,
        );
        invoke(
            "builtin_wcsncpy",
            &[
                (values[2]).wrapping_add(0x84u64),
                relocate("L\"onSt\""),
                0x4u64,
            ],
            memory,
        );
        {
            let stored = 0x73007500740061u64;
            memory.write_u64((values[2]).wrapping_add(0x8cu64), stored)?;
        }
        {
            let stored = u64::from((0x3eu64) as u16);
            memory.write_u16((values[2]).wrapping_add(0x94u64), stored as u16)?;
        }
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140062480` (`mspaint.c:88681`).
pub fn fun_140062480(
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
    values[1] = memory.read_u64((values[0]).wrapping_add(0x298u64))?;
    if u64::from(
        (u64::from(((memory.read_u64(values[1])?) as i64) != ((0x11d30728b96b3cabu64) as i64)))
            != 0
            || (u64::from(
                ((memory.read_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)))?) as i64)
                    != ((0x2ef31ef800007b9du64) as i64),
            )) != 0,
    ) != 0
    {
        values[1] = invoke("FUN_1400134c0", &[], memory);
    }
    invoke("FUN_1400622c0", &[values[0], values[1]], memory);
    Ok(None)
}

/// Translation of `FUN_140064fe0` (`mspaint.c:91211`).
pub fn fun_140064fe0(
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
    values[2] = invoke("FUN_140002080", &[0x46u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Vect\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"or`1\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"<Win\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"dows\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\".Sto\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"rage\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x8cu64),
            relocate("L\".ISt\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x94u64),
            relocate("L\"orag\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x65007400490065u64;
        memory.write_u64((values[2]).wrapping_add(0x9cu64), stored)?;
    }
    {
        let stored = u64::from((0x3e006du64) as u32);
        memory.write_u32((values[2]).wrapping_add(0xa4u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400657c0` (`mspaint.c:91689`).
pub fn fun_1400657c0(
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
    values[2] = invoke("FUN_140002080", &[0x48u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Iter\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"ator\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"`1<W\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"indo\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x7cu64),
            relocate("L\"ws.S\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x84u64),
            relocate("L\"tora\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x8cu64),
            relocate("L\"ge.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x94u64),
            relocate("L\"Stor\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x9cu64),
            relocate("L\"ageI\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0xa4u64),
            relocate("L\"tem>\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140067ff0` (`mspaint.c:93896`).
pub fn fun_140067ff0(
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
    let values = [arguments[0], arguments[1], arguments[2]];
    invoke(
        "CWnd::SetWindowPos",
        &[
            values[0],
            0x0u64,
            (u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64,
            (u64::from(memory.read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(4)))?)) as i32
                as i64 as u64,
            (((u64::from(memory.read_u32((values[2]).wrapping_add((0x2u64).wrapping_mul(4)))?))
                as i32 as i64 as u64)
                .wrapping_sub((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64))
                as i32 as i64 as u64,
            (((u64::from(memory.read_u32((values[2]).wrapping_add((0x3u64).wrapping_mul(4)))?))
                as i32 as i64 as u64)
                .wrapping_sub(
                    (u64::from(
                        memory.read_u32((values[2]).wrapping_add((0x1u64).wrapping_mul(4)))?,
                    )) as i32 as i64 as u64,
                )) as i32 as i64 as u64,
            0x14u64,
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140076880` (`mspaint.c:104298`).
pub fn fun_140076880(
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
    values[2] = (values[1]).wrapping_add(0x30u64);
    if u64::from(
        ((values[1]) as i64) == (((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64) as i64),
    ) != 0
    {
        values[2] = 0x0u64;
    }
    if u64::from(
        ((((u64::from(memory.read_u8((values[2]).wrapping_add((0x2u64).wrapping_mul(8)))?)) as i8
            as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = values[2];
            memory.write_u64(values[0], stored)?;
        }
        {
            let stored = values[0];
            memory.write_u64((values[2]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
        }
        {
            let stored = relocate("FUN_140033700");
            memory.write_u64(values[2], stored)?;
        }
    }
    invoke("FUN_140065160", &[], memory);
    Ok(None)
}

/// Translation of `FUN_140077770` (`mspaint.c:105023`).
pub fn fun_140077770(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140107040"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140077d10` (`mspaint.c:105378`).
pub fn fun_140077d10(
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
        let stored = u64::from((0x2u64) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    values[3] = invoke("CoTaskMemAlloc", &[0x20u64], memory);
    {
        let stored = values[3];
        memory.write_u64(values[2], stored)?;
    }
    if u64::from((values[3]) == (0x0u64)) != 0 {
        return Ok(Some(0x8007000eu64));
    }
    invoke(
        "memmove",
        &[values[3], relocate("DAT_140116e40"), 0x20u64],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400781c0` (`mspaint.c:105681`).
pub fn fun_1400781c0(
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
    values[2] = invoke("FUN_140002080", &[0x19u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICli\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"pboa\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"rdMa\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"nage\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x72u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x4cu64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_140079330` (`mspaint.c:106753`).
pub fn fun_140079330(
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
    values[2] = invoke("FUN_140002080", &[0x30u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Vect\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"or`1\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"<Str\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"ing>\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14007a5e0` (`mspaint.c:107673`).
pub fn fun_14007a5e0(
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
    values[2] = invoke("FUN_140002080", &[0x32u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Iter\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"ator\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"`1<S\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"trin\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x3e0067u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x7cu64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14008f670` (`mspaint.c:123948`).
pub fn fun_14008f670(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    if u64::from(
        ((((u64::from(memory.read_u8(relocate("DAT_1401d03a8"))?)) as i8 as i64 as u64) as i32
            as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_14008f530",
            &[
                relocate("FUN_140012480"),
                0x0u64,
                relocate("DAT_1401d03c0"),
                0x40u64,
            ],
            memory,
        );
        {
            let stored = (0x1u64) as i8 as i64 as u64;
            memory.write_u8(relocate("DAT_1401d03a8"), stored as u8)?;
        }
        return Ok(Some(relocate("DAT_1401d03c0")));
    }
    Ok(Some(relocate("DAT_1401d03c0")))
}

/// Translation of `FUN_140094f60` (`mspaint.c:128525`).
pub fn fun_140094f60(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64];
    values[3] = (values[0]).wrapping_add(0xcu64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_0000001c");
    }
    values[2] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
    {
        let stored = memory.read_u64(values[3])?;
        memory.write_u64(values[1], stored)?;
    }
    {
        let stored = values[2];
        memory.write_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140096ec0` (`mspaint.c:130479`).
pub fn fun_140096ec0(
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
    let mut values = [arguments[0], u64::from((arguments[1]) as u32), 0u64];
    values[2] = (values[0]).wrapping_add(0x8cu64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_000000a4");
    }
    {
        let stored = u64::from((values[1]) as u32);
        memory.write_u32(values[2], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140096ee0` (`mspaint.c:130494`).
pub fn fun_140096ee0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x8cu64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_000000a4");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400970e0` (`mspaint.c:130617`).
pub fn fun_1400970e0(
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
    values[2] = invoke("FUN_140002080", &[0x16u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICan\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"vasA\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x74007000610064u64;
        memory.write_u64((values[2]).wrapping_add(0x3cu64), stored)?;
    }
    {
        let stored = u64::from((0x720065u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x44u64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400982d0` (`mspaint.c:131632`).
pub fn fun_1400982d0(
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
    values[2] = invoke("FUN_140002080", &[0x21u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICan\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"vasI\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"mage\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"Sour\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ceAd\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"apte\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x72u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x5cu64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400999a0` (`mspaint.c:132761`).
pub fn fun_1400999a0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x68u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000078");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400999c0` (`mspaint.c:132776`).
pub fn fun_1400999c0(
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
    ];
    values[2] = (values[0]).wrapping_add(0x64u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000074");
    }
    if u64::from(
        ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32 as i64 as u64)
            as i64)
            != (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = (values[1]) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        values[3] = (values[0]).wrapping_add(0x8u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[3] = 0x18u64;
        }
        invoke(
            "FUN_140099c90",
            &[values[3], relocate("PTR_s_NumPreviewPages_14011b7a0")],
            memory,
        );
        values[3] = (values[0]).wrapping_add(0x58u64);
        values[4] = (values[0]).wrapping_add(0x40u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[3] = 0x68u64;
            values[4] = 0x50u64;
        }
        invoke("FUN_140099560", &[values[4], values[3]], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099a30` (`mspaint.c:132807`).
pub fn fun_140099a30(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x64u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000074");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099a50` (`mspaint.c:132822`).
pub fn fun_140099a50(
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
    ];
    values[2] = (values[0]).wrapping_add(0x60u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000070");
    }
    if u64::from(
        ((((u64::from(memory.read_u32(values[2])?)) as i32 as i64 as u64) as i32 as i64 as u64)
            as i64)
            != (((values[1]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = (values[1]) as i32 as i64 as u64;
            memory.write_u32(values[2], stored as u32)?;
        }
        values[3] = (values[0]).wrapping_add(0x8u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[3] = 0x18u64;
        }
        invoke(
            "FUN_140099c90",
            &[values[3], relocate("PTR_s_CurrentPage_14011b790")],
            memory,
        );
        values[3] = (values[0]).wrapping_add(0x58u64);
        values[4] = (values[0]).wrapping_add(0x40u64);
        if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
            values[3] = 0x68u64;
            values[4] = 0x50u64;
        }
        invoke("FUN_140099560", &[values[4], values[3]], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099ac0` (`mspaint.c:132853`).
pub fn fun_140099ac0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x60u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000070");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_140099b30` (`mspaint.c:132886`).
pub fn fun_140099b30(
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
    values[2] = invoke("FUN_140002080", &[0x2du64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICan\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"vasP\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"rint\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"Prev\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"iewI\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"mage\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Sour\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"ceAd\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x65007400700061u64;
        memory.write_u64((values[2]).wrapping_add(0x6cu64), stored)?;
    }
    {
        let stored = u64::from((0x72u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x74u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14009a2a0` (`mspaint.c:133303`).
pub fn fun_14009a2a0(
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
    values[2] = invoke("FUN_140002080", &[0x2au64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICan\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"vasT\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"humb\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"nail\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"Imag\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"eSou\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"rceA\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"dapt\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x720065u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x6cu64), stored as u32)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_14009b420` (`mspaint.c:134203`).
pub fn fun_14009b420(
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
    values[2] = invoke("FUN_140002080", &[0x11u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"IGeo\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"metr\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x79u64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x3cu64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400a32f0` (`mspaint.c:139988`).
pub fn fun_1400a32f0(
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
    values[1] = u64::from((invoke("FUN_1400a3d60", &[], memory)) as u8);
    invoke(
        "FUN_14004cad0",
        &[
            memory.read_u64((memory.read_u64(values[0])?).wrapping_add(0x40u64))?,
            values[1],
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400a4fb0` (`mspaint.c:141443`).
pub fn fun_1400a4fb0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000018");
    }
    {
        let stored = values[1];
        memory.write_u64(values[2], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a4fd0` (`mspaint.c:141458`).
pub fn fun_1400a4fd0(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000018");
    }
    {
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5530` (`mspaint.c:141801`).
pub fn fun_1400a5530(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x3e0u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_000003f0");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u8(values[2])?)) as u8)) as u8);
        memory.write_u8(values[1], stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5bb0` (`mspaint.c:142196`).
pub fn fun_1400a5bb0(
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
    values[2] = (values[0]).wrapping_add((0u64.wrapping_sub(0x10u64)) as i32 as i64 as u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = 0x0u64;
    }
    invoke(
        "FUN_14009d600",
        &[
            values[2],
            u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32),
            values[1],
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5c40` (`mspaint.c:142230`).
pub fn fun_1400a5c40(
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
    let values = [arguments[0], (arguments[1]) as i32 as i64 as u64];
    if u64::from(
        (u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        )) != (u64::from((values[1]) as u32)),
    ) != 0
    {
        {
            let stored = u64::from((values[1]) as u32);
            memory.write_u32(relocate("DAT_140179158"), stored as u32)?;
        }
        invoke(
            "FUN_14001bb70",
            &[values[0], relocate("PTR_s_ActiveColorSlot_140109518")],
            memory,
        );
        invoke("FUN_14001ac00", &[], memory);
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a5c80` (`mspaint.c:142243`).
pub fn fun_1400a5c80(
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
    let values = [arguments[0], arguments[1]];
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32(relocate("DAT_140179158"))?)) as u32)) as u32,
        );
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6e50` (`mspaint.c:143227`).
pub fn fun_1400a6e50(
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
    let values = [arguments[0], arguments[1]];
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u8(relocate("DAT_1401d085a"))?)) as u8)) as u8,
        );
        memory.write_u8(values[1], stored as u8)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400a6e60` (`mspaint.c:143236`).
pub fn fun_1400a6e60(
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
    values[2] = invoke("FUN_140002080", &[0x14u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Pain\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"tUI.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"ICom\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"mand\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x740073006f0048u64;
        memory.write_u64((values[2]).wrapping_add(0x3cu64), stored)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400a8480` (`mspaint.c:144635`).
pub fn fun_1400a8480(
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
    values[2] = invoke("FUN_140002080", &[0x2fu64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Vect\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"or`1\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = 0x74006e0049003cu64;
        memory.write_u64((values[2]).wrapping_add(0x6cu64), stored)?;
    }
    {
        let stored = u64::from((0x320033u64) as u32);
        memory.write_u32((values[2]).wrapping_add(0x74u64), stored as u32)?;
    }
    {
        let stored = u64::from((0x3eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x78u64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400a9340` (`mspaint.c:145466`).
pub fn fun_1400a9340(
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
    values[2] = invoke("FUN_140002080", &[0x31u64], memory);
    if u64::from(((values[2]).wrapping_add(0x1cu64)) == (0x0u64)) != 0 {
        values[3] = invoke("_errno", &[], memory);
        {
            let stored = (0x16u64) as i32 as i64 as u64;
            memory.write_u32(values[3], stored as u32)?;
        }
        invoke("_invalid_parameter_noinfo", &[], memory);
        {
            let stored = values[2];
            memory.write_u64(values[1], stored)?;
        }
        return Ok(Some(values[1]));
    }
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x1cu64),
            relocate("L\"Wind\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x24u64),
            relocate("L\"ows.\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x2cu64),
            relocate("L\"Foun\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x34u64),
            relocate("L\"dati\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x3cu64),
            relocate("L\"on.C\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x44u64),
            relocate("L\"olle\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x4cu64),
            relocate("L\"ctio\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x54u64),
            relocate("L\"ns.I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x5cu64),
            relocate("L\"Iter\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x64u64),
            relocate("L\"ator\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x6cu64),
            relocate("L\"`1<I\""),
            0x4u64,
        ],
        memory,
    );
    invoke(
        "builtin_wcsncpy",
        &[
            (values[2]).wrapping_add(0x74u64),
            relocate("L\"nt32\""),
            0x4u64,
        ],
        memory,
    );
    {
        let stored = u64::from((0x3eu64) as u16);
        memory.write_u16((values[2]).wrapping_add(0x7cu64), stored as u16)?;
    }
    {
        let stored = values[2];
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(values[1]))
}

/// Translation of `FUN_1400adf40` (`mspaint.c:149515`).
pub fn fun_1400adf40(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x28u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000038");
    }
    {
        let stored = memory.read_u64(values[2])?;
        memory.write_u64(values[1], stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ae040` (`mspaint.c:149574`).
pub fn fun_1400ae040(
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
    let mut values = [arguments[0], arguments[1], 0u64, 0u64];
    values[3] = (values[0]).wrapping_add(0x8u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[3] = relocate("DAT_00000018");
    }
    values[2] = memory.read_u64((values[3]).wrapping_add((0x1u64).wrapping_mul(8)))?;
    {
        let stored = memory.read_u64(values[3])?;
        memory.write_u64(values[1], stored)?;
    }
    {
        let stored = values[2];
        memory.write_u64((values[1]).wrapping_add((0x1u64).wrapping_mul(8)), stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ae060` (`mspaint.c:149592`).
pub fn fun_1400ae060(
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
    let mut values = [arguments[0], arguments[1], 0u64];
    values[2] = (values[0]).wrapping_add(0x18u64);
    if u64::from(((values[0]) as i64) == ((0x0u64) as i64)) != 0 {
        values[2] = relocate("DAT_00000028");
    }
    {
        let stored = u64::from((u64::from((u64::from(memory.read_u32(values[2])?)) as u32)) as u32);
        memory.write_u32(values[1], stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400b08c8` (`mspaint.c:155198`).
pub fn fun_1400b08c8(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    _relocate: &mut RelocateCall<'_>,
    invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    let mut values = [0u64];
    values[0] = invoke("FUN_1400124a0", &[], memory);
    {
        let stored = (memory.read_u64(values[0])?) | (0x24u64);
        memory.write_u64(values[0], stored)?;
    }
    values[0] = invoke("FUN_1400b08c0", &[], memory);
    {
        let stored = (memory.read_u64(values[0])?) | (0x2u64);
        memory.write_u64(values[0], stored)?;
    }
    Ok(None)
}

/// Translation of `FUN_1400b08e4` (`mspaint.c:155212`).
pub fn fun_1400b08e4(
    _arguments: &[u64],
    memory: &mut MemoryImage,
    relocate: &mut RelocateCall<'_>,
    _invoke: &mut RecordCall<'_>,
) -> Result<Option<u64>, AdapterError> {
    Ok(Some(u64::from(
        (u64::from(
            ((((u64::from(memory.read_u32(relocate("DAT_140179088"))?)) as i32 as i64 as u64) as i32
                as i64 as u64) as i64)
                == (((0x0u64) as i32 as i64 as u64) as i64),
        )) != 0,
    )))
}

/// Translation of `FUN_1400b8320` (`mspaint.c:162456`).
pub fn fun_1400b8320(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400b8330", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b8560` (`mspaint.c:162573`).
pub fn fun_1400b8560(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400b8570", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b9340` (`mspaint.c:163214`).
pub fn fun_1400b9340(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x98u64), stored)?;
    }
    invoke("FUN_1400b9360", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400b9350` (`mspaint.c:163224`).
pub fn fun_1400b9350(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x98u64), stored)?;
    }
    invoke("FUN_1400b9b60", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400ba220` (`mspaint.c:163909`).
pub fn fun_1400ba220(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400ba230", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bab90` (`mspaint.c:164266`).
pub fn fun_1400bab90(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400baba0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bb0e0` (`mspaint.c:164469`).
pub fn fun_1400bb0e0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400bb0f0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bba00` (`mspaint.c:164861`).
pub fn fun_1400bba00(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x98u64), stored)?;
    }
    invoke("FUN_1400bba10", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bbf50` (`mspaint.c:165096`).
pub fn fun_1400bbf50(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400bbf60", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bc1f0` (`mspaint.c:165238`).
pub fn fun_1400bc1f0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400bc200", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bc4a0` (`mspaint.c:165384`).
pub fn fun_1400bc4a0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x98u64), stored)?;
    }
    invoke("FUN_1400bc4b0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bc890` (`mspaint.c:165560`).
pub fn fun_1400bc890(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x58u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x58u64), stored)?;
    }
    invoke("FUN_1400bc8a0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bede0` (`mspaint.c:167211`).
pub fn fun_1400bede0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x28u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x28u64), stored)?;
    }
    invoke("FUN_1400bedf0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bf360` (`mspaint.c:167442`).
pub fn fun_1400bf360(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400bf370", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400bff00` (`mspaint.c:167923`).
pub fn fun_1400bff00(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x48u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x48u64), stored)?;
    }
    invoke("FUN_1400bff10", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c0250` (`mspaint.c:168099`).
pub fn fun_1400c0250(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0xb0u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0xb0u64), stored)?;
    }
    invoke("FUN_1400c0260", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c0b40` (`mspaint.c:168481`).
pub fn fun_1400c0b40(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c0b50", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c0db0` (`mspaint.c:168601`).
pub fn fun_1400c0db0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c0dc0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c1b60` (`mspaint.c:169138`).
pub fn fun_1400c1b60(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c1b70", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c2510` (`mspaint.c:169573`).
pub fn fun_1400c2510(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c2520", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c2df0` (`mspaint.c:169909`).
pub fn fun_1400c2df0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c2e00", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c3090` (`mspaint.c:170046`).
pub fn fun_1400c3090(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x98u64), stored)?;
    }
    invoke("FUN_1400c30b0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c30a0` (`mspaint.c:170056`).
pub fn fun_1400c30a0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c37a0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c3a30` (`mspaint.c:170501`).
pub fn fun_1400c3a30(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c3a40", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c3cc0` (`mspaint.c:170637`).
pub fn fun_1400c3cc0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c3cd0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c4050` (`mspaint.c:170798`).
pub fn fun_1400c4050(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x100u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x100u64), stored)?;
    }
    invoke("FUN_1400c4060", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c4950` (`mspaint.c:171138`).
pub fn fun_1400c4950(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x28u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x28u64), stored)?;
    }
    invoke("FUN_1400c4960", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c6480` (`mspaint.c:172240`).
pub fn fun_1400c6480(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x20u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x20u64), stored)?;
    }
    invoke("FUN_1400c6490", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c6a20` (`mspaint.c:172510`).
pub fn fun_1400c6a20(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c6a30", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c6d30` (`mspaint.c:172670`).
pub fn fun_1400c6d30(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c6d40", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c6fd0` (`mspaint.c:172792`).
pub fn fun_1400c6fd0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c6ff0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c6fe0` (`mspaint.c:172802`).
pub fn fun_1400c6fe0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c7750", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c7ea0` (`mspaint.c:173473`).
pub fn fun_1400c7ea0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c7eb0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c80f0` (`mspaint.c:173598`).
pub fn fun_1400c80f0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x88u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x88u64), stored)?;
    }
    invoke("FUN_1400c8100", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c85b0` (`mspaint.c:173820`).
pub fn fun_1400c85b0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x60u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x60u64), stored)?;
    }
    invoke("FUN_1400c85c0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c8c20` (`mspaint.c:174058`).
pub fn fun_1400c8c20(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x20u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x20u64), stored)?;
    }
    invoke("FUN_1400c8c30", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c91c0` (`mspaint.c:174343`).
pub fn fun_1400c91c0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c91d0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400c9d40` (`mspaint.c:174848`).
pub fn fun_1400c9d40(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400c9d50", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400ca4f0` (`mspaint.c:175149`).
pub fn fun_1400ca4f0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x98u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x98u64), stored)?;
    }
    invoke("FUN_1400ca500", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400cacd0` (`mspaint.c:175471`).
pub fn fun_1400cacd0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x90u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x90u64), stored)?;
    }
    invoke("FUN_1400cace0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400cb2b0` (`mspaint.c:175717`).
pub fn fun_1400cb2b0(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x30u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x30u64), stored)?;
    }
    invoke("FUN_1400cb2c0", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400cb670` (`mspaint.c:175920`).
pub fn fun_1400cb670(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0x30u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0x30u64), stored)?;
    }
    invoke("FUN_1400cb680", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400cb900` (`mspaint.c:176068`).
pub fn fun_1400cb900(
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
    {
        let stored = (memory.read_u64((values[0]).wrapping_add(0xa8u64))?) | (0x1u64);
        memory.write_u64((values[0]).wrapping_add(0xa8u64), stored)?;
    }
    invoke("FUN_1400cb910", &[], memory);
    Ok(None)
}

/// Translation of `FUN_1400cbec4` (`mspaint.c:176322`).
pub fn fun_1400cbec4(
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
    values[4] = memory.read_u64((values[3]).wrapping_add(0x38u64))?;
    invoke(
        "FUN_1400b0ea0",
        &[values[1], values[3], (values[4]).wrapping_add(0x4u64)],
        memory,
    );
    if u64::from(
        (u64::from(
            (u64::from(
                ((u64::from(
                    (u64::from(memory.read_u32((values[4]).wrapping_add(0x4u64))?)) as u32,
                )) & (((u64::from(
                    (u64::from(
                        (u64::from(
                            ((u64::from(
                                (u64::from(memory.read_u32((values[0]).wrapping_add(0x4u64))?))
                                    as u32,
                            )) & (0x66u64)) as u32,
                        )) as u32,
                    )) != (u64::from((0x0u64) as u32)),
                ))
                .wrapping_add(0x1u64)) as i32 as i64 as u64)) as u32,
            )) as u32,
        )) != (u64::from((0x0u64) as u32)),
    ) != 0
    {
        invoke(
            "__CxxFrameHandler4",
            &[values[0], values[1], values[2], values[3]],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400cc330` (`mspaint.c:176605`).
pub fn fun_1400cc330(
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
        "FUN_140004dd0",
        &[
            memory.read_u64((values[1]).wrapping_add(0x78u64))?,
            memory.read_u64((values[1]).wrapping_add(0x30u64))?,
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cc818` (`mspaint.c:176770`).
pub fn fun_1400cc818(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x80u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x20u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"Canvas::Apply\"");
        memory.write_u64((values[1]).wrapping_add(0x28u64), stored)?;
    }
    {
        let stored = 0xdu64;
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x88u64),
            (values[1]).wrapping_add(0x28u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x88u64),
            (values[1]).wrapping_add(0x20u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x88u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cc8a0` (`mspaint.c:176788`).
pub fn fun_1400cc8a0(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from((0x80004005u64) as u32);
        memory.write_u32((values[1]).wrapping_add(0x20u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"Canvas::Apply - unknown failure\"");
        memory.write_u64((values[1]).wrapping_add(0x28u64), stored)?;
    }
    {
        let stored = 0x1fu64;
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x88u64),
            (values[1]).wrapping_add(0x28u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x88u64),
            (values[1]).wrapping_add(0x20u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x88u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ccac8` (`mspaint.c:176806`).
pub fn fun_1400ccac8(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = (values[1]).wrapping_add(0x48u64);
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x80u64), stored)?;
    }
    {
        let stored = relocate("L\"CopyToClipboard\"");
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    {
        let stored = 0xfu64;
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x38u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendErrorEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x48u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    {
        let stored = relocate("L\"CopyToClipboardError\"");
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    {
        let stored = 0x14u64;
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x30u64),
            (values[1]).wrapping_add(0x38u64),
        ],
        memory,
    );
    {
        let stored = relocate("L\"CannotCompleteOperation\"");
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    {
        let stored = 0x17u64;
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x38u64),
        ],
        memory,
    );
    invoke(
        "FUN_140044770",
        &[
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x30u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x30u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ccd2c` (`mspaint.c:176887`).
pub fn fun_1400ccd2c(
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
        "FUN_140004e70",
        &[memory.read_u64((values[1]).wrapping_add(0x58u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cd2bc` (`mspaint.c:177008`).
pub fn fun_1400cd2bc(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = (values[1]).wrapping_add(0x60u64);
        memory.write_u64((values[1]).wrapping_add(0xb0u64), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x98u64), stored)?;
    }
    {
        let stored = relocate("L\"ReplaceFileCExp\"");
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    {
        let stored = 0xfu64;
        memory.write_u64((values[1]).wrapping_add(0x48u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0xa0u64),
            (values[1]).wrapping_add(0x40u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendErrorEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0xa0u64),
            (values[1]).wrapping_add(0x60u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0xa0u64)],
        memory,
    );
    invoke(
        "CException::Delete",
        &[memory.read_u64((values[1]).wrapping_add(0x58u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cd363` (`mspaint.c:177027`).
pub fn fun_1400cd363(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x50u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0xb0u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"ReplaceFileHR\"");
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    {
        let stored = 0xdu64;
        memory.write_u64((values[1]).wrapping_add(0x48u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0xa0u64),
            (values[1]).wrapping_add(0x40u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0xa0u64),
            (values[1]).wrapping_add(0xb0u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0xa0u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cd3ee` (`mspaint.c:177045`).
pub fn fun_1400cd3ee(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = (values[1]).wrapping_add(0x60u64);
        memory.write_u64((values[1]).wrapping_add(0xa0u64), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x98u64), stored)?;
    }
    {
        let stored = relocate("L\"ReplaceFile\"");
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    {
        let stored = 0xbu64;
        memory.write_u64((values[1]).wrapping_add(0x48u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0xb0u64),
            (values[1]).wrapping_add(0x40u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendErrorEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0xb0u64),
            (values[1]).wrapping_add(0x60u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0xb0u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cd888` (`mspaint.c:177147`).
pub fn fun_1400cd888(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x78u64))?;
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[2]).wrapping_add(0xcu64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x80u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"LoadProject\"");
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    {
        let stored = 0xbu64;
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x88u64),
            (values[1]).wrapping_add(0x38u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            memory.read_u64((values[1]).wrapping_add(0x30u64))?,
            (values[1]).wrapping_add(0x88u64),
            (values[1]).wrapping_add(0x80u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x88u64)],
        memory,
    );
    {
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64((values[1]).wrapping_add(0x90u64), stored)?;
    }
    {
        let stored = (values[1]).wrapping_add(0x90u64);
        memory.write_u64((values[1]).wrapping_add(0xc8u64), stored)?;
    }
    invoke(
        "Paint::Telemetry::TraceLogger::SendPerformanceCompleteEvent",
        &[
            memory.read_u64((values[1]).wrapping_add(0x30u64))?,
            0x1cu64,
            (values[1]).wrapping_add(0xd0u64),
            (values[1]).wrapping_add(0x90u64),
        ],
        memory,
    );
    {
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8((values[1]).wrapping_add(0x50u64), stored as u8)?;
    }
    values[3] = 0x3e9u64;
    if u64::from(
        ((((u64::from(memory.read_u32((values[2]).wrapping_add(0xcu64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == ((((0u64.wrapping_sub(0x7767d075u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                as i64),
    ) != 0
    {
        values[3] = 0x409u64;
    }
    invoke(
        "FUN_14006f460",
        &[
            relocate("DAT_1401d0520"),
            0x2u64,
            values[3],
            0x0u64,
            (values[1]).wrapping_add(0x38u64),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cd9a4` (`mspaint.c:177177`).
pub fn fun_1400cd9a4(
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
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x50u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"SaveProject\"");
        memory.write_u64((values[1]).wrapping_add(0xb0u64), stored)?;
    }
    {
        let stored = 0xbu64;
        memory.write_u64((values[1]).wrapping_add(0xb8u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x60u64),
            (values[1]).wrapping_add(0xb0u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            memory.read_u64((values[1]).wrapping_add(0x38u64))?,
            (values[1]).wrapping_add(0x60u64),
            (values[1]).wrapping_add(0x50u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x60u64)],
        memory,
    );
    {
        let stored = relocate("std::_Func_impl_no_alloc<>::vftable");
        memory.write_u64((values[1]).wrapping_add(0x68u64), stored)?;
    }
    {
        let stored = (values[1]).wrapping_add(0x68u64);
        memory.write_u64((values[1]).wrapping_add(0xa0u64), stored)?;
    }
    invoke(
        "Paint::Telemetry::TraceLogger::SendPerformanceCompleteEvent",
        &[
            memory.read_u64((values[1]).wrapping_add(0x38u64))?,
            0x1du64,
            (values[1]).wrapping_add(0xf0u64),
            (values[1]).wrapping_add(0x68u64),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cda6c` (`mspaint.c:177197`).
pub fn fun_1400cda6c(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x50u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x28u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"Recover\"");
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    {
        let stored = 0x7u64;
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x30u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x28u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x58u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cdb0c` (`mspaint.c:177215`).
pub fn fun_1400cdb0c(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x50u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x28u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"GetLayerStack\"");
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    {
        let stored = 0xdu64;
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x30u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x28u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x58u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cde82` (`mspaint.c:177275`).
pub fn fun_1400cde82(
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
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x78u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x40u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"PersistedFileAttributes::CopyToFile\"");
        memory.write_u64((values[1]).wrapping_add(0x58u64), stored)?;
    }
    {
        let stored = 0x23u64;
        memory.write_u64((values[1]).wrapping_add(0x60u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x48u64),
            (values[1]).wrapping_add(0x58u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            memory.read_u64((values[1]).wrapping_add(0x70u64))?,
            (values[1]).wrapping_add(0x48u64),
            (values[1]).wrapping_add(0x40u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x48u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ce4f0` (`mspaint.c:177432`).
pub fn fun_1400ce4f0(
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
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0x34u64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x20u64), stored as u32)?;
    }
    if u64::from(
        ((memory.read_u64((values[1]).wrapping_add(0x38u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140001b30",
            &[(values[1]).wrapping_add(0x38u64)],
            memory,
        );
    }
    if u64::from((memory.read_u64((values[1]).wrapping_add(0x28u64))?) != (0x0u64)) != 0 {
        invoke(
            "SysFreeString",
            &[memory.read_u64((values[1]).wrapping_add(0x28u64))?],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400ceb60` (`mspaint.c:177587`).
pub fn fun_1400ceb60(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x98u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0xb8u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"Layer::Preview\"");
        memory.write_u64((values[1]).wrapping_add(0xa0u64), stored)?;
    }
    {
        let stored = 0xeu64;
        memory.write_u64((values[1]).wrapping_add(0xa8u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0xd0u64),
            (values[1]).wrapping_add(0xa0u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0xd0u64),
            (values[1]).wrapping_add(0xb8u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0xd0u64)],
        memory,
    );
    {
        let stored = 0x0u64;
        memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x88u64))?, stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cf11a` (`mspaint.c:177720`).
pub fn fun_1400cf11a(
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
    values[2] = memory
        .read_u64((memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0x28u64))?;
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x38u64), stored)?;
    }
    invoke(
        "__ExceptionPtrCreate",
        &[(values[1]).wrapping_add(0x30u64)],
        memory,
    );
    invoke(
        "__ExceptionPtrCurrentException",
        &[(values[1]).wrapping_add(0x30u64)],
        memory,
    );
    invoke(
        "FUN_14003af90",
        &[values[2], (values[1]).wrapping_add(0x30u64)],
        memory,
    );
    invoke(
        "__ExceptionPtrDestroy",
        &[(values[1]).wrapping_add(0x30u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cf426` (`mspaint.c:177758`).
pub fn fun_1400cf426(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0xd0u64))?;
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[1]).wrapping_add(0x28u64), stored)?;
    }
    invoke(
        "__ExceptionPtrCreate",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    invoke(
        "__ExceptionPtrCurrentException",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    invoke(
        "FUN_14003af90",
        &[values[2], (values[1]).wrapping_add(0x20u64)],
        memory,
    );
    invoke(
        "__ExceptionPtrDestroy",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400cf4f8` (`mspaint.c:177791`).
pub fn fun_1400cf4f8(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x30u64))?;
    if u64::from(
        ((memory.read_u64((values[2]).wrapping_add(0x10u64))?) as i64) == ((0x0u64) as i64),
    ) != 0
    {
        {
            let stored = 0x0u64;
            memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
        }
        {
            let stored = 0x0u64;
            memory.write_u64((values[1]).wrapping_add(0x28u64), stored)?;
        }
        invoke(
            "__ExceptionPtrCreate",
            &[(values[1]).wrapping_add(0x20u64)],
            memory,
        );
        invoke(
            "__ExceptionPtrCurrentException",
            &[(values[1]).wrapping_add(0x20u64)],
            memory,
        );
        invoke(
            "FUN_14003af90",
            &[values[2], (values[1]).wrapping_add(0x20u64)],
            memory,
        );
        invoke(
            "__ExceptionPtrDestroy",
            &[(values[1]).wrapping_add(0x20u64)],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d235c` (`mspaint.c:179000`).
pub fn fun_1400d235c(
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
        let stored = memory.read_u64((values[1]).wrapping_add(0x70u64))?;
        memory.write_u64((values[1]).wrapping_add(0x88u64), stored)?;
    }
    invoke("AfxMessageBox", &[0xf188u64, 0x0u64, 0xffffffffu64], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d25e4` (`mspaint.c:179067`).
pub fn fun_1400d25e4(
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
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[1]).wrapping_add(0x64u64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x70u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"OLEEmbeddedClientInfo clientSite->QueryInterface\"");
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    {
        let stored = 0x30u64;
        memory.write_u64((values[1]).wrapping_add(0x48u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x40u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            memory.read_u64((values[1]).wrapping_add(0x30u64))?,
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x70u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    if u64::from(
        ((memory.read_u64((values[1]).wrapping_add(0x68u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140001b30",
            &[(values[1]).wrapping_add(0x68u64)],
            memory,
        );
    }
    if u64::from((memory.read_u64((values[1]).wrapping_add(0x58u64))?) != (0x0u64)) != 0 {
        invoke(
            "SysFreeString",
            &[memory.read_u64((values[1]).wrapping_add(0x58u64))?],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d2a2c` (`mspaint.c:179137`).
pub fn fun_1400d2a2c(
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
        let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
        memory.write_u64((values[1]).wrapping_add(0x30u64), stored)?;
    }
    invoke("AfxMessageBox", &[0xf188u64, 0x0u64, 0xffffffffu64], memory);
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d2aa0` (`mspaint.c:179147`).
pub fn fun_1400d2aa0(
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
        "_eh_vector_destructor_iterator_",
        &[
            (values[1]).wrapping_add(0x68u64),
            0x8u64,
            0x1u64,
            relocate("FUN_140002130"),
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400d2b48` (`mspaint.c:179156`).
pub fn fun_1400d2b48(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x50u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x60u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"OnShareBitmap\"");
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    {
        let stored = 0xdu64;
        memory.write_u64((values[1]).wrapping_add(0x48u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x40u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x60u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x58u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d2ca4` (`mspaint.c:179201`).
pub fn fun_1400d2ca4(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x88u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x98u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"OnShare\"");
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    {
        let stored = 0x7u64;
        memory.write_u64((values[1]).wrapping_add(0x28u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x90u64),
            (values[1]).wrapping_add(0x20u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x90u64),
            (values[1]).wrapping_add(0x98u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x90u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d2dd0` (`mspaint.c:179231`).
pub fn fun_1400d2dd0(
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
        "CException::Delete",
        &[memory.read_u64((values[1]).wrapping_add(0x20u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d3560` (`mspaint.c:179333`).
pub fn fun_1400d3560(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0xb8u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"ParseCommandLine::GetActivatedEventArgs\"");
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    {
        let stored = 0x27u64;
        memory.write_u64((values[1]).wrapping_add(0x28u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0xa0u64),
            (values[1]).wrapping_add(0x20u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0xa0u64),
            (values[1]).wrapping_add(0xb8u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0xa0u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d3c28` (`mspaint.c:179461`).
pub fn fun_1400d3c28(
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
            "FUN_14007c600",
            &[memory.read_u64((values[1]).wrapping_add(0x408u64))?],
            memory,
        )) as u32,
    );
    {
        let stored = u64::from((values[2]) as u32);
        memory.write_u32((values[1]).wrapping_add(0xe8u64), stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d5c9e` (`mspaint.c:180487`).
pub fn fun_1400d5c9e(
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
        "CException::Delete",
        &[memory.read_u64((values[1]).wrapping_add(0x28u64))?],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d5e08` (`mspaint.c:180496`).
pub fn fun_1400d5e08(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x88u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0xa8u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"CopyWICMetadata\"");
        memory.write_u64((values[1]).wrapping_add(0xd8u64), stored)?;
    }
    {
        let stored = 0xfu64;
        memory.write_u64((values[1]).wrapping_add(0xe0u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x90u64),
            (values[1]).wrapping_add(0xd8u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x90u64),
            (values[1]).wrapping_add(0xa8u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x90u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d61f8` (`mspaint.c:180551`).
pub fn fun_1400d61f8(
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
        let stored = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
        memory.write_u64((values[1]).wrapping_add(0x78u64), stored)?;
    }
    {
        let stored = u64::from((0x0u64) as u8);
        memory.write_u8((values[1]).wrapping_add(0x68u64), stored as u8)?;
    }
    invoke(
        "FUN_14006f460",
        &[
            relocate("DAT_1401d0520"),
            0x3u64,
            u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x10u64),
                )?)) as u32,
            ),
            0x0u64,
            (values[1]).wrapping_add(0x50u64),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d6280` (`mspaint.c:180563`).
pub fn fun_1400d6280(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x50u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"CSSMediaStore::RealizeMediaBitmap::Stroke\"");
        memory.write_u64((values[1]).wrapping_add(0x70u64), stored)?;
    }
    {
        let stored = 0x29u64;
        memory.write_u64((values[1]).wrapping_add(0x78u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x70u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x58u64),
            (values[1]).wrapping_add(0x50u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x58u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d632c` (`mspaint.c:180581`).
pub fn fun_1400d632c(
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
    values[2] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
    {
        let stored = u64::from(
            (u64::from(
                (u64::from(memory.read_u32(
                    (memory.read_u64((values[1]).wrapping_add(0x48u64))?).wrapping_add(0xcu64),
                )?)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x50u64), stored as u32)?;
    }
    {
        let stored = relocate("L\"CSSMediaStore::RealizeMediaBitmap::Fill\"");
        memory.write_u64((values[1]).wrapping_add(0x58u64), stored)?;
    }
    {
        let stored = 0x27u64;
        memory.write_u64((values[1]).wrapping_add(0x60u64), stored)?;
    }
    invoke(
        "FUN_140005310",
        &[
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x58u64),
        ],
        memory,
    );
    invoke(
        "Paint::Telemetry::TraceLogger::SendFailureEvent",
        &[
            values[2],
            (values[1]).wrapping_add(0x20u64),
            (values[1]).wrapping_add(0x50u64),
        ],
        memory,
    );
    invoke(
        "FUN_140002130",
        &[(values[1]).wrapping_add(0x20u64)],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d6650` (`mspaint.c:180721`).
pub fn fun_1400d6650(
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
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(
            memory.read_u64((values[1]).wrapping_add(0xa8u64))?,
            stored as u8,
        )?;
    }
    values[2] = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
    if u64::from(
        ((memory.read_u64((values[1]).wrapping_add(0x98u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140012570",
            &[
                memory.read_u64((values[1]).wrapping_add(0x98u64))?,
                memory.read_u64((values[1]).wrapping_add(0xa0u64))?,
                (values[2]).wrapping_add(0x18u64),
            ],
            memory,
        );
    }
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[2]).wrapping_add(0x20u64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x48u64), stored as u32)?;
    }
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[2]).wrapping_add(0x24u64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x4cu64), stored as u32)?;
    }
    {
        let stored = u64::from(
            (u64::from(
                ((u64::from(
                    ((u64::from(
                        (u64::from(memory.read_u32((values[2]).wrapping_add(0x1cu64))?)) as u32,
                    ) as u32)
                        .wrapping_shr(0x3u64 as u32) as u64) as u32,
                )) & (0x1u64)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x50u64), stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d6723` (`mspaint.c:180762`).
pub fn fun_1400d6723(
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
        "FUN_140090240",
        &[
            memory.read_u64((values[1]).wrapping_add(0x98u64))?,
            memory.read_u64((values[1]).wrapping_add(0xa0u64))?,
        ],
        memory,
    )) as i32 as i64 as u64;
    if u64::from(
        ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
            >= (((values[2]) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored = (values[2]) as i32 as i64 as u64;
            memory.write_u32((values[1]).wrapping_add(0x48u64), stored as u32)?;
        }
        values[3] = u64::from((invoke("FUN_140012f40", &[values[2]], memory)) as u32);
        {
            let stored = u64::from((values[3]) as u32);
            memory.write_u32((values[1]).wrapping_add(0x4cu64), stored as u32)?;
        }
        {
            let stored = u64::from((0x0u64) as u32);
            memory.write_u32((values[1]).wrapping_add(0x50u64), stored as u32)?;
        }
    }
    Ok(Some(u64::from(
        (u64::from(
            ((((0u64.wrapping_sub(0x1u64)) as i32 as i64 as u64) as i32 as i64 as u64) as i64)
                < (((values[2]) as i32 as i64 as u64) as i64),
        )) != 0,
    )))
}

/// Translation of `FUN_1400d6778` (`mspaint.c:180780`).
pub fn fun_1400d6778(
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
        let stored = u64::from((0x1u64) as u8);
        memory.write_u8(
            memory.read_u64((values[1]).wrapping_add(0xa8u64))?,
            stored as u8,
        )?;
    }
    values[2] = memory.read_u64((values[1]).wrapping_add(0x28u64))?;
    if u64::from(
        ((memory.read_u64((values[1]).wrapping_add(0x98u64))?) as i64) != ((0x0u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_140012570",
            &[
                memory.read_u64((values[1]).wrapping_add(0x98u64))?,
                memory.read_u64((values[1]).wrapping_add(0xa0u64))?,
                (values[2]).wrapping_add(0x18u64),
            ],
            memory,
        );
    }
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[2]).wrapping_add(0x20u64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x48u64), stored as u32)?;
    }
    {
        let stored = u64::from(
            (u64::from((u64::from(memory.read_u32((values[2]).wrapping_add(0x24u64))?)) as u32))
                as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x4cu64), stored as u32)?;
    }
    {
        let stored = u64::from(
            (u64::from(
                ((u64::from(
                    ((u64::from(
                        (u64::from(memory.read_u32((values[2]).wrapping_add(0x1cu64))?)) as u32,
                    ) as u32)
                        .wrapping_shr(0x3u64 as u32) as u64) as u32,
                )) & (0x1u64)) as u32,
            )) as u32,
        );
        memory.write_u32((values[1]).wrapping_add(0x50u64), stored as u32)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d830c` (`mspaint.c:181299`).
pub fn fun_1400d830c(
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
        "_seh_filter_exe",
        &[
            u64::from((u64::from(memory.read_u32(memory.read_u64(values[0])?)?)) as u32),
            values[0],
        ],
        memory,
    );
    Ok(None)
}

/// Translation of `FUN_1400d8348` (`mspaint.c:181316`).
pub fn fun_1400d8348(
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
        ((((u64::from(memory.read_u8((values[1]).wrapping_add(0x20u64))?)) as i8 as i64 as u64)
            as i32 as i64 as u64) as i64)
            == (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        invoke(
            "FUN_1400b02b4",
            &[
                memory.read_u64((values[1]).wrapping_add(0x60u64))?,
                memory.read_u64((values[1]).wrapping_add(0x68u64))?,
                memory.read_u64((values[1]).wrapping_add(0x70u64))?,
                memory.read_u64((values[1]).wrapping_add(0x78u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400d8570` (`mspaint.c:181448`).
pub fn fun_1400d8570(
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
        let stored = (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xd0u64);
        memory.write_u64((values[1]).wrapping_add(0x98u64), stored)?;
    }
    {
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x98u64))?)?;
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0x8au64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xf0u64);
            memory.write_u64((values[1]).wrapping_add(0xa0u64), stored)?;
        }
        {
            let stored = 0xf0u64;
            memory.write_u64((values[1]).wrapping_add(0xa8u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0xa8u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0xa0u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xe0u64);
            memory.write_u64((values[1]).wrapping_add(0xc8u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xd0u64);
            memory.write_u64((values[1]).wrapping_add(0xd0u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xd0u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64((values[1]).wrapping_add(0xd8u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0xd8u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0xc8u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xf0u64);
            memory.write_u64((values[1]).wrapping_add(0xe0u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xe0u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xe8u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xe0u64);
            memory.write_u64((values[1]).wrapping_add(0xf0u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xf0u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xf8u64), stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0xf8u64))?,
                memory.read_u64((values[1]).wrapping_add(0xe8u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400d8bfb` (`mspaint.c:181765`).
pub fn fun_1400d8bfb(
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
        let stored = (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0x388u64);
        memory.write_u64((values[1]).wrapping_add(0x1f8u64), stored)?;
    }
    {
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x1f8u64))?)?;
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0xbau64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0x3a0u64);
            memory.write_u64((values[1]).wrapping_add(0x200u64), stored)?;
        }
        {
            let stored = 0x1a0u64;
            memory.write_u64((values[1]).wrapping_add(0x208u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x208u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x200u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0x390u64);
            memory.write_u64((values[1]).wrapping_add(0x228u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0x388u64);
            memory.write_u64((values[1]).wrapping_add(0x230u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x230u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x40u64))?;
            memory.write_u64((values[1]).wrapping_add(0x238u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x238u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x228u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0x3a0u64);
            memory.write_u64((values[1]).wrapping_add(0x240u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x240u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x248u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x58u64))?).wrapping_add(0x390u64);
            memory.write_u64((values[1]).wrapping_add(0x250u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x250u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x258u64), stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0x258u64))?,
                memory.read_u64((values[1]).wrapping_add(0x248u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400d8fd1` (`mspaint.c:181929`).
pub fn fun_1400d8fd1(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0x48u64))?;
    values[3] = memory.read_u64((values[2]).wrapping_add(0xa0u64))?;
    {
        let stored = values[3];
        memory.write_u64((values[1]).wrapping_add(0x40u64), stored)?;
    }
    if u64::from(((values[3]) as i64) != ((0x0u64) as i64)) != 0 {
        invoke(
            "FUN_140001b30",
            &[(values[2]).wrapping_add(0xa0u64)],
            memory,
        );
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[2]).wrapping_add(0xa0u64), stored)?;
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400d94f0` (`mspaint.c:182225`).
pub fn fun_1400d94f0(
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
        let stored = (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xb0u64);
        memory.write_u64((values[1]).wrapping_add(0x80u64), stored)?;
    }
    {
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x80u64))?)?;
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0x92u64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xd0u64);
            memory.write_u64((values[1]).wrapping_add(0x88u64), stored)?;
        }
        {
            let stored = 0x240u64;
            memory.write_u64((values[1]).wrapping_add(0x90u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x90u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x88u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xc0u64);
            memory.write_u64((values[1]).wrapping_add(0xb0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xb0u64);
            memory.write_u64((values[1]).wrapping_add(0xb8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xb8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64((values[1]).wrapping_add(0xc0u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0xc0u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0xb0u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xd0u64);
            memory.write_u64((values[1]).wrapping_add(0xc8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xc8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xd0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xc0u64);
            memory.write_u64((values[1]).wrapping_add(0xd8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xd8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xe0u64), stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0xe0u64))?,
                memory.read_u64((values[1]).wrapping_add(0xd0u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400d9760` (`mspaint.c:182307`).
pub fn fun_1400d9760(
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
        let stored = (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xb0u64);
        memory.write_u64((values[1]).wrapping_add(0x80u64), stored)?;
    }
    {
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x80u64))?)?;
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0x7eu64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xd0u64);
            memory.write_u64((values[1]).wrapping_add(0x88u64), stored)?;
        }
        {
            let stored = 0x110u64;
            memory.write_u64((values[1]).wrapping_add(0x90u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x90u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x88u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xc0u64);
            memory.write_u64((values[1]).wrapping_add(0xb0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xb0u64);
            memory.write_u64((values[1]).wrapping_add(0xb8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xb8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64((values[1]).wrapping_add(0xc0u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0xc0u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0xb0u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xd0u64);
            memory.write_u64((values[1]).wrapping_add(0xc8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xc8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xd0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x30u64))?).wrapping_add(0xc0u64);
            memory.write_u64((values[1]).wrapping_add(0xd8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xd8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xe0u64), stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0xe0u64))?,
                memory.read_u64((values[1]).wrapping_add(0xd0u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400d9b20` (`mspaint.c:182463`).
pub fn fun_1400d9b20(
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
        let stored = (memory.read_u64((values[1]).wrapping_add(0x38u64))?).wrapping_add(0x40u64);
        memory.write_u64((values[1]).wrapping_add(0x60u64), stored)?;
    }
    {
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x60u64))?)?;
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0x92u64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x38u64))?).wrapping_add(0x60u64);
            memory.write_u64((values[1]).wrapping_add(0x68u64), stored)?;
        }
        {
            let stored = 0x1c0u64;
            memory.write_u64((values[1]).wrapping_add(0x70u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x70u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x68u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x38u64))?).wrapping_add(0x50u64);
            memory.write_u64((values[1]).wrapping_add(0x90u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x38u64))?).wrapping_add(0x40u64);
            memory.write_u64((values[1]).wrapping_add(0x98u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x98u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64((values[1]).wrapping_add(0xa0u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0xa0u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x90u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x38u64))?).wrapping_add(0x60u64);
            memory.write_u64((values[1]).wrapping_add(0xa8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xa8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xb0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x38u64))?).wrapping_add(0x50u64);
            memory.write_u64((values[1]).wrapping_add(0xb8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0xb8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0xc0u64), stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0xc0u64))?,
                memory.read_u64((values[1]).wrapping_add(0xb0u64))?,
            ],
            memory,
        );
    }
    Ok(None)
}

/// Translation of `FUN_1400da4b9` (`mspaint.c:182939`).
pub fn fun_1400da4b9(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0xc0u64))?;
    if u64::from(
        ((((u64::from(memory.read_u32((values[2]).wrapping_add(0xcu64))?)) as i32 as i64 as u64)
            as i32 as i64 as u64) as i64)
            != ((((0u64.wrapping_sub(0x7ff8fb39u64)) as i32 as i64 as u64) as i32 as i64 as u64)
                as i64),
    ) != 0
    {
        values[4] = invoke("Paint::Telemetry::TraceLogger::GetInstance", &[], memory);
        values[3] = memory.read_u64((values[1]).wrapping_add(0x60u64))?;
        {
            let stored = u64::from(
                (u64::from((u64::from(memory.read_u32((values[2]).wrapping_add(0xcu64))?)) as u32))
                    as u32,
            );
            memory.write_u32((values[3]).wrapping_add(0x1d8u64), stored as u32)?;
        }
        {
            let stored = relocate("L\"UploadEdgeHashOperation.UploadEdgeHashAsync\"");
            memory.write_u64((values[3]).wrapping_add(0x230u64), stored)?;
        }
        {
            let stored = 0x2bu64;
            memory.write_u64((values[3]).wrapping_add(0x238u64), stored)?;
        }
        invoke(
            "FUN_140005310",
            &[
                (values[3]).wrapping_add(0x1e0u64),
                (values[3]).wrapping_add(0x230u64),
            ],
            memory,
        );
        invoke(
            "Paint::Telemetry::TraceLogger::SendFailureEvent",
            &[
                values[4],
                (values[3]).wrapping_add(0x1e0u64),
                (values[3]).wrapping_add(0x1d8u64),
            ],
            memory,
        );
        invoke(
            "FUN_140002130",
            &[(values[3]).wrapping_add(0x1e0u64)],
            memory,
        );
    }
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400db514` (`mspaint.c:183836`).
pub fn fun_1400db514(
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
    values[2] = memory.read_u64((values[1]).wrapping_add(0xa8u64))?;
    {
        let stored = 0x0u64;
        memory.write_u64(values[2], stored)?;
    }
    {
        let stored = u64::from((0xffffu64) as u16);
        memory.write_u16(
            (values[2]).wrapping_add((0xbu64).wrapping_mul(8)),
            stored as u16,
        )?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[2]).wrapping_add((0x8du64).wrapping_mul(8)), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[2]).wrapping_add((0x8eu64).wrapping_mul(8)), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[2]).wrapping_add((0x8du64).wrapping_mul(8)), stored)?;
    }
    {
        let stored = 0x0u64;
        memory.write_u64((values[2]).wrapping_add((0x8eu64).wrapping_mul(8)), stored)?;
    }
    invoke(
        "__ExceptionPtrCreate",
        &[(values[2]).wrapping_add((0x8du64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "__ExceptionPtrCurrentException",
        &[(values[2]).wrapping_add((0x8du64).wrapping_mul(8))],
        memory,
    );
    invoke(
        "FUN_1400583a0",
        &[
            (values[2]).wrapping_add((0x2u64).wrapping_mul(8)),
            (values[2]).wrapping_add((0x8du64).wrapping_mul(8)),
        ],
        memory,
    );
    Ok(Some(0x0u64))
}

/// Translation of `FUN_1400db9af` (`mspaint.c:184079`).
pub fn fun_1400db9af(
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
        let stored = (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x1a8u64);
        memory.write_u64((values[1]).wrapping_add(0x180u64), stored)?;
    }
    {
        let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x180u64))?)?;
        memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
    }
    if u64::from(
        ((((u64::from(memory.read_u16(
            (memory.read_u64((values[1]).wrapping_add(0x20u64))?).wrapping_add(0x9au64),
        )?)) as i16 as i64 as u64) as i32 as i64 as u64) as i64)
            != (((0x0u64) as i32 as i64 as u64) as i64),
    ) != 0
    {
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x1c0u64);
            memory.write_u64((values[1]).wrapping_add(0x188u64), stored)?;
        }
        {
            let stored = 0x1b0u64;
            memory.write_u64((values[1]).wrapping_add(0x190u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x190u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x188u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x1b0u64);
            memory.write_u64((values[1]).wrapping_add(0x1b0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x1a8u64);
            memory.write_u64((values[1]).wrapping_add(0x1b8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x1b8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x20u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x20u64))?;
            memory.write_u64((values[1]).wrapping_add(0x1c0u64), stored)?;
        }
        {
            let stored = memory.read_u64((values[1]).wrapping_add(0x1c0u64))?;
            memory.write_u64(memory.read_u64((values[1]).wrapping_add(0x1b0u64))?, stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x1c0u64);
            memory.write_u64((values[1]).wrapping_add(0x1c8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x1c8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x1d0u64), stored)?;
        }
        {
            let stored =
                (memory.read_u64((values[1]).wrapping_add(0x40u64))?).wrapping_add(0x1b0u64);
            memory.write_u64((values[1]).wrapping_add(0x1d8u64), stored)?;
        }
        {
            let stored = memory.read_u64(memory.read_u64((values[1]).wrapping_add(0x1d8u64))?)?;
            memory.write_u64((values[1]).wrapping_add(0x1e0u64), stored)?;
        }
        invoke(
            "FUN_1400b022c",
            &[
                memory.read_u64((values[1]).wrapping_add(0x1e0u64))?,
                memory.read_u64((values[1]).wrapping_add(0x1d0u64))?,
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
    ("FUN_140001000", 0, fun_140001000),
    ("FUN_140001050", 0, fun_140001050),
    ("FUN_140001060", 0, fun_140001060),
    ("FUN_1400012c0", 0, fun_1400012c0),
    ("FUN_1400012d0", 0, fun_1400012d0),
    ("FUN_1400013f0", 0, fun_1400013f0),
    ("FUN_140001410", 0, fun_140001410),
    ("FUN_140001420", 0, fun_140001420),
    ("FUN_140001b80", 1, fun_140001b80),
    ("FUN_140003190", 2, fun_140003190),
    ("FUN_1400031f0", 2, fun_1400031f0),
    ("FUN_140003360", 2, fun_140003360),
    ("FUN_1400033b0", 2, fun_1400033b0),
    ("FUN_140004ee0", 2, fun_140004ee0),
    ("FUN_140009170", 2, fun_140009170),
    ("FUN_1400092a0", 3, fun_1400092a0),
    ("FUN_140009590", 2, fun_140009590),
    ("FUN_140009a80", 1, fun_140009a80),
    ("FUN_14000a2b0", 3, fun_14000a2b0),
    ("FUN_14000a580", 1, fun_14000a580),
    ("FUN_14000b590", 2, fun_14000b590),
    ("FUN_14000c540", 2, fun_14000c540),
    ("FUN_14000c650", 3, fun_14000c650),
    ("FUN_14000ca50", 1, fun_14000ca50),
    ("FUN_14000d1b0", 2, fun_14000d1b0),
    ("FUN_14000f390", 1, fun_14000f390),
    ("FUN_140013210", 1, fun_140013210),
    ("FUN_140014d50", 2, fun_140014d50),
    ("FUN_140015560", 2, fun_140015560),
    ("FUN_1400172a0", 1, fun_1400172a0),
    ("FUN_140017d90", 2, fun_140017d90),
    ("FUN_140017f70", 1, fun_140017f70),
    ("FUN_1400199e0", 1, fun_1400199e0),
    ("FUN_140019bb0", 2, fun_140019bb0),
    ("FUN_14001ac00", 0, fun_14001ac00),
    ("FUN_14001d660", 2, fun_14001d660),
    ("FUN_14001e2d0", 2, fun_14001e2d0),
    ("FUN_14001ef50", 1, fun_14001ef50),
    ("FUN_14001f480", 2, fun_14001f480),
    ("FUN_14001f4a0", 2, fun_14001f4a0),
    ("FUN_14001f600", 1, fun_14001f600),
    ("FUN_140022ba0", 2, fun_140022ba0),
    ("FUN_140022cc0", 1, fun_140022cc0),
    ("FUN_1400230b0", 2, fun_1400230b0),
    ("FUN_140024ba0", 2, fun_140024ba0),
    ("FUN_140024db0", 2, fun_140024db0),
    ("FUN_140024fc0", 2, fun_140024fc0),
    ("FUN_140033540", 2, fun_140033540),
    ("FUN_140033bf0", 2, fun_140033bf0),
    ("FUN_140033c10", 2, fun_140033c10),
    ("FUN_140033c50", 2, fun_140033c50),
    ("FUN_140033cd0", 2, fun_140033cd0),
    ("FUN_140033d10", 2, fun_140033d10),
    ("FUN_140036c90", 2, fun_140036c90),
    ("FUN_140036cb0", 2, fun_140036cb0),
    ("FUN_140036ec0", 2, fun_140036ec0),
    ("FUN_140037010", 1, fun_140037010),
    ("FUN_140037380", 2, fun_140037380),
    ("FUN_14003bde0", 2, fun_14003bde0),
    ("FUN_14003c730", 2, fun_14003c730),
    ("FUN_140041950", 2, fun_140041950),
    ("FUN_140041980", 2, fun_140041980),
    ("FUN_1400428b0", 2, fun_1400428b0),
    ("FUN_140042dd0", 2, fun_140042dd0),
    ("FUN_140042ed0", 1, fun_140042ed0),
    ("FUN_1400477f0", 0, fun_1400477f0),
    ("FUN_1400487d0", 2, fun_1400487d0),
    ("FUN_14004ab80", 2, fun_14004ab80),
    ("FUN_14004aba0", 2, fun_14004aba0),
    ("FUN_14004abc0", 2, fun_14004abc0),
    ("FUN_14004ac00", 2, fun_14004ac00),
    ("FUN_14004ac80", 2, fun_14004ac80),
    ("FUN_14004acc0", 2, fun_14004acc0),
    ("FUN_14004ad80", 2, fun_14004ad80),
    ("FUN_14004adf0", 2, fun_14004adf0),
    ("FUN_14004bdf0", 2, fun_14004bdf0),
    ("FUN_1400532e0", 0, fun_1400532e0),
    ("FUN_140053300", 0, fun_140053300),
    ("FUN_1400534a0", 2, fun_1400534a0),
    ("FUN_140055b10", 2, fun_140055b10),
    ("FUN_140056040", 3, fun_140056040),
    ("FUN_1400562d0", 2, fun_1400562d0),
    ("FUN_140056510", 3, fun_140056510),
    ("FUN_140056ca0", 3, fun_140056ca0),
    ("FUN_1400571a0", 2, fun_1400571a0),
    ("FUN_140057280", 1, fun_140057280),
    ("FUN_140057ed0", 1, fun_140057ed0),
    ("FUN_140058000", 3, fun_140058000),
    ("FUN_140059980", 2, fun_140059980),
    ("FUN_140062480", 1, fun_140062480),
    ("FUN_140064fe0", 2, fun_140064fe0),
    ("FUN_1400657c0", 2, fun_1400657c0),
    ("FUN_140067ff0", 3, fun_140067ff0),
    ("FUN_140076880", 2, fun_140076880),
    ("FUN_140077770", 3, fun_140077770),
    ("FUN_140077d10", 3, fun_140077d10),
    ("FUN_1400781c0", 2, fun_1400781c0),
    ("FUN_140079330", 2, fun_140079330),
    ("FUN_14007a5e0", 2, fun_14007a5e0),
    ("FUN_14008f670", 0, fun_14008f670),
    ("FUN_140094f60", 2, fun_140094f60),
    ("FUN_140096ec0", 2, fun_140096ec0),
    ("FUN_140096ee0", 2, fun_140096ee0),
    ("FUN_1400970e0", 2, fun_1400970e0),
    ("FUN_1400982d0", 2, fun_1400982d0),
    ("FUN_1400999a0", 2, fun_1400999a0),
    ("FUN_1400999c0", 2, fun_1400999c0),
    ("FUN_140099a30", 2, fun_140099a30),
    ("FUN_140099a50", 2, fun_140099a50),
    ("FUN_140099ac0", 2, fun_140099ac0),
    ("FUN_140099b30", 2, fun_140099b30),
    ("FUN_14009a2a0", 2, fun_14009a2a0),
    ("FUN_14009b420", 2, fun_14009b420),
    ("FUN_1400a32f0", 1, fun_1400a32f0),
    ("FUN_1400a4fb0", 2, fun_1400a4fb0),
    ("FUN_1400a4fd0", 2, fun_1400a4fd0),
    ("FUN_1400a5530", 2, fun_1400a5530),
    ("FUN_1400a5bb0", 2, fun_1400a5bb0),
    ("FUN_1400a5c40", 2, fun_1400a5c40),
    ("FUN_1400a5c80", 2, fun_1400a5c80),
    ("FUN_1400a6e50", 2, fun_1400a6e50),
    ("FUN_1400a6e60", 2, fun_1400a6e60),
    ("FUN_1400a8480", 2, fun_1400a8480),
    ("FUN_1400a9340", 2, fun_1400a9340),
    ("FUN_1400adf40", 2, fun_1400adf40),
    ("FUN_1400ae040", 2, fun_1400ae040),
    ("FUN_1400ae060", 2, fun_1400ae060),
    ("FUN_1400b08c8", 0, fun_1400b08c8),
    ("FUN_1400b08e4", 0, fun_1400b08e4),
    ("FUN_1400b8320", 1, fun_1400b8320),
    ("FUN_1400b8560", 1, fun_1400b8560),
    ("FUN_1400b9340", 1, fun_1400b9340),
    ("FUN_1400b9350", 1, fun_1400b9350),
    ("FUN_1400ba220", 1, fun_1400ba220),
    ("FUN_1400bab90", 1, fun_1400bab90),
    ("FUN_1400bb0e0", 1, fun_1400bb0e0),
    ("FUN_1400bba00", 1, fun_1400bba00),
    ("FUN_1400bbf50", 1, fun_1400bbf50),
    ("FUN_1400bc1f0", 1, fun_1400bc1f0),
    ("FUN_1400bc4a0", 1, fun_1400bc4a0),
    ("FUN_1400bc890", 1, fun_1400bc890),
    ("FUN_1400bede0", 1, fun_1400bede0),
    ("FUN_1400bf360", 1, fun_1400bf360),
    ("FUN_1400bff00", 1, fun_1400bff00),
    ("FUN_1400c0250", 1, fun_1400c0250),
    ("FUN_1400c0b40", 1, fun_1400c0b40),
    ("FUN_1400c0db0", 1, fun_1400c0db0),
    ("FUN_1400c1b60", 1, fun_1400c1b60),
    ("FUN_1400c2510", 1, fun_1400c2510),
    ("FUN_1400c2df0", 1, fun_1400c2df0),
    ("FUN_1400c3090", 1, fun_1400c3090),
    ("FUN_1400c30a0", 1, fun_1400c30a0),
    ("FUN_1400c3a30", 1, fun_1400c3a30),
    ("FUN_1400c3cc0", 1, fun_1400c3cc0),
    ("FUN_1400c4050", 1, fun_1400c4050),
    ("FUN_1400c4950", 1, fun_1400c4950),
    ("FUN_1400c6480", 1, fun_1400c6480),
    ("FUN_1400c6a20", 1, fun_1400c6a20),
    ("FUN_1400c6d30", 1, fun_1400c6d30),
    ("FUN_1400c6fd0", 1, fun_1400c6fd0),
    ("FUN_1400c6fe0", 1, fun_1400c6fe0),
    ("FUN_1400c7ea0", 1, fun_1400c7ea0),
    ("FUN_1400c80f0", 1, fun_1400c80f0),
    ("FUN_1400c85b0", 1, fun_1400c85b0),
    ("FUN_1400c8c20", 1, fun_1400c8c20),
    ("FUN_1400c91c0", 1, fun_1400c91c0),
    ("FUN_1400c9d40", 1, fun_1400c9d40),
    ("FUN_1400ca4f0", 1, fun_1400ca4f0),
    ("FUN_1400cacd0", 1, fun_1400cacd0),
    ("FUN_1400cb2b0", 1, fun_1400cb2b0),
    ("FUN_1400cb670", 1, fun_1400cb670),
    ("FUN_1400cb900", 1, fun_1400cb900),
    ("FUN_1400cbec4", 4, fun_1400cbec4),
    ("FUN_1400cc330", 2, fun_1400cc330),
    ("FUN_1400cc818", 2, fun_1400cc818),
    ("FUN_1400cc8a0", 2, fun_1400cc8a0),
    ("FUN_1400ccac8", 2, fun_1400ccac8),
    ("FUN_1400ccd2c", 2, fun_1400ccd2c),
    ("FUN_1400cd2bc", 2, fun_1400cd2bc),
    ("FUN_1400cd363", 2, fun_1400cd363),
    ("FUN_1400cd3ee", 2, fun_1400cd3ee),
    ("FUN_1400cd888", 2, fun_1400cd888),
    ("FUN_1400cd9a4", 2, fun_1400cd9a4),
    ("FUN_1400cda6c", 2, fun_1400cda6c),
    ("FUN_1400cdb0c", 2, fun_1400cdb0c),
    ("FUN_1400cde82", 2, fun_1400cde82),
    ("FUN_1400ce4f0", 2, fun_1400ce4f0),
    ("FUN_1400ceb60", 2, fun_1400ceb60),
    ("FUN_1400cf11a", 2, fun_1400cf11a),
    ("FUN_1400cf426", 2, fun_1400cf426),
    ("FUN_1400cf4f8", 2, fun_1400cf4f8),
    ("FUN_1400d235c", 2, fun_1400d235c),
    ("FUN_1400d25e4", 2, fun_1400d25e4),
    ("FUN_1400d2a2c", 2, fun_1400d2a2c),
    ("FUN_1400d2aa0", 2, fun_1400d2aa0),
    ("FUN_1400d2b48", 2, fun_1400d2b48),
    ("FUN_1400d2ca4", 2, fun_1400d2ca4),
    ("FUN_1400d2dd0", 2, fun_1400d2dd0),
    ("FUN_1400d3560", 2, fun_1400d3560),
    ("FUN_1400d3c28", 2, fun_1400d3c28),
    ("FUN_1400d5c9e", 2, fun_1400d5c9e),
    ("FUN_1400d5e08", 2, fun_1400d5e08),
    ("FUN_1400d61f8", 2, fun_1400d61f8),
    ("FUN_1400d6280", 2, fun_1400d6280),
    ("FUN_1400d632c", 2, fun_1400d632c),
    ("FUN_1400d6650", 2, fun_1400d6650),
    ("FUN_1400d6723", 2, fun_1400d6723),
    ("FUN_1400d6778", 2, fun_1400d6778),
    ("FUN_1400d830c", 1, fun_1400d830c),
    ("FUN_1400d8348", 2, fun_1400d8348),
    ("FUN_1400d8570", 2, fun_1400d8570),
    ("FUN_1400d8bfb", 2, fun_1400d8bfb),
    ("FUN_1400d8fd1", 2, fun_1400d8fd1),
    ("FUN_1400d94f0", 2, fun_1400d94f0),
    ("FUN_1400d9760", 2, fun_1400d9760),
    ("FUN_1400d9b20", 2, fun_1400d9b20),
    ("FUN_1400da4b9", 2, fun_1400da4b9),
    ("FUN_1400db514", 2, fun_1400db514),
    ("FUN_1400db9af", 2, fun_1400db9af),
];
