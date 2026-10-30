use mspaint_port::recovered::{adapters11, call_support::MemoryImage};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    fmt::Write,
};
const BASE: u64 = 0x100_0000;
const SIZE: usize = 16384;
const PAYLOADS: [u64; 8] = [
    0,
    1,
    u64::MAX,
    0x8000_0000_0000_0000,
    0x1234_5678_9abc_def0,
    0x1_0000_0001,
    BASE,
    0x1122_3344_5566_7788,
];
const INPUTS: &str = include_str!("fixtures/batch-011-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-011.csv");

#[test]
fn addressable_scratch_is_initialized_before_the_terminal_throw() {
    let mut memory = image(1, BASE, "");
    let mut calls = Vec::new();
    let result = adapters11::fun_140005800(
        &[],
        &mut memory,
        &mut |symbol| match symbol {
            "STACK_local_28" => BASE + 1024,
            "DAT_14016ab38" => BASE + 2048,
            _ => panic!("unexpected symbol {symbol}"),
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "FUN_140005790" => {
                    assert_eq!(args, &[BASE + 1024]);
                    memory.write_u64(args[0], 0x1234).unwrap();
                }
                "_CxxThrowException" => {
                    assert_eq!(args, &[BASE + 1024, BASE + 2048]);
                    assert_eq!(memory.read_u64(args[0]), Ok(0x1234));
                    memory.bytes[25] = 0x42;
                }
                _ => panic!("unexpected dependency"),
            }
            0
        },
    );
    assert_eq!(
        result,
        Err(adapters11::AdapterError::Halted("_CxxThrowException"))
    );
    assert_eq!(calls, ["FUN_140005790", "_CxxThrowException"]);
    assert_eq!(memory.bytes[25], 0x42);
}

#[test]
fn signed_failure_terminates_without_initializing_the_output() {
    for status in [0, 1, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0x1_0000_0000] {
        let mut memory = image(1, BASE, "");
        let before = memory.clone();
        let mut calls = 0;
        let result = adapters11::fun_140002770(
            &[BASE, status, 0x1234],
            &mut memory,
            &mut |_| panic!("unexpected lookup"),
            &mut |name, args, _| {
                calls += 1;
                assert_eq!(name, "FUN_140002540");
                assert_eq!(args, &[status as i32 as i64 as u64, 0x1234]);
                0
            },
        );
        if (status as i32) < 0 {
            assert_eq!(
                result,
                Err(adapters11::AdapterError::Halted("FUN_140002540"))
            );
            assert_eq!(calls, 1);
            assert_eq!(memory, before);
        } else {
            assert_eq!(result, Ok(Some(BASE)));
            assert_eq!(calls, 0);
            assert_eq!(memory.read_u32(BASE), Ok(status as u32));
            assert_eq!(&memory.bytes[4..], &before.bytes[4..]);
        }
    }
}

#[test]
fn virtual_cleanup_clears_ownership_after_the_callback_returns() {
    for owned in [BASE, BASE + 1024] {
        let mut memory = image(0, BASE, "");
        memory.write_u64(BASE + 56, owned).unwrap();
        memory.write_u64(owned, BASE + 2048).unwrap();
        memory.write_u64(BASE + 2048 + 32, 0xfeed).unwrap();
        let mut calls = 0;
        let result = adapters11::fun_140006680(
            &[BASE],
            &mut memory,
            &mut |_| panic!("unexpected lookup"),
            &mut |name, args, memory| {
                calls += 1;
                assert_eq!(name, "INDIRECT_CALL");
                assert_eq!(args, &[0xfeed, owned, u64::from(owned != BASE)]);
                assert_eq!(memory.read_u64(BASE + 56), Ok(owned));
                memory.bytes[100] = 0x42;
                0
            },
        );
        assert_eq!(result, Ok(None));
        assert_eq!(calls, 1);
        assert_eq!(memory.read_u64(BASE + 56), Ok(0));
        assert_eq!(memory.bytes[100], 0x42);
        memory.write_u64(BASE + 56, owned).unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            adapters11::fun_140006680(
                &[BASE],
                &mut memory,
                &mut |_| panic!("unexpected lookup"),
                &mut |_, _, memory| {
                    memory.bytes[101] = 0x43;
                    panic!("cleanup failed")
                },
            )
        }));
        assert!(result.is_err());
        assert_eq!(memory.read_u64(BASE + 56), Ok(owned));
        assert_eq!(memory.bytes[101], 0x43);
    }
}

#[test]
fn returned_output_pointer_is_read_after_callback_mutation_and_checked() {
    for returned in [BASE + 1024, u64::MAX - 1] {
        let mut memory = image(0, BASE, "");
        let result = adapters11::fun_140009560(
            &[BASE + 24, BASE + 512],
            &mut memory,
            &mut |symbol| {
                assert_eq!(symbol, "STACK_local_res8");
                BASE + 1024
            },
            &mut |name, args, memory| {
                assert_eq!(name, "FUN_140009d60");
                assert_eq!(args, &[BASE, BASE + 1024]);
                memory.write_u32(BASE + 1024, 0x8000_4005).unwrap();
                returned
            },
        );
        if returned == BASE + 1024 {
            assert_eq!(result, Ok(Some(0)));
            assert_eq!(memory.read_u32(BASE + 512), Ok(0x8000_4005));
        } else {
            assert_eq!(
                result,
                Err(adapters11::AdapterError::Memory(
                    mspaint_port::recovered::call_support::MemoryFault {
                        address: returned,
                        width: 4
                    }
                ))
            );
            assert_eq!(memory.read_u32(BASE + 512), Ok(0));
        }
        assert_eq!(memory.read_u32(BASE + 1024), Ok(0x8000_4005));
    }
}

#[test]
fn allocated_sentinel_preserves_global_pointer_and_exact_field_widths() {
    for returned in [BASE + 1024, u64::MAX - 3] {
        let mut memory = image(1, BASE, "");
        let mut calls = Vec::new();
        let result = adapters11::fun_140001010(
            &[],
            &mut memory,
            &mut |name| match name {
                "DAT_1401d0510" => BASE + 512,
                "FUN_1400df750" => 0xfeed,
                _ => panic!("unexpected lookup"),
            },
            &mut |name, args, memory| {
                calls.push(name);
                match name {
                    "operator_new" => {
                        assert_eq!(args, &[0x30]);
                        returned
                    }
                    "atexit" => {
                        assert_eq!(args, &[0xfeed]);
                        for offset in [0, 8, 16] {
                            assert_eq!(memory.read_u64(returned + offset), Ok(returned));
                        }
                        assert_eq!(memory.read_u16(returned + 24), Ok(0x101));
                        assert_eq!(memory.read_u16(returned + 26), Ok(0xffff));
                        0
                    }
                    _ => panic!("unexpected dependency"),
                }
            },
        );
        assert_eq!(memory.read_u64(BASE + 512), Ok(returned));
        if returned == BASE + 1024 {
            assert_eq!(result, Ok(None));
            assert_eq!(calls, ["operator_new", "atexit"]);
        } else {
            assert_eq!(
                result,
                Err(adapters11::AdapterError::Memory(
                    mspaint_port::recovered::call_support::MemoryFault {
                        address: returned,
                        width: 8
                    }
                ))
            );
            assert_eq!(calls, ["operator_new"]);
        }
    }
}
fn hex(s: &str) -> u64 {
    u64::from_str_radix(s, 16).unwrap()
}
fn symbol_hex(s: &str) -> String {
    s.bytes().map(|b| format!("{b:02x}")).collect()
}
fn from_hex(s: &str) -> String {
    let (pairs, tail) = s.as_bytes().as_chunks::<2>();
    assert!(tail.is_empty());
    String::from_utf8(
        pairs
            .iter()
            .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
            .collect(),
    )
    .unwrap()
}
fn bindings(s: &str) -> BTreeMap<String, usize> {
    s.split(';')
        .filter(|s| !s.is_empty())
        .map(|item| {
            let (name, offset) = item.split_once(':').unwrap();
            (from_hex(name), offset.parse().unwrap())
        })
        .collect()
}
fn canonical(value: u64, base: u64) -> u64 {
    match value.checked_sub(base) {
        Some(offset) if offset <= SIZE as u64 => BASE + offset,
        _ => value,
    }
}
fn canonical_image(memory: &MemoryImage) -> Vec<u8> {
    memory
        .bytes
        .as_chunks::<8>()
        .0
        .iter()
        .flat_map(|w| canonical(u64::from_le_bytes(*w), memory.base).to_le_bytes())
        .collect()
}
fn fingerprint(memory: &MemoryImage) -> u64 {
    canonical_image(memory)
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |h, b| {
            (h ^ u64::from(*b)).wrapping_mul(0x100_0000_01b3)
        })
}
fn image(seed: usize, base: u64, pointers: &str) -> MemoryImage {
    let mut memory = MemoryImage {
        base,
        bytes: (0..SIZE)
            .map(|i| match seed {
                0 => 0,
                1 => 255,
                _ => (i * 17 + seed * 31) as u8,
            })
            .collect(),
    };
    for pointer in pointers.split(';').filter(|s| !s.is_empty()) {
        let (at, target) = pointer.split_once(':').unwrap();
        memory
            .write_u64(
                base + at.parse::<u64>().unwrap(),
                base + target.parse::<u64>().unwrap(),
            )
            .unwrap();
    }
    memory
}

#[test]
fn all_220_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 1760);
    assert_eq!(cases.len(), 1760);
    for (input, expected) in inputs.into_iter().zip(cases) {
        let i: Vec<_> = input.split(',').collect();
        let f: Vec<_> = expected.split(',').collect();
        assert_eq!((&i[0], &i[1]), (&f[0], &f[1]));
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters11::HANDLERS.iter().find(|x| x.0 == name).unwrap();
        let globals = bindings(i[5]);
        let returned_pointers = bindings(i[6]);
        for base in [BASE, 0x1234_5678_0000] {
            let mut memory = image(seed, base, i[4]);
            let args: Vec<_> = i[2]
                .split(';')
                .filter(|s| !s.is_empty())
                .zip(i[3].split(';'))
                .map(|(value, offset)| {
                    if offset == "-" {
                        hex(value)
                    } else {
                        base + offset.parse::<u64>().unwrap()
                    }
                })
                .collect();
            assert_eq!(args.len(), *arity);
            assert_eq!(
                fingerprint(&memory),
                hex(f[3]),
                "initial state of {name}, seed {seed}"
            );
            let mut expected_image = canonical_image(&memory);
            for change in f[5].split(';').filter(|s| !s.is_empty()) {
                let (at, value) = change.split_once(':').unwrap();
                expected_image[hex(at) as usize] = hex(value) as u8;
            }
            let trace = RefCell::new(String::new());
            let calls = Cell::new(0usize);
            let result = function(
                &args,
                &mut memory,
                &mut |symbol| {
                    let value = globals
                        .get(symbol)
                        .map_or(PAYLOADS[(seed + 3) % 8], |&at| base + at as u64);
                    write!(
                        trace.borrow_mut(),
                        "R:{}:{:x}|",
                        symbol_hex(symbol),
                        canonical(value, base)
                    )
                    .unwrap();
                    value
                },
                &mut |symbol, args, memory| {
                    let mut trace = trace.borrow_mut();
                    write!(trace, "C:{}:", symbol_hex(symbol)).unwrap();
                    for &arg in args {
                        write!(trace, "{:x};", canonical(arg, base)).unwrap();
                    }
                    write!(trace, ":{:x}|", fingerprint(memory)).unwrap();
                    let index = calls.get();
                    calls.set(index + 1);
                    memory.bytes[SIZE - 16] ^= (index + 1) as u8;
                    returned_pointers
                        .get(symbol)
                        .map_or(PAYLOADS[seed] ^ (index as u64 * 0x101), |&at| {
                            base + at as u64
                        })
                },
            );
            let result = if f[2] == "!" {
                assert!(
                    matches!(result, Err(adapters11::AdapterError::Halted(_))),
                    "halt of {name}, seed {seed}: {result:?}"
                );
                None
            } else {
                result.unwrap()
            };
            assert_eq!(
                result.map(|v| canonical(v, base)),
                if f[2] == "-" || f[2] == "!" {
                    None
                } else {
                    Some(hex(f[2]))
                },
                "return of {name}, seed {seed}"
            );
            assert_eq!(
                *trace.borrow(),
                f[4],
                "events of {name}, seed {seed}, base {base:x}"
            );
            assert_eq!(
                canonical_image(&memory),
                expected_image,
                "final state of {name}, seed {seed}, base {base:x}"
            );
        }
    }
    assert_eq!(adapters11::HANDLERS.len(), 220);
}

#[test]
fn manifest_reaches_forty_five_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 220);
    let manifest = include_str!("../translations/batch-011.json");
    let prior = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
        include_str!("../translations/batch-004.json"),
        include_str!("../translations/batch-005.json"),
        include_str!("../translations/batch-006.json"),
        include_str!("../translations/batch-007.json"),
        include_str!("../translations/batch-008.json"),
        include_str!("../translations/batch-009.json"),
        include_str!("../translations/batch-010.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 220);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 220;
    assert_eq!(total, 1977);
    assert!(total * 100 >= 4392 * 45);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters11::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters11::HANDLERS {
        let args = vec![0; arity];
        for available in 0..arity {
            let mut memory = image(2, BASE, "");
            let before = memory.clone();
            let result = function(
                &args[..available],
                &mut memory,
                &mut |_| panic!("unexpected lookup"),
                &mut |_, _, _| panic!("unexpected call"),
            );
            assert_eq!(
                result,
                Err(AdapterError::Arguments(MissingArguments {
                    required: arity,
                    available
                })),
                "{name}"
            );
            assert_eq!(memory, before);
        }
    }
}
