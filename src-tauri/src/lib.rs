use std::sync::Mutex;
use tauri::menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, WebviewUrl};
use tauri_plugin_autostart::ManagerExt;

const APP_URL: &str = "https://internal.resikcemerlang.com";

struct AppState {
    autostart_menu_item: Mutex<CheckMenuItem<tauri::Wry>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .setup(|app| {
            // Window utama: muat web app langsung
            let url: tauri::Url = APP_URL.parse().expect("URL aplikasi tidak valid");
            tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Resik Internal")
                .inner_size(1280.0, 800.0)
                .min_inner_size(800.0, 600.0)
                .center()
                .build()?;

            // Tray menu ala WhatsApp Desktop
            let show_item = MenuItem::with_id(app, "show", "Buka Resik Internal", true, None::<&str>)?;
            let autostart_item = CheckMenuItem::with_id(
                app,
                "autostart",
                "Buka otomatis saat login",
                true,
                app.autolaunch().is_enabled().unwrap_or(false),
                None::<&str>,
            )?;
            let quit_item = MenuItem::with_id(app, "quit", "Keluar", true, None::<&str>)?;

            let menu = Menu::new(app)?;
            menu.append(&show_item)?;
            menu.append(&autostart_item)?;
            menu.append(&PredefinedMenuItem::separator(app)?)?;
            menu.append(&quit_item)?;

            app.manage(AppState {
                autostart_menu_item: Mutex::new(autostart_item.clone()),
            });

            let icon = match app.default_window_icon() {
                Some(icon) => icon.clone(),
                None => tauri::image::Image::new_owned(vec![0; 32 * 32 * 4], 32, 32),
            };

            TrayIconBuilder::with_id("main-tray")
                .tooltip("Resik Internal")
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    let autolaunch = app.autolaunch();
                    match event.id().as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "autostart" => {
                            let enabled = autolaunch.is_enabled().unwrap_or(false);
                            let result = if enabled {
                                autolaunch.disable()
                            } else {
                                autolaunch.enable()
                            };
                            if let Err(err) = result {
                                eprintln!("autostart error: {err}");
                            }
                            // Sinkronkan centang pada menu
                            let state = app.state::<AppState>();
                            if let Ok(item) = state.autostart_menu_item.lock() {
                                let _ = item.set_checked(!enabled);
                            };
                        }
                        "quit" => app.exit(0),
                        _ => {}
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Tutup = sembunyikan ke tray, app tetap jalan (keep running)
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("gagal menjalankan aplikasi Resik Internal");
}