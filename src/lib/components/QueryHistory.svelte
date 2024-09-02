<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import Icon from '@iconify/svelte';
	import { queriesStore } from '$lib/stores/queries.svelte';
	import type { StreamId } from '$lib/lens/types';
	import { queryPaneGroup } from './QueryPaneGroup.svelte';
	import { toast } from 'svelte-sonner';

	function renewStream({ id }: { id: StreamId }) {
		try {
			queryPaneGroup.renew(0, id);
		} catch (e) {
			toast.error(`${e}`);
		}
	}
</script>

<div class="flex flex-col gap-2">
	{#each queriesStore.streams as stream}
		{@const streamInfo = queriesStore.getStreamInfo(stream)}
		<div class="flex flex-row">
			<Label class="flex flex-col">
				<span>{streamInfo.id}</span>
				<span class="text-pretty text-xs text-muted-foreground">{streamInfo.query}</span>
				{#if stream.kind === 'full'}
					<div class="flex gap-1">
						<Icon icon="carbon:save" width={16} height={16} />
						<span class="text-pretty text-xs text-muted-foreground"
							>{stream.stream.rows.length} rows</span
						>
					</div>
				{/if}
			</Label>

			<div class="ml-auto flex flex-nowrap">
				{@render controlItem({
					icon: 'carbon:renew',
					tooltip: 'Renew',
					action: () => renewStream({ id: streamInfo.id })
				})}

				{@render controlItem({
					icon: 'carbon:trash-can',
					tooltip: 'Delete',
					action: () => queriesStore.delete(streamInfo.id)
				})}
			</div>
		</div>
	{/each}
</div>

{#snippet controlItem({ icon, tooltip, action }: { icon: string; tooltip: string; action: any })}
	<Tooltip.Root>
		<Tooltip.Trigger asChild let:builder>
			<Button builders={[builder]} variant="ghost" size="icon" on:click={action}>
				<Icon {icon} width={18} height={18} />
			</Button>
		</Tooltip.Trigger>
		<Tooltip.Content>
			<p>{tooltip}</p>
		</Tooltip.Content>
	</Tooltip.Root>
{/snippet}
