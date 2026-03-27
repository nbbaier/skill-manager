import { invoke } from '@tauri-apps/api/core';
import type { Agent, DiscoveredSkill } from './types.js';

export const appState = $state({
	agents: [] as Agent[],
	skills: [] as DiscoveredSkill[],
	selectedAgentId: null as string | null, // null means "All Skills"
	selectedSkillPath: null as string | null,
	loading: true
});

export async function loadData() {
	appState.loading = true;
	try {
		const [agents, skills] = await Promise.all([
			invoke<Agent[]>('list_agents'),
			invoke<DiscoveredSkill[]>('scan_skills')
		]);
		appState.agents = agents;
		appState.skills = skills;
	} catch (e) {
		console.error('Failed to load data:', e);
	} finally {
		appState.loading = false;
	}
}

export function selectAgent(agentId: string | null) {
	appState.selectedAgentId = agentId;
	appState.selectedSkillPath = null;
}

export function selectSkill(path: string) {
	appState.selectedSkillPath = path;
}

export function getFilteredSkills(): DiscoveredSkill[] {
	if (appState.selectedAgentId === null) {
		return appState.skills;
	}
	return appState.skills.filter((s) => s.agent_id === appState.selectedAgentId);
}

export function getSelectedSkill(): DiscoveredSkill | undefined {
	return appState.skills.find((s) => s.path === appState.selectedSkillPath);
}

export function getSkillAgents(skill: DiscoveredSkill): string[] {
	return appState.skills
		.filter((s) => s.dir_name === skill.dir_name)
		.map((s) => s.agent_name);
}
