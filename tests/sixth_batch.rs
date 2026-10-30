use mspaint_port::recovered::{call_support::MemoryImage, frames6};
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

fn image(entry: &frames6::FrameEntry, seed: usize, base: u64) -> MemoryImage {
    let mut memory = MemoryImage {
        base,
        bytes: (0..SIZE).map(|i| (i * 17 + seed * 31) as u8).collect(),
    };
    memory
        .write_u64(
            base + entry.input_offset as u64,
            if entry.scratch.is_some() {
                base + object_offset(seed)
            } else {
                PAYLOADS[seed]
            },
        )
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

fn canonical_image(entry: &frames6::FrameEntry, memory: &MemoryImage) -> Vec<u8> {
    // Allocation arguments are opaque words, even if a value happens to fall
    // within the relocated image's numeric address range.
    if entry.scratch.is_none() {
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

#[test]
fn all_220_functions_match_original_c_at_three_memory_bases() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in include_str!("fixtures/batch-006.csv").lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert_eq!(f[0], "F");
        let name = f[1];
        let seed = f[2].parse::<usize>().unwrap();
        let entry = frames6::HANDLERS.iter().find(|x| x.name == name).unwrap();
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
                        .map(|&word| {
                            if entry.scratch.is_some() {
                                canonical_word(word, base)
                            } else {
                                word
                            }
                        })
                        .collect::<Vec<_>>(),
                    fingerprint(&canonical_image(entry, memory)),
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
                canonical_image(entry, &memory),
                expected_image,
                "final state of {name}"
            );
        }
        cases += 1;
    }
    assert_eq!(cases, 1760);
    assert_eq!(covered.len(), 220);
    assert_eq!(frames6::HANDLERS.len(), 220);
    let manifest = include_str!("../translations/batch-006.json");
    let previous = [
        include_str!("../translations/batch-001.json"),
        include_str!("../translations/batch-002.json"),
        include_str!("../translations/batch-003.json"),
        include_str!("../translations/batch-004.json"),
        include_str!("../translations/batch-005.json"),
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
    assert_eq!(total, 879);
    assert!(
        total * 100 >= 4392 * 20,
        "20% function coverage was not reached"
    );
}

#[test]
fn truncated_input_reads_and_scratch_writes_never_invoke_dependencies() {
    let mut scratch_handlers = 0;
    let mut release_handlers = 0;
    for entry in frames6::HANDLERS {
        for available in [0, entry.input_offset, entry.input_offset + 7] {
            let mut memory = image(entry, 0, BASE);
            memory.bytes.truncate(available);
            let before = memory.clone();
            assert!(
                (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
                    panic!("dependency invoked after invalid input read")
                })
                .is_err(),
                "{}",
                entry.name
            );
            assert_eq!(memory, before);
        }
        if let Some((scratch_offset, _)) = entry.scratch {
            scratch_handlers += 1;
            assert!(scratch_offset >= entry.input_offset + 8);
            let mut memory = image(entry, 0, BASE);
            memory.bytes.truncate(scratch_offset + 7);
            let before = memory.clone();
            assert!(
                (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
                    panic!("dependency invoked after invalid scratch write")
                })
                .is_err(),
                "{}",
                entry.name
            );
            assert_eq!(memory, before);
        } else {
            release_handlers += 1;
        }
    }
    assert_eq!((scratch_handlers, release_handlers), (143, 77));
}

#[test]
fn null_release_arguments_are_forwarded_with_the_original_size() {
    let expected_calls = include_str!("fixtures/batch-006.csv");
    for entry in frames6::HANDLERS.iter().filter(|x| x.scratch.is_none()) {
        let mut memory = image(entry, 0, BASE);
        let before = memory.clone();
        let fields: Vec<_> = expected_calls
            .lines()
            .find(|line| line.starts_with(&format!("F,{},0,", entry.name)))
            .unwrap()
            .split(',')
            .collect();
        let expected: Vec<_> = fields[6]
            .split(';')
            .filter(|x| !x.is_empty())
            .map(hex)
            .collect();
        let mut called = 0;
        (entry.handler)(BASE, &mut memory, &mut |symbol, args, memory| {
            called += 1;
            assert_eq!(symbol, "FUN_1400b022c");
            assert_eq!(args, expected);
            assert_eq!(args[0], 0);
            assert_ne!(args[1], 0);
            assert_eq!(*memory, before);
            // Mutating the input field must persist after the supplied call.
            memory
                .write_u64(BASE + entry.input_offset as u64, u64::MAX)
                .unwrap();
            0x1234_5678
        })
        .unwrap();
        assert_eq!(called, 1);
        assert_eq!(
            memory.read_u64(BASE + entry.input_offset as u64),
            Ok(u64::MAX)
        );
    }
}

#[test]
fn indirect_read_failure_preserves_completed_pointer_write() {
    let entry = frames6::HANDLERS
        .iter()
        .find(|x| x.name == "FUN_1400dcb69")
        .unwrap();
    let (scratch_offset, member_offset) = entry.scratch.unwrap();
    for invalid in [0, u64::MAX - member_offset as u64 + 1] {
        let mut memory = image(entry, 0, BASE);
        memory
            .write_u64(BASE + entry.input_offset as u64, invalid)
            .unwrap();
        let result = (entry.handler)(BASE, &mut memory, &mut |_, _, _| {
            panic!("dependency invoked after invalid indirect read")
        });
        let address = invalid.wrapping_add(member_offset as u64);
        assert_eq!(result.unwrap_err().address, address);
        assert_eq!(memory.read_u64(BASE + scratch_offset as u64), Ok(address));
    }
}

#[test]
fn loaded_arguments_keep_wrapping_machine_arithmetic() {
    let entry = frames6::HANDLERS
        .iter()
        .find(|x| x.name == "FUN_1400dd608")
        .unwrap();
    let (_, member_offset) = entry.scratch.unwrap();
    for word in [0, 1, u64::MAX - 15, u64::MAX, 0x8000_0000_0000_0000] {
        let mut memory = image(entry, 0, BASE);
        memory
            .write_u64(BASE + object_offset(0) + member_offset as u64, word)
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
fn callback_panics_preserve_handler_and_callback_updates() {
    for entry in frames6::HANDLERS {
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
        if let Some((scratch_offset, member_offset)) = entry.scratch {
            assert_eq!(
                memory.read_u64(BASE + scratch_offset as u64),
                Ok(BASE + object_offset(0) + member_offset as u64)
            );
        }
    }
}
