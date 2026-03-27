<script lang="ts">
	import './layout.css';

	let { children } = $props();
</script>

<div class="h-screen w-screen overflow-hidden antialiased">
	{@render children()}
</div>
