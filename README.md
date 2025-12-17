# Redis egui Client

基于 egui 的 macOS Redis 客户端,采用异步 IO 和 lazy loading 优化性能。

## 技术栈

- **UI**: egui + eframe
- **渲染**: wgpu (macOS 使用 Metal)
- **异步运行时**: tokio
- **Redis**: redis-rs
- **状态管理**: Arc + RwLock

## 功能特性

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

### Release 模式(优化启动速度)

```bash
cargo build --release
./target/release/redis-egui-client
```

Release 配置已启用:
- LTO (Link Time Optimization)
- 单代码生成单元
- 符号剥离

## 使用说明

1. 在顶部输入 Redis 连接地址(默认: `redis://127.0.0.1:6379`)
2. 点击"连接"按钮
3. 使用数据库下拉菜单切换 DB
4. 左侧面板可过滤和选择 Key
5. 右侧面板显示 Key 的值
6. 顶部命令行可执行任意 Redis 命令

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
- `RedisClient`: Redis 连接管理
- `AppState`: 应用状态(Arc + RwLock)
- `RedisApp`: UI 渲染逻辑

## 依赖版本

- egui: 0.30
- eframe: 0.30 (wgpu + glow)
- tokio: 1.0
- redis: 0.27
