<script lang="ts">
	import * as Resizable from '$lib/components/ui/resizable/index.js';
</script>

<Resizable.PaneGroup direction="horizontal" class="h-full">
	<!-- Agent Sidebar -->
	<Resizable.Pane defaultSize={20} minSize={15} maxSize={30}>
		<div class="flex h-full flex-col border-r border-border">
			<div class="border-b border-border px-4 py-3">
				<h2 class="text-sm font-semibold tracking-tight">Agents</h2>
			</div>
			<div class="flex-1 p-4">
				<p class="text-sm text-muted-foreground">Agent list placeholder</p>
			</div>
		</div>
	</Resizable.Pane>

	<Resizable.Handle />

	<!-- Skill List Sidebar -->
	<Resizable.Pane defaultSize={25} minSize={20} maxSize={35}>
		<div class="flex h-full flex-col border-r border-border">
			<div class="border-b border-border px-4 py-3">
				<h2 class="text-sm font-semibold tracking-tight">Skills</h2>
			</div>
			<div class="flex-1 p-4">
				<p class="text-sm text-muted-foreground">Skill list placeholder</p>
			</div>
		</div>
	</Resizable.Pane>

	<Resizable.Handle />

	<!-- Main Content Panel -->
	<Resizable.Pane defaultSize={55}>
		<div class="flex h-full flex-col">
			<div class="border-b border-border px-4 py-3">
				<h2 class="text-sm font-semibold tracking-tight">Detail</h2>
			</div>
			<div class="flex-1 p-4">
				<p class="text-sm text-muted-foreground">Select a skill to view details</p>
			</div>
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
