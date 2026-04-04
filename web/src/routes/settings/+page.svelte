<script lang="ts">
</script>

<svelte:head>
	<title>wshm - Settings</title>
</svelte:head>

<div class="page-header">
	<h2>Settings</h2>
	<p>Current configuration (read-only)</p>
</div>

<div class="card">
	<h3>Configuration</h3>
	<p class="note">
		Settings are loaded from <code>.wshm/config.toml</code> in your repository.
		Edit that file directly to change configuration.
	</p>

	<div class="config-section">
		<h4>Triage</h4>
		<dl>
			<dt>Enabled</dt><dd>true</dd>
			<dt>Auto-fix</dt><dd>false</dd>
			<dt>Auto-fix confidence</dt><dd>0.85</dd>
		</dl>
	</div>

	<div class="config-section">
		<h4>PR Analysis</h4>
		<dl>
			<dt>Enabled</dt><dd>true</dd>
			<dt>Auto-label</dt><dd>true</dd>
			<dt>Risk labels</dt><dd>true</dd>
		</dl>
	</div>

	<div class="config-section">
		<h4>Merge Queue</h4>
		<dl>
			<dt>Enabled</dt><dd>true</dd>
			<dt>Merge threshold</dt><dd>15</dd>
			<dt>Strategy</dt><dd>rebase</dd>
		</dl>
	</div>

	<div class="config-section">
		<h4>Conflicts</h4>
		<dl>
			<dt>Enabled</dt><dd>true</dd>
			<dt>Auto-resolve</dt><dd>false</dd>
			<dt>Auto-resolve confidence</dt><dd>0.85</dd>
		</dl>
	</div>

	<div class="config-section">
		<h4>Sync</h4>
		<dl>
			<dt>Interval</dt><dd>5 minutes</dd>
			<dt>Full sync interval</dt><dd>24 hours</dd>
		</dl>
	</div>
</div>

<style>
	h3 {
		font-size: 1rem;
		color: #e6edf3;
		margin-bottom: 0.5rem;
	}

	.note {
		color: #8b949e;
		font-size: 0.875rem;
		margin-bottom: 1.5rem;
	}

	code {
		background: #21262d;
		padding: 0.125rem 0.375rem;
		border-radius: 0.25rem;
		font-size: 0.8125rem;
		color: #e6edf3;
	}

	.config-section {
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid #21262d;
	}

	.config-section:last-child {
		border-bottom: none;
		margin-bottom: 0;
		padding-bottom: 0;
	}

	h4 {
		font-size: 0.875rem;
		color: #58a6ff;
		margin-bottom: 0.5rem;
		font-weight: 600;
	}

	dl {
		display: grid;
		grid-template-columns: 180px 1fr;
		gap: 0.25rem 1rem;
	}

	dt {
		color: #8b949e;
		font-size: 0.875rem;
	}

	dd {
		color: #c9d1d9;
		font-size: 0.875rem;
		font-family: ui-monospace, SFMono-Regular, 'SF Mono', Menlo, monospace;
	}
</style>
