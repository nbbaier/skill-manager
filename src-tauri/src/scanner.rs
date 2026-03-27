use crate::agents::Agent;
use crate::parser::{parse_skill_file, ParsedSkill};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredSkill {
    pub dir_name: String,
    pub path: PathBuf,
    pub agent_id: String,
    pub agent_name: String,
    pub parsed: ParsedSkill,
    pub is_symlink: bool,
    pub canonical_path: Option<PathBuf>,
}

pub fn scan_agent_skills(agent: &Agent) -> Vec<DiscoveredSkill> {
    let mut skills = Vec::new();

    let dir = &agent.global_path;
    if !dir.exists() || !dir.is_dir() {
        return skills;
    }

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return skills,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let skill_md = path.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let parsed = match parse_skill_file(&skill_md) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let dir_name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        let is_symlink = std::fs::symlink_metadata(&path)
            .map(|m| m.file_type().is_symlink())
            .unwrap_or(false);

        let canonical_path = if is_symlink {
            std::fs::canonicalize(&path).ok()
        } else {
            None
        };

        skills.push(DiscoveredSkill {
            dir_name,
            path: path.clone(),
            agent_id: agent.id.clone(),
            agent_name: agent.name.clone(),
            parsed,
            is_symlink,
            canonical_path,
        });
    }

    skills
}

pub fn scan_all_agents(agents: &[Agent]) -> Vec<DiscoveredSkill> {
    agents.iter().flat_map(scan_agent_skills).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::Agent;
    use std::fs;
    use tempfile::TempDir;

    fn make_agent(dir: &std::path::Path) -> Agent {
        Agent {
            id: "test-agent".to_string(),
            name: "Test Agent".to_string(),
            global_path: dir.to_path_buf(),
        }
    }

    #[test]
    fn test_scan_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let agent = make_agent(tmp.path());
        let skills = scan_agent_skills(&agent);
        assert!(skills.is_empty());
    }

    #[test]
    fn test_scan_nonexistent_dir() {
        let agent = Agent {
            id: "test".to_string(),
            name: "Test".to_string(),
            global_path: PathBuf::from("/nonexistent/path/skills"),
        };
        let skills = scan_agent_skills(&agent);
        assert!(skills.is_empty());
    }

    #[test]
    fn test_scan_discovers_skill() {
        let tmp = TempDir::new().unwrap();
        let skill_dir = tmp.path().join("my-skill");
        fs::create_dir(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: My Skill\ndescription: A great skill\n---\n# My Skill\nDoes things.",
        )
        .unwrap();

        let agent = make_agent(tmp.path());
        let skills = scan_agent_skills(&agent);
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].dir_name, "my-skill");
        assert_eq!(
            skills[0].parsed.frontmatter.name.as_deref(),
            Some("My Skill")
        );
        assert_eq!(skills[0].agent_id, "test-agent");
        assert!(!skills[0].is_symlink);
    }

    #[test]
    fn test_scan_ignores_dir_without_skill_md() {
        let tmp = TempDir::new().unwrap();
        let not_a_skill = tmp.path().join("not-a-skill");
        fs::create_dir(&not_a_skill).unwrap();
        fs::write(not_a_skill.join("README.md"), "Not a skill").unwrap();

        let agent = make_agent(tmp.path());
        let skills = scan_agent_skills(&agent);
        assert!(skills.is_empty());
    }

    #[test]
    fn test_scan_multiple_skills() {
        let tmp = TempDir::new().unwrap();
        for name in &["skill-a", "skill-b", "skill-c"] {
            let dir = tmp.path().join(name);
            fs::create_dir(&dir).unwrap();
            fs::write(
                dir.join("SKILL.md"),
                format!("---\nname: {}\n---\nBody.", name),
            )
            .unwrap();
        }

        let agent = make_agent(tmp.path());
        let skills = scan_agent_skills(&agent);
        assert_eq!(skills.len(), 3);
    }

    #[test]
    fn test_scan_all_agents_combines() {
        let tmp1 = TempDir::new().unwrap();
        let tmp2 = TempDir::new().unwrap();

        let dir1 = tmp1.path().join("skill-1");
        fs::create_dir(&dir1).unwrap();
        fs::write(dir1.join("SKILL.md"), "---\nname: S1\n---\n").unwrap();

        let dir2 = tmp2.path().join("skill-2");
        fs::create_dir(&dir2).unwrap();
        fs::write(dir2.join("SKILL.md"), "---\nname: S2\n---\n").unwrap();

        let agents = vec![
            Agent {
                id: "a1".to_string(),
                name: "Agent 1".to_string(),
                global_path: tmp1.path().to_path_buf(),
            },
            Agent {
                id: "a2".to_string(),
                name: "Agent 2".to_string(),
                global_path: tmp2.path().to_path_buf(),
            },
        ];

        let skills = scan_all_agents(&agents);
        assert_eq!(skills.len(), 2);
        assert!(skills.iter().any(|s| s.agent_id == "a1"));
        assert!(skills.iter().any(|s| s.agent_id == "a2"));
    }

    #[test]
    fn test_scan_detects_symlink() {
        let tmp = TempDir::new().unwrap();
        let canonical = tmp.path().join("canonical-skill");
        fs::create_dir(&canonical).unwrap();
        fs::write(
            canonical.join("SKILL.md"),
            "---\nname: Linked\n---\nBody.",
        )
        .unwrap();

        let skills_dir = tmp.path().join("agent-skills");
        fs::create_dir(&skills_dir).unwrap();

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&canonical, skills_dir.join("linked-skill")).unwrap();
            let agent = make_agent(&skills_dir);
            let skills = scan_agent_skills(&agent);
            assert_eq!(skills.len(), 1);
            assert!(skills[0].is_symlink);
            assert!(skills[0].canonical_path.is_some());
        }
    }
}
