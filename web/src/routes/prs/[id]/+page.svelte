<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fetchPulls, type PullRequest } from '$lib/api';
	import { Card } from 'flowbite-svelte';
	import PrDetail from '$lib/components/PrDetail.svelte';

	let pr: PullRequest | null = $state(null);
	let error: string | null = $state(null);
	let id = $derived(Number($page.params.id));

	onMount(async () => {
		try {
			const all = await fetchPulls();
			pr = all.find(p => p.number === id) ?? null;
			if (!pr) error = `PR #${id} not found`;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load';
		}
	});
</script>

<svelte:head>
	<title>wshm - PR #{id}</title>
</svelte:head>

<div class="mb-4">
	<a href="/prs" class="text-sm text-blue-400 hover:text-blue-300">← Back to Pull Requests</a>
</div>

{#if error}
	<Card class="bg-gray-800 max-w-none"><p class="text-red-400">{error}</p></Card>
{:else if pr}
	<div class="mb-4">
		<h2 class="text-xl font-semibold text-gray-100">
			<span class="mono text-gray-500">#{pr.number}</span> {pr.title}
		</h2>
	</div>
	<PrDetail {pr} />
{:else}
	<p class="text-gray-500">Loading...</p>
{/if}
