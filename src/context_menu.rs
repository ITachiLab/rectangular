//! This modules gathers functions related to any kind of interaction with context menus.

use windows::core::PCSTR;
use windows::s;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::{high_word_signed, low_word_signed};

const MENU_ENTRIES: [(MENU_ITEM_FLAGS, usize, PCSTR); 1] = [
    (MF_STRING, 1, s!("Exit"))
];

pub struct ContextMenu {
    handle: HMENU,
    owner: HWND,
}

impl ContextMenu {
    pub unsafe fn new(owner: HWND) -> Option<ContextMenu> {
        let handle = CreatePopupMenu().ok()?;
        let menu = ContextMenu { handle, owner };

        for (flags, id, title) in &MENU_ENTRIES {
            AppendMenuA(handle, *flags, *id, *title);
        }

        Some(menu)
    }

    pub unsafe fn from_window(owner: HWND) -> *const ContextMenu {
        GetWindowLongPtrA(owner, WINDOW_LONG_PTR_INDEX(0)) as *const ContextMenu
    }

    pub unsafe fn bind_to_window(&self) {
        let ptr = self as *const ContextMenu;
        SetWindowLongPtrA(self.owner, WINDOW_LONG_PTR_INDEX(0), ptr as isize);
    }

    pub unsafe fn show(&self, wparam: WPARAM) {
        let x = low_word_signed!(wparam.0);
        let y = high_word_signed!(wparam.0);

        SetForegroundWindow(self.owner);
        TrackPopupMenu(self.handle, TPM_LEFTALIGN | TPM_LEFTBUTTON | TPM_BOTTOMALIGN, x as i32,
                       y as i32, 0, self.owner, None);
        PostMessageA(self.owner, WM_NULL, WPARAM(0), LPARAM(0));
    }
}

impl Drop for ContextMenu {
    fn drop(&mut self) {
        unsafe {
            DestroyMenu(self.handle);
        }
    }
}