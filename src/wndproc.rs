//! This module contains code of the main window procedure.

use std::cell::RefCell;
use std::rc::Rc;

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, COLOR_WINDOWFRAME, EndPaint, FillRect, HBRUSH, PAINTSTRUCT,
};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::app_window::AppWindow;
use crate::WINDOW_EXTRAS_MAIN;

/// This is a window procedure, the glue between Windows and the application.
///
/// The code within the procedure is limited to the necessary minimum. Message processing and other
/// logic lives inside windows' implementations which are based on [`AppWindow`] trait.
///
/// The messages serviced here are:
///
/// ### `WM_CREATE`
/// In order to route messages coming to a specific window to its Rust object, a connection between
/// the system window and its object must be maintained somehow. Here it's done by storing a pointer
/// to the [`AppWindow`] trait inside the windows' memory. Every window implementation in
/// Rectangular has to implement this trait because it exposes methods necessary to route the
/// messages from here to the owning object. Thanks to this it's possible to leverage the
/// polymorphic behavior and do not care about the concrete implementation.
///
/// The pointer to [`AppWindow`] is passed to `WM_CREATE` as a result of calling
/// [`CreateWindowExA`]. **Important: the pointer is not managed by any of the smart pointer
/// mechanisms!** A memory leak can occur if the pointer is not stored anywhere, and it's not
/// later deallocated.
///
/// When the necessary stuff is done, the window implementation can add some more processing by
/// implementing [`AppWindow::handle_create`].
///
/// ### `WM_PAINT`
/// It's a default painting implementation as suggested by [`WM_PAINT` documentation](https://learn.microsoft.com/en-us/windows/win32/gdi/wm-paint#example).
///
/// ### `WM_CLOSE`
/// Triggers [`DestroyWindow`].
///
/// ### `WM_NCDESTROY`
/// `WM_NCDESTROY` is the last message sent to a window being destroyed. After this, no more
/// messages will be sent, so the window's resources can be safely freed. This is the place where
/// the pointer to [`AppWindow`] is converted back to the [`Box`], so it can be properly dropped.
/// When that pointer was the last reference to the window object, the object will be dropped too.
///
/// ### Other messages
/// Messages not handled here are routed to the object associated with the window. Window
/// implementations can handle them in [`AppWindow::process_message`] method.
pub extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM,
                               lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                let create_struct = lparam.0 as *const CREATESTRUCTA;
                let app_window = (*create_struct).lpCreateParams
                    as *mut Rc<RefCell<dyn AppWindow>>;

                SetWindowLongPtrA(window, WINDOW_EXTRAS_MAIN, app_window as isize);

                (*app_window).borrow_mut()
                    .handle_create((*create_struct).hInstance, window, message, wparam, lparam)
            }
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);

                FillRect(hdc, &ps.rcPaint, HBRUSH(COLOR_WINDOWFRAME.0 as i32 as isize));
                EndPaint(window, &ps);
                LRESULT(0)
            }
            WM_CLOSE => {
                DestroyWindow(window);
                LRESULT(0)
            }
            WM_NCDESTROY => {
                let app_window =
                    GetWindowLongPtrA(window, WINDOW_EXTRAS_MAIN)
                        as *mut Rc<RefCell<dyn AppWindow>>;

                if !app_window.is_null() {
                    let unboxed = Box::from_raw(app_window);
                    let result = (*unboxed).borrow()
                        .process_message(window, message, wparam, lparam);
                    result
                } else {
                    DefWindowProcA(window, message, wparam, lparam)
                }
            }
            _ => {
                let app_window = GetWindowLongPtrA(window, WINDOW_EXTRAS_MAIN)
                    as *mut Rc<RefCell<dyn AppWindow>>;

                // Some messages could be sent before the window is instantiated, so route them to
                // the default procedure.
                if !app_window.is_null() {
                    (*app_window).borrow().process_message(window, message, wparam, lparam)
                } else {
                    DefWindowProcA(window, message, wparam, lparam)
                }
            }
        }
    }
}
