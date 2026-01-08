# Redis egui Client

基于 egui 的 macOS Redis 客户端,采用异步 IO 和 lazy loading 优化性能。

## 技术栈

- **UI**: egui + eframe
- **渲染**: wgpu (macOS 使用 Metal)
- **异步运行时**: tokio
- **Redis**: redis-rs
- **状态管理**: Arc + RwLock

## 功能特性

- ✅ 连接列表管理(从配置文件加载)
- ✅ 新建/保存连接配置
- ✅ 连接 Redis 服务器
- ✅ 多数据库切换
- ✅ Key 列表展示与过滤
- ✅ Value 预览(String, List, Hash, Set, ZSet)
- ✅ 命令输入与执行
- ✅ Lazy loading (大数据集分页加载)
- ✅ UI 线程永不阻塞

## 编译与运行

### 开发模式

```bash
cargo run
```

### 开发 & Git hooks

项目包含一个 pre-commit hook，用于在提交前自动运行格式化。要启用仓库内的 hooks（只需在本地运行一次）：

```bash
sh scripts/setup-git-hooks.sh
# 之后每次 git commit 时会自动运行 `cargo fmt --all`，并将格式化改动暂存
```

如果你更喜欢手动运行格式化：

```bash
cargo fmt --all
```

### macOS App Bundle

To build a macOS `.app` bundle and installer packages use `cargo-bundle`.

1. Install the packaging tool:

```bash
cargo install cargo-bundle
```

2. Build a macOS app (local host arch):

```bash
./scripts/build-macos.sh
```

3. Build for specific architectures (optional):

```bash
# Apple Silicon
cargo bundle --release --target aarch64-apple-darwin

# Intel
cargo bundle --release --target x86_64-apple-darwin
```

4. The generated bundles and installers will be in `./dist/`.

Notes:
- To distribute on the App Store or notarize, you must code sign and notarize the packages using your Apple Developer account.
- Provide a proper `assets/icon.icns` file to set the app icon and update `identifier` in `Cargo.toml`.

Alternative: simple manual bundle

If you prefer a minimal manual bundling step (no extra tools), build the release binary and run:

```bash
cargo build --release
./scripts/make-macos-app.sh
```

This will create `RedisGUI.app` in the workspace root; it does not perform codesigning or notarization.

Generating a default app icon

Note: `cargo-bundle` reads the icon path from `Cargo.toml` (`package.metadata.bundle.icon = "assets/icon.icns"`). The manual bundler (`scripts/make-macos-app.sh`) copies any `assets/icon.icns` into the app as `AppIcon.icns` (this matches the `CFBundleIconFile` used in the generated `Info.plist`).

A simple placeholder vector icon is included at `assets/icon.svg`. To generate `assets/icon.icns` from the SVG (required by `cargo-bundle` and the manual bundler), run:

```bash
./scripts/generate-icon.sh
```

This script uses either `rsvg-convert` or ImageMagick's `convert` to rasterize the SVG, then uses macOS `sips` and `iconutil` to produce `assets/icon.icns`.

### Release 模式(优化启动速度)

```bash
cargo build --release
./target/release/redis-egui-client
```

Release 配置已启用:
- LTO (Link Time Optimization)
- 单代码生成单元
- 符号剥离

## 配置文件

连接配置存储在: `~/.config/rudist/config.toml`

首次启动会自动创建默认配置文件,包含本地 Redis 连接。

配置文件格式:
```toml
[[connections]]
name = "本地 Redis"
url = "redis://127.0.0.1:6379"

[[connections]]
name = "生产环境"
url = "redis://prod.example.com:6379"
```

## 使用说明

1. 在顶部下拉框选择已保存的连接
2. 点击 "+ 新建" 可添加新连接并保存到配置文件
3. 点击"连接"按钮建立连接
4. 使用数据库下拉菜单切换 DB
5. 左侧面板可过滤和选择 Key
6. 右侧面板显示 Key 的值
7. 顶部命令行可执行任意 Redis 命令

## 架构设计

### 异步 IO
- 所有 Redis 操作都通过 tokio 异步执行
- UI 线程通过 `try_read()`/`try_write()` 轮询状态
- 不会阻塞渲染循环

### Lazy Loading
- List/Hash/Set 类型初始只加载长度
- 点击"加载"按钮后才获取具体内容
- 避免大数据集造成 UI 卡顿

### 状态管理
- `Config`: 配置文件管理(TOML)
- `RedisClient`: Redis 连接管理
- `AppState`: 应用状态(Arc + RwLock)
- `RedisApp`: UI 渲染逻辑

## 依赖版本

- egui: 0.30
- eframe: 0.30 (wgpu + glow)
- tokio: 1.0
- redis: 0.27
- toml: 0.8
- dirs: 5.0
