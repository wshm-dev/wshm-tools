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
</script>

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
