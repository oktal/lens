<script lang="ts">
	import { type QueryStream } from './QueryStream.svelte';
	import * as Select from '$lib/components/ui/select';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import { Textarea } from './ui/textarea';
	import { Button } from './ui/button';
	import Icon from '@iconify/svelte';
	import { Label } from './ui/label';
	import Progress from './ui/progress/progress.svelte';

	let { stream } = $props<{ stream: QueryStream }>();

	let currentPage = $state(0);
	let rowsPerPage = $state(10);

	let start = $derived(currentPage * rowsPerPage);
	let end = $derived(Math.min(start + rowsPerPage, stream.rows.length));

	let slice = $derived(stream.rows.slice(start, end));
	let lastPage = $derived(Math.max(Math.ceil(stream.rows.length / rowsPerPage) - 1, 0));

	const rowsPerPageItems = [10, 25, 50, 100].map((x) => {
		return {
			label: `${x}`,
			value: x
		};
	});
</script>

<Tabs.Root value="table">
	<Tabs.List>
		<Tabs.Trigger value="table">Table</Tabs.Trigger>
		<Tabs.Trigger value="text">Text</Tabs.Trigger>
	</Tabs.List>
	<Tabs.Content value="table">
		{@render dataTable()}
	</Tabs.Content>
	<Tabs.Content value="text">
		{@render rawText()}
	</Tabs.Content>
</Tabs.Root>

{#snippet dataTable()}
	{#if stream.state === 'running'}
		<Progress value={undefined} />
	{/if}
	<Table.Root>
		<Table.Header>
			<Table.Row>
				{#each stream.columns as column}
					<Table.Head>{column}</Table.Head>
				{/each}
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#each slice as values}
				<Table.Row>
					{#each values as cell}
						<Table.Cell class="p-1 font-medium">{cell}</Table.Cell>
					{/each}
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>

	{@render pagination()}
{/snippet}

{#snippet pagination()}
	<div class="flex items-center justify-end space-x-4 py-4">
		<Label>Rows per page</Label>
		<Select.Root
			items={rowsPerPageItems}
			selected={rowsPerPageItems[0]}
			onSelectedChange={(v) => v && (rowsPerPage = v.value)}
		>
			<Select.Trigger class="w-[80px]">
				<Select.Value />
			</Select.Trigger>
			<Select.Content>
				{#each rowsPerPageItems as { value, label }}
					<Select.Item {value}>
						{label}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>

		<Label>{start + 1}-{end} of {stream.rows.length} ({currentPage}/{lastPage} page)</Label>

		{@render paginationItem({
			icon: 'material-symbols:first-page',
			tooltip: 'First page',
			disabled: currentPage == 0,
			action: () => (currentPage = 0)
		})}
		{@render paginationItem({
			icon: 'material-symbols:chevron-left',
			tooltip: 'Previous page',
			disabled: currentPage == 0,
			action: () => (currentPage -= 1)
		})}
		{@render paginationItem({
			icon: 'material-symbols:chevron-right',
			tooltip: 'Next page',
			disabled: currentPage == lastPage,
			action: () => (currentPage += 1)
		})}
		{@render paginationItem({
			icon: 'material-symbols:last-page',
			tooltip: 'Last page',
			disabled: currentPage == lastPage,
			action: () => (currentPage = lastPage)
		})}
	</div>
{/snippet}

{#snippet paginationItem({
	icon,
	tooltip,
	disabled,
	action
}: {
	icon: string;
	tooltip: string;
	disabled: boolean;
	action: any;
})}
	<Tooltip.Root>
		<Tooltip.Trigger asChild let:builder>
			<Button builders={[builder]} variant="ghost" size="icon" {disabled} on:click={action}>
				<Icon {icon} />
			</Button>
		</Tooltip.Trigger>
		<Tooltip.Content>
			<p>{tooltip}</p>
		</Tooltip.Content>
	</Tooltip.Root>
{/snippet}

{#snippet rawText()}
	{stream.columns}
	<Textarea
		readonly
		class="h-full"
		value={slice.map((row: string[]) => row.join(',')).join('\n')}
	/>
{/snippet}
