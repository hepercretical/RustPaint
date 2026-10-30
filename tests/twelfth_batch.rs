use mspaint_port::recovered::{adapters12, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-012-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-012.csv");

fn frame_address(function: &str, symbol: &str, anchor: u64) -> u64 {
    let offsets = adapters12::FRAME_OFFSETS
        .iter()
        .find(|x| x.0 == function)
        .unwrap()
        .1;
    let offset = offsets.iter().find(|x| x.0 == symbol).unwrap().1;
    anchor.checked_add_signed(i64::from(offset)).unwrap()
}

#[test]
fn exception_copy_observes_adjacent_pointer_and_byte_fields() {
    for base in [BASE, 0x1234_5678_0000] {
        let anchor = base + 9000;
        let mut memory = image(1, base, "");
        memory
            .write_u64(base + 16000, 0x1234_5678_9abc_def0)
            .unwrap();
        let mut calls = 0;
        let result = adapters12::fun_140001df0(
            &[base + 1024, 0xfeed],
            &mut memory,
            &mut |symbol| match symbol {
                "DAT_140179040" => base + 16000,
                "std::exception::vftable" => 0x1111,
                _ => frame_address("FUN_140001df0", symbol, anchor),
            },
            &mut |symbol, args, memory| {
                calls += 1;
                assert_eq!(symbol, "__std_exception_copy");
                assert_eq!(args, &[anchor - 0x28]);
                assert_eq!(memory.read_u64(args[0]), Ok(0xfeed));
                assert_eq!(memory.read_u8(args[0] + 8), Ok(1));
                assert_eq!(memory.read_u8(args[0] + 9), Ok(255));
                assert_eq!(
                    memory.read_u64(anchor - 0x18),
                    Ok(0x1234_5678_9abc_def0 ^ (anchor - 0x48))
                );
                assert_eq!(memory.read_u64(base + 1024), Ok(0x1111));
                assert_eq!(memory.read_u64(base + 1032), Ok(0));
                assert_eq!(memory.read_u64(base + 1040), Ok(0));
                memory.bytes[100] = 0x42;
                0
            },
        );
        assert_eq!(result, Ok(Some(base + 1024)));
        assert_eq!(calls, 1);
        assert_eq!(memory.bytes[100], 0x42);
    }
}

#[test]
fn rectangle_exception_passes_four_contiguous_words_to_its_constructor() {
    let anchor = BASE + 9000;
    let mut memory = image(0, BASE, "");
    let words = [0xffff_ffff, 0x8000_0000, 0x1234_5678, 0x7fff_ffff];
    for (index, word) in words.into_iter().enumerate() {
        memory
            .write_u32(BASE + 512 + index as u64 * 4, word)
            .unwrap();
    }
    let mut calls = Vec::new();
    let result = adapters12::fun_140019d80(
        &[0, BASE + 512, 0xfeed],
        &mut memory,
        &mut |symbol| match symbol {
            "\"exists\"" => BASE + 15000,
            "DAT_14016acd0" => BASE + 16000,
            _ => frame_address("FUN_140019d80", symbol, anchor),
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "FUN_140019900" => {
                    assert_eq!(args, &[anchor - 0xb8, BASE + 15000]);
                    memory.write_u64(args[0], 0xbeef).unwrap();
                }
                "FUN_140019650" => {
                    assert_eq!(args, &[anchor - 0x98, anchor - 0xb8, 0xfeed, anchor - 0xc8]);
                    for (index, word) in words.into_iter().enumerate() {
                        assert_eq!(memory.read_u32(args[3] + index as u64 * 4), Ok(word));
                    }
                    assert_eq!(memory.read_u64(args[1]), Ok(0xbeef));
                    memory.write_u64(args[0], 0xabcd).unwrap();
                }
                "_CxxThrowException" => {
                    assert_eq!(args, &[anchor - 0x98, BASE + 16000]);
                    assert_eq!(memory.read_u64(args[0]), Ok(0xabcd));
                }
                _ => panic!("unexpected dependency"),
            }
            0
        },
    );
    assert_eq!(
        result,
        Err(adapters12::AdapterError::Halted("_CxxThrowException"))
    );
    assert_eq!(
        calls,
        ["FUN_140019900", "FUN_140019650", "_CxxThrowException"]
    );
}

#[test]
fn callback_output_is_loaded_after_the_call_and_invalid_spans_keep_prior_effects() {
    for bad_output in [false, true] {
        let mut memory = image(0, BASE, "");
        let anchor = BASE + 9000;
        let output = if bad_output {
            u64::MAX - 3
        } else {
            anchor - 0x28
        };
        let result = adapters12::fun_1400415b0(
            &[BASE + 128, 0xfeed, BASE + 512],
            &mut memory,
            &mut |symbol| match symbol {
                "DAT_140179040" => BASE + 16000,
                "STACK_local_28" => output,
                _ => frame_address("FUN_1400415b0", symbol, anchor),
            },
            &mut |symbol, args, memory| {
                assert_eq!(symbol, "FUN_1400036a0");
                assert_eq!(args, &[BASE + 128 + 0x38, output, anchor - 0x20]);
                assert_eq!(memory.read_u64(args[2]), Ok(0xfeed));
                memory.bytes[100] = 0x42;
                if !bad_output {
                    memory.write_u64(output, 0x1234).unwrap();
                }
                0
            },
        );
        if bad_output {
            assert_eq!(
                result,
                Err(adapters12::AdapterError::Memory(
                    mspaint_port::recovered::call_support::MemoryFault {
                        address: output,
                        width: 8
                    }
                ))
            );
            assert_eq!(memory.read_u64(BASE + 512), Ok(0));
        } else {
            assert_eq!(result, Ok(Some(0)));
            assert_eq!(memory.read_u64(BASE + 512), Ok(0x1234));
        }
        assert_eq!(memory.bytes[100], 0x42);
        assert_eq!(memory.read_u64(anchor - 0x20), Ok(0xfeed));
    }
}

#[test]
fn partial_word_update_preserves_high_pointer_bits_and_releases_before_delegating() {
    let base = 0x1234_5678_0000;
    let anchor = base + 9000;
    let object = base + 128;
    let lock = object + 0x48;
    let status = 0x8000_4005;
    let mut memory = image(0, base, "");
    let mut calls = Vec::new();
    let result = adapters12::fun_1400097e0(
        &[object + 16, 0xfeed],
        &mut memory,
        &mut |symbol| {
            if symbol == "DAT_140179040" {
                base + 16000
            } else {
                frame_address("FUN_1400097e0", symbol, anchor)
            }
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "AcquireSRWLockExclusive" => {
                    assert_eq!(args, &[lock]);
                    assert_eq!(memory.read_u64(anchor - 0x68), Ok(lock));
                    memory.write_u32(object + 0x60, status).unwrap();
                }
                "ReleaseSRWLockExclusive" => assert_eq!(args, &[lock]),
                "FUN_140009eb0" => {
                    assert_eq!(args, &[anchor - 0x30, object, anchor - 0x68]);
                    assert_eq!(memory.read_u64(args[0]), Ok(0xfeed));
                    assert_eq!(
                        memory.read_u64(args[2]),
                        Ok((lock & 0xffff_ffff_0000_0000) | u64::from(status))
                    );
                }
                _ => panic!("unexpected dependency {symbol}"),
            }
            0
        },
    );
    assert_eq!(result, Ok(Some(0)));
    assert_eq!(memory.read_u8(object + 100), Ok(1));
    assert_eq!(
        calls,
        [
            "AcquireSRWLockExclusive",
            "ReleaseSRWLockExclusive",
            "FUN_140009eb0"
        ]
    );
}

#[test]
fn already_initialized_guard_builds_terminal_error_without_extra_cleanup() {
    let anchor = BASE + 9000;
    let object = BASE + 128;
    let mut memory = image(0, BASE, "");
    memory.write_u8(object + 100, 1).unwrap();
    let mut calls = Vec::new();
    let result = adapters12::fun_1400097e0(
        &[object + 16, 0],
        &mut memory,
        &mut |symbol| match symbol {
            "DAT_140179040" => BASE + 16000,
            "DAT_14016a900" => BASE + 15000,
            _ if symbol.starts_with('"') => BASE + 14000,
            _ => frame_address("FUN_1400097e0", symbol, anchor),
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "AcquireSRWLockExclusive" => assert_eq!(args, &[object + 0x48]),
                "FUN_14000a900" => {
                    assert_eq!(args, &[anchor - 0x48, anchor - 0x60]);
                    assert_eq!(memory.read_u32(args[1]), Ok(0x155a));
                    assert_eq!(memory.read_u64(args[1] + 8), Ok(BASE + 14000));
                    assert_eq!(memory.read_u64(args[1] + 16), Ok(0));
                    memory.write_u32(args[0], 0x8000_4005).unwrap();
                }
                "_CxxThrowException" => {
                    assert_eq!(args, &[anchor - 0x48, BASE + 15000]);
                    assert_eq!(memory.read_u32(args[0]), Ok(0x8000_4005));
                }
                _ => panic!("unexpected dependency {symbol}"),
            }
            0
        },
    );
    assert_eq!(
        result,
        Err(adapters12::AdapterError::Halted("_CxxThrowException"))
    );
    assert_eq!(
        calls,
        [
            "AcquireSRWLockExclusive",
            "FUN_14000a900",
            "_CxxThrowException"
        ]
    );
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
fn all_219_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 1752);
    assert_eq!(cases.len(), 3504);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters12::HANDLERS.iter().find(|x| x.0 == name).unwrap();
        let globals = bindings(i[5]);
        let returned_pointers = bindings(i[6]);
        for (variant, base) in [BASE, 0x1234_5678_0000].into_iter().enumerate() {
            let f: Vec<_> = expected[variant].split(',').collect();
            assert_eq!((&i[0], &i[1]), (&f[0], &f[1]));
            assert_eq!(f[6], variant.to_string());
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
                    matches!(result, Err(adapters12::AdapterError::Halted(_))),
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
    assert_eq!(adapters12::HANDLERS.len(), 219);
}

#[test]
fn manifest_reaches_fifty_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 219);
    let manifest = include_str!("../translations/batch-012.json");
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
        include_str!("../translations/batch-011.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 219);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 219;
    assert_eq!(total, 2196);
    assert!(total * 100 >= 4392 * 50);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters12::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters12::HANDLERS {
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
