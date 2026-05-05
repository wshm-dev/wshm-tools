<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { fetchIssues, type Issue } from '$lib/api';
	import { Card } from 'flowbite-svelte';
	import IssueDetail from '$lib/components/IssueDetail.svelte';

	let issue: Issue | null = $state(null);
	let error: string | null = $state(null);
	let id = $derived(Number($page.params.id));

	onMount(async () => {
		try {
			const all = await fetchIssues();
			issue = all.find(i => i.number === id) ?? null;
			if (!issue) error = `Issue #${id} not found`;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load';
		}
	});
</script>

<svelte:head>
	<title>wshm - Issue #{id}</title>
</svelte:head>

<div class="mb-4">
	<a href="/issues" class="text-sm text-blue-400 hover:text-blue-300">← Back to Issues</a>
</div>

{#if error}
	<Card class="bg-gray-800 max-w-none"><p class="text-red-400">{error}</p></Card>
{:else if issue}
	<div class="mb-4">
		<h2 class="text-xl font-semibold text-gray-100">
			<span class="mono text-gray-500">#{issue.number}</span> {issue.title}
		</h2>
	</div>
	<IssueDetail {issue} />
{:else}
	<p class="text-gray-500">Loading...</p>
{/if}
