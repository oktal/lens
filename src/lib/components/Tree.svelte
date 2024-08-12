<script lang="ts">
	import * as Collapsible from '$lib/components/ui/collapsible/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import Icon from '@iconify/svelte';
</script>

<Collapsible.Root>
	<Collapsible.Trigger asChild let:builder>
		<Button builders={[builder]} variant="ghost" class="flex w-full justify-start gap-1">
			<Icon icon="mdi:caret" class="rotate-90" width={24} height={24} />
			<Icon icon="carbon:db2-database" width={20} height={20} />
			taxis
		</Button>
	</Collapsible.Trigger>
	<Collapsible.Content>
		<Collapsible.Root class="w-full pl-5">
			<Collapsible.Trigger asChild let:builder>
				<Button builders={[builder]} variant="ghost" class="flex w-full justify-start gap-1">
					<Icon icon="mdi:caret" class="rotate-90" width={24} height={24} />
					<Icon icon="material-symbols-light:schema-outline" width={20} height={20} />
					v1
				</Button>
			</Collapsible.Trigger>
			<Collapsible.Content>
				<Collapsible.Root class="w-full pl-5">
					<Collapsible.Trigger asChild let:builder>
						<Button builders={[builder]} variant="ghost" class="flex w-full justify-start gap-1">
							<Icon icon="mdi:caret" class="rotate-90" width={24} height={24} />
							<Icon icon="carbon:data-table" width={20} height={20} />
							taxis
						</Button>
					</Collapsible.Trigger>
					<Collapsible.Content></Collapsible.Content>
				</Collapsible.Root>
			</Collapsible.Content>
		</Collapsible.Root>
	</Collapsible.Content>
</Collapsible.Root>
