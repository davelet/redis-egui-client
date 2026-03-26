# Rudist - Redis GUI Client

[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/release.yml)
[![CI](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml/badge.svg)](https://github.com/davelet/redis-egui-client/actions/workflows/docs.yml)
[![Version](https://img.shields.io/github/v/release/davelet/redis-egui-client?label=version&color=blue)](https://github.com/davelet/redis-egui-client/releases)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)

[English](README.md) | [中文](README.zh-CN.md)

一个现代化、高性能的 Redis 图形化客户端，基于 Rust 和 egui 构建，支持 macOS 和 Windows。

## 核心特性

### 多连接管理
- **多标签页支持** - 同时打开多个 Redis 连接，在标签页间快速切换
- **连接颜色标识** - 为不同环境（开发/测试/生产）分配不同颜色，一目了然
- **配置持久化** - 连接设置自动保存，下次启动即可使用
- **一键连接** - 快速恢复上次打开的所有连接

### 数据浏览与编辑
- **全类型支持** - 完整支持 String、List、Hash、Set、ZSet 类型
- **智能 JSON 处理** - 编辑时自动格式化 JSON 数据，保存时智能压缩
- **懒加载设计** - 大数据量下流畅浏览，按需加载键和值
- **实时过滤** - 键列表支持实时搜索过滤
- **按冒号分组** - 按冒号分隔符将键组织成树形结构（如 `user:123:profile`）

### 高效操作
- **键盘优先** - 快捷键支持，减少鼠标操作，提升效率
- **命令行模式** - 直接执行任意 Redis 命令
- **TTL 管理** - 便捷查看和修改键过期时间
- **数据操作** - 支持复制、删除、重命名等常用操作

### 性能优化
- **异步 I/O** - 基于 Tokio 的异步架构，操作永不阻塞
- **增量加载** - 大量键时分批加载，界面保持响应
- **内存优化** - 大值按需加载，防止内存暴涨

### 跨平台支持
- **macOS** - 原生 App Bundle (.app)，支持 Apple Silicon (M1/M2/M3)
  - ⚠️ **Intel Mac 不提供预构建版本**，Intel 用户需要[自行编译](#从源码构建)
- **Windows** - MSI 安装程序，支持应用程序图标和系统 PATH 集成

## 快速开始

### 安装

从 [GitHub Releases](https://github.com/davelet/redis-egui-client/releases) 下载最新版本。

#### macOS
下载 `.dmg` 文件，将应用拖入应用程序文件夹。

#### Windows
下载并运行 `.msi` 安装程序。

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/davelet/redis-egui-client.git
cd redis-egui-client

# 开发模式运行
cargo run

# 发布构建
cargo build --release
```

### macOS App Bundle 构建

```bash
# 安装 cargo-bundle
cargo install cargo-bundle

# 构建 macOS 应用
./scripts/build-macos.sh
```

### Windows 构建（包含图标支持）

Windows 构建会自动包含应用程序图标生成。确保你已安装 Windows 目标的 Rust 工具链：

```bash
# 为 Windows 构建（如果已设置交叉编译，可以从任何平台构建）
cargo build --release --target x86_64-pc-windows-msvc
```

## 使用指南

### 创建连接
1. 点击顶部下拉框选择 "+ 新建"
2. 填写连接信息（名称、主机、端口、密码等）
3. 选择连接颜色以区分不同环境
4. 点击保存，连接配置将持久化到本地

### 管理数据
- **浏览键** - 左侧面板显示所有键，支持 `*` 通配符过滤
- **查看值** - 点击键在右侧查看详细内容
- **编辑数据** - 双击值或点击编辑按钮，JSON 数据自动格式化
- **执行命令** - 在底部命令行输入任意 Redis 命令，按回车执行

### 键盘快捷键
| 快捷键 | 功能 |
|--------|------|
| `Cmd/Ctrl + T` | 新建标签页 |
| `Cmd/Ctrl + W` | 关闭当前标签页 |
| `Cmd/Ctrl + R` | 刷新当前键 |
| `Cmd/Ctrl + F` | 聚焦到键过滤框 |
| `Cmd/Ctrl + 1-9` | 切换到标签页 1-9 |
| `Cmd/Ctrl + 0` | 切换到最右边的标签页 |
| `Cmd/Ctrl + E` | 切换命令行面板 |
| `上/下箭头` | 在命令行中导航历史记录 |
| `Cmd/Ctrl + Shift + D` | 删除重复的标签页和无连接的标签页 |

## 配置说明

配置文件位置：
- **macOS**: `~/.config/rudist/config.toml`
- **Windows**: `%APPDATA%\rudist\config.toml`

示例配置：
```toml
[[connections]]
name = "本地开发"
host = "localhost"
port = 6379
password = ""
database = 0
color = "#4CAF50"

[[connections]]
name = "生产环境"
host = "redis.example.com"
port = 6380
password = "secret"
database = 0
color = "#F44336"
```

## 贡献指南

欢迎提交 Issue 和 PR！

```bash
# 设置 Git hooks
sh scripts/setup-git-hooks.sh

# 代码格式化
cargo fmt --all

# 运行测试
cargo test
```

## 许可证

Apache License 2.0 - 详见 [LICENSE](LICENSE) 文件

## 作者

Sheldon.Wei <sheldon.sh.hb@gmail.com>

---

⭐ 如果这个项目对你有帮助，请给个 Star 支持一下！
