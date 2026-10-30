use mspaint_port::recovered::{adapters26, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-026-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-026.csv");

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
        let (_, arity, function) = adapters26::HANDLERS.iter().find(|x| x.0 == name).unwrap();
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
                    matches!(result, Err(adapters26::AdapterError::Halted(_))),
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
    assert_eq!(adapters26::HANDLERS.len(), 22);
}

#[test]
fn manifest_advances_translation_without_reusing_definitions() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 22);
    let manifest = include_str!("../translations/batch-026.json");
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
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 22);
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
        + 22;
    assert_eq!(total, 4366);
    assert!(total * 100 >= 4410 * 99);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters26::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters26::HANDLERS {
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
fn frame_address(function: &str, symbol: &str) -> u64 {
    let (_, slots) = adapters26::FRAME_OFFSETS
        .iter()
        .find(|(name, _)| *name == function)
        .unwrap();
    let (_, offset) = slots.iter().find(|(name, _)| *name == symbol).unwrap();
    (BASE + 0xc00).wrapping_add_signed(i64::from(*offset))
}

#[test]
fn graphics_constructor_receives_output_slot_and_preserves_record_padding() {
    let mut memory = MemoryImage {
        base: BASE,
        bytes: vec![0xaa; 4096],
    };
    for (at, value) in [
        (0x108, BASE + 0x200),
        (0x240, BASE + 0x300),
        (0x110, BASE + 0x400),
        (0x118, BASE + 0x280),
        (0x288, 0x123),
        (0x500, 0x9000),
    ] {
        memory.write_u64(BASE + at, value).unwrap();
    }
    memory.write_u8(BASE + 0x400, 0).unwrap();
    memory.write_u32(BASE + 0x680, 100).unwrap();
    memory.write_u32(BASE + 0x684, 200).unwrap();
    let mut events = Vec::new();
    let result = adapters26::fun_1400375c0(
        &[BASE + 0x100, BASE + 0x800],
        &mut memory,
        &mut |symbol| {
            if symbol.starts_with("STACK_") {
                frame_address("FUN_1400375c0", symbol)
            } else {
                assert_eq!(symbol, "DAT_140179040");
                BASE + 0xf00
            }
        },
        &mut |symbol, args, image| {
            events.push(symbol);
            match symbol {
                "BasePaint::Image::Graphics" => {
                    assert_eq!(
                        args,
                        &[
                            BASE + 0x300,
                            frame_address("FUN_1400375c0", "STACK_local_28")
                        ]
                    );
                    image.write_u64(args[1], BASE + 0x500).unwrap();
                    0
                }
                "GdipGetImageHeight" | "GdipGetImageWidth" => {
                    assert_eq!(args[0], 0x123);
                    image
                        .write_u32(args[1], if symbol.ends_with("Height") { 25 } else { 50 })
                        .unwrap();
                    0
                }
                "GdipDrawImageRectI" => {
                    assert_eq!(args, &[0x9000, 0x123, 0, 0]);
                    0
                }
                "BasePaint::Image::Size" => {
                    assert_eq!(args, &[BASE + 0x300]);
                    BASE + 0x680
                }
                "GdipDeleteGraphics" => {
                    assert_eq!(args, &[0x9000]);
                    0
                }
                "GdipFree" => {
                    assert_eq!(args, &[BASE + 0x500]);
                    0
                }
                _ => panic!("unexpected call {symbol}"),
            }
        },
    )
    .unwrap();
    assert_eq!(result, Some(BASE + 0x800));
    assert_eq!(memory.read_u32(BASE + 0x800).unwrap(), 0xaaaa_aa01);
    assert_eq!(memory.read_u64(BASE + 0x804).unwrap(), 0);
    assert_eq!(memory.read_u32(BASE + 0x80c).unwrap(), 100);
    assert_eq!(memory.read_u32(BASE + 0x810).unwrap(), 200);
    assert_eq!(memory.read_u8(BASE + 0x7ff).unwrap(), 0xaa);
    assert_eq!(memory.read_u8(BASE + 0x814).unwrap(), 0xaa);
    assert_eq!(
        events,
        [
            "BasePaint::Image::Graphics",
            "GdipGetImageHeight",
            "GdipGetImageWidth",
            "GdipDrawImageRectI",
            "BasePaint::Image::Size",
            "GdipDeleteGraphics",
            "GdipFree"
        ]
    );
}

#[test]
fn double_hex_formatter_uses_ieee_register_bits_and_signed_exponents() {
    for (value, expected) in [
        (0.0, "0p+0"),
        (1.0, "1p+0"),
        (2.0, "1p+1"),
        (0.5, "1p-1"),
        (1.5, "1.8p+0"),
        (-1.5, "-1.8p+0"),
        (-0.0, "-0p+0"),
    ] {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0xaa; 4096],
        };
        memory.write_u64(BASE + 0x400, f64::to_bits(value)).unwrap();
        for (i, byte) in b"0123456789abcdefghijklmnopqrstuvwxyzinf\0"
            .iter()
            .enumerate()
        {
            memory.write_u8(BASE + 0x500 + i as u64, *byte).unwrap();
        }
        memory
            .write_u32(BASE + 0x540, u32::from_le_bytes(*b"0p+0"))
            .unwrap();
        let result = adapters26::fun_14002da40(
            &[BASE + 0x100, BASE + 0x200, BASE + 0x280, 0, 4],
            &mut memory,
            &mut |symbol| match symbol {
                "REGISTER_in_XMM3" => BASE + 0x400,
                "\"0p+0\"" => BASE + 0x540,
                "\"0123456789abcdefghijklmnopqrstuvwxyzinf\"" => BASE + 0x500,
                _ => panic!("unexpected symbol {symbol}"),
            },
            &mut |symbol, args, image| {
                match symbol {
                    "builtin_strncpy" => {
                        image
                            .copy_record(args[0], args[1], args[2] as usize)
                            .unwrap();
                    }
                    "FUN_140030210" => {
                        assert_eq!(args[4], 10);
                        let digits = (args[3] as u32).to_string();
                        for (i, byte) in digits.bytes().enumerate() {
                            image.write_u8(args[1] + i as u64, byte).unwrap();
                        }
                        image
                            .write_u64(args[0], args[1] + digits.len() as u64)
                            .unwrap();
                        image.write_u32(args[0] + 8, 0).unwrap();
                    }
                    _ => panic!("unexpected call {symbol}"),
                }
                0
            },
        )
        .unwrap();
        assert_eq!(result, Some(BASE + 0x100));
        assert_eq!(
            memory.read_u64(BASE + 0x100).unwrap(),
            BASE + 0x200 + expected.len() as u64
        );
        assert_eq!(memory.read_u32(BASE + 0x108).unwrap(), 0);
        assert_eq!(
            &memory.bytes[0x200..0x200 + expected.len()],
            expected.as_bytes()
        );
        assert_eq!(memory.bytes[0x200 + expected.len()], 0xaa);
    }
}

#[test]
fn float_formatter_preserves_bits_before_widening_and_classifies_specials() {
    for (bits, expected) in [
        (1.5f32.to_bits(), "finite"),
        ((-1.5f32).to_bits(), "-finite"),
        (0x7f80_0000, "inf"),
        (0xff80_0000, "-inf"),
        (0x7fc0_0000, "nan"),
        (0xffc0_0000, "-nan(ind)"),
        (0x7f80_0001, "nan(snan)"),
    ] {
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0xaa; 4096],
        };
        memory.write_u32(BASE + 0x400, bits).unwrap();
        let literals = [
            ("\"inf\"", 0x500, b"inf\0".as_slice()),
            ("\"nan\"", 0x520, b"nan\0".as_slice()),
            ("\"nan(ind)\"", 0x540, b"nan(ind)\0".as_slice()),
            ("\"nan(snan)\"", 0x560, b"nan(snan)\0".as_slice()),
        ];
        for (_, at, bytes) in literals {
            for (i, byte) in bytes.iter().enumerate() {
                memory.write_u8(BASE + at + i as u64, *byte).unwrap();
            }
        }
        let mut delegated = false;
        let result = adapters26::fun_140030050(
            &[BASE + 0x100, BASE + 0x200, BASE + 0x280, 0, 1, 6],
            &mut memory,
            &mut |symbol| {
                if symbol == "REGISTER_in_XMM3" {
                    BASE + 0x400
                } else {
                    BASE + literals
                        .iter()
                        .find(|(s, _, _)| *s == symbol)
                        .unwrap_or_else(|| panic!("unexpected symbol {symbol}"))
                        .1
                }
            },
            &mut |symbol, args, image| {
                match symbol {
                    "FUN_14002f400" => {
                        delegated = true;
                        assert_eq!(args[3], 1.5f64.to_bits());
                        assert_eq!(args[4], 6);
                        assert_eq!(args[1], BASE + 0x200 + u64::from(bits >> 31));
                        let digits = b"finite";
                        for (i, byte) in digits.iter().enumerate() {
                            image.write_u8(args[1] + i as u64, *byte).unwrap();
                        }
                        image.write_u64(args[0], args[1] + 6).unwrap();
                        image.write_u32(args[0] + 8, 0).unwrap();
                    }
                    "memcpy" => {
                        image
                            .copy_record(args[0], args[1], args[2] as usize)
                            .unwrap();
                    }
                    _ => panic!("unexpected call {symbol}"),
                }
                0
            },
        )
        .unwrap();
        assert_eq!(result, Some(BASE + 0x100));
        assert_eq!(delegated, expected.ends_with("finite"));
        assert_eq!(
            &memory.bytes[0x200..0x200 + expected.len()],
            expected.as_bytes()
        );
        assert_eq!(
            memory.read_u64(BASE + 0x100).unwrap(),
            BASE + 0x200 + expected.len() as u64
        );
        assert_eq!(memory.read_u32(BASE + 0x108).unwrap(), 0);
    }
}
