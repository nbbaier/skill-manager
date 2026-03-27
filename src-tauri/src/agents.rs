use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub global_path: PathBuf,
}

impl Agent {
    fn new(id: &str, name: &str, relative_path: &str) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Self {
            id: id.to_string(),
            name: name.to_string(),
            global_path: home.join(relative_path),
        }
    }
}

pub fn get_agents() -> Vec<Agent> {
    vec![
        Agent::new("claude-code", "Claude Code", ".claude/skills"),
        Agent::new("codex", "Codex", ".codex/skills"),
        Agent::new("cursor", "Cursor", ".cursor/skills"),
        Agent::new("gemini-cli", "Gemini CLI", ".gemini/skills"),
        Agent::new("github-copilot", "GitHub Copilot", ".copilot/skills"),
        Agent::new("amp", "Amp", ".config/agents/skills"),
        Agent::new("droid", "Droid", ".factory/skills"),
        Agent::new("pi", "Pi", ".pi/agent/skills"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_count() {
        let agents = get_agents();
        assert_eq!(agents.len(), 8);
    }

    #[test]
    fn test_agent_ids_unique() {
        let agents = get_agents();
        let mut ids: Vec<&str> = agents.iter().map(|a| a.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), agents.len());
    }

    #[test]
    fn test_agent_paths_end_with_skills() {
        let agents = get_agents();
        for agent in &agents {
            assert!(
                agent.global_path.ends_with("skills"),
                "Agent {} path should end with 'skills': {:?}",
                agent.name,
                agent.global_path
            );
        }
    }

    #[test]
    fn test_known_agents_present() {
        let agents = get_agents();
        let names: Vec<&str> = agents.iter().map(|a| a.name.as_str()).collect();
        assert!(names.contains(&"Claude Code"));
        assert!(names.contains(&"Codex"));
        assert!(names.contains(&"Cursor"));
        assert!(names.contains(&"Gemini CLI"));
        assert!(names.contains(&"GitHub Copilot"));
        assert!(names.contains(&"Amp"));
        assert!(names.contains(&"Droid"));
        assert!(names.contains(&"Pi"));
    }

    #[test]
    fn test_claude_code_path() {
        let agents = get_agents();
        let claude = agents.iter().find(|a| a.id == "claude-code").unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(claude.global_path, home.join(".claude/skills"));
    }

    #[test]
    fn test_amp_path() {
        let agents = get_agents();
        let amp = agents.iter().find(|a| a.id == "amp").unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(amp.global_path, home.join(".config/agents/skills"));
    }
}
