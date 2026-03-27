<script lang="ts">
	import { appState, selectAgent } from '$lib/stores.svelte.js';

	const totalSkillCount = $derived(appState.skills.length);

	function skillCountForAgent(agentId: string): number {
		return appState.skills.filter((s) => s.agent_id === agentId).length;
	}
</script>

<div class="flex h-full flex-col border-r border-border">
	<div class="border-b border-border px-4 py-3">
		<h2 class="text-sm font-semibold tracking-tight">Agents</h2>
	</div>
	<div class="flex-1 overflow-y-auto">
		<ul class="py-1">
			<li>
				<button
					class="flex w-full items-center justify-between px-4 py-2 text-left text-sm transition-colors hover:bg-accent {appState.selectedAgentId === null ? 'bg-accent font-medium' : ''}"
					onclick={() => selectAgent(null)}
				>
					<span>All Skills</span>
					<span class="text-xs text-muted-foreground">{totalSkillCount}</span>
				</button>
			</li>
			<li><hr class="mx-4 my-1 border-border" /></li>
			{#each appState.agents as agent (agent.id)}
				{@const count = skillCountForAgent(agent.id)}
				<li>
					<button
						class="flex w-full items-center justify-between px-4 py-2 text-left text-sm transition-colors hover:bg-accent {appState.selectedAgentId === agent.id ? 'bg-accent font-medium' : ''}"
						onclick={() => selectAgent(agent.id)}
					>
						<span>{agent.name}</span>
						<span class="text-xs text-muted-foreground">{count}</span>
					</button>
				</li>
			{/each}
		</ul>
	</div>
</div>
