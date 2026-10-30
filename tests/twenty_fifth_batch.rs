use mspaint_port::recovered::{adapters25, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-025-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-025.csv");

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
fn all_22_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 176);
    assert_eq!(cases.len(), 352);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters25::HANDLERS.iter().find(|x| x.0 == name).unwrap();
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
                    matches!(result, Err(adapters25::AdapterError::Halted(_))),
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
    assert_eq!(adapters25::HANDLERS.len(), 22);
}

#[test]
fn manifest_advances_translation_without_reusing_definitions() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 22);
    let manifest = include_str!("../translations/batch-025.json");
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
        include_str!("../translations/batch-023.json"),
        include_str!("../translations/batch-024.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 22);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 22;
    assert_eq!(total, 4344);
    assert!(total * 1000 >= 4410 * 985);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters25::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters25::HANDLERS {
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
fn format_fill_and_alignment_write_the_recovered_record_fields() {
    let mut memory = MemoryImage {
        base: BASE,
        bytes: vec![0xaa; 2048],
    };
    memory.write_u16(BASE + 0x100, b'*' as u16).unwrap();
    memory.write_u16(BASE + 0x102, b'>' as u16).unwrap();
    memory.write_u64(BASE + 0x200, BASE + 0x300).unwrap();
    let mut calls = Vec::new();
    let result = adapters25::fun_140027140(
        &[BASE + 0x100, BASE + 0x104, BASE + 0x200],
        &mut memory,
        &mut |_| panic!("unexpected lookup"),
        &mut |symbol, args, memory| {
            calls.push(symbol);
            match symbol {
                "memmove" => memory
                    .copy_record(args[0], args[1], args[2] as usize)
                    .unwrap(),
                "memset" => {
                    for i in 0..args[2] {
                        memory.write_u8(args[0] + i, args[1] as u8).unwrap();
                    }
                }
                _ => panic!("unexpected dependency {symbol}"),
            }
            args[0]
        },
    )
    .unwrap();
    assert_eq!(result, Some(BASE + 0x104));
    assert_eq!(calls, ["memmove", "memset"]);
    assert_eq!(memory.bytes[0x309], 2);
    assert_eq!(memory.bytes[0x30e], 1);
    assert_eq!(&memory.bytes[0x310..0x314], &[b'*', 0, 0, 0]);
    assert_eq!(memory.bytes[0x308], 0xaa);
    assert_eq!(memory.bytes[0x314], 0xaa);
}

#[test]
fn incomplete_surrogate_and_brace_fill_halt_before_any_record_store() {
    use adapters25::AdapterError;
    for units in [vec![0xd800u16], vec![b'{' as u16, b'^' as u16]] {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0xaa; 2048],
        };
        for (i, &unit) in units.iter().enumerate() {
            memory.write_u16(BASE + 0x100 + i as u64 * 2, unit).unwrap();
        }
        memory.write_u64(BASE + 0x200, BASE + 0x300).unwrap();
        let before = memory.clone();
        let mut calls = Vec::new();
        let result = adapters25::fun_140027140(
            &[
                BASE + 0x100,
                BASE + 0x100 + units.len() as u64 * 2,
                BASE + 0x200,
            ],
            &mut memory,
            &mut |symbol| {
                assert!(symbol.starts_with('"'));
                BASE + 0x400
            },
            &mut |symbol, args, _| {
                calls.push(symbol);
                assert_eq!(args, [BASE + 0x400]);
                0
            },
        );
        assert_eq!(result, Err(AdapterError::Halted("FUN_140032060")));
        assert_eq!(calls, ["FUN_140032060"]);
        assert_eq!(memory, before);
    }
}

#[test]
fn heap_free_full_return_word_supplies_the_missing_upper_register_bits() {
    let offsets = adapters25::FRAME_OFFSETS
        .iter()
        .find(|x| x.0 == "FUN_14006a430")
        .unwrap()
        .1;
    for returned in [0, 1, u64::MAX, 0xfeed_face_1234_56ab] {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0; 4096],
        };
        memory.write_u64(BASE + 0x100, BASE + 0x300).unwrap();
        memory.write_u32(BASE + 0x318, 1).unwrap();
        let mut calls = Vec::new();
        let result = adapters25::fun_14006a430(
            &[BASE + 0x100, BASE + 0x200],
            &mut memory,
            &mut |symbol| match symbol {
                "DAT_140179040" => BASE + 0x700,
                "FUN_1400c85b0" => 0x1111,
                "FUN_1400c85c0" => 0x2222,
                _ => {
                    let offset = offsets.iter().find(|x| x.0 == symbol).unwrap().1;
                    (BASE + 0x900).wrapping_add_signed(i64::from(offset))
                }
            },
            &mut |symbol, args, memory| {
                calls.push(symbol);
                match symbol {
                    "operator_new" => {
                        assert_eq!(args, [0x180]);
                        BASE + 0x500
                    }
                    "FUN_1400c85c0" => {
                        assert_eq!(args, [BASE + 0x500]);
                        memory.write_u64(BASE + 0x100, BASE + 0x300).unwrap();
                        0
                    }
                    "GetProcessHeap" => 0x777,
                    "HeapFree" => {
                        assert_eq!(args, [0x777, 0, BASE + 0x300]);
                        returned
                    }
                    _ => panic!("unexpected dependency {symbol}"),
                }
            },
        )
        .unwrap();
        assert_eq!(result, Some(returned & !0xff));
        assert_eq!(memory.read_u32(BASE + 0x318).unwrap(), 0);
        assert_eq!(memory.read_u64(BASE + 0x100).unwrap(), 0);
        assert_eq!(
            calls,
            [
                "operator_new",
                "FUN_1400c85c0",
                "GetProcessHeap",
                "HeapFree"
            ]
        );
    }
}
