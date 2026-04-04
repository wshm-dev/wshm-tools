const BASE = '/api/v1';

async function get<T>(path: string, params?: Record<string, string>): Promise<T> {
	const url = new URL(path, window.location.origin);
	url.pathname = `${BASE}${path}`;
	if (params) {
		for (const [key, value] of Object.entries(params)) {
			url.searchParams.set(key, value);
		}
	}
	const res = await fetch(url.toString());
	if (!res.ok) {
		throw new Error(`API error: ${res.status} ${res.statusText}`);
	}
	return res.json();
}

export interface Status {
	issues_open: number;
	issues_closed: number;
	prs_open: number;
	prs_closed: number;
	untriaged: number;
	conflicts: number;
	last_sync: string | null;
}

export interface Issue {
	number: number;
	title: string;
	state: string;
	labels: string[];
	priority: string | null;
	category: string | null;
	created_at: string;
	updated_at: string;
}

export interface PullRequest {
	number: number;
	title: string;
	state: string;
	risk: string | null;
	ci_status: string | null;
	has_conflicts: boolean;
	created_at: string;
	updated_at: string;
}

export interface TriageResult {
	issue_number: number;
	category: string;
	confidence: number;
	priority: string;
	acted_at: string | null;
}

export interface QueueEntry {
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
	return get<Status>('/status');
}

export function fetchIssues(state: string = 'open'): Promise<Issue[]> {
	return get<Issue[]>('/issues', { state });
}

export function fetchPulls(state: string = 'open'): Promise<PullRequest[]> {
	return get<PullRequest[]>('/pulls', { state });
}

export function fetchTriage(): Promise<TriageResult[]> {
	return get<TriageResult[]>('/triage');
}

export function fetchQueue(): Promise<QueueEntry[]> {
	return get<QueueEntry[]>('/queue');
}

export function fetchActivity(): Promise<ActivityEntry[]> {
	return get<ActivityEntry[]>('/activity');
}
