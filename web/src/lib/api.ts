import { get } from 'svelte/store';
import { selectedRepo } from './stores';

const BASE = '/api/v1';

function repoParams(): Record<string, string> {
	const repo = get(selectedRepo);
	return repo ? { repo } : {};
}

async function apiGet<T>(path: string, params?: Record<string, string>): Promise<T> {
	// `new URL` already splits "/logs?tail=100" into pathname="/logs" and
	// searchParams="tail=100". We prepend BASE to `url.pathname` (NOT the
	// raw `path`) so a literal '?' in the original path is not
	// percent-encoded into the pathname (oauth2-proxy then sees a path
	// that doesn't match any backend route and falls through to the SPA).
	const url = new URL(path, window.location.origin);
	url.pathname = `${BASE}${url.pathname}`;
	const merged = { ...repoParams(), ...params };
	for (const [key, value] of Object.entries(merged)) {
		url.searchParams.set(key, value);
	}
	const res = await fetch(url.toString());
	if (!res.ok) {
		throw new Error(`API error: ${res.status} ${res.statusText}`);
	}
	// oauth2-proxy returns HTML (200 or 302→follow→HTML) when the SSO
	// session expired silently. Without this guard `res.json()` throws
	// "Unexpected token '<'". Surface a clean message instead of
	// auto-redirecting (auto-redirect caused a re-login loop on the
	// /logs polling page).
	const ct = res.headers.get('content-type') ?? '';
	if (ct.includes('text/html')) {
		throw new Error('SSO session expired — refresh the page to sign in');
	}
	return res.json();
}

export interface RepoInfo {
	slug: string;
	open_issues: number;
	untriaged: number;
	open_prs: number;
	unanalyzed: number;
	conflicts: number;
	last_sync: string | null;
	apply: boolean;
}

export interface Status {
	open_issues: number;
	untriaged: number;
	open_prs: number;
	unanalyzed: number;
	conflicts: number;
	last_sync: string | null;
	repos: RepoInfo[];
}

export interface Issue {
	repo: string;
	number: number;
	title: string;
	body: string | null;
	state: string;
	labels: string[];
	author: string | null;
	priority: string | null;
	category: string | null;
	pr_status: string | null;
	created_at: string;
	updated_at: string;
}

export interface PullRequest {
	repo: string;
	number: number;
	title: string;
	body: string | null;
	state: string;
	labels: string[];
	author: string | null;
	head_ref: string | null;
	base_ref: string | null;
	risk: string | null;
	risk_level: string | null;
	pr_type: string | null;
	summary: string | null;
	ci_status: string | null;
	mergeable: boolean | null;
	created_at: string;
	updated_at: string;
}

export interface TriageResult {
	repo: string;
	issue_number: number;
	category: string;
	confidence: number;
	priority: string;
	summary: string | null;
	acted_at: string | null;
}

export interface QueueEntry {
	repo: string;
	pr_number: number;
	title: string;
	score: number;
	ci_passing: boolean;
	approvals: number;
	has_conflicts: boolean;
	risk: string | null;
}

export interface ActivityEntry {
	id: number;
	action: string;
	target_type: string;
	target_number: number;
	summary: string;
	created_at: string;
}

export function fetchStatus(): Promise<Status> {
	return apiGet<Status>('/status');
}

export interface IssueBrief {
	number: number;
	title: string;
	priority: string | null;
	category: string | null;
	labels: string[];
	age_days: number;
}

export interface PrBrief {
	number: number;
	title: string;
	risk_level: string | null;
	ci_status: string | null;
	has_conflicts: boolean;
	age_days: number;
}

export interface Summary {
	repo: string;
	timestamp: string;
	open_issues: number;
	untriaged_issues: number;
	high_priority_issues: IssueBrief[];
	top_issues: IssueBrief[];
	open_prs: number;
	unanalyzed_prs: number;
	high_risk_prs: PrBrief[];
	top_prs: PrBrief[];
	conflicts: number;
}

export function fetchSummary(): Promise<Summary> {
	return apiGet<Summary>('/summary');
}

export function fetchIssues(state: string = 'open'): Promise<Issue[]> {
	return apiGet<Issue[]>('/issues', { state });
}

export function fetchPulls(state: string = 'open'): Promise<PullRequest[]> {
	return apiGet<PullRequest[]>('/pulls', { state });
}

export function fetchTriage(): Promise<TriageResult[]> {
	return apiGet<TriageResult[]>('/triage');
}

export function fetchQueue(): Promise<QueueEntry[]> {
	return apiGet<QueueEntry[]>('/queue');
}

export function fetchActivity(): Promise<ActivityEntry[]> {
	return apiGet<ActivityEntry[]>('/activity');
}

// ---------------------------------------------------------------------------
// Changelog
// ---------------------------------------------------------------------------

export interface ChangelogPr {
	repo: string;
	number: number;
	title: string;
	author: string | null;
	labels: string[];
	merged_at: string;
}

export interface ChangelogSection {
	name: string;
	pull_requests: ChangelogPr[];
}

export interface ChangelogResult {
	sections: ChangelogSection[];
}

export function fetchChangelog(): Promise<ChangelogResult> {
	return apiGet<ChangelogResult>('/changelog');
}

// ---------------------------------------------------------------------------
// Revert
// ---------------------------------------------------------------------------

export interface RevertRepo {
	repo: string;
	triage_results: number;
	pr_analyses: number;
	labels_to_remove: number;
}

export interface RevertPreview {
	repos: RevertRepo[];
}

export function fetchRevertPreview(): Promise<RevertPreview> {
	return apiGet<RevertPreview>('/revert/preview');
}

// ---------------------------------------------------------------------------
// Backups
// ---------------------------------------------------------------------------

export interface BackupEntry {
	name: string;
	path: string;
	size: number;
	created_at: string;
}

export interface BackupsResult {
	backups: BackupEntry[];
}

export function fetchBackups(): Promise<BackupsResult> {
	return apiGet<BackupsResult>('/backups');
}

export async function createBackup(): Promise<{ status: string; message: string }> {
	const res = await fetch(`${BASE}/backup`, { method: 'POST' });
	return res.json();
}

export async function restoreBackup(filename: string): Promise<{ status: string; message: string }> {
	const res = await fetch(`${BASE}/restore`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ path: filename }),
	});
	return res.json();
}

// ---------------------------------------------------------------------------
// Sync (manual)
// ---------------------------------------------------------------------------

export interface SyncResult {
	status: string;
	synced: string[];
	errors: { repo: string; error: string }[];
}

export async function syncFull(): Promise<SyncResult> {
	const url = new URL('/api/v1/sync/full', window.location.origin);
	const repo = get(selectedRepo);
	if (repo) url.searchParams.set('repo', repo);
	const res = await fetch(url.toString(), { method: 'POST' });
	return res.json();
}

export async function syncIncremental(): Promise<SyncResult> {
	const url = new URL('/api/v1/sync/incremental', window.location.origin);
	const repo = get(selectedRepo);
	if (repo) url.searchParams.set('repo', repo);
	const res = await fetch(url.toString(), { method: 'POST' });
	return res.json();
}

// ---------------------------------------------------------------------------
// License
// ---------------------------------------------------------------------------

export interface LicenseFeature {
	id: string;
	label: string;
	enabled: boolean;
}

export interface LicenseInfo {
	is_pro: boolean;
	plan: string;
	features: LicenseFeature[];
	oss_features: string[];
}

export function fetchLicense(): Promise<LicenseInfo> {
	return apiGet<LicenseInfo>('/license');
}

export async function activateLicense(key: string): Promise<{ status: string; plan?: string; message: string }> {
	const res = await fetch(`${BASE}/license/activate`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify({ license_key: key }),
	});
	return res.json();
}

// ---------------------------------------------------------------------------
// Repos & Auth (Settings page)
// ---------------------------------------------------------------------------

export interface RepoListEntry {
	slug: string;
	apply: boolean;
	wshm_dir: string;
}

export interface ReposListResponse {
	repos: RepoListEntry[];
	dynamic_add_supported: boolean;
}

export function fetchRepos(): Promise<ReposListResponse> {
	return apiGet<ReposListResponse>('/repos');
}

async function apiPost<T>(path: string, body: unknown): Promise<T> {
	const res = await fetch(`${BASE}${path}`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(body),
	});
	const json = await res.json();
	if (!res.ok) {
		const msg = (json && (json.error || json.message)) || `HTTP ${res.status}`;
		throw new Error(msg);
	}
	return json as T;
}

export function addRepo(slug: string, path?: string): Promise<{ status: string; slug: string; path: string; message: string }> {
	return apiPost('/repos', path ? { slug, path } : { slug });
}

export interface AuthStatus {
	github: boolean;
	anthropic: 'oauth' | 'api_key' | null;
}

export function fetchAuthStatus(): Promise<AuthStatus> {
	return apiGet<AuthStatus>('/auth/status');
}

export function setGithubToken(token: string): Promise<{ status: string; message: string }> {
	return apiPost('/auth/github', { token });
}

export function setAnthropicToken(token: string, kind: 'oauth' | 'api_key'): Promise<{ status: string; message: string }> {
	return apiPost('/auth/anthropic', { token, kind });
}

export type Role = 'admin' | 'operator' | 'member' | 'viewer';

export interface RepoFeatures {
	collect_issues: boolean;
	collect_prs: boolean;
	triage_issues: boolean;
	analyze_prs: boolean;
	review_prs: boolean;
	auto_pr: boolean;
	auto_merge: boolean;
}

export async function fetchRepoFeatures(slug: string): Promise<RepoFeatures> {
	return apiGet<RepoFeatures>(`/repos/${encodeURIComponent(slug)}/features`);
}

export async function updateRepoFeatures(
	slug: string,
	patch: Partial<RepoFeatures>
): Promise<RepoFeatures> {
	const res = await fetch(`/api/v1/repos/${encodeURIComponent(slug)}/features`, {
		method: 'PATCH',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(patch)
	});
	if (!res.ok) {
		const body = await res.json().catch(() => ({}));
		throw new Error(body.error ?? `HTTP ${res.status}`);
	}
	return res.json();
}

export interface Me {
	id?: number;
	email: string | null;
	username: string | null;
	role?: Role;
	auth_method: 'sso' | 'local';
}

export function fetchMe(): Promise<Me> {
	return apiGet<Me>('/auth/me');
}

export interface UserRecord {
	id: number;
	email: string;
	username: string | null;
	role: Role;
	sso_provider: string | null;
	created_at: string;
	last_login_at: string | null;
}

export interface UsersListResponse {
	users: UserRecord[];
}

export function fetchUsers(): Promise<UsersListResponse> {
	return apiGet<UsersListResponse>('/users');
}

export async function createUser(payload: {
	email: string;
	username?: string;
	password: string;
	role: Role;
}): Promise<{ id: number }> {
	return apiPost('/users', payload);
}

export async function updateUser(
	id: number,
	payload: { role?: Role; password?: string }
): Promise<{ status: string }> {
	const res = await fetch(`/api/v1/users/${id}`, {
		method: 'PATCH',
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(payload)
	});
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}

export async function deleteUser(id: number): Promise<{ status: string }> {
	const res = await fetch(`/api/v1/users/${id}`, { method: 'DELETE' });
	if (!res.ok) throw new Error(await res.text());
	return res.json();
}

export interface LogEntry {
	id: number;
	at: string;
	level: 'ERROR' | 'WARN' | 'INFO' | 'DEBUG' | 'TRACE';
	target: string;
	message: string;
}

export interface LogsResponse {
	entries: LogEntry[];
	last_id: number | null;
}

export function fetchLogs(opts: { tail?: number; level?: string; since?: number } = {}): Promise<LogsResponse> {
	// Params go through apiGet's second argument so they get attached via
	// URLSearchParams. Putting them in the path string causes apiGet's
	// `url.pathname = ${BASE}${path}` to percent-encode the `?`, which
	// makes oauth2-proxy hand the request to the SPA fallback (returns
	// HTML and breaks JSON parsing).
	const params: Record<string, string> = {};
	if (opts.tail !== undefined) params.tail = String(opts.tail);
	if (opts.level) params.level = opts.level;
	if (opts.since !== undefined) params.since = String(opts.since);
	return apiGet<LogsResponse>('/logs', params);
}

export interface SecretRecord {
	id: number;
	scope: 'global' | 'repo';
	slug: string | null;
	key: string;
	value: string;     // always "••••••••" except after a reveal
	updated_at: string;
	updated_by: number | null;
}

export function fetchSecrets(): Promise<{ secrets: SecretRecord[] }> {
	return apiGet('/secrets');
}

export function putSecret(input: {
	scope: 'global' | 'repo';
	slug?: string;
	key: string;
	value: string;
}): Promise<{ id: number }> {
	return apiPost('/secrets', input);
}

export async function revealSecret(id: number): Promise<{ value: string }> {
	const res = await fetch(`${BASE}/secrets/${id}/reveal`, { method: 'POST' });
	const json = await res.json();
	if (!res.ok) throw new Error(json.error ?? `HTTP ${res.status}`);
	return json;
}

export async function deleteSecret(id: number): Promise<{ status: string }> {
	const res = await fetch(`${BASE}/secrets/${id}`, { method: 'DELETE' });
	const json = await res.json();
	if (!res.ok) throw new Error(json.error ?? `HTTP ${res.status}`);
	return json;
}
