<script lang="ts">
	import { type QueryStream } from './QueryStream.svelte';
	import QueryResultHeader from './QueryResultHeader.svelte';
	import * as Select from '$lib/components/ui/select';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import Button from '$lib/components/ui/button/button.svelte';
	import { Textarea } from './ui/textarea';
	import Icon from '@iconify/svelte';
	import { Label } from './ui/label';
	import Progress from './ui/progress/progress.svelte';
	import type {
		ColumnDef,
		PaginationState,
		Row,
		SortingState,
		TableOptions,
		Updater
	} from '@tanstack/svelte-table';
	import {
		FlexRender,
		createColumnHelper,
		createTable,
		getCoreRowModel,
		getPaginationRowModel,
		getSortedRowModel,
		renderComponent
	} from '@tanstack/svelte-table';

	let { stream } = $props<{ stream: QueryStream }>();

	const columnHelper = createColumnHelper<string[]>();

	const columns: ColumnDef<string[], string>[] = stream.columns.map(
		(column: string, idx: number) => {
			return columnHelper.accessor((row: string[]) => row[idx], {
				id: column,
				header: ({ header }) => renderComponent(QueryResultHeader, { label: column, header })
			});
		}
	);

	let sorting = $state<SortingState>([]);
	let pagination = $state<PaginationState>({
		pageIndex: 0,
		pageSize: 10
	});

	function setSorting(updater: Updater<SortingState>) {
		if (updater instanceof Function) {
			sorting = updater(sorting);
		} else sorting = updater;
	}

	function setPagination(updater: Updater<PaginationState>) {
		if (updater instanceof Function) {
			pagination = updater(pagination);
		} else pagination = updater;
	}

	const options: TableOptions<string[]> = {
		get data() {
			return stream.rows;
		},
		columns,
		state: {
			get sorting() {
				return sorting;
			},
			get pagination() {
				return pagination;
			}
		},
		onSortingChange: setSorting,
		onPaginationChange: setPagination,
		getCoreRowModel: getCoreRowModel(),
		getSortedRowModel: getSortedRowModel(),
		getPaginationRowModel: getPaginationRowModel()
	};

	const table = createTable(options);

	let start = $derived(pagination.pageIndex * pagination.pageSize);
	let end = $derived(Math.min(start + pagination.pageSize, stream.rows.length));
	let lastPage = $derived(Math.max(Math.ceil(stream.rows.length / pagination.pageSize) - 1, 0));

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
			{#each table.getHeaderGroups() as headerGroup}
				<Table.Row>
					{#each headerGroup.headers as header}
						<Table.Head>
							{#if !header.isPlaceholder}
								<FlexRender
									content={header.column.columnDef.header}
									context={header.getContext()}
								/>
							{/if}
						</Table.Head>
					{/each}
				</Table.Row>
			{/each}
		</Table.Header>
		<Table.Body>
			{#each table.getRowModel().rows as row}
				<Table.Row>
					{#each row.getVisibleCells() as cell}
						<Table.Cell class="p-1 px-8 font-medium">
							<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
						</Table.Cell>
					{/each}
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>

	{@render paginationControl()}
{/snippet}

{#snippet paginationControl()}
	<div class="flex items-center justify-end space-x-4 py-4">
		<Label>Rows per page</Label>
		<Select.Root
			items={rowsPerPageItems}
			selected={rowsPerPageItems[0]}
			onSelectedChange={(v) => v && table.setPageSize(v.value)}
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
			>Page {pagination.pageIndex + 1} of {lastPage} ({start}-{end} of {stream.rows.length} rows)</Label
		>

		{@render paginationItem({
			icon: 'material-symbols:first-page',
			tooltip: 'First page',
			disabled: !table.getCanPreviousPage(),
			action: () => table.firstPage()
		})}
		{@render paginationItem({
			icon: 'material-symbols:chevron-left',
			tooltip: 'Previous page',
			disabled: !table.getCanPreviousPage(),
			action: () => table.previousPage()
		})}
		{@render paginationItem({
			icon: 'material-symbols:chevron-right',
			tooltip: 'Next page',
			disabled: !table.getCanNextPage(),
			action: () => table.nextPage()
		})}
		{@render paginationItem({
			icon: 'material-symbols:last-page',
			tooltip: 'Last page',
			disabled: !table.getCanNextPage(),
			action: () => table.lastPage()
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
	<Textarea readonly class="h-full" />
{/snippet}
