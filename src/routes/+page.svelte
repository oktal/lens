<script lang="ts">
	import TablesTab from '$lib/components/TablesTab.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Resizable from '$lib/components/ui/resizable';
	import DatasourcesTab from '$lib/components/DatasourcesTab.svelte';
	import QueryTab from '$lib/components/QueryTab.svelte';
</script>

<Resizable.PaneGroup direction="horizontal" class="items-stretch rounded-lg border">
	<Resizable.Pane defaultSize={20}>
		<Tabs.Root value="table" class="ml-2 mt-2 h-screen">
			<Tabs.List>
				<Tabs.Trigger value="table">Tables</Tabs.Trigger>
				<Tabs.Trigger value="datasource">Datasources</Tabs.Trigger>
			</Tabs.List>
			<Tabs.Content value="table" class="h-full"><TablesTab /></Tabs.Content>
			<Tabs.Content value="datasource" class="h-full"><DatasourcesTab /></Tabs.Content>
		</Tabs.Root>
	</Resizable.Pane>
	<Resizable.Handle />
	<Resizable.Pane defaultSize={80}>
		<QueryTab />
	</Resizable.Pane>
</Resizable.PaneGroup>
