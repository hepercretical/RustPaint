use mspaint_port::recovered::{adapters23, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-023-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-023.csv");

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
fn image(seed: usize, base: u64, pointers: &str, zero_ranges: &str) -> MemoryImage {
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
    for range in zero_ranges.split(';').filter(|s| !s.is_empty()) {
        let (a, b) = range.split_once(':').unwrap();
        memory.bytes[a.parse::<usize>().unwrap()..b.parse::<usize>().unwrap()].fill(0);
    }
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
fn all_88_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 704);
    assert_eq!(cases.len(), 1408);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters23::HANDLERS.iter().find(|x| x.0 == name).unwrap();
        let globals = bindings(i[5]);
        let returned_pointers = bindings(i[6]);
        let floating_returns: BTreeMap<String, u64> = i[10]
            .split(';')
            .filter(|x| !x.is_empty())
            .map(|row| {
                let (k, v) = row.split_once(':').unwrap();
                (from_hex(k), hex(v))
            })
            .collect();
        let outputs: Vec<Vec<usize>> = i[7]
            .split(';')
            .filter(|x| !x.is_empty())
            .map(|x| x.split(':').map(|n| n.parse().unwrap()).collect())
            .collect();
        for (variant, base) in [BASE, 0x1234_5678_0000].into_iter().enumerate() {
            let f: Vec<_> = expected[variant].split(',').collect();
            assert_eq!((&i[0], &i[1]), (&f[0], &f[1]));
            assert_eq!(f[6], variant.to_string());
            let mut memory = image(
                if i[9].is_empty() {
                    seed
                } else {
                    i[9].parse().unwrap()
                },
                base,
                i[4],
                i[8],
            );
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
                    if let Some(&value) = floating_returns.get(symbol) {
                        value
                    } else {
                        returned_pointers
                            .get(symbol)
                            .map_or(PAYLOADS[seed] ^ (index as u64 * 0x101), |&at| {
                                base + at as u64
                            })
                    }
                },
            );
            let result = if f[2] == "!" {
                assert!(
                    matches!(result, Err(adapters23::AdapterError::Halted(_))),
                    "halt of {name}, seed {seed}: {result:?}"
                );
                None
            } else {
                result.unwrap_or_else(|e| {
                    panic!("return of {name}, seed {seed}, base {base:x}: {e:?}")
                })
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
    assert_eq!(adapters23::HANDLERS.len(), 88);
}

#[test]
fn manifest_advances_translation_without_reusing_definitions() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 88);
    let manifest = include_str!("../translations/batch-023.json");
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
        include_str!("../translations/batch-013.json"),
        include_str!("../translations/batch-014.json"),
        include_str!("../translations/batch-015.json"),
        include_str!("../translations/batch-016.json"),
        include_str!("../translations/batch-017.json"),
        include_str!("../translations/batch-018.json"),
        include_str!("../translations/batch-019.json"),
        include_str!("../translations/batch-020.json"),
        include_str!("../translations/batch-021.json"),
        include_str!("../translations/batch-022.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 88);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 88;
    assert_eq!(total, 4278);
    assert!(total * 100 >= 4410 * 97);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters23::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters23::HANDLERS {
        let args = vec![0; arity];
        for available in 0..arity {
            let mut memory = image(2, BASE, "", "");
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
fn record_copy_preserves_overlapping_source_and_checks_both_ranges() {
    let mut memory = MemoryImage {
        base: BASE,
        bytes: (0..12).collect(),
    };
    memory.copy_record(BASE + 2, BASE, 8).unwrap();
    assert_eq!(memory.bytes, [0, 1, 0, 1, 2, 3, 4, 5, 6, 7, 10, 11]);
    let before = memory.clone();
    assert!(memory.copy_record(BASE + 8, BASE, 8).is_err());
    assert_eq!(memory, before);
    assert!(memory.copy_record(BASE, BASE + 9, 8).is_err());
    assert_eq!(memory, before);
    memory.copy_record(u64::MAX, u64::MAX, 0).unwrap();
    assert_eq!(memory, before);
}

#[test]
fn size_record_copy_keeps_signed_coordinate_bits() {
    let mut memory = MemoryImage {
        base: BASE,
        bytes: vec![0xaa; 512],
    };
    memory.write_u64(BASE + 0x30, BASE + 0x100).unwrap();
    let mut calls = Vec::new();
    let result = adapters23::fun_140036d50(
        &[BASE, BASE + 0x80],
        &mut memory,
        &mut |symbol| {
            assert_eq!(symbol, "STACK_local_res8");
            BASE + 0x180
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "BasePaint::Image::Size" => {
                    assert_eq!(args, [BASE + 0x100]);
                    0
                }
                "FromNative" => {
                    assert_eq!(args, [BASE + 0x180]);
                    memory.write_u32(args[0], (-7i32) as u32).unwrap();
                    memory.write_u32(args[0] + 4, 12345).unwrap();
                    0
                }
                _ => panic!("unexpected dependency"),
            }
        },
    )
    .unwrap();
    assert_eq!(result, Some(0));
    assert_eq!(calls, ["BasePaint::Image::Size", "FromNative"]);
    assert_eq!(memory.read_u64(BASE + 0x80).unwrap(), 0x0000_3039_ffff_fff9);
    assert_eq!(memory.bytes[0x7f], 0xaa);
    assert_eq!(memory.bytes[0x88], 0xaa);
}

#[test]
fn mouse_input_record_keeps_coordinates_padding_and_frame_aliases() {
    let mut memory = MemoryImage {
        base: BASE,
        bytes: vec![0xaa; 1024],
    };
    memory.write_u64(BASE + 0x70, BASE + 0x100).unwrap();
    memory.write_u64(BASE + 0x100, BASE + 0x180).unwrap();
    memory.write_u64(BASE + 0x1c0, 0x1234).unwrap();
    let mut calls = Vec::new();
    adapters23::fun_140095dc0(
        &[BASE, 0xffff_fff9_1234_5678],
        &mut memory,
        &mut |symbol| match symbol {
            "DAT_140179040" => BASE + 0x200,
            "STACK_auStack_58" => BASE + 0x240,
            "STACK_local_38" => BASE + 0x260,
            "STACK_LStackX_14" => BASE + 0x284,
            "STACK_local_10" => BASE + 0x288,
            _ => panic!("unexpected symbol {symbol}"),
        },
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "INDIRECT_CALL" => {
                    assert_eq!(args, [0x1234]);
                    0
                }
                "SendInput" => {
                    assert_eq!(args, [1, BASE + 0x260, 40]);
                    assert_eq!(memory.read_u32(args[1]).unwrap(), 0);
                    assert_eq!(memory.read_u32(args[1] + 4).unwrap(), 0xaaaa_aaaa);
                    assert_eq!(memory.read_u64(args[1] + 8).unwrap(), 0xffff_fff9_1234_5678);
                    assert_eq!(memory.read_u32(args[1] + 16).unwrap(), 0);
                    assert_eq!(memory.read_u64(args[1] + 20).unwrap(), 1);
                    assert_eq!(memory.read_u32(args[1] + 28).unwrap(), 0xaaaa_aaaa);
                    // LStackX_14 aliases the upper four bytes of dwExtraInfo.
                    assert_eq!(
                        memory.read_u64(args[1] + 32).unwrap(),
                        0xffff_fff9_0000_0000
                    );
                    1
                }
                _ => panic!("unexpected dependency"),
            }
        },
    )
    .unwrap();
    assert_eq!(calls, ["INDIRECT_CALL", "SendInput"]);
}

#[test]
fn five_byte_update_clears_the_fifth_byte_and_preserves_upper_three() {
    use adapters23::AdapterError;
    let mut memory = MemoryImage {
        base: BASE,
        bytes: vec![0xff; 1024],
    };
    memory.write_u64(BASE + 0x40, BASE + 0x180).unwrap();
    memory.write_u64(BASE + 0x138, 0).unwrap();
    let offsets = adapters23::FRAME_OFFSETS
        .iter()
        .find(|(name, _)| *name == "FUN_140036750")
        .unwrap()
        .1;
    let mut calls = Vec::new();
    let result = adapters23::fun_140036750(
        &[BASE, 0, BASE + 0x100],
        &mut memory,
        &mut |symbol| {
            if symbol == "DAT_140179040" {
                return BASE + 0x200;
            }
            let offset = offsets.iter().find(|(name, _)| *name == symbol).unwrap().1;
            (BASE + 0x300).wrapping_add_signed(i64::from(offset))
        },
        &mut |symbol, args, _| {
            calls.push(symbol);
            match symbol {
                "BasePaint::Image::Size" => BASE + 0x200,
                "FUN_140035f80" => {
                    assert_eq!(args, [BASE, 0, 0xffff_ff00_ffff_ffff]);
                    0
                }
                "std::_Xbad_function_call" => 0,
                _ => panic!("unexpected dependency {symbol}"),
            }
        },
    );
    assert_eq!(
        result,
        Err(AdapterError::Halted("std::_Xbad_function_call"))
    );
    assert_eq!(
        calls,
        [
            "BasePaint::Image::Size",
            "FUN_140035f80",
            "std::_Xbad_function_call"
        ]
    );
    assert_eq!(
        memory.read_u64(BASE + 0x288).unwrap(),
        0xffff_ff00_ffff_ffff
    );
}
