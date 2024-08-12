<script lang="ts">
	import TablesTab from '$lib/components/TablesTab.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Resizable from '$lib/components/ui/resizable';
	import DatasourcesTab from '$lib/components/DatasourcesTab.svelte';
</script>

<Resizable.PaneGroup direction="horizontal" class="h-full items-stretch rounded-lg border">
	<Resizable.Pane defaultSize={20}>
		<Tabs.Root value="account" class="ml-2 mt-2 h-full">
			<Tabs.List>
				<Tabs.Trigger value="table">Tables</Tabs.Trigger>
				<Tabs.Trigger value="datasource">Datasources</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="table"><TablesTab /></Tabs.Content>
			<Tabs.Content value="datasource"><DatasourcesTab /></Tabs.Content>
		</Tabs.Root>
	</Resizable.Pane>
	<Resizable.Handle />
	<Resizable.Pane defaultSize={80}>
		<div class="flex h-full items-center justify-center p-6">
			<span class="font-semibold">Content</span>
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
