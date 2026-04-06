<script lang="ts">
	import { onMount } from 'svelte';
	import { fetchLicense } from '$lib/api';
	import { Card } from 'flowbite-svelte';
	import type { Snippet } from 'svelte';

	let { feature, children }: { feature: string; children: Snippet } = $props();
	let allowed = $state(false);
	let loading = $state(true);

	onMount(async () => {
		try {
			const lic = await fetchLicense();
			allowed = lic.features.find((f) => f.id === feature)?.enabled ?? false;
		} catch {
			// ignore
		}
		loading = false;
	});
</script>

{#if loading}
	<p class="text-gray-500">Loading...</p>
{:else if allowed}
	{@render children()}
{:else}
	<Card class="bg-gray-800 border-gray-700 text-center py-12">
		<div class="text-4xl mb-4 text-gray-600 font-bold">PRO</div>
		<h3 class="text-lg font-semibold text-gray-300 mb-2">This feature requires wshm Pro</h3>
		<p class="text-sm text-gray-500 mb-4">Unlock {feature} and all premium features.</p>
		<a
			href="https://wshm.dev/pro"
			target="_blank"
			class="inline-block rounded bg-blue-600 px-4 py-2 text-sm text-white hover:bg-blue-500"
			>Upgrade to Pro</a
		>
		<p class="text-xs text-gray-600 mt-3">Or enter your license key in Settings</p>
	</Card>
{/if}
