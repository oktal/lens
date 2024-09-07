<script lang="ts">
	import QueryPane from '$lib/components/query/QueryPane.svelte';
	import * as Resizable from '$lib/components/ui/resizable';
	import { QueryPaneGroup, queryPaneGroup } from '$lib/components/query/QueryPaneGroup.svelte';
</script>

{#if queryPaneGroup.panes.length > 1}
	<Resizable.PaneGroup direction={queryPaneGroup.direction!}>
		<Resizable.Pane>
			<QueryPane paneId={0} closable direction={queryPaneGroup.direction!} />
		</Resizable.Pane>
		<Resizable.Handle withHandle />
		<Resizable.Pane>
			<QueryPane paneId={1} closable direction={queryPaneGroup.direction!} />
		</Resizable.Pane>
	</Resizable.PaneGroup>
{:else}
	<QueryPane paneId={0} />
{/if}
