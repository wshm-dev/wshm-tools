<p align="center">
  <img src="../assets/wizard.png" alt="wshm" width="600"/>
</p>

<p align="center">
  <a href="../README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  Español •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  <a href="README.zh.md">中文</a> •
  <a href="README.ko.md">한국어</a> •
  <a href="README.pt.md">Português</a>
</p>

---

# wshm — Agente IA para el mantenimiento de repos GitHub

**Your repo's wish is my command.**

wshm es una herramienta CLI + GitHub Action que actua como agente autonomo de mantenimiento de repositorios.

## Como funciona

Un solo binario. Tus claves API. Tus datos. Cero infraestructura.

```
GitHub API → Sincronizacion incremental → SQLite local → IA → Acciones (etiqueta, comentario, PR, merge)
```

Tres modos: CLI, GitHub Action o daemon persistente con webhooks/polling.

## Caracteristicas

- **Triaje automatico** — Clasificacion, etiquetado y priorizacion de issues por IA
- **Analisis de PRs** — Resumen, evaluacion de riesgo, checklist de revision
- **Auto-fix** — Generacion de PRs draft para bugs simples
- **Cola de merge** — Scoring y clasificacion de PRs, auto-merge sobre el umbral
- **Resolucion de conflictos** — Deteccion y resolucion automatica (nunca force-push)
- **Review inline** — Comentarios de revision linea por linea por IA
- **Auto-asignacion** — Asignacion ponderada de maintainers
- **Blacklist de labels** — Impedir la aplicacion de ciertos labels
- **Retriage periodico** — Re-evaluacion de triajes obsoletos
- **Notificaciones** — Roadmap diario a Discord, Slack, Teams o webhook: top 10 issues y PRs pendientes, ordenados por prioridad y antiguedad
- **Dashboard y reportes** — Dashboard HTML y reportes markdown/PDF
- **Totalmente personalizable** — Plantillas para cada comentario y branding

Un solo binario. Tus claves API. Tus datos. Cero infraestructura.

## Seguridad

- **Dry-run por defecto** — `--apply` necesario para ejecutar acciones
- **Umbral de confianza** — nunca actua de forma autonoma por debajo de 0.85
- **Sin force-push** — resolucion de conflictos mediante nuevos commits
- **Idempotente** — re-ejecutar = mismo resultado
- **Tokens seguros** — siempre desde variables de entorno

## Acceso anticipado

> **wshm esta actualmente en beta privada.**
>
> **[contact@rtk-ai.app](mailto:contact@rtk-ai.app)** — Acceso anticipado disponible.

---

<p align="center">
  <sub>Construido con Rust. Cero infra. Un solo binario.</sub><br>
  <sub>&copy; 2026 <a href="https://rtk-ai.app">rtk-ai</a></sub>
</p>
