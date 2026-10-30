use mspaint_port::recovered::{call_support::MemoryImage, frames5};
use std::collections::BTreeSet;

const BASE: u64 = 0x100_0000;
const SIZE: usize = 4096;

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

fn object_offset(seed: usize) -> u64 {
    1024 + (seed % 4) as u64 * 256
}

fn image(entry: &frames5::FrameEntry, seed: usize, base: u64) -> MemoryImage {
    let mut memory = MemoryImage {
        base,
        bytes: (0..SIZE).map(|i| (i * 17 + seed * 31) as u8).collect(),
    };
    memory
        .write_u64(base + entry.outer_offset as u64, base + object_offset(seed))
        .unwrap();
    memory
}

// Canonicalize relocated pointer words exactly as the independent C harness
// does. The high-base case also checks valid spans ending at u64::MAX.
fn canonical_word(value: u64, base: u64) -> u64 {
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
        .flat_map(|word| canonical_word(u64::from_le_bytes(*word), memory.base).to_le_bytes())
        .collect()
}

fn fingerprint(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3)
    })
}

#[test]
fn all_219_functions_match_original_c_at_three_memory_bases() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in include_str!("fixtures/batch-005.csv").lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert_eq!(f[0], "F");
        let name = f[1];
        let seed = f[2].parse::<usize>().unwrap();
        let entry = frames5::HANDLERS.iter().find(|x| x.name == name).unwrap();
        covered.insert(name);
        let expected_args: Vec<u64> = f[6].split(';').filter(|x| !x.is_empty()).map(hex).collect();
        let expected_image = bytes(f[8]);
        for base in [BASE, 0x1234_5678_0000, u64::MAX - SIZE as u64 + 1] {
            let mut memory = image(entry, seed, base);
            let mut calls = Vec::new();
            (entry.handler)(base, &mut memory, &mut |symbol, args, memory| {
                calls.push((
                    symbol,
                    args.iter()
                        .map(|&word| canonical_word(word, base))
                        .collect::<Vec<_>>(),
                    fingerprint(&canonical_image(memory)),
                ));
                memory.bytes[4080] ^= 0x5a;
                // Original handlers discard this result.
                0xfeed_face_cafe_beef ^ seed as u64
            })
            .unwrap();
            assert_eq!(calls.len(), f[3].parse::<usize>().unwrap(), "{name}");
            assert_eq!(calls[0].0, f[4], "{name}");
            assert_eq!(calls[0].1.len(), f[5].parse::<usize>().unwrap(), "{name}");
            assert_eq!(
                calls[0].1, expected_args,
                "{name}, seed {seed}, base {base:x}"
            );
            assert_eq!(calls[0].2, hex(f[7]), "callback state of {name}");
            assert_eq!(
                canonical_image(&memory),
                expected_image,
                "final state of {name}"
            );
        }
        cases += 1;
    }
    assert_eq!(cases, 1752);
    assert_eq!(covered.len(), 219);
    assert_eq!(frames5::HANDLERS.len(), 219);
    let manifest = include_str!("../translations/batch-005.json");
    let previous = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
        include_str!("../translations/batch-004.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 219);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(previous.iter().all(|prior| !prior.contains(name)));
    }
    let total = 15
        + previous
            .iter()
            .map(|x| x.matches("\"original\":").count())
            .sum::<usize>()
        + 219;
    assert_eq!(total, 659);
    assert!(
        total * 100 >= 4392 * 15,
        "15% function coverage was not reached"
    );
}

#[test]
fn incomplete_frame_reads_and_writes_fail_before_dependency_calls() {
    for entry in frames5::HANDLERS {
        // First input read is incomplete; no scratch update can have occurred.
        let mut memory = image(entry, 0, BASE);
        memory.bytes.truncate(entry.outer_offset + 7);
        let before = memory.clone();
        assert!(
            (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
                panic!("dependency called after failed input read")
            })
            .is_err(),
            "{}",
            entry.name
        );
        assert_eq!(memory, before);

        // The input read succeeds but the first scratch field is incomplete.
        assert!(entry.scratch_offset > entry.outer_offset + 8);
        let mut memory = image(entry, 0, BASE);
        memory.bytes.truncate(entry.scratch_offset + 7);
        let before = memory.clone();
        assert!(
            (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
                panic!("dependency called after failed scratch write")
            })
            .is_err(),
            "{}",
            entry.name
        );
        assert_eq!(memory, before);
    }
}

#[test]
fn indirect_read_failure_retains_the_preceding_scratch_write() {
    let entry = frames5::HANDLERS
        .iter()
        .find(|x| x.name == "FUN_1400d912c")
        .unwrap();
    for invalid_object in [0, u64::MAX - 0x107] {
        let mut memory = image(entry, 0, BASE);
        memory
            .write_u64(BASE + entry.outer_offset as u64, invalid_object)
            .unwrap();
        let result = (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
            panic!("dependency called after failed member read")
        });
        assert!(result.is_err());
        assert_eq!(
            memory.read_u64(BASE + entry.scratch_offset as u64),
            Ok(invalid_object.wrapping_add(0x108))
        );
    }
}

#[test]
fn loaded_call_arguments_wrap_instead_of_panicking() {
    // Original body loads the saved member, then adds 0x10 before calling.
    let entry = frames5::HANDLERS
        .iter()
        .find(|x| x.name == "FUN_1400d9668")
        .unwrap();
    for word in [0, 1, u64::MAX - 15, u64::MAX, 0x8000_0000_0000_0000] {
        let mut memory = image(entry, 0, BASE);
        memory
            .write_u64(BASE + object_offset(0) + entry.member_offset as u64, word)
            .unwrap();
        let mut called = 0;
        (entry.handler)(BASE, &mut memory, &mut |symbol, args, memory| {
            called += 1;
            assert_eq!(symbol, "thunk_FUN_140008c50");
            assert_eq!(args, &[word.wrapping_add(0x10)]);
            assert_eq!(memory.read_u64(BASE + 0x20), Ok(word));
            memory.bytes[4080] = 0x42;
            u64::MAX
        })
        .unwrap();
        assert_eq!(called, 1);
        assert_eq!(memory.bytes[4080], 0x42);
    }
}

#[test]
fn callback_panics_retain_completed_scratch_and_callback_writes() {
    for entry in frames5::HANDLERS {
        let mut memory = image(entry, 0, BASE);
        let mut observed = None;
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (entry.handler)(BASE, &mut memory, &mut |_, _, memory| {
                memory.bytes[4080] ^= 0x5a;
                observed = Some(memory.clone());
                panic!("dependency failed")
            })
        }));
        assert!(result.is_err(), "{}", entry.name);
        assert_eq!(Some(&memory), observed.as_ref(), "{}", entry.name);
        assert_eq!(
            memory.read_u64(BASE + entry.scratch_offset as u64),
            Ok(BASE + object_offset(0) + entry.member_offset as u64)
        );
    }
}
