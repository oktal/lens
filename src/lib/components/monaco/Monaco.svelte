<script lang="ts">
	import type Monaco from 'monaco-editor';
	import monaco from '$lib/monaco';
	import { defineThemes, setTheme } from './themes';
	import { onDestroy, onMount } from 'svelte';
	import { createEventDispatcher } from 'svelte';
	import type { HTMLAttributes } from 'svelte/elements';

	interface Props extends HTMLAttributes<HTMLDivElement> {
		editor?: Monaco.editor.IStandaloneCodeEditor;
		value: string;

		language?: string;
		theme?: string;
		options?: Monaco.editor.IStandaloneEditorConstructionOptions;

		onReady?: (editor: Monaco.editor.IStandaloneCodeEditor) => void;
	}

	let {
		editor = undefined,
		value = $bindable(),
		language = undefined,
		theme = undefined,
		options = { value, automaticLayout: true },
		onReady = undefined,

		...restProps
	}: Props = $props();

	let container: HTMLDivElement;
	let model = $derived(editor?.getModel());

	$effect(() => {
		if (theme) setTheme(theme);
	});

	$effect(() => {
		editor?.updateOptions(options);
	});

	$effect(() => {
		if (model && language) monaco.editor.setModelLanguage(model, language);
	});

	$effect(() => {
		if (editor && editor.getValue() != value) {
			const position = editor.getPosition();
			editor.setValue(value);
			if (position) editor.setPosition(position);
		}
	});

	onMount(async () => {
		editor = monaco.editor.create(container, options);

		if (onReady) onReady(editor);

		defineThemes();
		if (theme) setTheme(theme);

		editor.getModel()!.onDidChangeContent(() => {
			if (!editor) return;
			value = editor.getValue();
		});
	});

	onDestroy(() => editor?.dispose());
</script>

<div bind:this={container} {...restProps}></div>
