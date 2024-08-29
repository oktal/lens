<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Button } from '$lib/components/ui/button';
	import * as Tooltip from '$lib/components/ui/tooltip/index';
	import Icon from '@iconify/svelte';
	import { queriesStore } from '$lib/stores/queries.svelte';
	import type { StreamId } from '$lib/lens/types';

	function renewStream({ id }: { id: StreamId }) {}
</script>

<div class="flex flex-col gap-2">
	{#each queriesStore.streams as stream}
		<div class="flex flex-row">
			<Label class="flex flex-col">
				<span>{stream.id}</span>
				<span class="text-pretty text-xs text-muted-foreground">{stream.query}</span>
				{#if stream.kind === 'full'}
					<span class="text-pretty text-xs text-muted-foreground"
						>{stream.stream.rows.length} rows</span
					>
				{/if}
			</Label>

			<div class="ml-auto flex flex-nowrap">
				<!-- {@render controlItem({ -->
				<!-- 	icon: 'carbon:renew', -->
				<!-- 	tooltip: 'Renew', -->
				<!-- 	action: renewStream({ id: stream.id }) -->
				<!-- })} -->

				{@render controlItem({
					icon: 'carbon:trash-can',
					tooltip: 'Delete',
					action: () => queriesStore.delete(stream.id)
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
