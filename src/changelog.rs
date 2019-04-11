//! Project changelog

/// Release 0.3.0
///
/// * Removed the `Receivable` trait, because it is difficult to write meaningful code with `<T as
/// Receivable>` for `T ≠ RawFd`.
/// * Removed the `Sendable` trait. While the generalisation here is sound and is beneficial, the
/// inconsistency between ability to send any sendable and then only being able to receive a
/// `RawFd` was deemed to be not worth it.
pub mod r0_3_0 {}

/// Release 0.2.1
///
/// Removed an accidentally publicly exported internal function.
///
/// 0.2.0 has been yanked.
pub mod r0_2_1 {}

/// Release 0.2.0
///
/// Pure-Rust reimplementation of the crate.
pub mod r0_2_0 {}
