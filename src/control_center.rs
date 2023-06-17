//! This module contains implementation of "Control Center" window.
//!
//! "Control Center" is a pop-up window which shows up when user clicks on the tray icon with a left
//! mouse button. The window presents possible options of arranging windows on a monitor.

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use windows::s;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::app_window::AppWindow;
use crate::{high_word_signed, low_word, low_word_signed, make_window_object, WINDOW_CLASS_NAME};

const WINDOW_WIDTH: i32 = 300;
const WINDOW_HEIGHT: i32 = 200;

pub struct ControlCenter {
    window_handle: HWND,
}

impl Default for ControlCenter {
    fn default() -> Self {
        ControlCenter {
            window_handle: HWND(0),
        }
    }
}

impl AppWindow for ControlCenter {
    fn handle_create(&mut self, _app_instance: HINSTANCE, window: HWND, _message: u32,
                     _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        self.window_handle = window;
        LRESULT(0)
    }

    fn process_message(&self, _window: HWND, message: u32, wparam: WPARAM,
                       lparam: LPARAM) -> LRESULT {
        match message {
            WM_ACTIVATE => {
                if low_word!(wparam.0) as u32 == WA_INACTIVE {
                    self.hide();
                }
            }
            _ => unsafe { return DefWindowProcW(self.window_handle, message, wparam, lparam); }
        }

        LRESULT(0)
    }
}

impl ControlCenter {
    pub fn new(instance: HINSTANCE) -> Rc<RefCell<ControlCenter>> {
        let (ptr, raw) = make_window_object!(ControlCenter);

        unsafe {
            CreateWindowExA(
                WS_EX_PALETTEWINDOW,
                WINDOW_CLASS_NAME,
                s!(""),
                WS_POPUP | WS_THICKFRAME,
                0, 0, WINDOW_WIDTH, WINDOW_HEIGHT,
                HWND::default(),
                None,
                instance,
                Some(raw),
            );
        }

        ptr
    }

    /// Show the Control Center window.
    ///
    /// The Control Center window is created along with the Rectangular's main window, but it's
    /// initially hidden. When called, this method shows the window near the notification area, and
    /// sets it as the foreground window. The proper position can be calculated from the `wparam`
    /// parameter which is usually obtained from a messages sent when the notification icon has been
    /// clicked.
    pub fn show(&self, wparam: WPARAM) {
        unsafe {
            let x = low_word_signed!(wparam.0) as i32;
            let y = high_word_signed!(wparam.0) as i32;
            let point = POINT { x, y };
            let size = SIZE { cx: WINDOW_WIDTH, cy: WINDOW_HEIGHT };
            let mut result = RECT::default();

            let calc_flags = TPM_CENTERALIGN | TPM_VCENTERALIGN | TPM_VERTICAL | TPM_WORKAREA;
            let show_flags = SWP_NOSIZE | SWP_SHOWWINDOW;

            CalculatePopupWindowPosition(&point, &size, calc_flags.0, None, &mut result);
            SetWindowPos(self.window_handle, HWND_TOPMOST, result.left, result.top, 0, 0,
                         show_flags);
            SetForegroundWindow(self.window_handle);
        }
    }

    /// Hide the Control Center window.
    ///
    /// The Control Center window should never be destroyed, unless Rectangular is closed. Instead,
    /// the window is simply hidden when it's no longer needed.
    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.window_handle, SW_HIDE);
        }
    }
}
