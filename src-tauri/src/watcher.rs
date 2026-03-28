use crate::agents::get_agents;
use log::{info, warn};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::sync::mpsc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

const DEBOUNCE_DURATION: Duration = Duration::from_secs(2);

/// Starts file watchers on all agent skill directories.
/// Emits a "skills-changed" event to the frontend when changes are detected.
/// Returns a handle that keeps the watcher alive; dropping it stops watching.
pub fn start_watchers(
    app_handle: AppHandle,
) -> Option<notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>> {
    let agents = get_agents();

    let (tx, rx) = mpsc::channel();

    let mut debouncer = match new_debouncer(DEBOUNCE_DURATION, tx) {
        Ok(d) => d,
        Err(e) => {
            warn!("Failed to create file watcher: {}", e);
            return None;
        }
    };

    let mut watched_count = 0;
    for agent in &agents {
        let path = &agent.global_path;
        if path.exists() && path.is_dir() {
            if let Err(e) = debouncer
                .watcher()
                .watch(path, notify::RecursiveMode::Recursive)
            {
                warn!(
                    "Failed to watch directory for {}: {} ({})",
                    agent.name,
                    path.display(),
                    e
                );
            } else {
                info!("Watching skill directory for {}: {}", agent.name, path.display());
                watched_count += 1;
            }
        }
    }

    info!("File watchers established on {} directories", watched_count);

    // Spawn a thread to consume debounced events and emit to frontend
    std::thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(Ok(events)) => {
                    // Only emit if there are meaningful events (not just access)
                    let has_changes = events
                        .iter()
                        .any(|e| matches!(e.kind, DebouncedEventKind::Any | DebouncedEventKind::AnyContinuous));

                    if has_changes {
                        info!("File system changes detected, notifying frontend");
                        if let Err(e) = app_handle.emit("skills-changed", ()) {
                            warn!("Failed to emit skills-changed event: {}", e);
                        }
                    }
                }
                Ok(Err(errors)) => {
                    warn!("File watcher errors: {:?}", errors);
                }
                Err(_) => {
                    // Channel closed, watcher was dropped — exit thread
                    info!("File watcher channel closed, stopping event listener");
                    break;
                }
            }
        }
    });

    Some(debouncer)
}
