//! This module contains implementation of a notification icon.
//!
//! The notification icon is the primary, and only entry point of Rectangular. By interacting with
//! the icon in the notification area, user can control behavior of Rectangular, modify settings
//! and arrange other windows. Aside from keyboard shortcuts, any interaction with Rectangular
//! begins with clicking on the notification icon with either primary or secondary mouse button.
//!
//! # Primary mouse button
//!
//! When user clicks on the notification icon with the primary mouse button, a small window near the
//! notification area appears. The window offers numerous ways of arranging windows on the screen.
//!
//! # Secondary mouse button
//!
//! When user clicks on the notification icon with the secondary mouse button, a context menu
//! appears. The context menu includes entries related strictly to Rectangular itself, like:
//! settings, help or possibility to exit the application.

use std::mem;

use windows::core::{
    GUID, PCSTR,
};
use windows::s;
use windows::Win32::{
    Foundation::*, UI::Shell::*, UI::WindowsAndMessaging::*,
};

use crate::WM_NIACTION;

/// Application internal ID of the notification icon.
const ICON_UID: u32 = 1;

/// A text that will appear in the notification icon's tooltip.
const ICON_TITLE: PCSTR = s!("Rectangular");

/// An ID of a resource with the icon.
const ICON_RESOURCE_ID: PCSTR = PCSTR(1 as *const u8);

/// This structure represents a notification icon displayed in the system's notification area.
///
/// The purpose of this structure is to keep data related to the notification icon in one place,
/// these being: handle of the main window and handle of notification icon's icon.
pub struct NotificationIcon {
    window_handle: HWND,
    icon_handle: HICON,
}

impl Default for NotificationIcon {
    fn default() -> Self {
        NotificationIcon {
            window_handle: Default::default(),
            icon_handle: Default::default(),
        }
    }
}

impl NotificationIcon {
    pub fn new(window_handle: HWND, module_instance: HINSTANCE) -> NotificationIcon {
        unsafe {
            let image_handle = LoadImageA(module_instance, ICON_RESOURCE_ID, IMAGE_ICON,
                                          16, 16, LR_DEFAULTCOLOR).expect("Could not load icon.");
            NotificationIcon { window_handle, icon_handle: HICON(image_handle.0) }
        }
    }

    /// Initialize and add an icon to the notification area.
    pub fn add_to_window(&self) {
        let mut icon_data = NOTIFYICONDATAA {
            cbSize: mem::size_of::<NOTIFYICONDATAA>() as u32,
            hWnd: self.window_handle,
            uID: ICON_UID,
            uFlags: NIF_MESSAGE | NIF_INFO | NIF_ICON | NIF_TIP | NIF_STATE | NIF_SHOWTIP,
            uCallbackMessage: WM_NIACTION,
            hIcon: self.icon_handle,
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

        unsafe {
            for (i, ch) in ICON_TITLE.as_bytes().iter().enumerate() {
                icon_data.szTip[i] = CHAR(*ch);
            }

            Shell_NotifyIconA(NIM_ADD, &icon_data)
                .expect("Could not add icon to notification area.");
            Shell_NotifyIconA(NIM_SETVERSION, &icon_data)
                .expect("Could not set version of notification icon.");
        }
    }
}

impl Drop for NotificationIcon {
    fn drop(&mut self) {
        let icon_data = NOTIFYICONDATAA {
            hWnd: self.window_handle,
            uID: ICON_UID,
            ..Default::default()
        };

        unsafe {
            Shell_NotifyIconA(NIM_DELETE, &icon_data);
            DestroyIcon(self.icon_handle);
        }
    }
}
