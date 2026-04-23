<p align="center">
  <img src="../assets/wizard.png" alt="wshm" width="600"/>
</p>

<p align="center">
  <a href="../README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  Deutsch •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.pt.md">Português</a>
</p>

---

# wshm — KI-Agent fur GitHub-Repository-Wartung

**Your repo's wish is my command.**

wshm ist ein CLI-Tool + GitHub Action, das als autonomer Agent fur die Repository-Wartung fungiert.

## Wie es funktioniert

Ein einziges Binary. Ihre API-Schlussel. Ihre Daten. Null Infrastruktur.

```
GitHub API → Inkrementelle Synchronisation → Lokales SQLite → KI → Aktionen (Label, Kommentar, PR, Merge)
```

Drei Modi: CLI, GitHub Action oder persistenter Daemon mit Webhooks/Polling.

## Funktionen

- **Automatische Issue-Triage** — Klassifizierung, Labelling und Priorisierung durch KI
- **PR-Analyse** — Zusammenfassung, Risikobewertung, Review-Checkliste
- **Auto-Fix** — Erstellung von Draft-PRs fur einfache Bugs
- **Merge-Queue** — Scoring und Ranking von PRs, Auto-Merge uber Schwellenwert
- **Konfliktlosung** — Erkennung und automatische Losung (nie Force-Push)
- **Inline-Review** — KI-gestutzte Zeile-fur-Zeile Code-Review
- **Auto-Zuweisung** — Gewichtete Zufallszuweisung von Maintainern
- **Label-Blacklist** — Bestimmte Labels von der Anwendung ausschliessen
- **Periodische Neubewertung** — Veraltete Triage-Ergebnisse neu bewerten
- **Benachrichtigungen** — Tagliche Roadmap an Discord, Slack, Teams oder Webhook: Top 10 Issues und PRs, sortiert nach Prioritat und Alter
- **Dashboard und Berichte** — HTML-Dashboard und Markdown/PDF-Berichte
- **Vollstandig anpassbar** — Templates fur jeden Kommentar und fur das Branding

Eine einzige Binary. Ihre API-Schlussel. Ihre Daten. Null Infrastruktur.

## Sicherheit

- **Dry-run standardmassig** — `--apply` erforderlich, um Aktionen auszufuhren
- **Konfidenzschwelle** — keine autonome Aktion unterhalb von 0.85
- **Kein Force-Push** — Konfliktlosung uber neue Commits
- **Idempotent** — Neuausfuhrung = gleiches Ergebnis
- **Sichere Tokens** — immer aus Umgebungsvariablen

## Early Access

> **wshm befindet sich derzeit in der privaten Beta.**
>
> **[contact@rtk-ai.app](mailto:contact@rtk-ai.app)** — Early Access verfugbar.

---

<p align="center">
  <sub>Gebaut mit Rust. Keine Infra. Eine Binary.</sub><br>
  <sub>&copy; 2026 <a href="https://rtk-ai.app">rtk-ai</a></sub>
</p>
