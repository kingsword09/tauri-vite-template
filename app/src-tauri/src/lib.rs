use tauri::{Manager, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
fn show_main_window(window: tauri::Window) {
    window.get_webview_window("main").unwrap().show().unwrap();
}

#[tauri::command]
fn set_window_theme(window: tauri::Window, theme: Option<&str>) {
    let webview_window = window.get_webview_window("main").unwrap();
    set_macos_window_theme(webview_window, theme);
}

fn set_macos_window_theme(window: tauri::WebviewWindow, theme: Option<&str>) {
    #[cfg(target_os = "macos")]
    {
        use objc2_app_kit::{NSAppearance, NSAppearanceNameAqua};
        use objc2_foundation::{ns_string, NSArray, NSString};

        // @see: https://github.com/zng-ui/zng/blob/d004475ed46bb0ffbd4a048eaa80803b2244ad6f/crates/zng-view/src/config/macos.rs
        let appearance = unsafe { NSAppearance::currentDrawingAppearance() };
        fn dark_appearance_name() -> &'static NSString {
            // Don't use the static `NSAppearanceNameDarkAqua` to allow linking on macOS < 10.14
            ns_string!("NSAppearanceNameDarkAqua")
        }
        let best_match = appearance.bestMatchFromAppearancesWithNames(&NSArray::from_slice(&[
            unsafe { NSAppearanceNameAqua },
            dark_appearance_name(),
        ]));
        let theme = match theme {
            Some(theme) => theme,
            None => {
                if let Some(best_match) = best_match {
                    if *best_match == *dark_appearance_name() {
                        "dark"
                    } else {
                        "light"
                    }
                } else {
                    "light"
                }
            }
        };

        unsafe {
            use cocoa::appkit::{NSColor, NSWindow};
            use cocoa::base::{id, nil};

            let ns_window = window.ns_window().unwrap() as id;
            let bg_color = match theme {
                "light" => NSColor::colorWithRed_green_blue_alpha_(
                    nil,
                    255.0 / 255.0, // R: 255
                    255.0 / 255.0, // G: 255
                    255.0 / 255.0, // B: 255
                    1.0,           // A: 1.0
                ),
                _ => NSColor::colorWithRed_green_blue_alpha_(
                    nil,
                    9.0 / 255.0,  // R: 9
                    9.0 / 255.0,  // G: 9
                    11.0 / 255.0, // B: 11
                    1.0,          // A: 1.0
                ),
            };
            ns_window.setBackgroundColor_(bg_color);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Used to initialize macOS windows, following the system theme.
            #[cfg(target_os = "macos")]
            {
                let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default())
                    .title("Tauri Vite")
                    .inner_size(800.0, 600.0);

                let win_builder = win_builder.title_bar_style(TitleBarStyle::Transparent);

                let webview_window = win_builder.build().unwrap();
                webview_window.hide()?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![show_main_window, set_window_theme])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
