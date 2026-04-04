<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchPulls, type PullRequest } from '$lib/api';

	let pulls: PullRequest[] = $state([]);
	let error: string | null = $state(null);

	function timeAgo(dateStr: string): string {
		const diff = Date.now() - new Date(dateStr).getTime();
		const days = Math.floor(diff / 86400000);
		if (days === 0) return 'today';
		if (days === 1) return '1 day ago';
		return `${days} days ago`;
	}

	onMount(async () => {
		try {
			pulls = await fetchPulls();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load pull requests';
		}
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
					<th>#</th>
					<th>Title</th>
					<th>State</th>
					<th>Risk</th>
					<th>CI</th>
					<th>Conflicts</th>
					<th>Age</th>
				</tr>
			</thead>
			<tbody>
				{#each pulls as pr}
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
