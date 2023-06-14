//! Rectangular Library crate.
//!
//! Aside from the typical use-case of a library crate, which is bringing components into the scope,
//! the crate is also used to keep all globally used constants in one place.

use windows::core::PCSTR;
use windows::s;
use windows::Win32::UI::WindowsAndMessaging::{WINDOW_LONG_PTR_INDEX, WM_USER};

pub mod notification;
pub mod utils;
pub mod context_menu;
pub mod rectangular_window;
pub mod control_center;
pub mod app_window;
pub mod wndproc;

/// A window class used by all Rectangular windows.
pub const WINDOW_CLASS_NAME: PCSTR = s!("Rectangular_Common_Class");

/// An index in "window extras" where reference to a window implementation is kept.
pub const WINDOW_EXTRAS_MAIN: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(0);

/// An ID of the message which will be used to communicate with the main window's message loop.
pub const WM_NIACTION: u32 = WM_USER + 1;
