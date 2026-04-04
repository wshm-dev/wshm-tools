<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchQueue, type QueueEntry } from '$lib/api';

	let entries: QueueEntry[] = $state([]);
	let error: string | null = $state(null);

	onMount(async () => {
		try {
			entries = await fetchQueue();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load merge queue';
		}
	});
</script>

<svelte:head>
	<title>wshm - Merge Queue</title>
</svelte:head>

<div class="page-header">
	<h2>Merge Queue</h2>
	<p>Pull requests ranked by merge readiness score</p>
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
					<th>Rank</th>
					<th>PR</th>
					<th>Title</th>
					<th>Score</th>
					<th>CI</th>
					<th>Approvals</th>
					<th>Conflicts</th>
					<th>Risk</th>
				</tr>
			</thead>
			<tbody>
				{#each entries as entry, i}
					<tr>
						<td class="rank">{i + 1}</td>
						<td>#{entry.pr_number}</td>
						<td>{entry.title}</td>
						<td>
							<span class="score"
								class:score-high={entry.score >= 15}
								class:score-mid={entry.score >= 5 && entry.score < 15}
								class:score-low={entry.score < 5}>
								{entry.score}
							</span>
						</td>
						<td>
							{#if entry.ci_passing}
								<span class="badge badge-green">passing</span>
							{:else}
								<span class="badge badge-red">failing</span>
							{/if}
						</td>
						<td>{entry.approvals}</td>
						<td>
							{#if entry.has_conflicts}
								<span class="badge badge-red">yes</span>
							{:else}
								<span class="badge badge-green">no</span>
							{/if}
						</td>
						<td>
							{#if entry.risk}
								<span class="badge"
									class:badge-green={entry.risk === 'low'}
									class:badge-yellow={entry.risk === 'medium'}
									class:badge-red={entry.risk === 'high'}>
									{entry.risk}
								</span>
							{:else}
								<span class="muted">-</span>
							{/if}
						</td>
					</tr>
				{:else}
					<tr>
						<td colspan="8" class="empty">No pull requests in queue</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

<style>
	.rank {
		color: #484f58;
		font-weight: 700;
		font-size: 0.875rem;
	}

	.score {
		font-weight: 700;
		font-variant-numeric: tabular-nums;
	}

	.score-high {
		color: #3fb950;
	}

	.score-mid {
		color: #d29922;
	}

	.score-low {
		color: #f85149;
	}

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
