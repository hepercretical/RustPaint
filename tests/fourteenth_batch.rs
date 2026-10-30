use mspaint_port::recovered::{adapters14, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-014-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-014.csv");

fn frame_address(function: &str, symbol: &str, anchor: u64) -> u64 {
    let offsets = adapters14::FRAME_OFFSETS
        .iter()
        .find(|x| x.0 == function)
        .unwrap()
        .1;
    anchor
        .checked_add_signed(i64::from(offsets.iter().find(|x| x.0 == symbol).unwrap().1))
        .unwrap()
}

#[test]
fn switch_aliases_narrow_the_selector_and_halt_on_unknown_tags() {
    let mut memory = image(0, BASE, "", "");
    for (tag, expected) in (0x5eu64..=0x64).zip([0, 1, 6, 8, 3, 7, 5]) {
        for input in [tag, tag + 7, tag + (1 << 32)] {
            assert_eq!(
                adapters14::fun_14009bca0(
                    &[input],
                    &mut memory,
                    &mut |_| panic!("unexpected relocation"),
                    &mut |_, _, _| panic!("unexpected call")
                )
                .unwrap(),
                Some(expected)
            );
        }
    }
    let mut calls = Vec::new();
    assert_eq!(
        adapters14::fun_14009bca0(&[0], &mut memory, &mut |_| 0, &mut |name, args, _| {
            calls.push((name, args.to_vec()));
            99
        }),
        Err(adapters14::AdapterError::Halted("FUN_14000a4a0"))
    );
    assert_eq!(calls, [("FUN_14000a4a0", vec![])]);
}

#[test]
fn width_validation_switch_preserves_signed_and_unsigned_boundaries() {
    for (tag, value, failure) in [
        (1, 0x7fff_ffff, None),
        (1, 0xffff_ffff_0000_0001, None),
        (1, 0x8000_0000, Some("Negative width.")),
        (2, 0xffff_ffff_0000_0001, None),
        (2, 0x8000_0000, Some("Number is too big.")),
        (3, u64::MAX, Some("Negative width.")),
        (3, 0x8000_0000, Some("Number is too big.")),
        (4, 0x7fff_ffff, None),
        (4, u64::MAX, Some("Number is too big.")),
    ] {
        let mut memory = image(0, BASE, "", "");
        memory.write_u64(BASE, tag | 0x100).unwrap();
        memory.write_u64(BASE + 8, value).unwrap();
        let original = memory.bytes.clone();
        let mut messages = Vec::new();
        let mut calls = Vec::new();
        let result = adapters14::fun_14002c4f0(
            &[BASE],
            &mut memory,
            &mut |name| {
                messages.push(name);
                0xfeed
            },
            &mut |name, args, _| {
                calls.push((name, args.to_vec()));
                0
            },
        );
        if let Some(message) = failure {
            assert_eq!(
                result,
                Err(adapters14::AdapterError::Halted("FUN_140032060"))
            );
            assert_eq!(messages, [format!("\"{message}\"")]);
            assert_eq!(calls, [("FUN_140032060", vec![0xfeed])]);
        } else {
            assert_eq!(result, Ok(None));
            assert!(calls.is_empty());
            assert!(messages.is_empty());
        }
        assert_eq!(memory.bytes, original);
    }
}

#[test]
fn message_fields_control_shift_notification_and_return_narrowing() {
    for (message, key, flags, notify, early) in [
        (0x100, 0x10, 0, true, false),
        (0x100, 0x10, 1 << 30, false, false),
        (0x101, 0x10, 0, true, false),
        (0x101, 0x11, 0, false, false),
        (0x106, 0x21, 0, false, true),
        (0x106, 0x20, 0, false, false),
    ] {
        let mut memory = image(0, BASE, "", "");
        let object = BASE + 1024;
        let msg = BASE + 2048;
        memory.write_u32(msg + 8, message).unwrap();
        memory.write_u64(msg + 16, key).unwrap();
        memory.write_u64(msg + 24, flags).unwrap();
        let mut calls = Vec::new();
        let result = adapters14::fun_140069750(
            &[object, msg],
            &mut memory,
            &mut |_| 0,
            &mut |name, args, _| {
                calls.push((name, args.to_vec()));
                if name == "CFrameWnd::PreTranslateMessage" {
                    0x8000_0001
                } else {
                    0
                }
            },
        )
        .unwrap();
        let mut expected = vec![("ContentPreTranslateMessage", vec![msg])];
        if notify {
            expected.push(("FUN_1400a3c50", vec![object + 0x1e0]));
        }
        if !early {
            expected.push(("CFrameWnd::PreTranslateMessage", vec![object, msg]));
        }
        assert_eq!(calls, expected);
        assert_eq!(result, Some(if early { 1 } else { 0xffff_ffff_8000_0001 }));
    }
    let mut memory = image(0, BASE, "", "");
    assert_eq!(
        adapters14::fun_140069750(
            &[0, u64::MAX],
            &mut memory,
            &mut |_| 0,
            &mut |name, _, _| {
                assert_eq!(name, "ContentPreTranslateMessage");
                1
            }
        )
        .unwrap(),
        Some(1)
    );
}

#[test]
fn event_failure_uses_explicit_return_address_context_after_bool_narrowing() {
    for returned in [0, 1, 0x1_0000_0000, u64::MAX] {
        let mut memory = image(0, BASE, "", "");
        memory.write_u64(BASE + 16, BASE + 128).unwrap();
        memory.write_u64(BASE + 128, 0x1234).unwrap();
        let mut relocations = Vec::new();
        let mut calls = Vec::new();
        let result = adapters14::fun_14007a6e0(
            &[BASE],
            &mut memory,
            &mut |name| {
                relocations.push(name);
                0xbeef
            },
            &mut |name, args, memory| {
                calls.push((name, args.to_vec()));
                memory.bytes[512] = 0xa5;
                returned
            },
        );
        let failed = returned as u32 == 0;
        if failed {
            assert_eq!(
                result,
                Err(adapters14::AdapterError::Halted("FUN_140011cf0"))
            );
            assert_eq!(relocations, ["CONTEXT_unaff_retaddr"]);
            assert_eq!(
                calls,
                [
                    ("SetEvent", vec![0x1234]),
                    ("FUN_140011cf0", vec![0xbeef, 0x9c8])
                ]
            );
        } else {
            assert_eq!(result, Ok(Some(0)));
            assert!(relocations.is_empty());
            assert_eq!(calls, [("SetEvent", vec![0x1234])]);
        }
        assert_eq!(memory.bytes[512], 0xa5);
    }
}

#[test]
fn split_return_signature_and_byte_output_keep_prior_effects_on_fault() {
    for output in [BASE + 512, u64::MAX] {
        let mut memory = image(1, BASE, "", "");
        let anchor = BASE + 20000;
        let cookie = BASE + 30000;
        memory.write_u64(cookie, 0x1234).unwrap();
        let mut calls = 0;
        let result = adapters14::fun_1400a5260(
            &[0x77, 0xfeed, 0, output],
            &mut memory,
            &mut |name| {
                if name == "DAT_140179040" {
                    cookie
                } else {
                    frame_address("FUN_1400a5260", name, anchor)
                }
            },
            &mut |name, args, memory| {
                assert_eq!(name, "FUN_14009dc50");
                assert_eq!(args[0], 0x77);
                assert_eq!(memory.read_u64(args[1]).unwrap(), 0xfeed);
                memory.bytes[100] = 0x42;
                calls += 1;
                0x1_0000_00a5
            },
        );
        assert_eq!(calls, 1);
        assert_eq!(memory.bytes[100], 0x42);
        if output == u64::MAX {
            assert!(
                matches!(result, Err(adapters14::AdapterError::Memory(f)) if f.address == u64::MAX && f.width == 1)
            );
            assert_eq!(memory.bytes[512], 255);
        } else {
            assert_eq!(result, Ok(Some(0)));
            assert_eq!(&memory.bytes[512..514], &[0xa5, 255]);
        }
    }
}

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
fn all_451_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 3608);
    assert_eq!(cases.len(), 7216);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters14::HANDLERS.iter().find(|x| x.0 == name).unwrap();
        let globals = bindings(i[5]);
        let returned_pointers = bindings(i[6]);
        let outputs: Vec<Vec<usize>> = i[7]
            .split(';')
            .filter(|x| !x.is_empty())
            .map(|x| x.split(':').map(|n| n.parse().unwrap()).collect())
            .collect();
        for (variant, base) in [BASE, 0x1234_5678_0000].into_iter().enumerate() {
            let f: Vec<_> = expected[variant].split(',').collect();
            assert_eq!((&i[0], &i[1]), (&f[0], &f[1]));
            assert_eq!(f[6], variant.to_string());
            let mut memory = image(seed, base, i[4], i[8]);
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
                    returned_pointers
                        .get(symbol)
                        .map_or(PAYLOADS[seed] ^ (index as u64 * 0x101), |&at| {
                            base + at as u64
                        })
                },
            );
            let result = if f[2] == "!" {
                assert!(
                    matches!(result, Err(adapters14::AdapterError::Halted(_))),
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
    assert_eq!(adapters14::HANDLERS.len(), 451);
}

#[test]
fn manifest_reaches_seventy_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 451);
    let manifest = include_str!("../translations/batch-014.json");
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
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 451);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 451;
    assert_eq!(total, 3087);
    assert!(total * 100 >= 4410 * 70);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters14::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters14::HANDLERS {
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
