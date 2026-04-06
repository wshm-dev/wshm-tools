<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchPulls, type PullRequest } from '$lib/api';

	let pulls: PullRequest[] = $state([]);
	let error: string | null = $state(null);
	let sortBy: string = $state('number');
	let sortAsc: boolean = $state(false);

	function timeAgo(dateStr: string): string {
		const diff = Date.now() - new Date(dateStr).getTime();
		const days = Math.floor(diff / 86400000);
		if (days === 0) return 'today';
		if (days === 1) return '1 day ago';
		return `${days} days ago`;
	}

	function ageDays(dateStr: string): number {
		return Math.floor((Date.now() - new Date(dateStr).getTime()) / 86400000);
	}

	const riskOrder: Record<string, number> = { low: 0, medium: 1, high: 2 };
	const ciOrder: Record<string, number> = { success: 0, pending: 1, failure: 2 };

	function toggleSort(column: string) {
		if (sortBy === column) {
			sortAsc = !sortAsc;
		} else {
			sortBy = column;
			sortAsc = true;
		}
	}

	function arrow(column: string): string {
		if (sortBy !== column) return '';
		return sortAsc ? 'v' : '^';
	}

	function arrowClass(column: string): string {
		return sortBy === column ? 'sort-arrow active' : 'sort-arrow';
	}

	let sorted = $derived(
		[...pulls].sort((a, b) => {
			let cmp = 0;
			switch (sortBy) {
				case 'number': cmp = a.number - b.number; break;
				case 'title': cmp = a.title.localeCompare(b.title); break;
				case 'risk': cmp = (riskOrder[a.risk ?? ''] ?? 9) - (riskOrder[b.risk ?? ''] ?? 9); break;
				case 'ci_status': cmp = (ciOrder[a.ci_status ?? ''] ?? 9) - (ciOrder[b.ci_status ?? ''] ?? 9); break;
				case 'age': cmp = ageDays(a.created_at) - ageDays(b.created_at); break;
				default: cmp = 0;
			}
			return sortAsc ? cmp : -cmp;
		})
	);

	async function load() {
		try {
			error = null;
			pulls = await fetchPulls();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load pull requests';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Pull Requests</title>
</svelte:head>

<div class="page-header">
	<h2>Pull Requests</h2>
	<p>All tracked pull requests from the repository</p>
</div>

{#if error}
	<div class="card" style="border-color: #f85149;">
		<p style="color: #f85149;">{error}</p>
	</div>
{:else}
	<div class="card">
		<table>
			<thead>
				<tr>
					<th class="sortable" onclick={() => toggleSort('number')}># <span class={arrowClass('number')}>{arrow('number')}</span></th>
					<th class="sortable" onclick={() => toggleSort('title')}>Title <span class={arrowClass('title')}>{arrow('title')}</span></th>
					<th>State</th>
					<th class="sortable" onclick={() => toggleSort('risk')}>Risk <span class={arrowClass('risk')}>{arrow('risk')}</span></th>
					<th class="sortable" onclick={() => toggleSort('ci_status')}>CI <span class={arrowClass('ci_status')}>{arrow('ci_status')}</span></th>
					<th>Conflicts</th>
					<th class="sortable" onclick={() => toggleSort('age')}>Age <span class={arrowClass('age')}>{arrow('age')}</span></th>
				</tr>
			</thead>
			<tbody>
				{#each sorted as pr}
					<tr>
						<td>{pr.number}</td>
						<td>{pr.title}</td>
						<td>
							<span class="badge" class:badge-green={pr.state === 'open'} class:badge-red={pr.state === 'closed'}>
								{pr.state}
							</span>
						</td>
						<td>
							{#if pr.risk}
								<span class="badge"
									class:badge-green={pr.risk === 'low'}
									class:badge-yellow={pr.risk === 'medium'}
									class:badge-red={pr.risk === 'high'}>
									{pr.risk}
								</span>
							{:else}
								<span class="muted">-</span>
							{/if}
						</td>
						<td>
							{#if pr.ci_status}
								<span class="badge"
									class:badge-green={pr.ci_status === 'success'}
									class:badge-yellow={pr.ci_status === 'pending'}
									class:badge-red={pr.ci_status === 'failure'}>
									{pr.ci_status}
								</span>
							{:else}
								<span class="muted">-</span>
							{/if}
						</td>
						<td>
							{#if pr.has_conflicts}
								<span class="badge badge-red">yes</span>
							{:else}
								<span class="badge badge-green">no</span>
							{/if}
						</td>
						<td class="muted">{timeAgo(pr.created_at)}</td>
					</tr>
				{:else}
					<tr>
						<td colspan="7" class="empty">No pull requests found</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

<style>
	.muted {
		color: #8b949e;
		font-size: 0.875rem;
	}

	.empty {
		text-align: center;
		color: #484f58;
		padding: 2rem 0;
	}
</style>
