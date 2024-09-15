<script lang="ts" context="module">
	export type NodeContext =
		| {
				kind: 'database';
				database: string;
		  }
		| { kind: 'schema'; database: string; schema: string }
		| { kind: 'table'; database: string; schema: string; table: string };

	export type FileDropEvent = {
		filePath: string;
		context: NodeContext;
	};
</script>

<script lang="ts">
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import * as ContextMenu from '$lib/components/ui/context-menu';
	import { Button } from '$lib/components/ui/button/index.js';
	import Icon from '@iconify/svelte';

	import type { Database, LogicalDataType } from '$lib/lens/types';
	import { SvelteMap } from 'svelte/reactivity';
	import type { DropEvent } from '$lib/dropevent';
	import { mode } from 'mode-watcher';

	type NodeType = 'database' | 'schema' | 'table';

	let {
		databases,
		onCreate = undefined,
		onDrop = undefined,
		onFileDropped = undefined
	}: {
		databases: Database[];
		onCreate?: (ctx: NodeContext) => void;
		onDrop?: (ctx: NodeContext) => void;
		onFileDropped?: (ev: FileDropEvent) => void;
	} = $props();

	const logicalTypeIcons: Record<LogicalDataType, string> = {
		string: 'carbon:string-text',
		boolean: 'carbon:boolean',
		null: 'mdi:null',
		integer: 'carbon:string-integer',
		decimal: 'carbon:character-decimal',
		timestamp: 'carbon:time',
		date: 'carbon:calendar',
		time: 'carbon:time',
		dictionary: 'carbon:book'
	};

	type NodeProperties = {
		icon: string;
		parent: NodeType | undefined;
		child: NodeType | undefined;
		droppable: boolean;
	};

	const nodes: Record<NodeType, NodeProperties> = {
		database: {
			icon: 'carbon:db2-database',
			parent: undefined,
			child: 'schema',
			droppable: false
		},
		schema: {
			icon: 'carbon:merge',
			parent: 'database',
			child: 'table',
			droppable: true
		},
		table: {
			icon: 'carbon:data-table',
			parent: 'schema',
			child: undefined,
			droppable: true
		}
	};

	let expandMap = new SvelteMap<
		string,
		{
			get expanded(): boolean;
			set expanded(val: boolean);
		}
	>();

	let dragHoverClass = $derived($mode === 'light' ? 'draghover' : 'draghover-dark');

	function toggleExpanded(id: string) {
		if (!expandMap.has(id)) {
			let expanded = $state(true);
			expandMap.set(id, {
				get expanded(): boolean {
					return expanded;
				},
				set expanded(val: boolean) {
					expanded = val;
				}
			});
		} else {
			let state = expandMap.get(id)!;
			state.expanded = !state.expanded;
		}
	}

	function handleDragEnter(ev: DragEvent) {
		ev.preventDefault();

		if (ev.target instanceof Element) {
			ev.target.classList.add(dragHoverClass);
		}
	}

	function handleDragLeave(ev: DragEvent) {
		ev.preventDefault();

		if (ev.target instanceof Element) {
			ev.target.classList.remove(dragHoverClass);
		}
	}

	function handleDropEvent(ev: DropEvent, context: NodeContext) {
		ev.preventDefault();
		if (!ev.dataTransfer) return;

		if (ev.dataTransfer.items) {
			const item = ev.dataTransfer.items[0];

			if (item.kind === 'file') {
				const file = item.getAsFile();
				if (file) {
					if (onFileDropped) {
						onFileDropped({
							filePath: file.name,
							context
						});
					}
				}
			} else if (item.kind === 'string') {
				item.getAsString((file: string) => {
					if (onFileDropped) {
						onFileDropped({
							filePath: file,
							context
						});
					}
				});
			}
		}
	}
</script>

{#each databases as { name: databaseName, schemas }}
	<Collapsible.Root>
		{@render trigger({
			nodeType: 'database',
			label: databaseName,
			context: {
				kind: 'database',
				database: databaseName
			}
		})}

		<Collapsible.Content>
			{#each schemas as { name: schemaName, tables }}
				<Collapsible.Root class="w-full pl-5">
					{@render trigger({
						nodeType: 'schema',
						label: schemaName,
						context: {
							kind: 'schema',
							database: databaseName,
							schema: schemaName
						}
					})}
					<Collapsible.Content>
						{#each tables as { name, schema: { fields } }}
							<Collapsible.Root class="w-full pl-5">
								{@render trigger({
									nodeType: 'table',
									label: name,
									context: {
										kind: 'table',
										database: databaseName,
										schema: schemaName,
										table: name
									}
								})}

								<Collapsible.Content>
									<div class="pl-5">
										{#each fields as { name, data_type: { kind, logical } }}
											<Button
												variant="ghost"
												size="sm"
												class="flex h-8 w-full items-center justify-start gap-1"
											>
												<Icon icon={logicalTypeIcons[logical]} width={18} height={18} />
												{name}
												<span class="ml-auto mr-5 text-xs font-bold text-gray-400">
													{kind}
												</span>
											</Button>
										{/each}
									</div>
								</Collapsible.Content>
							</Collapsible.Root>
						{/each}
					</Collapsible.Content>
				</Collapsible.Root>
			{/each}
		</Collapsible.Content>
	</Collapsible.Root>
{/each}

{#snippet trigger({
	nodeType,
	label,
	context
}: {
	nodeType: NodeType;
	label: string;
	context: NodeContext;
})}
	{@const id = `${nodeType}-${label}`}
	{@const icon = nodes[context.kind].icon}
	<Collapsible.Trigger asChild let:builder>
		<ContextMenu.Root>
			<ContextMenu.Trigger>
				<Button
					builders={[builder]}
					variant="ghost"
					class="flex w-full justify-start gap-1"
					on:click={() => toggleExpanded(id)}
					ondragenter={handleDragEnter}
					ondragleave={handleDragLeave}
					ondrop={(ev: DropEvent) => handleDropEvent(ev, context)}
				>
					{@const state = expandMap.get(id)?.expanded ? 'expanded' : 'collapsed'}
					{@render carret({ state })}
					<Icon {icon} width={20} height={20} />
					{label}
				</Button>
			</ContextMenu.Trigger>
			<ContextMenu.Content>
				{#if nodes[nodeType].child}
					{@const child = nodes[nodeType].child}
					<ContextMenu.Item class="flex gap-1" onclick={() => onCreate && onCreate(context)}>
						<Icon icon={nodes[child].icon} />
						New {child}
					</ContextMenu.Item>
				{/if}
				<ContextMenu.Item
					disabled={!nodes[context.kind].droppable}
					class="flex gap-1 text-red-700"
					onclick={() => onDrop && onDrop(context)}
				>
					<Icon icon="carbon:trash-can" />
					Delete
				</ContextMenu.Item>
			</ContextMenu.Content>
		</ContextMenu.Root>
	</Collapsible.Trigger>
{/snippet}

{#snippet carret({ state }: { state: 'expanded' | 'collapsed' })}
	{@const rotation = state === 'collapsed' ? 'rotate-90' : 'rotate-180'}
	<Icon icon="mdi:caret" class="duration-100 ease-in-out {rotation}" width={24} height={24} />
{/snippet}
