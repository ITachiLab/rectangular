use std::mem;
use windows::core::{
    PCSTR, GUID,
};
use windows::Win32::{
    Foundation::*, UI::WindowsAndMessaging::*, UI::Shell::*,
};
use windows::s;

/// An ID of the message which will be used to communicate with the main window's message loop.
pub const CALLBACK_MESSAGE: u32 = WM_USER + 1;

/// Application internal ID of the notification icon.
const ICON_UID: u32 = 1;

/// A text that will appear in the notification icon's tooltip.
const ICON_TITLE: PCSTR = s!("Rectangular");

/// An ID of a resource with the icon.
const ICON_RESOURCE_ID: PCSTR = PCSTR(1 as *const u8);

/// Initialize and add an icon to the notification area.
pub unsafe fn add_notification_icon(instance: HINSTANCE, main_window: HWND) {
    let handle = LoadImageA(instance, ICON_RESOURCE_ID, IMAGE_ICON,
                            16, 16, LR_DEFAULTCOLOR).expect("Could not load icon.");

    let mut icon_data = NOTIFYICONDATAA {
        cbSize: mem::size_of::<NOTIFYICONDATAA>() as u32,
        hWnd: main_window,
        uID: ICON_UID,
        uFlags: NIF_MESSAGE | NIF_INFO | NIF_ICON | NIF_TIP | NIF_STATE | NIF_SHOWTIP,
        uCallbackMessage: CALLBACK_MESSAGE,
        hIcon: HICON(handle.0),
        szTip: [CHAR(0); 128],
        dwState: NOTIFY_ICON_STATE(0),
        dwStateMask: 0,
        szInfo: [CHAR(0); 256],
        Anonymous: NOTIFYICONDATAA_0 { uVersion: NOTIFYICON_VERSION_4 },
        szInfoTitle: [CHAR(0); 64],
        dwInfoFlags: NIIF_NONE,
        guidItem: GUID::zeroed(),
        hBalloonIcon: Default::default(),
    };

    for (i, ch) in ICON_TITLE.as_bytes().iter().enumerate() {
        icon_data.szTip[i] = CHAR(*ch);
    }

    Shell_NotifyIconA(NIM_ADD, &icon_data)
        .expect("Could not add icon to notification area.");
    Shell_NotifyIconA(NIM_SETVERSION, &icon_data)
        .expect("Could not set version of notification icon.");
}

pub unsafe fn delete_notification_icon(main_window: HWND) {
    let icon_data = NOTIFYICONDATAA {
        hWnd: main_window,
        uID: ICON_UID,
        ..Default::default()
    };

    Shell_NotifyIconA(NIM_DELETE, &icon_data);
}