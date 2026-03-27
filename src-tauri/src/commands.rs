use crate::agents::{get_agents, Agent};
use crate::scanner::{scan_all_agents, DiscoveredSkill};

#[tauri::command]
pub fn list_agents() -> Vec<Agent> {
    get_agents()
}

#[tauri::command]
pub fn scan_skills() -> Vec<DiscoveredSkill> {
    let agents = get_agents();
    scan_all_agents(&agents)
}
