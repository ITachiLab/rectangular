use windows::{
    core::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*, Win32::System::Threading::*,
};

fn main() -> Result<()> {
    unsafe {
        let event = CreateEventW(None, true, false, None)?;
        SetEvent(event).ok()?;
        WaitForSingleObject(event, 0);
        CloseHandle(event).ok()?;

        MessageBoxA(None, s!("Hello"), s!("Caption"), MB_OK);
        MessageBoxW(None, w!("World"), w!("Caption"), MB_OK);
    }

    Ok(())
}
