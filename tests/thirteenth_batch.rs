use mspaint_port::recovered::{adapters13, call_support::MemoryImage};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    fmt::Write,
};
const BASE: u64 = 0x100_0000;
const SIZE: usize = 32768;
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
const INPUTS: &str = include_str!("fixtures/batch-013-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-013.csv");

fn frame_address(function: &str, symbol: &str, anchor: u64) -> u64 {
    let offsets = adapters13::FRAME_OFFSETS
        .iter()
        .find(|x| x.0 == function)
        .unwrap()
        .1;
    let offset = offsets.iter().find(|x| x.0 == symbol).unwrap().1;
    anchor.checked_add_signed(i64::from(offset)).unwrap()
}

#[test]
fn reference_release_preserves_wrapping_counts_and_orders_free_before_pointer_clear() {
    for count in [0u32, 1, 2, 0x7fff_ffff, 0x8000_0000, u32::MAX] {
        let mut memory = image(0, BASE, "");
        let allocation = BASE + 1024;
        memory.write_u64(BASE, allocation).unwrap();
        memory.write_u32(allocation + 24, count).unwrap();
        let mut calls = Vec::new();
        let next = count.wrapping_sub(1);
        let result = adapters13::fun_140002130(
            &[BASE],
            &mut memory,
            &mut |_| panic!("unexpected lookup"),
            &mut |symbol, args, memory| {
                calls.push(symbol);
                assert_eq!(memory.read_u32(allocation + 24), Ok(next));
                assert_eq!(memory.read_u64(BASE), Ok(allocation));
                match symbol {
                    "GetProcessHeap" => assert!(args.is_empty()),
                    "HeapFree" => assert_eq!(args, &[0xfeed, 0, allocation]),
                    "abort" => assert!(args.is_empty()),
                    _ => panic!("unexpected dependency {symbol}"),
                }
                0xfeed
            },
        );
        if count != 1 && (next as i32) < 0 {
            assert_eq!(result, Err(adapters13::AdapterError::Halted("abort")));
            assert_eq!(memory.read_u64(BASE), Ok(allocation));
            assert_eq!(calls, ["abort"]);
        } else {
            assert_eq!(result, Ok(None));
            assert_eq!(memory.read_u64(BASE), Ok(0));
            if count == 1 {
                assert_eq!(calls, ["GetProcessHeap", "HeapFree"]);
            } else {
                assert!(calls.is_empty());
            }
        }
        assert_eq!(memory.read_u32(allocation + 24), Ok(next));
    }
}

#[test]
fn short_circuit_guard_skips_invalid_pointer_and_assignment_operand() {
    let mut memory = image(0, BASE, "");
    memory.write_u64(BASE, u64::MAX - 1).unwrap();
    let before = memory.clone();
    let result = adapters13::fun_1400017f0(
        &[BASE],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |_, _, _| panic!("inactive guard called runtime"),
    );
    assert_eq!(result, Ok(None));
    assert_eq!(memory, before);
    memory.write_u8(BASE + 8, 0x80).unwrap();
    memory.write_u64(BASE, 0).unwrap();
    let before = memory.clone();
    let result = adapters13::fun_1400017f0(
        &[BASE],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |_, _, _| panic!("null allocation called runtime"),
    );
    assert_eq!(result, Ok(None));
    assert_eq!(memory, before);
    memory.write_u64(BASE, BASE + 1024).unwrap();
    memory.write_u32(BASE + 1024 + 24, 2).unwrap();
    let result = adapters13::fun_1400017f0(
        &[BASE],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |_, _, _| panic!("retained allocation called runtime"),
    );
    assert_eq!(result, Ok(None));
    assert_eq!(memory.read_u32(BASE + 1024 + 24), Ok(1));
    assert_eq!(memory.read_u64(BASE), Ok(0));
}

#[test]
fn retry_loop_reloads_callback_changes_and_keeps_writes_on_panic() {
    let mut memory = image(0, BASE, "");
    memory.write_u64(BASE, BASE + 1024).unwrap();
    memory.write_u64(BASE + 1024, 1).unwrap();
    let mut calls = 0;
    let result = adapters13::fun_140008c00(
        &[BASE],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |symbol, args, memory| {
            calls += 1;
            assert_eq!(symbol, "_Thrd_yield");
            assert!(args.is_empty());
            assert_eq!(memory.read_u64(BASE + 1024), Ok(0));
            memory
                .write_u64(BASE + 1024, if calls < 3 { 1 } else { 2 })
                .unwrap();
            0
        },
    );
    assert_eq!(result, Ok(None));
    assert_eq!(calls, 3);
    assert_eq!(memory.read_u64(BASE + 1024), Ok(0));
    memory.write_u64(BASE + 1024, 1).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        adapters13::fun_140008c00(
            &[BASE],
            &mut memory,
            &mut |_| panic!("unexpected lookup"),
            &mut |_, _, memory| {
                memory.bytes[100] = 0x42;
                panic!("yield failed")
            },
        )
    }));
    assert!(result.is_err());
    assert_eq!(memory.read_u64(BASE + 1024), Ok(0));
    assert_eq!(memory.bytes[100], 0x42);
}

#[test]
fn cleanup_jump_skips_later_global_release_on_underflow() {
    for count in [0u32, 1, 2] {
        let mut memory = image(0, BASE, "");
        let allocation = BASE + 1024;
        memory.write_u64(BASE + 48, allocation).unwrap();
        memory.write_u32(allocation + 24, count).unwrap();
        memory.write_u32(BASE + 2048, 1).unwrap();
        let mut calls = Vec::new();
        let result = adapters13::fun_140002820(
            &[BASE],
            &mut memory,
            &mut |symbol| match symbol {
                "Paint::Background::vftable" => 0xfeed,
                "DAT_1401cfae8" => {
                    assert_ne!(count, 0);
                    BASE + 2048
                }
                _ => panic!("unexpected lookup {symbol}"),
            },
            &mut |symbol, _, _| {
                calls.push(symbol);
                0x1234
            },
        );
        assert_eq!(memory.read_u64(BASE), Ok(0xfeed));
        assert_eq!(memory.read_u32(allocation + 24), Ok(count.wrapping_sub(1)));
        if count == 0 {
            assert_eq!(result, Err(adapters13::AdapterError::Halted("abort")));
            assert_eq!(memory.read_u64(BASE + 48), Ok(allocation));
            assert_eq!(memory.read_u32(BASE + 2048), Ok(1));
            assert_eq!(calls, ["FUN_140003920", "abort"]);
        } else {
            assert_eq!(result, Ok(None));
            assert_eq!(memory.read_u64(BASE + 48), Ok(0));
            assert_eq!(memory.read_u32(BASE + 2048), Ok(0));
            assert_eq!(calls.last(), Some(&"FUN_140003ae0"));
            assert_eq!(calls.contains(&"HeapFree"), count == 1);
        }
    }
}

#[test]
fn indirect_output_changes_branch_and_later_calls_keep_their_explicit_arity() {
    for clear_source in [false, true] {
        let anchor = BASE + 20000;
        let source = BASE + 512;
        let output = BASE + 1024;
        let object = BASE + 2048;
        let table = BASE + 3072;
        let mut memory = image(0, BASE, "");
        memory.write_u64(source, object).unwrap();
        memory.write_u64(object, table).unwrap();
        memory.write_u64(table, 0xaaaa).unwrap();
        memory.write_u64(table + 8, 0xbbbb).unwrap();
        let mut calls = Vec::new();
        let result = adapters13::fun_140009f20(
            &[output, source],
            &mut memory,
            &mut |symbol| match symbol {
                "DAT_140179040" => BASE + 30000,
                "DAT_1401057b0" => BASE + 29000,
                _ => frame_address("FUN_140009f20", symbol, anchor),
            },
            &mut |symbol, args, memory| {
                calls.push(symbol);
                match symbol {
                    "INDIRECT_CALL" if args[0] == 0xaaaa => {
                        assert_eq!(args, &[0xaaaa, object, BASE + 29000, anchor - 0x50]);
                        assert_eq!(memory.read_u64(args[3]), Ok(0));
                        memory.write_u64(args[3], 0xfeed).unwrap();
                    }
                    "FUN_140001b30" => {
                        assert_eq!(args, &[anchor - 0x50]);
                        assert_eq!(memory.read_u64(args[0]), Ok(0xfeed));
                        memory.write_u64(args[0], 0).unwrap();
                        if clear_source {
                            memory.write_u64(source, 0).unwrap();
                        }
                    }
                    "INDIRECT_CALL" => assert_eq!(args, &[0xbbbb]),
                    _ => panic!("unexpected dependency {symbol}"),
                }
                0
            },
        );
        assert_eq!(result, Ok(Some(output)));
        assert_eq!(
            memory.read_u64(output),
            Ok(if clear_source { 0 } else { object })
        );
        assert_eq!(calls.len(), if clear_source { 2 } else { 3 });
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
fn all_440_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 3520);
    assert_eq!(cases.len(), 7040);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters13::HANDLERS.iter().find(|x| x.0 == name).unwrap();
        let globals = bindings(i[5]);
        let returned_pointers = bindings(i[6]);
        let outputs: Vec<Vec<usize>> = i[7]
            .split(';')
            .filter(|x| !x.is_empty())
            .map(|x| x.split(':').map(|n| n.parse().unwrap()).collect())
            .collect();
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
                    for row in &outputs {
                        if row[0] == index {
                            memory
                                .write_u64(args[row[1]] + row[2] as u64, base + row[3] as u64)
                                .unwrap();
                        }
                    }
                    returned_pointers
                        .get(symbol)
                        .map_or(PAYLOADS[seed] ^ (index as u64 * 0x101), |&at| {
                            base + at as u64
                        })
                },
            );
            let result = if f[2] == "!" {
                assert!(
                    matches!(result, Err(adapters13::AdapterError::Halted(_))),
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
    assert_eq!(adapters13::HANDLERS.len(), 440);
}

#[test]
fn manifest_reaches_sixty_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 440);
    let manifest = include_str!("../translations/batch-013.json");
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
        include_str!("../translations/batch-012.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 440);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 440;
    assert_eq!(total, 2636);
    assert!(total * 100 >= 4392 * 60);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters13::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters13::HANDLERS {
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
