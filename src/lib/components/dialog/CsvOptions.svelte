<script lang="ts">
	import { Label } from '$lib/components/ui/label/index';
	import { Switch } from '$lib/components/ui/switch/index';
	import type { CsvConfig } from '$lib/lens/types';
	import Input from '$lib/components/ui/input/input.svelte';

	let { config }: { config: CsvConfig } = $props();
</script>

<div class="grid items-center gap-2">
	<div class="flex items-center justify-between space-x-2">
		<Label for="has-header" class="flex flex-col space-y-1">
			<span>Has header</span>
			<span class="text-xs font-normal leading-snug text-muted-foreground">
				Specifies whether there is a CSV header (i.e. the first line consists of is column names).
			</span>
		</Label>
		<Switch id="has-header" checked={config.hasHeader} aria-label="Has header" />
	</div>

	<div class="flex items-center justify-between space-x-2">
		<Label>Delimiter</Label>
		<Input class="col-span-2" bind:value={config.delimiter} />
	</div>
</div>
