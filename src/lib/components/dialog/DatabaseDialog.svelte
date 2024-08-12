<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';

	export function show(): Promise<string> {
		return new Promise<string>((accept, reject) => {
			accept_ = accept;
			reject_ = reject;
			open = true;
		});
	}

	let open = false;
	let name: string = '';

	let accept_: ((databse: string) => void) | undefined = undefined;
	let reject_: ((reason?: any) => void) | undefined = undefined;

	function closeDialog() {
		open = false;
		if (accept_) accept_(name);
	}

	function cancel() {
		open = false;
		if (reject_) reject_('dialog has been cancelled by user');
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="sm:max-w-[425px]">
		<Dialog.Header>
			<Dialog.Title>Create database</Dialog.Title>
			<Dialog.Description>Create a new database</Dialog.Description>
		</Dialog.Header>
		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-4 items-center gap-4">
				<Label for="name" class="text-right">Name</Label>
				<Input id="name" class="col-span-3" bind:value={name} />
			</div>
		</div>
		<Dialog.Footer>
			<Button on:click={cancel}>Cancel</Button>
			<Button on:click={closeDialog}>Save</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

