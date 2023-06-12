//! This module is dedicated to main window.
//!
//! [`RectangularWindow`] implements behaviour of the main application window which is a so-called
//! "message-only window".

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use windows::core::PCSTR;
use windows::s;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::app_window::AppWindow;
use crate::context_menu::ContextMenu;
use crate::control_center::ControlCenter;
use crate::{low_word, WINDOW_CLASS_NAME, WM_NIACTION};
use crate::notification::NotificationIcon;

/// A name of the main application window.
const WINDOW_NAME: PCSTR = s!("Rectangular");

/// This structure represents the main application window.
///
/// The purpose of this class is to be a container for things that are strictly related to the main
/// window. All components relying on main window and its handle should be a part of this structure.
/// The main reason of centralising this is to avoid keeping too many global variables for all the
/// stuff that is going to live throughout the application's lifetime.
pub struct RectangularWindow {
    window_handle: HWND,

    pub context_menu: ContextMenu,
    pub notification_icon: NotificationIcon,
    pub control_center: Rc<RefCell<ControlCenter>>,
}

impl Default for RectangularWindow {
    fn default() -> Self {
        RectangularWindow {
            window_handle: Default::default(),
            context_menu: Default::default(),
            notification_icon: Default::default(),
            control_center: Default::default(),
        }
    }
}

impl AppWindow for RectangularWindow {
    fn handle_create(&mut self, app_instance: HINSTANCE, window: HWND, _message: u32,
                     _wparam: WPARAM, _lparam: LPARAM) -> LRESULT {
        self.window_handle = window;

        self.notification_icon = NotificationIcon::new(window, app_instance);
        self.notification_icon.add_to_window();

        self.context_menu = ContextMenu::new(window);

        self.control_center = ControlCenter::new(app_instance);

        LRESULT(0)
    }

    fn process_message(&self, _window: HWND, message: u32, wparam: WPARAM,
                       lparam: LPARAM) -> LRESULT {
        match message {
            WM_DESTROY => {
                unsafe { PostQuitMessage(0); }
            },
            WM_COMMAND => {
                unsafe { PostMessageA(self.window_handle, WM_CLOSE, WPARAM(0), LPARAM(0)); }
            },
            WM_NIACTION => match low_word!(lparam.0) as u32 {
                WM_CONTEXTMENU => {
                    self.context_menu.show(wparam);
                },
                WM_LBUTTONUP => {
                    self.control_center.borrow().show(wparam);
                },
                _ => unsafe { return DefWindowProcA(self.window_handle, message, wparam, lparam); }
            },
            _ => unsafe { return DefWindowProcA(self.window_handle, message, wparam, lparam); }
        }

        LRESULT(0)
    }
}

impl RectangularWindow {
    /// Create a new system window and a [`RectangularWindow`] instance.
    pub fn new(instance: HINSTANCE) -> Rc<RefCell<RectangularWindow>> {
        let my_rc = Rc::new(RefCell::new(RectangularWindow::default()));
        let boxed = Box::new(Rc::clone(&my_rc) as Rc<RefCell<dyn AppWindow>>);
        let raw = Box::into_raw(boxed);

        unsafe {
            CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                WINDOW_CLASS_NAME,
                WINDOW_NAME,
                WINDOW_STYLE::default(),
                0, 0, 0, 0,
                HWND_MESSAGE,
                None,
                instance,
                Some(raw as *const Rc<RefCell<dyn AppWindow>> as *const c_void),
            );
        }

        my_rc
    }
}