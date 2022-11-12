#![windows_subsystem = "windows"]

use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,
    Win32::System::LibraryLoader::GetModuleHandleA
};
use rectangular::{
    notification
};

const WINDOW_CLASS_NAME: PCSTR = s!("main");
const WINDOW_NAME: PCSTR = s!("Rectangular");

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;

        let wc = WNDCLASSA {
            style: Default::default(),
            lpfnWndProc: Some(wndproc),
            hInstance: instance,
            lpszClassName: WINDOW_CLASS_NAME,
            ..Default::default()
        };

        RegisterClassA(&wc);

        let window_handle = CreateWindowExA(
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
            None,
        );

        notification::add_notification_icon(instance, window_handle);

        let mut message = MSG::default();
        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            TranslateMessage(&message);
            DispatchMessageA(&message);
        }
    }

    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}