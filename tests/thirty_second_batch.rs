use mspaint_port::recovered::{call_support::MemoryImage, runtime32::*};

fn symbol_address(symbol: &'static str) -> u64 {
    match symbol {
        "DAT_1401cf481" => 0x1000,
        "DAT_1401cf488" => 0x1010,
        "uRam00000001401cf490" => 0x1018,
        "_DAT_1401cf498" => 0x1020,
        "_DAT_1401cf4a0" | "DAT_1401cf4a0" => 0x1028,
        "uRam00000001401cf4a8" => 0x1030,
        "_DAT_1401cf4b0" => 0x1038,
        _ => panic!("unexpected source symbol {symbol}"),
    }
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|x| format!("{x:02x}")).collect()
}
fn digest(bytes: &[u8]) -> u64 {
    bytes.iter().fold(14695981039346656037, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(1099511628211)
    })
}

#[test]
fn exit_tables_match_original_c_results_writes_and_ordered_calls() {
    let mut cases = 0;
    for line in include_str!("fixtures/runtime32.csv")
        .lines()
        .filter(|l| l.starts_with("O,"))
    {
        let p: Vec<_> = line.split(',').collect();
        let n: Vec<u64> = p[1..9].iter().map(|s| s.parse().unwrap()).collect();
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0xa5; 128],
        };
        memory.write_u8(0x1000, n[0] as u8).unwrap();
        for i in 0..6 {
            memory
                .write_u64(0x1010 + i * 8, 0x1122334455667700 + i)
                .unwrap();
        }
        let mut trace = String::new();
        let mut init_calls = 0u64;
        let actual = named_l154902(
            n[1] as u32,
            &mut memory,
            &mut symbol_address,
            &mut |symbol, args, m| match symbol {
                "__scrt_is_ucrt_dll_in_use" => {
                    assert!(args.is_empty());
                    trace.push_str("1:0;");
                    n[2]
                }
                "_initialize_onexit_table" => {
                    assert_eq!(args.len(), 1);
                    trace.push_str(&format!("2:{};", args[0] - 0x1010));
                    assert_eq!(args[0], 0x1010 + init_calls * 24);
                    for j in 0..3 {
                        m.write_u64(args[0] + j * 8, 0x1234567800000000 + init_calls * 16 + j)
                            .unwrap();
                    }
                    let result = if init_calls == 0 { n[3] } else { n[4] };
                    init_calls += 1;
                    if n[5] != 0 {
                        m.write_u8(0x1000, 0x7f).unwrap();
                    }
                    result
                }
                "FUN_1400b090c" => {
                    assert_eq!(args, &[5]);
                    trace.push_str("3:5;");
                    0
                }
                "swi" => {
                    assert_eq!(args, &[3]);
                    trace.push_str("4:3;");
                    0x123456789abcdef0
                }
                "INDIRECT_CALL" => {
                    assert_eq!(args, &[0x123456789abcdef0]);
                    trace.push_str("5:1311768467463790320;");
                    0xfedcba9876543210
                }
                _ => panic!("unexpected callee {symbol}"),
            },
        )
        .unwrap();
        assert_eq!(actual, n[6], "{line}");
        assert_eq!(memory.read_u8(0x1000).unwrap(), n[7] as u8, "{line}");
        assert_eq!(hex(&memory.bytes[16..64]), p[9], "{line}");
        assert_eq!(trace, p[10], "{line}");
        assert!(memory.bytes[1..16]
            .iter()
            .chain(&memory.bytes[64..])
            .all(|b| *b == 0xa5));
        cases += 1;
    }
    assert_eq!(cases, 1728);
}

#[test]
fn stack_probe_matches_original_c_across_page_boundaries() {
    let mut cases = 0;
    for line in include_str!("fixtures/runtime32.csv")
        .lines()
        .filter(|l| l.starts_with("P,"))
    {
        let n: Vec<u64> = line
            .split(',')
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();
        let mut memory = MemoryImage {
            base: 0x10000,
            bytes: vec![0xa5; 0x9000],
        };
        named_l176343(
            ProbeContext {
                local_res8: memory.base + n[0],
                stack_limit: memory.base + n[1],
                allocation: n[2],
            },
            &mut memory,
        )
        .unwrap();
        assert_eq!(digest(&memory.bytes), n[3], "{line}");
        cases += 1;
    }
    assert_eq!(cases, 120);
}

#[test]
fn stack_probe_underflow_and_memory_faults_preserve_exact_page_writes() {
    let mut memory = MemoryImage {
        base: 0,
        bytes: vec![0xa5; 0x5000],
    };
    named_l176343(
        ProbeContext {
            local_res8: 0x4000,
            allocation: 0x5000,
            stack_limit: 0x5000,
        },
        &mut memory,
    )
    .unwrap();
    for (i, byte) in memory.bytes.iter().enumerate() {
        assert_eq!(*byte, if i % 0x1000 == 0 { 0 } else { 0xa5 });
    }
    let mut memory = MemoryImage {
        base: 0x3000,
        bytes: vec![0xa5; 0x2000],
    };
    let fault = named_l176343(
        ProbeContext {
            local_res8: 0x6000,
            allocation: 0x5000,
            stack_limit: 0x5000,
        },
        &mut memory,
    )
    .unwrap_err();
    assert_eq!(fault.address, 0x2000);
    assert_eq!(fault.width, 1);
    for (i, byte) in memory.bytes.iter().enumerate() {
        assert_eq!(*byte, if i % 0x1000 == 0 { 0 } else { 0xa5 });
    }
    let mut empty = MemoryImage {
        base: 0,
        bytes: vec![],
    };
    named_l176343(
        ProbeContext {
            local_res8: u64::MAX,
            allocation: 0,
            stack_limit: u64::MAX,
        },
        &mut empty,
    )
    .unwrap();
    let fault = named_l176343(
        ProbeContext {
            local_res8: 0,
            allocation: 1,
            stack_limit: 1,
        },
        &mut empty,
    )
    .unwrap_err();
    assert_eq!(fault.address, 1u64.wrapping_sub(0x1000));
}

#[test]
fn global_faults_keep_completed_stores_without_marking_initialization_complete() {
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0; 32],
    };
    let mut calls = 0;
    let fault = named_l154902(
        0,
        &mut memory,
        &mut symbol_address,
        &mut |symbol, args, _| {
            assert_eq!(symbol, "__scrt_is_ucrt_dll_in_use");
            assert!(args.is_empty());
            calls += 1;
            0
        },
    )
    .unwrap_err();
    assert_eq!(fault.address, 0x1020);
    assert_eq!(fault.width, 8);
    assert_eq!(calls, 1);
    assert_eq!(memory.read_u8(0x1000).unwrap(), 0);
    assert_eq!(memory.read_u64(0x1010).unwrap(), u64::MAX);
    assert_eq!(memory.read_u64(0x1018).unwrap(), u64::MAX);
}
