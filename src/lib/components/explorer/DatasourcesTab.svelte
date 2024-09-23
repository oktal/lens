<script lang="ts">
	import Icon from '@iconify/svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import DatasourceDialog from '$lib/components/dialog/datasource/DatasourceDialog.svelte';
	import type { DatasourceConfig } from '$lib/lens/types';
	import { client } from '$lib/lens/api';

	import { toast } from 'svelte-sonner';

	type DatasourceItem = { kind: 's3' | 'gcs'; url: string; config: Array<string> };

	let datasourceDialog: DatasourceDialog;
	let datasources: DatasourceConfig[] = $state([]);

	const storeIcons: Record<string, string> = {
		s3: 'mdi:aws',
		gcs: 'mdi:google'
	};

	async function createDatasource() {
		const config = await datasourceDialog.show();
		try {
			await client.create.datasource(config);

			toast.success(`Datasource ${config.url} successfully created`);
		} catch (e) {
			toast.error(`Error creating datasource: ${e}`);
		}

		datasources = await client.list.datasources();
	}

	function toDatasourceItem({ url, store }: DatasourceConfig): DatasourceItem | undefined {
		if ('s3' in store) {
			const s3Config = store.s3;
			const config = s3Config.bucket ? [s3Config.bucket] : [];

			return {
				kind: 's3',
				url,
				config: [...config, s3Config.region]
			};
		}

		if ('gcs' in store) {
			const gcsConfig = store.gcs;

			return {
				kind: 'gcs',
				url,
				config: [gcsConfig.bucket]
			};
		}

		return undefined;
	}

	$effect(() => {
		client.list.datasources().then((sources: DatasourceConfig[]) => (datasources = sources));
	});
</script>

<div class="flex flex-col gap-1">
	<Button variant="secondary" size="sm" class="flex w-min gap-1" on:click={createDatasource}>
		<Icon icon="carbon:document-add" width={22} height={22} />
		Create
	</Button>

	{#each datasources as datasource}
		{@const item = toDatasourceItem(datasource)}
		{#if item}
			<div class="rounded-md p-2 transition-all hover:bg-accent hover:text-accent-foreground">
				<div class="flex gap-4">
					<Icon icon={storeIcons[item.kind]} width={32} height={32} />
					<div class="grid gap-1">
						<p class="text-sm font-bold leading-none">{item.url}</p>
						{#each item.config as configItem}
							<p class="text-xs font-medium text-slate-500">{configItem}</p>
						{/each}
					</div>
				</div>
			</div>
		{/if}
	{/each}
</div>

<DatasourceDialog bind:this={datasourceDialog} />
