import { writable, derived, get } from 'svelte/store';
import en from './en.json';
import fr from './fr.json';

export type Locale = 'en' | 'fr';

export const LOCALES: { code: Locale; flag: string; label: string }[] = [
	{ code: 'en', flag: '🇬🇧', label: 'English' },
	{ code: 'fr', flag: '🇫🇷', label: 'Français' }
];

const messages: Record<Locale, Record<string, string>> = { en, fr };

function detectInitial(): Locale {
	if (typeof window === 'undefined') return 'en';
	try {
		const saved = localStorage.getItem('wshm-locale');
		if (saved === 'en' || saved === 'fr') return saved;
	} catch {
		/* ignore */
	}
	if (typeof navigator !== 'undefined' && navigator.language?.toLowerCase().startsWith('fr')) {
		return 'fr';
	}
	return 'en';
}

export const locale = writable<Locale>(detectInitial());

locale.subscribe((v) => {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem('wshm-locale', v);
	} catch {
		/* ignore */
	}
});

export function tr(key: string, l?: Locale): string {
	const lc = l ?? get(locale);
	return messages[lc][key] ?? messages.en[key] ?? key;
}

export const t = derived(locale, ($locale) => (key: string) => tr(key, $locale));
