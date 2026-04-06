<script lang="ts">
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchStatus, type Status } from '$lib/api';

	let status: Status | null = $state(null);
	let error: string | null = $state(null);

	async function load() {
		try {
			error = null;
			status = await fetchStatus();
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load status';
		}
	}

	onMount(() => {
		load();
		const unsub = selectedRepo.subscribe(() => { load(); });
		return unsub;
	});
</script>

<svelte:head>
	<title>wshm - Dashboard</title>
</svelte:head>

<div class="page-header">
	<h2>Dashboard</h2>
	<p>Repository status overview</p>
</div>

{#if error}
	<div class="card" style="border-color: #f85149;">
		<p style="color: #f85149;">{error}</p>
		<p style="color: #8b949e; font-size: 0.875rem; margin-top: 0.5rem;">
			Make sure the wshm server is running.
		</p>
	</div>
{:else}
	<div class="stats-grid">
		<div class="card stat-card">
			<div class="stat-label">Open Issues</div>
			<div class="stat-value">{status?.open_issues ?? '--'}</div>
		</div>
		<div class="card stat-card">
			<div class="stat-label">Open PRs</div>
			<div class="stat-value">{status?.open_prs ?? '--'}</div>
		</div>
		<div class="card stat-card">
			<div class="stat-label">Untriaged</div>
			<div class="stat-value">{status?.untriaged ?? '--'}</div>
		</div>
		<div class="card stat-card">
			<div class="stat-label">Conflicts</div>
			<div class="stat-value">{status?.conflicts ?? '--'}</div>
		</div>
	</div>

	<div class="card" style="margin-top: 1.5rem;">
		<h2>Sync Status</h2>
		<p style="color: #8b949e; font-size: 0.875rem;">
			Last sync: {status?.last_sync ?? 'Never'}
		</p>
	</div>
{/if}

<style>
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1rem;
	}

	.stat-card {
		text-align: center;
	}

	.stat-label {
		color: #8b949e;
		font-size: 0.8125rem;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		margin-bottom: 0.5rem;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		color: #e6edf3;
	}
</style>
