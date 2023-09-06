use std::sync::{Arc, Mutex};
use tauri::async_runtime::spawn;
use tauri::menu::MenuBuilder;
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager, RunEvent, Runtime};

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)
                .unwrap();
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Error while building the application");

    let menu_list = Arc::new(Mutex::new(Vec::new()));
    app.manage(menu_list.clone());

    let handle = app.handle().clone();
    app.listen_global("refresh_menu", move |_| {
        let menu = MenuBuilder::new(&handle)
            .text("hello world")
            .separator()
            .text("hello world, again")
            .build()
            .unwrap();

        // never drop a single menu
        menu_list.lock().unwrap().push(menu.clone());
        let tray = handle.tray_by_id("tray").unwrap();
        tray.set_menu(Some(menu)).unwrap();
    });

    // 1 second timer to update the menu
    let handle = app.handle().clone();
    spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            handle.trigger_global("refresh_menu", None);
        }
    });

    app.run(process_application_event);
}

pub fn process_application_event<R: Runtime>(_app: &AppHandle<R>, event: RunEvent) {
    match event {
        _ => {}
    }
}
