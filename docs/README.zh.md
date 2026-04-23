<p align="center">
  <img src="../assets/wizard.png" alt="wshm" width="600"/>
</p>

<p align="center">
  <a href="../README.md">English</a> •
  <a href="README.fr.md">Français</a> •
  <a href="README.es.md">Español</a> •
  <a href="README.de.md">Deutsch</a> •
  <a href="README.ja.md">日本語</a> •
  中文 •
  <a href="README.ko.md">한국어</a> •
  <a href="README.pt.md">Português</a>
</p>

---

# wshm — GitHub仓库维护AI代理

**Your repo's wish is my command.**

wshm是一个CLI工具 + GitHub Action，作为自主仓库维护代理运行。

## 工作原理

单一二进制文件。您的API密钥。您的数据。零基础设施。

```
GitHub API → 增量同步 → 本地SQLite → AI → 操作（标签、评论、PR、合并）
```

三种模式：CLI、GitHub Action或带有Webhooks/轮询的持久守护进程。

## 功能

- **Issue自动分类** — AI驱动的分类、标签和优先级设定
- **PR分析** — 摘要、风险评估、审查清单
- **自动修复** — 为简单Bug自动生成Draft PR
- **合并队列** — PR评分和排名，超过阈值自动合并
- **冲突解决** — 检测和自动解决（从不force-push）
- **行内审查** — AI驱动的逐行代码审查
- **自动分配** — 加权随机分配维护者
- **标签黑名单** — 防止应用特定标签
- **定期重新分类** — 按计划重新评估过时的分类结果
- **通知** — 每日路线图发送到Discord、Slack、Teams或Webhook：按优先级和时间排序的前10个Issue和PR
- **仪表板和报告** — HTML仪表板和Markdown/PDF报告
- **完全可定制** — 每条评论和品牌推广的模板

单一二进制文件。您的API密钥。您的数据。零基础设施。

## 安全性

- **默认干运行** — 需要 `--apply` 才能执行操作
- **置信度阈值** — 低于0.85时不会自主执行操作
- **不使用force-push** — 通过新提交解决冲突
- **幂等性** — 重复运行结果相同
- **令牌安全** — 始终从环境变量读取

## 早期访问

> **wshm目前处于私有测试阶段。**
>
> **[contact@rtk-ai.app](mailto:contact@rtk-ai.app)** — 早期访问开放中

---

<p align="center">
  <sub>使用Rust构建。零基础设施。单一二进制。</sub><br>
  <sub>&copy; 2026 <a href="https://rtk-ai.app">rtk-ai</a></sub>
</p>
