<script lang="ts">
	type Props = {
		value?: number,
		min?: number,
		max?: number,

		height?: number,
	}

	let { value = undefined, min = 0, max = 100, height = 2 }: Props = $props();

	let fillPercent = $derived(value ? (100 * (value - min)) / (max - min) : 0);
	let indeterminate = $derived(value === undefined || value < 0);
	let indeterminateClass = $derived(indeterminate ? 'anim-indeterminate': '');

	const heightClass = `h-${height}`;
</script>

<!-- Track -->
<div
	class="progress-bar w-full overflow-hidden bg-secondary {heightClass} rounded-full"
	data-testid="progress-bar"
	role="progressbar"
	aria-valuenow={value}
	aria-valuemin={min}
	aria-valuemax={max - min}
>
	<!-- Meter -->
	<div class="progress-bar-meter h-full bg-primary rounded-full {indeterminateClass} transition-[width]" style:width="{indeterminate ? 100 : fillPercent}%"></div>
</div>

<style lang="postcss">
	.anim-indeterminate {
		transform-origin: 0% 50%;
		animation: anim-indeterminate 2s infinite linear;
	}
	/* prettier-ignore */
	@keyframes anim-indeterminate {
		0% { transform: translateX(0) scaleX(0); }
		40% { transform: translateX(0) scaleX(0.4); }
		100% { transform: translateX(100%) scaleX(0.5); }
	}
</style>