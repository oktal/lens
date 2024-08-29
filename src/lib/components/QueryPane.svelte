<script lang="ts" context="module">
	export type SplitDirection = 'vertical' | 'horizontal';
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import Icon from '@iconify/svelte';
	import { type QueryStream } from '$lib/stores/QueryStream.svelte';
	import QueryResultsTable from './QueryResultsTable.svelte';
	import { mount } from 'svelte';
	import ExportDialog from './dialog/ExportDialog.svelte';
	import { toast } from 'svelte-sonner';
	import { client } from '$lib/lens/api';
	import { queriesStore } from '$lib/stores/queries.svelte';

	interface Props {
		onSplit: (directory: SplitDirection) => void;
		direction?: SplitDirection;

		closable?: boolean;
		onClose?: () => void;
	}

	let { onSplit, direction, onClose, closable = false }: Props = $props();

	let queryString = $state('');
	let queryStream = $state<QueryStream | undefined>(undefined);
	let queryError: any | undefined = $state(undefined);

	async function runQuery() {
		const handleError = (error: any) => {
			queryError = error;
			queryStream = undefined;
		};

		const fetch = async () => {
			try {
				while (queryStream && queryStream.state === 'running' && queryStream.hasNext) {
					await queryStream.fetchNext();
				}
			} catch (e) {
				handleError(e);
			}
		};

		if (queryStream?.state === 'paused') {
			queryStream.resume();
			fetch();
		} else {
			try {
				queryError = undefined;
				queryStream = await queriesStore.run(queryString);
			} catch (e) {
				handleError(e);
			}

			fetch();
		}
	}

	async function exportResults() {
		if (queryStream === undefined) return;

		const dialog = mount(ExportDialog, {
			target: document.body
		});

		const exportOptions = await dialog.show();

		try {
			const count = await client.stream.export(queryStream.streamId, exportOptions);
			toast.success(`Exported ${count} rows to ${exportOptions.path}`);
		} catch (e) {
			toast.error(`Failed to export data: ${e}`);
		}
	}

	function clear() {
		queryString = '';
		queryStream = undefined;
	}
</script>

<div class="flex flex-col gap-1">
	{@render topBar()}

	<div class="m-4 flex flex-col gap-2">
		<Label for="query" class="text-sm font-semibold">Query</Label>
		<Textarea id="query" bind:value={queryString} />

		{#if queryError}
			<p class="text-red-600">{queryError}</p>
		{/if}
	</div>

	<Separator />

	{#if queryStream}
		<div class="m-4">
			<QueryResultsTable stream={queryStream} />
		</div>
	{/if}
</div>

{#snippet topBar()}
	<div class="flex h-14 flex-row items-center gap-1 border">
		<div class="ml-2">
			{@render topBarItem({
				icon: 'carbon:play',
				tooltip: queryStream?.state === 'paused' ? 'Resume query' : 'Execute query',
				disabled: queryStream?.state === 'running' || queryString == '',
				action: runQuery
			})}

			{@render topBarItem({
				icon: 'carbon:pause',
				tooltip: 'Pause running query',
				disabled: queryStream?.state !== 'running',
				action: () => queryStream!.pause()
			})}

			{@render topBarItem({
				icon: 'carbon:stop',
				tooltip: 'Stop running query',
				disabled:
					queryStream === undefined ||
					queryStream.state === 'stopped' ||
					queryStream.state === 'finished',
				action: () => queryStream!.stop()
			})}
		</div>

		<Separator orientation="vertical" />
		<div class="ml-2">
			{@render topBarItem({
				icon: 'carbon:export',
				tooltip: 'Export',
				disabled: queryStream === undefined,
				action: exportResults
			})}

			{@render topBarItem({
				icon: 'carbon:erase',
				tooltip: 'Clear',
				disabled: queryStream === undefined,
				action: clear
			})}

			{@render topBarItem({
				icon: 'carbon:save',
				tooltip: 'Save results to history',
				disabled: queryStream === undefined,
				action: () => queryStream && queriesStore.save(queryStream)
			})}
		</div>

		<div class="ml-auto mr-2 flex gap-1">
			{#if closable}
				<Button
					variant="secondary"
					size="sm"
					class="flex gap-1"
					on:click={() => onClose && onClose()}
				>
					{#if direction === 'horizontal'}
						<Icon icon="carbon:side-panel-close-filled" width={18} height={18} />
					{:else if direction === 'vertical'}
						<Icon icon="carbon:side-panel-close-filled" class="rotate-90" width={18} height={18} />
					{/if}
					Close
				</Button>
			{/if}

			{@render splitMenu()}
		</div>
	</div>
{/snippet}

{#snippet topBarItem({
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
				<Icon {icon} width={18} height={18} />
			</Button>
		</Tooltip.Trigger>
		<Tooltip.Content>
			<p>{tooltip}</p>
		</Tooltip.Content>
	</Tooltip.Root>
{/snippet}

{#snippet splitMenu()}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger>
			<Button variant="secondary" class="flex gap-1" size="sm">
				<Icon icon="carbon:split-screen" width={24} height={24} />Split</Button
			></DropdownMenu.Trigger
		>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
				<DropdownMenu.Item class="flex gap-1" on:click={() => onSplit('horizontal')}>
					<Icon icon="mdi:arrow-split-vertical" width={20} height={20} />
					Vertical
				</DropdownMenu.Item>
				<DropdownMenu.Item class="flex gap-1" on:click={() => onSplit('vertical')}>
					<Icon icon="mdi:arrow-split-vertical" class="rotate-90" width={20} height={20} />
					Horizontal
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/snippet}
