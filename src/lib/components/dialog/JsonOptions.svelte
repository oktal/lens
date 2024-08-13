<script lang="ts">
	import { Label } from '$lib/components/ui/label/index';
	import { Switch } from '$lib/components/ui/switch/index';
	import * as Select from '$lib/components/ui/select';

	import type { JsonConfig } from '$lib/lens/types';

	let { config }: { config: JsonConfig } = $props();

	let compressed = $state(false);

	const compressions: Array<JsonConfig['compression']> = ['gzip', 'bzip2', 'xz', 'zstd'];
	const compressionItems = compressions.map((c) => {
		return {
			value: c,
			label: c
		};
	});

	$effect(() => {
		if (!compressed) config.compression = 'uncompressed';
	});
</script>

<div class="grid items-center gap-2">
	<div class="flex items-center justify-between space-x-2">
		<Label for="is-compressed" class="flex flex-col space-y-1">
			<span>Compression</span>
		</Label>
		<Switch id="is-compressed" aria-label="Is compressed" bind:checked={compressed} />
	</div>

	{#if compressed}
		<Select.Root
			items={compressionItems}
			onSelectedChange={(c) => (config.compression = c?.value ?? 'uncompressed')}
		>
			<Select.Trigger>
				<Select.Value />
			</Select.Trigger>
			<Select.Content>
				{#each compressions as compression}
					<Select.Item value={compression}>
						{compression}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>
	{/if}
</div>
