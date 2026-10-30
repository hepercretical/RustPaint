use mspaint_port::recovered::{adapters10, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-010-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-010.csv");
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
    assert_eq!(cases.len(), 1752);
    for (input, expected) in inputs.into_iter().zip(cases) {
        let i: Vec<_> = input.split(',').collect();
        let f: Vec<_> = expected.split(',').collect();
        assert_eq!((&i[0], &i[1]), (&f[0], &f[1]));
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters10::HANDLERS.iter().find(|x| x.0 == name).unwrap();
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
            )
            .unwrap();
            assert_eq!(
                result.map(|v| canonical(v, base)),
                if f[2] == "-" { None } else { Some(hex(f[2])) },
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
    assert_eq!(adapters10::HANDLERS.len(), 219);
}

#[test]
fn manifest_reaches_forty_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 219);
    let manifest = include_str!("../translations/batch-010.json");
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
    assert_eq!(total, 1757);
    assert!(total * 5 >= 4392 * 2);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters9::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters10::HANDLERS {
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

#[test]
fn globals_use_resolved_memory_and_exact_field_widths() {
    for returned in [0, 1, 0x100, 0xff, 0x8000_0000, u64::MAX] {
        let mut memory = image(1, BASE, "");
        let mut expected = memory.clone();
        expected.bytes[1024] = returned as u8;
        let events = RefCell::new(Vec::new());
        let result = adapters10::fun_140001420(
            &[],
            &mut memory,
            &mut |symbol| {
                events.borrow_mut().push(symbol);
                assert_eq!(symbol, "DAT_1401cfad1");
                BASE + 1024
            },
            &mut |symbol, args, _| {
                events.borrow_mut().push(symbol);
                assert_eq!(symbol, "AfxInitialize");
                assert_eq!(args, &[0, 0xe00]);
                returned
            },
        );
        assert_eq!(result, Ok(None));
        assert_eq!(memory, expected);
        assert_eq!(*events.borrow(), ["AfxInitialize", "DAT_1401cfad1"]);
    }
    for value in [0, 1, 0x8000_0000, u32::MAX] {
        let mut memory = image(0, BASE, "");
        memory.write_u32(BASE + 1024, value).unwrap();
        let before = memory.clone();
        let result = adapters10::fun_1400b08e4(
            &[],
            &mut memory,
            &mut |symbol| {
                assert_eq!(symbol, "DAT_140179088");
                BASE + 1024
            },
            &mut |_, _, _| panic!("unexpected call"),
        );
        assert_eq!(result, Ok(Some(u64::from(value == 0))));
        assert_eq!(memory, before);
    }
}

#[test]
fn telemetry_preserves_literal_status_and_call_order_with_callback_mutation() {
    let mut memory = image(0, BASE, "");
    memory.write_u64(BASE + 0x80, BASE + 3072).unwrap();
    memory.write_u32(BASE + 3072 + 0xc, 0x8000_4005).unwrap();
    for (i, word) in "Canvas::Apply\0".encode_utf16().enumerate() {
        memory.write_u16(BASE + 6000 + i as u64 * 2, word).unwrap();
    }
    let mut calls = Vec::new();
    let result = adapters10::fun_1400cc818(
        &[0, BASE],
        &mut memory,
        &mut |symbol| {
            assert_eq!(symbol, "L\"Canvas::Apply\"");
            BASE + 6000
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "Paint::Telemetry::TraceLogger::GetInstance" => {
                    assert!(args.is_empty());
                    return 0x1234;
                }
                "FUN_140005310" => {
                    assert_eq!(args, &[BASE + 0x88, BASE + 0x28]);
                    assert_eq!(memory.read_u32(BASE + 0x20), Ok(0x8000_4005));
                    assert_eq!(memory.read_u64(BASE + 0x28), Ok(BASE + 6000));
                    assert_eq!(memory.read_u64(BASE + 0x30), Ok(13));
                    memory.write_u32(BASE + 0x20, 0x8007_0005).unwrap();
                    memory.write_u64(BASE + 0x88, BASE + 6000).unwrap();
                }
                "Paint::Telemetry::TraceLogger::SendFailureEvent" => {
                    assert_eq!(args, &[0x1234, BASE + 0x88, BASE + 0x20]);
                    assert_eq!(memory.read_u32(BASE + 0x20), Ok(0x8007_0005));
                }
                "FUN_140002130" => {
                    assert_eq!(args, &[BASE + 0x88]);
                    memory.write_u64(BASE + 0x88, 0).unwrap();
                }
                _ => panic!("unexpected telemetry dependency"),
            }
            u64::MAX
        },
    );
    assert_eq!(result, Ok(Some(0)));
    assert_eq!(
        calls,
        [
            "Paint::Telemetry::TraceLogger::GetInstance",
            "FUN_140005310",
            "Paint::Telemetry::TraceLogger::SendFailureEvent",
            "FUN_140002130"
        ]
    );
    assert_eq!(memory.read_u64(BASE + 0x88), Ok(0));
    let mut observed = None;
    let mut count = 0;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        adapters10::fun_1400cc818(
            &[0, BASE],
            &mut memory,
            &mut |_| BASE + 6000,
            &mut |_, _, memory| {
                count += 1;
                if count == 2 {
                    memory.bytes[200] = 0x42;
                    observed = Some(memory.clone());
                    panic!("string construction failed");
                }
                0x1234
            },
        )
    }));
    assert!(result.is_err());
    assert_eq!(count, 2);
    assert_eq!(Some(&memory), observed.as_ref());
}

#[test]
fn indirect_guard_failure_retains_completed_scratch_writes() {
    use mspaint_port::recovered::adapters9::AdapterError;
    let mut memory = image(0, BASE, "");
    memory.write_u64(BASE + 0x30, BASE + 3072).unwrap();
    let result = adapters10::fun_1400d8570(
        &[0, BASE],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |_, _, _| panic!("invalid guard called runtime"),
    );
    let Err(AdapterError::Memory(fault)) = result else {
        panic!("expected an indirect access failure")
    };
    assert_eq!((fault.address, fault.width), (0x8a, 2));
    assert_eq!(memory.read_u64(BASE + 0x98), Ok(BASE + 3072 + 0xd0));
    assert_eq!(memory.read_u64(BASE + 0x20), Ok(0));
    memory.write_u64(BASE + 3072 + 0xd0, BASE + 6000).unwrap();
    let result = adapters10::fun_1400d8570(
        &[0, BASE],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |_, _, _| panic!("zero guard called runtime"),
    );
    assert_eq!(result, Ok(None));
    assert_eq!(memory.read_u64(BASE + 0x20), Ok(BASE + 6000));
}

#[test]
fn invalid_second_returned_pointer_preserves_the_first_update() {
    use mspaint_port::recovered::adapters9::AdapterError;
    let mut memory = image(0, BASE, "");
    memory.write_u64(BASE + 1024, 0x100).unwrap();
    let mut calls = 0;
    let result = adapters10::fun_1400b08c8(
        &[],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |symbol, args, memory| {
            assert!(args.is_empty());
            calls += 1;
            if calls == 1 {
                assert_eq!(symbol, "FUN_1400124a0");
                BASE + 1024
            } else {
                assert_eq!(symbol, "FUN_1400b08c0");
                assert_eq!(memory.read_u64(BASE + 1024), Ok(0x124));
                memory.bytes[32] = 0x42;
                u64::MAX - 3
            }
        },
    );
    let Err(AdapterError::Memory(fault)) = result else {
        panic!("expected a returned-pointer failure")
    };
    assert_eq!((fault.address, fault.width), (u64::MAX - 3, 8));
    assert_eq!(calls, 2);
    assert_eq!(memory.read_u64(BASE + 1024), Ok(0x124));
    assert_eq!(memory.bytes[32], 0x42);
}

#[test]
fn status_conversion_narrows_before_branching_and_returns_a_boolean_word() {
    for status in [
        0,
        1,
        0x7fff_ffff,
        0x8000_0000,
        0xffff_ffff,
        0x1_0000_0000,
        u64::MAX,
    ] {
        let mut memory = image(1, BASE, "");
        memory.write_u64(BASE + 0x98, 0x1111).unwrap();
        memory.write_u64(BASE + 0xa0, 0x2222).unwrap();
        let mut calls = 0;
        let signed = status as i32;
        let result = adapters10::fun_1400d6723(
            &[0, BASE],
            &mut memory,
            &mut |_| panic!("unexpected lookup"),
            &mut |symbol, args, _| {
                calls += 1;
                match symbol {
                    "FUN_140090240" => {
                        assert_eq!(args, &[0x1111, 0x2222]);
                        status
                    }
                    "FUN_140012f40" => {
                        assert_eq!(args, &[signed as i64 as u64]);
                        0x1234_5678_8765_4321
                    }
                    _ => panic!("unexpected status dependency"),
                }
            },
        );
        assert_eq!(result, Ok(Some(u64::from(signed >= 0))));
        assert_eq!(calls, if signed < 0 { 2 } else { 1 });
        if signed < 0 {
            assert_eq!(memory.read_u32(BASE + 0x48), Ok(status as u32));
            assert_eq!(memory.read_u32(BASE + 0x4c), Ok(0x8765_4321));
            assert_eq!(memory.read_u32(BASE + 0x50), Ok(0));
        } else {
            assert!(memory.bytes[0x48..0x54].iter().all(|&b| b == 255));
        }
    }
}
