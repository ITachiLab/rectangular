use std::cell::RefCell;
use std::rc::Rc;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, COLOR_WINDOWFRAME, EndPaint, FillRect, HBRUSH, PAINTSTRUCT
};
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::app_window::AppWindow;
use crate::WINDOW_EXTRAS_MAIN;

/// This is a window procedure, the glue between Windows and the application.
///
/// The code within the procedure is limited to the necessary minimum. Message processing and other
/// logic lives inside windows' implementations which are based on [`AppWindow`] trait.
///
/// `WM_CREATE` messages are routed to [`AppWindow::handle_create()`] methods. Other messages are
/// handled by [`AppWindow::process_message`].
pub extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
