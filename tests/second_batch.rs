use mspaint_port::recovered::{
    addresses, atomics, captures2, cleanup, exceptions, runtime, streams,
};
use std::{
    cell::Cell,
    collections::BTreeSet,
    sync::atomic::{AtomicI32, AtomicI64, AtomicU32, Ordering},
};

fn hex(value: &str) -> u64 {
    u64::from_str_radix(value.trim_start_matches("0x"), 16).unwrap()
}
fn bytes(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
fn number(value: &str) -> i32 {
    value.parse().unwrap()
}
fn index(value: &str) -> Option<usize> {
    let value = number(value);
    if value < 0 {
        None
    } else {
        Some(value as usize)
    }
}
thread_local! {static DESTROYED_OFFSET: Cell<Option<usize>> = const {Cell::new(None)};}
fn destroy_object(object: &mut [u8]) {
    DESTROYED_OFFSET.set(Some(384 - object.len()));
    object[0] ^= 0x5a;
}

#[test]
fn all_100_new_functions_match_original_c() {
    let mut covered = BTreeSet::new();
    let mut cases = 0;
    for line in include_str!("fixtures/batch-002.csv").lines() {
        let f: Vec<&str> = line.split(',').collect();
        let name = f[1];
        covered.insert(name);
        match f[0] {
            "C" => {
                let seed = f[2].parse::<usize>().unwrap();
                let mut source: Vec<u8> = (0..64).map(|i| (i * 17 + seed * 31) as u8).collect();
                let mut destination = vec![(0xa5 + seed) as u8; 64];
                let (_, _, _, construct) = captures2::CONSTRUCTORS
                    .iter()
                    .find(|x| x.0 == name)
                    .unwrap();
                construct(&mut source, &mut destination, 0x1234_5678_9abc_def0).unwrap();
                assert_eq!(source, bytes(f[3]), "source of {name}, seed {seed}");
                assert_eq!(
                    destination,
                    bytes(f[4]),
                    "destination of {name}, seed {seed}"
                );
            }
            "E" => {
                let seed = f[2].parse::<usize>().unwrap();
                let mut record = vec![(0xa5 + seed) as u8; 40];
                let (_, _, construct) = exceptions::CONSTRUCTORS
                    .iter()
                    .find(|x| x.0 == name)
                    .unwrap();
                construct(&mut record, 0x1234_5678_9abc_def0, 0x1122_3344_5566_7788).unwrap();
                assert_eq!(record, bytes(f[3]), "{name}");
            }
            "A" => {
                let (_, adjust) = addresses::ADJUSTERS.iter().find(|x| x.0 == name).unwrap();
                assert_eq!(adjust(hex(f[2])), hex(f[3]), "{line}");
            }
            "I" => {
                let (_, resolve) = addresses::RESOLVERS.iter().find(|x| x.0 == name).unwrap();
                assert_eq!(
                    resolve(hex(f[2]), [hex(f[3]), hex(f[4])]),
                    hex(f[5]),
                    "{line}"
                );
            }
            "L" => {
                // The original C destructor records its object offset and flips
                // the first byte. Compare both the call and all record bytes.
                let expected = bytes(f[4]);
                let mut record = vec![0xa5; 384];
                let manifest = include_str!("../translations/batch-002.json");
                assert!(manifest.contains(name));
                // The native fixture includes the complete final flags; find
                // their original storage from the source-derived layout table.
                let layout = cleanup::LAYOUTS.iter().find(|x| x.0 == name).unwrap();
                record[layout.1..layout.1 + 4].copy_from_slice(&(hex(f[2]) as u32).to_le_bytes());
                let (_, _, handler) = cleanup::HANDLERS.iter().find(|x| x.0 == name).unwrap();
                DESTROYED_OFFSET.set(None);
                handler(&mut record, destroy_object).unwrap();
                assert_eq!(
                    DESTROYED_OFFSET.get().map(|x| x as i32).unwrap_or(-1),
                    number(f[3]),
                    "{line}"
                );
                assert_eq!(record, expected, "{name}");
            }
            "T" => {
                let initial = hex(f[2]);
                let replacement = hex(f[3]);
                let (result, current) = match name {
                    "FUN_140005d40" => {
                        let word = AtomicI64::new(initial as i64);
                        atomics::fun_140005d40(&word);
                        (0, word.load(Ordering::SeqCst) as u64)
                    }
                    "FUN_14003b160" => {
                        let word = AtomicU32::new(initial as u32);
                        let result = atomics::fun_14003b160(&word, replacement as u32);
                        (u64::from(result), u64::from(word.load(Ordering::SeqCst)))
                    }
                    _ => {
                        let count = AtomicI32::new(initial as i32);
                        let result = match name {
                            "FUN_140004310" => atomics::fun_140004310(&count),
                            "FUN_140004980" => atomics::fun_140004980(&count),
                            "FUN_140004a00" => atomics::fun_140004a00(&count),
                            "FUN_140011240" => atomics::fun_140011240(&count),
                            "FUN_140011770" => atomics::fun_140011770(&count),
                            "FUN_140066460" => atomics::fun_140066460(&count),
                            _ => panic!("unknown atomic {name}"),
                        };
                        (
                            result as u64,
                            u64::from(count.load(Ordering::SeqCst) as u32),
                        )
                    }
                };
                assert_eq!(result, hex(f[4]), "result {line}");
                assert_eq!(current, hex(f[5]), "state {line}");
            }
            "M" => {
                let messages = [None, Some(""), Some("paint"), Some("Unknown exception")];
                assert_eq!(
                    runtime::fun_140002020(messages[f[2].parse::<usize>().unwrap()]).as_bytes(),
                    bytes(f[3])
                );
            }
            "N" => {
                let seed = f[2].parse::<usize>().unwrap();
                let mut input: Vec<u8> = (0..32).map(|i| b'a' + (i % 26) as u8).collect();
                input[seed] = 0;
                let view = runtime::fun_1400052e0(&input).unwrap();
                assert_eq!(view.as_ptr(), input.as_ptr());
                assert_eq!(view.len(), f[3].parse::<usize>().unwrap());
                assert_eq!(view, bytes(f[4]));
            }
            "S" => {
                let value = match name {
                    "FUN_1400186e0" => runtime::fun_1400186e0(),
                    "FUN_140018cb0" => runtime::fun_140018cb0(),
                    "FUN_140018d70" => runtime::fun_140018d70(),
                    _ => panic!("{name}"),
                };
                assert_eq!(value, f[2]);
            }
            "K" => {
                let value = match name {
                    "FUN_140047f50" => runtime::fun_140047f50(),
                    "FUN_14007c270" => runtime::fun_14007c270(),
                    "FUN_14009b370" => u64::from(runtime::fun_14009b370()),
                    "FUN_1400b08a8" => runtime::fun_1400b08a8(),
                    _ => panic!("{name}"),
                };
                assert_eq!(value, hex(f[2]));
            }
            "F" => {
                let count = f[2].parse::<usize>().unwrap();
                let mut data: [u64; 12] = std::array::from_fn(|i| 0xa5a5_a5a5_a5a5_a5a5 ^ i as u64);
                data[11] = 0x1122_3344_5566_7788 ^ count as u64;
                let value = data[11];
                runtime::fun_140041c00(&mut data[..count], value);
                let actual: Vec<u8> = data.into_iter().flat_map(u64::to_le_bytes).collect();
                assert_eq!(actual, bytes(f[3]));
            }
            "V" => {
                let mut actual = Vec::new();
                runtime::fun_1400b02b4(16, 8, f[2].parse().unwrap(), |address| {
                    actual.push(address)
                });
                let expected: Vec<u64> =
                    f[3].split(';').filter(|x| !x.is_empty()).map(hex).collect();
                assert_eq!(actual, expected);
            }
            "G" => {
                let mut output = [0; 2];
                assert_eq!(runtime::fun_1400613a0(&mut output), hex(f[2]));
                assert_eq!(output, [hex(f[3]), hex(f[4])]);
            }
            "B" => {
                let seed = f[2].parse::<usize>().unwrap();
                let mut record: Vec<u8> = (0..384).map(|i| (i * 17 + seed * 31) as u8).collect();
                match name {
                    "FUN_140094330" => {
                        assert_eq!(runtime::fun_140094330(&record).unwrap(), hex(f[3]))
                    }
                    "FUN_1400cccac" => {
                        assert_eq!(runtime::fun_1400cccac(&mut record), Ok(0));
                        assert_eq!(record, bytes(f[3]));
                    }
                    "FUN_1400cccd5" => {
                        assert_eq!(runtime::fun_1400cccd5(&mut record), Ok(0));
                        assert_eq!(record, bytes(f[3]));
                    }
                    "FUN_1400d2512" => {
                        assert_eq!(runtime::fun_1400d2512(&mut record), Ok(0));
                        assert_eq!(record, bytes(f[3]));
                    }
                    _ => panic!("{name}"),
                }
            }
            "O" => {
                let mut output = 123;
                assert_eq!(runtime::fun_1400cc35c(&mut output), hex(f[2]));
                assert_eq!(u64::from(output), hex(f[3]));
            }
            "H" => assert_eq!(runtime::fun_1400d83cf(&(hex(f[2]) as i32)), f[3] == "1"),
            "J" => {
                let mut target = 0x1234_5678_9abc_def0;
                runtime::fun_140024b70(hex(f[2]) as i32, hex(f[3]) as u8, &mut target);
                assert_eq!(target as u64, hex(f[4]));
            }
            "W" => {
                let seed = number(f[2]);
                let mut buffer: Vec<u16> = (0..16).map(|i| (i * 257 + seed) as u16).collect();
                let mut stream = streams::Utf16Stream {
                    buffer: &mut buffer,
                    get: index(f[3]),
                    put: index(f[4]),
                    available: number(f[5]),
                    high_water: number(f[6]) as usize,
                    flags: number(f[7]) as u8,
                    begin: number(f[9]) as usize,
                };
                let result = if name == "FUN_140014350" {
                    i32::from(streams::fun_140014350(&mut stream).unwrap())
                } else {
                    i32::from(streams::fun_1400143c0(&mut stream, number(f[8]) as i16).unwrap())
                };
                assert_eq!(result, number(f[10]), "{line}");
                assert_eq!(
                    stream.get.map(|x| x as i32).unwrap_or(-1),
                    number(f[11]),
                    "{line}"
                );
                assert_eq!(stream.available, number(f[12]), "{line}");
                assert_eq!(stream.high_water, number(f[13]) as usize, "{line}");
                let data: Vec<u8> = stream.buffer.iter().flat_map(|x| x.to_le_bytes()).collect();
                assert_eq!(data, bytes(f[14]), "{line}");
            }
            _ => panic!("unknown fixture {line}"),
        }
        cases += 1;
    }
    assert_eq!(cases, 1086);
    assert_eq!(covered.len(), 100);
    let manifest = include_str!("../translations/batch-002.json");
    let previous = include_str!("../translations/batch-001.json");
    assert_eq!(manifest.matches("\"original\":").count(), 100);
    for name in covered {
        assert!(manifest.contains(&format!("\"original\": \"{name}\"")));
        assert!(!previous.contains(name));
    }
}

#[test]
fn new_constructors_reject_truncation_without_partial_mutation() {
    for &(name, source_size, dest_size, construct) in captures2::CONSTRUCTORS {
        for available in 0..source_size {
            let mut s = vec![0x5a; available];
            let mut d = vec![0xa5; dest_size];
            assert!(construct(&mut s, &mut d, 7).is_err(), "{name}");
            assert!(s.iter().all(|&x| x == 0x5a));
            assert!(d.iter().all(|&x| x == 0xa5));
        }
        for available in 0..dest_size {
            let mut s = vec![0x5a; source_size];
            let mut d = vec![0xa5; available];
            assert!(construct(&mut s, &mut d, 7).is_err(), "{name}");
            assert!(s.iter().all(|&x| x == 0x5a));
            assert!(d.iter().all(|&x| x == 0xa5));
        }
    }
    for &(name, required, construct) in exceptions::CONSTRUCTORS {
        for available in 0..required {
            let mut d = vec![0xa5; available];
            assert!(construct(&mut d, 7, 8).is_err(), "{name}");
            assert!(d.iter().all(|&x| x == 0xa5));
        }
    }
}

#[test]
fn atomic_increments_remain_linearizable_under_contention() {
    for increment in [
        atomics::fun_140004310,
        atomics::fun_140004980,
        atomics::fun_140004a00,
        atomics::fun_140066460,
    ] {
        let count = AtomicI32::new(0);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                let count = &count;
                scope.spawn(move || {
                    for _ in 0..1000 {
                        increment(count);
                    }
                });
            }
        });
        assert_eq!(count.load(Ordering::SeqCst), 8000);
    }
    for increment in [atomics::fun_140011240, atomics::fun_140011770] {
        let count = AtomicI32::new(i32::MAX - 4000);
        std::thread::scope(|scope| {
            for _ in 0..8 {
                let count = &count;
                scope.spawn(move || {
                    for _ in 0..1000 {
                        increment(count);
                    }
                });
            }
        });
        assert_eq!(count.load(Ordering::SeqCst), i32::MAX);
    }
}

#[test]
fn cleanup_errors_are_atomic_and_panicking_destructors_leave_flag_cleared() {
    fn must_not_run(_: &mut [u8]) {
        panic!("unexpected destructor");
    }
    fn fail(_: &mut [u8]) {
        panic!("destructor panic");
    }
    for &(name, required, handler) in cleanup::HANDLERS {
        for available in 0..required {
            let mut record = vec![0xa5; available];
            assert!(handler(&mut record, must_not_run).is_err(), "{name}");
            assert!(record.iter().all(|&x| x == 0xa5));
        }
        let &(_, flag, mask, _) = cleanup::LAYOUTS.iter().find(|x| x.0 == name).unwrap();
        let mut record = vec![0; 384];
        record[flag..flag + 4].copy_from_slice(&mask.to_le_bytes());
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handler(&mut record, fail)));
        assert!(result.is_err());
        assert_eq!(
            u32::from_le_bytes(record[flag..flag + 4].try_into().unwrap()) & mask,
            0
        );
        assert_eq!(handler(&mut record, must_not_run), Ok(()));
    }
}

#[test]
fn safe_adapters_reject_unbounded_or_invalid_states() {
    assert_eq!(runtime::fun_1400052e0(b"paint"), Err(runtime::MissingNul));
    let mut buffer = [65, 66];
    let mut stream = streams::Utf16Stream {
        buffer: &mut buffer,
        get: Some(3),
        put: None,
        available: 0,
        high_water: 0,
        begin: 0,
        flags: 0,
    };
    assert_eq!(
        streams::fun_140014350(&mut stream),
        Err(streams::InvalidStreamState)
    );
    assert_eq!(
        streams::fun_1400143c0(&mut stream, 65),
        Err(streams::InvalidStreamState)
    );
    assert_eq!(stream.buffer, [65, 66]);
    assert_eq!(stream.get, Some(3));
    stream.get = Some(2);
    stream.available = 1;
    assert_eq!(
        streams::fun_140014350(&mut stream),
        Err(streams::InvalidStreamState)
    );
    let mut record = [0xa5; 99];
    assert!(runtime::fun_1400cccac(&mut record).is_err());
    assert!(runtime::fun_1400cccd5(&mut record).is_err());
    assert_eq!(record, [0xa5; 99]);
    assert!(runtime::fun_140094330(&[0; 71]).is_err());
    let mut record = [0xa5; 295];
    assert!(runtime::fun_1400d2512(&mut record).is_err());
    assert_eq!(record, [0xa5; 295]);
}
