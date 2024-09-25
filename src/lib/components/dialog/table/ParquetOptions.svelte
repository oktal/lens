<script lang="ts">
	import { Label } from '$lib/components/ui/label/index';
	import { Switch } from '$lib/components/ui/switch/index';

	let enablePageIndex = $state(true);
	let pushdownFilters = $state(true);
	let pruning = $state(false);

	export function getOptions(): Record<string, any> {
		return { enablePageIndex, pushdownFilters, pruning };
	}
</script>

<div class="grid items-center gap-2">
	<div class="flex items-center justify-between space-x-2">
		<Label for="enable-page-index" class="flex flex-col space-y-1">
			<span>Enable page index</span>
			<span class="text-xs font-normal leading-snug text-muted-foreground">
				Read the Parquet data page level metadata (the Page Index) if present, to reduce the I/O and
				number of rows decoded.
			</span>
		</Label>
		<Switch id="enable-page-index" bind:checked={enablePageIndex} aria-label="Enable Page Index" />
	</div>

	<div class="flex items-center justify-between space-x-2">
		<Label for="pruning" class="flex flex-col space-y-1">
			<span>Pruning</span>
			<span class="text-xs font-normal leading-snug text-muted-foreground">
				Attempt to skip entire row groups based on the predicate in the query and the metadata
				(min/max values) stored in the parquet file
			</span>
		</Label>
		<Switch id="pruning" bind:checked={pruning} aria-label="Pruning" />
	</div>

	<div class="flex items-center justify-between space-x-2">
		<Label for="pushdown-filters" class="flex flex-col space-y-1">
			<span>Pushdown filters</span>
			<span class="text-xs font-normal leading-snug text-muted-foreground">
				Filter expressions applied during the parquet decoding operation to reduce the number of
				rows decoded. This optimization is sometimes called "late materialization".
			</span>
		</Label>
		<Switch id="pushdown-filters" bind:checked={pushdownFilters} aria-label="Pushdown filters" />
	</div>
</div>
