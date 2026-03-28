use crate::agents::{get_agents, Agent};
use log::{info, warn};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

const DEBOUNCE_DURATION: Duration = Duration::from_secs(2);
const DIR_CHECK_INTERVAL: Duration = Duration::from_secs(30);

/// Try to add a recursive watch on an agent's skill directory.
/// Returns true if the watch was newly added.
fn try_watch(
    debouncer: &mut notify_debouncer_mini::Debouncer<notify::RecommendedWatcher>,
    agent: &Agent,
    watched: &mut HashSet<PathBuf>,
) -> bool {
    let path = &agent.global_path;
    if watched.contains(path) || !path.exists() || !path.is_dir() {
        return false;
    }
    match debouncer
        .watcher()
        .watch(path, notify::RecursiveMode::Recursive)
    {
        Ok(()) => {
            info!(
                "Watching skill directory for {}: {}",
                agent.name,
                path.display()
            );
            watched.insert(path.clone());
            true
        }
        Err(e) => {
            warn!(
                "Failed to watch directory for {}: {} ({})",
                agent.name,
                path.display(),
                e
            );
            false
        }
    }
}

/// Starts file watchers on all agent skill directories.
/// Emits a "skills-changed" event to the frontend when changes are detected.
/// Periodically checks for newly-created directories and adds watches for them.
/// Returns a thread handle that keeps the watcher alive; dropping it stops watching.
pub fn start_watchers(app_handle: AppHandle) -> Option<std::thread::JoinHandle<()>> {
    let agents = get_agents();

    let (tx, rx) = mpsc::channel();

    let mut debouncer = match new_debouncer(DEBOUNCE_DURATION, tx) {
        Ok(d) => d,
        Err(e) => {
            warn!("Failed to create file watcher: {}", e);
            return None;
        }
    };

    let mut watched = HashSet::new();
    for agent in &agents {
        try_watch(&mut debouncer, agent, &mut watched);
    }
    info!("File watchers established on {} directories", watched.len());

    let handle = std::thread::spawn(move || {
        // Keep debouncer alive inside this thread
        let _debouncer_guard = &mut debouncer;

        loop {
            match rx.recv_timeout(DIR_CHECK_INTERVAL) {
                Ok(Ok(events)) => {
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
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    // Periodically check for newly-created skill directories
                    for agent in &agents {
                        if try_watch(_debouncer_guard, agent, &mut watched) {
                            // A new directory appeared — notify frontend so it can refresh
                            if let Err(e) = app_handle.emit("skills-changed", ()) {
                                warn!("Failed to emit skills-changed event: {}", e);
                            }
                        }
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    info!("File watcher channel closed, stopping event listener");
                    break;
                }
            }
        }
    });

    Some(handle)
}
