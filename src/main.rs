#![windows_subsystem = "windows"]

use windows::core::Result;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::*;

use rectangular::rectangular_window::RectangularWindow;
use rectangular::WINDOW_CLASS_NAME;
use rectangular::wndproc::wndproc;

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