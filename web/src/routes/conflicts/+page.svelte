<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { Card, Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Badge } from 'flowbite-svelte';
	import ProGate from '$lib/ProGate.svelte';
	import { fetchConflicts, type ConflictEntry } from '$lib/api';

	let conflicts: ConflictEntry[] = $state([]);
	let error: string | null = $state(null);

	function timeAgo(dateStr: string): string {
		const diff = Date.now() - new Date(dateStr).getTime();
		const days = Math.floor(diff / 86400000);
		if (days === 0) return 'today';
		if (days === 1) return '1d';
		return `${days}d`;
	}

	function statusColor(status: string): 'red' | 'yellow' | 'green' | 'dark' {
		switch (status) {
			case 'detected': return 'red';
			case 'resolving': return 'yellow';
			case 'resolved': return 'green';
			default: return 'dark';
		}
	}

	async function load() {
		try {
			error = null;
			conflicts = await fetchConflicts();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load conflicts';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Conflict Resolution</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Conflict Resolution</h2>
	<p class="text-sm text-gray-500">PRs with merge conflicts and AI-assisted resolution status</p>
</div>

<ProGate feature="conflicts">
	{#if error}
		<Card class="border-red-500 bg-gray-800">
			<p class="text-red-400">{error}</p>
		</Card>
	{:else if conflicts.length === 0}
		<Card class="bg-gray-800 border-gray-700">
			<p class="text-gray-500 text-center py-8">No conflicts detected. Conflicts will appear here when PRs have merge conflicts.</p>
		</Card>
	{:else}
		<div class="overflow-x-auto">
			<Table striped hoverable class="w-full">
				<TableHead class="text-xs uppercase text-gray-400">
					<TableHeadCell class="px-2 py-1.5 w-[60px]">PR</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5">Title</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5">Conflict Files</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[100px]">Status</TableHeadCell>
					<TableHeadCell class="px-2 py-1.5 w-[70px]">Date</TableHeadCell>
				</TableHead>
				<TableBody>
					{#each conflicts as c}
						<TableBodyRow>
							<TableBodyCell class="px-2 py-1.5 mono text-gray-200">#{c.pr_number}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 text-gray-200 truncate max-w-[250px]">{c.title}</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 mono text-gray-400 text-xs">
								{#each c.conflict_files as f}
									<span class="block">{f}</span>
								{/each}
							</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5">
								<Badge color={statusColor(c.status)}>{c.status}</Badge>
							</TableBodyCell>
							<TableBodyCell class="px-2 py-1.5 text-gray-400 mono">{timeAgo(c.detected_at)}</TableBodyCell>
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/if}
</ProGate>
