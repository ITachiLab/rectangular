#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use windows::core::Result;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{
    BeginPaint, COLOR_WINDOWFRAME, EndPaint, FillRect, HBRUSH, PAINTSTRUCT
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use rectangular::app_window::AppWindow;
use rectangular::rectangular_window::RectangularWindow;
use rectangular::{WINDOW_CLASS_NAME, WINDOW_EXTRAS_MAIN};

fn main() -> Result<()> {
    unsafe {
        let app_instance = GetModuleHandleA(None)
            .expect("A valid application handle should be returned from GetModuleHandleA");

        let wc = WNDCLASSA {
            style: CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: app_instance,
            lpszClassName: WINDOW_CLASS_NAME,
            cbWndExtra: ((isize::BITS / 8) * 4) as i32,
            hCursor: LoadCursorW(None, IDC_ARROW)
                .expect("A cursor should be loaded by LoadCursorW"),
            ..Default::default()
        };

        RegisterClassA(&wc);

        let _ = RectangularWindow::new(app_instance);

        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }

        UnregisterClassA(WINDOW_CLASS_NAME, app_instance);
    }

    Ok(())
}


/// This is a window procedure, the glue between Windows and the application.
///
/// The code within the procedure is limited to the necessary minimum. Message processing and other
/// logic lives inside windows' implementations which are based on [`AppWindow`] trait.
///
/// `WM_CREATE` messages are routed to [`AppWindow::handle_create()`] methods. Other messages are
/// handled by [`AppWindow::process_message`].
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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
