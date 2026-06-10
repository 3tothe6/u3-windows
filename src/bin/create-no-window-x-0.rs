#![windows_subsystem = "windows"]

use winrt_notification::Toast;

fn main() {
    if let Err(e) = u3_windows::create_no_window_x::main(true) {
        Toast::new(Toast::POWERSHELL_APP_ID)
            .title(&format!("create-no-window-x-0 error: {e:?}"))
            .show()
            .unwrap();
    }
}
