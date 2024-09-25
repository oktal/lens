<script lang="ts">
	import Icon from '@iconify/svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Card from '$lib/components/ui/card';
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import DatasourceDialog from '$lib/components/dialog/datasource/DatasourceDialog.svelte';
	import type { DatasourceConfig, StoreType } from '$lib/lens/types';
	import { client } from '$lib/lens/api';

	import { toast } from 'svelte-sonner';

	type DatasourceDisplayItem = { kind: StoreType; url: string; config: Array<string> };

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

	function toDatasourceDisplayItem({ url, store }: DatasourceConfig): DatasourceDisplayItem {
		if (store.kind === 's3') {
			const { bucket, region } = store.config;
			const config = bucket ? [bucket] : [];

			return {
				kind: 's3',
				url,
				config: [...config, region]
			};
		}

		if (store.kind === 'gcs') {
			const { bucket } = store.config;

			return {
				kind: 'gcs',
				url,
				config: [bucket]
			};
		}

		throw Error('unhandled store');
	}

	const datasourceItems: DatasourceDisplayItem[] = $derived(
		datasources.map((ds) => toDatasourceDisplayItem(ds))
	);
	const datasourceGroupItems = $derived(Object.groupBy(datasourceItems, ({ kind }) => kind));

	$effect(() => {
		client.list.datasources().then((sources: DatasourceConfig[]) => (datasources = sources));
	});
</script>

<div class="flex flex-col gap-1">
	<Button variant="secondary" size="sm" class="flex w-min gap-1" on:click={createDatasource}>
		<Icon icon="carbon:document-add" width={22} height={22} />
		Create
	</Button>

	{#each Object.entries(datasourceGroupItems) as [kind, datasourceItems]}
		{#each datasourceItems as { url, config }}
			<div
				class="mb-1 mr-2 rounded-md border p-2 shadow-sm transition-all hover:bg-accent hover:text-accent-foreground"
			>
				<div class="flex items-center gap-4">
					<Icon icon={storeIcons[kind]} width={38} height={38} />
					<div class="grid gap-1">
						<p class="text-sm font-bold leading-none">{url}</p>
						{#each config as configItem}
							<p class="text-xs font-medium text-slate-500">{configItem}</p>
						{/each}
					</div>
				</div>
			</div>
		{/each}
	{/each}
</div>

<DatasourceDialog bind:this={datasourceDialog} />
