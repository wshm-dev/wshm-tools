<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchActivity, fetchIssues, fetchPulls, type ActivityEntry, type Issue, type PullRequest } from '$lib/api';
	import { multiSort, toggleSort as toggle, sortArrow, sortIndex, sortArrowClass, type SortColumn } from '$lib/sort';
	import { applyFilters } from '$lib/filter';
	import { paginate, totalPages, PAGE_SIZE } from '$lib/paginate';
	import { Table, TableHead, TableHeadCell, TableBody, TableBodyRow, TableBodyCell, Badge, Input, Modal } from 'flowbite-svelte';
	import IssueDetail from '$lib/components/IssueDetail.svelte';
	import PrDetail from '$lib/components/PrDetail.svelte';

	let activities: ActivityEntry[] = $state([]);
	let error: string | null = $state(null);
	let sortColumns: SortColumn[] = $state([{ key: 'created_at', asc: false }]);
	let filters: Record<string, string> = $state({
		created_at: '', action: '', target: '', summary: ''
	});

	function formatTime(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
	}

	function handleSort(key: string, event: MouseEvent) {
		sortColumns = toggle(sortColumns, key, event.shiftKey);
	}

	let enriched = $derived(activities.map(a => ({
		...a,
		target: `${a.target_type} #${a.target_number}`
	})));

	let filtered = $derived(applyFilters(enriched, {
		created_at: filters.created_at,
		action: filters.action,
		target: filters.target,
		summary: filters.summary
	}));

	let sorted = $derived(multiSort(filtered, sortColumns));
	let page = $state(0);
	let pages = $derived(totalPages(sorted.length));
	let paged = $derived(paginate(sorted, page));

	async function load() {
		page = 0;
		try {
			error = null;
			activities = await fetchActivity();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load activity';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});

	function actionColor(action: string): 'blue' | 'green' | 'yellow' | 'gray' {
		if (action === 'triage') return 'blue';
		if (action === 'merge') return 'green';
		if (action === 'analyze') return 'yellow';
		return 'gray';
	}

	let issueModalOpen = $state(false);
	let activeIssue: Issue | null = $state(null);
	let prModalOpen = $state(false);
	let activePr: PullRequest | null = $state(null);
	let detailLoading = $state(false);
	let detailError: string | null = $state(null);

	async function openTarget(targetType: string, num: number) {
		detailError = null;
		detailLoading = true;
		const isPr = targetType === 'pr' || targetType === 'pull' || targetType === 'pull_request';
		if (isPr) {
			activePr = null;
			prModalOpen = true;
			try {
				const all = await fetchPulls();
				activePr = all.find((p) => p.number === num) ?? null;
				if (!activePr) detailError = `PR #${num} not found`;
			} catch (e) {
				detailError = e instanceof Error ? e.message : 'Failed to load';
			}
		} else {
			activeIssue = null;
			issueModalOpen = true;
			try {
				const all = await fetchIssues();
				activeIssue = all.find((i) => i.number === num) ?? null;
				if (!activeIssue) detailError = `Issue #${num} not found`;
			} catch (e) {
				detailError = e instanceof Error ? e.message : 'Failed to load';
			}
		}
		detailLoading = false;
	}
</script>

<svelte:head>
	<title>wshm - Activity</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Activity Log</h2>
	<p class="text-sm text-gray-500">Recent triage and analysis actions</p>
</div>

{#if error}
	<div class="rounded-lg border border-red-500 bg-gray-800 p-5">
		<p class="text-red-400">{error}</p>
	</div>
{:else}
	<div class="w-full overflow-x-auto">
		<Table striped hoverable class="w-full">
			<TableHead class="text-xs uppercase text-gray-400">
				<TableHeadCell class="cursor-pointer select-none px-2 py-1.5 w-[180px]" onclick={(e: MouseEvent) => handleSort('created_at', e)}>
					Time <span class={sortArrowClass(sortColumns, 'created_at')}>{sortArrow(sortColumns, 'created_at')}</span>{#if sortIndex(sortColumns, 'created_at') > 0}<span class="text-[0.625rem] text-blue-400 ml-0.5">{sortIndex(sortColumns, 'created_at')}</span>{/if}
				</TableHeadCell>
				<TableHeadCell class="cursor-pointer select-none px-2 py-1.5 w-[90px]" onclick={(e: MouseEvent) => handleSort('action', e)}>
					Action <span class={sortArrowClass(sortColumns, 'action')}>{sortArrow(sortColumns, 'action')}</span>{#if sortIndex(sortColumns, 'action') > 0}<span class="text-[0.625rem] text-blue-400 ml-0.5">{sortIndex(sortColumns, 'action')}</span>{/if}
				</TableHeadCell>
				<TableHeadCell class="cursor-pointer select-none px-2 py-1.5 w-[120px]" onclick={(e: MouseEvent) => handleSort('target', e)}>
					Target <span class={sortArrowClass(sortColumns, 'target')}>{sortArrow(sortColumns, 'target')}</span>{#if sortIndex(sortColumns, 'target') > 0}<span class="text-[0.625rem] text-blue-400 ml-0.5">{sortIndex(sortColumns, 'target')}</span>{/if}
				</TableHeadCell>
				<TableHeadCell class="cursor-pointer select-none px-2 py-1.5" onclick={(e: MouseEvent) => handleSort('summary', e)}>
					Summary <span class={sortArrowClass(sortColumns, 'summary')}>{sortArrow(sortColumns, 'summary')}</span>{#if sortIndex(sortColumns, 'summary') > 0}<span class="text-[0.625rem] text-blue-400 ml-0.5">{sortIndex(sortColumns, 'summary')}</span>{/if}
				</TableHeadCell>
			</TableHead>
			<TableBody>
				<TableBodyRow class="border-b border-gray-700">
					<TableBodyCell class="px-2 py-1"><Input type="text" bind:value={filters.created_at} placeholder="filter..." size="sm" class="!py-0.5 !px-1 text-xs" /></TableBodyCell>
					<TableBodyCell class="px-2 py-1"><Input type="text" bind:value={filters.action} placeholder="filter..." size="sm" class="!py-0.5 !px-1 text-xs" /></TableBodyCell>
					<TableBodyCell class="px-2 py-1"><Input type="text" bind:value={filters.target} placeholder="filter..." size="sm" class="!py-0.5 !px-1 text-xs" /></TableBodyCell>
					<TableBodyCell class="px-2 py-1"><Input type="text" bind:value={filters.summary} placeholder="filter..." size="sm" class="!py-0.5 !px-1 text-xs" /></TableBodyCell>
				</TableBodyRow>
				{#each paged as entry}
					<TableBodyRow class="cursor-pointer" onclick={() => openTarget(entry.target_type, entry.target_number)}>
						<TableBodyCell class="px-2 py-1.5 text-gray-500 whitespace-nowrap text-sm">{formatTime(entry.created_at)}</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5">
							<Badge color={actionColor(entry.action)}>{entry.action}</Badge>
						</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5 whitespace-nowrap mono">{entry.target_type} #{entry.target_number}</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5">{entry.summary}</TableBodyCell>
					</TableBodyRow>
				{:else}
					<TableBodyRow>
						<TableBodyCell colspan={4} class="text-center text-gray-600 py-8">No activity recorded yet</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
	<Modal
		bind:open={issueModalOpen}
		size="xl"
		dismissable
		class="!max-w-[80vw] w-[80vw] bg-gray-900 border-gray-700"
		bodyClass="text-gray-200"
	>
		{#snippet header()}
			<div class="flex w-full items-center gap-3 pr-2">
				<span class="mono text-gray-500 text-sm">#{activeIssue?.number ?? ''}</span>
				<span class="text-base font-semibold text-gray-100 truncate">
					{activeIssue?.title ?? (detailLoading ? 'Loading…' : '')}
				</span>
			</div>
		{/snippet}
		{#if detailLoading}
			<p class="text-gray-500 text-sm">Loading…</p>
		{:else if detailError}
			<p class="text-red-400 text-sm">{detailError}</p>
		{:else if activeIssue}
			<IssueDetail issue={activeIssue} />
			<div class="text-right pt-2">
				<a href="/issues/{activeIssue.number}" class="text-xs text-blue-400 hover:text-blue-300">
					Open full page →
				</a>
			</div>
		{/if}
	</Modal>

	<Modal
		bind:open={prModalOpen}
		size="xl"
		dismissable
		class="!max-w-[80vw] w-[80vw] bg-gray-900 border-gray-700"
		bodyClass="text-gray-200"
	>
		{#snippet header()}
			<div class="flex w-full items-center gap-3 pr-2">
				<span class="mono text-gray-500 text-sm">#{activePr?.number ?? ''}</span>
				<span class="text-base font-semibold text-gray-100 truncate">
					{activePr?.title ?? (detailLoading ? 'Loading…' : '')}
				</span>
			</div>
		{/snippet}
		{#if detailLoading}
			<p class="text-gray-500 text-sm">Loading…</p>
		{:else if detailError}
			<p class="text-red-400 text-sm">{detailError}</p>
		{:else if activePr}
			<PrDetail pr={activePr} />
			<div class="text-right pt-2">
				<a href="/prs/{activePr.number}" class="text-xs text-blue-400 hover:text-blue-300">
					Open full page →
				</a>
			</div>
		{/if}
	</Modal>

	{#if pages > 1}
		<div class="flex items-center justify-between mt-2 text-sm text-gray-400">
			<span>{sorted.length} results (page {page + 1}/{pages})</span>
			<div class="flex gap-1">
				<button onclick={() => page = 0} disabled={page === 0} class="px-2 py-0.5 rounded border border-gray-600 hover:border-blue-500 disabled:opacity-30 disabled:cursor-default text-xs">|&lt;</button>
				<button onclick={() => page = Math.max(0, page - 1)} disabled={page === 0} class="px-2 py-0.5 rounded border border-gray-600 hover:border-blue-500 disabled:opacity-30 disabled:cursor-default text-xs">&lt;</button>
				<button onclick={() => page = Math.min(pages - 1, page + 1)} disabled={page >= pages - 1} class="px-2 py-0.5 rounded border border-gray-600 hover:border-blue-500 disabled:opacity-30 disabled:cursor-default text-xs">&gt;</button>
				<button onclick={() => page = pages - 1} disabled={page >= pages - 1} class="px-2 py-0.5 rounded border border-gray-600 hover:border-blue-500 disabled:opacity-30 disabled:cursor-default text-xs">&gt;|</button>
			</div>
		</div>
	{/if}
{/if}
