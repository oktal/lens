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
	import Grid from '$lib/components/ui/grid/grid.svelte';
	import { open as dialogOpen } from '@tauri-apps/api/dialog';

	import type { Database, DatasourceConfig, FileType } from '$lib/lens/types';
	import Icon from '@iconify/svelte';
	import type { Component, SvelteComponent } from 'svelte';
	import CsvOptions from './CsvOptions.svelte';
	import ParquetOptions from './ParquetOptions.svelte';
	import JsonOptions from './JsonOptions.svelte';

	interface Props {
		databases: Database[];
		datasources: DatasourceConfig[];

		info?: Partial<TableInfo>;
	}

	export function show(): Promise<TableInfo> {
		return new Promise<TableInfo>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;
			open = true;
		});
	}

	type DatasourceItem = { kind: 's3' | 'gcs' | 'file' | 'dir'; url: string };
	type OptionsComponent = Component<{}, { getOptions: () => Record<string, any> }>;

	let { databases, datasources, info }: Props = $props();

	const databaseItems = databases.map((db: Database) => {
		return {
			value: db.name,
			label: db.name
		};
	});

	const FILE_DATASOURCE: DatasourceItem = {
		kind: 'file',
		url: 'file://'
	};

	const DIR_DATASOURCE: DatasourceItem = {
		kind: 'dir',
		url: 'file://'
	};

	const datasourceItems: Array<{ value: DatasourceItem; label: string }> = [
		{
			value: FILE_DATASOURCE,
			label: FILE_DATASOURCE.url
		},
		{
			value: DIR_DATASOURCE,
			label: 'dir://'
		},
		...datasources.map((ds) => {
			return {
				value: { kind: ds.store.kind, url: ds.url },
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

	class Partition {
		name = $state('');
		type = $state('');
	}

	class Table {
		private readonly fileTypeOptions: Record<FileType, OptionsComponent | undefined> = {
			csv: CsvOptions,
			arrow: undefined,
			parquet: ParquetOptions,
			avro: undefined,
			json: JsonOptions
		};

		database = $state('');
		schema = $state('');
		name = $state('');
		fileType = $state<FileType>('csv');

		partitions = $state<Partition[]>([]);

		dataSource = $state<DatasourceItem>({ kind: 'file', url: 'file://' });
		locationPath = $state('');

		browsable = $derived.by(() => {
			const browsable = this.dataSource.kind == 'file' || this.dataSource.kind == 'dir';

			if (browsable) {
				const dir = this.dataSource.kind == 'dir';
				return { dir };
			}

			return undefined;
		});

		public static fromInfo(info: Partial<TableInfo>): Table {
			let table = new Table();

			const { database, schema, name, fileType } = info;

			table.database = database ?? '';
			table.schema = schema ?? '';
			table.name = name ?? '';
			table.fileType = fileType ?? 'csv';

			return table;
		}

		addPartition() {
			this.partitions.push(new Partition());
		}

		deletePartition(index: number) {
			this.partitions = [...this.partitions.slice(0, index), ...this.partitions.slice(index + 1)];
		}

		get optionsComponent(): OptionsComponent | undefined {
			return this.fileTypeOptions[this.fileType];
		}

		getLocationPath(): string {
			if (this.dataSource?.kind === 'dir') {
				// DataFusion requires that table path that represent a directory structure
				// ends with with a '/' delimiter
				if (!this.locationPath.endsWith('/')) return this.locationPath + '/';
			}

			return this.locationPath;
		}

		async openFileBrowser() {
			if (!this.browsable) return;

			const directory = this.browsable.dir;

			const selected = await dialogOpen({
				multiple: false,
				directory
			});
			if (Array.isArray(selected)) {
				this.locationPath = selected[0];
			} else if (selected !== null) {
				this.locationPath = selected;
			}
		}
	}

	const table = info ? Table.fromInfo(info) : new Table();

	let optionsComponent = $state<
		SvelteComponent<{}, { getOptions: () => Record<string, any> }> | undefined
	>(undefined);

	let schemas = $derived(
		databases
			.filter((db) => db.name == table.database)
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

	const dataTypes = ['Bool', 'Double', 'Int', 'Date', 'String'];

	let accept_: ((info: TableInfo) => void) | undefined = undefined;
	let reject_: ((reason?: any) => void) | undefined = undefined;

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

	function closeDialog() {
		open = false;
		if (accept_) {
			const { database, schema, name, fileType, partitions, dataSource } = table;
			const locationPath = table.getLocationPath();
			const location = `${dataSource?.url}${locationPath}`;
			const options = createOptions(optionsComponent?.getOptions() ?? {});
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
				<Select.Root
					selected={{ label: table.database, value: table.database }}
					items={databaseItems}
					onSelectedChange={(v) => v && (table.database = v.value)}
				>
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
				<Select.Root
					selected={{ label: table.schema, value: table.schema }}
					items={schemaItems}
					onSelectedChange={(v) => v && (table.schema = v.value)}
				>
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
				<Input id="name" bind:value={table.name} />
			</Grid>

			<Label>Type</Label>
			<ToggleGroup.Root variant="outline" type="single" bind:value={table.fileType}>
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

			{#if table.optionsComponent}
				<Label class="col-span-3">Options</Label>

				<Card.Root class="col-span-3">
					<div class="m-4">
						<svelte:component this={table.optionsComponent} bind:this={optionsComponent} />
					</div>
				</Card.Root>
			{/if}

			<div class="flex flex-row items-center justify-items-center">
				<Label>Partitioned by</Label>
				<Button
					variant="default"
					size="icon"
					class="ml-auto h-8 w-8"
					on:click={() => table.addPartition()}
				>
					<Icon icon="carbon:add" width={24} height={24} class="w-full" /></Button
				>
			</div>

			<Grid cols={['3fr', '2fr', '1fr']} class="justify-evenly gap-1">
				{#each table.partitions as partition, i}
					<Input bind:value={partition.name} />

					<Select.Root
						items={dataTypes.map((dt) => {
							return { label: dt, value: dt };
						})}
						onSelectedChange={(v) => v && (partition.type = v.value)}
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

					<Button
						variant="outline"
						size="icon"
						class="ml-auto"
						on:click={() => table.deletePartition(i)}
					>
						<Icon icon="mdi:delete" />
					</Button>
				{/each}
			</Grid>

			<Label>Location</Label>
			<div class="flex flex-row gap-1">
				<div class="w-32">
					<Select.Root
						items={datasourceItems}
						selected={{ value: FILE_DATASOURCE, label: FILE_DATASOURCE.url }}
						onSelectedChange={(s) => s && (table.dataSource = s.value)}
					>
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
				<Input bind:value={table.locationPath} class="flex-1" />

				{#if table.browsable}
					<Button variant="secondary" size="default" on:click={() => table.openFileBrowser()}>
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
