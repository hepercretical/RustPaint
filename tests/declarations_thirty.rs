use mspaint_port::{declaration_support::*, declarations30::*};
#[test]
fn original_c_record_sizes_and_field_offsets_match_rust_views() {
    assert_eq!(t_fd609cf74451::SIZE, 8);
    assert_eq!(t_fd609cf74451::F_X.offset, 0);
    assert_eq!(t_fd609cf74451::F_Y.offset, 4);
    assert_eq!(t_a2484b182201::SIZE, 8);
    assert_eq!(t_a2484b182201::F_CX.offset, 0);
    assert_eq!(t_a2484b182201::F_CY.offset, 4);
    assert_eq!(t_9bbc0df2157e::SIZE, 16);
    assert_eq!(t_9bbc0df2157e::F_DATA1.offset, 0);
    assert_eq!(t_9bbc0df2157e::F_DATA2.offset, 4);
    assert_eq!(t_9bbc0df2157e::F_DATA3.offset, 6);
    assert_eq!(t_9bbc0df2157e::F_DATA4.offset, 8);
    assert_eq!(t_272e88af6e21::SIZE, 16);
    assert_eq!(t_272e88af6e21::F_LEFT.offset, 0);
    assert_eq!(t_272e88af6e21::F_TOP.offset, 4);
    assert_eq!(t_272e88af6e21::F_RIGHT.offset, 8);
    assert_eq!(t_272e88af6e21::F_BOTTOM.offset, 12);
    assert_eq!(t_90b4e80c4b85::SIZE, 40);
    assert_eq!(t_90b4e80c4b85::F_BISIZE.offset, 0);
    assert_eq!(t_90b4e80c4b85::F_BIWIDTH.offset, 4);
    assert_eq!(t_90b4e80c4b85::F_BIHEIGHT.offset, 8);
    assert_eq!(t_90b4e80c4b85::F_BIPLANES.offset, 12);
    assert_eq!(t_90b4e80c4b85::F_BIBITCOUNT.offset, 14);
    assert_eq!(t_90b4e80c4b85::F_BICOMPRESSION.offset, 16);
    assert_eq!(t_90b4e80c4b85::F_BISIZEIMAGE.offset, 20);
    assert_eq!(t_90b4e80c4b85::F_BIXPELSPERMETER.offset, 24);
    assert_eq!(t_90b4e80c4b85::F_BIYPELSPERMETER.offset, 28);
    assert_eq!(t_90b4e80c4b85::F_BICLRUSED.offset, 32);
    assert_eq!(t_90b4e80c4b85::F_BICLRIMPORTANT.offset, 36);
    assert_eq!(t_47fc86ee7976::SIZE, 20);
    assert_eq!(t_47fc86ee7976::F_MACHINE.offset, 0);
    assert_eq!(t_47fc86ee7976::F_NUMBEROFSECTIONS.offset, 2);
    assert_eq!(t_47fc86ee7976::F_TIMEDATESTAMP.offset, 4);
    assert_eq!(t_47fc86ee7976::F_POINTERTOSYMBOLTABLE.offset, 8);
    assert_eq!(t_47fc86ee7976::F_NUMBEROFSYMBOLS.offset, 12);
    assert_eq!(t_47fc86ee7976::F_SIZEOFOPTIONALHEADER.offset, 16);
    assert_eq!(t_47fc86ee7976::F_CHARACTERISTICS.offset, 18);
    assert_eq!(t_4a3399835aa5::SIZE, 128);
    assert_eq!(t_4a3399835aa5::F_E_MAGIC.offset, 0);
    assert_eq!(t_4a3399835aa5::F_E_CBLP.offset, 2);
    assert_eq!(t_4a3399835aa5::F_E_CP.offset, 4);
    assert_eq!(t_4a3399835aa5::F_E_CRLC.offset, 6);
    assert_eq!(t_4a3399835aa5::F_E_CPARHDR.offset, 8);
    assert_eq!(t_4a3399835aa5::F_E_MINALLOC.offset, 10);
    assert_eq!(t_4a3399835aa5::F_E_MAXALLOC.offset, 12);
    assert_eq!(t_4a3399835aa5::F_E_SS.offset, 14);
    assert_eq!(t_4a3399835aa5::F_E_SP.offset, 16);
    assert_eq!(t_4a3399835aa5::F_E_CSUM.offset, 18);
    assert_eq!(t_4a3399835aa5::F_E_IP.offset, 20);
    assert_eq!(t_4a3399835aa5::F_E_CS.offset, 22);
    assert_eq!(t_4a3399835aa5::F_E_LFARLC.offset, 24);
    assert_eq!(t_4a3399835aa5::F_E_OVNO.offset, 26);
    assert_eq!(t_4a3399835aa5::F_E_RES.offset, 28);
    assert_eq!(t_4a3399835aa5::F_E_OEMID.offset, 36);
    assert_eq!(t_4a3399835aa5::F_E_OEMINFO.offset, 38);
    assert_eq!(t_4a3399835aa5::F_E_RES2.offset, 40);
    assert_eq!(t_4a3399835aa5::F_E_LFANEW.offset, 60);
    assert_eq!(t_4a3399835aa5::F_E_PROGRAM.offset, 64);
    assert_eq!(t_048cb0a340e5::SIZE, 4);
    assert_eq!(t_048cb0a340e5::F_PHYSICALADDRESS.offset, 0);
    assert_eq!(t_048cb0a340e5::F_VIRTUALSIZE.offset, 0);
    assert_eq!(t_d6601bc9eca9::SIZE, 40);
    assert_eq!(t_d6601bc9eca9::F_NAME.offset, 0);
    assert_eq!(t_d6601bc9eca9::F_MISC.offset, 8);
    assert_eq!(t_d6601bc9eca9::F_VIRTUALADDRESS.offset, 12);
    assert_eq!(t_d6601bc9eca9::F_SIZEOFRAWDATA.offset, 16);
    assert_eq!(t_d6601bc9eca9::F_POINTERTORAWDATA.offset, 20);
    assert_eq!(t_d6601bc9eca9::F_POINTERTORELOCATIONS.offset, 24);
    assert_eq!(t_d6601bc9eca9::F_POINTERTOLINENUMBERS.offset, 28);
    assert_eq!(t_d6601bc9eca9::F_NUMBEROFRELOCATIONS.offset, 32);
    assert_eq!(t_d6601bc9eca9::F_NUMBEROFLINENUMBERS.offset, 34);
    assert_eq!(t_d6601bc9eca9::F_CHARACTERISTICS.offset, 36);
}

// BEGIN MANUAL CONSUMER TESTS
#[test]
fn signed_record_fields_and_union_views_preserve_exact_bytes() {
    use mspaint_port::recovered::call_support::MemoryImage;
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0xaa; 128],
    };
    t_fd609cf74451::F_X.store(&mut memory, 0x1010, &-7).unwrap();
    t_fd609cf74451::F_Y
        .store(&mut memory, 0x1010, &i32::MAX)
        .unwrap();
    assert_eq!(t_fd609cf74451::F_X.load(&memory, 0x1010).unwrap(), -7);
    let point = t_fd609cf74451::load(&memory, 0x1010).unwrap();
    assert_eq!(&point.bytes[..4], &(-7i32).to_le_bytes());
    assert_eq!(memory.bytes[15], 0xaa);
    assert_eq!(memory.bytes[24], 0xaa);
    t_048cb0a340e5::F_PHYSICALADDRESS
        .store(&mut memory, 0x1020, &0xdeadbeef)
        .unwrap();
    assert_eq!(
        t_048cb0a340e5::F_VIRTUALSIZE.load(&memory, 0x1020).unwrap(),
        0xdeadbeef
    );
    let records = [point.clone(), point];
    records.store(&mut memory, 0x1040).unwrap();
    assert_eq!(
        <[t_fd609cf74451; 2]>::load(&memory, 0x1040).unwrap(),
        records
    );
}
#[test]
fn typed_global_aliases_keep_one_storage_location_without_native_dereference() {
    use mspaint_port::recovered::call_support::MemoryImage;
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0xaa; 64],
    };
    let mut resolver = |symbol| {
        assert_eq!(symbol, "DAT_1401cf430");
        0x1010
    };
    let data = G_L4983.address(&mut resolver);
    let code = G_L4987.address(&mut resolver);
    assert_eq!(G_L4983.preferred_address, G_L4987.preferred_address);
    data.store(&mut memory, &Address::new(0x123456789abcdef0))
        .unwrap();
    assert_eq!(code.load(&memory).unwrap().0, 0x123456789abcdef0);
    code.store(&mut memory, &CodeAddress(0xfedcba9876543210))
        .unwrap();
    assert_eq!(data.load(&memory).unwrap().value, 0xfedcba9876543210);
    t_16e802e5e765::F_UNIQUEPROCESS
        .store(&mut memory, 0x1020, &Address::new(0x9876))
        .unwrap();
    assert_eq!(memory.read_u64(0x1020).unwrap(), 0x9876);
}
#[test]
fn complete_ranges_are_checked_before_record_or_array_writes() {
    use mspaint_port::recovered::call_support::MemoryImage;
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0xaa; 16],
    };
    let before = memory.clone();
    assert!(t_fd609cf74451 { bytes: [0; 8] }
        .store(&mut memory, 0x100c)
        .is_err());
    assert_eq!(memory, before);
    assert!([1u32, 2u32, 3u32].store(&mut memory, 0x1008).is_err());
    assert_eq!(memory, before);
    assert!(Field::<u64>::new(usize::MAX).load(&memory, 0x1000).is_err());
    let overflow = MemoryImage {
        base: u64::MAX - 1,
        bytes: vec![0; 16],
    };
    assert!(load_bytes::<8>(&overflow, u64::MAX - 1).is_err());
}
#[test]
fn relative_addresses_scalar_aliases_and_float_payloads_retain_source_widths() {
    use mspaint_port::recovered::call_support::MemoryImage;
    let mut memory = MemoryImage {
        base: 0x1000,
        bytes: vec![0; 64],
    };
    let relative = Relative32::<Unknown>::new(0x1234);
    relative.store(&mut memory, 0x1000).unwrap();
    assert_eq!(Relative32::<Unknown>::SIZE, 4);
    assert_eq!(relative.resolve(0x140000000).unwrap().value, 0x140001234);
    assert!(Relative32::<Unknown>::new(1).resolve(u64::MAX).is_none());
    let signed: alias_l23::A0 = -1;
    assert_eq!(signed, -1i16);
    assert_eq!(alias_l23::A0::SIZE, 2);
    f32::from_bits(0xffc01234)
        .store(&mut memory, 0x1008)
        .unwrap();
    assert_eq!(f32::load(&memory, 0x1008).unwrap().to_bits(), 0xffc01234);
    Enum32(0x80000000).store(&mut memory, 0x1010).unwrap();
    assert_eq!(memory.read_u32(0x1010).unwrap(), 0x80000000);
}
