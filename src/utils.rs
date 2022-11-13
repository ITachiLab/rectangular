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
/// assert_eq!(rectangular::low_word!(0xDEADBEEF), 0xBEEF);
/// ```
#[macro_export]
macro_rules! low_word {
    ($x:expr) => {
        ($x as u32 & 0xFFFF) as u16
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
/// assert_eq!(rectangular::high_word!(0xDEADBEEF), 0xDEAD);
/// ```
#[macro_export]
macro_rules! high_word {
    ($x:expr) => {
        (($x as u32 >> 16) & 0xFFFF) as u16
    }
}

/// Macro replacement for Windows API's `GET_X_LPARAM`.
///
/// This macro is similar to [`low_word!`] - it returns a lower 16 bits of the supplied number. The
/// difference is the return type of the macro, this macro preserves sign of the lower word, thus
/// making the return type `i16`.
///
/// # Examples
///
/// ```
/// assert_eq!(rectangular::low_word_signed!(0x1234FFFF), -1);
/// ```
#[macro_export]
macro_rules! low_word_signed {
    ($x:expr) => {
        ($x as u32 & 0xFFFF) as i16
    }
}

/// Macro replacement for Windows API's `GET_Y_LPARAM`.
///
/// This macro is similar to [`high_word!`] - it returns a higher 16 bits of the supplied number.
/// The difference is the return type of the macro, this macro preserves sign of the higher word,
/// thus making the return type `i16`.
///
/// # Examples
///
/// ```
/// assert_eq!(rectangular::high_word_signed!(0xFFFF1234), -1);
/// ```
#[macro_export]
macro_rules! high_word_signed {
    ($x:expr) => {
        (($x as u32 >> 16) & 0xFFFF) as i16
    }
}
