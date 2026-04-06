<script lang="ts">
	import type { Snippet } from 'svelte';
	import { onMount } from 'svelte';
	import { selectedRepo } from '$lib/stores';
	import { fetchStatus, type RepoInfo } from '$lib/api';

	let { children }: { children: Snippet } = $props();

	let repos: RepoInfo[] = $state([]);

	const navItems = [
		{ href: '/', label: 'Dashboard' },
		{ href: '/issues', label: 'Issues' },
		{ href: '/prs', label: 'Pull Requests' },
		{ href: '/triage', label: 'Triage' },
		{ href: '/queue', label: 'Merge Queue' },
		{ href: '/activity', label: 'Activity' },
		{ href: '/settings', label: 'Settings' }
	];

	function handleRepoChange(event: Event) {
		const value = (event.target as HTMLSelectElement).value;
		selectedRepo.set(value === '' ? null : value);
	}

	onMount(async () => {
		try {
			const status = await fetchStatus();
			repos = status.repos;
		} catch {
			// silently ignore — repos list will stay empty
		}
	});
</script>

<div class="app">
	<nav class="sidebar">
		<div class="logo">
			<h1>wshm</h1>
			<span class="tagline">wishmaster</span>
		</div>
		<div class="repo-selector">
			<label for="repo-select">Repository</label>
			<select id="repo-select" onchange={handleRepoChange}>
				<option value="">All repos</option>
				{#each repos as r}
					<option value={r.slug}>{r.slug}</option>
				{/each}
			</select>
		</div>
		<ul>
			{#each navItems as item}
				<li>
					<a href={item.href}>{item.label}</a>
				</li>
			{/each}
		</ul>
	</nav>
	<main class="content">
		{@render children()}
	</main>
</div>

<style>
	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	:global(body) {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
		background: #0d1117;
		color: #c9d1d9;
		line-height: 1.5;
	}

	:global(a) {
		color: #58a6ff;
		text-decoration: none;
	}

	:global(a:hover) {
		text-decoration: underline;
	}

	:global(table) {
		width: 100%;
		border-collapse: collapse;
	}

	:global(th),
	:global(td) {
		padding: 0.625rem 0.75rem;
		text-align: left;
		border-bottom: 1px solid #21262d;
	}

	:global(th) {
		color: #8b949e;
		font-weight: 600;
		font-size: 0.8125rem;
		text-transform: uppercase;
		letter-spacing: 0.04em;
	}

	:global(th.sortable) {
		cursor: pointer;
		user-select: none;
		white-space: nowrap;
	}

	:global(th.sortable:hover) {
		color: #e6edf3;
	}

	:global(th .sort-arrow) {
		display: inline-block;
		width: 1em;
		text-align: center;
		color: #484f58;
		font-size: 0.75rem;
	}

	:global(th .sort-arrow.active) {
		color: #58a6ff;
	}

	:global(tr:hover) {
		background: #161b22;
	}

	:global(.badge) {
		display: inline-block;
		padding: 0.125rem 0.5rem;
		border-radius: 999px;
		font-size: 0.75rem;
		font-weight: 600;
	}

	:global(.badge-green) {
		background: #1b4332;
		color: #3fb950;
	}

	:global(.badge-yellow) {
		background: #3d2e00;
		color: #d29922;
	}

	:global(.badge-red) {
		background: #3d1418;
		color: #f85149;
	}

	:global(.badge-blue) {
		background: #0c2d6b;
		color: #58a6ff;
	}

	:global(.badge-gray) {
		background: #21262d;
		color: #8b949e;
	}

	:global(.card) {
		background: #161b22;
		border: 1px solid #21262d;
		border-radius: 0.5rem;
		padding: 1.25rem;
	}

	:global(h2) {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: 1rem;
		color: #e6edf3;
	}

	:global(.page-header) {
		margin-bottom: 1.5rem;
	}

	:global(.page-header h2) {
		margin-bottom: 0.25rem;
	}

	:global(.page-header p) {
		color: #8b949e;
		font-size: 0.875rem;
	}

	.app {
		display: flex;
		min-height: 100vh;
	}

	.sidebar {
		width: 220px;
		background: #161b22;
		border-right: 1px solid #21262d;
		padding: 1.25rem 0;
		flex-shrink: 0;
		position: fixed;
		top: 0;
		left: 0;
		bottom: 0;
		overflow-y: auto;
	}

	.logo {
		padding: 0 1.25rem 1.25rem;
		border-bottom: 1px solid #21262d;
		margin-bottom: 0.75rem;
	}

	.logo h1 {
		font-size: 1.375rem;
		font-weight: 700;
		color: #e6edf3;
		letter-spacing: -0.02em;
	}

	.tagline {
		font-size: 0.75rem;
		color: #484f58;
	}

	.repo-selector {
		padding: 0.5rem 1.25rem 0.75rem;
		border-bottom: 1px solid #21262d;
		margin-bottom: 0.5rem;
	}

	.repo-selector label {
		display: block;
		font-size: 0.6875rem;
		text-transform: uppercase;
		letter-spacing: 0.04em;
		color: #484f58;
		margin-bottom: 0.375rem;
	}

	.repo-selector select {
		width: 100%;
		background: #0d1117;
		color: #c9d1d9;
		border: 1px solid #30363d;
		border-radius: 0.375rem;
		padding: 0.375rem 0.5rem;
		font-size: 0.8125rem;
		font-family: inherit;
		cursor: pointer;
		outline: none;
	}

	.repo-selector select:focus {
		border-color: #58a6ff;
	}

	.repo-selector select option {
		background: #161b22;
		color: #c9d1d9;
	}

	ul {
		list-style: none;
	}

	li a {
		display: block;
		padding: 0.5rem 1.25rem;
		color: #8b949e;
		font-size: 0.875rem;
		transition: color 0.15s, background 0.15s;
	}

	li a:hover {
		color: #e6edf3;
		background: #21262d;
		text-decoration: none;
	}

	.content {
		flex: 1;
		margin-left: 220px;
		padding: 2rem;
		max-width: 1200px;
	}
</style>
