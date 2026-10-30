use mspaint_port::recovered::{adapters16, call_support::MemoryImage};
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
const INPUTS: &str = include_str!("fixtures/batch-016-inputs.csv");
const CASES: &str = include_str!("fixtures/batch-016.csv");

fn frame_address(function: &str, symbol: &str, anchor: u64) -> u64 {
    let offsets = adapters16::FRAME_OFFSETS
        .iter()
        .find(|x| x.0 == function)
        .unwrap()
        .1;
    anchor
        .checked_add_signed(i64::from(offsets.iter().find(|x| x.0 == symbol).unwrap().1))
        .unwrap()
}

#[test]
fn allocation_cleanup_divides_signed_lengths_before_selecting_header() {
    for length in [-24i64, -23, -1, 0, 23, 4103, 4104] {
        for gap in [24u64, 40] {
            let mut memory = image(0, BASE, "", "");
            let allocation = BASE + 4096;
            memory.write_u64(BASE, allocation).unwrap();
            memory.write_u64(BASE + 8, 0x1234).unwrap();
            memory
                .write_u64(BASE + 16, allocation.checked_add_signed(length).unwrap())
                .unwrap();
            memory.write_u64(allocation - 8, allocation - gap).unwrap();
            let header = ((length / 24) * 24) as u64 > 4095;
            let invalid = header && gap == 40;
            let mut calls = Vec::new();
            let result = adapters16::fun_140078500(
                &[BASE],
                &mut memory,
                &mut |_| panic!("unexpected relocation"),
                &mut |name, args, memory| {
                    assert_eq!(memory.read_u64(BASE).unwrap(), allocation);
                    calls.push((name, args.to_vec()));
                    0
                },
            );
            if invalid {
                assert_eq!(
                    result,
                    Err(adapters16::AdapterError::Halted("_invoke_watson"))
                );
                assert_eq!(calls, [("_invoke_watson", vec![0; 5])]);
                assert_eq!(memory.read_u64(BASE).unwrap(), allocation);
            } else {
                assert_eq!(result, Ok(None));
                assert_eq!(
                    calls,
                    [(
                        "FUN_1400b022c",
                        vec![if header { allocation - gap } else { allocation }]
                    )]
                );
                assert_eq!(&memory.bytes[..24], &[0; 24]);
            }
        }
    }
}

#[test]
fn zero_page_size_reports_arithmetic_error_after_prior_callback_effects() {
    for page_size in [0, 16] {
        let mut memory = image(0, BASE, "", "");
        let frame = frame_address("NAMED_L150256", "STACK_auStack_88", BASE + 20000);
        let mut calls = Vec::new();
        let result = adapters16::named_l150256(
            &[BASE + 1024, 64],
            &mut memory,
            &mut |name| frame_address("NAMED_L150256", name, BASE + 20000),
            &mut |name, args, memory| {
                calls.push(name);
                match name {
                    "VirtualQuery" => {
                        memory.write_u8(frame + 0x44, 0x44).unwrap();
                        1
                    }
                    "GetSystemInfo" => {
                        assert_eq!(args, [frame + 0x50]);
                        memory.write_u32(args[0] + 4, page_size).unwrap();
                        memory.bytes[512] = 0xa5;
                        0
                    }
                    _ => panic!("unexpected call"),
                }
            },
        );
        assert_eq!(calls, ["VirtualQuery", "GetSystemInfo"]);
        assert_eq!(memory.bytes[512], 0xa5);
        assert_eq!(memory.read_u64(frame - 8).unwrap(), 0x1400ae5cc);
        assert_eq!(
            result,
            if page_size == 0 {
                Err(adapters16::AdapterError::Arithmetic("division by zero"))
            } else {
                Ok(None)
            }
        );
    }
}

#[test]
fn placeholder_flag_is_one_byte_and_message_ranges_keep_their_boundaries() {
    for message in [
        0x100, 0x101, 0x113, 0x200, 0x201, 0x20e, 0x20f, 0x246, 0x247, 0x248, 0x24c,
    ] {
        let mut memory = image(1, BASE, "", "");
        memory.bytes[0x2a6] = 0;
        let msg = BASE + 1024;
        memory.write_u32(msg + 8, message).unwrap();
        let result = adapters16::fun_14006e750(
            &[BASE, msg],
            &mut memory,
            &mut |_| 0,
            &mut |name, args, _| {
                assert_eq!(name, "CWinThread::IsIdleMessage");
                assert_eq!(args, [BASE, msg]);
                0x8000_0001
            },
        )
        .unwrap();
        let expected =
            matches!(message, 0x100 | 0x101 | 0x113 | 0x201..=0x20e | 0x246..=0x247 | 0x24c);
        assert_eq!(memory.bytes[0x2a6], u8::from(expected));
        assert_eq!((memory.bytes[0x2a5], memory.bytes[0x2a7]), (255, 255));
        assert_eq!(result, Some(0xffff_ffff_8000_0001));
    }
    let mut memory = image(0, BASE, "", "");
    assert_eq!(
        adapters16::fun_14006e750(
            &[BASE, u64::MAX],
            &mut memory,
            &mut |_| 0,
            &mut |_, _, _| 0x1_0000_0000
        )
        .unwrap(),
        Some(0)
    );
}

#[test]
fn counter_alias_wraps_and_observes_callback_updates_before_decrement() {
    for count in [0, u64::MAX] {
        for cached in [false, true] {
            let mut memory = image(0, BASE, "", "");
            let counter = BASE + 512;
            let cache = BASE + 520;
            let cookie = BASE + 528;
            let object = BASE + 1024;
            let table = BASE + 2048;
            memory.write_u64(counter, count).unwrap();
            memory
                .write_u64(cache, if cached { object } else { 0 })
                .unwrap();
            memory.write_u64(object, table).unwrap();
            memory.write_u64(table + 48, 0xbeef).unwrap();
            memory.write_u64(BASE + 128, 0x1234).unwrap();
            let mut calls = 0;
            let result = adapters16::fun_1400327f0(
                &[BASE, BASE + 128],
                &mut memory,
                &mut |name| match name {
                    "_DAT_1401cfcd8" | "DAT_1401cfcd8" => counter,
                    "DAT_1401cfcd0" => cache,
                    "DAT_140179040" => cookie,
                    name if name.starts_with("STACK_") => {
                        frame_address("FUN_1400327f0", name, BASE + 20000)
                    }
                    _ => 0,
                },
                &mut |name, args, memory| {
                    calls += 1;
                    if cached {
                        assert_eq!(name, "INDIRECT_CALL");
                        assert_eq!(&args[..3], &[0xbeef, object, 0x1234]);
                        assert_eq!(memory.read_u64(counter).unwrap(), count.wrapping_add(1));
                        memory.write_u64(counter, 100).unwrap();
                        memory.write_u64(args[3], 0xcafe).unwrap();
                    } else {
                        assert_eq!(name, "FUN_1400325e0");
                        assert_eq!(&args[..2], &[0, BASE]);
                        assert_eq!(memory.read_u64(counter).unwrap(), count);
                    }
                    0
                },
            )
            .unwrap();
            assert_eq!(result, Some(BASE));
            assert_eq!(calls, 1);
            assert_eq!(
                memory.read_u64(counter).unwrap(),
                if cached { 99 } else { count }
            );
            if cached {
                assert_eq!(memory.read_u64(BASE).unwrap(), 0xcafe);
            }
        }
    }
}

#[test]
fn concatenated_56_bit_words_preserve_high_address_bits_and_low_flag() {
    for base in [BASE, 0x8000_1234_5678_0000] {
        let mut memory = image(0, base, "", "");
        let record = base + 128;
        let first = base + 256;
        let object = base + 512;
        let table = base + 1024;
        memory.write_u64(record + 48, first).unwrap();
        memory.write_u64(first + 40, object).unwrap();
        memory.write_u64(object, table).unwrap();
        memory.write_u64(table + 8, 0xbeef).unwrap();
        let mut calls = 0;
        assert_eq!(
            adapters16::fun_1400cf090(
                &[0, record],
                &mut memory,
                &mut |_| 0,
                &mut |name, args, _| {
                    assert_eq!(name, "INDIRECT_CALL");
                    assert_eq!(
                        args,
                        [
                            0xbeef,
                            object,
                            ((object + 16) & !255) | 1,
                            0,
                            0,
                            object + 16
                        ]
                    );
                    calls += 1;
                    0
                }
            )
            .unwrap(),
            Some(0)
        );
        assert_eq!(calls, 1);
    }
}

#[test]
fn nonempty_cleanup_loop_releases_before_clear_and_skips_shared_free() {
    let mut memory = image(0, BASE, "", "");
    let first = BASE + 512;
    let second = BASE + 1024;
    memory.write_u64(BASE, first).unwrap();
    memory.write_u64(BASE + 16, second).unwrap();
    memory.write_u32(first + 24, 1).unwrap();
    memory.write_u32(second + 24, 2).unwrap();
    let mut calls = Vec::new();
    assert_eq!(
        adapters16::fun_140023540(
            &[BASE, BASE + 32],
            &mut memory,
            &mut |_| 0,
            &mut |name, args, memory| {
                calls.push((name, args.to_vec()));
                assert_eq!(memory.read_u64(BASE).unwrap(), first);
                assert_eq!(memory.read_u32(first + 24).unwrap(), 0);
                if name == "GetProcessHeap" {
                    0x77
                } else {
                    1
                }
            }
        )
        .unwrap(),
        None
    );
    assert_eq!(
        calls,
        [
            ("GetProcessHeap", vec![]),
            ("HeapFree", vec![0x77, 0, first])
        ]
    );
    assert_eq!(memory.read_u64(BASE).unwrap(), 0);
    assert_eq!(memory.read_u64(BASE + 16).unwrap(), 0);
    assert_eq!(memory.read_u32(second + 24).unwrap(), 1);
    assert_eq!(
        adapters16::fun_140023540(
            &[u64::MAX, u64::MAX],
            &mut memory,
            &mut |_| 0,
            &mut |_, _, _| panic!("unexpected call")
        )
        .unwrap(),
        None
    );
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
        let (_, arity, function) = adapters16::HANDLERS.iter().find(|x| x.0 == name).unwrap();
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
                    returned_pointers
                        .get(symbol)
                        .map_or(PAYLOADS[seed] ^ (index as u64 * 0x101), |&at| {
                            base + at as u64
                        })
                },
            );
            let result = if f[2] == "!" {
                assert!(
                    matches!(result, Err(adapters16::AdapterError::Halted(_))),
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
    assert_eq!(adapters16::HANDLERS.len(), 441);
}

#[test]
fn manifest_reaches_ninety_percent_without_reusing_addresses() {
    use std::collections::BTreeSet;
    let covered: BTreeSet<_> = CASES
        .lines()
        .map(|s| s.split(',').next().unwrap())
        .collect();
    assert_eq!(covered.len(), 441);
    let manifest = include_str!("../translations/batch-016.json");
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
    assert_eq!(total, 3969);
    assert!(total * 100 >= 4410 * 90);
}

#[test]
fn missing_arguments_cause_no_memory_or_runtime_effects() {
    use mspaint_port::recovered::{adapters16::AdapterError, call_support::MissingArguments};
    for &(name, arity, function) in adapters16::HANDLERS {
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
