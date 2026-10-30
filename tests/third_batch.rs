use mspaint_port::recovered::{
    call_support::{MemoryFault, MemoryImage, MissingArguments},
    forward3, unwind3, virtual3,
};
use std::collections::BTreeSet;

const BASE: u64 = 0x100_0000;
fn hex(value: &str) -> u64 {
    u64::from_str_radix(value.trim_start_matches("0x"), 16).unwrap()
}
fn bytes(value: &str) -> Vec<u8> {
    let (pairs, remainder) = value.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
fn arguments(seed: usize) -> [u64; 3] {
    [
        [0, 0, 0],
        [1, 2, 3],
        [8, 16, 32],
        [u64::MAX, u64::MAX, u64::MAX],
        [i64::MAX as u64, 1 << 63, 0xdead_beef],
        [u64::MAX - 7, 0x8000_0000, 0x1_0000_0001],
        [0x1000, 0x1234_5678_9abc_def0, 0x8765_4321_0000_0000],
        [1 << 63, 0x7fff_ffff, 0x1122_3344_5566_7788],
    ][seed]
}
fn image(seed: usize) -> MemoryImage {
    MemoryImage {
        base: BASE,
        bytes: (0..2048).map(|i| (i * 17 + seed * 31) as u8).collect(),
    }
}
fn prepare(entry: &unwind3::UnwindEntry, seed: usize, flags: u32) -> (MemoryImage, u64) {
    let mut memory = image(seed);
    let flag_address = if let Some((outer, flag)) = entry.indirect {
        memory.write_u64(BASE + outer as u64, BASE + 1024).unwrap();
        BASE + 1024 + flag as u64
    } else {
        BASE + entry.flag_offset as u64
    };
    memory.write_u32(flag_address, flags).unwrap();
    (memory, flag_address)
}

#[test]
fn all_177_additional_functions_match_the_original_c() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in include_str!("fixtures/batch-003.csv").lines() {
        let fields: Vec<&str> = line.split(',').collect();
        let name = fields[1];
        covered.insert(name);
        match fields[0] {
            "F" => {
                let seed = fields[2].parse::<usize>().unwrap();
                let (_, arity, forward) = forward3::FORWARDERS
                    .iter()
                    .find(|entry| entry.0 == name)
                    .unwrap();
                let parameters = arguments(seed);
                let mut calls = Vec::new();
                let result = forward(&parameters[..*arity], &mut |symbol, args| {
                    calls.push((symbol, args.to_vec()));
                    0xfeed_face_cafe_beef ^ seed as u64
                })
                .unwrap();
                let expected_return = if fields[3] == "-" {
                    None
                } else {
                    Some(hex(fields[3]))
                };
                assert_eq!(result, expected_return, "{line}");
                assert_eq!(calls.len(), fields[4].parse::<usize>().unwrap(), "{line}");
                assert_eq!(calls[0].0, fields[5], "{line}");
                let expected_args: Vec<u64> = fields[7]
                    .split(';')
                    .filter(|x| !x.is_empty())
                    .map(hex)
                    .collect();
                assert_eq!(expected_args.len(), fields[6].parse::<usize>().unwrap());
                assert_eq!(calls[0].1, expected_args, "{line}");
            }
            "U" => {
                let seed = fields[2].parse::<usize>().unwrap();
                let flags = hex(fields[3]) as u32;
                let entry = unwind3::HANDLERS
                    .iter()
                    .find(|entry| entry.name == name)
                    .unwrap();
                let (mut memory, flag_address) = prepare(entry, seed, flags);
                let mut calls = Vec::new();
                (entry.handler)(BASE, &mut memory, &mut |symbol, address, memory| {
                    calls.push((symbol, address, memory.read_u32(flag_address).unwrap()));
                    memory.bytes[2032] ^= 0x5a;
                })
                .unwrap();
                let expected_count = fields[4].parse::<usize>().unwrap();
                assert_eq!(calls.len(), expected_count, "{name}, seed {seed}");
                if expected_count != 0 {
                    assert_eq!(calls[0].0, fields[5], "{name}, seed {seed}");
                    assert_eq!(calls[0].1, hex(fields[6]), "{name}, seed {seed}");
                    assert_eq!(calls[0].2, hex(fields[7]) as u32, "{name}, seed {seed}");
                    assert_eq!(
                        calls[0].2 & entry.mask,
                        0,
                        "callback observed uncleared flag {name}"
                    );
                }
                assert_eq!(
                    memory.bytes,
                    bytes(fields[8]),
                    "record of {name}, seed {seed}"
                );
            }
            "V" => {
                let (_, _, invoke) = virtual3::ADAPTERS
                    .iter()
                    .find(|entry| entry.0 == name)
                    .unwrap();
                let mut calls = Vec::new();
                invoke(BASE, &mut |object, slot, args| {
                    calls.push((object, slot, args.to_vec()))
                });
                assert_eq!(calls.len(), fields[2].parse::<usize>().unwrap(), "{line}");
                assert_eq!(calls[0].0, BASE, "{line}");
                assert_eq!(calls[0].1, fields[3].parse::<usize>().unwrap(), "{line}");
                let arity = fields[4].parse::<usize>().unwrap();
                assert_eq!(calls[0].2.len(), arity, "{line}");
                if arity != 0 {
                    assert_eq!(calls[0].2, [hex(fields[5]), hex(fields[6])], "{line}");
                }
            }
            _ => panic!("unknown reference case {line}"),
        }
        cases += 1;
    }
    assert_eq!(cases, 1207);
    assert_eq!(covered.len(), 177);
    let manifest = include_str!("../translations/batch-003.json");
    let previous = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 177);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(previous.iter().all(|prior| !prior.contains(name)));
    }
}

#[test]
fn missing_arguments_never_invoke_a_dependency() {
    for &(name, required, forward) in forward3::FORWARDERS {
        for available in 0..required {
            let mut called = false;
            assert_eq!(
                forward(&vec![7; available], &mut |_, _| {
                    called = true;
                    0
                }),
                Err(MissingArguments {
                    required,
                    available
                }),
                "{name}"
            );
            assert!(!called, "{name}");
        }
    }
}

#[test]
fn clearing_cleanup_flags_prevents_repeated_destruction() {
    for entry in unwind3::HANDLERS {
        let (mut memory, flag_address) = prepare(entry, 0, entry.mask);
        let mut calls = 0;
        (entry.handler)(BASE, &mut memory, &mut |_, _, image| {
            calls += 1;
            assert_eq!(image.read_u32(flag_address).unwrap() & entry.mask, 0);
        })
        .unwrap();
        assert_eq!(calls, 1, "{}", entry.name);
        assert_eq!(memory.read_u32(flag_address).unwrap() & entry.mask, 0);
        (entry.handler)(BASE, &mut memory, &mut |_, _, _| calls += 1).unwrap();
        assert_eq!(calls, 1, "{}", entry.name);
    }
}

#[test]
fn callbacks_can_unwind_after_flags_are_cleared() {
    for entry in unwind3::HANDLERS {
        let (mut memory, flag_address) = prepare(entry, 0, entry.mask);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (entry.handler)(BASE, &mut memory, &mut |_, _, _| panic!("dependency panic")).unwrap();
        }));
        assert!(result.is_err(), "{}", entry.name);
        assert_eq!(
            memory.read_u32(flag_address).unwrap() & entry.mask,
            0,
            "{}",
            entry.name
        );
    }
}

#[test]
fn invalid_memory_returns_errors_without_invoking_dependencies() {
    for entry in unwind3::HANDLERS {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: Vec::new(),
        };
        let mut called = false;
        assert!(
            (entry.handler)(BASE, &mut memory, &mut |_, _, _| called = true).is_err(),
            "{}",
            entry.name
        );
        assert!(!called);
        assert!(memory.bytes.is_empty());
        if let Some((outer, _)) = entry.indirect {
            let mut memory = image(0);
            memory.write_u64(BASE + outer as u64, u64::MAX).unwrap();
            assert!(
                (entry.handler)(BASE, &mut memory, &mut |_, _, _| called = true).is_err(),
                "{}",
                entry.name
            );
            assert!(!called);
        }
    }
}

#[test]
fn checked_memory_rejects_overflow_underflow_and_partial_writes() {
    let mut memory = MemoryImage {
        base: BASE,
        bytes: vec![0xa5; 16],
    };
    for address in [0, BASE - 1, BASE + 13, u64::MAX] {
        assert_eq!(
            memory.read_u32(address),
            Err(MemoryFault { address, width: 4 })
        );
        assert_eq!(
            memory.write_u32(address, 0),
            Err(MemoryFault { address, width: 4 })
        );
        assert_eq!(memory.bytes, [0xa5; 16]);
    }
    assert!(memory.write_u64(BASE + 9, 0).is_err());
    assert_eq!(memory.bytes, [0xa5; 16]);
    memory.write_u32(BASE + 1, 0x1234_5678).unwrap();
    assert_eq!(memory.read_u32(BASE + 1), Ok(0x1234_5678));
    memory.write_u64(BASE + 8, u64::MAX).unwrap();
    assert_eq!(memory.read_u64(BASE + 8), Ok(u64::MAX));
    let mut high = MemoryImage {
        base: u64::MAX - 3,
        bytes: vec![0xa5; 16],
    };
    assert!(high.read_u64(u64::MAX - 3).is_err());
    assert!(high.write_u64(u64::MAX - 3, 0).is_err());
    assert_eq!(high.bytes, [0xa5; 16]);
    assert_eq!(high.read_u32(u64::MAX - 3), Ok(0xa5a5_a5a5));
}

#[test]
fn forwarders_propagate_callback_panics_instead_of_returning_success() {
    for &(_, required, forward) in forward3::FORWARDERS {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            forward(&vec![0; required], &mut |_, _| panic!("dependency failure")).unwrap();
        }));
        assert!(result.is_err());
    }
}
