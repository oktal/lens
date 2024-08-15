<script lang="ts">
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Select from '$lib/components/ui/select';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { ScrollArea } from '$lib/components/ui/scroll-area';
	import type { AmazonS3Config } from '$lib/lens/types';
	import awsRegions from 'aws-regions';
	import Icon from '@iconify/svelte';

	let { config }: { config: AmazonS3Config } = $props();

	let ssoStartUrl: string = $state('');
	let ssoAccountId: string = $state('');
	let ssoRole: string = $state('');

	const regions = awsRegions.list().filter((r) => r.public);
	const regionItems = regions.map((region) => {
		return {
			value: region,
			label: `${region.code}`
		};
	});

	const regionFlags: Record<string, string> = {
		'us-east-1': 'openmoji:flag-united-states',
		'us-east-2': 'openmoji:flag-united-states',
		'us-west-1': 'openmoji:flag-united-states',
		'us-west-2': 'openmoji:flag-united-states',
		'ca-central-1': 'openmoji:flag-canada',
		'eu-north-1': 'openmoji:flag-sweden',
		'eu-west-1': 'openmoji:flag-ireland',
		'eu-west-2': 'openmoji:flag-england',
		'eu-west-3': 'openmoji:flag-france',
		'eu-central-1': 'openmoji:flag-germany',
		'eu-south-1': 'openmoji:flag-italy',
		'af-south-1': 'openmoji:flag-south-africa',
		'ap-northeast-1': 'openmoji:flag-japan',
		'ap-northeast-2': 'openmoji:flag-south-korea',
		'ap-northeast-3': 'openmoji:flag-japan',
		'ap-southeast-1': 'openmoji:flag-singapore',
		'ap-southeast-2': 'openmoji:flag-australia',
		'ap-southeast-3': 'openmoji:flag-indonesia',
		'ap-east-1': 'openmoji:flag-hong-kong-sar-china',
		'ap-south-1': 'openmoji:flag-india',
		'sa-east-1': 'openmoji:flag-brazil',
		'me-south-1': 'openmoji:flag-bahrain',
		'cn-north-1': 'openmoji:flag-china',
		'cn-northwest-1': 'openmoji:flag-china'
	};
</script>

<div class="flex w-full flex-col gap-2">
	<Label>Authentication</Label>
	<Tabs.Root value="basic" class="ml-2 mt-2 h-full">
		<Tabs.List>
			<Tabs.Trigger value="basic">Basic</Tabs.Trigger>
			<Tabs.Trigger value="sso">SSO</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="basic">
			<div class="flex flex-col gap-1">
				<Input required bind:value={config.accessKeyId} placeholder="Access key id" />
				<Input required bind:value={config.secretAccessKey} placeholder="Secret access key" />
			</div>
		</Tabs.Content>
		<Tabs.Content value="sso">
			<div class="flex flex-col gap-1">
				<Input required bind:value={ssoStartUrl} placeholder="Start URL" />
				<Input required bind:value={ssoAccountId} placeholder="Account ID" />
				<Input required bind:value={ssoRole} placeholder="Role" />
			</div>
		</Tabs.Content>
	</Tabs.Root>

	<Select.Root items={regionItems} onSelectedChange={(v) => v && (config.region = v.value.code)}>
		<Select.Trigger>
			<Select.Value placeholder="Region" />
		</Select.Trigger>
		<Select.Content>
			<ScrollArea class="h-96">
				{#each regionItems as { value, label }}
					<Select.Item {value} class="flex gap-1">
						<Icon icon={regionFlags[value.code]} width={24} height={24} />
						<span>{value.code}</span>
						<span class="ml-auto mr-2 text-xs font-medium">{value.name}</span>
					</Select.Item>
				{/each}
			</ScrollArea>
		</Select.Content>
	</Select.Root>

	<Input bind:value={config.bucket} placeholder="Bucket" />
</div>
