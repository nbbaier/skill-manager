<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { marked } from 'marked';
	import { getSelectedSkill, getSkillAgents } from '$lib/stores.svelte.js';
	import type { FileEntry } from '$lib/types.js';
	import ChevronRight from '@lucide/svelte/icons/chevron-right';
	import FolderIcon from '@lucide/svelte/icons/folder';
	import FileIcon from '@lucide/svelte/icons/file';

	const skill = $derived(getSelectedSkill());
	const agents = $derived(skill ? getSkillAgents(skill) : []);

	let fileTree = $state<FileEntry[]>([]);
	let fileTreeLoading = $state(false);
	let fileTreeError = $state<string | null>(null);

	let renderedBody = $derived.by(() => {
		if (!skill?.parsed.body.trim()) return '';
		return marked.parse(skill.parsed.body, { async: false }) as string;
	});

	let metadataEntries = $derived.by(() => {
		if (!skill?.parsed.frontmatter.metadata) return [];
		return Object.entries(skill.parsed.frontmatter.metadata);
	});

	$effect(() => {
		if (skill) {
			loadFileTree(skill.canonical_path ?? skill.path);
		} else {
			fileTree = [];
		}
	});

	async function loadFileTree(path: string) {
		fileTreeLoading = true;
		fileTreeError = null;
		try {
			fileTree = await invoke<FileEntry[]>('list_skill_files', { path });
		} catch (e) {
			fileTreeError = e instanceof Error ? e.message : String(e);
			fileTree = [];
		} finally {
			fileTreeLoading = false;
		}
	}
</script>

{#snippet fileTreeNode(entries: FileEntry[], depth: number)}
	{#each entries as entry}
		{#if entry.is_dir}
			<details class="group" open={depth < 2}>
				<summary
					class="flex cursor-pointer items-center gap-1 rounded px-1 py-0.5 text-xs hover:bg-muted"
					style="padding-left: {depth * 12 + 4}px"
				>
					<ChevronRight
						class="h-3 w-3 shrink-0 transition-transform group-open:rotate-90"
					/>
					<FolderIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
					<span class="truncate">{entry.name}</span>
				</summary>
				{@render fileTreeNode(entry.children, depth + 1)}
			</details>
		{:else}
			<div
				class="flex items-center gap-1 rounded px-1 py-0.5 text-xs hover:bg-muted"
				style="padding-left: {depth * 12 + 4 + 16}px"
			>
				<FileIcon class="h-3.5 w-3.5 shrink-0 text-muted-foreground" />
				<span class="truncate">{entry.name}</span>
			</div>
		{/if}
	{/each}
{/snippet}

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
							{#if metadataEntries.length > 0}
								<tr class="border-b border-border">
									<td class="px-3 py-2 font-medium text-muted-foreground">Metadata</td>
									<td class="px-3 py-2">
										<div class="space-y-1">
											{#each metadataEntries as [key, value]}
												<div class="text-xs">
													<span class="font-medium">{key}:</span>
													<span class="font-mono text-muted-foreground">
														{typeof value === 'object'
															? JSON.stringify(value)
															: String(value)}
													</span>
												</div>
											{/each}
										</div>
									</td>
								</tr>
							{/if}
						</tbody>
					</table>
				</div>

				<!-- Rendered markdown body -->
				{#if renderedBody}
					<div>
						<h4 class="mb-2 text-sm font-semibold text-muted-foreground">Instructions</h4>
						<div
							class="prose prose-sm dark:prose-invert max-w-none rounded-md border border-border p-4"
						>
							{@html renderedBody}
						</div>
					</div>
				{/if}

				<!-- File tree -->
				<div>
					<h4 class="mb-2 text-sm font-semibold text-muted-foreground">Files</h4>
					<div class="rounded-md border border-border p-2">
						{#if fileTreeLoading}
							<p class="px-2 py-1 text-xs text-muted-foreground">Loading...</p>
						{:else if fileTreeError}
							<p class="px-2 py-1 text-xs text-destructive">{fileTreeError}</p>
						{:else if fileTree.length === 0}
							<p class="px-2 py-1 text-xs text-muted-foreground">No files found</p>
						{:else}
							{@render fileTreeNode(fileTree, 0)}
						{/if}
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	:global(.prose h1) {
		font-size: 1.25rem;
		font-weight: 700;
		margin-top: 0;
		margin-bottom: 0.5rem;
	}
	:global(.prose h2) {
		font-size: 1.1rem;
		font-weight: 600;
		margin-top: 1rem;
		margin-bottom: 0.5rem;
	}
	:global(.prose h3) {
		font-size: 1rem;
		font-weight: 600;
		margin-top: 0.75rem;
		margin-bottom: 0.25rem;
	}
	:global(.prose p) {
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}
	:global(.prose ul) {
		list-style-type: disc;
		padding-left: 1.5rem;
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}
	:global(.prose ol) {
		list-style-type: decimal;
		padding-left: 1.5rem;
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}
	:global(.prose li) {
		margin-top: 0.25rem;
		margin-bottom: 0.25rem;
	}
	:global(.prose code) {
		font-size: 0.85em;
		background-color: var(--color-muted);
		padding: 0.15em 0.3em;
		border-radius: 0.25rem;
	}
	:global(.prose pre) {
		background-color: var(--color-muted);
		border: 1px solid var(--color-border);
		border-radius: 0.375rem;
		padding: 0.75rem;
		overflow-x: auto;
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
	}
	:global(.prose pre code) {
		background: none;
		padding: 0;
		font-size: 0.8em;
	}
	:global(.prose a) {
		color: var(--color-accent-foreground);
		text-decoration: underline;
	}
	:global(.prose blockquote) {
		border-left: 3px solid var(--color-border);
		padding-left: 0.75rem;
		margin-top: 0.5rem;
		margin-bottom: 0.5rem;
		color: var(--color-muted-foreground);
	}
</style>
