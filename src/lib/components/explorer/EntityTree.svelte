<script lang="ts" context="module">
	export type FileDropProperties =
		| {
				level: 'database';
				database: string;
		  }
		| { level: 'schema'; database: string; schema: string };

	export type FileDropEvent = {
		filePath: string;
		detail: FileDropProperties;
	};
</script>

<script lang="ts">
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import Icon from '@iconify/svelte';

	import type { Database, LogicalDataType } from '$lib/lens/types';
	import { SvelteMap } from 'svelte/reactivity';
	import type { DropEvent } from '$lib/dropevent';
	import { mode } from 'mode-watcher';

	let {
		databases,
		onFileDropped = undefined
	}: { databases: Database[]; onFileDropped?: (ev: FileDropEvent) => void } = $props();

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

	function handleDropEvent(ev: DropEvent, properties: FileDropProperties) {
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
							detail: properties
						});
					}
				}
			} else if (item.kind === 'string') {
				item.getAsString((file: string) => {
					if (onFileDropped) {
						onFileDropped({
							filePath: file,
							detail: properties
						});
					}
				});
			}
		}
	}
</script>

<Collapsible.Root>
	{#each databases as { name: databaseName, schemas }}
		{@render trigger({
			id: `database-${databaseName}`,
			icon: 'carbon:db2-database',
			label: databaseName,
			dropProperties: {
				level: 'database',
				database: databaseName
			}
		})}

		<Collapsible.Content>
			{#each schemas as { name: schemaName, tables }}
				<Collapsible.Root class="w-full pl-5">
					{@render trigger({
						id: `schema-${name}`,
						icon: 'material-symbols-light:schema-outline',
						label: schemaName,
						dropProperties: {
							level: 'schema',
							database: databaseName,
							schema: schemaName
						}
					})}
					<Collapsible.Content>
						{#each tables as { name, schema: { fields } }}
							<Collapsible.Root class="w-full pl-5">
								{@render trigger({
									id: `table-${name}`,
									icon: 'carbon:data-table',
									label: name,
									dropProperties: {
										level: 'schema',
										database: databaseName,
										schema: schemaName
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
	{/each}
</Collapsible.Root>

{#snippet trigger({
	id,
	icon,
	label,
	dropProperties
}: {
	id: string;
	icon: string;
	label: string;
	dropProperties: FileDropProperties;
})}
	<Collapsible.Trigger asChild let:builder>
		<Button
			builders={[builder]}
			variant="ghost"
			class="flex w-full justify-start gap-1"
			on:click={() => toggleExpanded(id)}
			ondrop={(e: DropEvent) => handleDropEvent(e, dropProperties)}
			ondragenter={handleDragEnter}
			ondragleave={handleDragLeave}
		>
			{@const state = expandMap.get(id)?.expanded ? 'expanded' : 'collapsed'}
			{@render carret({ state })}
			<Icon {icon} width={20} height={20} />
			{label}
		</Button>
	</Collapsible.Trigger>
{/snippet}

{#snippet carret({ state }: { state: 'expanded' | 'collapsed' })}
	{@const rotation = state === 'collapsed' ? 'rotate-90' : 'rotate-180'}
	<Icon icon="mdi:caret" class="duration-100 ease-in-out {rotation}" width={24} height={24} />
{/snippet}
