<script lang="ts">
	import type { Snippet } from 'svelte';
	import { type Dimension } from './types';
	import type { HTMLAttributes } from 'svelte/elements';

	type Template = number | Dimension[];

	interface Props extends HTMLAttributes<HTMLDivElement> {
		cols?: Template;
		rows?: Template;
		children: Snippet;
	}

	let { cols, rows, children, ...restProps }: Props = $props();

	function createTemplate(template: Template | undefined): string {
		if (typeof template === 'undefined') return 'none';
		if (typeof template === 'number') return `repeat(${template}, minmax(0, 1fr))`;

		if (Array.isArray(template)) {
			return template.join(' ');
		}

		return 'none';
	}

	const styleToString = (style: Record<string, number | string | undefined>): string => {
		return Object.keys(style).reduce((str, key) => {
			if (style[key] === undefined) return str;
			return str + `${key}:${style[key]};`;
		}, '');
	};

	const style = {
		display: 'grid',
		'grid-template-columns': createTemplate(cols),
		'grid-template-rows': createTemplate(rows)
	};
</script>

<div style={styleToString(style)} {...restProps}>
	{@render children()}
</div>
