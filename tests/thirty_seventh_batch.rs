use mspaint_port::recovered::{call_support::MemoryImage, search29::SearchError, substring37};
#[test]
fn both_substring_bodies_match_hardware_backed_c_and_scalar_searches() {
    let mut cases = 0;
    for line in include_str!("fixtures/substring37.csv").lines() {
        let p: Vec<u64> = line.split(',').map(|x| x.parse().unwrap()).collect();
        let (which, feature, n, len, scenario, expected) =
            (p[0], p[1], p[2] as usize, p[3] as usize, p[4], p[5]);
        let mut a: [u16; 128] = core::array::from_fn(|i| ((i * 37 + 11) % 103) as u16);
        let mut b: [u16; 32] = core::array::from_fn(|i| ((i * 13 + 7) % 103) as u16);
        let pos = match scenario {
            0 => -1,
            1 => 0,
            2 => n as i64 - len as i64,
            3 => 7,
            _ => 15,
        };
        if scenario == 4 {
            a.fill(1);
            b.fill(1);
        } else if pos >= 0 && pos as usize + len <= n {
            a[pos as usize..pos as usize + len].copy_from_slice(&b[..len]);
        }
        let mut memory = MemoryImage {
            base: 0x1000,
            bytes: vec![0; 640],
        };
        memory
            .write_u32(0x1000, if feature != 0 { 4 } else { 0 })
            .unwrap();
        for (i, x) in a.iter().enumerate() {
            memory.write_u16(0x1040 + i as u64 * 2, *x).unwrap();
        }
        for (i, x) in b.iter().enumerate() {
            memory.write_u16(0x1180 + i as u64 * 2, *x).unwrap();
        }
        memory.bytes[0x200..0x220].fill(255);
        let mut resolve = |s| match s {
            "DAT_14017909c" => 0x1000,
            "DAT_1400e7ed0" => 0x1220,
            _ => panic!("unexpected symbol {s}"),
        };
        let args = [0x1040, 0x1040 + n as u64 * 2, 0x1180, len as u64];
        let result = if which == 0 {
            substring37::fun_1400b1350(&args, &mut memory, &mut resolve, &mut |_, _, _| {
                panic!("unexpected dependency")
            })
        } else {
            substring37::named_l156673(&args, &mut memory, &mut resolve, &mut |_, _, _| {
                panic!("unexpected dependency")
            })
        }
        .unwrap()
        .unwrap();
        assert_eq!(result - 0x1040, expected, "{line}");
        let scalar = if len == 0 {
            0
        } else if len > n {
            n
        } else {
            (0..=n - len)
                .find(|i| a[*i..*i + len] == b[..len])
                .unwrap_or(n)
        };
        assert_eq!(result, 0x1040 + scalar as u64 * 2, "{line}");
        cases += 1;
    }
    assert_eq!(cases, 24700);
}
#[test]
fn substring_missing_arguments_have_no_effects() {
    for count in 0..4 {
        let mut memory = MemoryImage {
            base: 0,
            bytes: vec![],
        };
        let result = substring37::fun_1400b1350(
            &vec![0; count],
            &mut memory,
            &mut |_| panic!("lookup"),
            &mut |_, _, _| panic!("call"),
        );
        assert!(matches!(result, Err(SearchError::Arguments(_))));
    }
}
#[test]
fn empty_and_oversized_needles_skip_memory_and_vector_reads_remain_complete() {
    let mut empty = MemoryImage {
        base: 0,
        bytes: vec![],
    };
    assert_eq!(
        substring37::fun_1400b1350(
            &[5, 9, 999, 0],
            &mut empty,
            &mut |_| panic!("lookup"),
            &mut |_, _, _| panic!("call")
        )
        .unwrap(),
        Some(5)
    );
    assert_eq!(
        substring37::fun_1400b1350(
            &[5, 9, 999, 3],
            &mut empty,
            &mut |_| panic!("lookup"),
            &mut |_, _, _| panic!("call")
        )
        .unwrap(),
        Some(9)
    );
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0; 24],
    };
    memory.write_u8(0x1000, 4).unwrap();
    let result = substring37::fun_1400b1350(
        &[0x1010, 0x1020, 0x1004, 2],
        &mut memory,
        &mut |_| 0x1000,
        &mut |_, _, _| panic!("call"),
    );
    assert!(matches!(result, Err(SearchError::Memory(_))));
}
