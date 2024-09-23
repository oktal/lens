<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label/index.js';
	import type { DatasourceConfig } from '$lib/lens/types';
	import Icon from '@iconify/svelte';
	import type { Component } from 'svelte';
	import AmazonS3Options from './AmazonS3Options.svelte';
	import GoogleCloudStorageOptions from './GoogleCloudStorageOptions.svelte';

	export function show(): Promise<DatasourceConfig> {
		return new Promise<DatasourceConfig>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;
			open = true;
		});
	}

	type DataSource = 's3' | 'gcs';

	type OptionsComponent = Component<{}, { getConfig: () => any }>;
	type DatasourceItem = {
		label: string;
		icon: string;
		get url(): string;
		set url(value: string);
		options?: OptionsComponent;
	};

	let options = $state<OptionsComponent | undefined>(undefined);

	let datasources: Record<DataSource, DatasourceItem> = {
		s3: useDatasource({
			label: 'Amazon S3',
			icon: 'mdi:aws',
			defaultUrl: 's3://',
			options: AmazonS3Options
		}),
		gcs: useDatasource({
			label: 'Google Cloud Storage',
			icon: 'mdi:google',
			defaultUrl: 'gs://',
			options: GoogleCloudStorageOptions
		})
	};

	let open = $state(false);
	let selected: DataSource = $state('s3');

	let accept_: (config: DatasourceConfig) => void;
	let reject_: () => void;

	function useDatasource({
		label,
		icon,
		defaultUrl,
		options
	}: {
		label: string;
		icon: string;
		defaultUrl: string;
		options?: OptionsComponent;
	}): DatasourceItem {
		let url = $state(defaultUrl);

		return {
			label,
			icon,
			get url() {
				return url;
			},
			set url(value: string) {
				url = value;
			},
			options
		};
	}

	function closeDialog() {
		open = false;
		const datasource = datasources[selected];
		const storeConfig = options?.getConfig();

		const config: DatasourceConfig = {
			url: datasource.url,
			store: Object.fromEntries([[selected, storeConfig]])
		};

		if (accept_) accept_(config);
	}

	function cancel() {
		open = false;
		if (reject_) reject_();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Create data source</Dialog.Title>
			<Dialog.Description>Instruct lens where to find your data</Dialog.Description>
		</Dialog.Header>
		<div class="flex flex-col gap-2">
			<Label>Type</Label>
			<ToggleGroup.Root
				variant="outline"
				type="single"
				class="place-self-start"
				bind:value={selected}
			>
				{#each Object.entries(datasources) as [id, { label, icon }]}
					<ToggleGroup.Item value={id} aria-label={label} class="flex gap-2">
						<Icon {icon} width={32} height={32} />
						{id}
					</ToggleGroup.Item>
				{/each}
			</ToggleGroup.Root>

			<div class="flex w-full max-w-sm flex-col gap-1.5">
				<Label>URL</Label>
				<Input bind:value={datasources[selected].url} />

				<p class="text-sm text-muted-foreground">
					The URI your datasource will be accessible from. You can provide a prefix or bucket (eg
					{datasources[selected].url}my-bucket)
				</p>
			</div>
			<Label>Options</Label>

			<Card.Root>
				<div class="m-2">
					<svelte:component this={datasources[selected].options} bind:this={options} />
				</div>
			</Card.Root>
		</div>
		<Dialog.Footer>
			<Button on:click={cancel}>Cancel</Button>
			<Button on:click={closeDialog}>Create</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
