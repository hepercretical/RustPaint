use mspaint_port::recovered::{adapters15, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-015-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-015.csv");

fn frame_address(function: &str, symbol: &str, anchor: u64) -> u64 {
    let offsets = adapters15::FRAME_OFFSETS
        .iter()
        .find(|x| x.0 == function)
        .unwrap()
        .1;
    anchor
        .checked_add_signed(i64::from(offsets.iter().find(|x| x.0 == symbol).unwrap().1))
        .unwrap()
}

#[test]
fn release_clears_owner_before_dispatch_and_keeps_clear_on_load_fault() {
    for table in [BASE + 256, u64::MAX] {
        let mut memory = image(0, BASE, "", "");
        memory.write_u64(BASE, BASE + 128).unwrap();
        memory.write_u64(BASE + 128, table).unwrap();
        memory.write_u64(BASE + 272, 0x1234).unwrap();
        let mut calls = 0;
        let result = adapters15::fun_140001b30(
            &[BASE],
            &mut memory,
            &mut |_| panic!("unexpected relocation"),
            &mut |name, args, memory| {
                assert_eq!(name, "INDIRECT_CALL");
                assert_eq!(args, [0x1234, BASE + 128]);
                assert_eq!(memory.read_u64(BASE).unwrap(), 0);
                memory.bytes[512] = 0xa5;
                calls += 1;
                0
            },
        );
        assert_eq!(memory.read_u64(BASE).unwrap(), 0);
        if table == u64::MAX {
            assert!(matches!(result, Err(adapters15::AdapterError::Memory(_))));
            assert_eq!(calls, 0);
        } else {
            assert_eq!(result, Ok(None));
            assert_eq!(calls, 1);
            assert_eq!(memory.bytes[512], 0xa5);
        }
    }
}

#[test]
fn dwm_callback_output_and_fallback_share_the_recovered_frame() {
    for handled in [0, 1, 0x1_0000_0000, u64::MAX] {
        let mut memory = image(0, BASE, "", "");
        memory.write_u64(BASE + 64, 0x777).unwrap();
        let cookie = BASE + 30000;
        memory.write_u64(cookie, 0xabcd).unwrap();
        let mut calls = Vec::new();
        let result = adapters15::fun_1400694a0(
            &[BASE, 0x1_0000_0100, 0x10, u64::MAX],
            &mut memory,
            &mut |name| {
                if name == "DAT_140179040" {
                    cookie
                } else {
                    frame_address("FUN_1400694a0", name, BASE + 20000)
                }
            },
            &mut |name, args, memory| {
                calls.push(name);
                match name {
                    "DwmDefWindowProc" => {
                        assert_eq!(&args[..4], &[0x777, 0x100, 0x10, u64::MAX]);
                        memory.write_u64(args[4], 0x1234_5678_9abc_def0).unwrap();
                        handled
                    }
                    "CWnd::WindowProc" => {
                        assert_eq!(args, [BASE, 0x100, 0x10, u64::MAX]);
                        0xfedc_ba98_7654_3210
                    }
                    _ => panic!("unexpected callback"),
                }
            },
        )
        .unwrap();
        if handled as u32 == 0 {
            assert_eq!(calls, ["DwmDefWindowProc", "CWnd::WindowProc"]);
            assert_eq!(result, Some(0xfedc_ba98_7654_3210));
        } else {
            assert_eq!(calls, ["DwmDefWindowProc"]);
            assert_eq!(result, Some(0x1234_5678_9abc_def0));
        }
    }
}

#[test]
fn duplicate_named_destructors_keep_their_individual_offsets_and_sizes() {
    for (function, offset, size) in [
        (adapters15::named_l10100 as adapters15::Handler, 8, 0x18),
        (adapters15::named_l10117 as adapters15::Handler, 0x10, 0x28),
    ] {
        for flags in [0, 1, 2, 0x1_0000_0001] {
            let mut memory = image(0, BASE, "", "");
            let mut calls = Vec::new();
            assert_eq!(
                function(
                    &[BASE, flags],
                    &mut memory,
                    &mut |_| 0,
                    &mut |name, args, _| {
                        calls.push((name, args.to_vec()));
                        0
                    }
                )
                .unwrap(),
                Some(BASE)
            );
            let mut expected = vec![("FUN_140006890", vec![BASE + offset])];
            if flags & 1 != 0 {
                expected.push(("FUN_1400b022c", vec![BASE, size]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[test]
fn named_entry_and_crt_paths_preserve_call_order_and_distinct_arities() {
    let mut memory = image(0, BASE, "", "");
    let mut calls = Vec::new();
    assert_eq!(
        adapters15::named_l154672(&[], &mut memory, &mut |_| 0, &mut |name, args, memory| {
            calls.push((name, args.to_vec()));
            memory.bytes[0] += 1;
            0
        })
        .unwrap(),
        None
    );
    assert_eq!(
        calls,
        [("FUN_1400b07f8", vec![]), ("FUN_1400b003c", vec![])]
    );
    assert_eq!(memory.bytes[0], 2);
    for (global, flag, expected_calls) in [(0, 1, 2), (1, 0, 2), (1, 1, 0), (1, 0x100, 2)] {
        memory.bytes[0] = global;
        calls.clear();
        assert_eq!(
            adapters15::named_l155000(
                &[0x1a5, flag],
                &mut memory,
                &mut |name| {
                    assert_eq!(name, "DAT_1401cf480");
                    BASE
                },
                &mut |name, args, _| {
                    calls.push((name, args.to_vec()));
                    99
                }
            )
            .unwrap(),
            Some(1)
        );
        assert_eq!(calls.len(), expected_calls);
        if expected_calls != 0 {
            assert_eq!(
                calls,
                [("FUN_14009b370", vec![]), ("FUN_14009b370", vec![0xa5])]
            );
        }
    }
}

#[test]
fn named_import_uses_the_runtime_callback_and_guard_body_has_no_effects() {
    let mut memory = image(0, BASE, "", "");
    let mut calls = 0;
    assert_eq!(
        adapters15::named_l150054(
            &[BASE + 128],
            &mut memory,
            &mut |_| 0,
            &mut |name, args, memory| {
                assert_eq!(name, "CoTaskMemFree");
                assert_eq!(args, [BASE + 128]);
                memory.bytes[128] = 0xa5;
                calls += 1;
                u64::MAX
            }
        )
        .unwrap(),
        None
    );
    assert_eq!(calls, 1);
    let before = memory.clone();
    assert_eq!(
        adapters15::named_l5649(
            &[],
            &mut memory,
            &mut |_| panic!("unexpected relocation"),
            &mut |_, _, _| panic!("unexpected call")
        )
        .unwrap(),
        None
    );
    assert_eq!(memory, before);
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
fn all_441_functions_match_native_c_at_two_memory_bases() {
    let inputs: Vec<_> = INPUTS.lines().collect();
    let cases: Vec<_> = CASES.lines().collect();
    assert_eq!(inputs.len(), 3528);
    assert_eq!(cases.len(), 7056);
    let (pairs, tail) = cases.as_chunks::<2>();
    assert!(tail.is_empty());
    for (input, expected) in inputs.into_iter().zip(pairs) {
        let i: Vec<_> = input.split(',').collect();
        let name = i[0];
        let seed: usize = i[1].parse().unwrap();
        let (_, arity, function) = adapters15::HANDLERS.iter().find(|x| x.0 == name).unwrap();
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
                    matches!(result, Err(adapters15::AdapterError::Halted(_))),
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
    assert_eq!(adapters15::HANDLERS.len(), 441);
}

#[test]
fn manifest_reaches_eighty_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 441);
    let manifest = include_str!("../translations/batch-015.json");
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
    ];
    assert_eq!(manifest.matches("\"original\":").count(), 441);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(prior.iter().all(|s| !s.contains(name)));
    }
    let total = 15
        + prior
            .iter()
            .map(|s| s.matches("\"original\":").count())
            .sum::<usize>()
        + 441;
    assert_eq!(total, 3528);
    assert!(total * 100 >= 4410 * 80);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters15::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters15::HANDLERS {
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
