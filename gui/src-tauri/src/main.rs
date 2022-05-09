#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use gtk::prelude::GtkWindowExt;
use tauri::Manager;

#[tauri::command]
fn hello(name: &str) -> Result<String, String> {
    if name.contains(' ') {
        Err("Name should not contains white-space".to_string())
    } else {
        Ok(format!("Hello, {}!", name))
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").expect("cannot get main window");
            #[cfg(debug_assertions)]
            window.open_devtools();
            let gtk_window = window
                .gtk_window()
                .expect("cannot get gtk window from main window");
            gtk_window.stick();
            gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop); // Make window a background window.
            gtk_window.set_skip_pager_hint(true); // make it non-searchable
            gtk_window.set_skip_taskbar_hint(true); // do not show it in taskbar
            gtk_window.set_keep_below(true); // draw below all regular windows
            gtk_window.set_decorated(false); // remove window border and title bar
            gtk_window.set_accept_focus(true); // make it non-selectable
            gtk_window.set_resizable(false); // prevent window from resizing
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
