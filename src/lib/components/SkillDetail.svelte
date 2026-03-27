<script lang="ts">
	import { getSelectedSkill, getSkillAgents } from '$lib/stores.svelte.js';

	const skill = $derived(getSelectedSkill());
	const agents = $derived(skill ? getSkillAgents(skill) : []);
</script>

<div class="flex h-full flex-col">
	<div class="border-b border-border px-4 py-3">
		<h2 class="text-sm font-semibold tracking-tight">Detail</h2>
	</div>
	<div class="flex-1 overflow-y-auto p-4">
		{#if !skill}
			<p class="text-sm text-muted-foreground">Select a skill to view details</p>
		{:else}
			<div class="space-y-4">
				<div>
					<h3 class="text-lg font-semibold">
						{skill.parsed.frontmatter.name ?? skill.dir_name}
					</h3>
					{#if skill.parsed.frontmatter.description}
						<p class="mt-1 text-sm text-muted-foreground">
							{skill.parsed.frontmatter.description}
						</p>
					{/if}
				</div>

				<!-- Metadata table -->
				<div class="rounded-md border border-border">
					<table class="w-full text-sm">
						<tbody>
							<tr class="border-b border-border">
								<td class="px-3 py-2 font-medium text-muted-foreground">Directory</td>
								<td class="px-3 py-2 font-mono text-xs">{skill.dir_name}</td>
							</tr>
							<tr class="border-b border-border">
								<td class="px-3 py-2 font-medium text-muted-foreground">Path</td>
								<td class="px-3 py-2 font-mono text-xs break-all">{skill.path}</td>
							</tr>
							{#if skill.parsed.frontmatter.license}
								<tr class="border-b border-border">
									<td class="px-3 py-2 font-medium text-muted-foreground">License</td>
									<td class="px-3 py-2">{skill.parsed.frontmatter.license}</td>
								</tr>
							{/if}
							{#if skill.is_symlink && skill.canonical_path}
								<tr class="border-b border-border">
									<td class="px-3 py-2 font-medium text-muted-foreground">Symlink</td>
									<td class="px-3 py-2 font-mono text-xs break-all"
										>{skill.canonical_path}</td
									>
								</tr>
							{/if}
							<tr class="border-b border-border">
								<td class="px-3 py-2 font-medium text-muted-foreground">Agents</td>
								<td class="px-3 py-2">
									<div class="flex flex-wrap gap-1">
										{#each agents as agentName}
											<span
												class="inline-flex items-center rounded-sm bg-secondary px-1.5 py-0.5 text-xs font-medium text-secondary-foreground"
											>
												{agentName}
											</span>
										{/each}
									</div>
								</td>
							</tr>
							{#if skill.parsed.frontmatter.compatibility}
								<tr class="border-b border-border">
									<td class="px-3 py-2 font-medium text-muted-foreground"
										>Compatibility</td
									>
									<td class="px-3 py-2">
										<div class="flex flex-wrap gap-1">
											{#each skill.parsed.frontmatter.compatibility as compat}
												<span
													class="inline-flex items-center rounded-sm bg-secondary px-1.5 py-0.5 text-xs font-medium text-secondary-foreground"
												>
													{compat}
												</span>
											{/each}
										</div>
									</td>
								</tr>
							{/if}
							{#if skill.parsed.frontmatter.allowed_tools}
								<tr class="border-b border-border">
									<td class="px-3 py-2 font-medium text-muted-foreground"
										>Allowed Tools</td
									>
									<td class="px-3 py-2">
										<div class="flex flex-wrap gap-1">
											{#each skill.parsed.frontmatter.allowed_tools as tool}
												<span
													class="inline-flex items-center rounded-sm bg-secondary px-1.5 py-0.5 text-xs font-medium text-secondary-foreground"
												>
													{tool}
												</span>
											{/each}
										</div>
									</td>
								</tr>
							{/if}
						</tbody>
					</table>
				</div>

				<!-- Markdown body -->
				{#if skill.parsed.body.trim()}
					<div>
						<h4 class="mb-2 text-sm font-semibold text-muted-foreground">Instructions</h4>
						<pre
							class="whitespace-pre-wrap rounded-md border border-border bg-muted/50 p-3 font-mono text-xs leading-relaxed">{skill.parsed.body}</pre>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
