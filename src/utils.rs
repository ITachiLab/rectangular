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

/// Create a window object, and get its reference-counting pointer, along with a raw pointer.
///
/// This is a helper macro for use within `new` methods of structures implementing window
/// behaviour. The macro creates a new [`Rc`] with a [`RefCell`] wrapping a concrete implementation
/// of the window, then it creates a clone of that pointer (effectively incrementing a number of
/// strong references), puts the clone on a heap, and makes it an unmanaged raw pointer.
///
/// The macro returns a tuple where the first element is the original [`Rc`] pointer, and the other
/// element is the raw pointer which was placed on a heap. The raw pointer must be given to
/// [`CreateWindowExA`] function, so it's passed to `WM_CREATE` message where it's stored within the
/// window's memory, while the original [`Rc`] can be either discarded, or stored for future
/// interactions with the instance.
///
/// It's crucial to remember about a proper deallocation of the raw pointer.
///
/// # Examples
///
/// In the below example, the original pointer is discarded, so the only entity keeping the
/// `MyWindow` object alive is a raw pointer created from the cloned original pointer. Since the
/// raw pointer is not owned by anything yet, it will keep the `MyWindow` alive until it's properly
/// dropped (usually by reclaiming it with [`Box::from_raw`]).
///
/// ```
/// pub fn new(instance: HINSTANCE) {
///     let (_, raw) = make_window_object!(MyWindow);
///
///     unsafe {
///         CreateWindowExA(
///             WS_EX_PALETTEWINDOW,
///             WINDOW_CLASS_NAME,
///             s!(""),
///             WS_POPUP | WS_THICKFRAME | WS_VISIBLE,
///             0, 0, WINDOW_WIDTH, WINDOW_HEIGHT,
///             HWND::default(),
///             None,
///             instance,
///             Some(raw),
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! make_window_object {
    ($x:ty) => {
        {
            let rc = Rc::new(RefCell::new(<$x>::default()));
            let boxed = Box::new(Rc::clone(&rc) as Rc<RefCell<dyn AppWindow>>);
            (rc, Box::into_raw(boxed) as *mut Rc<RefCell<dyn AppWindow>> as *const c_void)
        }
    }
}
