use mspaint_port::recovered::guard40::{self, TailContext};
#[test]
fn both_guard_copies_transfer_to_rax_without_changing_the_caller_frame() {
    let initial = (
        vec![0x55u8; 256],
        [0x123456789abcdef0u64; 16],
        [[0xabu8; 64]; 32],
        0x202u64,
        0x7fff0000u64,
    );
    for target in [0x140001000, 0x1400dfa50, 0x7ff600001000] {
        let mut first = TailContext {
            instruction_pointer: 0x1400cc000,
            rax: target,
            preserved: initial.clone(),
        };
        let mut second = first.clone();
        guard40::named_l176477(&mut first);
        guard40::named_l176492(&mut second);
        assert_eq!(first, second);
        assert_eq!(first.instruction_pointer, target);
        assert_eq!(first.rax, target);
        assert_eq!(first.preserved, initial);
    }
}
#[test]
fn tail_transition_keeps_noncopy_context_owned_by_the_driver() {
    let mut context = TailContext {
        instruction_pointer: 0x1400cc000,
        rax: 0x140001000,
        preserved: Box::new(String::from("same return address and stack frame")),
    };
    let allocation = &*context.preserved as *const String;
    guard40::named_l176477(&mut context);
    assert_eq!(allocation, &*context.preserved as *const String);
}
