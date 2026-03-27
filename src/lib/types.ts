export interface Agent {
	id: string;
	name: string;
	global_path: string;
}

export interface SkillFrontmatter {
	name: string | null;
	description: string | null;
	license: string | null;
	compatibility: string[] | null;
	allowed_tools: string[] | null;
	metadata: Record<string, unknown> | null;
}

export interface ParsedSkill {
	frontmatter: SkillFrontmatter;
	body: string;
}

export interface DiscoveredSkill {
	dir_name: string;
	path: string;
	agent_id: string;
	agent_name: string;
	parsed: ParsedSkill;
	is_symlink: boolean;
	canonical_path: string | null;
}
