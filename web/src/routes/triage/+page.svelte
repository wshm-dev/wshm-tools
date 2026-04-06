<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchTriage, type TriageResult } from '$lib/api';

	let results: TriageResult[] = $state([]);
	let error: string | null = $state(null);
	let sortBy: string = $state('issue_number');
	let sortAsc: boolean = $state(true);

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
		[...results].sort((a, b) => {
			let cmp = 0;
			switch (sortBy) {
				case 'issue_number': cmp = a.issue_number - b.issue_number; break;
				case 'category': cmp = a.category.localeCompare(b.category); break;
				case 'confidence': cmp = a.confidence - b.confidence; break;
				case 'priority': cmp = a.priority.localeCompare(b.priority); break;
				case 'acted_at': cmp = (a.acted_at ?? '').localeCompare(b.acted_at ?? ''); break;
				default: cmp = 0;
			}
			return sortAsc ? cmp : -cmp;
		})
	);

	async function load() {
		try {
			error = null;
			results = await fetchTriage();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load triage results';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Triage</title>
</svelte:head>

<div class="page-header">
	<h2>Triage Results</h2>
	<p>AI classification results for issues</p>
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
					<th class="sortable" onclick={() => toggleSort('issue_number')}>Issue <span class={arrowClass('issue_number')}>{arrow('issue_number')}</span></th>
					<th class="sortable" onclick={() => toggleSort('category')}>Category <span class={arrowClass('category')}>{arrow('category')}</span></th>
					<th class="sortable" onclick={() => toggleSort('confidence')}>Confidence <span class={arrowClass('confidence')}>{arrow('confidence')}</span></th>
					<th class="sortable" onclick={() => toggleSort('priority')}>Priority <span class={arrowClass('priority')}>{arrow('priority')}</span></th>
					<th class="sortable" onclick={() => toggleSort('acted_at')}>Acted At <span class={arrowClass('acted_at')}>{arrow('acted_at')}</span></th>
				</tr>
			</thead>
			<tbody>
				{#each sorted as result}
					<tr>
						<td><a href="/issues">#{result.issue_number}</a></td>
						<td>
							<span class="badge"
								class:badge-red={result.category === 'bug'}
								class:badge-blue={result.category === 'feature'}
								class:badge-yellow={result.category === 'needs-info'}
								class:badge-gray={result.category === 'duplicate' || result.category === 'wontfix'}>
								{result.category}
							</span>
						</td>
						<td>
							<span class="confidence"
								class:high={result.confidence >= 0.85}
								class:medium={result.confidence >= 0.6 && result.confidence < 0.85}
								class:low={result.confidence < 0.6}>
								{(result.confidence * 100).toFixed(0)}%
							</span>
						</td>
						<td>{result.priority}</td>
						<td class="muted">{result.acted_at ?? 'Not acted'}</td>
					</tr>
				{:else}
					<tr>
						<td colspan="5" class="empty">No triage results yet</td>
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

	.confidence {
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}

	.confidence.high {
		color: #3fb950;
	}

	.confidence.medium {
		color: #d29922;
	}

	.confidence.low {
		color: #f85149;
	}
</style>
