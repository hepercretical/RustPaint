use mspaint_port::recovered::{call_support::MemoryImage, errors4, frames4};
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
        .map(|x| u8::from_str_radix(std::str::from_utf8(x).unwrap(), 16).unwrap())
        .collect()
}
fn image(seed: usize) -> MemoryImage {
    MemoryImage {
        base: BASE,
        bytes: (0..2048).map(|i| (i * 17 + seed * 31) as u8).collect(),
    }
}
fn fingerprint(image: &MemoryImage) -> u64 {
    image
        .bytes
        .iter()
        .fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
        })
}
fn frame_image(entry: &frames4::FrameEntry, seed: usize) -> MemoryImage {
    let mut memory = image(seed);
    memory
        .write_u64(BASE + entry.outer_offset as u64, BASE + 1024)
        .unwrap();
    memory
}

#[test]
fn all_48_additions_match_original_c_and_reach_ten_percent() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in include_str!("fixtures/batch-004.csv").lines() {
        let f: Vec<&str> = line.split(',').collect();
        let name = f[1];
        covered.insert(name);
        let seed = f[2].parse::<usize>().unwrap();
        let mut calls = Vec::new();
        let (memory, offset) = match f[0] {
            "E" => {
                let (_, construct) = errors4::INITIALIZERS
                    .iter()
                    .find(|entry| entry.0 == name)
                    .unwrap();
                let mut memory = image(seed);
                let result = construct(
                    BASE,
                    hex(f[3]) as u32,
                    hex(f[4]),
                    &mut memory,
                    &mut |symbol, args, memory| {
                        calls.push((symbol, args.to_vec(), fingerprint(memory)));
                        memory.bytes[2032] ^= 0x5a;
                        0xfeed_face_cafe_beef ^ seed as u64
                    },
                )
                .unwrap();
                assert_eq!(result, hex(f[5]), "{name}, seed {seed}");
                (memory, 6)
            }
            "F" => {
                let entry = frames4::HANDLERS
                    .iter()
                    .find(|entry| entry.name == name)
                    .unwrap();
                let mut memory = frame_image(entry, seed);
                (entry.handler)(BASE, &mut memory, &mut |symbol, args, memory| {
                    calls.push((symbol, args.to_vec(), fingerprint(memory)));
                    memory.bytes[2032] ^= 0x5a;
                    0xfeed_face_cafe_beef ^ seed as u64
                })
                .unwrap();
                (memory, 3)
            }
            _ => panic!("unknown case {line}"),
        };
        assert_eq!(calls.len(), f[offset].parse::<usize>().unwrap(), "{name}");
        assert_eq!(calls[0].0, f[offset + 1], "{name}");
        assert_eq!(
            calls[0].1.len(),
            f[offset + 2].parse::<usize>().unwrap(),
            "{name}"
        );
        let expected_args: Vec<u64> = f[offset + 3]
            .split(';')
            .filter(|x| !x.is_empty())
            .map(hex)
            .collect();
        assert_eq!(
            calls[0].1, expected_args,
            "arguments of {name}, seed {seed}"
        );
        assert_eq!(
            calls[0].2,
            hex(f[offset + 4]),
            "callback-time state of {name}, seed {seed}"
        );
        assert_eq!(
            memory.bytes,
            bytes(f[offset + 5]),
            "final record of {name}, seed {seed}"
        );
        cases += 1;
    }
    assert_eq!(cases, 384);
    assert_eq!(covered.len(), 48);
    let manifest = include_str!("../translations/batch-004.json");
    let previous = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 48);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(previous.iter().all(|prior| !prior.contains(name)));
    }
    let total = 15
        + previous
            .iter()
            .map(|prior| prior.matches("\"original\":").count())
            .sum::<usize>()
        + 48;
    assert_eq!(total, 440);
    assert!(total * 10 >= 4392, "10% function coverage was not reached");
}

#[test]
fn short_error_records_fail_before_dependency_calls_and_retain_prior_writes() {
    for &(name, construct) in errors4::INITIALIZERS {
        for available in 0..24 {
            let mut memory = MemoryImage {
                base: BASE,
                bytes: vec![0xa5; available],
            };
            let mut called = false;
            assert!(
                construct(BASE, 7, 8, &mut memory, &mut |_, _, _| {
                    called = true;
                    0
                })
                .is_err(),
                "{name}, size {available}"
            );
            assert!(!called);
            if available >= 12 {
                assert_eq!(memory.read_u64(BASE), Ok(0));
                assert_eq!(memory.read_u32(BASE + 8), Ok(0xaabb_ccdd));
            } else {
                assert!(memory.bytes.iter().all(|&byte| byte == 0xa5));
            }
        }
    }
}

#[test]
fn error_initializers_preserve_callback_mutations_and_return_record_address() {
    for &(name, construct) in errors4::INITIALIZERS {
        let mut memory = image(0);
        let mut called = 0;
        let result = construct(
            BASE,
            0x8007_0005,
            u64::MAX,
            &mut memory,
            &mut |symbol, args, memory| {
                called += 1;
                assert_eq!(symbol, "FUN_1400021a0");
                assert_eq!(args[0], BASE);
                assert_eq!(args[2], 0);
                assert_eq!(args[3], u64::MAX);
                assert_eq!(memory.read_u64(BASE), Ok(0));
                assert_eq!(memory.read_u32(BASE + 8), Ok(0xaabb_ccdd));
                assert_eq!(memory.read_u32(BASE + 12), Ok(args[1] as u32));
                assert_eq!(memory.read_u64(BASE + 16), Ok(0));
                memory.write_u32(BASE + 12, 0xf00d_dead).unwrap();
                0x1122_3344_5566_7788
            },
        );
        assert_eq!(result, Ok(BASE), "{name}");
        assert_eq!(called, 1);
        assert_eq!(memory.read_u32(BASE + 12), Ok(0xf00d_dead));
    }
}

#[test]
fn frame_access_failures_do_not_invoke_dependencies() {
    for entry in frames4::HANDLERS {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: Vec::new(),
        };
        let mut called = false;
        assert!(
            (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
                called = true;
                0
            })
            .is_err(),
            "{}",
            entry.name
        );
        assert!(!called);
        assert!(memory.bytes.is_empty());
    }
    // This handler dereferences the saved member after recording its address.
    // Preserve that first scratch write if the subsequent read is invalid.
    let entry = frames4::HANDLERS
        .iter()
        .find(|entry| entry.name == "FUN_1400d83f0")
        .unwrap();
    let mut memory = frame_image(entry, 0);
    memory
        .write_u64(BASE + entry.outer_offset as u64, 0)
        .unwrap();
    let mut called = false;
    assert!((entry.handler)(BASE, &mut memory, &mut |_, _, _| {
        called = true;
        0
    })
    .is_err());
    assert!(!called);
    assert_eq!(
        memory.read_u64(BASE + entry.scratch_offset as u64),
        Ok(entry.member_offset as u64)
    );
}

#[test]
fn dependency_panics_preserve_completed_record_updates() {
    for &(name, construct) in errors4::INITIALIZERS {
        let mut memory = image(0);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            construct(BASE, 0x8007_0005, 0, &mut memory, &mut |_, _, _| {
                panic!("dependency failed")
            })
        }));
        assert!(result.is_err(), "{name}");
        assert_eq!(memory.read_u64(BASE), Ok(0));
        assert_eq!(memory.read_u32(BASE + 8), Ok(0xaabb_ccdd));
        assert_eq!(memory.read_u64(BASE + 16), Ok(0));
    }
    for entry in frames4::HANDLERS {
        let mut memory = frame_image(entry, 0);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
                panic!("dependency failed")
            })
        }));
        assert!(result.is_err(), "{}", entry.name);
        assert_eq!(
            memory.read_u64(BASE + entry.scratch_offset as u64),
            Ok(BASE + 1024 + entry.member_offset as u64)
        );
    }
}
