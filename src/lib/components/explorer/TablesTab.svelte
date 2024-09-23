<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Button from '$lib/components/ui/button/button.svelte';

	import Icon from '@iconify/svelte';
	import { client } from '$lib/lens/api';
	import { FileTypes, type Database, type FileType } from '$lib/lens/types';

	import DatabaseDialog from '$lib/components/dialog/DatabaseDialog.svelte';
	import { Progress } from '$lib/components/ui/progress';
	import SchemaDialog, { type SchemaInfo } from '$lib/components/dialog/SchemaDialog.svelte';
	import TableDialog, { type TableInfo } from '$lib/components/dialog/table/TableDialog.svelte';
	import EntityTree, { type FileDropEvent, type NodeContext } from './EntityTree.svelte';

	import { toast } from 'svelte-sonner';
	import { mount } from 'svelte';

	let databases = $state<Database[]>([]);
	let loadPromise = $state<Promise<void>>(Promise.resolve());

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

	async function createSchema(info?: Partial<SchemaInfo>) {
		const databaseNames = databases.map((db: Database) => db.name);

		const dialog = mount(SchemaDialog, {
			target: document.body,
			props: {
				databaseNames,
				info
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

	async function createTable(info?: Partial<TableInfo>) {
		const datasources = await client.list.datasources();

		const dialog = mount(TableDialog, {
			target: document.body,
			props: {
				databases,
				datasources,
				info
			}
		});

		const table = await dialog.show();
		const tableRef = `${table.database}.${table.schema}.${table.name}`;

		let query = `CREATE EXTERNAL TABLE ${tableRef}`;
		const partitionColumnDefs = table.partitions
			.filter((p) => p.type !== undefined)
			.map((p) => {
				return `${p.name} ${p.type}`;
			})
			.join(', ');

		if (partitionColumnDefs.length > 0) {
			query += `(${partitionColumnDefs})`;
		}

		query += ` STORED AS ${table.fileType}`;

		if (table.partitions.length > 0) {
			const partitionDefs = table.partitions.map((p) => p.name).join(',');
			query += ` PARTITIONED BY (${partitionDefs})`;
		}

		query += ` LOCATION '${table.location}'`;

		const options = Object.keys(table.options).map((key: string) => {
			const value = table.options[key];
			return `'${key}' '${value}'`;
		});

		if (options.length > 0) {
			query += ` OPTIONS (${options.join(', ')})`;
		}

		loadPromise = new Promise<void>(async (accept, _reject) => {
			try {
				await client.sql.run(query);
				toast.success(`Table ${tableRef} has been created`);
			} catch (e) {
				toast.error(`Error creating table ${tableRef}: ${e}`);
			}

			databases = await client.list.databases();
			accept();
		});
	}

	async function handleCreate(ctx: NodeContext) {
		if (ctx.kind === 'database') {
			await createSchema({ database: ctx.database });
		} else if (ctx.kind === 'schema') {
			const { database, schema } = ctx;
			await createTable({
				database,
				schema
			});
		}
	}

	async function handleDrop(ctx: NodeContext) {
		const formatDropStatement = (): string => {
			if (ctx.kind === 'database') {
				return `drop database ${ctx.database}`;
			} else if (ctx.kind === 'schema') {
				return `drop schema ${ctx.database}.${ctx.schema}`;
			} else if (ctx.kind === 'table') {
				return `drop table ${ctx.database}.${ctx.schema}.${ctx.table}`;
			}

			return '';
		};

		loadPromise = new Promise<void>(async (accept, _reject) => {
			try {
				const statement = formatDropStatement();
				await client.sql.run(statement);
			} catch (e) {
				toast.error(`Error dropping: ${e}`);
			}

			databases = await client.list.databases();
			accept();
		});
	}

	async function handleFileDropped(ev: FileDropEvent) {
		const parseFilePath = (filePath: string): { baseName: string; extension: string } => {
			const pathSeparator = filePath.includes('\\') ? '\\' : '/';
			const fileNameWithExtension = filePath.split(pathSeparator).pop() || '';

			const lastDotIndex = fileNameWithExtension.lastIndexOf('.');
			if (lastDotIndex === -1) {
				return { baseName: fileNameWithExtension, extension: '' };
			}

			const baseName = fileNameWithExtension.slice(0, lastDotIndex);
			const extension = fileNameWithExtension.slice(lastDotIndex);

			return { baseName, extension };
		};

		const getTableBaseInfo = (): Partial<TableInfo> => {
			const context = ev.context;
			if (context.kind === 'database') {
				const { database } = context;
				return { database };
			} else if (context.kind === 'schema' || context.kind === 'table') {
				const { database, schema } = context;
				return { database, schema };
			}

			return {};
		};

		const isKnownFileType = (extension: string): extension is FileType => {
			return FileTypes.includes(extension);
		};

		const { baseName, extension } = parseFilePath(ev.filePath);
		const fileType = extension.slice(1);

		const tableInfo = {
			...getTableBaseInfo(),
			name: baseName,
			location: ev.filePath,
			fileType: isKnownFileType(fileType) ? fileType : undefined
		};

		await createTable(tableInfo);
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
			icon: 'carbon:merge',
			onClick: () => createSchema(undefined)
		},
		{
			label: 'Table',
			icon: 'carbon:data-table',
			onClick: () => createTable(undefined)
		}
	];

	$effect(() => {
		client.list.databases().then((dbs: Database[]) => (databases = dbs));
	});
</script>

<div class="flex flex-col gap-1">
	<div class="flex">
		{@render addMenu(addMenuItems)}
	</div>

	{#await loadPromise}
		<Progress value={undefined} />
	{/await}
	<EntityTree
		{databases}
		onCreate={handleCreate}
		onDrop={handleDrop}
		onFileDropped={handleFileDropped}
	/>
</div>

{#snippet addMenu(items: MenuItem[])}
	<DropdownMenu.Root>
		<DropdownMenu.Trigger>
			<Button variant="secondary" size="sm" class="flex gap-1">
				<Icon icon="carbon:document-add" width={22} height={22} />
				Create
			</Button>
		</DropdownMenu.Trigger>
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
