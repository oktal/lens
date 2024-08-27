<script lang="ts" context="module">
	export type TableInfo = {
		database: string;
		schema: string;
		name: string;

		fileType: FileType;
		options: Record<string, string>;
		partitions: Array<{ name: string; type?: string }>;
		location: string;
	};
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import * as Card from '$lib/components/ui/card';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Select from '$lib/components/ui/select';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import { open as dialogOpen } from '@tauri-apps/api/dialog';

	import type {
		CsvConfig,
		Database,
		DatasourceConfig,
		FileType,
		JsonConfig,
		ParquetConfig
	} from '$lib/lens/types';
	import Icon from '@iconify/svelte';
	import type { Component } from 'svelte';
	import CsvOptions from './CsvOptions.svelte';
	import ParquetOptions from './ParquetOptions.svelte';
	import JsonOptions from './JsonOptions.svelte';
	import Grid from '../ui/grid/grid.svelte';

	interface Props {
		databases: Database[];
		datasources: DatasourceConfig[];
	}

	export function show(): Promise<TableInfo> {
		return new Promise<TableInfo>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;
			open = true;
		});
	}

	type DatasourceItem = { kind: 's3' | 'gcs' | 'file' | 'dir'; url: string };

	let { databases, datasources }: Props = $props();

	const databaseItems = databases.map((db: Database) => {
		return {
			value: db.name,
			label: db.name
		};
	});

	const datasourceItems: Array<{ value: DatasourceItem | undefined; label: string }> = [
		{
			value: {
				kind: 'file',
				url: 'file://'
			},
			label: 'file://'
		},
		{
			value: {
				kind: 'dir',
				url: 'file://'
			},
			label: 'dir://'
		},
		...datasources.map((ds) => {
			return {
				value: toDatasourceItem(ds),
				label: ds.url
			};
		})
	];

	const fileTypeIcons: Record<FileType, string> = {
		csv: 'carbon:csv',
		arrow: 'mdi:arrow',
		parquet: 'simple-icons:apacheparquet',
		avro: 'simple-icons:favro',
		json: 'carbon:json'
	};

	const storeIcons: Record<string, string> = {
		s3: 'mdi:aws',
		gcs: 'mdi:google',
		file: 'mdi:file',
		dir: 'mdi:folder'
	};

	let database = $state('');
	let schema = $state('');
	let name = $state('');
	let fileType: FileType = $state('csv');
	let partitions = $state<
		{
			get name(): string;
			set name(val: string);
			get type(): string | undefined;
			set type(val: string);
		}[]
	>([]);
	let dataSource: DatasourceItem | undefined = $state();
	let locationPath = $state('');

	let schemas = $derived(
		databases
			.filter((db) => db.name == database)
			.flatMap((db) => db.schemas.map((schema) => schema.name))
	);
	let schemaItems = $derived(
		schemas.map((schema: string) => {
			return {
				value: schema,
				label: schema
			};
		})
	);

	let open = $state(false);

	let browsable = $derived.by(() => {
		if (typeof dataSource !== 'undefined') {
			const browsable = dataSource.kind == 'file' || dataSource.kind == 'dir';

			if (browsable) {
				const dir = dataSource.kind == 'dir';
				return { dir };
			}

			return undefined;
		} else {
			return undefined;
		}
	});

	let csvConfig: CsvConfig = $state({
		hasHeader: false,
		delimiter: ','
	});

	let parquetConfig: ParquetConfig = $state({
		enablePageIndex: true,
		pruning: true,
		pushdownFilters: false
	});

	let jsonConfig: JsonConfig = $state({
		compression: 'uncompressed'
	});

	let fileTypeOpts: Record<
		FileType,
		{
			config: Record<string, string | number | boolean> | {};
			component?: Component<{ config: any }>;
		}
	> = {
		csv: {
			config: csvConfig,
			component: CsvOptions
		},
		arrow: {
			config: {},
			component: undefined
		},
		parquet: {
			config: parquetConfig,
			component: ParquetOptions
		},
		avro: {
			config: {},
			component: undefined
		},
		json: {
			config: jsonConfig,
			component: JsonOptions
		}
	};

	const dataTypes = ['Bool', 'Double', 'Int', 'Date', 'String'];

	let accept_: ((info: TableInfo) => void) | undefined = undefined;
	let reject_: ((reason?: any) => void) | undefined = undefined;

	function deletePartition(index: number) {
		partitions = [...partitions.slice(0, index), ...partitions.slice(index + 1)];
	}

	function createPartition() {
		let name = $state('');
		let type = $state<string | undefined>(undefined);

		const partition = {
			get name(): string {
				return name;
			},
			set name(val: string) {
				name = val;
			},

			get type(): string | undefined {
				return type;
			},
			set type(val: string) {
				type = val;
			}
		};

		partitions.push(partition);
	}

	async function openFileBrowser(directory: boolean) {
		const selected = await dialogOpen({
			multiple: false,
			directory
		});
		if (Array.isArray(selected)) {
			locationPath = selected[0];
		} else if (selected !== null) {
			locationPath = selected;
		}
	}

	function toDatasourceItem({ url, store }: DatasourceConfig): DatasourceItem | undefined {
		if ('s3' in store) {
			return {
				kind: 's3',
				url
			};
		}

		if ('gcs' in store) {
			return {
				kind: 'gcs',
				url
			};
		}

		return undefined;
	}

	function createOptions(
		options: Record<string, number | string | boolean>
	): Record<string, string> {
		const toSnakeCase = (str: string): string => {
			const result = str.replace(/([A-Z])/g, ' $1');
			return result.split(' ').join('_').toLowerCase();
		};

		let opts: Record<string, string> = {};
		Object.entries(options).forEach(([k, v]) => {
			const key = toSnakeCase(k);

			if (typeof v === 'string') opts[key] = v;
			else if (typeof v === 'boolean') opts[key] = v ? 'true' : 'false';
			else if (typeof v === 'number') opts[key] = v.toString();
		});

		return opts;
	}

	function getLocationPath(): string {
		if (dataSource?.kind === 'dir') {
			// DataFusion requires that table path that represent a directory structure
			// ends with with a '/' delimiter
			if (!locationPath.endsWith('/')) return locationPath + '/';
		}

		return locationPath;
	}

	function closeDialog() {
		open = false;
		if (accept_) {
			const options = createOptions(fileTypeOpts[fileType].config);
			const location = `${dataSource?.url}${getLocationPath()}`;
			accept_({ database, schema, name, fileType, partitions, options, location });
		}
	}

	function cancel() {
		open = false;
		if (reject_) reject_('dialog has been cancelled by user');
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="overflow-scroll sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Create table</Dialog.Title>
			<Dialog.Description
				>Create a named entity for your data that can be queried later on through SQL</Dialog.Description
			>
		</Dialog.Header>
		<div class="flex flex-col gap-4 py-4">
			<Grid rows={3} cols={['1fr', '2fr']} class="items-center gap-2">
				<Label>Database</Label>
				<Select.Root items={databaseItems} onSelectedChange={(v) => v && (database = v.value)}>
					<Select.Trigger>
						<Select.Value />
					</Select.Trigger>
					<Select.Content>
						{#each databaseItems as { value, label }}
							<Select.Item {value} class="flex gap-1">
								{label}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>

				<Label>Schema</Label>
				<Select.Root items={schemaItems} onSelectedChange={(v) => v && (schema = v.value)}>
					<Select.Trigger>
						<Select.Value />
					</Select.Trigger>
					<Select.Content>
						{#each schemaItems as { value, label }}
							<Select.Item {value} class="flex gap-1">
								{label}
							</Select.Item>
						{/each}
					</Select.Content>
				</Select.Root>

				<Label for="name">Name</Label>
				<Input id="name" bind:value={name} />
			</Grid>

			<Label>Type</Label>
			<ToggleGroup.Root variant="outline" type="single" bind:value={fileType}>
				{#each Object.entries(fileTypeIcons) as [fileType, icon]}
					<ToggleGroup.Item
						value={fileType}
						aria-label={fileType}
						class="flex h-full w-full flex-col gap-2"
					>
						<Icon {icon} width={24} height={24} />
						<span class="text-sm">{fileType}</span>
					</ToggleGroup.Item>
				{/each}
			</ToggleGroup.Root>

			{#if fileTypeOpts[fileType].component}
				{@const { config, component } = fileTypeOpts[fileType]}

				<Label class="col-span-3">Options</Label>

				<Card.Root class="col-span-3">
					<div class="m-4">
						<svelte:component this={component} {config} />
					</div>
				</Card.Root>
			{/if}

			<div class="flex flex-row items-center justify-items-center">
				<Label>Partitioned by</Label>
				<Button variant="default" size="icon" class="ml-auto h-8 w-8" on:click={createPartition}>
					<Icon icon="carbon:add" width={24} height={24} class="w-full" /></Button
				>
			</div>

			<Grid cols={['3fr', '2fr', '1fr']} class="justify-evenly gap-1">
				{#each partitions as partition, i}
					<Input bind:value={partition.name} />

					<Select.Root
						items={dataTypes.map((dt) => {
							return { label: dt, value: dt };
						})}
						onSelectedChange={(v) => v && (partitions[i].type = v.value)}
					>
						<Select.Trigger>
							<Select.Value placeholder="Type" />
						</Select.Trigger>
						<Select.Content>
							{#each dataTypes as dataType}
								<Select.Item value={dataType}>{dataType}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>

					<Button variant="outline" size="icon" class="ml-auto" on:click={() => deletePartition(i)}>
						<Icon icon="mdi:delete" />
					</Button>
				{/each}
			</Grid>

			<Label>Location</Label>
			<div class="flex flex-row gap-1">
				<div class="w-32">
					<Select.Root items={datasourceItems} onSelectedChange={(s) => (dataSource = s?.value)}>
						<Select.Trigger>
							<Select.Value />
						</Select.Trigger>
						<Select.Content>
							{#each datasourceItems as { value, label }}
								{#if value}
									<Select.Item {value} class="flex items-center gap-1">
										<Icon icon={storeIcons[value.kind]} width={24} height={24} />
										{label}
									</Select.Item>
								{/if}
							{/each}
						</Select.Content>
					</Select.Root>
				</div>
				<Input bind:value={locationPath} class="flex-1" />

				{#if browsable}
					<Button
						variant="secondary"
						size="default"
						on:click={() => openFileBrowser(browsable.dir)}
					>
						<Icon icon="mdi:dots-horizontal" />
					</Button>
				{/if}
			</div>
		</div>
		<Dialog.Footer>
			<Button on:click={cancel}>Cancel</Button>
			<Button on:click={closeDialog}>Create</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
