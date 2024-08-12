<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Icon from '@iconify/svelte';

	import DatabaseDialog from '$lib/components/dialog/DatabaseDialog.svelte';
	import Tree from './Tree.svelte';
	import Button from './ui/button/button.svelte';

	let databaseDialog: DatabaseDialog;

	async function createDatabase() {
		const database = await databaseDialog.show();
		console.log('Create database ' + database);
	}

	type MenuItem = {
		label: string;
		icon: string;
		onClick: () => void;
	};

	const addMenuItems: MenuItem[] = [
		{
			label: 'Database',
			icon: 'carbon:db2-database',
			onClick: createDatabase
		},
		{
			label: 'Schema',
			icon: 'material-symbols-light:schema-outline'
		},
		{
			label: 'Table',
			icon: 'carbon:data-table'
		}
	];
</script>

<div class="flex flex-col gap-1">
	<div class="place-self-end px-2 py-2">
		{@render addMenu(addMenuItems)}
	</div>

	<Tree />
</div>

<DatabaseDialog bind:this={databaseDialog} />

{#snippet addMenu(items: MenuItem[])}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger>
			<Button variant="default" size="icon">
				<Icon icon="carbon:add" width={24} height={24} /></Button
			></DropdownMenu.Trigger
		>
		<DropdownMenu.Content>
			<DropdownMenu.Group>
				{#each items as { label, icon, onClick }}
					<DropdownMenu.Item class="flex gap-1" on:click={onClick}>
						<Icon {icon} width={20} height={20} />
						{label}
					</DropdownMenu.Item>
				{/each}
			</DropdownMenu.Group>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
{/snippet}
