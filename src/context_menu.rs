//! This module contains implementation of a context menu.
//!
//! The context menu appears when a user clicks a notification icon with a secondary mouse button.
//! The menu is the primary control center of the Rectangular application, as Rectangular does not
//! have any windows.

use windows::core::PCSTR;
use windows::s;
use windows::Win32::{
    Foundation::*, UI::WindowsAndMessaging::*,
};

use crate::{high_word_signed, low_word_signed};

/// An array of tuples, each representing a descriptor of the context menu item.
///
/// The tuple members are:
/// 0. Menu item flags ([`MENU_ITEM_FLAGS`]).
/// 1. Index of the menu entry, this will be used by `WM_COMMAND` message when user selects an
/// entry.
/// 2. Title of the entry.
const MENU_ENTRIES: [(MENU_ITEM_FLAGS, usize, PCSTR); 1] = [
    (MF_STRING, 1, s!("Exit"))
];

/// ContextMenu includes data and methods strictly related to the context menu of a notification
/// icon.
pub struct ContextMenu {
    menu_handle: HMENU,
    window_handle: HWND,
}

impl Default for ContextMenu {
    fn default() -> Self {
        ContextMenu {
            menu_handle: Default::default(),
            window_handle: Default::default()
        }
    }
}

impl ContextMenu {
    /// Create a new context menu.
    ///
    /// Upon creating, all menu entries from [`MENU_ENTRIES`] are added to the menu.
    ///
    /// # Panics
    ///
    /// Panics if a call to [`CreatePopupMenu`] failed.
    pub fn new(window_handle: HWND) -> ContextMenu {
        unsafe {
            let menu_handle = CreatePopupMenu().expect("Could not create popup menu.");

            for (flags, id, title) in &MENU_ENTRIES {
                AppendMenuA(menu_handle, *flags, *id, *title);
            }

            ContextMenu { menu_handle, window_handle }
        }
    }

    /// Show the context menu.
    ///
    /// This method extracts X and Y position of the cursor from the supplied `WPARAM` and uses
    /// these coordinates to show a context menu.
    ///
    /// The method follows the recommended way of showing menus in order to avoid glitches:
    /// [TrackPopupMenu#Remarks](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-trackpopupmenu#remarks).
    pub fn show(&self, wparam: WPARAM) {
        let x = low_word_signed!(wparam.0);
        let y = high_word_signed!(wparam.0);

        unsafe {
            SetForegroundWindow(self.window_handle);
            TrackPopupMenu(self.menu_handle, TPM_LEFTALIGN | TPM_LEFTBUTTON | TPM_BOTTOMALIGN,
                           x as i32, y as i32, 0, self.window_handle, None);
            PostMessageA(self.window_handle, WM_NULL, WPARAM(0), LPARAM(0));
        }
    }
}

impl Drop for ContextMenu {
    fn drop(&mut self) {
        unsafe {
            DestroyMenu(self.menu_handle);
        }
    }
}