<script lang="ts">
	import { appState, getFilteredSkills, getSkillAgents, selectSkill } from '$lib/stores.svelte.js';

	const filteredSkills = $derived(getFilteredSkills());
	const showAllSkills = $derived(appState.selectedAgentId === null);
</script>

<div class="flex h-full flex-col border-r border-border">
	<div class="border-b border-border px-4 py-3">
		<h2 class="text-sm font-semibold tracking-tight">
			{#if showAllSkills}
				All Skills
			{:else}
				{appState.agents.find((a) => a.id === appState.selectedAgentId)?.name ?? 'Skills'}
			{/if}
		</h2>
	</div>
	<div class="flex-1 overflow-y-auto">
		{#if appState.loading}
			<div class="p-4">
				<p class="text-sm text-muted-foreground">Scanning skills…</p>
			</div>
		{:else if filteredSkills.length === 0}
			<div class="p-4">
				<p class="text-sm text-muted-foreground">No skills found.</p>
			</div>
		{:else}
			<ul class="py-1">
				{#each filteredSkills as skill (skill.path)}
					{@const agents = showAllSkills ? getSkillAgents(skill) : []}
					<li>
						<button
							class="w-full px-4 py-2.5 text-left transition-colors hover:bg-accent {appState.selectedSkillPath === skill.path ? 'bg-accent' : ''}"
							onclick={() => selectSkill(skill.path)}
						>
							<div class="text-sm font-medium">
								{skill.parsed.frontmatter.name ?? skill.dir_name}
							</div>
							{#if skill.parsed.frontmatter.description}
								<div class="mt-0.5 line-clamp-2 text-xs text-muted-foreground">
									{skill.parsed.frontmatter.description}
								</div>
							{/if}
							{#if showAllSkills && agents.length > 0}
								<div class="mt-1 flex flex-wrap gap-1">
									{#each agents as agentName}
										<span
											class="inline-flex items-center rounded-sm bg-secondary px-1.5 py-0.5 text-[10px] font-medium text-secondary-foreground"
										>
											{agentName}
										</span>
									{/each}
								</div>
							{/if}
						</button>
					</li>
				{/each}
			</ul>
		{/if}
	</div>
</div>
