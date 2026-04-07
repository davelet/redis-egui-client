# Rudist - AI 驱动的 Redis GUI 客户端

[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml)
[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml)
[![Version](https://img.shields.io/github/v/release/davelet/redis-egui-client?label=version&color=blue)](https://github.com/davelet/redis-egui-client/releases)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg](LICENSE)

[English](README.md) | [中文](README.zh-CN.md)

一个内置 AI 助手的现代化 Redis GUI 客户端。用自然语言管理 Redis，键盘优先设计，流畅的用户体验。

---

## ✨ AI 助手

**用自然语言和 Redis 对话。** 无需记忆命令。

```
你: "显示所有 user:* 匹配的键"
AI: → SCAN 0 MATCH user:* COUNT 100

你: "session:abc123 的 TTL 是多少？"
AI: → TTL session:abc123

你: "删除所有过期的测试键"
AI: → 分析并建议清理命令
```

### 两种 AI 模式

| 模式 | 说明 | 适用场景 |
|------|------|----------|
| **Chat 模式** | 自然语言 → Redis 命令（需确认后执行） | 安全探索、学习 Redis |
| **Agent 模式** | 完整工具访问，直接执行 18+ 操作 | 高级用户、复杂工作流 |

### AI 能力（18+ 工具）

- **查询**: 过滤键、获取键信息、检查存在性、数据库统计
- **写入**: 设置值、设置 TTL、重命名/删除键
- **数据结构**: Hash (hset/hdel)、List (lset/rpush)、Set (sadd/srem)、ZSet (zadd/zrem)
- **数据库**: 执行命令、切换数据库

### 灵活的模型支持

支持 OpenAI、Claude、Ollama（本地）、OpenRouter 及任何 OpenAI 兼容 API。API 密钥安全存储在系统钥匙串中（绝不写入配置文件）。

👉 **快速开始**: 按 `Cmd/Ctrl + E` 打开 AI 面板，输入你的问题。

---

## ⌨️ 键盘优先设计

**最少鼠标，最快速度。** 每个操作都有快捷键。

### 核心快捷键

| 快捷键 | 功能 |
|--------|------|
| `Cmd/Ctrl + E` | **切换 AI 面板** ← 从这里开始 |
| `Cmd/Ctrl + T` | 新建标签页 |
| `Cmd/Ctrl + W` | 关闭标签页 |
| `Cmd/Ctrl + R` | 刷新键 |
| `Cmd/Ctrl + F` | 聚焦过滤框 |
| `1-9` | 快速连接（已保存的连接） |
| `Cmd/Ctrl + 1-9` | 切换标签页 |
| `↑/↓` | 命令历史导航 |
| `Esc` | 关闭面板/对话框 |

所有快捷键可在设置中自定义 (`Cmd/Ctrl + ,`)。

---

## 🎯 流畅的用户体验

- **Toast 提示** - 连接错误、警告通知，悬停可暂停计时
- **多标签页工作流** - 独立会话，颜色区分环境
- **懒加载** - 大数据集流畅加载，界面永不卡顿
- **智能 JSON** - 查看时自动格式化，保存时智能压缩
- **进度指示** - 所有异步操作都有视觉反馈

---

## 核心功能

### 连接管理
- **快速连接** - 一行 URL: `redis://pass@host:port/db` 或 `rediss://` TLS
- **多标签页** - 同时管理多个 Redis 实例
- **颜色标识** - 开发/测试/生产环境一目了然
- **配置持久化** - 自动保存，启动即恢复

### 数据操作
- **全类型支持** - String、List、Hash、Set、ZSet
- **键树形视图** - 按冒号分隔组织 (`user:123:profile`)
- **实时过滤** - `*` 通配符模式匹配
- **TTL 管理** - 查看和修改过期时间
- **增删改查** - 创建、编辑、删除键，均有确认提示

### 性能优化
- **异步 I/O** - 基于 Tokio，永不阻塞 UI
- **增量加载** - 百万键分批加载
- **内存高效** - 大值按需加载

### 跨平台
- **macOS** - 原生 .app，支持 Apple Silicon (M1/M2/M3)
- **Windows** - MSI 安装程序，PATH 集成

---

## 快速开始

### 安装

从 [GitHub Releases](https://github.com/davelet/redis-egui-client/releases) 下载：

- **macOS**: `.dmg` → 拖入应用程序文件夹
- **Windows**: `.msi` → 运行安装程序

### 从源码构建

```bash
git clone https://github.com/davelet/redis-egui-client.git
cd redis-egui-client
cargo run
```

### 第一次使用

1. 按 `Cmd/Ctrl + N` 创建连接
2. 输入主机/端口（或使用快速连接 URL）
3. 按 `Cmd/Ctrl + E` 打开 AI 面板
4. 输入: "显示所有键" ← 开始探索！

---

## 文档

- [AI 配置](docs/ai/configuration.md) - 设置模型和 API 密钥
- [AI 工具列表](docs/ai/tools.md) - 完整的 18+ 工具参考
- [键盘快捷键](docs/usage/shortcuts.md) - 完整快捷键参考
- [使用指南](docs/usage/index.md) - 完整文档

---

## 贡献

```bash
sh scripts/setup-git-hooks.sh
cargo fmt --all
cargo test
```

## 许可证

Apache License 2.0

## 作者

Sheldon.Wei <sheldon.sh.hb@gmail.com>

---

⭐ 如果这个项目对你有帮助，请给个 Star！