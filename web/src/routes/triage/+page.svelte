<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchTriage, type TriageResult } from '$lib/api';

	let results: TriageResult[] = $state([]);
	let error: string | null = $state(null);

	onMount(async () => {
		try {
			results = await fetchTriage();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load triage results';
		}
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
					<th>Issue</th>
					<th>Category</th>
					<th>Confidence</th>
					<th>Priority</th>
					<th>Acted At</th>
				</tr>
			</thead>
			<tbody>
				{#each results as result}
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
