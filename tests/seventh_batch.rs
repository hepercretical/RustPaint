use mspaint_port::recovered::{call_support::MemoryImage, dispatch7, memory7};
use std::collections::BTreeSet;

const BASE: u64 = 0x100_0000;
const SIZE: usize = 4096;
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
const FLAGS: [u64; 8] = [0, 1, 2, 0x100, 0x80, 0xff, u64::MAX, 0x8000_0000_0000_0000];
const CONDITIONS: [u16; 8] = [0, 1, 0x8000, 0xffff, 0x7fff, 0x100, 0xff, 0];
const MEMORY_CASES: &str = include_str!("fixtures/batch-007-memory.csv");
const DISPATCH_CASES: &str = include_str!("fixtures/batch-007-dispatch.csv");

fn hex(s: &str) -> u64 {
    u64::from_str_radix(s, 16).unwrap()
}
fn bytes(s: &str) -> Vec<u8> {
    let (pairs, remainder) = s.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect()
}
fn canonical_word(word: u64, base: u64) -> u64 {
    match word.checked_sub(base) {
        Some(offset) if offset <= SIZE as u64 => BASE + offset,
        _ => word,
    }
}
fn canonical_image(entry: &memory7::Entry, memory: &MemoryImage) -> Vec<u8> {
    // Opaque release arguments retain their raw values even if they happen
    // to lie within the relocated frame's numeric address range.
    if entry.guard.is_none() {
        return memory.bytes.clone();
    }
    memory
        .bytes
        .as_chunks::<8>()
        .0
        .iter()
        .flat_map(|word| canonical_word(u64::from_le_bytes(*word), memory.base).to_le_bytes())
        .collect()
}
fn fingerprint(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
    })
}
fn image(entry: &memory7::Entry, seed: usize, base: u64) -> MemoryImage {
    let mut memory = MemoryImage {
        base,
        bytes: (0..SIZE).map(|i| (i * 17 + seed * 31) as u8).collect(),
    };
    let value = if let Some((guard_offset, _, _)) = entry.guard {
        let object_offset = 1024 + (seed % 4) * 256;
        let offset = object_offset + guard_offset;
        memory.bytes[offset..offset + 2].copy_from_slice(&CONDITIONS[seed].to_le_bytes());
        base + object_offset as u64
    } else {
        PAYLOADS[seed]
    };
    memory
        .write_u64(base + entry.input_offset as u64, value)
        .unwrap();
    memory
}

#[test]
fn all_115_memory_handlers_match_original_c_at_three_bases() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in MEMORY_CASES.lines() {
        let f: Vec<_> = line.split(',').collect();
        assert_eq!(f[0], "M");
        let name = f[1];
        let seed: usize = f[2].parse().unwrap();
        let entry = memory7::HANDLERS.iter().find(|x| x.name == name).unwrap();
        covered.insert(name);
        let expected_args: Vec<_> = f[6].split(';').filter(|s| !s.is_empty()).map(hex).collect();
        let expected_image = bytes(f[8]);
        let count: usize = f[3].parse().unwrap();
        for base in [BASE, 0x1234_5678_0000, u64::MAX - SIZE as u64 + 1] {
            let mut memory = image(entry, seed, base);
            let mut calls = Vec::new();
            (entry.handler)(base, &mut memory, &mut |symbol, args, memory| {
                calls.push((
                    symbol,
                    args.iter()
                        .map(|&word| {
                            if entry.guard.is_some() {
                                canonical_word(word, base)
                            } else {
                                word
                            }
                        })
                        .collect::<Vec<_>>(),
                    fingerprint(&canonical_image(entry, memory)),
                ));
                memory.bytes[4080] ^= 0x5a;
                0xfeed_face_cafe_beef ^ seed as u64
            })
            .unwrap();
            assert_eq!(calls.len(), count, "{name}, seed {seed}");
            if count == 0 {
                assert_eq!((f[4], f[5], f[6], f[7]), ("-", "0", "", "0"));
            } else {
                assert_eq!(count, 1);
                assert_eq!(calls[0].0, f[4], "{name}");
                assert_eq!(calls[0].1.len(), f[5].parse::<usize>().unwrap());
                assert_eq!(calls[0].1, expected_args, "{name}");
                assert_eq!(calls[0].2, hex(f[7]), "callback state of {name}");
            }
            assert_eq!(
                canonical_image(entry, &memory),
                expected_image,
                "final state of {name}, seed {seed}, base {base:x}"
            );
        }
        cases += 1;
    }
    assert_eq!((cases, covered.len()), (920, 115));
    assert_eq!(memory7::HANDLERS.len(), 115);
    assert_eq!(
        memory7::HANDLERS
            .iter()
            .filter(|x| x.guard.is_some())
            .count(),
        48
    );
}

fn parse_trace(trace: &str) -> Vec<(&str, Vec<u64>)> {
    trace
        .split('|')
        .filter(|x| !x.is_empty())
        .map(|call| {
            let (symbol, args) = call.split_once(':').unwrap();
            (
                symbol,
                args.split(';').filter(|x| !x.is_empty()).map(hex).collect(),
            )
        })
        .collect()
}

#[test]
fn all_104_dispatch_helpers_match_original_c_calls_returns_and_lookups() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in DISPATCH_CASES.lines() {
        let f: Vec<_> = line.split(',').collect();
        let name = f[1];
        let seed: usize = f[2].parse().unwrap();
        let mut lookups = Vec::new();
        let mut calls = Vec::new();
        let mut relocate = |symbol| {
            lookups.push(symbol);
            PAYLOADS[(seed + 3) % 8]
        };
        let mut invoke = |symbol, args: &[u64]| {
            calls.push((symbol, args.to_vec()));
            0xfeed_face_cafe_beef ^ seed as u64
        };
        let result = match f[0] {
            "global" => {
                let (_, function) = dispatch7::GLOBALS.iter().find(|x| x.0 == name).unwrap();
                function(
                    PAYLOADS[seed],
                    PAYLOADS[(seed + 1) % 8],
                    PAYLOADS[(seed + 2) % 8],
                    &mut relocate,
                    &mut invoke,
                );
                None
            }
            "relocation" => {
                let (_, function) = dispatch7::ADDRESSES.iter().find(|x| x.0 == name).unwrap();
                Some(function(&mut relocate))
            }
            "plain" => {
                let (_, function) = dispatch7::CONSTANTS.iter().find(|x| x.0 == name).unwrap();
                Some(function())
            }
            "scalar" => {
                let (_, function) = dispatch7::SCALARS.iter().find(|x| x.0 == name).unwrap();
                function(PAYLOADS[seed], FLAGS[seed], &mut invoke)
            }
            _ => panic!("Unknown category"),
        };
        assert_eq!(
            result,
            if f[3] == "-" { None } else { Some(hex(f[3])) },
            "return of {name}, seed {seed}"
        );
        let expected_lookups: Vec<_> = if f[4] == "-" { vec![] } else { vec![f[4]] };
        assert_eq!(lookups, expected_lookups, "lookup of {name}");
        assert_eq!(calls, parse_trace(f[5]), "calls of {name}, seed {seed}");
        covered.insert(name);
        cases += 1;
    }
    assert_eq!((cases, covered.len()), (832, 104));
}

#[test]
fn new_addresses_are_distinct_and_reach_twenty_five_percent() {
    let covered: BTreeSet<_> = MEMORY_CASES
        .lines()
        .chain(DISPATCH_CASES.lines())
        .map(|line| line.split(',').nth(1).unwrap())
        .collect();
    assert_eq!(covered.len(), 219);
    let manifest = include_str!("../translations/batch-007.json");
    let previous = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
        include_str!("../translations/batch-004.json"),
        include_str!("../translations/batch-005.json"),
        include_str!("../translations/batch-006.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 219);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(previous.iter().all(|x| !x.contains(name)));
    }
    let total = 15
        + previous
            .iter()
            .map(|x| x.matches("\"original\":").count())
            .sum::<usize>()
        + 219;
    assert_eq!(total, 1098);
    assert!(total * 4 >= 4392);
}

#[test]
fn guarded_accesses_preserve_branch_and_partial_write_behavior() {
    for entry in memory7::HANDLERS {
        let mut memory = image(entry, 0, BASE);
        memory.bytes.truncate(entry.input_offset + 7);
        let before = memory.clone();
        assert!((entry.handler)(BASE, &mut memory, &mut |_, _, _| panic!(
            "invalid input read invoked a dependency"
        ))
        .is_err());
        assert_eq!(memory, before);
        if entry.guard.is_some() {
            let mut memory = image(entry, 0, BASE);
            memory
                .write_u64(BASE + entry.input_offset as u64, 0)
                .unwrap();
            let before = memory.clone();
            assert!((entry.handler)(BASE, &mut memory, &mut |_, _, _| panic!(
                "invalid guard read invoked a dependency"
            ))
            .is_err());
            assert_eq!(memory, before);
        }
    }
    let entry = memory7::HANDLERS
        .iter()
        .find(|x| x.name == "FUN_1400ce960")
        .unwrap();
    let (guard_offset, size_offset, pointer_offset) = entry.guard.unwrap();
    let mut memory = image(entry, 0, BASE);
    memory.bytes.truncate(pointer_offset + 7);
    memory
        .write_u64(BASE + entry.input_offset as u64, BASE + 256)
        .unwrap();
    memory.bytes[256 + guard_offset..258 + guard_offset].copy_from_slice(&0u16.to_le_bytes());
    let before = memory.clone();
    // A zero guard suppresses both scratch writes, even with a short target.
    (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
        panic!("zero guard invoked dependency")
    })
    .unwrap();
    assert_eq!(memory, before);
    memory.bytes[256 + guard_offset..258 + guard_offset].copy_from_slice(&0x8000u16.to_le_bytes());
    let result = (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
        panic!("failed second write invoked dependency")
    });
    assert_eq!(result.unwrap_err().address, BASE + pointer_offset as u64);
    assert_eq!(memory.read_u64(BASE + size_offset as u64), Ok(0x230));
}

#[test]
fn dependency_and_resolver_panics_propagate_without_later_calls() {
    for entry in memory7::HANDLERS {
        let mut memory = image(entry, 1, BASE);
        let mut observed = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (entry.handler)(BASE, &mut memory, &mut |_, _, memory| {
                memory.bytes[4080] ^= 0x5a;
                observed = Some(memory.clone());
                panic!("dependency failed")
            })
        }));
        assert!(result.is_err(), "{}", entry.name);
        assert_eq!(Some(&memory), observed.as_ref());
    }
    for &(name, function) in dispatch7::SCALARS {
        let mut calls = 0;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            function(17, 1, &mut |_, _| {
                calls += 1;
                panic!("cleanup failed")
            })
        }));
        assert!(result.is_err(), "{name}");
        assert_eq!(calls, 1);
    }
    for &(name, function) in dispatch7::GLOBALS {
        let mut called = false;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            function(
                1,
                2,
                3,
                &mut |_| panic!("missing relocation"),
                &mut |_, _| {
                    called = true;
                    0
                },
            )
        }));
        assert!(result.is_err(), "{name}");
        assert!(!called);
    }
}

#[test]
fn sixteen_bit_reads_check_exact_width_and_address_overflow() {
    let memory = MemoryImage {
        base: u64::MAX - 1,
        bytes: vec![0x34, 0x80],
    };
    assert_eq!(memory.read_u16(u64::MAX - 1), Ok(0x8034));
    let fault = memory.read_u16(u64::MAX).unwrap_err();
    assert_eq!((fault.address, fault.width), (u64::MAX, 2));
    assert!(memory.read_u16(u64::MAX - 2).is_err());
    let memory = MemoryImage {
        base: BASE,
        bytes: vec![0xff],
    };
    assert_eq!(memory.read_u16(BASE).unwrap_err().width, 2);
}
