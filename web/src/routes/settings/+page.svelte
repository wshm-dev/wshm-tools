<script lang="ts">
	import { Card } from 'flowbite-svelte';
	import { colorConfig, type ColorConfig } from '$lib/colors';

	let colors: ColorConfig = $state({ ...colorConfig.defaults });
	colorConfig.subscribe(c => colors = { ...c });

	function save() {
		colorConfig.save(colors);
	}

	function reset() {
		colorConfig.reset();
		colors = { ...colorConfig.defaults };
	}
</script>

<svelte:head>
	<title>wshm - Settings</title>
</svelte:head>

<div class="mb-6">
	<h2 class="text-xl font-semibold text-gray-100 mb-1">Settings</h2>
	<p class="text-sm text-gray-500">Configuration and display preferences</p>
</div>

<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
	<Card class="bg-gray-800 border-gray-700">
		<h3 class="text-base font-semibold text-gray-100 mb-4">Color Scheme</h3>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-3">Issue PR Status</h4>
			<div class="space-y-2">
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.noPr} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">No PR linked</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.noPr}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.hasPr} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">PR open</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.hasPr}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.prReady} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">PR ready to merge</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.prReady}</span>
				</label>
			</div>
		</div>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-3">Priority</h4>
			<div class="space-y-2">
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.critical} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Critical</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.critical}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.high} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">High</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.high}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.medium} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Medium</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.medium}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.low} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Low</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.low}</span>
				</label>
			</div>
		</div>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-3">PR Risk Level</h4>
			<div class="space-y-2">
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.riskHigh} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">High risk</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.riskHigh}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.riskMedium} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Medium risk</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.riskMedium}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.riskLow} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Low risk</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.riskLow}</span>
				</label>
			</div>
		</div>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-3">Category</h4>
			<div class="space-y-2">
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.bug} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Bug</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.bug}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.feature} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Feature / Enhancement</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.feature}</span>
				</label>
				<label class="flex items-center gap-3">
					<input type="color" bind:value={colors.docs} onchange={save} class="w-8 h-6 rounded border border-gray-600 bg-transparent cursor-pointer" />
					<span class="text-sm text-gray-300">Documentation</span>
					<span class="ml-auto text-xs mono text-gray-500">{colors.docs}</span>
				</label>
			</div>
		</div>

		<div class="flex gap-2">
			<button onclick={reset} class="rounded border border-gray-600 px-3 py-1.5 text-sm text-gray-400 hover:border-red-500 hover:text-red-400">
				Reset to defaults
			</button>
		</div>
	</Card>

	<Card class="bg-gray-800 border-gray-700">
		<h3 class="text-base font-semibold text-gray-100 mb-4">Configuration</h3>
		<p class="text-sm text-gray-500 mb-6">
			Loaded from <code class="rounded bg-gray-700 px-1.5 py-0.5 text-sm text-gray-100">.wshm/config.toml</code>
		</p>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-2">Triage</h4>
			<dl class="grid grid-cols-[160px_1fr] gap-x-4 gap-y-1">
				<dt class="text-sm text-gray-500">Enabled</dt><dd class="text-sm text-gray-300 mono">true</dd>
				<dt class="text-sm text-gray-500">Auto-fix</dt><dd class="text-sm text-gray-300 mono">false</dd>
				<dt class="text-sm text-gray-500">Confidence</dt><dd class="text-sm text-gray-300 mono">0.85</dd>
			</dl>
		</div>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-2">PR Analysis</h4>
			<dl class="grid grid-cols-[160px_1fr] gap-x-4 gap-y-1">
				<dt class="text-sm text-gray-500">Enabled</dt><dd class="text-sm text-gray-300 mono">true</dd>
				<dt class="text-sm text-gray-500">Auto-label</dt><dd class="text-sm text-gray-300 mono">true</dd>
				<dt class="text-sm text-gray-500">Risk labels</dt><dd class="text-sm text-gray-300 mono">true</dd>
			</dl>
		</div>

		<div class="mb-4 border-b border-gray-700 pb-4">
			<h4 class="text-sm font-semibold text-blue-400 mb-2">Merge Queue</h4>
			<dl class="grid grid-cols-[160px_1fr] gap-x-4 gap-y-1">
				<dt class="text-sm text-gray-500">Threshold</dt><dd class="text-sm text-gray-300 mono">15</dd>
				<dt class="text-sm text-gray-500">Strategy</dt><dd class="text-sm text-gray-300 mono">rebase</dd>
			</dl>
		</div>

		<div>
			<h4 class="text-sm font-semibold text-blue-400 mb-2">Sync</h4>
			<dl class="grid grid-cols-[160px_1fr] gap-x-4 gap-y-1">
				<dt class="text-sm text-gray-500">Interval</dt><dd class="text-sm text-gray-300 mono">5 min</dd>
				<dt class="text-sm text-gray-500">Full sync</dt><dd class="text-sm text-gray-300 mono">24h</dd>
			</dl>
		</div>
	</Card>
</div>

<div class="mt-4">
	<Card class="bg-gray-800 border-gray-700">
		<h3 class="text-base font-semibold text-gray-100 mb-3">Legend</h3>
		<div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
			<div>
				<h4 class="text-gray-500 mb-2 text-xs uppercase">Issue PR Status</h4>
				<div class="space-y-1">
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.noPr}"></span> No PR</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.hasPr}"></span> PR open</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.prReady}"></span> PR ready</div>
				</div>
			</div>
			<div>
				<h4 class="text-gray-500 mb-2 text-xs uppercase">Priority</h4>
				<div class="space-y-1">
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.critical}"></span> Critical</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.high}"></span> High</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.medium}"></span> Medium</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.low}"></span> Low</div>
				</div>
			</div>
			<div>
				<h4 class="text-gray-500 mb-2 text-xs uppercase">Risk</h4>
				<div class="space-y-1">
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.riskHigh}"></span> High</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.riskMedium}"></span> Medium</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.riskLow}"></span> Low</div>
				</div>
			</div>
			<div>
				<h4 class="text-gray-500 mb-2 text-xs uppercase">Category</h4>
				<div class="space-y-1">
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.bug}"></span> Bug</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.feature}"></span> Feature</div>
					<div class="flex items-center gap-2"><span class="w-3 h-3 rounded" style="background: {colors.docs}"></span> Docs</div>
				</div>
			</div>
		</div>
	</Card>
</div>
