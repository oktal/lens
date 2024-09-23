<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import type { GoogleCloudStorageConfig } from '$lib/lens/types';

	let serviceAccountKey = $state<string | undefined>(undefined);
	let serviceAccountPath = $state<string | undefined>(undefined);
	let applicationCredentialsPath = $state<string | undefined>(undefined);
	let bucket = $state('');

	export function getConfig(): GoogleCloudStorageConfig {
		return {
			serviceAccountKey,
			serviceAccountPath,
			applicationCredentialsPath,
			bucket
		};
	}
</script>

<div class="flex w-full flex-col gap-2">
	<Input bind:value={serviceAccountKey} placeholder="Account key" />
	<Input bind:value={serviceAccountPath} placeholder="Account path" />
	<Input bind:value={applicationCredentialsPath} placeholder="Application credentials path" />
	<Input bind:value={bucket} placeholder="Bucket" />
</div>
