# Redis egui Client

基于 egui 的 macOS Redis 客户端,采用异步 IO 和 lazy loading 优化性能。

## 功能特性

- ✅ redis 客户端
- ✅ 全键盘操作，脱离鼠标手
- ✅ 多 Tab 支持，可同时打开多个连接
- ✅ 连接颜色标识，快速区分不同环境

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

## 配置文件

连接配置存储在: `~/.config/rudist/config.toml`

首次启动会自动创建默认配置文件。

## 使用说明

### 多 Tab 管理
- 顶部 Tab 栏显示所有打开的连接
- 点击 "+ Tab" 创建新的空白 Tab
- 点击 Tab 上的 "✕" 关闭 Tab（自动断开连接）
- 最后一个 Tab 不能关闭
- Tab 显示：彩色圆点 ● = 连接颜色，🟢 = 已连接状态

### 连接操作
1. 在顶部下拉框选择已保存的连接
2. 点击 "+ 新建" 可添加新连接并保存到配置文件
   - 可为每个连接设置颜色，便于区分不同环境（如开发/测试/生产）
   - 连接颜色会在界面顶部和下拉列表中显示为彩色圆点 ●
3. 点击"连接"按钮在当前 Tab 中建立连接
4. 点击 "📑 新建连接" 在新 Tab 中打开所选连接（快速创建多连接）
5. 使用数据库下拉菜单切换 DB
6. 左侧面板可过滤和选择 Key
7. 右侧面板显示 Key 的值
8. 顶部命令行可执行任意 Redis 命令
