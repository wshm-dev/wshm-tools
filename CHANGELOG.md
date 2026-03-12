# Changelog

## 0.1.0 (2026-03-12)


### Features

* add 15 AI providers (Anthropic, OpenAI, Gemini, Mistral, Groq, DeepSeek, xAI, Together, Fireworks, Perplexity, Cohere, OpenRouter, Ollama, Azure, custom) ([a0d8e2e](https://github.com/pszymkowiak/wshm/commit/a0d8e2eb00c4c99d565c036c87297838f4ffbf5f))
* add CI/CD workflows and systemd install/uninstall ([#1](https://github.com/pszymkowiak/wshm/issues/1)) ([d8adef9](https://github.com/pszymkowiak/wshm/commit/d8adef90c364d39ee0da5793b7c709fd4b062d35))
* add Claude OAuth login (Max/Pro/Team subscription support) ([0f80031](https://github.com/pszymkowiak/wshm/commit/0f80031e7d454e1e3d775e90bee074d6123c2475))
* auto-fix security hardening ([#7](https://github.com/pszymkowiak/wshm/issues/7)) ([be3212c](https://github.com/pszymkowiak/wshm/commit/be3212c4962d8c7e6633349d24ec3bbfed9e9864))
* auto-fix trigger from triage pipeline ([#6](https://github.com/pszymkowiak/wshm/issues/6)) ([b5589c6](https://github.com/pszymkowiak/wshm/commit/b5589c64ef7811b9f191c24efc818352181a8caf))
* bot-style comments with emojis and automated banner ([#5](https://github.com/pszymkowiak/wshm/issues/5)) ([2b379b2](https://github.com/pszymkowiak/wshm/commit/2b379b222d11b214a7e4abcfae037090a6c07d00))
* daemon mode, login, polling, slash commands, branding ([b79b42b](https://github.com/pszymkowiak/wshm/commit/b79b42bf1806c17a99a9cc993262e23ef1ad3eae))
* initial project scaffold — CLI, DB, GitHub sync, AI pipelines ([f5263ee](https://github.com/pszymkowiak/wshm/commit/f5263ee6e440a563cee562ebef1d54212644dceb))
* secure auto-update with SHA256 checksum verification ([#2](https://github.com/pszymkowiak/wshm/issues/2)) ([eccca1a](https://github.com/pszymkowiak/wshm/commit/eccca1a4b0345c78f40070d78c42c5c3bde5586c))
* wshm — full initial implementation ([be69681](https://github.com/pszymkowiak/wshm/commit/be6968162d1281ad3e09f1bfa71819b0340bcc56))


### Bug Fixes

* add pagination for issues/PRs and gh auth token fallback ([465fbf4](https://github.com/pszymkowiak/wshm/commit/465fbf4ac01aba2ffb491640af611c0ed8d0bf86))
* use `claude -p` for OAuth/Max/Pro instead of API direct call ([#4](https://github.com/pszymkowiak/wshm/issues/4)) ([4595b43](https://github.com/pszymkowiak/wshm/commit/4595b43e6f05d3c973959267a1e59ee17d24d996))
