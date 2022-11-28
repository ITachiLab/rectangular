//! This module is dedicated to main window.
//!
//! [`RectangularWindow`] structure is the central component of the application. It is responsible
//! for instantiation and initialization of other components, and also for dispatching proper
//! window messages to them.

use std::ffi::c_void;
use std::ptr::{null};
use windows::core::PCSTR;
use windows::s;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::context_menu::ContextMenu;
use crate::low_word;
use crate::notification::NotificationIcon;

/// An ID of the message which will be used to communicate with the main window's message loop.
pub const WM_NIACTION: u32 = WM_USER + 1;

const WINDOW_CLASS_NAME: PCSTR = s!("main");
const WINDOW_NAME: PCSTR = s!("Rectangular");
const WINDOW_EXTRAS_MAIN: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(0);

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
}

impl Default for RectangularWindow {
    fn default() -> Self {
        RectangularWindow {
            window_handle: Default::default(),
            context_menu: Default::default(),
            notification_icon: Default::default(),
        }
    }
}

impl RectangularWindow {
    /// Create a new system window and a [`RectangularWindow`] for keeping the components together.
    ///
    /// The method prepares and creates a new system window for Rectangular. It also takes care of
    /// instantiating [`RectangularWindow`] structure and passing it properly to the window
    /// procedure for initialization during the `WM_CREATE` message.
    ///
    /// The return value is a boxed [`RectangularWindow`]. The reason for returning a [`Box`] is a
    /// necessity of storing a pointer to [`RectangularWindow`] instance within a window memory
    /// space, hence the instance cannot live on the stack because it will be invalidated soon after
    /// the `new()` method finishes. Thanks to boxing, the instance lives on the heap and the box
    /// can be safely moved out of the method, while leaving the wrapped instance intact.
    pub fn new() -> Box<RectangularWindow> {
        let mut window_box = Box::new(RectangularWindow::default());
        let window_ptr: *mut RectangularWindow = &mut *window_box;

        unsafe {
            let instance = GetModuleHandleA(None)
                .expect("Could not get handle of the application's module.");

            let wc = WNDCLASSA {
                style: Default::default(),
                lpfnWndProc: Some(wndproc),
                hInstance: instance,
                lpszClassName: WINDOW_CLASS_NAME,
                cbWndExtra: (isize::BITS / 8) as i32,
                ..Default::default()
            };

            RegisterClassA(&wc);
            CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                WINDOW_CLASS_NAME,
                WINDOW_NAME,
                WINDOW_STYLE::default(),
                0,
                0,
                0,
                0,
                HWND_MESSAGE,
                None,
                instance,
                Some(window_ptr as *mut c_void),
            );

            window_box
        }
    }

    /// Enter the window message loop.
    ///
    /// Call to this function never returns until the application is going to be closed.
    pub fn enter_loop(&self) {
        let mut message = MSG::default();

        unsafe {
            while GetMessageA(&mut message, HWND(0), 0, 0).into() {
                TranslateMessage(&message);
                DispatchMessageA(&message);
            }
        }
    }

    /// Process a windows message.
    ///
    /// This is the main handler of windows messages, it is called from inside the window procedure.
    /// The handler dispatches messages to appropriate methods of either this class or underlying
    /// components, in particular: context menu and notification icon.
    fn process_message(&self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match message {
            WM_DESTROY => self.handle_destroy(),
            WM_COMMAND => self.handle_command(),
            WM_NIACTION => match low_word!(lparam.0) as u32 {
                WM_CONTEXTMENU => self.context_menu.show(wparam),
                _ => unsafe { DefWindowProcA(self.window_handle, message, wparam, lparam) }
            }
            _ => unsafe { DefWindowProcA(self.window_handle, message, wparam, lparam) }
        }
    }

    /// Handler of `WM_CREATE` message.
    ///
    /// In contrast to other handlers, this handler is not called from
    /// [`RectangularWindow::process_message`] method because the instance is not yet fully
    /// initialized. This method is called automatically only once from the window procedure.
    fn handle_create(&mut self, window_handle: HWND, app_instance: HINSTANCE) -> LRESULT {
        self.window_handle = window_handle;

        self.notification_icon = NotificationIcon::new(window_handle, app_instance);
        self.notification_icon.add_to_window();

        self.context_menu = ContextMenu::new(window_handle);

        LRESULT(0)
    }

    /// Handler of `WM_COMMAND` message.
    fn handle_command(&self) -> LRESULT {
        unsafe { PostMessageA(self.window_handle, WM_CLOSE, WPARAM(0), LPARAM(0)); }
        LRESULT(0)
    }

    /// Handler of `WM_DESTROY` message.
    ///
    /// This is a perfect place to do any cleanup actions before the application closes. Anything
    /// that is not covered by [`Drop`] trait must be "dropped" here.
    fn handle_destroy(&self) -> LRESULT {
        unsafe { PostQuitMessage(0); }
        LRESULT(0)
    }
}

/// This is a window procedure, the glue between Windows and the application.
///
/// The code within the procedure is limited to the necessary minimum. Message processing and other
/// logic lives inside [`RectangularWindow`] instance.
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        if message == WM_CREATE {
            let create_struct = lparam.0 as *const CREATESTRUCTA;
            let rectangular_window = (*create_struct).lpCreateParams as *mut RectangularWindow;

            SetWindowLongPtrA(window, WINDOW_EXTRAS_MAIN, rectangular_window as isize);

            (*rectangular_window).handle_create(window, (*create_struct).hInstance)
        } else {
            let rectangular_window =
                GetWindowLongPtrA(window, WINDOW_EXTRAS_MAIN) as *const RectangularWindow;

            // Some messages are sent before the RectangularWindow is instantiated.
            if rectangular_window != null() {
                (*rectangular_window).process_message(message, wparam, lparam)
            } else {
                DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }
}
