<script lang="ts" context="module">
	export type Mode = 'light' | 'dark';
</script>

<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import Icon from '@iconify/svelte';

	interface Props {
		onToggle?: (mode: Mode) => void;
		onReset?: () => void;
	}

	let { onToggle, onReset }: Props = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button builders={[builder]} variant="ghost" size="icon">
			<Icon
				icon="carbon:sun"
				class="rotate-0 scale-100 transition-all duration-300 dark:-rotate-90 dark:scale-0"
				width={20}
				height={20}
			/>
			<Icon
				icon="carbon:moon"
				class="absolute rotate-90 scale-0 transition-all duration-300 dark:rotate-0 dark:scale-100"
				width={20}
				height={20}
			/>
			<span class="sr-only">Toggle theme</span>
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end">
		<DropdownMenu.Item class="flex gap-1" on:click={() => onToggle && onToggle('light')}>
			<Icon icon="carbon:sun" width={18} height={18} />
			Light
		</DropdownMenu.Item>
		<DropdownMenu.Item class="flex gap-1" on:click={() => onToggle && onToggle('dark')}>
			<Icon icon="carbon:moon" width={18} height={18} />
			Dark
		</DropdownMenu.Item>
		<DropdownMenu.Item class="flex gap-1" on:click={() => onReset && onReset()}>
			<Icon icon="carbon:brightness-contrast" width={18} height={18} />
			System
		</DropdownMenu.Item>
	</DropdownMenu.Content>
</DropdownMenu.Root>
