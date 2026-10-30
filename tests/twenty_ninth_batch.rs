use mspaint_port::recovered::{call_support::MemoryImage, search29};
const BASE: u64 = 0x1000000;
fn image() -> MemoryImage {
    let mut m = MemoryImage {
        base: BASE,
        bytes: vec![0x13; 512],
    };
    m.bytes[0..32].fill(255);
    m.bytes[32..64].fill(0);
    m
}
fn lookup(s: &str) -> u64 {
    match s {
        "DAT_14017909c" => BASE + 64,
        "DAT_1400e7ed0" => BASE + 32,
        _ => panic!("unexpected symbol {s}"),
    }
}
#[test]
fn six_search_bodies_match_14550_normalized_native_c_cases_and_scalar_search() {
    let rows = include_str!("fixtures/search29.csv");
    assert_eq!(rows.lines().count(), 14550);
    for line in rows.lines() {
        let x: Vec<u64> = line.split(',').map(|s| s.parse().unwrap()).collect();
        let (function, flags, n, pattern, expected) =
            (x[0] as usize, x[1] as u32, x[2] as usize, x[3], x[4]);
        let width = if function % 3 == 1 { 1 } else { 2 };
        let reverse = function % 3 == 0;
        let needle = if pattern == 4 { 0x13 } else { 0xbe };
        let mut memory = image();
        memory.write_u32(BASE + 64, flags).unwrap();
        let mut values = Vec::new();
        for i in 0..n {
            let mut value = (i * 37 + 11) % 101;
            if pattern == 1 && i == n / 2
                || pattern == 2 && i % 7 == 0
                || pattern == 3 && (i == 0 || i == n - 1)
            {
                value = 0xbe;
            }
            if pattern == 4 {
                value = 0x13;
            }
            values.push(value);
            memory
                .write_word(BASE + 128 + (i * width) as u64, width, value as u64)
                .unwrap();
        }
        let scalar = if reverse {
            values.iter().rposition(|&v| v == needle)
        } else {
            values.iter().position(|&v| v == needle)
        }
        .map_or(n * width, |i| i * width) as u64;
        assert_eq!(expected, scalar, "native C versus scalar: {line}");
        let before = memory.clone();
        let result = search29::HANDLERS[function].2(
            &[BASE + 128, BASE + 128 + (n * width) as u64, needle as u64],
            &mut memory,
            &mut lookup,
            &mut |s, _, _| panic!("unexpected runtime call {s}"),
        )
        .unwrap();
        assert_eq!(result, Some(BASE + 128 + expected), "{line}");
        assert_eq!(memory, before);
    }
}
#[test]
fn masked_tails_do_not_read_disabled_dwords_past_the_image() {
    for function in 0..3 {
        let width = if function == 1 { 1 } else { 2 };
        let reverse = function == 0;
        let mut memory = MemoryImage {
            base: BASE,
            bytes: vec![0x13; 100],
        };
        memory.bytes[..32].fill(255);
        memory.bytes[32..64].fill(0);
        memory.write_u32(BASE + 60, 0x20).unwrap();
        let location = if reverse { 64 } else { 100 - width };
        memory
            .write_word(BASE + location as u64, width, 0xbe)
            .unwrap();
        let result = search29::HANDLERS[function].2(
            &[BASE + 64, BASE + 100, 0xbe],
            &mut memory,
            &mut |s| {
                if s == "DAT_14017909c" {
                    BASE + 60
                } else {
                    assert_eq!(s, "DAT_1400e7ed0");
                    BASE + 32
                }
            },
            &mut |_, _, _| panic!(),
        )
        .unwrap();
        assert_eq!(result, Some(BASE + location as u64));
    }
}
#[test]
fn vector_branches_read_the_full_block_before_selecting_a_match() {
    let mut memory = image();
    memory.bytes.truncate(136);
    memory.write_u32(BASE + 64, 4).unwrap();
    memory.bytes[128] = 0xbe;
    let error = search29::fun_1400b1150(
        &[BASE + 128, BASE + 144, 0xbe],
        &mut memory,
        &mut lookup,
        &mut |_, _, _| panic!(),
    )
    .unwrap_err();
    assert!(matches!(error, search29::SearchError::Memory(_)));
    memory.write_u32(BASE + 64, 0).unwrap();
    assert_eq!(
        search29::fun_1400b1150(
            &[BASE + 128, BASE + 144, 0xbe],
            &mut memory,
            &mut lookup,
            &mut |_, _, _| panic!()
        )
        .unwrap(),
        Some(BASE + 128)
    );
}
#[test]
fn missing_arguments_and_empty_ranges_have_no_dependency_effects() {
    for (_, _, function) in search29::HANDLERS {
        let mut memory = image();
        for count in 0..3 {
            assert!(matches!(
                function(
                    &[0, 0, 0][..count],
                    &mut memory,
                    &mut |_| panic!(),
                    &mut |_, _, _| panic!()
                ),
                Err(search29::SearchError::Arguments(_))
            ));
        }
        assert_eq!(
            function(
                &[BASE + 128, BASE + 128, 0],
                &mut memory,
                &mut |_| panic!(),
                &mut |_, _, _| panic!()
            )
            .unwrap(),
            Some(BASE + 128)
        );
    }
}
