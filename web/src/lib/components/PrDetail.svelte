<script lang="ts">
	import { Card, Badge } from 'flowbite-svelte';
	import type { PullRequest } from '$lib/api';

	let { pr }: { pr: PullRequest } = $props();

	function ageDays(dateStr: string): number {
		return Math.floor((Date.now() - new Date(dateStr).getTime()) / 86400000);
	}

	function riskColor(risk: string | null): 'red' | 'yellow' | 'green' {
		if (risk === 'high') return 'red';
		if (risk === 'medium') return 'yellow';
		return 'green';
	}

	let githubUrl = $derived(`https://github.com/${pr.repo}/pull/${pr.number}`);
</script>

<div class="mb-3 flex items-center gap-2 text-xs">
	<a
		href={githubUrl}
		target="_blank"
		rel="noopener noreferrer"
		class="inline-flex items-center gap-1.5 text-blue-400 hover:text-blue-300 underline"
	>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="h-4 w-4" aria-hidden="true">
			<path d="M12 .5C5.65.5.5 5.65.5 12c0 5.08 3.29 9.39 7.86 10.91.58.1.79-.25.79-.56v-2c-3.2.7-3.88-1.36-3.88-1.36-.53-1.34-1.29-1.7-1.29-1.7-1.05-.72.08-.7.08-.7 1.16.08 1.77 1.19 1.77 1.19 1.04 1.78 2.72 1.27 3.39.97.1-.75.41-1.27.74-1.56-2.55-.29-5.24-1.28-5.24-5.69 0-1.26.45-2.29 1.18-3.1-.12-.29-.51-1.46.11-3.04 0 0 .97-.31 3.18 1.18a11 11 0 0 1 5.79 0c2.21-1.49 3.18-1.18 3.18-1.18.62 1.58.23 2.75.11 3.04.74.81 1.18 1.84 1.18 3.1 0 4.42-2.69 5.4-5.25 5.68.42.36.79 1.07.79 2.16v3.2c0 .31.21.67.8.56C20.21 21.39 23.5 17.08 23.5 12 23.5 5.65 18.35.5 12 .5z" />
		</svg>
		<span>{githubUrl}</span>
	</a>
</div>

<div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">State</div>
		<Badge color={pr.state === 'open' ? 'green' : 'red'}>{pr.state}</Badge>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Risk</div>
		<Badge color={riskColor(pr.risk_level)}>{pr.risk_level ?? '-'}</Badge>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">CI Status</div>
		<span class="text-gray-200">{pr.ci_status ?? '-'}</span>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Age</div>
		<span class="mono text-gray-200">{ageDays(pr.created_at)}d</span>
	</Card>
</div>

<div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Conflicts</div>
		<Badge color={pr.mergeable === false ? 'red' : 'green'}>
			{pr.mergeable === false ? 'Yes' : 'No'}
		</Badge>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Branch</div>
		<span class="text-gray-300 text-sm mono">{pr.head_ref ?? '-'} → {pr.base_ref ?? '-'}</span>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Author</div>
		<span class="text-gray-300">{pr.author ?? '-'}</span>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Created</div>
		<span class="mono text-gray-300">{pr.created_at?.slice(0, 10)}</span>
	</Card>
</div>

{#if pr.labels && pr.labels.length > 0}
	<Card class="bg-gray-800 p-3 mb-4 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-2">Labels</div>
		<div class="flex flex-wrap gap-1">
			{#each pr.labels as label}
				<Badge color="blue">{label}</Badge>
			{/each}
		</div>
	</Card>
{/if}

{#if pr.body}
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-2">Description</div>
		<pre class="text-sm text-gray-300 whitespace-pre-wrap break-words max-h-96 overflow-y-auto">{pr.body}</pre>
	</Card>
{/if}
