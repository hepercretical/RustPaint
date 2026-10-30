use mspaint_port::recovered::{adapters27, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-027-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-027.csv");

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
fn all_10_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 80);
    assert_eq!(cases.len(), 160);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters27::HANDLERS.iter().find(|x| x.0 == name).unwrap();
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
            for patch in i[11].split(';').filter(|s| !s.is_empty()) {
                let (at, data) = patch.split_once(':').unwrap();
                let at: usize = at.parse().unwrap();
                for (offset, pair) in data.as_bytes().as_chunks::<2>().0.iter().enumerate() {
                    memory.bytes[at + offset] =
                        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap();
                }
            }
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
                            let (address, target) = if row.len() == 4 {
                                (args[row[1]] + row[2] as u64, row[3])
                            } else {
                                (
                                    memory.read_u64(args[row[1]] + row[2] as u64).unwrap()
                                        + row[3] as u64,
                                    row[4],
                                )
                            };
                            memory.write_u64(address, base + target as u64).unwrap();
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
                    matches!(result, Err(adapters27::AdapterError::Halted(_))),
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
    assert_eq!(adapters27::HANDLERS.len(), 10);
}

#[test]
fn manifest_advances_translation_without_reusing_definitions() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 10);
    let manifest = include_str!("../translations/batch-027.json");
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
        include_str!("../translations/batch-025.json"),
        include_str!("../translations/batch-026.json"),
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 10);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior
            .iter()
            .all(|s| !s.contains(&format!("\"original\": \"{name}\""))));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 10;
    assert_eq!(total, 4376);
    assert!(total * 100 >= 4410 * 99);
    let added_lines: usize = manifest
        .split("\"source_lines\": ")
        .skip(1)
        .map(|part| {
            part.split(',')
                .next()
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    assert_eq!(added_lines, 4221);
    assert!((154987 + added_lines) * 100 >= 187218 * 85);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters27::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters27::HANDLERS {
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
fn bitmap_frame(symbol: &str) -> u64 {
    let (_, slots) = adapters27::FRAME_OFFSETS
        .iter()
        .find(|(name, _)| *name == "FUN_140045890")
        .unwrap();
    let (_, offset) = slots.iter().find(|(name, _)| *name == symbol).unwrap();
    (BASE + 0x3000).wrapping_add_signed(i64::from(*offset))
}
fn bitmap_memory() -> MemoryImage {
    MemoryImage {
        base: BASE,
        bytes: vec![0xaa; 16384],
    }
}
fn bitmap_lookup(symbol: &str) -> u64 {
    if symbol.starts_with("STACK_") {
        bitmap_frame(symbol)
    } else {
        assert_eq!(symbol, "DAT_140179040");
        BASE + 0x3f00
    }
}
fn fill_bitmap(image: &mut MemoryImage, address: u64) {
    image.write_u32(address, 0).unwrap();
    image.write_u32(address + 4, 37).unwrap();
    image.write_u32(address + 8, 11).unwrap();
    image.write_u32(address + 12, 148).unwrap();
    image.write_u16(address + 16, 1).unwrap();
    image.write_u16(address + 18, 32).unwrap();
    image.write_u64(address + 24, 0).unwrap();
}

#[test]
fn bitmap_header_keeps_dimensions_depth_and_dpi_before_allocation_failure() {
    for (bits, expected) in [
        (0, 32),
        (1, 1),
        (2, 4),
        (4, 4),
        (5, 8),
        (8, 8),
        (9, 24),
        (24, 24),
        (25, 32),
        (32, 32),
    ] {
        let mut memory = bitmap_memory();
        let mut saw_header = false;
        let result = adapters27::fun_140045890(
            &[BASE + 0x100, 0x123, bits, BASE + 0x108, 96, 144],
            &mut memory,
            &mut bitmap_lookup,
            &mut |symbol, args, image| match symbol {
                "GetObjectW" => {
                    assert_eq!(&args[..2], &[0x123, 32]);
                    fill_bitmap(image, args[2]);
                    32
                }
                "PaletteSize" => {
                    saw_header = true;
                    let at = args[0];
                    assert_eq!(image.read_u32(at).unwrap(), 40);
                    assert_eq!(image.read_u32(at + 4).unwrap(), 37);
                    assert_eq!(image.read_u32(at + 8).unwrap(), 11);
                    assert_eq!(image.read_u16(at + 12).unwrap(), 1);
                    assert_eq!(image.read_u16(at + 14).unwrap(), expected);
                    assert_eq!(image.read_u32(at + 24).unwrap(), 96);
                    assert_eq!(image.read_u32(at + 28).unwrap(), 144);
                    assert_eq!(image.read_u64(at + 32).unwrap(), 0);
                    0
                }
                "GlobalAlloc" => {
                    assert_eq!(args, &[0x40, 40]);
                    0
                }
                _ => panic!("unexpected call {symbol}"),
            },
        )
        .unwrap();
        assert_eq!(result, Some(BASE + 0x100));
        assert!(saw_header);
        assert_eq!(memory.read_u64(BASE + 0x100).unwrap(), 0);
        assert_eq!(memory.read_u32(BASE + 0x108).unwrap(), 0xaaaa_aaaa);
    }
}

#[test]
fn bitmap_success_computes_padded_size_restores_header_and_releases_gdi_objects() {
    let mut memory = bitmap_memory();
    let mut allocations = 0;
    let mut reads = 0;
    let mut selections = 0;
    let mut cleanup = Vec::new();
    let result = adapters27::fun_140045890(
        &[BASE + 0x100, 0x123, 24, BASE + 0x108, 96, 144],
        &mut memory,
        &mut bitmap_lookup,
        &mut |symbol, args, image| match symbol {
            "GetObjectW" => {
                fill_bitmap(image, args[2]);
                32
            }
            "PaletteSize" => 0,
            "GlobalAlloc" => {
                allocations += 1;
                assert_eq!(args, &[0x40, if allocations == 1 { 40 } else { 1272 }]);
                if allocations == 1 {
                    BASE + 0x1000
                } else {
                    BASE + 0x1800
                }
            }
            "CreateBitmap" => {
                assert_eq!(args, &[2, 2, 1, 32, 0]);
                0x9000
            }
            "CreateCompatibleDC" => {
                assert_eq!(args, &[0]);
                0x9100
            }
            "SelectObject" => {
                selections += 1;
                assert_eq!(
                    args,
                    &[0x9100, if selections == 1 { 0x9000 } else { 0x9200 }]
                );
                0x9200
            }
            "GetDIBits" => {
                reads += 1;
                assert_eq!(args[0], 0x9100);
                assert_eq!(args[1], 0x123);
                assert_eq!(args[2], 0);
                assert_eq!(args[3], 11);
                if reads == 1 {
                    assert_eq!(args[4], 0);
                    assert_eq!(args[5], BASE + 0x1000);
                } else {
                    assert_eq!(args[4], BASE + 0x1800 + 40);
                    // The fallback sizes the allocation; the copied source header still has zero.
                    assert_eq!(image.read_u32(args[5] + 20).unwrap(), 0);
                    image.write_u32(args[5] + 20, 0xdead_beef).unwrap();
                }
                11
            }
            "memcpy" => {
                assert_eq!(args, &[BASE + 0x1800, BASE + 0x1000, 40]);
                image.copy_record(args[0], args[1], 40).unwrap();
                args[0]
            }
            "GlobalFree" => {
                assert_eq!(args, &[BASE + 0x1000]);
                cleanup.push(symbol);
                0
            }
            "DeleteObject" => {
                assert_eq!(args, &[0x9000]);
                cleanup.push(symbol);
                1
            }
            "DeleteDC" => {
                assert_eq!(args, &[0x9100]);
                cleanup.push(symbol);
                1
            }
            _ => panic!("unexpected call {symbol}"),
        },
    )
    .unwrap();
    assert_eq!(result, Some(BASE + 0x100));
    assert_eq!(memory.read_u64(BASE + 0x100).unwrap(), BASE + 0x1800);
    assert_eq!(memory.read_u32(BASE + 0x108).unwrap(), 1272);
    assert_eq!(memory.read_u32(BASE + 0x1800 + 20).unwrap(), 0);
    assert_eq!(allocations, 2);
    assert_eq!(reads, 2);
    assert_eq!(selections, 2);
    assert_eq!(cleanup, ["GlobalFree", "DeleteObject", "DeleteDC"]);
}
