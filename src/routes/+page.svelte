<script lang="ts">
	import { onMount } from 'svelte';
	import * as Resizable from '$lib/components/ui/resizable/index.js';
	import AgentSidebar from '$lib/components/AgentSidebar.svelte';
	import SkillList from '$lib/components/SkillList.svelte';
	import SkillDetail from '$lib/components/SkillDetail.svelte';
	import { loadData } from '$lib/stores.svelte.js';

	onMount(() => {
		loadData();
	});
</script>

<Resizable.PaneGroup direction="horizontal" class="h-full">
	<!-- Agent Sidebar -->
	<Resizable.Pane defaultSize={20} minSize={15} maxSize={30}>
		<AgentSidebar />
	</Resizable.Pane>

	<Resizable.Handle />

	<!-- Skill List Sidebar -->
	<Resizable.Pane defaultSize={25} minSize={20} maxSize={35}>
		<SkillList />
	</Resizable.Pane>

	<Resizable.Handle />

	<!-- Main Content Panel -->
	<Resizable.Pane defaultSize={55}>
		<SkillDetail />
	</Resizable.Pane>
</Resizable.PaneGroup>
