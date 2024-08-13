<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Icon from '@iconify/svelte';
	import { client } from '$lib/lens/api';
	import type { Database } from '$lib/lens/types';

	import DatabaseDialog from '$lib/components/dialog/DatabaseDialog.svelte';
	import Tree from './Tree.svelte';
	import Button from './ui/button/button.svelte';
	import { toast } from 'svelte-sonner';
	import SchemaDialog from './dialog/SchemaDialog.svelte';
	import TableDialog from './dialog/TableDialog.svelte';
	import { mount } from 'svelte';

	let databases: Database[] = $state([]);

	$effect(() => {
		client.list.databases().then((dbs: Database[]) => (databases = dbs));
	});

	async function createDatabase() {
		const dialog = mount(DatabaseDialog, {
			target: document.body
		});

		const database = await dialog.show();
		const sql = `CREATE DATABASE ${database}`;

		try {
			await client.sql.run(sql);
			toast.success(`Database ${database} successfully created`);
		} catch (e) {
			toast.error(`Failed to create database ${database}: ${e}`);
		}

		databases = await client.list.databases();
	}

	async function createSchema() {
		const databaseNames = databases.map((db: Database) => db.name);

		const dialog = mount(SchemaDialog, {
			target: document.body,
			props: {
				databaseNames
			}
		});
		const { database, name } = await dialog.show();

		const schemaRef = `${database}.${name}`;
		const query = `CREATE SCHEMA ${schemaRef}`;
		try {
			await client.sql.run(query);
			toast.success(`Schema ${schemaRef} has been created`);
		} catch (e) {
			toast.error(`Failed to create schema ${schemaRef}: ${e}`);
		}

		databases = await client.list.databases();
	}

	async function createTable() {
		const datasources = await client.list.datasources();

		const dialog = mount(TableDialog, {
			target: document.body,
			props: {
				databases,
				datasources
			}
		});

		const table = await dialog.show();
		const tableRef = `${table.database}.${table.schema}.${table.name}`;

		let query = `CREATE EXTERNAL TABLE ${tableRef} STORED AS ${table.fileType}`;

		if (table.partitions.length > 0) {
			query += ` PARTITIONED BY (${table.partitions.join(',')})`;
		}

		query += ` LOCATION '${table.location}'`;

		try {
			await client.sql.run(query);
			toast.success(`Table ${tableRef} has been created`);
		} catch (e) {
			toast.error(`Error creating table ${tableRef}: ${e}`);
		}

		databases = await client.list.databases();
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
			icon: 'material-symbols-light:schema-outline',
			onClick: createSchema
		},
		{
			label: 'Table',
			icon: 'carbon:data-table',
			onClick: createTable
		}
	];
</script>

<div class="flex flex-col gap-1">
	<div class="place-self-end px-2 py-2">
		{@render addMenu(addMenuItems)}
	</div>

	<Tree {databases} />
</div>

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
