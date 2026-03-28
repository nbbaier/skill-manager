mod agents;
mod commands;
mod parser;
mod scanner;
mod watcher;

use tauri::Manager;

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

            // Start file watchers on all agent skill directories.
            // The watcher handle is stored in managed state so it lives
            // for the lifetime of the application and is cleaned up on shutdown.
            let watcher = watcher::start_watchers(app.handle().clone());
            app.manage(WatcherState { _watcher: watcher });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_agents,
            commands::scan_skills,
            commands::list_skill_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Holds the file watcher thread handle so it stays alive for the app's lifetime.
struct WatcherState {
    _watcher: Option<std::thread::JoinHandle<()>>,
}
