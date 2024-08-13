<script lang="ts">
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import Icon from '@iconify/svelte';

	import type { Database, LogicalDataType } from '$lib/lens/types';

	let { databases }: { databases: Database[] } = $props();

	const logicalTypeIcons: Record<LogicalDataType, string> = {
		string: 'carbon:string-text',
		boolean: 'carbon:boolean',
		null: 'mdi:null',
		integer: 'carbon:string-integer',
		decimal: 'carbon:character-decimal',
		timestamp: 'carbon:time',
		date: 'carbon:calendar',
		time: 'carbon:time'
	};
</script>

<Collapsible.Root>
	{#each databases as { name, schemas }}
		{@render trigger({ icon: 'carbon:db2-database', label: name })}

		<Collapsible.Content>
			{#each schemas as { name, tables }}
				<Collapsible.Root class="w-full pl-5">
					{@render trigger({ icon: 'material-symbols-light:schema-outline', label: name })}
					<Collapsible.Content>
						{#each tables as { name, schema: { fields } }}
							<Collapsible.Root class="w-full pl-5">
								{@render trigger({ icon: 'carbon:data-table', label: name })}

								<Collapsible.Content>
									<div class="pl-5">
										{#each fields as { name, data_type: { kind, logical } }}
											<Button variant="ghost" size="sm" class="flex w-full justify-start gap-1">
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

{#snippet trigger({ icon, label }: { icon: string; label: string })}
	<Collapsible.Trigger asChild let:builder>
		<Button builders={[builder]} variant="ghost" class="flex w-full justify-start gap-1">
			<Icon icon="mdi:caret" class="rotate-90" width={24} height={24} />
			<Icon {icon} width={20} height={20} />
			{label}
		</Button>
	</Collapsible.Trigger>
{/snippet}
