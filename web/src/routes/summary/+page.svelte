<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import {
		fetchSummary,
		fetchIssues,
		fetchPulls,
		type Summary,
		type Issue,
		type PullRequest
	} from '$lib/api';
	import { Card, Badge, Modal } from 'flowbite-svelte';
	import IssueDetail from '$lib/components/IssueDetail.svelte';
	import PrDetail from '$lib/components/PrDetail.svelte';

	let summary: Summary | null = $state(null);
	let error: string | null = $state(null);

	async function load() {
		try {
			error = null;
			summary = await fetchSummary();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load summary';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});

	function priorityColor(p: string | null): 'red' | 'yellow' | 'blue' | 'dark' {
		if (p === 'critical' || p === 'high') return 'red';
		if (p === 'medium') return 'yellow';
		if (p === 'low') return 'blue';
		return 'dark';
	}

	let issueModalOpen = $state(false);
	let activeIssue: Issue | null = $state(null);
	let issueLoading = $state(false);
	let issueError: string | null = $state(null);

	let prModalOpen = $state(false);
	let activePr: PullRequest | null = $state(null);
	let prLoading = $state(false);
	let prError: string | null = $state(null);

	async function openIssue(num: number) {
		issueModalOpen = true;
		activeIssue = null;
		issueError = null;
		issueLoading = true;
		try {
			const all = await fetchIssues();
			activeIssue = all.find((i) => i.number === num) ?? null;
			if (!activeIssue) issueError = `Issue #${num} not found`;
		} catch (e) {
			issueError = e instanceof Error ? e.message : 'Failed to load';
		}
		issueLoading = false;
	}

	async function openPr(num: number) {
		prModalOpen = true;
		activePr = null;
		prError = null;
		prLoading = true;
		try {
			const all = await fetchPulls();
			activePr = all.find((p) => p.number === num) ?? null;
			if (!activePr) prError = `PR #${num} not found`;
		} catch (e) {
			prError = e instanceof Error ? e.message : 'Failed to load';
		}
		prLoading = false;
	}
</script>

<svelte:head>
	<title>wshm - Summary</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Summary</h2>
	<p class="text-sm text-gray-500">Daily digest — same data as Discord notifications</p>
</div>

{#if error}
	<Card class="border-red-500 bg-gray-800 max-w-none">
		<p class="text-red-400">{error}</p>
		<p class="mt-2 text-sm text-gray-500">The wshm daemon must expose <code>/api/v1/summary</code>.</p>
	</Card>
{:else if summary}
	<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
		<Card class="bg-gray-800 border-gray-700 text-center max-w-none">
			<div class="text-xs uppercase tracking-wider text-gray-500 mb-2">Open Issues</div>
			<div class="text-3xl font-bold text-gray-100 mono">{summary.open_issues}</div>
			<div class="text-xs text-gray-500 mt-1">{summary.untriaged_issues} untriaged</div>
		</Card>
		<Card class="bg-gray-800 border-gray-700 text-center max-w-none">
			<div class="text-xs uppercase tracking-wider text-gray-500 mb-2">Open PRs</div>
			<div class="text-3xl font-bold text-gray-100 mono">{summary.open_prs}</div>
			<div class="text-xs text-gray-500 mt-1">{summary.unanalyzed_prs} unanalyzed</div>
		</Card>
		<Card class="bg-gray-800 border-gray-700 text-center max-w-none">
			<div class="text-xs uppercase tracking-wider text-gray-500 mb-2">Conflicts</div>
			<div class="text-3xl font-bold {summary.conflicts > 0 ? 'text-red-400' : 'text-gray-100'} mono">{summary.conflicts}</div>
		</Card>
		<Card class="bg-gray-800 border-gray-700 text-center max-w-none">
			<div class="text-xs uppercase tracking-wider text-gray-500 mb-2">Action Required</div>
			<div class="text-3xl font-bold {summary.high_priority_issues.length > 0 ? 'text-red-400' : 'text-gray-100'} mono">{summary.high_priority_issues.length}</div>
		</Card>
	</div>

	{#if summary.high_priority_issues.length > 0}
		<Card class="bg-gray-800 border-gray-700 mb-4 max-w-none">
			<h3 class="text-lg font-semibold text-red-400 mb-3">Action Required</h3>
			<ul class="space-y-2">
				{#each summary.high_priority_issues.slice(0, 10) as issue (issue.number)}
					<li
						class="flex items-start gap-2 text-sm cursor-pointer hover:bg-gray-700/50 rounded px-1 py-0.5"
						onclick={() => openIssue(issue.number)}
						onkeydown={(e) => e.key === 'Enter' && openIssue(issue.number)}
						role="button"
						tabindex="0"
					>
						<span class="text-yellow-400 mono">#{issue.number}</span>
						<Badge color={priorityColor(issue.priority)}>{issue.priority ?? '?'}</Badge>
						<span class="text-gray-300 flex-1">{issue.title}</span>
						{#if issue.age_days > 0}<span class="text-gray-500 text-xs">{issue.age_days}d</span>{/if}
					</li>
				{/each}
			</ul>
		</Card>
	{/if}

	{#if summary.high_risk_prs.length > 0}
		<Card class="bg-gray-800 border-gray-700 mb-4 max-w-none">
			<h3 class="text-lg font-semibold text-purple-400 mb-3">Attention PRs</h3>
			<ul class="space-y-2">
				{#each summary.high_risk_prs.slice(0, 10) as pr (pr.number)}
					<li
						class="flex items-start gap-2 text-sm cursor-pointer hover:bg-gray-700/50 rounded px-1 py-0.5"
						onclick={() => openPr(pr.number)}
						onkeydown={(e) => e.key === 'Enter' && openPr(pr.number)}
						role="button"
						tabindex="0"
					>
						<span class="text-yellow-400 mono">#{pr.number}</span>
						{#if pr.risk_level}<Badge color="purple">risk:{pr.risk_level}</Badge>{/if}
						{#if pr.has_conflicts}<Badge color="red">CONFLICT</Badge>{/if}
						<span class="text-gray-300 flex-1">{pr.title}</span>
						{#if pr.age_days > 0}<span class="text-gray-500 text-xs">{pr.age_days}d</span>{/if}
					</li>
				{/each}
			</ul>
		</Card>
	{/if}

	{#if summary.top_issues.length > 0}
		<Card class="bg-gray-800 border-gray-700 mb-4 max-w-none">
			<h3 class="text-lg font-semibold text-cyan-400 mb-3">Issues TODO</h3>
			<ul class="space-y-2">
				{#each summary.top_issues as issue (issue.number)}
					<li
						class="flex items-start gap-2 text-sm cursor-pointer hover:bg-gray-700/50 rounded px-1 py-0.5"
						onclick={() => openIssue(issue.number)}
						onkeydown={(e) => e.key === 'Enter' && openIssue(issue.number)}
						role="button"
						tabindex="0"
					>
						<span class="text-yellow-400 mono">#{issue.number}</span>
						<Badge color={priorityColor(issue.priority)}>{issue.priority ?? '-'}</Badge>
						{#if issue.category}<span class="text-gray-500 text-xs">{issue.category}</span>{/if}
						<span class="text-gray-300 flex-1">{issue.title}</span>
						{#if issue.age_days > 0}<span class="text-gray-500 text-xs">{issue.age_days}d</span>{/if}
					</li>
				{/each}
			</ul>
		</Card>
	{/if}

	{#if summary.top_prs.length > 0}
		<Card class="bg-gray-800 border-gray-700 mb-4 max-w-none">
			<h3 class="text-lg font-semibold text-cyan-400 mb-3">PRs TODO</h3>
			<ul class="space-y-2">
				{#each summary.top_prs as pr (pr.number)}
					<li
						class="flex items-start gap-2 text-sm cursor-pointer hover:bg-gray-700/50 rounded px-1 py-0.5"
						onclick={() => openPr(pr.number)}
						onkeydown={(e) => e.key === 'Enter' && openPr(pr.number)}
						role="button"
						tabindex="0"
					>
						<span class="text-yellow-400 mono">#{pr.number}</span>
						{#if pr.risk_level}<Badge color="purple">{pr.risk_level}</Badge>{/if}
						{#if pr.has_conflicts}<Badge color="red">CONFLICT</Badge>{/if}
						<span class="text-gray-300 flex-1">{pr.title}</span>
						{#if pr.age_days > 0}<span class="text-gray-500 text-xs">{pr.age_days}d</span>{/if}
					</li>
				{/each}
			</ul>
		</Card>
	{/if}

	<p class="text-xs text-gray-500 mt-4">Generated at {summary.timestamp}</p>
{:else}
	<p class="text-gray-500">Loading…</p>
{/if}

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
				{activeIssue?.title ?? (issueLoading ? 'Loading…' : '')}
			</span>
		</div>
	{/snippet}
	{#if issueLoading}
		<p class="text-gray-500 text-sm">Loading…</p>
	{:else if issueError}
		<p class="text-red-400 text-sm">{issueError}</p>
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
				{activePr?.title ?? (prLoading ? 'Loading…' : '')}
			</span>
		</div>
	{/snippet}
	{#if prLoading}
		<p class="text-gray-500 text-sm">Loading…</p>
	{:else if prError}
		<p class="text-red-400 text-sm">{prError}</p>
	{:else if activePr}
		<PrDetail pr={activePr} />
		<div class="text-right pt-2">
			<a href="/prs/{activePr.number}" class="text-xs text-blue-400 hover:text-blue-300">
				Open full page →
			</a>
		</div>
	{/if}
</Modal>
