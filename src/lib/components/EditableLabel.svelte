<script lang="ts" context="module">
	export type EditMode = 'show' | 'edit';
</script>

<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { cn } from '$lib/utils';

	interface Props {
		value?: string;
		mode?: EditMode;
		labelClass?: string;
		inputClass?: string;
		onToggle?: (mode: EditMode) => void;
	}

	let {
		value = $bindable(),
		mode = $bindable('show'),
		labelClass = '',
		inputClass = '',
		onToggle = undefined
	}: Props = $props();

	export function toggleEdit() {
		if (mode === 'show') {
			mode = 'edit';
		} else {
			mode = 'show';
		}

		if (onToggle) onToggle(mode);
	}

	function handleKeyDown(ev: KeyboardEvent) {
		if (ev.code === 'Enter') toggleEdit();
	}

	function focus(input: HTMLInputElement) {
		input.focus();
	}
</script>

{#if mode === 'show'}
	<Label class={labelClass}>{value}</Label>
{:else if mode === 'edit'}
	<input
		bind:value
		class={cn('border-input bg-background text-sm ring-offset-background', inputClass)}
		onkeydown={handleKeyDown}
		use:focus
	/>
{/if}
