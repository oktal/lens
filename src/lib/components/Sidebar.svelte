<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { Button } from '$lib/components/ui/button';
	import ModeToggle, { type Mode } from './ModeToggle.svelte';

	import { resetMode, setMode } from 'mode-watcher';
	import Icon from '@iconify/svelte';
	import { open } from '@tauri-apps/api/shell';

	function openGitHubPage() {
		open('https://github.com/oktal/lens');
	}
</script>

<div class="flex h-screen w-12 flex-col items-center gap-1 border">
	<Tooltip.Root>
		<Tooltip.Trigger>
			<Button variant="ghost" size="icon">
				<Icon icon="carbon:cube" width={20} height={20} />
			</Button>
		</Tooltip.Trigger>
		<Tooltip.Content>
			<p>Explore</p>
		</Tooltip.Content>
	</Tooltip.Root>

	<div class="mt-auto items-center">
		<Tooltip.Root>
			<Tooltip.Trigger>
				<Button variant="ghost" size="icon" on:click={openGitHubPage}>
					<Icon icon="mdi:github" width={24} height={24} />
				</Button>
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>GitHub</p>
			</Tooltip.Content>
		</Tooltip.Root>

		<Tooltip.Root>
			<Tooltip.Trigger>
				<ModeToggle onToggle={(mode: Mode) => setMode(mode)} onReset={() => resetMode()} />
			</Tooltip.Trigger>
			<Tooltip.Content>
				<p>Switch mode</p>
			</Tooltip.Content>
		</Tooltip.Root>
	</div>
</div>
