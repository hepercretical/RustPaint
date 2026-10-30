//! The recovered guard thunk is exactly `JMP RAX` (FF E0).
//! This safe execution-model transition does not perform a host indirect call.

/// The target register and program counter are explicit. `preserved` carries
/// all remaining caller state (registers, flags, stack and memory) unchanged.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TailContext<T> {
    pub instruction_pointer: u64,
    pub rax: u64,
    pub preserved: T,
}

/// Source copy at line 176477, recovered from the thunk at VA 0x1400cc000.
/// The execution driver fetches/dispatches the new PC with the same caller frame.
pub fn named_l176477<T>(state: &mut TailContext<T>) {
    state.instruction_pointer = state.rax;
}

/// The second printed source copy has the identical one-instruction body.
pub fn named_l176492<T>(state: &mut TailContext<T>) {
    state.instruction_pointer = state.rax;
}
