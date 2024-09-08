<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Tooltip from '$lib/components/ui/tooltip/index';

	import ExportDialog from '$lib/components/dialog/ExportDialog.svelte';
	import QueryResultsTable from './QueryResultsTable.svelte';
	import { queryPaneGroup, type SplitDirection } from './QueryPaneGroup.svelte';

	import Icon from '@iconify/svelte';
	import { mount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { client } from '$lib/lens/api';
	import { ScrollArea } from '../ui/scroll-area';

	interface Props {
		direction?: SplitDirection;
		paneId: number;

		closable?: boolean;
	}

	let { direction, paneId, closable = false }: Props = $props();

	let queryError: any | undefined = $state(undefined);

	const paneOverlayId = paneId + 1;

	async function runQuery() {
		const handleError = (error: any) => {
			queryError = error;
		};

		const fetch = async () => {
			const queryStream = queryPaneGroup.panes[paneId]?.stream;
			try {
				while (queryStream && queryStream.state === 'running' && queryStream.hasNext) {
					await queryStream.fetchNext();
				}
			} catch (e) {
				handleError(e);
			}
		};

		const queryStream = queryPaneGroup.panes[paneId]?.stream;
		if (queryStream?.state === 'paused') {
			queryStream.resume();
			fetch();
		} else {
			try {
				queryError = undefined;
				await queryPaneGroup.run(paneId);
			} catch (e) {
				handleError(e);
			}

			fetch();
		}
	}

	async function exportResults() {
		const queryStream = queryPaneGroup.panes[paneId]?.stream;
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
</script>

<div class="relative flex h-full max-h-screen w-full flex-col gap-1">
	{@render topBar()}

	<div class="m-4 flex flex-col gap-2">
		<Label for="query" class="text-sm font-semibold">Query</Label>
		<Textarea id="query" bind:value={queryPaneGroup.panes[paneId]!.query} />

		{#if queryError}
			<p class="text-red-600">{queryError}</p>
		{/if}
	</div>

	<Separator />

	{#if queryPaneGroup.panes[paneId]?.stream}
		<div class="m-4 h-full">
			<QueryResultsTable stream={queryPaneGroup.panes[paneId].stream} />
		</div>
	{/if}

	{#if queryPaneGroup.overlayVisible}
		<div class="absolute bottom-0 left-0 z-50">
			<Icon icon="carbon:number-{paneOverlayId}" width={80} height={80} />
		</div>
	{/if}
</div>

{#snippet topBar()}
	{@const queryStream = queryPaneGroup.panes[paneId]?.stream}
	<div class="flex h-14 flex-row items-center gap-1 border">
		<div class="ml-2">
			{@render topBarItem({
				icon: 'carbon:play',
				tooltip: queryStream?.state === 'paused' ? 'Resume query' : 'Execute query',
				disabled: queryStream?.state === 'running' || queryPaneGroup.panes[paneId]?.query == '',
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
				disabled: queryPaneGroup.panes[paneId].stream === undefined,
				action: exportResults
			})}

			{@render topBarItem({
				icon: 'carbon:erase',
				tooltip: 'Clear',
				disabled: queryStream === undefined,
				action: () => queryPaneGroup.clear(paneId)
			})}

			{@render topBarItem({
				icon: 'carbon:save',
				tooltip: 'Save results to history',
				disabled: queryStream === undefined,
				action: () => queryPaneGroup.save(paneId)
			})}
		</div>

		<div class="ml-auto mr-2 flex gap-1">
			{#if closable}
				<Button
					variant="secondary"
					size="sm"
					class="flex gap-1"
					on:click={() => queryPaneGroup.close(paneId)}
				>
					{#if direction === 'horizontal'}
						{@const rotationClass = paneId == 0 ? 'rotate-180' : ''}
						<Icon
							icon="carbon:side-panel-close-filled"
							class={rotationClass}
							width={18}
							height={18}
						/>
					{:else if direction === 'vertical'}
						{@const rotationClass = paneId == 0 ? '-rotate-90' : 'rotate-90'}
						<Icon
							icon="carbon:side-panel-close-filled"
							class={rotationClass}
							width={18}
							height={18}
						/>
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
				<DropdownMenu.Item class="flex gap-1" on:click={() => queryPaneGroup.split('horizontal')}>
					<Icon icon="mdi:arrow-split-vertical" width={20} height={20} />
					Vertical
				</DropdownMenu.Item>
				<DropdownMenu.Item class="flex gap-1" on:click={() => queryPaneGroup.split('vertical')}>
					<Icon icon="mdi:arrow-split-vertical" class="rotate-90" width={20} height={20} />
					Horizontal
				</DropdownMenu.Item>
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/snippet}
