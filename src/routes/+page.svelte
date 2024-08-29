<script lang="ts">
	import TablesTab from '$lib/components/TablesTab.svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Resizable from '$lib/components/ui/resizable';
	import DatasourcesTab from '$lib/components/DatasourcesTab.svelte';
	import QueryTab from '$lib/components/QueryTab.svelte';
	import Icon from '@iconify/svelte';
	import QueryHistory from '$lib/components/QueryHistory.svelte';
</script>

<Resizable.PaneGroup direction="horizontal" class="h-screen items-stretch rounded-lg border">
	<Resizable.Pane defaultSize={20} class="h-screen">
		<Collapsible.Root open>
			<Collapsible.Trigger asChild let:builder>
				<Button builders={[builder]} variant="outline" class="flex w-full justify-start gap-2">
					<Icon icon="carbon:cube" width={18} height={18} />
					Entities
					<Icon icon="carbon:chevron-sort" class="ml-auto" width={22} height={22} />
				</Button>
			</Collapsible.Trigger>
			<Collapsible.Content>
				<Tabs.Root value="table" class="ml-2 mt-2">
					<Tabs.List>
						<Tabs.Trigger value="table">Tables</Tabs.Trigger>
						<Tabs.Trigger value="datasource">Datasources</Tabs.Trigger>
					</Tabs.List>
					<Tabs.Content value="table" class="h-full"><TablesTab /></Tabs.Content>
					<Tabs.Content value="datasource" class="h-full"><DatasourcesTab /></Tabs.Content>
				</Tabs.Root>
			</Collapsible.Content>
		</Collapsible.Root>

		<Collapsible.Root open>
			<Collapsible.Trigger asChild let:builder>
				<Button builders={[builder]} variant="outline" class="flex w-full justify-start gap-2">
					<Icon icon="carbon:catalog" width={18} height={18} />
					Query history
					<Icon icon="carbon:chevron-sort" class="ml-auto" width={22} height={22} />
				</Button>
			</Collapsible.Trigger>
			<Collapsible.Content class="ml-2 mt-2">
				<QueryHistory />
			</Collapsible.Content>
		</Collapsible.Root>
	</Resizable.Pane>
	<Resizable.Handle />
	<Resizable.Pane defaultSize={80}>
		<QueryTab />
	</Resizable.Pane>
</Resizable.PaneGroup>
