use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillFrontmatter {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub compatibility: Option<Vec<String>>,
    #[serde(default, alias = "allowed-tools")]
    pub allowed_tools: Option<Vec<String>>,
    #[serde(default)]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSkill {
    pub frontmatter: SkillFrontmatter,
    pub body: String,
}

pub fn parse_skill_md(content: &str) -> ParsedSkill {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---\n") && !trimmed.starts_with("---\r\n") {
        return ParsedSkill {
            frontmatter: SkillFrontmatter::default(),
            body: content.to_string(),
        };
    }

    // Find closing ---
    let after_open = &trimmed[3..];
    if let Some(close_pos) = after_open.find("\n---") {
        let yaml_str = &after_open[..close_pos];
        let body_start = close_pos + 4; // "\n---"
        let body = after_open[body_start..].trim_start_matches(['\n', '\r']).to_string();

        let frontmatter: SkillFrontmatter =
            serde_yml::from_str(yaml_str).unwrap_or_default();

        ParsedSkill { frontmatter, body }
    } else {
        ParsedSkill {
            frontmatter: SkillFrontmatter::default(),
            body: content.to_string(),
        }
    }
}

pub fn parse_skill_file(path: &Path) -> Result<ParsedSkill, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;
    Ok(parse_skill_md(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_frontmatter() {
        let content = r#"---
name: Test Skill
description: A test skill for testing
license: MIT
compatibility:
  - claude-code
  - cursor
allowed-tools:
  - Bash
---
# Test Skill

This is the body of the skill.
"#;
        let parsed = parse_skill_md(content);
        assert_eq!(parsed.frontmatter.name.as_deref(), Some("Test Skill"));
        assert_eq!(
            parsed.frontmatter.description.as_deref(),
            Some("A test skill for testing")
        );
        assert_eq!(parsed.frontmatter.license.as_deref(), Some("MIT"));
        assert_eq!(
            parsed.frontmatter.compatibility,
            Some(vec!["claude-code".to_string(), "cursor".to_string()])
        );
        assert_eq!(
            parsed.frontmatter.allowed_tools,
            Some(vec!["Bash".to_string()])
        );
        assert!(parsed.body.contains("# Test Skill"));
        assert!(parsed.body.contains("This is the body"));
    }

    #[test]
    fn test_parse_minimal_frontmatter() {
        let content = "---\nname: Minimal\n---\nBody text here.";
        let parsed = parse_skill_md(content);
        assert_eq!(parsed.frontmatter.name.as_deref(), Some("Minimal"));
        assert!(parsed.frontmatter.description.is_none());
        assert!(parsed.body.contains("Body text here."));
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "# Just a markdown file\n\nNo frontmatter here.";
        let parsed = parse_skill_md(content);
        assert!(parsed.frontmatter.name.is_none());
        assert!(parsed.body.contains("Just a markdown file"));
    }

    #[test]
    fn test_parse_empty_body() {
        let content = "---\nname: No Body\ndescription: Has no body\n---\n";
        let parsed = parse_skill_md(content);
        assert_eq!(parsed.frontmatter.name.as_deref(), Some("No Body"));
        assert!(parsed.body.is_empty() || parsed.body.trim().is_empty());
    }

    #[test]
    fn test_parse_malformed_yaml() {
        let content = "---\n: invalid yaml [[[}\n---\nBody here.";
        let parsed = parse_skill_md(content);
        // Should fall back to defaults
        assert!(parsed.frontmatter.name.is_none());
        assert!(parsed.body.contains("Body here."));
    }

    #[test]
    fn test_parse_empty_string() {
        let parsed = parse_skill_md("");
        assert!(parsed.frontmatter.name.is_none());
    }
}
