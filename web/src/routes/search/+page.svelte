<script lang="ts">
	import { onMount } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import {
		searchAll,
		fetchIssues,
		fetchPulls,
		type SearchHit,
		type Issue,
		type PullRequest
	} from '$lib/api';
	import {
		Alert,
		Badge,
		Button,
		Card,
		Input,
		Modal,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import IssueDetail from '$lib/components/IssueDetail.svelte';
	import PrDetail from '$lib/components/PrDetail.svelte';
	import TablePagination from '$lib/components/TablePagination.svelte';

	const PAGE_KEY = 'wshm.pageSize.search';
	function readStoredLimit(): number {
		try {
			const raw = localStorage.getItem(PAGE_KEY);
			const n = raw ? Number(raw) : NaN;
			return Number.isFinite(n) && n > 0 ? n : 50;
		} catch {
			return 50;
		}
	}

	let q: string = $state(($page.url.searchParams.get('q') ?? '').trim());
	let qInput: string = $state(q);
	let pageLimit = $state(readStoredLimit());
	let pageOffset = $state(0);
	let total = $state(0);
	let hits: SearchHit[] = $state([]);
	let loading = $state(false);
	let error: string | null = $state(null);

	let loadToken = 0;
	async function load() {
		const myToken = ++loadToken;
		if (!q) {
			hits = [];
			total = 0;
			return;
		}
		loading = true;
		error = null;
		try {
			const data = await searchAll({ q, limit: pageLimit, offset: pageOffset });
			if (myToken !== loadToken) return;
			hits = data.items;
			total = data.total;
			pageLimit = data.limit;
			pageOffset = data.offset;
		} catch (e) {
			if (myToken !== loadToken) return;
			error = e instanceof Error ? e.message : 'Search failed';
		}
		if (myToken === loadToken) loading = false;
	}

	function onSubmit(event: Event) {
		event.preventDefault();
		q = qInput.trim();
		pageOffset = 0;
		const url = new URL($page.url);
		if (q) url.searchParams.set('q', q);
		else url.searchParams.delete('q');
		goto(url.pathname + url.search, { replaceState: true, keepFocus: true });
		load();
	}

	function onPageChange(next: { limit: number; offset: number }) {
		pageLimit = next.limit;
		pageOffset = next.offset;
		load();
	}

	function kindLabel(k: SearchHit['kind']): string {
		return { issue: 'Issue', pull: 'PR', triage: 'Triage', comment: 'Comment' }[k];
	}
	function kindColor(k: SearchHit['kind']): 'blue' | 'green' | 'yellow' | 'gray' {
		return ({ issue: 'blue', pull: 'green', triage: 'yellow', comment: 'gray' } as const)[k];
	}

	let modalOpen = $state(false);
	let modalKind: SearchHit['kind'] | null = $state(null);
	let activeIssue: Issue | null = $state(null);
	let activePr: PullRequest | null = $state(null);
	let detailLoading = $state(false);
	let detailError: string | null = $state(null);

	async function openHit(hit: SearchHit) {
		modalKind = hit.kind === 'comment' ? 'issue' : hit.kind === 'triage' ? 'issue' : hit.kind;
		modalOpen = true;
		activeIssue = null;
		activePr = null;
		detailLoading = true;
		detailError = null;
		try {
			if (modalKind === 'pull') {
				const all = await fetchPulls({ limit: 500 });
				activePr = all.items.find((p) => p.repo === hit.repo && p.number === hit.number) ?? null;
				if (!activePr) detailError = `PR #${hit.number} not found`;
			} else {
				const all = await fetchIssues({ limit: 500 });
				activeIssue =
					all.items.find((i) => i.repo === hit.repo && i.number === hit.number) ?? null;
				if (!activeIssue) detailError = `Issue #${hit.number} not found`;
			}
		} catch (e) {
			detailError = e instanceof Error ? e.message : 'Failed to load';
		}
		detailLoading = false;
	}

	onMount(() => {
		load();
	});
</script>

<svelte:head>
	<title>wshm - Search{q ? ` · ${q}` : ''}</title>
</svelte:head>

<div class="mb-4">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Search</h2>
	<p class="text-sm text-gray-500">
		Full-text search across issues, pull requests, triage results, and comments.
	</p>
</div>

<form onsubmit={onSubmit} class="mb-4 flex gap-2">
	<Input
		type="search"
		bind:value={qInput}
		placeholder="Search… (e.g. 'hermes', 'oauth flow', 'cve-2025-')"
		size="md"
		class="flex-1"
	/>
	<Button type="submit" color="blue" disabled={loading}>
		{loading ? 'Searching…' : 'Search'}
	</Button>
</form>

{#if error}
	<Alert color="red" class="mb-3">{error}</Alert>
{/if}

{#if !q && !loading}
	<Card class="bg-gray-800 border-gray-700 max-w-none">
		<p class="text-gray-500 text-center py-4 text-sm">
			Type a query above to search across all your repos. Multi-word queries
			narrow (AND); each token does a prefix match (<code>hermes</code> matches
			<code>hermes-agent-cli</code>).
		</p>
	</Card>
{:else if total === 0 && !loading && q}
	<Card class="bg-gray-800 border-gray-700 max-w-none">
		<p class="text-gray-500 text-center py-4 text-sm">
			No matches for <code>{q}</code>.
		</p>
	</Card>
{:else}
	<div class="w-full overflow-x-auto">
		<Table striped hoverable class="w-full">
			<TableHead class="text-xs uppercase text-gray-400">
				<TableHeadCell class="px-2 py-1.5 w-[80px]">Kind</TableHeadCell>
				<TableHeadCell class="px-2 py-1.5 w-[200px]">Repo</TableHeadCell>
				<TableHeadCell class="px-2 py-1.5 w-[80px]">#</TableHeadCell>
				<TableHeadCell class="px-2 py-1.5">Match</TableHeadCell>
				<TableHeadCell class="px-2 py-1.5 w-[160px]">Updated</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each hits as hit}
					<TableBodyRow class="cursor-pointer" onclick={() => openHit(hit)}>
						<TableBodyCell class="px-2 py-1.5">
							<Badge color={kindColor(hit.kind)}>{kindLabel(hit.kind)}</Badge>
						</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5 mono text-xs text-gray-400">{hit.repo}</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5 mono">#{hit.number}</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5 text-sm">
							{#if hit.title}
								<div class="font-semibold text-gray-200 truncate">{hit.title}</div>
							{/if}
							{#if hit.snippet}
								<!-- snippet contains <mark>…</mark> from FTS5 — render as HTML -->
								<div class="text-xs text-gray-400 truncate">{@html hit.snippet}</div>
							{/if}
						</TableBodyCell>
						<TableBodyCell class="px-2 py-1.5 text-xs text-gray-500 mono whitespace-nowrap">
							{hit.updated_at?.slice(0, 10) ?? ''}
						</TableBodyCell>
					</TableBodyRow>
				{:else}
					<TableBodyRow>
						<TableBodyCell colspan={5} class="text-center text-gray-600 py-8">
							{loading ? 'Searching…' : 'No matches'}
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>

	<TablePagination
		{total}
		limit={pageLimit}
		offset={pageOffset}
		storageKey={PAGE_KEY}
		onChange={onPageChange}
	/>
{/if}

<Modal
	bind:open={modalOpen}
	size="xl"
	dismissable
	class="!max-w-[80vw] w-[80vw] bg-gray-900 border-gray-700"
	bodyClass="text-gray-200"
>
	{#snippet header()}
		<div class="flex w-full items-center gap-3 pr-2">
			<span class="mono text-gray-500 text-sm">
				{modalKind === 'pull' ? 'PR' : 'Issue'} #{activePr?.number ?? activeIssue?.number ?? ''}
			</span>
			<span class="text-base font-semibold text-gray-100 truncate">
				{activePr?.title ?? activeIssue?.title ?? (detailLoading ? 'Loading…' : '')}
			</span>
		</div>
	{/snippet}
	{#if detailLoading}
		<p class="text-gray-500 text-sm">Loading…</p>
	{:else if detailError}
		<p class="text-red-400 text-sm">{detailError}</p>
	{:else if modalKind === 'pull' && activePr}
		<PrDetail pr={activePr} />
		<div class="text-right pt-2">
			<a href="/prs/{activePr.number}" class="text-xs text-blue-400 hover:text-blue-300">
				Open full page →
			</a>
		</div>
	{:else if activeIssue}
		<IssueDetail issue={activeIssue} />
		<div class="text-right pt-2">
			<a href="/issues/{activeIssue.number}" class="text-xs text-blue-400 hover:text-blue-300">
				Open full page →
			</a>
		</div>
	{/if}
</Modal>
