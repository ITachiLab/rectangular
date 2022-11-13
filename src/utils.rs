//! A module with utilities common across project's modules.
//!
//! This is a perfect place to put macros and functions that do not belong to any specific context
//! and are used by different modules within the project. Just make sure this file doesn't grow too
//! much. If it happens to, refactor it to submodules.

/// Macro replacement for Windows API's `LOWORD`.
///
/// This macro is intended to be used only with number values. The result of the macro is a lower
/// word (lower 16 bits) of the supplied number.
///
/// # Examples
///
/// ```
/// assert_eq!(rectangular::low_word!(0xDEADBEEF), 0xBEEF)
/// ```
#[macro_export]
macro_rules! low_word {
    ($x:expr) => {
        $x & 0xFFFF
    }
}

/// Macro replacement for Windows API's `HIWORD`.
///
/// This macro is intended to be used only with number values. The result of the macro is a higher
/// word (higher 16 bits) of the supplied number.
///
/// # Examples
///
/// ```
/// assert_eq!(rectangular::high_word!(0xDEADBEEF), 0xDEAD)
/// ```
#[macro_export]
macro_rules! high_word {
    ($x:expr) => {
        loword!($x >> 16)
    }
}