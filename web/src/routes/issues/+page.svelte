<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchIssues, type Issue } from '$lib/api';

	let issues: Issue[] = $state([]);
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
		[...issues].sort((a, b) => {
			let cmp = 0;
			switch (sortBy) {
				case 'number': cmp = a.number - b.number; break;
				case 'title': cmp = a.title.localeCompare(b.title); break;
				case 'priority': cmp = (a.priority ?? '').localeCompare(b.priority ?? ''); break;
				case 'category': cmp = (a.category ?? '').localeCompare(b.category ?? ''); break;
				case 'age': cmp = ageDays(a.created_at) - ageDays(b.created_at); break;
				default: cmp = 0;
			}
			return sortAsc ? cmp : -cmp;
		})
	);

	async function load() {
		try {
			error = null;
			issues = await fetchIssues();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load issues';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Issues</title>
</svelte:head>

<div class="page-header">
	<h2>Issues</h2>
	<p>All tracked issues from the repository</p>
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
					<th>Labels</th>
					<th class="sortable" onclick={() => toggleSort('priority')}>Priority <span class={arrowClass('priority')}>{arrow('priority')}</span></th>
					<th class="sortable" onclick={() => toggleSort('category')}>Category <span class={arrowClass('category')}>{arrow('category')}</span></th>
					<th class="sortable" onclick={() => toggleSort('age')}>Age <span class={arrowClass('age')}>{arrow('age')}</span></th>
				</tr>
			</thead>
			<tbody>
				{#each sorted as issue}
					<tr>
						<td>{issue.number}</td>
						<td>{issue.title}</td>
						<td>
							<span class="badge" class:badge-green={issue.state === 'open'} class:badge-red={issue.state === 'closed'}>
								{issue.state}
							</span>
						</td>
						<td>
							{#each issue.labels as label}
								<span class="badge badge-blue">{label}</span>
							{/each}
						</td>
						<td>{issue.priority ?? '-'}</td>
						<td>{issue.category ?? '-'}</td>
						<td class="muted">{timeAgo(issue.created_at)}</td>
					</tr>
				{:else}
					<tr>
						<td colspan="7" class="empty">No issues found</td>
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
