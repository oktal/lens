<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Select from '$lib/components/ui/select';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import { LanguageIdEnum } from 'monaco-sql-languages';
	import { mode } from 'mode-watcher';
	import { Temporal } from '@js-temporal/polyfill';

	import ExportDialog from '$lib/components/dialog/ExportDialog.svelte';
	import QueryResultsTable from './QueryResultsTable.svelte';
	import { queryPaneGroup, type SplitDirection } from './QueryPaneGroup.svelte';

	import Icon from '@iconify/svelte';
	import { mount } from 'svelte';
	import { toast } from 'svelte-sonner';

	import { client } from '$lib/lens/api';
	import Monaco from '../monaco/Monaco.svelte';

	interface Props {
		direction?: SplitDirection;
		paneId: number;

		closable?: boolean;
	}

	let { direction, paneId, closable = false }: Props = $props();

	class QueryTitle {
		private _mode = $state<'show' | 'edit'>('show');

		toggleEdit() {
			if (this._mode === 'show') {
				this._mode = 'edit';
			} else {
				queryPaneGroup.setTitle(paneId);
				this._mode = 'show';
			}
		}

		get mode() {
			return this._mode;
		}

		handleKeyDown(ev: KeyboardEvent) {
			if (ev.code === 'Enter') this.toggleEdit();
		}

		focus(input: HTMLInputElement) {
			input.focus();
		}

		reset() {
			this._mode = 'show';
		}
	}

	const queryTitle = new QueryTitle();
	let queryError: any | undefined = $state(undefined);

	let editorLanguage = $state('mysql');
	let editorTheme = $derived($mode === 'light' ? 'dawn' : 'tomorrownight-bright');

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

				if (queryStream && queryStream.state === 'finished')
					queryPaneGroup.panes[paneId].stopWatch.stop();
			} catch (e) {
				handleError(e);
			}
		};

		const queryStream = queryPaneGroup.panes[paneId]?.stream;
		if (queryStream?.state === 'paused') {
			queryStream.resume();
			queryPaneGroup.panes[paneId].stopWatch.resume();
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

	function clear() {
		queryTitle.reset();
		queryPaneGroup.clear(paneId);
	}

	function formatDuration(duration: Temporal.Duration | undefined): string {
		if (!duration) return '';
		const durationString = duration.toString({ smallestUnit: 'millisecond' });
		return durationString
			.replace('PT', '')
			.replaceAll(/(S|H|M|D)/g, ':')
			.replace(/:$/, '');
	}
</script>

<div class="relative flex h-full max-h-screen w-full flex-col gap-1 overflow-auto">
	{@render topBar()}

	<div class="m-4 flex flex-col gap-2">
		<div class="flex items-center">
			<Label for="query" class="text-sm font-semibold">Query</Label>

			<div class="ml-auto w-32">
				<Select.Root
					selected={{ value: 'mysql', label: 'mysql' }}
					onSelectedChange={(v) => v && (editorLanguage = v.value)}
				>
					<Select.Trigger>
						<Select.Value />
					</Select.Trigger>
					<Select.Content>
						{#each Object.values(LanguageIdEnum) as lang}
							<Select.Item value={lang}>
								{lang}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>
			</div>
		</div>

		<Monaco
			bind:value={queryPaneGroup.panes[paneId].query}
			language={editorLanguage}
			theme={editorTheme}
			class="h-72 w-full"
		/>

		{#if queryError}
			<p class="text-red-600">{queryError}</p>
		{/if}
	</div>

	<Separator />

	{#if queryPaneGroup.panes[paneId]?.stream}
		<div class="m-4">
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
	<div class="flex h-10 flex-row items-center gap-1 border">
		<div class="ml-2">
			{@render topBarItem({
				icon: 'carbon:play',
				tooltip: queryStream?.state === 'paused' ? 'Resume query' : 'Execute query',
				disabled: queryStream?.state === 'running' || queryPaneGroup.panes[paneId].query == '',
				action: runQuery
			})}

			{@render topBarItem({
				icon: 'carbon:pause',
				tooltip: 'Pause running query',
				disabled: queryStream?.state !== 'running',
				action: () => queryPaneGroup.pause(paneId)
			})}

			{@render topBarItem({
				icon: 'carbon:stop',
				tooltip: 'Stop running query',
				disabled:
					queryStream === undefined ||
					queryStream.state === 'stopped' ||
					queryStream.state === 'finished',
				action: () => queryPaneGroup.stop(paneId)
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
				action: clear
			})}

			{@render topBarItem({
				icon: 'carbon:save',
				tooltip: 'Save results to history',
				disabled: queryStream === undefined,
				action: () => queryPaneGroup.save(paneId)
			})}
		</div>

		<Separator orientation="vertical" />
		<div class="ml-2 flex items-center gap-1">
			{#if queryTitle.mode === 'show'}
				<Label class="ml-2">{queryPaneGroup.panes[paneId].title}</Label>
			{:else if queryTitle.mode === 'edit'}
				<input
					bind:value={queryPaneGroup.panes[paneId].title}
					class="border-input bg-background text-sm ring-offset-background"
					onkeydown={(e) => queryTitle.handleKeyDown(e)}
					use:queryTitle.focus
				/>
			{/if}
			{@render topBarItem({
				icon: queryTitle.mode === 'show' ? 'carbon:pen' : 'carbon:checkmark',
				tooltip: 'Rename query',
				disabled: false,
				action: () => queryTitle.toggleEdit()
			})}
		</div>

		<div class="ml-auto mr-2 flex items-center gap-1">
			{#if queryPaneGroup.panes[paneId].stopWatch.elapsed}
				<div class="flex flex-row gap-1">
					<Icon icon="carbon:timer" width={18} height={18} />
					<Label>
						{formatDuration(queryPaneGroup.panes[paneId].stopWatch.elapsed)}
					</Label>
				</div>
			{/if}
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
