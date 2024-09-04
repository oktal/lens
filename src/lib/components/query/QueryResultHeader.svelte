<script lang="ts">
	import type { Header } from '@tanstack/svelte-table';
	import Button from '$lib/components/ui/button/button.svelte';
	import Icon from '@iconify/svelte';

	type Props = {
		label?: string;
		header: Header<string[], unknown>;
	};

	const { label, header }: Props = $props();
</script>

{#if header.column.getCanSort()}
	<Button variant="ghost" class="flex gap-1" on:click={() => header.column.toggleSorting()}>
		{label ?? header.column.id}
		{#if header.column.getNextSortingOrder().toString() === 'asc' || header.column
				.getIsSorted()
				.toString() === 'asc'}
			<Icon icon="carbon:arrow-up" />
		{:else if header.column.getNextSortingOrder().toString() === 'desc' || header.column
				.getIsSorted()
				.toString() === 'desc'}
			<Icon icon="carbon:arrow-down" />
		{/if}
	</Button>
{:else}
	<span class="px-4">{label ?? header.column.id}</span>
{/if}
