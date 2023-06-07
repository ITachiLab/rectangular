//! This module it entirely dedicated to [`AppWindow`] trait.

use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};

/// A trait defining methods common to all application windows.
///
/// The purpose of this trait is to allow dispatching windows messages to instances responsible for
/// those windows.
pub trait AppWindow {
    /// Handle `WM_CREATE` message sent to the window.
    ///
    /// `WM_CREATE` is usually the first message received by the newly created window, so it also
    /// requires special handling. The method receives handle to the created window, as well as an
    /// application [`HINSTANCE`].
    ///
    /// The default message handling still applies, in a sense that the method must return
    /// [`LRESULT`] according to the method result.
    fn handle_create(&mut self, app_instance: HINSTANCE, window: HWND, message: u32,
                     wparam: WPARAM, lparam: LPARAM) -> LRESULT;

    /// Handle a message sent to the window.
    ///
    /// This method will be invoked for every windows message which is not `WM_CREATE` message. The
    /// method receives the same parameters as the window procedure, and is also expected to return
    /// [`LRESULT`].
    fn process_message(&self, window: HWND, message: u32, wparam: WPARAM,
                       lparam: LPARAM) -> LRESULT;
}