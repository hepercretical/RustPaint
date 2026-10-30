use mspaint_port::recovered::{call_support::MemoryImage, sections34::fun_1400b0538};
fn resolve(symbol: &'static str) -> u64 {
    match symbol {
        "IMAGE_SECTION_HEADER_140000230" => 0x1000,
        "DAT_140000348" => 0x1000 + 7 * 40,
        _ => panic!("unexpected symbol {symbol}"),
    }
}
#[test]
fn section_scan_matches_original_c_boundaries_order_flags_and_wrapping() {
    let mut cases = 0;
    for line in include_str!("fixtures/sections34.csv").lines() {
        let p: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0; 7 * 40],
        };
        for i in 0..7u32 {
            let (mut start, mut size, mut flags) = (
                (i + 1) * 0x1000,
                0x823,
                if i % 2 != 0 { 0x80000000 } else { 0x40000040 },
            );
            match p[0] {
                1 => size = 0,
                2 => {
                    start = 0xfffff000 + i * 0x100;
                    size = 0x2000;
                }
                3 => {
                    start = 0x1000;
                    size = 0x3000;
                    flags = if i == 0 { 0x80000000 } else { 0 };
                }
                4 => {
                    start = i * 73;
                    size = 0x1ff;
                    flags = 0x7fffffff;
                }
                5 => {
                    start = (7 - i) * 0x1000;
                    size = 0x1823;
                    flags = 0xffffffff;
                }
                _ => {}
            }
            let at = 0x1000 + u64::from(i) * 40;
            memory.write_u32(at + 8, size).unwrap();
            memory.write_u32(at + 12, start).unwrap();
            memory.write_u32(at + 36, flags).unwrap();
        }
        assert_eq!(
            fun_1400b0538(p[1], &memory, &mut resolve).unwrap(),
            p[2],
            "{line}"
        );
        cases += 1;
    }
    assert_eq!(cases, 336);
}
#[test]
fn empty_scan_and_short_circuit_reads_preserve_source_accesses() {
    let empty = MemoryImage {
        base: 0,
        bytes: vec![],
    };
    assert_eq!(fun_1400b0538(0, &empty, &mut |_| 42).unwrap(), 0);
    // VirtualAddress may be read even when the size and flags are unavailable.
    let mut memory = MemoryImage {
        base: 0x100c,
        bytes: vec![0; 4],
    };
    memory.write_u32(0x100c, 0x1000).unwrap();
    let mut single = |s| if s == "DAT_140000348" { 0x1028 } else { 0x1000 };
    assert_eq!(fun_1400b0538(0x140000001, &memory, &mut single).unwrap(), 0);
    let fault = fun_1400b0538(0x140001000, &memory, &mut single).unwrap_err();
    assert_eq!(fault.address, 0x1008);
    assert_eq!(fault.width, 4);
}
#[test]
fn result_retains_high_bytes_and_only_reads_flags_for_a_matching_range() {
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0; 16],
    };
    memory.write_u32(0x1008, 0x823).unwrap();
    memory.write_u32(0x100c, 0x1000).unwrap();
    let mut single = |s| if s == "DAT_140000348" { 0x1028 } else { 0x1000 };
    assert_eq!(
        fun_1400b0538(0x140002000, &memory, &mut single).unwrap(),
        0x1800
    );
    let fault = fun_1400b0538(0x140001001, &memory, &mut single).unwrap_err();
    assert_eq!(fault.address, 0x1024);
    assert_eq!(fault.width, 4);
    memory.bytes.resize(40, 0);
    memory.write_u32(0x1024, 0x80000000).unwrap();
    assert_eq!(
        fun_1400b0538(0x140001001, &memory, &mut single).unwrap(),
        0x1800
    );
    memory.write_u32(0x1024, 0x7fffffff).unwrap();
    assert_eq!(
        fun_1400b0538(0x140001001, &memory, &mut single).unwrap(),
        0x1801
    );
}
