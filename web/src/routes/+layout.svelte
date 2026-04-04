<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();

	const navItems = [
		{ href: '/', label: 'Dashboard' },
		{ href: '/issues', label: 'Issues' },
		{ href: '/prs', label: 'Pull Requests' },
		{ href: '/triage', label: 'Triage' },
		{ href: '/queue', label: 'Merge Queue' },
		{ href: '/activity', label: 'Activity' },
		{ href: '/settings', label: 'Settings' }
	];
</script>

<div class="app">
	<nav class="sidebar">
		<div class="logo">
			<h1>wshm</h1>
			<span class="tagline">wishmaster</span>
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
