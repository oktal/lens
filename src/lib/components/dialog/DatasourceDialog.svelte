<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as ToggleGroup from '$lib/components/ui/toggle-group';
	import * as Card from '$lib/components/ui/card';
	import { Label } from '$lib/components/ui/label/index.js';
	import type { AmazonS3Config, GoogleCloudStorageConfig, DatasourceConfig } from '$lib/lens/types';
	import { Button } from '../ui/button';
	import Icon from '@iconify/svelte';
	import { Input } from '../ui/input';
	import type { Component } from 'svelte';
	import AmazonS3Options from './AmazonS3Options.svelte';

	export function show(): Promise<DatasourceConfig> {
		return new Promise<DatasourceConfig>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;
			open = true;
		});
	}

	type DataSource = 's3' | 'gcs';

	let amazonS3Config: AmazonS3Config = {
		accessKeyId: '',
		secretAccessKey: '',
		bucket: '',
		region: ''
	};

	let gcsConfig: GoogleCloudStorageConfig = {
		serviceAccountPath: '',
		serviceAccountKey: '',
		applicationCredentialsPath: ''
	};

	let datasources: Record<
		DataSource,
		{
			label: string;
			icon: string;
			url: string;
			options: Component<{ config: AmazonS3Config | GoogleCloudStorageConfig }>;
			config: AmazonS3Config | GoogleCloudStorageConfig;
		}
	> = {
		s3: {
			label: 'Amazon S3',
			icon: 'mdi:aws',
			url: 's3://',
			options: AmazonS3Options,
			config: amazonS3Config
		},
		gcs: {
			label: 'Google Cloud Storage',
			icon: 'mdi:google',
			url: 'gcp://',
			config: gcsConfig
		}
	};

	let open = $state(false);
	let selected: DataSource = $state('s3');

	let accept_: (config: DatasourceConfig) => void;
	let reject_: () => void;

	function closeDialog() {
		open = false;
		const datasource = datasources[selected];
		let storeConfig: any = {};
		storeConfig[selected] = datasource.config;

		const config: DatasourceConfig = {
			url: datasource.url,
			store: storeConfig
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
			<Dialog.Description>Create a new data source</Dialog.Description>
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
					<svelte:component
						this={datasources[selected].options}
						config={datasources[selected].config}
					/>
				</div>
			</Card.Root>
		</div>
		<Dialog.Footer>
			<Button on:click={cancel}>Cancel</Button>
			<Button on:click={closeDialog}>Create</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
