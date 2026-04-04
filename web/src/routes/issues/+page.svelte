<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchIssues, type Issue } from '$lib/api';

	let issues: Issue[] = $state([]);
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
			issues = await fetchIssues();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load issues';
		}
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
					<th>#</th>
					<th>Title</th>
					<th>State</th>
					<th>Labels</th>
					<th>Priority</th>
					<th>Category</th>
					<th>Age</th>
				</tr>
			</thead>
			<tbody>
				{#each issues as issue}
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
