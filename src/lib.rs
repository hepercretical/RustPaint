//! Incremental translation of `mspaint.c`. This is not yet a runnable Paint app.
//!
//! Original function addresses are recorded at each translated entry point.
//! Windows integers retain their original widths, including on non-Windows hosts.
//! Unresolved pointers, globals, vtables and imports are not fabricated.

#![forbid(unsafe_code)]

pub mod bitmap;
pub mod geometry;
pub mod numeric;
pub mod recovered;
pub mod status;
pub mod strings;

pub mod canvas;
pub mod declaration_support;
pub mod declarations30;
pub mod declarations31;
pub mod declarations38;
pub mod declarations40;
pub mod return_types41;
pub mod runtime;
