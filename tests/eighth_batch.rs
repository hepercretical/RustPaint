use mspaint_port::recovered::{
    call_support::MemoryImage,
    dispatch8,
    memory8::{self, Kind},
};
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
const MEMORY_CASES: &str = include_str!("fixtures/batch-008-memory.csv");
const DISPATCH_CASES: &str = include_str!("fixtures/batch-008-dispatch.csv");
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
fn canonical_word(word: u64, base: u64) -> u64 {
    match word.checked_sub(base) {
        Some(offset) if offset <= SIZE as u64 => BASE + offset,
        _ => word,
    }
}
fn canonical_image(entry: &memory8::Entry, memory: &MemoryImage) -> Vec<u8> {
    // Only reset handlers contain actual relocated pointers. Descriptor symbol
    // values and the other handlers' payload words retain their supplied bits.
    if entry.kind != Kind::Reset {
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
fn object_offset(seed: usize) -> u64 {
    1024 + (seed % 4) as u64 * 256
}
fn image(entry: &memory8::Entry, seed: usize, base: u64) -> MemoryImage {
    let mut memory = MemoryImage {
        base,
        bytes: (0..SIZE).map(|i| (i * 17 + seed * 31) as u8).collect(),
    };
    if entry.kind == Kind::Reset {
        memory
            .write_u64(base + entry.input_offset as u64, base + object_offset(seed))
            .unwrap();
    }
    memory
}

#[test]
fn all_123_memory_translations_match_original_c_at_three_bases() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in MEMORY_CASES.lines() {
        let f: Vec<_> = line.split(',').collect();
        let name = f[1];
        let seed: usize = f[2].parse().unwrap();
        let entry = memory8::HANDLERS.iter().find(|x| x.name == name).unwrap();
        let expected_image = bytes(f[10]);
        let expected_lookup = String::from_utf8(bytes(f[4])).unwrap();
        let expected_args: Vec<_> = f[8].split(';').filter(|x| !x.is_empty()).map(hex).collect();
        for base in [BASE, 0x1234_5678_0000, u64::MAX - SIZE as u64 + 1] {
            let mut memory = image(entry, seed, base);
            let mut lookups = Vec::new();
            let mut calls = Vec::new();
            let result = (entry.handler)(
                base,
                &mut memory,
                &mut |symbol| {
                    lookups.push(symbol);
                    PAYLOADS[(seed + 3) % 8]
                },
                &mut |symbol, args, memory| {
                    calls.push((
                        symbol,
                        args.iter()
                            .map(|&word| canonical_word(word, base))
                            .collect::<Vec<_>>(),
                        fingerprint(&canonical_image(entry, memory)),
                    ));
                    memory.bytes[4080] ^= 0x5a;
                    if entry.kind == Kind::Recover {
                        let returned = base + 2048 + seed as u64 * 8;
                        memory.write_u32(returned, PAYLOADS[seed] as u32).unwrap();
                        returned
                    } else {
                        0xfeed_face_cafe_beef ^ seed as u64
                    }
                },
            )
            .unwrap();
            assert_eq!(canonical_word(result, base), hex(f[3]), "return of {name}");
            let expected_lookups: Vec<_> = if expected_lookup == "-" {
                vec![]
            } else {
                vec![expected_lookup.as_str()]
            };
            assert_eq!(lookups, expected_lookups, "lookup of {name}");
            assert_eq!(
                calls.len(),
                f[5].parse::<usize>().unwrap(),
                "calls of {name}"
            );
            if calls.is_empty() {
                assert_eq!((f[6], f[7], f[8], f[9]), ("-", "0", "", "0"));
            } else {
                assert_eq!(calls[0].0, f[6], "callee of {name}");
                assert_eq!(calls[0].1.len(), f[7].parse::<usize>().unwrap());
                assert_eq!(calls[0].1, expected_args, "arguments of {name}");
                assert_eq!(calls[0].2, hex(f[9]), "callback state of {name}");
            }
            assert_eq!(
                canonical_image(entry, &memory),
                expected_image,
                "final state of {name}, seed {seed}, base {base:x}"
            );
        }
        covered.insert(name);
        cases += 1;
    }
    assert_eq!((cases, covered.len()), (984, 123));
    assert_eq!(memory8::HANDLERS.len(), 123);
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
fn all_97_dispatch_translations_match_original_c() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in DISPATCH_CASES.lines() {
        let f: Vec<_> = line.split(',').collect();
        let name = f[1];
        let seed: usize = f[2].parse().unwrap();
        let mut calls = Vec::new();
        let mut lookups = Vec::new();
        let mut relocate = |symbol| {
            lookups.push(symbol);
            PAYLOADS[(seed + 3) % 8]
        };
        let mut invoke = |symbol, args: &[u64]| {
            calls.push((symbol, args.to_vec()));
            0xfeed_face_cafe_beef ^ seed as u64
        };
        let result = match f[0] {
            "rtti" => {
                let (_, function) = dispatch8::RTTI.iter().find(|x| x.0 == name).unwrap();
                Some(function(&mut relocate))
            }
            "scalar" => {
                let (_, function) = dispatch8::SCALARS.iter().find(|x| x.0 == name).unwrap();
                Some(function(PAYLOADS[seed], FLAGS[seed], &mut invoke))
            }
            "global" => {
                let (_, function) = dispatch8::GLOBALS.iter().find(|x| x.0 == name).unwrap();
                function(&mut relocate, &mut invoke);
                None
            }
            "empty" => {
                let (_, function) = dispatch8::NO_OPS.iter().find(|x| x.0 == name).unwrap();
                function();
                None
            }
            _ => panic!("unknown case"),
        };
        let expected_lookup = String::from_utf8(bytes(f[4])).unwrap();
        let expected_lookups: Vec<_> = if expected_lookup == "-" {
            vec![]
        } else {
            vec![expected_lookup.as_str()]
        };
        assert_eq!(lookups, expected_lookups, "lookup of {name}");
        assert_eq!(
            result,
            if f[3] == "-" { None } else { Some(hex(f[3])) },
            "return of {name}, seed {seed}"
        );
        assert_eq!(calls, parse_trace(f[5]), "calls of {name}, seed {seed}");
        covered.insert(name);
        cases += 1;
    }
    assert_eq!((cases, covered.len()), (776, 97));
}

#[test]
fn manifest_and_reference_cases_reach_thirty_percent_without_duplicates() {
    let covered: BTreeSet<_> = MEMORY_CASES
        .lines()
        .chain(DISPATCH_CASES.lines())
        .map(|line| line.split(',').nth(1).unwrap())
        .collect();
    assert_eq!(covered.len(), 220);
    let manifest = include_str!("../translations/batch-008.json");
    let prior = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
        include_str!("../translations/batch-004.json"),
        include_str!("../translations/batch-005.json"),
        include_str!("../translations/batch-006.json"),
        include_str!("../translations/batch-007.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 220);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|x| !x.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|x| x.matches("\"original\":").count())
            .sum::<usize>()
        + 220;
    assert_eq!(total, 1318);
    assert!(total * 10 >= 4392 * 3);
}

#[test]
fn descriptors_retain_first_write_before_pointer_faults_or_resolver_panics() {
    for entry in memory8::HANDLERS
        .iter()
        .filter(|x| x.kind == Kind::Descriptor)
    {
        for available in 0..16 {
            let mut memory = MemoryImage {
                base: BASE,
                bytes: vec![0xa5; available],
            };
            let mut lookups = 0;
            let result = (entry.handler)(
                BASE,
                &mut memory,
                &mut |_| {
                    lookups += 1;
                    u64::MAX
                },
                &mut |_, _, _| panic!("unexpected dependency"),
            );
            assert!(result.is_err());
            assert_eq!(lookups, usize::from(available >= 4));
            if available < 4 {
                assert!(memory.bytes.iter().all(|&x| x == 0xa5));
            } else {
                assert_ne!(&memory.bytes[..4], &[0xa5; 4]);
                assert!(memory.bytes[4..].iter().all(|&x| x == 0xa5));
            }
        }
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0xa5; 16],
        };
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (entry.handler)(
                BASE,
                &mut memory,
                &mut |_| panic!("missing relocation"),
                &mut |_, _, _| panic!("unexpected dependency"),
            )
        }));
        assert!(result.is_err());
        assert_ne!(&memory.bytes[..4], &[0xa5; 4]);
        assert_eq!(&memory.bytes[4..], &[0xa5; 12]);
    }
}

#[test]
fn reset_handlers_retain_zeroed_fields_on_later_faults_and_callback_panics() {
    for entry in memory8::HANDLERS.iter().filter(|x| x.kind == Kind::Reset) {
        let mut memory = image(entry, 0, BASE);
        memory
            .write_u64(BASE + entry.input_offset as u64, BASE + SIZE as u64 - 8)
            .unwrap();
        let result = (entry.handler)(
            BASE,
            &mut memory,
            &mut |_| panic!("unexpected relocation"),
            &mut |_, _, _| panic!("invalid reset invoked dependency"),
        );
        assert!(result.is_err());
        assert_eq!(memory.read_u64(BASE + SIZE as u64 - 8), Ok(0));
        let mut memory = image(entry, 0, BASE);
        let mut observed = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (entry.handler)(
                BASE,
                &mut memory,
                &mut |_| panic!("unexpected relocation"),
                &mut |_, _, memory| {
                    assert_eq!(memory.read_u64(BASE + object_offset(0)), Ok(0));
                    assert_eq!(
                        memory.read_u16(BASE + object_offset(0) + entry.word_offset as u64),
                        Ok(0xffff)
                    );
                    memory.bytes[4080] ^= 0x5a;
                    observed = Some(memory.clone());
                    panic!("cleanup failed")
                },
            )
        }));
        assert!(result.is_err());
        assert_eq!(Some(&memory), observed.as_ref());
    }
}

#[test]
fn recoveries_load_after_callbacks_and_reject_invalid_returned_spans() {
    for entry in memory8::HANDLERS.iter().filter(|x| x.kind == Kind::Recover) {
        for returned in [0, u64::MAX - 2] {
            let mut memory = image(entry, 0, BASE);
            let mut expected = memory.clone();
            expected.bytes[4080] ^= 0x5a;
            let result = (entry.handler)(
                BASE,
                &mut memory,
                &mut |_| panic!("unexpected relocation"),
                &mut |_, _, memory| {
                    memory.bytes[4080] ^= 0x5a;
                    returned
                },
            );
            let fault = result.unwrap_err();
            assert_eq!((fault.address, fault.width), (returned, 4));
            assert_eq!(memory, expected);
        }
        let mut memory = image(entry, 0, BASE);
        let result = (entry.handler)(
            BASE,
            &mut memory,
            &mut |_| panic!("unexpected relocation"),
            &mut |_, _, memory| {
                memory.write_u32(BASE + 3072, 0x8000_ffff).unwrap();
                BASE + 3072
            },
        );
        assert_eq!(result, Ok(0));
        assert_eq!(
            memory.read_u32(BASE + entry.output_offset as u64),
            Ok(0x8000_ffff)
        );
    }
    for entry in memory8::HANDLERS.iter().filter(|x| x.kind == Kind::Byte) {
        for returned in [0, 0x100, 0x1ff, u64::MAX] {
            let mut memory = image(entry, 0, BASE);
            assert_eq!(
                (entry.handler)(
                    BASE,
                    &mut memory,
                    &mut |_| panic!("unexpected relocation"),
                    &mut |_, _, _| returned
                ),
                Ok(0)
            );
            assert_eq!(memory.bytes[entry.output_offset], returned as u8);
        }
        let mut memory = image(entry, 0, BASE);
        memory.bytes.truncate(entry.output_offset);
        let mut called = false;
        let result = (entry.handler)(
            BASE,
            &mut memory,
            &mut |_| panic!("unexpected relocation"),
            &mut |_, _, memory| {
                called = true;
                memory.bytes[0] = 0x42;
                0xff
            },
        );
        assert!(result.is_err());
        assert!(called);
        assert_eq!(memory.bytes[0], 0x42);
    }
}

#[test]
fn narrow_writes_check_exact_width_and_preserve_neighbors() {
    let mut memory = MemoryImage {
        base: u64::MAX - 3,
        bytes: vec![0xa5; 4],
    };
    memory.write_u8(u64::MAX, 0x42).unwrap();
    assert_eq!(memory.bytes, [0xa5, 0xa5, 0xa5, 0x42]);
    memory.write_u16(u64::MAX - 1, 0x8034).unwrap();
    assert_eq!(memory.bytes, [0xa5, 0xa5, 0x34, 0x80]);
    let before = memory.clone();
    let fault = memory.write_u16(u64::MAX, 0xffff).unwrap_err();
    assert_eq!(fault.width, 2);
    assert_eq!(memory, before);
    assert!(memory.write_u8(u64::MAX - 4, 0).is_err());
    assert_eq!(memory, before);
}

#[test]
fn dispatch_dependency_and_resolver_panics_prevent_later_calls() {
    for &(name, function) in dispatch8::SCALARS {
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
    for &(name, function) in dispatch8::GLOBALS {
        let mut called = false;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            function(&mut |_| panic!("missing relocation"), &mut |_, _| {
                called = true;
                0
            })
        }));
        assert!(result.is_err(), "{name}");
        assert!(!called);
    }
    for &(name, function) in dispatch8::RTTI {
        let mut lookups = 0;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            function(&mut |_| {
                lookups += 1;
                panic!("missing RTTI relocation")
            })
        }));
        assert!(result.is_err(), "{name}");
        assert_eq!(lookups, 1);
    }
}
