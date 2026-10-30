use mspaint_port::recovered::{adapters9, call_support::MemoryImage};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeSet,
    fmt::Write,
};
const BASE: u64 = 0x100_0000;
const SIZE: usize = 4096;
const OFFSETS: [u64; 4] = [128, 1024, 2048, 3072];
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
const CASES: &str = include_str!("fixtures/batch-009.csv");
fn hex(s: &str) -> u64 {
    u64::from_str_radix(s, 16).unwrap()
}
fn bytes(s: &str) -> Vec<u8> {
    let (pairs, tail) = s.as_bytes().as_chunks::<2>();
    assert!(tail.is_empty());
    pairs
        .iter()
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect()
}
fn symbol_hex(symbol: &str) -> String {
    symbol.bytes().map(|b| format!("{b:02x}")).collect()
}
fn canonical(word: u64, base: u64) -> u64 {
    match word.checked_sub(base) {
        Some(offset) if offset <= SIZE as u64 => BASE + offset,
        _ => word,
    }
}
fn canonical_image(memory: &MemoryImage) -> Vec<u8> {
    memory
        .bytes
        .as_chunks::<8>()
        .0
        .iter()
        .flat_map(|word| canonical(u64::from_le_bytes(*word), memory.base).to_le_bytes())
        .collect()
}
fn fingerprint(memory: &MemoryImage) -> u64 {
    canonical_image(memory)
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |hash, b| {
            (hash ^ u64::from(*b)).wrapping_mul(0x100_0000_01b3)
        })
}
fn image(seed: usize, base: u64) -> MemoryImage {
    MemoryImage {
        base,
        bytes: (0..SIZE)
            .map(|i| match seed {
                0 => 0,
                1 => 255,
                _ => (i * 17 + seed * 31) as u8,
            })
            .collect(),
    }
}
fn arguments(entry: &adapters9::Entry, seed: usize, base: u64) -> Vec<u64> {
    entry
        .pointers
        .iter()
        .enumerate()
        .map(|(i, &pointer)| {
            if pointer {
                base + OFFSETS[i]
            } else {
                PAYLOADS[(seed + i) % 8]
            }
        })
        .collect()
}

#[test]
fn all_220_functions_match_original_c_memory_returns_and_ordered_events() {
    let mut covered = BTreeSet::new();
    let mut count = 0;
    for line in CASES.lines() {
        let f: Vec<_> = line.split(',').collect();
        let name = f[0];
        let seed: usize = f[1].parse().unwrap();
        let entry = adapters9::HANDLERS.iter().find(|x| x.name == name).unwrap();
        let expected_image = bytes(f[4]);
        for base in [BASE, 0x1234_5678_0000] {
            let trace = RefCell::new(String::new());
            let calls = Cell::new(0usize);
            let mut memory = image(seed, base);
            let result = (entry.handler)(
                &arguments(entry, seed, base),
                &mut memory,
                &mut |symbol| {
                    let value = PAYLOADS[(seed + 3) % 8];
                    write!(trace.borrow_mut(), "R:{}:{value:x}|", symbol_hex(symbol)).unwrap();
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
                    memory.bytes[4080] ^= (index + 1) as u8;
                    PAYLOADS[seed] ^ (index as u64 * 0x101)
                },
            )
            .unwrap();
            assert_eq!(
                result.map(|value| canonical(value, base)),
                if f[2] == "-" { None } else { Some(hex(f[2])) },
                "return of {name}, seed {seed}"
            );
            assert_eq!(
                *trace.borrow(),
                f[3],
                "events of {name}, seed {seed}, base {base:x}"
            );
            assert_eq!(
                canonical_image(&memory),
                expected_image,
                "final memory of {name}, seed {seed}, base {base:x}"
            );
        }
        covered.insert(name);
        count += 1;
    }
    assert_eq!((count, covered.len()), (1760, 220));
    assert_eq!(adapters9::HANDLERS.len(), 220);
}

#[test]
fn manifest_reaches_thirty_five_percent_without_reusing_addresses() {
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|line| line.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 220);
    let manifest = include_str!("../translations/batch-009.json");
    let previous = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
        include_str!("../translations/batch-004.json"),
        include_str!("../translations/batch-005.json"),
        include_str!("../translations/batch-006.json"),
        include_str!("../translations/batch-007.json"),
        include_str!("../translations/batch-008.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 220);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(previous.iter().all(|prior| !prior.contains(name)));
    }
    let total = 15
        + previous
            .iter()
            .map(|x| x.matches("\"original\":").count())
            .sum::<usize>()
        + 220;
    assert_eq!(total, 1538);
    assert!(total * 100 >= 4392 * 35);
}

#[test]
fn missing_arguments_fail_before_memory_symbol_or_dependency_effects() {
    use mspaint_port::recovered::call_support::MissingArguments;
    for entry in adapters9::HANDLERS {
        let args = arguments(entry, 0, BASE);
        for available in 0..args.len() {
            let mut memory = image(3, BASE);
            let before = memory.clone();
            let result = (entry.handler)(
                &args[..available],
                &mut memory,
                &mut |_| panic!("missing arguments resolved a symbol"),
                &mut |_, _, _| panic!("missing arguments invoked a dependency"),
            );
            assert_eq!(
                result,
                Err(adapters9::AdapterError::Arguments(MissingArguments {
                    required: args.len(),
                    available
                })),
                "{}",
                entry.name
            );
            assert_eq!(memory, before);
        }
    }
}

#[test]
fn exception_copy_keeps_callback_mutations_and_replaces_only_the_header() {
    for callback_return in [0, u64::MAX, 0x1234_5678] {
        let mut memory = image(0, BASE);
        let events = RefCell::new(Vec::new());
        let result = adapters9::fun_140001f30(
            &[BASE, BASE + 1024],
            &mut memory,
            &mut |symbol| {
                events.borrow_mut().push(symbol);
                match symbol {
                    "std::exception::vftable" => 0x1111,
                    "std::invalid_argument::vftable" => 0x2222,
                    _ => panic!("unexpected symbol"),
                }
            },
            &mut |symbol, args, memory| {
                events.borrow_mut().push(symbol);
                assert_eq!(symbol, "__std_exception_copy");
                assert_eq!(args, &[BASE + 1032]);
                assert_eq!(memory.read_u64(BASE), Ok(0x1111));
                assert_eq!(memory.read_u64(BASE + 8), Ok(0));
                assert_eq!(memory.read_u64(BASE + 16), Ok(0));
                memory.write_u64(BASE, 0xaaaa).unwrap();
                memory.write_u64(BASE + 8, 0xbbbb).unwrap();
                memory.write_u64(BASE + 16, 0xcccc).unwrap();
                callback_return
            },
        );
        assert_eq!(result, Ok(Some(BASE)));
        assert_eq!(
            *events.borrow(),
            [
                "std::exception::vftable",
                "__std_exception_copy",
                "std::invalid_argument::vftable"
            ]
        );
        assert_eq!(memory.read_u64(BASE), Ok(0x2222));
        assert_eq!(memory.read_u64(BASE + 8), Ok(0xbbbb));
        assert_eq!(memory.read_u64(BASE + 16), Ok(0xcccc));
    }
}

#[test]
fn exception_construction_retains_partial_writes_before_faults() {
    for available in 0..24 {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0xa5; available],
        };
        let mut lookups = 0;
        let result = adapters9::fun_140001f30(
            &[BASE, BASE + 1024],
            &mut memory,
            &mut |_| {
                lookups += 1;
                0x1111
            },
            &mut |_, _, _| panic!("short destination invoked copy"),
        );
        assert!(matches!(result, Err(adapters9::AdapterError::Memory(_))));
        assert_eq!(lookups, 1);
        if available >= 8 {
            assert_eq!(memory.read_u64(BASE), Ok(0x1111));
        }
        if available >= 16 {
            assert_eq!(memory.read_u64(BASE + 8), Ok(0));
        }
    }
    let mut memory = image(0, BASE);
    let snapshot = RefCell::new(None);
    let mut lookups = 0;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        adapters9::fun_140001f30(
            &[BASE, BASE + 1024],
            &mut memory,
            &mut |_| {
                lookups += 1;
                if lookups == 2 {
                    panic!("final relocation failed");
                }
                0x1111
            },
            &mut |_, _, memory| {
                memory.write_u64(BASE + 8, 0x5555).unwrap();
                *snapshot.borrow_mut() = Some(memory.clone());
                0
            },
        )
    }));
    assert!(result.is_err());
    assert_eq!(Some(&memory), snapshot.borrow().as_ref());
}

#[test]
fn destructor_calls_cleanup_before_testing_the_release_flag() {
    for flags in [0, 1, 2, 0x100, u64::MAX] {
        let mut memory = image(0, BASE);
        let mut calls = Vec::new();
        let result = adapters9::fun_140001ee0(
            &[BASE, flags],
            &mut memory,
            &mut |symbol| {
                assert_eq!(symbol, "std::exception::vftable");
                0x9876
            },
            &mut |symbol, args, memory| {
                assert_eq!(memory.read_u64(BASE), Ok(0x9876));
                calls.push((symbol, args.to_vec()));
                u64::MAX
            },
        );
        assert_eq!(result, Ok(Some(BASE)));
        assert_eq!(calls[0], ("__std_exception_destroy", vec![BASE + 8]));
        assert_eq!(calls.len(), 1 + (flags & 1) as usize);
        if flags & 1 != 0 {
            assert_eq!(calls[1], ("FUN_1400b022c", vec![BASE, 0x18]));
        }
    }
    let mut memory = image(0, BASE);
    let mut calls = 0;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        adapters9::fun_140001ee0(
            &[BASE, 1],
            &mut memory,
            &mut |_| 0x9876,
            &mut |symbol, _, memory| {
                calls += 1;
                assert_eq!(symbol, "__std_exception_destroy");
                memory.bytes[32] = 0x42;
                panic!("cleanup failed")
            },
        )
    }));
    assert!(result.is_err());
    assert_eq!(calls, 1);
    assert_eq!(memory.read_u64(BASE), Ok(0x9876));
    assert_eq!(memory.bytes[32], 0x42);
}

#[test]
fn byte_fields_are_read_at_their_original_width() {
    for byte in [0, 1, 0x80, 0xff] {
        let mut memory = image(1, BASE);
        memory.write_u64(BASE + 8, 0x1234).unwrap();
        memory.bytes[16] = byte;
        let mut calls = 0;
        let result = adapters9::fun_14003c340(
            &[BASE],
            &mut memory,
            &mut |_| panic!("unexpected relocation"),
            &mut |symbol, args, _| {
                calls += 1;
                assert_eq!(symbol, "FUN_140035440");
                assert_eq!(args, &[0x1234, u64::from(byte)]);
                0
            },
        );
        assert_eq!(result, Ok(None));
        assert_eq!(calls, 1);
    }
    let memory = MemoryImage {
        base: u64::MAX,
        bytes: vec![0x80],
    };
    assert_eq!(memory.read_u8(u64::MAX), Ok(0x80));
    assert_eq!(memory.read_u8(u64::MAX - 1).unwrap_err().width, 1);
}
