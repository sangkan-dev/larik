use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager,
};

const MENU_QUIT: &str = "app_quit";
const MENU_RELOAD: &str = "app_reload";
const MENU_DEVTOOLS: &str = "app_devtools";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::default().build());

    #[cfg(debug_assertions)]
    let builder = builder.plugin(
        tauri_plugin_mcp_bridge::Builder::new()
            .bind_address("127.0.0.1")
            .build(),
    );

    builder
        .setup(|app| {
            let handle = app.handle();
            let reload = MenuItemBuilder::with_id(MENU_RELOAD, "Reload Window")
                .accelerator("Ctrl+R")
                .build(handle)?;
            #[cfg(debug_assertions)]
            let devtools = MenuItemBuilder::with_id(MENU_DEVTOOLS, "Toggle DevTools")
                .accelerator("F12")
                .build(handle)?;
            let quit = MenuItemBuilder::with_id(MENU_QUIT, "Quit Larik")
                .accelerator("Ctrl+Q")
                .build(handle)?;
            let app_menu = SubmenuBuilder::new(handle, "Larik").item(&reload);
            #[cfg(debug_assertions)]
            let app_menu = app_menu.item(&devtools);
            let app_menu = app_menu.separator().item(&quit).build()?;
            let edit_menu = SubmenuBuilder::new(handle, "Edit")
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?;
            let menu = MenuBuilder::new(handle)
                .item(&app_menu)
                .item(&edit_menu)
                .build()?;

            app.set_menu(menu)?;
            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            MENU_QUIT => app.exit(0),
            MENU_RELOAD => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.location.reload()");
                }
            }
            MENU_DEVTOOLS =>
            {
                #[cfg(debug_assertions)]
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_devtools_open() {
                        window.close_devtools();
                    } else {
                        window.open_devtools();
                    }
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
