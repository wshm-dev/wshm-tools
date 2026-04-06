<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { Card, Badge } from 'flowbite-svelte';
	import ProGate from '$lib/ProGate.svelte';
	import { fetchChangelog, type ChangelogEntry } from '$lib/api';

	let entries: ChangelogEntry[] = $state([]);
	let error: string | null = $state(null);

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric'
		});
	}

	function typeColor(type: string): 'green' | 'blue' | 'yellow' | 'red' | 'dark' {
		switch (type) {
			case 'feature': return 'green';
			case 'fix': return 'red';
			case 'refactor': return 'yellow';
			case 'docs': return 'blue';
			default: return 'dark';
		}
	}

	async function load() {
		try {
			error = null;
			entries = await fetchChangelog();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load changelog';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Changelog</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Changelog</h2>
	<p class="text-sm text-gray-500">Auto-generated changelog from merged pull requests</p>
</div>

<ProGate feature="changelog">
	{#if error}
		<Card class="border-red-500 bg-gray-800">
			<p class="text-red-400">{error}</p>
		</Card>
	{:else if entries.length === 0}
		<Card class="bg-gray-800 border-gray-700">
			<p class="text-gray-500 text-center py-8">No changelog entries yet. Entries will be generated from merged PRs when the changelog feature is active.</p>
		</Card>
	{:else}
		<div class="space-y-4">
			{#each entries as entry}
				<Card class="bg-gray-800 border-gray-700">
					<div class="flex items-center justify-between mb-3">
						<h3 class="text-base font-semibold text-gray-100">{entry.version}</h3>
						<span class="text-sm text-gray-500">{formatDate(entry.date)}</span>
					</div>
					<ul class="space-y-1.5">
						{#each entry.items as item}
							<li class="flex items-start gap-2 text-sm">
								<Badge color={typeColor(item.type)} class="mt-0.5 flex-shrink-0">{item.type}</Badge>
								<span class="text-gray-300">{item.description}</span>
								{#if item.pr_number}
									<span class="text-gray-600 mono text-xs flex-shrink-0">#{item.pr_number}</span>
								{/if}
							</li>
						{/each}
					</ul>
				</Card>
			{/each}
		</div>
	{/if}
</ProGate>
