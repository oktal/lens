<script lang="ts" context="module">
	export type SchemaInfo = {
		database: string;
		name: string;
	};
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import * as Select from '$lib/components/ui/select';

	export function show(): Promise<SchemaInfo> {
		return new Promise<SchemaInfo>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;
			open = true;
		});
	}

	interface Props {
		databaseNames: string[];
		info?: Partial<SchemaInfo>;
	}

	let { databaseNames, info = undefined }: Props = $props();
	const databaseItems = databaseNames.map((database) => {
		return {
			label: database,
			value: database
		};
	});

	let open = $state(false);
	let database = $state(info?.database ?? '');
	let name = $state('');

	let accept_: ((info: SchemaInfo) => void) | undefined = undefined;
	let reject_: ((reason?: any) => void) | undefined = undefined;

	function closeDialog() {
		open = false;
		if (accept_) accept_({ database, name });
	}

	function cancel() {
		open = false;
		if (reject_) reject_('dialog has been cancelled by user');
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Create schema</Dialog.Title>
			<Dialog.Description>Create a schema under a specified database</Dialog.Description>
		</Dialog.Header>
		<div class="grid grid-cols-3 gap-4 py-4">
			<Label class="shrink">Database</Label>
			<Select.Root
				selected={{ label: database, value: database }}
				items={databaseItems}
				onSelectedChange={(v) => v && (database = v.value)}
			>
				<Select.Trigger class="col-span-2">
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

			<Label for="name">Name</Label>
			<Input id="name" class="col-span-2" bind:value={name} />
		</div>
		<Dialog.Footer>
			<Button on:click={cancel}>Cancel</Button>
			<Button on:click={closeDialog}>Create</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
