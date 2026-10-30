use mspaint_port::declaration_support::Wire;
use mspaint_port::recovered::call_support::MemoryImage;
use mspaint_port::return_types41::*;

#[test]
fn pointer_return_remains_an_image_address_with_checked_record_access() {
    let pointer = decode_fun_140052150(0x140001003);
    let memory = MemoryImage {
        base: 0x140001000,
        bytes: vec![0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0x12, 0, 0, 0],
    };
    let point = pointer.load(&memory).unwrap();
    assert_eq!(point.bytes, [0xff, 0xff, 0xff, 0xff, 0x12, 0, 0, 0]);
    assert!(decode_fun_140052150(0x140001004).load(&memory).is_err());
    assert_eq!(decode_fun_1400022b0(u64::MAX).value, u64::MAX);
    assert_eq!(<R_FUN_140052150 as Wire>::SIZE, 8);
}

#[test]
fn narrow_return_discards_only_the_unobservable_upper_register_bits() {
    for word in [0, 1, 0x100, 0x123456789abcdef0, u64::MAX] {
        let value = decode_fun_1400356d0(word);
        assert_eq!(value.to_le_bytes(), [word.to_le_bytes()[0]]);
        assert_eq!(decode_fun_1400048e0(word), word);
    }
    assert_eq!(<R_FUN_1400356d0 as Wire>::SIZE, 1);
    assert_eq!(<R_FUN_1400048e0 as Wire>::SIZE, 8);
}
