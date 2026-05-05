<script lang="ts">
	import { Card, Badge } from 'flowbite-svelte';
	import type { Issue } from '$lib/api';

	let { issue }: { issue: Issue } = $props();

	function ageDays(dateStr: string): number {
		return Math.floor((Date.now() - new Date(dateStr).getTime()) / 86400000);
	}

	let githubUrl = $derived(`https://github.com/${issue.repo}/issues/${issue.number}`);
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
