<script lang="ts">
	import { onMount } from 'svelte';
	import {
		Card,
		Badge,
		Tabs,
		TabItem,
		Button,
		Input,
		Label,
		Helper,
		Alert,
		Heading,
		Radio,
		Table,
		TableHead,
		TableHeadCell,
		TableBody,
		TableBodyRow,
		TableBodyCell,
		Modal,
	} from 'flowbite-svelte';
	import { colorConfig, type ColorConfig } from '$lib/colors';
	import { t } from '$lib/i18n';

	let translate = $state<(k: string) => string>((k) => k);
	t.subscribe((fn) => (translate = fn));
	import {
		fetchLicense,
		activateLicense,
		fetchRepos,
		addRepo,
		fetchAuthStatus,
		setGithubToken,
		setAnthropicToken,
		fetchSecrets,
		putSecret,
		revealSecret,
		deleteSecret,
		fetchUsers,
		createUser,
		updateUser,
		deleteUser,
		fetchRepoFeatures,
		updateRepoFeatures,
		type RepoFeatures,
		type LicenseInfo,
		type ReposListResponse,
		type AuthStatus,
		type SecretRecord,
		type UserRecord,
		type Role,
	} from '$lib/api';

	let colors: ColorConfig = $state({ ...colorConfig.defaults });
	colorConfig.subscribe(c => (colors = { ...c }));

	// License
	let license: LicenseInfo | null = $state(null);
	let licenseKey: string = $state('');
	let activating: boolean = $state(false);
	let activateMessage: string | null = $state(null);
	let activateError: boolean = $state(false);

	// Repositories
	let reposList: ReposListResponse | null = $state(null);
	let newRepoSlug: string = $state('');
	let newRepoPath: string = $state('');
	let addingRepo: boolean = $state(false);
	let addRepoMessage: string | null = $state(null);
	let addRepoError: boolean = $state(false);

	// Auth
	let authStatus: AuthStatus | null = $state(null);
	let ghToken: string = $state('');
	let savingGh: boolean = $state(false);
	let ghMessage: string | null = $state(null);
	let ghError: boolean = $state(false);

	let anthropicToken: string = $state('');
	let anthropicKind: 'oauth' | 'api_key' = $state('oauth');
	let savingAnthropic: boolean = $state(false);
	let anthropicMessage: string | null = $state(null);
	let anthropicError: boolean = $state(false);

	// Encrypted secrets
	let secrets: SecretRecord[] = $state([]);
	let secretsError: string | null = $state(null);
	let newSecretScope: 'global' | 'repo' = $state('global');
	let newSecretSlug: string = $state('');
	let newSecretKey: string = $state('');
	let newSecretValue: string = $state('');
	let savingSecret: boolean = $state(false);
	let secretMessage: string | null = $state(null);
	let secretMessageErr: boolean = $state(false);
	let revealedId: number | null = $state(null);
	let revealedValue: string | null = $state(null);

	async function refreshSecrets() {
		try {
			const r = await fetchSecrets();
			secrets = r.secrets;
			secretsError = null;
		} catch (e) {
			secretsError = e instanceof Error ? e.message : 'load failed';
		}
	}

	async function handleAddSecret() {
		if (!newSecretKey.trim() || !newSecretValue) return;
		savingSecret = true; secretMessage = null; secretMessageErr = false;
		try {
			await putSecret({
				scope: newSecretScope,
				slug: newSecretScope === 'repo' ? newSecretSlug.trim() : undefined,
				key: newSecretKey.trim(),
				value: newSecretValue
			});
			secretMessage = 'Secret saved (encrypted).';
			newSecretKey = ''; newSecretValue = ''; newSecretSlug = '';
			await refreshSecrets();
		} catch (e) {
			secretMessage = e instanceof Error ? e.message : 'save failed';
			secretMessageErr = true;
		}
		savingSecret = false;
	}

	async function handleReveal(id: number) {
		try {
			const r = await revealSecret(id);
			revealedId = id;
			revealedValue = r.value;
			// Auto-hide after 30s
			setTimeout(() => {
				if (revealedId === id) { revealedId = null; revealedValue = null; }
			}, 30000);
		} catch (e) {
			secretsError = e instanceof Error ? e.message : 'reveal failed';
		}
	}

	async function handleDeleteSecret(id: number) {
		try {
			await deleteSecret(id);
			if (revealedId === id) { revealedId = null; revealedValue = null; }
			await refreshSecrets();
		} catch (e) {
			secretsError = e instanceof Error ? e.message : 'delete failed';
		}
	}

	// Users (RBAC)
	let users: UserRecord[] = $state([]);
	let usersError: string | null = $state(null);
	let newUserEmail: string = $state('');
	let newUserUsername: string = $state('');
	let newUserPassword: string = $state('');
	let newUserRole: Role = $state('member');
	let creatingUser: boolean = $state(false);
	let userMessage: string | null = $state(null);
	let userMessageErr: boolean = $state(false);

	// Modal state for Users CRUD
	let createUserModalOpen: boolean = $state(false);
	let editUserModalOpen: boolean = $state(false);
	let deleteUserModalOpen: boolean = $state(false);
	let editingUser: UserRecord | null = $state(null);
	let editRole: Role = $state('member');
	let editPassword: string = $state('');
	let savingEdit: boolean = $state(false);
	let deletingUser: UserRecord | null = $state(null);
	let deletingNow: boolean = $state(false);

	function openCreateUser() {
		newUserEmail = ''; newUserUsername = ''; newUserPassword = ''; newUserRole = 'member';
		userMessage = null; userMessageErr = false;
		createUserModalOpen = true;
	}

	function openEditUser(u: UserRecord) {
		editingUser = u;
		editRole = u.role;
		editPassword = '';
		userMessage = null; userMessageErr = false;
		editUserModalOpen = true;
	}

	function openDeleteUser(u: UserRecord) {
		deletingUser = u;
		deleteUserModalOpen = true;
	}

	async function handleSaveEdit() {
		if (!editingUser) return;
		savingEdit = true;
		try {
			const payload: { role?: Role; password?: string } = {};
			if (editRole !== editingUser.role) payload.role = editRole;
			if (editPassword) {
				if (editPassword.length < 6) {
					userMessage = 'Password must be at least 6 characters';
					userMessageErr = true;
					savingEdit = false;
					return;
				}
				payload.password = editPassword;
			}
			if (Object.keys(payload).length === 0) {
				editUserModalOpen = false;
				savingEdit = false;
				return;
			}
			await updateUser(editingUser.id, payload);
			userMessage = `Updated ${editingUser.email}.`;
			userMessageErr = false;
			editUserModalOpen = false;
			await refreshUsers();
		} catch (e) {
			userMessage = e instanceof Error ? e.message : 'update failed';
			userMessageErr = true;
		}
		savingEdit = false;
	}

	// CSV → trimmed array, drops blanks. Used by the filters UI inputs.
	function parseCsv(s: string): string[] {
		return s.split(',').map((x) => x.trim()).filter((x) => x.length > 0);
	}

	/// Sensible default filter values for repos using GitHub's standard
	/// label set (bug, duplicate, wontfix, invalid, question, good first
	/// issue, help wanted, enhancement, documentation). Applied in one
	/// click via the "Apply GitHub defaults" button in the modal.
	function applyGithubDefaults() {
		if (!featuresDraft) return;
		featuresDraft.filters.skip_authors = [
			'dependabot[bot]',
			'renovate[bot]',
			'github-actions[bot]'
		];
		featuresDraft.filters.triage_skip_labels = [
			'wontfix',
			'duplicate',
			'invalid',
			'question'
		];
		featuresDraft.filters.auto_pr_only_labels = ['good first issue', 'help wanted'];
		featuresDraft.filters.auto_merge_only_authors = ['dependabot[bot]'];
		featuresDraft.filters.auto_merge_only_labels = ['auto-merge'];
		featuresDraft.filters.auto_merge_max_loc = 200;
		featuresDraft.filters.skip_drafts = true;
	}

	// Repo features (per-repo toggles)
	let featuresModalOpen: boolean = $state(false);
	let featuresSlug: string = $state('');
	let featuresDraft: RepoFeatures | null = $state(null);
	let featuresSaving: boolean = $state(false);
	let featuresMessage: string | null = $state(null);
	let featuresMessageErr: boolean = $state(false);

	async function openFeaturesModal(slug: string) {
		featuresSlug = slug;
		featuresDraft = null;
		featuresMessage = null;
		featuresMessageErr = false;
		featuresModalOpen = true;
		try {
			featuresDraft = await fetchRepoFeatures(slug);
		} catch (e) {
			featuresMessage = e instanceof Error ? e.message : 'Failed to load features';
			featuresMessageErr = true;
		}
	}

	async function handleSaveFeatures() {
		if (!featuresDraft) return;
		featuresSaving = true;
		try {
			await updateRepoFeatures(featuresSlug, featuresDraft);
			featuresMessage = 'Features saved.';
			featuresMessageErr = false;
			featuresModalOpen = false;
			await refreshRepos();
		} catch (e) {
			featuresMessage = e instanceof Error ? e.message : 'save failed';
			featuresMessageErr = true;
		}
		featuresSaving = false;
	}

	async function handleConfirmDelete() {
		if (!deletingUser) return;
		deletingNow = true;
		try {
			await deleteUser(deletingUser.id);
			userMessage = `Deleted ${deletingUser.email}.`;
			userMessageErr = false;
			deleteUserModalOpen = false;
			deletingUser = null;
			await refreshUsers();
		} catch (e) {
			userMessage = e instanceof Error ? e.message : 'delete failed';
			userMessageErr = true;
		}
		deletingNow = false;
	}

	async function refreshUsers() {
		try {
			const r = await fetchUsers();
			users = r.users;
			usersError = null;
		} catch (e) {
			usersError = e instanceof Error ? e.message : 'load failed';
		}
	}

	async function handleCreateUser() {
		if (!newUserEmail.trim() || !newUserPassword) return;
		creatingUser = true; userMessage = null; userMessageErr = false;
		try {
			await createUser({
				email: newUserEmail.trim(),
				username: newUserUsername.trim() || undefined,
				password: newUserPassword,
				role: newUserRole,
			});
			userMessage = `User ${newUserEmail} created.`;
			createUserModalOpen = false;
			await refreshUsers();
		} catch (e) {
			userMessage = e instanceof Error ? e.message : 'create failed';
			userMessageErr = true;
		}
		creatingUser = false;
	}

	function saveColors() { colorConfig.save(colors); }
	function resetColors() { colorConfig.reset(); colors = { ...colorConfig.defaults }; }

	async function refreshRepos() {
		try { reposList = await fetchRepos(); } catch { /* ignore */ }
	}
	async function refreshAuth() {
		try { authStatus = await fetchAuthStatus(); } catch { /* ignore */ }
	}

	async function handleAddRepo() {
		if (!newRepoSlug.trim()) return;
		addingRepo = true; addRepoMessage = null; addRepoError = false;
		try {
			const r = await addRepo(newRepoSlug.trim(), newRepoPath.trim() || undefined);
			addRepoMessage = r.message;
			newRepoSlug = ''; newRepoPath = '';
			await refreshRepos();
		} catch (e) {
			addRepoMessage = e instanceof Error ? e.message : 'Add failed';
			addRepoError = true;
		}
		addingRepo = false;
	}

	async function handleSetGithub() {
		if (!ghToken.trim()) return;
		savingGh = true; ghMessage = null; ghError = false;
		try {
			const r = await setGithubToken(ghToken.trim());
			ghMessage = r.message; ghToken = '';
			await refreshAuth();
		} catch (e) {
			ghMessage = e instanceof Error ? e.message : 'Save failed';
			ghError = true;
		}
		savingGh = false;
	}

	async function handleSetAnthropic() {
		if (!anthropicToken.trim()) return;
		savingAnthropic = true; anthropicMessage = null; anthropicError = false;
		try {
			const r = await setAnthropicToken(anthropicToken.trim(), anthropicKind);
			anthropicMessage = r.message; anthropicToken = '';
			await refreshAuth();
		} catch (e) {
			anthropicMessage = e instanceof Error ? e.message : 'Save failed';
			anthropicError = true;
		}
		savingAnthropic = false;
	}

	async function handleActivate() {
		if (!licenseKey.trim()) return;
		activating = true; activateMessage = null; activateError = false;
		try {
			const r = await activateLicense(licenseKey.trim());
			if (r.status === 'ok') {
				activateMessage = r.message; activateError = false; licenseKey = '';
				license = await fetchLicense();
			} else {
				activateMessage = r.message; activateError = true;
			}
		} catch (e) {
			activateMessage = e instanceof Error ? e.message : 'Activation failed';
			activateError = true;
		}
		activating = false;
	}

	onMount(async () => {
		try { license = await fetchLicense(); } catch { /* ignore */ }
		await refreshRepos();
		await refreshAuth();
		await refreshSecrets();
		await refreshUsers();
	});
</script>

<svelte:head>
	<title>wshm - Settings</title>
</svelte:head>

<div class="mb-4">
	<Heading tag="h2" class="text-xl mb-1">Settings</Heading>
	<p class="text-sm text-gray-500">Connections, license, appearance, and configuration</p>
</div>

<Tabs tabStyle="underline" contentClass="bg-transparent p-0 mt-4">
	<!-- ========================= REPOSITORIES ========================= -->
	<TabItem open title="Repositories">
		<div class="w-full">
			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">Repositories</Heading>

				{#if reposList}
					<div class="mb-3">
						<h4 class="text-xs font-semibold text-blue-400 mb-2">Configured ({reposList.repos.length})</h4>
						{#if reposList.repos.length === 0}
							<p class="text-xs text-gray-500">None configured.</p>
						{:else}
							<ul class="space-y-1 text-xs">
								{#each reposList.repos as r}
									<li class="flex items-center justify-between gap-2">
										<span class="text-gray-300 mono">{r.slug}</span>
										<div class="flex items-center gap-2">
											<Badge color={r.apply ? 'green' : 'dark'}>{r.apply ? 'apply' : 'dry-run'}</Badge>
											<Button color="alternative" size="xs" onclick={() => openFeaturesModal(r.slug)}>
												Edit features
											</Button>
										</div>
									</li>
								{/each}
							</ul>
						{/if}
					</div>

					<div class="border-t border-gray-700 pt-3 space-y-2">
						{#if addRepoMessage}
							<Alert color={addRepoError ? 'red' : 'green'} class="text-xs py-2">{addRepoMessage}</Alert>
						{/if}

						{#if reposList.dynamic_add_supported}
							<form onsubmit={(e) => { e.preventDefault(); handleAddRepo(); }} class="space-y-2">
								<div>
									<Label for="repo-slug" class="text-xs mb-1">Slug</Label>
									<Input id="repo-slug" type="text" bind:value={newRepoSlug} placeholder="owner/repo" disabled={addingRepo} size="sm" />
								</div>
								<div>
									<Label for="repo-path" class="text-xs mb-1">Path (optional)</Label>
									<Input id="repo-path" type="text" bind:value={newRepoPath} placeholder="/abs/path" disabled={addingRepo} size="sm" />
								</div>
								<Button type="submit" color="blue" disabled={addingRepo || !newRepoSlug.trim()} size="sm" class="w-full">
									{addingRepo ? 'Adding...' : 'Add repository'}
								</Button>
							</form>
						{:else}
							<Helper>
								Dynamic add not available — daemon running mono-repo. Edit
								<code class="rounded bg-gray-700 px-1 py-0.5">~/.wshm/global.toml</code> and restart.
							</Helper>
						{/if}
					</div>
				{:else}
					<p class="text-sm text-gray-500">Loading...</p>
				{/if}
			</Card>
		</div>
	</TabItem>

	<!-- ========================= GIT PROVIDERS ========================= -->
	<TabItem title="Git providers">
		<div class="w-full">
			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">GitHub authentication</Heading>

				{#if authStatus}
					<div class="mb-3">
						<Badge large color={authStatus.github ? 'green' : 'dark'}>
							{authStatus.github ? 'Configured' : 'Not configured'}
						</Badge>
					</div>

					<Helper class="mb-3">
						Personal Access Token. <a href="https://github.com/settings/tokens" target="_blank" class="text-blue-400 hover:underline">Generate one</a> with <code class="rounded bg-gray-700 px-1 py-0.5">repo</code> scope.
					</Helper>

					{#if ghMessage}
						<Alert color={ghError ? 'red' : 'green'} class="text-xs py-2 mb-2">{ghMessage}</Alert>
					{/if}

					<form onsubmit={(e) => { e.preventDefault(); handleSetGithub(); }} class="space-y-2">
						<div>
							<Label for="gh-token" class="text-xs mb-1">Token</Label>
							<Input id="gh-token" type="password" bind:value={ghToken} placeholder="ghp_..." disabled={savingGh} size="sm" />
						</div>
						<Button type="submit" color="blue" disabled={savingGh || !ghToken.trim()} size="sm" class="w-full">
							{savingGh ? 'Saving...' : 'Save token'}
						</Button>
					</form>
				{:else}
					<p class="text-sm text-gray-500">Loading...</p>
				{/if}
			</Card>
			<Helper class="mt-3 text-xs">More providers (GitLab, Gitea, Forgejo, Azure DevOps) coming soon.</Helper>
		</div>
	</TabItem>

	<!-- ========================= AI PROVIDERS ========================= -->
	<TabItem title="AI providers">
		<div class="w-full">
			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">Claude / Anthropic authentication</Heading>

				{#if authStatus}
					<div class="mb-3">
						<Badge large color={authStatus.anthropic ? 'green' : 'dark'}>
							{authStatus.anthropic === 'oauth'
								? 'OAuth (Max/Pro)'
								: authStatus.anthropic === 'api_key'
									? 'API key'
									: 'Not configured'}
						</Badge>
					</div>

					<Helper class="mb-3">
						OAuth token (run <code class="rounded bg-gray-700 px-1 py-0.5">claude /token</code> in Claude Code) or an
						<a href="https://console.anthropic.com/" target="_blank" class="text-blue-400 hover:underline">API key</a>.
					</Helper>

					{#if anthropicMessage}
						<Alert color={anthropicError ? 'red' : 'green'} class="text-xs py-2 mb-2">{anthropicMessage}</Alert>
					{/if}

					<form onsubmit={(e) => { e.preventDefault(); handleSetAnthropic(); }} class="space-y-2">
						<div class="flex gap-4 text-xs">
							<Radio bind:group={anthropicKind} value="oauth" disabled={savingAnthropic}>OAuth</Radio>
							<Radio bind:group={anthropicKind} value="api_key" disabled={savingAnthropic}>API key</Radio>
						</div>
						<div>
							<Label for="anth-token" class="text-xs mb-1">Token</Label>
							<Input id="anth-token" type="password" bind:value={anthropicToken} placeholder={anthropicKind === 'oauth' ? 'sk-ant-oat01-...' : 'sk-ant-api03-...'} disabled={savingAnthropic} size="sm" />
						</div>
						<Button type="submit" color="blue" disabled={savingAnthropic || !anthropicToken.trim()} size="sm" class="w-full">
							{savingAnthropic ? 'Saving...' : 'Save token'}
						</Button>
					</form>
				{:else}
					<p class="text-sm text-gray-500">Loading...</p>
				{/if}
			</Card>
			<Helper class="mt-3 text-xs">More providers (OpenAI, Gemini, Ollama, Azure OpenAI) coming soon.</Helper>
		</div>
	</TabItem>

	<!-- ============================ LICENSE ============================ -->
	<TabItem title="License">
		<Card class="bg-gray-800 border-gray-700 max-w-none">
			<Heading tag="h3" class="text-base mb-4">License</Heading>

			{#if license}
				<div class="flex items-center gap-3 mb-4">
					<Badge large color={license.is_pro ? 'green' : 'dark'}>{license.plan.toUpperCase()}</Badge>
					{#if !license.is_pro}
						<span class="text-sm text-gray-400">Free tier</span>
					{/if}
				</div>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
					<div>
						<h4 class="text-sm font-semibold text-gray-400 mb-2">OSS Features (included)</h4>
						<div class="flex flex-wrap gap-1">
							{#each license.oss_features as f}
								<Badge color="blue">{f}</Badge>
							{/each}
						</div>
					</div>
					<div>
						<h4 class="text-sm font-semibold text-gray-400 mb-2">Pro Features</h4>
						<div class="space-y-1">
							{#each license.features as f}
								<div class="flex items-center justify-between text-sm">
									<span class="text-gray-300">{f.label}</span>
									<Badge color={f.enabled ? 'green' : 'dark'}>{f.enabled ? 'Active' : 'Locked'}</Badge>
								</div>
							{/each}
						</div>
					</div>
				</div>

				<div class="border-t border-gray-700 pt-3">
					{#if activateMessage}
						<Alert color={activateError ? 'red' : 'green'} class="text-xs py-2 mb-2">{activateMessage}</Alert>
					{/if}

					<Helper class="mb-2">{license.is_pro ? 'Update license key:' : 'Enter your license key:'}</Helper>
					<form onsubmit={(e) => { e.preventDefault(); handleActivate(); }} class="flex gap-2">
						<Input type="text" bind:value={licenseKey} placeholder="wshm-pro-xxxx-xxxx-xxxx" disabled={activating} size="sm" class="flex-1" />
						<Button type="submit" color="blue" disabled={activating || !licenseKey.trim()} size="sm">
							{activating ? 'Activating...' : 'Activate'}
						</Button>
					</form>

					{#if !license.is_pro}
						<p class="text-xs text-gray-500 mt-2">
							<a href="https://wshm.dev/pro" target="_blank" class="text-blue-400 hover:underline">Get a license</a>
						</p>
					{/if}
				</div>
			{:else}
				<p class="text-sm text-gray-500">Loading...</p>
			{/if}
		</Card>
	</TabItem>

	<!-- ========================== APPEARANCE ========================== -->
	<TabItem title="Appearance">
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">Color Scheme</Heading>

				<div class="mb-3 border-b border-gray-700 pb-3">
					<h4 class="text-xs font-semibold text-blue-400 mb-2">Issue PR Status</h4>
					<div class="space-y-1.5">
						{#each [['noPr', 'No PR'], ['hasPr', 'PR open'], ['prReady', 'PR ready']] as [key, label]}
							<label class="flex items-center gap-2">
								<input type="color" bind:value={colors[key]} onchange={saveColors} class="w-6 h-5 rounded border border-gray-600 bg-transparent cursor-pointer" />
								<span class="text-xs text-gray-300">{label}</span>
								<span class="ml-auto text-[0.6rem] mono text-gray-600">{colors[key]}</span>
							</label>
						{/each}
					</div>
				</div>

				<div class="mb-3 border-b border-gray-700 pb-3">
					<h4 class="text-xs font-semibold text-blue-400 mb-2">Priority</h4>
					<div class="space-y-1.5">
						{#each [['critical', 'Critical'], ['high', 'High'], ['medium', 'Medium'], ['low', 'Low']] as [key, label]}
							<label class="flex items-center gap-2">
								<input type="color" bind:value={colors[key]} onchange={saveColors} class="w-6 h-5 rounded border border-gray-600 bg-transparent cursor-pointer" />
								<span class="text-xs text-gray-300">{label}</span>
								<span class="ml-auto text-[0.6rem] mono text-gray-600">{colors[key]}</span>
							</label>
						{/each}
					</div>
				</div>

				<div class="mb-3 border-b border-gray-700 pb-3">
					<h4 class="text-xs font-semibold text-blue-400 mb-2">Risk / Category</h4>
					<div class="space-y-1.5">
						{#each [['riskHigh', 'Risk: High'], ['riskMedium', 'Risk: Medium'], ['riskLow', 'Risk: Low'], ['bug', 'Bug'], ['feature', 'Feature'], ['docs', 'Docs']] as [key, label]}
							<label class="flex items-center gap-2">
								<input type="color" bind:value={colors[key]} onchange={saveColors} class="w-6 h-5 rounded border border-gray-600 bg-transparent cursor-pointer" />
								<span class="text-xs text-gray-300">{label}</span>
								<span class="ml-auto text-[0.6rem] mono text-gray-600">{colors[key]}</span>
							</label>
						{/each}
					</div>
				</div>

				<Button onclick={resetColors} color="alternative" size="xs">Reset defaults</Button>
			</Card>

			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">Color Legend</Heading>
				<div class="grid grid-cols-2 gap-4 text-xs text-gray-300">
					<div>
						<h4 class="text-gray-500 mb-1 text-[0.6rem] uppercase">PR Status</h4>
						{#each [['noPr', 'No PR'], ['hasPr', 'PR open'], ['prReady', 'PR ready']] as [key, label]}
							<div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded inline-block" style="background: {colors[key]}"></span> {label}</div>
						{/each}
					</div>
					<div>
						<h4 class="text-gray-500 mb-1 text-[0.6rem] uppercase">Priority</h4>
						{#each [['critical', 'Critical'], ['high', 'High'], ['medium', 'Medium'], ['low', 'Low']] as [key, label]}
							<div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded inline-block" style="background: {colors[key]}"></span> {label}</div>
						{/each}
					</div>
					<div>
						<h4 class="text-gray-500 mb-1 text-[0.6rem] uppercase">Risk</h4>
						{#each [['riskHigh', 'High'], ['riskMedium', 'Medium'], ['riskLow', 'Low']] as [key, label]}
							<div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded inline-block" style="background: {colors[key]}"></span> {label}</div>
						{/each}
					</div>
					<div>
						<h4 class="text-gray-500 mb-1 text-[0.6rem] uppercase">Category</h4>
						{#each [['bug', 'Bug'], ['feature', 'Feature'], ['docs', 'Docs']] as [key, label]}
							<div class="flex items-center gap-1.5"><span class="w-2.5 h-2.5 rounded inline-block" style="background: {colors[key]}"></span> {label}</div>
						{/each}
					</div>
				</div>
			</Card>
		</div>
	</TabItem>

	<!-- ========================= CONFIGURATION ========================= -->
	<TabItem title="Configuration">
		<Card class="bg-gray-800 border-gray-700 max-w-none">
			<Heading tag="h3" class="text-base mb-4">Configuration</Heading>
			<Helper class="mb-4">
				Read from <code class="rounded bg-gray-700 px-1 py-0.5">.wshm/config.toml</code>. Edit the file and restart the daemon to change.
			</Helper>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
				{#each [
					['Triage', [['Enabled', 'true'], ['Auto-fix', 'false'], ['Confidence', '0.85']]],
					['PR Analysis', [['Enabled', 'true'], ['Auto-label', 'true'], ['Risk labels', 'true']]],
					['Merge Queue', [['Threshold', '15'], ['Strategy', 'rebase']]],
					['Sync', [['Interval', '5 min'], ['Full sync', '24h']]]
				] as [section, items]}
					<div class="border border-gray-700 rounded p-3">
						<h4 class="text-xs font-semibold text-blue-400 mb-2">{section}</h4>
						<dl class="grid grid-cols-[120px_1fr] gap-x-2 gap-y-0.5">
							{#each items as [key, val]}
								<dt class="text-xs text-gray-500">{key}</dt>
								<dd class="text-xs text-gray-300 mono">{val}</dd>
							{/each}
						</dl>
					</div>
				{/each}
			</div>
		</Card>
	</TabItem>

	<!-- ========================= SECRETS ============================ -->
	<TabItem title="Secrets">
		<!-- Disambiguation banner: this tab is for advanced / per-repo
		     secrets. Common GitHub / Anthropic tokens belong in their
		     dedicated tabs. -->
		<Alert color="blue" class="mb-4 text-sm">
			<span class="font-semibold">Advanced encrypted secrets store.</span>
			For your GitHub token, use <strong>Git providers</strong>; for Anthropic / OpenAI / Gemini API keys, use <strong>AI providers</strong>.
			This tab is for <em>additional</em> secrets (per-repo overrides, custom integrations) stored encrypted alongside the token store.
		</Alert>

		<!-- Doc / how-to: create a github_token. Toggleable so admins
		     who already know the drill don't see it every visit. -->
		<details class="mb-4 rounded border border-gray-700 bg-gray-800/60 open:bg-gray-800">
			<summary class="cursor-pointer px-4 py-3 text-sm font-semibold text-blue-300 hover:text-blue-200">
				ℹ️ {translate('secrets.help.title')}
			</summary>
			<div class="px-4 pb-4 pt-1 text-sm text-gray-300 space-y-2">
				<p>{translate('secrets.help.intro')}</p>
				<ol class="list-decimal list-inside space-y-1 ms-2">
					<li>{translate('secrets.help.step1')}</li>
					<li>{translate('secrets.help.step2')}</li>
					<li>{translate('secrets.help.step3')}</li>
					<li>{translate('secrets.help.step4')}</li>
				</ol>
				<p class="text-xs text-gray-400 italic">
					💡 {translate('secrets.help.tip')}
				</p>
				<a
					href="https://github.com/settings/tokens"
					target="_blank"
					rel="noopener noreferrer"
					class="inline-block mt-1 text-blue-400 hover:text-blue-300 underline text-xs"
				>
					→ {translate('secrets.help.link')}
				</a>
			</div>
		</details>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
			<!-- Stored secrets list -->
			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">Stored secrets</Heading>
				<Helper class="mb-3">
					Encrypted at rest with AES-256-GCM. Plaintext is never written to disk.
				</Helper>
				{#if secretsError}
					<Alert color="red" class="text-xs py-2 mb-2">{secretsError}</Alert>
				{/if}
				{#if secrets.length === 0}
					<p class="text-sm text-gray-500">No secrets stored. Use the form on the right to add one.</p>
				{:else}
					<Table hoverable={true} class="text-xs">
						<TableHead>
							<TableHeadCell>Scope</TableHeadCell>
							<TableHeadCell>Key</TableHeadCell>
							<TableHeadCell>Value</TableHeadCell>
							<TableHeadCell>Updated</TableHeadCell>
							<TableHeadCell><span class="sr-only">Actions</span></TableHeadCell>
						</TableHead>
						<TableBody>
							{#each secrets as s (s.id)}
								<TableBodyRow>
									<TableBodyCell>
										<Badge color={s.scope === 'global' ? 'blue' : 'green'}>
											{s.scope}{s.slug ? `: ${s.slug}` : ''}
										</Badge>
									</TableBodyCell>
									<TableBodyCell class="mono text-gray-200">{s.key}</TableBodyCell>
									<TableBodyCell class="mono text-gray-300">
										{revealedId === s.id && revealedValue ? revealedValue : '••••••••'}
									</TableBodyCell>
									<TableBodyCell class="text-gray-500">
										{new Date(s.updated_at).toLocaleString()}
									</TableBodyCell>
									<TableBodyCell class="text-right whitespace-nowrap">
										<Button color="alternative" size="xs" onclick={() => handleReveal(s.id)}>
											{revealedId === s.id ? 'Hide' : 'Reveal'}
										</Button>
										<Button color="red" size="xs" onclick={() => handleDeleteSecret(s.id)}>
											Delete
										</Button>
									</TableBodyCell>
								</TableBodyRow>
							{/each}
						</TableBody>
					</Table>
				{/if}
			</Card>

			<!-- Add new secret -->
			<Card class="bg-gray-800 border-gray-700 max-w-none">
				<Heading tag="h3" class="text-base mb-4">Add a secret</Heading>
				{#if secretMessage}
					<Alert color={secretMessageErr ? 'red' : 'green'} class="text-xs py-2 mb-2">
						{secretMessage}
					</Alert>
				{/if}
				<form onsubmit={(e) => { e.preventDefault(); handleAddSecret(); }} class="space-y-3">
					<div>
						<Label class="text-xs mb-1">Scope</Label>
						<div class="flex gap-3 text-sm">
							<Radio bind:group={newSecretScope} value="global">Global</Radio>
							<Radio bind:group={newSecretScope} value="repo">Per-repo</Radio>
						</div>
					</div>
					{#if newSecretScope === 'repo'}
						<div>
							<Label for="sec-slug" class="text-xs mb-1">Repository slug</Label>
							<Input id="sec-slug" type="text" bind:value={newSecretSlug}
								placeholder="owner/repo" disabled={savingSecret} size="sm" />
						</div>
					{/if}
					<div>
						<Label for="sec-key" class="text-xs mb-1">Key</Label>
						<Input id="sec-key" type="text" bind:value={newSecretKey}
							placeholder="github_token, anthropic_api_key, …"
							disabled={savingSecret} size="sm" />
						<Helper class="text-xs mt-1">
							Common keys: <code>github_token</code>, <code>anthropic_oauth_token</code>,
							<code>anthropic_api_key</code>.
						</Helper>
					</div>
					<div>
						<Label for="sec-value" class="text-xs mb-1">Value</Label>
						<Input id="sec-value" type="password" bind:value={newSecretValue}
							placeholder="paste secret value" disabled={savingSecret} size="sm" />
					</div>
					<Button type="submit" color="blue" size="sm" class="w-full"
						disabled={savingSecret || !newSecretKey.trim() || !newSecretValue
							|| (newSecretScope === 'repo' && !newSecretSlug.trim())}>
						{savingSecret ? 'Saving...' : 'Save secret'}
					</Button>
				</form>
			</Card>
		</div>
	</TabItem>

	<!-- ========================= USERS (RBAC) ========================= -->
	<TabItem title="Users">
		<Card class="bg-gray-800 border-gray-700 max-w-none">
			<div class="flex items-start justify-between mb-4 gap-3">
				<div>
					<Heading tag="h3" class="text-base">User accounts</Heading>
					<Helper class="mt-1">
						Local accounts and SSO-upserted users. Roles control access to admin
						pages (Settings) and mutating actions.
					</Helper>
				</div>
				<Button color="blue" size="sm" onclick={openCreateUser} class="shrink-0">
					+ Add user
				</Button>
			</div>
			{#if usersError}
				<Alert color="red" class="text-xs py-2 mb-2">{usersError}</Alert>
			{/if}
			{#if userMessage}
				<Alert color={userMessageErr ? 'red' : 'green'} class="text-xs py-2 mb-2">
					{userMessage}
				</Alert>
			{/if}
			{#if users.length === 0}
				<p class="text-sm text-gray-500">No users yet. Click "+ Add user" to create one.</p>
			{:else}
				<Table hoverable={true} class="text-xs">
					<TableHead>
						<TableHeadCell>Identity</TableHeadCell>
						<TableHeadCell>Auth</TableHeadCell>
						<TableHeadCell>Role</TableHeadCell>
						<TableHeadCell>Last login</TableHeadCell>
						<TableHeadCell><span class="sr-only">Actions</span></TableHeadCell>
					</TableHead>
					<TableBody>
						{#each users as u (u.id)}
							<TableBodyRow>
								<TableBodyCell>
									<div class="mono text-gray-200">{u.email}</div>
									{#if u.username && u.username !== u.email}
										<div class="text-[0.65rem] text-gray-500">@{u.username}</div>
									{/if}
								</TableBodyCell>
								<TableBodyCell>
									<Badge color={u.sso_provider ? 'purple' : 'blue'}>
										{u.sso_provider ?? 'local'}
									</Badge>
								</TableBodyCell>
								<TableBodyCell>
									<Badge color={u.role === 'admin' ? 'red' : u.role === 'operator' ? 'orange' : u.role === 'member' ? 'blue' : 'gray'}>
										{u.role}
									</Badge>
								</TableBodyCell>
								<TableBodyCell class="text-gray-500">
									{u.last_login_at ? new Date(u.last_login_at).toLocaleString() : '—'}
								</TableBodyCell>
								<TableBodyCell class="text-right whitespace-nowrap">
									<Button color="alternative" size="xs" onclick={() => openEditUser(u)}>
										Edit
									</Button>
									<Button color="red" size="xs" onclick={() => openDeleteUser(u)}>
										Delete
									</Button>
								</TableBodyCell>
							</TableBodyRow>
						{/each}
					</TableBody>
				</Table>
			{/if}
		</Card>
	</TabItem>
</Tabs>

<!-- Create user modal -->
<Modal
	bind:open={createUserModalOpen}
	title="Add user"
	size="md"
	dismissable
	class="bg-gray-900 border-gray-700"
	bodyClass="text-gray-200"
>
	<form onsubmit={(e) => { e.preventDefault(); handleCreateUser(); }} class="space-y-3">
		<div>
			<Label for="user-email" class="text-xs mb-1">Email or identifier</Label>
			<Input id="user-email" type="text" bind:value={newUserEmail}
				placeholder="alice@example.com or alice" disabled={creatingUser} size="sm" />
		</div>
		<div>
			<Label for="user-username" class="text-xs mb-1">Username (optional)</Label>
			<Input id="user-username" type="text" bind:value={newUserUsername}
				placeholder="alice" disabled={creatingUser} size="sm" />
		</div>
		<div>
			<Label for="user-password" class="text-xs mb-1">Password</Label>
			<Input id="user-password" type="password" bind:value={newUserPassword}
				placeholder="min 6 chars" disabled={creatingUser} size="sm" />
		</div>
		<div>
			<Label class="text-xs mb-1">Role</Label>
			<div class="flex flex-col gap-1 text-sm">
				<Radio bind:group={newUserRole} value="admin">
					<span class="font-semibold">admin</span>
					<span class="text-xs text-gray-500 ml-1">— full control (users, license, secrets)</span>
				</Radio>
				<Radio bind:group={newUserRole} value="operator">
					<span class="font-semibold">operator</span>
					<span class="text-xs text-gray-500 ml-1">— sync, merge, revert, backups</span>
				</Radio>
				<Radio bind:group={newUserRole} value="member">
					<span class="font-semibold">member</span>
					<span class="text-xs text-gray-500 ml-1">— triage, comments, labels</span>
				</Radio>
				<Radio bind:group={newUserRole} value="viewer">
					<span class="font-semibold">viewer</span>
					<span class="text-xs text-gray-500 ml-1">— read-only</span>
				</Radio>
			</div>
		</div>
		<div class="flex gap-2 pt-2">
			<Button color="alternative" size="sm" class="flex-1"
				onclick={() => createUserModalOpen = false} disabled={creatingUser}>
				Cancel
			</Button>
			<Button type="submit" color="blue" size="sm" class="flex-1"
				disabled={creatingUser || !newUserEmail.trim() || !newUserPassword || newUserPassword.length < 6}>
				{creatingUser ? 'Creating…' : 'Create'}
			</Button>
		</div>
	</form>
</Modal>

<!-- Edit user modal -->
<Modal
	bind:open={editUserModalOpen}
	title={editingUser ? `Edit ${editingUser.email}` : 'Edit user'}
	size="md"
	dismissable
	class="bg-gray-900 border-gray-700"
	bodyClass="text-gray-200"
>
	{#if editingUser}
		<form onsubmit={(e) => { e.preventDefault(); handleSaveEdit(); }} class="space-y-3">
			<div>
				<Label class="text-xs mb-1">Role</Label>
				<div class="flex flex-col gap-1 text-sm">
					<Radio bind:group={editRole} value="admin">
						<span class="font-semibold">admin</span>
						<span class="text-xs text-gray-500 ml-1">— full control</span>
					</Radio>
					<Radio bind:group={editRole} value="operator">
						<span class="font-semibold">operator</span>
						<span class="text-xs text-gray-500 ml-1">— sync, merge, revert, backups</span>
					</Radio>
					<Radio bind:group={editRole} value="member">
						<span class="font-semibold">member</span>
						<span class="text-xs text-gray-500 ml-1">— triage, comments, labels</span>
					</Radio>
					<Radio bind:group={editRole} value="viewer">
						<span class="font-semibold">viewer</span>
						<span class="text-xs text-gray-500 ml-1">— read-only</span>
					</Radio>
				</div>
			</div>
			<div>
				<Label for="edit-pw" class="text-xs mb-1">New password (leave empty to keep current)</Label>
				<Input id="edit-pw" type="password" bind:value={editPassword}
					placeholder="min 6 chars" disabled={savingEdit} size="sm" />
			</div>
			<div class="flex gap-2 pt-2">
				<Button color="alternative" size="sm" class="flex-1"
					onclick={() => editUserModalOpen = false} disabled={savingEdit}>
					Cancel
				</Button>
				<Button type="submit" color="blue" size="sm" class="flex-1" disabled={savingEdit}>
					{savingEdit ? 'Saving…' : 'Save'}
				</Button>
			</div>
		</form>
	{/if}
</Modal>

<!-- Edit features modal -->
<Modal
	bind:open={featuresModalOpen}
	title={featuresSlug ? `Features — ${featuresSlug}` : 'Features'}
	size="lg"
	dismissable
	class="bg-gray-900 border-gray-700"
	bodyClass="text-gray-200"
>
	{#if featuresMessage}
		<Alert color={featuresMessageErr ? 'red' : 'green'} class="text-xs py-2 mb-3">
			{featuresMessage}
		</Alert>
	{/if}
	{#if !featuresDraft}
		<p class="text-sm text-gray-500">Loading…</p>
	{:else}
		<div class="space-y-4">
			<div>
				<h4 class="text-xs uppercase text-gray-500 font-semibold mb-2">Read-only collection</h4>
				<div class="space-y-1.5">
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.collect_issues} class="rounded" />
						<span><strong>Collect issues</strong> <span class="text-xs text-gray-500">— sync open + closed issues to local DB</span></span>
					</label>
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.collect_prs} class="rounded" />
						<span><strong>Collect PRs</strong> <span class="text-xs text-gray-500">— sync pull requests + CI status</span></span>
					</label>
				</div>
			</div>

			<div>
				<h4 class="text-xs uppercase text-gray-500 font-semibold mb-2">AI classification</h4>
				<div class="space-y-1.5">
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.triage_issues} class="rounded" />
						<span><strong>Triage issues</strong> <span class="text-xs text-gray-500">— AI category/priority + label issues</span></span>
					</label>
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.analyze_prs} class="rounded" />
						<span><strong>Analyze PRs</strong> <span class="text-xs text-gray-500">— AI risk/type/summary + comment</span></span>
					</label>
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.review_prs} class="rounded" />
						<span>
							<strong>Review PRs (Pro)</strong>
							<span class="text-xs text-gray-500">— inline code review on PRs</span>
						</span>
					</label>
				</div>
			</div>

			<div>
				<h4 class="text-xs uppercase text-gray-500 font-semibold mb-2">Auto-actions</h4>
				<div class="space-y-1.5">
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.auto_pr} class="rounded" />
						<span><strong>Auto-fix PR</strong> <span class="text-xs text-gray-500">— open PRs for simple bugs</span></span>
					</label>
					<label class="flex items-center gap-2 text-sm">
						<input type="checkbox" bind:checked={featuresDraft.auto_merge} class="rounded" />
						<span><strong>Auto-merge</strong> <span class="text-xs text-gray-500">— merge ready PRs once checks pass</span></span>
					</label>
				</div>
			</div>

			<!-- Advanced filters: collapsible. Free-text comma-separated for arrays. -->
			<details class="rounded border border-gray-700 bg-gray-900/40">
				<summary class="cursor-pointer px-3 py-2 text-sm font-semibold text-blue-300 hover:text-blue-200">
					Advanced filters
				</summary>
				<div class="p-3 space-y-3 text-sm">
					<!-- One-click defaults aligned with GitHub's standard label set. -->
					<div class="flex items-start justify-between gap-3 rounded border border-blue-700/40 bg-blue-900/20 p-3">
						<div class="text-xs">
							<p class="font-semibold text-blue-300 mb-1">GitHub default labels</p>
							<p class="text-gray-400">
								Pre-fill sensible filters for repos using GitHub's standard label set
								(<code class="text-gray-300">bug</code>,
								<code class="text-gray-300">wontfix</code>,
								<code class="text-gray-300">duplicate</code>,
								<code class="text-gray-300">good first issue</code>, …).
								Click to apply, then tune below.
							</p>
						</div>
						<Button color="blue" size="xs" onclick={applyGithubDefaults} class="shrink-0">
							Apply GitHub defaults
						</Button>
					</div>

					<details class="rounded border border-gray-700 bg-gray-900/40">
						<summary class="cursor-pointer px-3 py-2 text-xs font-semibold text-gray-400 hover:text-gray-200">
							ℹ️ GitHub standard labels — what they mean
						</summary>
						<div class="p-3 text-xs space-y-1 text-gray-400">
							<div><code class="text-red-300">bug</code> — Something isn't working. <em>Triage candidate.</em></div>
							<div><code class="text-cyan-300">enhancement</code> — New feature or request.</div>
							<div><code class="text-blue-300">documentation</code> — Doc improvements.</div>
							<div><code class="text-yellow-300">good first issue</code> — Good for newcomers. <em>Auto-fix candidate.</em></div>
							<div><code class="text-green-300">help wanted</code> — Extra attention is needed.</div>
							<div><code class="text-purple-300">question</code> — Further info requested. <em>Skip triage (human judgment).</em></div>
							<div><code class="text-gray-500">duplicate</code> — Already exists. <em>Skip triage.</em></div>
							<div><code class="text-gray-500">invalid</code> — Doesn't seem right. <em>Skip triage.</em></div>
							<div><code class="text-gray-500">wontfix</code> — Will not be worked on. <em>Skip triage.</em></div>
						</div>
					</details>
					<div>
						<h5 class="text-xs uppercase text-gray-500 font-semibold mb-1">Global</h5>
						<Label class="text-xs mb-1">Skip authors (comma-separated)</Label>
						<Input
							size="sm"
							placeholder="dependabot[bot], renovate[bot]"
							value={featuresDraft.filters.skip_authors.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.skip_authors = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<Label class="text-xs mb-1 mt-2">Target branches (vide = toutes)</Label>
						<Input
							size="sm"
							placeholder="main, develop"
							value={featuresDraft.filters.target_branches.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.target_branches = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<label class="flex items-center gap-2 text-sm mt-2">
							<input type="checkbox" bind:checked={featuresDraft.filters.skip_drafts} class="rounded" />
							<span>Skip draft PRs</span>
						</label>
					</div>

					<div>
						<h5 class="text-xs uppercase text-gray-500 font-semibold mb-1">Triage</h5>
						<Label class="text-xs mb-1">Only labels (whitelist)</Label>
						<Input
							size="sm"
							placeholder="needs-triage, bug"
							value={featuresDraft.filters.triage_only_labels.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.triage_only_labels = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<Label class="text-xs mb-1 mt-2">Skip labels</Label>
						<Input
							size="sm"
							placeholder="wontfix, duplicate"
							value={featuresDraft.filters.triage_skip_labels.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.triage_skip_labels = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<Label class="text-xs mb-1 mt-2">Max age (days, 0 = no limit)</Label>
						<Input
							type="number"
							size="sm"
							bind:value={featuresDraft.filters.triage_max_age_days}
						/>
					</div>

					<div>
						<h5 class="text-xs uppercase text-gray-500 font-semibold mb-1">Analyze PRs</h5>
						<div class="grid grid-cols-2 gap-2">
							<div>
								<Label class="text-xs mb-1">Min LOC (0 = no min)</Label>
								<Input type="number" size="sm" bind:value={featuresDraft.filters.analyze_min_loc} />
							</div>
							<div>
								<Label class="text-xs mb-1">Max LOC (0 = no max)</Label>
								<Input type="number" size="sm" bind:value={featuresDraft.filters.analyze_max_loc} />
							</div>
						</div>
					</div>

					<div>
						<h5 class="text-xs uppercase text-gray-500 font-semibold mb-1">Auto-fix PR</h5>
						<Label class="text-xs mb-1">Only labels (whitelist)</Label>
						<Input
							size="sm"
							placeholder="good-first-issue, auto-fix"
							value={featuresDraft.filters.auto_pr_only_labels.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.auto_pr_only_labels = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<Label class="text-xs mb-1 mt-2">Target branch (empty = repo default)</Label>
						<Input size="sm" placeholder="main" bind:value={featuresDraft.filters.auto_pr_target_branch} />
					</div>

					<div>
						<h5 class="text-xs uppercase text-gray-500 font-semibold mb-1">Auto-merge</h5>
						<Label class="text-xs mb-1">Only authors (whitelist)</Label>
						<Input
							size="sm"
							placeholder="dependabot[bot]"
							value={featuresDraft.filters.auto_merge_only_authors.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.auto_merge_only_authors = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<Label class="text-xs mb-1 mt-2">Only labels (whitelist)</Label>
						<Input
							size="sm"
							placeholder="auto-merge"
							value={featuresDraft.filters.auto_merge_only_labels.join(', ')}
							onchange={(e) => {
								featuresDraft!.filters.auto_merge_only_labels = parseCsv((e.currentTarget as HTMLInputElement).value);
							}}
						/>
						<div class="grid grid-cols-2 gap-2 mt-2">
							<div>
								<Label class="text-xs mb-1">Min approvals</Label>
								<Input type="number" size="sm" bind:value={featuresDraft.filters.auto_merge_min_approvals} />
							</div>
							<div>
								<Label class="text-xs mb-1">Max LOC (0 = no max)</Label>
								<Input type="number" size="sm" bind:value={featuresDraft.filters.auto_merge_max_loc} />
							</div>
						</div>
					</div>
				</div>
			</details>

			<div class="flex gap-2 pt-2">
				<Button color="alternative" size="sm" class="flex-1"
					onclick={() => featuresModalOpen = false} disabled={featuresSaving}>
					Cancel
				</Button>
				<Button color="blue" size="sm" class="flex-1"
					onclick={handleSaveFeatures} disabled={featuresSaving}>
					{featuresSaving ? 'Saving…' : 'Save'}
				</Button>
			</div>
		</div>
	{/if}
</Modal>

<!-- Delete user confirm modal -->
<Modal
	bind:open={deleteUserModalOpen}
	title="Delete user"
	size="sm"
	dismissable
	class="bg-gray-900 border-gray-700"
	bodyClass="text-gray-200"
>
	{#if deletingUser}
		<p class="text-sm">
			Delete <span class="mono text-red-300">{deletingUser.email}</span>?
			This cannot be undone.
		</p>
		<div class="flex gap-2 pt-4">
			<Button color="alternative" size="sm" class="flex-1"
				onclick={() => deleteUserModalOpen = false} disabled={deletingNow}>
				Cancel
			</Button>
			<Button color="red" size="sm" class="flex-1"
				onclick={handleConfirmDelete} disabled={deletingNow}>
				{deletingNow ? 'Deleting…' : 'Delete'}
			</Button>
		</div>
	{/if}
</Modal>
