use mspaint_port::{
    declaration_support::{Address, Wire},
    declarations38::*,
    recovered::call_support::MemoryImage,
};
#[test]
fn template_alias_pointers_preserve_exact_words_and_checked_storage() {
    let mut image = MemoryImage {
        base: 0x1000,
        bytes: vec![0; 24],
    };
    let a: alias_l1740::A1 = Address::new(0x123456789abcdef0);
    let b: alias_l1755::A1 = Address::new(0xffffffffffffffff);
    let c: alias_l1770::A1 = Address::new(0);
    a.store(&mut image, 0x1000).unwrap();
    b.store(&mut image, 0x1008).unwrap();
    c.store(&mut image, 0x1010).unwrap();
    assert_eq!(
        alias_l1740::A1::load(&image, 0x1000).unwrap().value,
        a.value
    );
    assert_eq!(
        alias_l1755::A1::load(&image, 0x1008).unwrap().value,
        b.value
    );
    assert_eq!(
        alias_l1770::A1::load(&image, 0x1010).unwrap().value,
        c.value
    );
    assert!(a.store(&mut image, 0x1011).is_err());
}
