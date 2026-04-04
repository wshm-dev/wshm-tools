<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchActivity, type ActivityEntry } from '$lib/api';

	let activities: ActivityEntry[] = $state([]);
	let error: string | null = $state(null);

	function formatTime(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
	}

	onMount(async () => {
		try {
			activities = await fetchActivity();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load activity';
		}
	});
</script>

<svelte:head>
	<title>wshm - Activity</title>
</svelte:head>

<div class="page-header">
	<h2>Activity Log</h2>
	<p>Recent triage and analysis actions</p>
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
					<th>Time</th>
					<th>Action</th>
					<th>Target</th>
					<th>Summary</th>
				</tr>
			</thead>
			<tbody>
				{#each activities as entry}
					<tr>
						<td class="muted nowrap">{formatTime(entry.created_at)}</td>
						<td>
							<span class="badge"
								class:badge-blue={entry.action === 'triage'}
								class:badge-green={entry.action === 'merge'}
								class:badge-yellow={entry.action === 'analyze'}
								class:badge-gray={true}>
								{entry.action}
							</span>
						</td>
						<td class="nowrap">{entry.target_type} #{entry.target_number}</td>
						<td>{entry.summary}</td>
					</tr>
				{:else}
					<tr>
						<td colspan="4" class="empty">No activity recorded yet</td>
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

	.nowrap {
		white-space: nowrap;
	}

	.empty {
		text-align: center;
		color: #484f58;
		padding: 2rem 0;
	}
</style>
