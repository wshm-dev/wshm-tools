<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { Card, Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Badge } from 'flowbite-svelte';
	import ProGate from '$lib/ProGate.svelte';
	import { fetchImprovements, type ImprovementEntry } from '$lib/api';

	let improvements: ImprovementEntry[] = $state([]);
	let error: string | null = $state(null);

	function statusColor(status: string): 'blue' | 'yellow' | 'green' | 'dark' {
		switch (status) {
			case 'proposed': return 'blue';
			case 'created': return 'yellow';
			case 'fixed': return 'green';
			default: return 'dark';
		}
	}

	function effortColor(effort: string): 'green' | 'yellow' | 'red' | 'dark' {
		switch (effort) {
			case 'low': return 'green';
			case 'medium': return 'yellow';
			case 'high': return 'red';
			default: return 'dark';
		}
	}

	async function load() {
		try {
			error = null;
			improvements = await fetchImprovements();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load improvements';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Improvement Proposals</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Improvement Proposals</h2>
	<p class="text-sm text-gray-500">AI-suggested improvements for the codebase</p>
</div>

<ProGate feature="improve">
	{#if error}
		<Card class="border-red-500 bg-gray-800">
			<p class="text-red-400">{error}</p>
		</Card>
	{:else if improvements.length === 0}
		<Card class="bg-gray-800 border-gray-700">
			<p class="text-gray-500 text-center py-8">No improvement proposals yet. Proposals will appear here when the AI identifies codebase improvements.</p>
		</Card>
	{:else}
		<div class="overflow-x-auto">
			<Table striped hoverable class="w-full">
				<TableHead class="text-xs uppercase text-gray-400">
					<TableHeadCell class="px-2 py-1.5">Title</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[100px]">Category</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[80px]">Effort</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5">File(s)</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[100px]">Status</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each improvements as imp}
						<TableBodyRow>
							<TableBodyCell class="px-2 py-1.5 text-gray-200">{imp.title}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5">
								<Badge color="dark">{imp.category}</Badge>
							</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5">
								<Badge color={effortColor(imp.effort)}>{imp.effort}</Badge>
							</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 mono text-gray-400 text-xs">
								{#each imp.files as f}
									<span class="block">{f}</span>
								{/each}
							</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5">
								<Badge color={statusColor(imp.status)}>{imp.status}</Badge>
							</TableBodyCell>
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/if}
</ProGate>
