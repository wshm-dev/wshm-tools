<script lang="ts">
	import { Card, Badge } from 'flowbite-svelte';
	import type { Issue } from '$lib/api';

	let { issue }: { issue: Issue } = $props();

	function ageDays(dateStr: string): number {
		return Math.floor((Date.now() - new Date(dateStr).getTime()) / 86400000);
	}
</script>

<div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">State</div>
		<Badge color={issue.state === 'open' ? 'green' : 'red'}>{issue.state}</Badge>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Priority</div>
		<span class="text-gray-200">{issue.priority ?? '-'}</span>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Category</div>
		<span class="text-gray-200">{issue.category ?? '-'}</span>
	</Card>
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-1">Age</div>
		<span class="mono text-gray-200">{ageDays(issue.created_at)}d</span>
	</Card>
</div>

{#if issue.labels.length > 0}
	<Card class="bg-gray-800 p-3 mb-4 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-2">Labels</div>
		<div class="flex flex-wrap gap-1">
			{#each issue.labels as label}
				<Badge color="blue">{label}</Badge>
			{/each}
		</div>
	</Card>
{/if}

<Card class="bg-gray-800 p-3 mb-4 max-w-none">
	<div class="text-[0.625rem] uppercase text-gray-500 mb-2">Details</div>
	<div class="grid grid-cols-2 gap-2 text-sm">
		<div><span class="text-gray-500">Author:</span> <span class="text-gray-300">{issue.author ?? '-'}</span></div>
		<div><span class="text-gray-500">Created:</span> <span class="text-gray-300 mono">{issue.created_at?.slice(0, 10)}</span></div>
		<div><span class="text-gray-500">Updated:</span> <span class="text-gray-300 mono">{issue.updated_at?.slice(0, 10)}</span></div>
	</div>
</Card>

{#if issue.body}
	<Card class="bg-gray-800 p-3 max-w-none">
		<div class="text-[0.625rem] uppercase text-gray-500 mb-2">Body</div>
		<pre class="text-sm text-gray-300 whitespace-pre-wrap break-words max-h-96 overflow-y-auto">{issue.body}</pre>
	</Card>
{/if}
