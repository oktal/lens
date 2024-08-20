<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import { Label } from '$lib/components/ui/label';
	import { Separator } from '$lib/components/ui/separator';
	import { Textarea } from '$lib/components/ui/textarea';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import Icon from '@iconify/svelte';
	import { type QueryStream, useQueryStream } from './QueryStream.svelte';
	import QueryResultsTable from './QueryResultsTable.svelte';

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
				queryStream = await useQueryStream(queryString);
			} catch (e) {
				handleError(e);
			}

			fetch();
		}
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
				tooltip: queryStream === undefined ? 'Execute query' : 'Resume query',
				disabled: queryStream?.state === 'running' || queryString == '',
				action: runQuery
			})}

			{@render topBarItem({
				icon: 'carbon:pause',
				tooltip: 'Pause running query',
				disabled: queryStream?.state !== 'running',
				action: () => (queryStream!.pause())
			})}

			{@render topBarItem({
				icon: 'carbon:stop',
				tooltip: 'Stop running query',
				disabled: queryStream === undefined || queryStream.state === 'stopped',
				action: () => (queryStream!.stop())
			})}
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
