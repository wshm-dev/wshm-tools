<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchActivity, type ActivityEntry } from '$lib/api';

	let activities: ActivityEntry[] = $state([]);
	let error: string | null = $state(null);
	let sortBy: string = $state('date');
	let sortAsc: boolean = $state(false);

	function formatTime(dateStr: string): string {
		return new Date(dateStr).toLocaleString();
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
		[...activities].sort((a, b) => {
			let cmp = 0;
			switch (sortBy) {
				case 'date': cmp = a.created_at.localeCompare(b.created_at); break;
				case 'type': cmp = a.action.localeCompare(b.action); break;
				default: cmp = 0;
			}
			return sortAsc ? cmp : -cmp;
		})
	);

	async function load() {
		try {
			error = null;
			activities = await fetchActivity();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load activity';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
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
					<th class="sortable" onclick={() => toggleSort('date')}>Time <span class={arrowClass('date')}>{arrow('date')}</span></th>
					<th class="sortable" onclick={() => toggleSort('type')}>Action <span class={arrowClass('type')}>{arrow('type')}</span></th>
					<th>Target</th>
					<th>Summary</th>
				</tr>
			</thead>
			<tbody>
				{#each sorted as entry}
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
