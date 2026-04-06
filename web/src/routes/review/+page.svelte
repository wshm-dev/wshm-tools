<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { Card, Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Badge } from 'flowbite-svelte';
	import ProGate from '$lib/ProGate.svelte';
	import { fetchReviews, type ReviewEntry } from '$lib/api';

	let reviews: ReviewEntry[] = $state([]);
	let error: string | null = $state(null);

	function timeAgo(dateStr: string): string {
		const diff = Date.now() - new Date(dateStr).getTime();
		const days = Math.floor(diff / 86400000);
		if (days === 0) return 'today';
		if (days === 1) return '1d';
		return `${days}d`;
	}

	function severityColor(severity: string): 'red' | 'yellow' | 'blue' | 'dark' {
		switch (severity) {
			case 'error': return 'red';
			case 'warning': return 'yellow';
			case 'info': return 'blue';
			default: return 'dark';
		}
	}

	async function load() {
		try {
			error = null;
			reviews = await fetchReviews();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load reviews';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Code Review</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Code Review</h2>
	<p class="text-sm text-gray-500">AI-generated inline review comments on pull requests</p>
</div>

<ProGate feature="review">
	{#if error}
		<Card class="border-red-500 bg-gray-800">
			<p class="text-red-400">{error}</p>
		</Card>
	{:else if reviews.length === 0}
		<Card class="bg-gray-800 border-gray-700">
			<p class="text-gray-500 text-center py-8">No review comments yet. Reviews will appear here once PRs are analyzed with the review feature enabled.</p>
		</Card>
	{:else}
		<div class="overflow-x-auto">
			<Table striped hoverable class="w-full">
				<TableHead class="text-xs uppercase text-gray-400">
					<TableHeadCell class="px-2 py-1.5 w-[60px]">PR</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5">File</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[60px]">Line</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5">Comment</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[90px]">Severity</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[70px]">Date</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each reviews as r}
						<TableBodyRow>
							<TableBodyCell class="px-2 py-1.5 mono text-gray-200">#{r.pr_number}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 mono text-gray-300 text-xs truncate max-w-[200px]">{r.file}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 mono text-gray-400">{r.line}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 text-gray-200 text-sm">{r.comment}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5">
								<Badge color={severityColor(r.severity)}>{r.severity}</Badge>
							</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 text-gray-400 mono">{timeAgo(r.created_at)}</TableBodyCell>
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/if}
</ProGate>
