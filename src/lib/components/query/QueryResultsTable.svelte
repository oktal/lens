<script lang="ts">
	import * as Select from '$lib/components/ui/select';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Label } from '$lib/components/ui/label';
	import Progress from '$lib/components/ui/progress/progress.svelte';
	import { Textarea } from '$lib/components/ui/textarea';

	import { type QueryStream } from '$lib/stores/QueryStream.svelte';

	import Icon from '@iconify/svelte';
	import { ScrollArea } from '../ui/scroll-area';

	let { stream } = $props<{ stream: QueryStream }>();

	class Pagination {
		pageIndex = $state(0);
		pageSize = $state(10);

		start = $derived(pagination.pageIndex * pagination.pageSize);
		end = $derived(Math.min(this.start + pagination.pageSize, stream.rows.length));
		lastPage = $derived(Math.max(Math.ceil(stream.rows.length / pagination.pageSize) - 1, 0));
		page = $derived(
			stream.rows.slice(this.start, this.end).map((row: string[], idx: number) => {
				return {
					index: idx + this.start,
					row
				};
			})
		);

		gotoPage(pageIndex: number) {
			this.pageIndex = pageIndex;
		}

		gotoNextPage() {
			if (this.pageIndex < this.lastPage) this.gotoPage(this.pageIndex + 1);
		}

		gotoPreviousPage() {
			if (this.pageIndex > 0) this.gotoPage(this.pageIndex - 1);
		}

		gotoFirstPage() {
			this.gotoPage(0);
		}

		gotoLastPage() {
			this.gotoPage(this.lastPage);
		}

		canGoToNextPage(): boolean {
			return this.pageIndex < this.lastPage;
		}

		canGoToPreviousPage(): boolean {
			return this.pageIndex > 0;
		}

		setPageSize(pageSize: number) {
			this.pageSize = pageSize;
		}
	}

	let pagination = new Pagination();
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
		{@render paginationControl()}
	</Tabs.Content>
	<Tabs.Content value="text">
		{@render rawText()}
		{@render paginationControl()}
	</Tabs.Content>
</Tabs.Root>

{#snippet dataTable()}
	{#if stream.state === 'running'}
		<Progress value={undefined} />
	{/if}

	<ScrollArea orientation="both" class="h-screen max-h-[60vh] w-screen">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-[50px]">#</Table.Head>
					{#each stream.columns as column}
						<Table.Head>{column}</Table.Head>
					{/each}
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each pagination.page as { index, row }}
					<Table.Row>
						<Table.Cell class="p-1 px-4 font-medium">{index}</Table.Cell>
						{#each row as cell}
							<Table.Cell class="p-1 px-4 font-medium">{cell}</Table.Cell>
						{/each}
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	</ScrollArea>
{/snippet}

{#snippet paginationControl()}
	<div class="flex items-center justify-end space-x-4 py-4">
		<Label>Rows per page</Label>
		<Select.Root
			items={rowsPerPageItems}
			selected={rowsPerPageItems[0]}
			onSelectedChange={(v) => v && pagination.setPageSize(v.value)}
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

		<Label
			>Page {pagination.pageIndex + 1} of {pagination.lastPage} ({pagination.start}-{pagination.end}
			of {stream.rows.length} rows)</Label
		>

		{@render paginationItem({
			icon: 'material-symbols:first-page',
			tooltip: 'First page',
			disabled: !pagination.canGoToPreviousPage(),
			action: () => pagination.gotoFirstPage()
		})}
		{@render paginationItem({
			icon: 'material-symbols:chevron-left',
			tooltip: 'Previous page',
			disabled: !pagination.canGoToPreviousPage(),
			action: () => pagination.gotoPreviousPage()
		})}
		{@render paginationItem({
			icon: 'material-symbols:chevron-right',
			tooltip: 'Next page',
			disabled: !pagination.canGoToNextPage(),
			action: () => pagination.gotoNextPage()
		})}
		{@render paginationItem({
			icon: 'material-symbols:last-page',
			tooltip: 'Last page',
			disabled: !pagination.canGoToNextPage(),
			action: () => pagination.gotoLastPage()
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
		value={pagination.page
			.map((page: any) => {
				return page.row;
			})
			.join('\n')}
	/>
{/snippet}
