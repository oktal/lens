<script lang="ts">
	import QueryPane, { type SplitDirection } from './QueryPane.svelte';
	import * as Resizable from '$lib/components/ui/resizable';

	let direction = $state<SplitDirection | undefined>(undefined);

	function onSplit(split: SplitDirection) {
		direction = split;
	}
</script>

{#if direction}
	<Resizable.PaneGroup {direction}>
		<Resizable.Pane>
			<QueryPane {onSplit} />
		</Resizable.Pane>
		<Resizable.Handle withHandle />
		<Resizable.Pane>
			<QueryPane {onSplit} closable {direction} onClose={() => (direction = undefined)} />
		</Resizable.Pane>
	</Resizable.PaneGroup>
{:else}
	<QueryPane {onSplit} />
{/if}
