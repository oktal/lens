<script>
	import { Toaster } from 'svelte-sonner';
	import '../app.css';
	import Sidebar from '$lib/components/Sidebar.svelte';
	import { ModeWatcher } from 'mode-watcher';
	import { startListeningForDragAndDropEvents } from '$lib/dropevent';

	startListeningForDragAndDropEvents();
</script>

<Toaster />
<ModeWatcher />

<div class="flex flex-row">
	<Sidebar />
	<slot />
</div>
