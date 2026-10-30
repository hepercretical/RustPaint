use mspaint_port::declaration_support::{Address, Enum32, TextFault, TextGlobal, Unknown, Wire};
use mspaint_port::declarations40::{self as d, t_7e18d64b2a05 as Hierarchy};
use mspaint_port::recovered::call_support::MemoryImage;

#[test]
fn hierarchy_uses_four_byte_nested_rvas_instead_of_native_pointers() {
    let base = 0x140000000;
    let mut m = MemoryImage {
        base,
        bytes: vec![0xaa; 80],
    };
    Hierarchy::SIGNATURE.store(&mut m, base, &0).unwrap();
    Hierarchy::ATTRIBUTES.store(&mut m, base, &3).unwrap();
    Hierarchy::NUM_BASE_CLASSES.store(&mut m, base, &1).unwrap();
    Hierarchy::BASE_CLASS_ARRAY
        .store(&mut m, base, &d::RttiBaseArrayRva::new(24))
        .unwrap();
    d::RttiBaseRva::new(40).store(&mut m, base + 24).unwrap();
    assert_eq!(
        &m.bytes[..16],
        &[0, 0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 24, 0, 0, 0]
    );
    assert_eq!(m.bytes[16], 0xaa);
    let array = Hierarchy::BASE_CLASS_ARRAY
        .load(&m, base)
        .unwrap()
        .resolve(base)
        .unwrap();
    let descriptor = array.load(&m).unwrap().resolve(base).unwrap();
    assert_eq!(descriptor.value, base + 40);
    assert_eq!(descriptor.load(&m).unwrap().bytes.len(), 28);
    assert_eq!(Hierarchy::SIZE, 16);
    assert_eq!(Hierarchy::ALIGN, 4);
    assert_eq!(d::RttiBaseRva::new(0).resolve(base).unwrap().value, base);
    assert!(d::RttiBaseRva::new(1).resolve(u64::MAX).is_none());
    assert!(Hierarchy::BASE_CLASS_ARRAY.load(&m, base + 68).is_err());
}

#[test]
fn text_declarations_resolve_symbols_and_preserve_raw_units() {
    let mut m = MemoryImage {
        base: 100,
        bytes: vec![0; 24],
    };
    m.bytes[..4].copy_from_slice(&[0xff, b'a', 0, 0x55]);
    let mut resolve = |name| {
        assert_eq!(name, d::G_L3922.symbol);
        100
    };
    assert_eq!(
        d::G_L3922.read(&m, &mut resolve, 3).unwrap(),
        vec![0xff, b'a']
    );
    assert!(matches!(
        d::G_L3922.read(&m, &mut resolve, 2),
        Err(TextFault::Unterminated { limit: 2, .. })
    ));
    m.write_u16(105, 0xd800).unwrap();
    m.write_u16(107, 0xdc01).unwrap();
    m.write_u16(109, 0).unwrap();
    for global in [&d::G_L4411, &d::G_L4471, &d::G_L4653, &d::G_L4684] {
        assert!(global.preferred_address.is_some());
        assert_eq!(
            global.read(&m, &mut |_| 105, 3).unwrap(),
            vec![0xd800, 0xdc01]
        );
        assert!(matches!(
            global.read(&m, &mut |_| 123, 1),
            Err(TextFault::Memory(_))
        ));
    }
    let edge = MemoryImage {
        base: u64::MAX,
        bytes: vec![1],
    };
    assert!(matches!(
        TextGlobal::<u8>::new("edge", None).read(&edge, &mut |_| u64::MAX, 2),
        Err(TextFault::Memory(_))
    ));
    assert!(matches!(
        TextGlobal::<u16>::new("edge", None).read(&edge, &mut |_| u64::MAX, 1),
        Err(TextFault::Memory(_))
    ));
    assert!(matches!(
        d::G_L3922.read(&m, &mut |_| 100, 0),
        Err(TextFault::Unterminated { limit: 0, .. })
    ));
}

#[test]
fn opaque_aliases_and_exception_callback_keep_source_word_widths() {
    fn routine(
        record: Address<mspaint_port::declarations30::t_e7ed2387b2bc>,
        frame: Address<Unknown>,
        context: Address<mspaint_port::declarations30::t_ea23a4932299>,
        dispatcher: Address<Unknown>,
    ) -> Enum32 {
        assert_eq!(
            [record.value, frame.value, context.value, dispatcher.value],
            [1, 2, 3, 4]
        );
        Enum32(3)
    }
    let callback: d::ExceptionRoutine = routine;
    assert_eq!(
        callback(
            Address::new(1),
            Address::new(2),
            Address::new(3),
            Address::new(4)
        )
        .0,
        3
    );
    let mut m = MemoryImage {
        base: 0,
        bytes: vec![0; 24],
    };
    d::alias_l1508::A1::new(0x123456789abcdef0)
        .store(&mut m, 0)
        .unwrap();
    d::alias_l1513::A1::new(0x8877665544332211)
        .store(&mut m, 8)
        .unwrap();
    d::alias_l1825::A1::new(u64::MAX).store(&mut m, 16).unwrap();
    assert_eq!(
        d::alias_l1508::A1::load(&m, 0).unwrap().value,
        0x123456789abcdef0
    );
    assert_eq!(
        d::alias_l1513::A1::load(&m, 8).unwrap().value,
        0x8877665544332211
    );
    assert_eq!(d::alias_l1825::A1::load(&m, 16).unwrap().value, u64::MAX);
}
