use tauri::menu::{MenuBuilder, MenuItem, SubmenuBuilder};
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let about =
                MenuItem::with_id(app, "about", "About SVG Path Editor", true, None::<&str>)?;
            let quit = MenuItem::with_id(
                app,
                "quit",
                "Quit SVG Path Editor",
                true,
                Some("CmdOrCtrl+Q"),
            )?;
            let app_menu = SubmenuBuilder::new(app, "SVG Path Editor")
                .item(&about)
                .separator()
                .item(&quit)
                .build()?;

            let new_path =
                MenuItem::with_id(app, "new-path", "New Path", true, Some("CmdOrCtrl+N"))?;
            let open_paths = MenuItem::with_id(
                app,
                "open-paths",
                "Open Saved Path",
                true,
                Some("CmdOrCtrl+O"),
            )?;
            let save_path =
                MenuItem::with_id(app, "save-path", "Save Path", true, Some("CmdOrCtrl+S"))?;
            let export_svg =
                MenuItem::with_id(app, "export-svg", "Export SVG", true, Some("CmdOrCtrl+E"))?;
            let file_menu = SubmenuBuilder::new(app, "File")
                .item(&new_path)
                .separator()
                .item(&open_paths)
                .item(&save_path)
                .item(&export_svg)
                .build()?;

            let undo = MenuItem::with_id(app, "undo", "Undo", true, Some("CmdOrCtrl+Z"))?;
            let redo = MenuItem::with_id(app, "redo", "Redo", true, Some("CmdOrCtrl+Shift+Z"))?;
            let delete_selection = MenuItem::with_id(
                app,
                "delete-selection",
                "Delete Selection",
                true,
                None::<&str>,
            )?;
            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .item(&undo)
                .item(&redo)
                .separator()
                .copy()
                .cut()
                .paste()
                .select_all()
                .separator()
                .item(&delete_selection)
                .build()?;

            let zoom_in = MenuItem::with_id(app, "zoom-in", "Zoom In", true, Some("CmdOrCtrl+="))?;
            let zoom_out =
                MenuItem::with_id(app, "zoom-out", "Zoom Out", true, Some("CmdOrCtrl+-"))?;
            let zoom_fit =
                MenuItem::with_id(app, "zoom-fit", "Zoom to Fit", true, Some("CmdOrCtrl+0"))?;
            let toggle_sidebar = MenuItem::with_id(
                app,
                "toggle-sidebar",
                "Toggle Sidebar",
                true,
                Some("CmdOrCtrl+B"),
            )?;
            let view_menu = SubmenuBuilder::new(app, "View")
                .item(&zoom_in)
                .item(&zoom_out)
                .item(&zoom_fit)
                .separator()
                .item(&toggle_sidebar)
                .separator()
                .fullscreen()
                .build()?;

            let window_menu = SubmenuBuilder::new(app, "Window")
                .minimize()
                .separator()
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&app_menu, &file_menu, &edit_menu, &view_menu, &window_menu])
                .build()?;
            app.set_menu(menu)?;

            app.on_menu_event(|app_handle, event| match event.id().0.as_str() {
                "quit" => app_handle.exit(0),
                "new-path" | "open-paths" | "save-path" | "export-svg" | "undo" | "redo"
                | "delete-selection" | "zoom-in" | "zoom-out" | "zoom-fit" | "toggle-sidebar" => {
                    let _ = app_handle.emit("native-command", event.id().0.as_str());
                }
                _ => {}
            });

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
