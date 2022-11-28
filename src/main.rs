#![windows_subsystem = "windows"]

use windows::core::Result;
use rectangular::rectangular_window::RectangularWindow;

fn main() -> Result<()> {
    let rectangular_window: Box<RectangularWindow> = RectangularWindow::new();
    (*rectangular_window).enter_loop();

    Ok(())
}
