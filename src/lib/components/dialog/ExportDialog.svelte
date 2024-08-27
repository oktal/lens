<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Switch } from '$lib/components/ui/switch/index';
	import * as Card from '$lib/components/ui/card';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Select from '$lib/components/ui/select';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import type { ExportFileFormat, ExportOptions } from '$lib/lens/types';
	import Icon from '@iconify/svelte';
	import { Grid } from '../ui/grid';
	import { save as dialogSave } from '@tauri-apps/api/dialog';

	type Option<T> = {
		get value(): T;
		set value(val: T);
	};

	type WriteOptions = {
		overwrite: Option<boolean>;
		singleFile: Option<boolean>;

		partitionBy: Option<Option<string>[]>;
	};

	let open = $state(false);
	let format = $state<ExportFileFormat>('csv');

	let exportPath = $state('');
	let writeOptions = useOptions();

	const exportFormatIcons: Record<ExportFileFormat, string> = {
		csv: 'carbon:csv',
		parquet: 'simple-icons:apacheparquet',
		json: 'carbon:json'
	};

	export async function show(): Promise<ExportOptions> {
		return new Promise<ExportOptions>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;

			open = true;
		});
	}

	let accept_: (options: ExportOptions) => void;
	let reject_: () => void;

	function useOption<T>(initial: T): Option<T> {
		let value = $state(initial);

		return {
			get value(): T {
				return value;
			},
			set value(val: T) {
				value = val;
			}
		};
	}

	function useOptions(): WriteOptions {
		return {
			overwrite: useOption(false),
			singleFile: useOption(false),
			partitionBy: useOption([])
		};
	}

	function createPartition() {
		const partition = useOption<string>('');
		writeOptions.partitionBy.value.push(partition);
	}

	function deletePartition(index: number) {
		writeOptions.partitionBy.value = [
			...writeOptions.partitionBy.value.slice(0, index),
			...writeOptions.partitionBy.value.slice(index + 1)
		];
	}

	async function openFileBrowser() {
		const selected = await dialogSave();
		if (Array.isArray(selected)) {
			exportPath = selected[0];
		} else if (selected !== null) {
			exportPath = selected;
		}
	}

	function closeDialog() {
		open = false;
		if (accept_) {
			const { overwrite, singleFile, partitionBy } = writeOptions;

			const options = {
				format,
				writeOptions: {
					overwrite: overwrite.value,
					singleFile: singleFile.value,
					partitionBy: partitionBy.value.map((p: Option<string>) => p.value)
				},
				path: exportPath
			};

			accept_(options);
		}
	}

	function cancel() {
		open = false;
		if (reject_) reject_();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[680px] overflow-scroll sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Export</Dialog.Title>
			<Dialog.Description>Export data in your favorite file format</Dialog.Description>
		</Dialog.Header>
		<div class="flex flex-col gap-4">
			<Label>Format</Label>

			<ToggleGroup.Root class="justify-start" variant="outline" type="single" bind:value={format}>
				{#each Object.entries(exportFormatIcons) as [fileType, icon]}
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

			<Label>Options</Label>
			<Card.Root>
				<div class="m-4">
					{@render displayWriteOptions()}
				</div>
			</Card.Root>

			<div class="flex flex-row items-center">
				<Label>Location</Label>
			</div>

			<div class="flex flex-row gap-1">
				<Input bind:value={exportPath} />
				<Button variant="secondary" size="default" on:click={openFileBrowser}>
					<Icon icon="mdi:dots-horizontal" />
				</Button>
			</div>
		</div>
		<Dialog.Footer>
			<Button on:click={cancel}>Cancel</Button>
			<Button on:click={closeDialog}>Export</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

{#snippet displayWriteOptions()}
	<div class="grid items-center gap-2">
		<div class="flex items-center justify-between space-x-2">
			<Label for="overwrite" class="flex flex-col space-y-1">
				<span>Overwrite</span>
				<span class="text-xs font-normal leading-snug text-muted-foreground">
					Controls if existing data should be overwritten
				</span>
			</Label>
			<Switch id="overwrite" checked={writeOptions.overwrite.value} aria-label="Overwrite" />
		</div>

		<div class="flex items-center justify-between space-x-2">
			<Label for="single-file" class="flex flex-col space-y-1">
				<span>Single file</span>
				<span class="text-xs font-normal leading-snug text-muted-foreground">
					Controls if all partitions should be coalesced into a single output file Generally will
					have slower performance when set to true.
				</span>
			</Label>
			<Switch id="single-file" checked={writeOptions.singleFile.value} aria-label="Single file" />
		</div>
	</div>

	<div class="flex flex-row items-center justify-items-center">
		<Label>Partition by</Label>
		<Button variant="default" size="icon" class="ml-auto h-8 w-8" on:click={createPartition}>
			<Icon icon="carbon:add" width={24} height={24} class="w-full" /></Button
		>
	</div>

	<span class="text-xs font-normal leading-snug text-muted-foreground">
		Sets which columns should be used for hive-style partitioned writes by name.
	</span>
	<Grid cols={['90%', '10%']} class="justify-evenly gap-1">
		{#each writeOptions.partitionBy.value as partition, i}
			<Input bind:value={partition.value} class="w-full" />

			<Button variant="outline" size="icon" class="ml-auto" on:click={() => deletePartition(i)}>
				<Icon icon="mdi:delete" />
			</Button>
		{/each}
	</Grid>
{/snippet}
