# 超级订单管理系统 - 桌面版

基于 Tauri + Vue 3 + Element Plus 的桌面订单管理系统。

## 技术栈

- **Tauri**: 桌面应用框架 (Rust 后端)
- **Vue 3**: 前端框架
- **Element Plus**: UI 组件库
- **Pinia**: 状态管理
- **SQLite**: 本地数据库
- **Vite**: 构建工具

## 项目结构

```
desktop/
├── src/
│   ├── renderer/         # Vue 3 前端
│   │   ├── main.js       # Vue 入口
│   │   ├── App.vue       # 根组件
│   │   ├── router/       # 路由配置
│   │   ├── views/       # 页面组件
│   │   ├── api/         # API 封装
│   │   └── mock/        # 浏览器开发 Mock
│   └── tauri/          # Tauri 后端 (Rust)
│       ├── src/
│       │   ├── main.rs      # 主入口
│       │   ├── db.rs        # 数据库操作
│       │   ├── commands.rs  # Tauri 命令
│       │   └── lib.rs       # 库入口
│       ├── Cargo.toml       # Rust 依赖配置
│       ├── tauri.conf.json  # Tauri 配置
│       └── icons/          # 应用图标
├── src-tauri/           # Tauri 后端目录（实际）
│   └── (同 src/tauri/)
├── public/               # 静态资源
├── package.json         # 项目配置
└── vite.config.js       # Vite 配置
```

## 开发环境运行

### 前置要求

1. **Node.js**: 18+ 
2. **Rust**: 1.70+ 
3. **系统依赖**:
   - macOS: Xcode Command Line Tools
   - Linux: `libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libayatana2.0-dev`
   - Windows: WebView2 (Windows 11+) 或 Microsoft Visual C++ Redistributable

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 安装依赖

```bash
cd desktop
npm install
```

### Tauri 桌面应用运行

**开发模式（推荐）**：
```bash
npm run tauri
```

这将同时启动：
- Vite 开发服务器（端口 5173）
- Tauri 桌面窗口
- 自动热重载

**仅前端开发（浏览器模式）**：
```bash
npm run dev
```

适用于前端界面调试，使用 mock 数据。

### Tauri 常用命令

```bash
# 开发模式
npm run tauri

# 构建 Tauri 应用
npm run tauri:build

# 开发模式（详细日志）
npm run tauri dev

# 构建（仅编译 Rust，不打包）
npm run tauri build
```

### Rust 相关命令

```bash
# 进入 Tauri 目录
cd src-tauri

# 检查代码（快速编译检查）
cargo check

# 清理构建缓存
cargo clean

# 运行测试
cargo test

# 更新依赖
cargo update
```

### 其他开发命令

```bash
# 前端开发服务器
npm run dev

# 构建前端
npm run build

# 预览构建后的前端
npm run preview

# 代码检查
npm run lint

# 类型检查
npm run typecheck
```

## 数据库准备

确保数据库文件存在：

```bash
# 从项目根目录运行
.venv/bin/python scripts/init_db.py
```

数据库位置：`~/.super-order/super_order.db`

## 导入 SKU 数据

```bash
# 从项目根目录运行
.venv/bin/python scripts/import_sku.py doc/SKU.xlsx
```

## 功能模块

- **首页**: 数据统计和快速导航
- **SKU 管理**: 产品的增删改查、搜索、导入导出
- **订单管理**: 订单的创建、查看、编辑
- **客户管理**: 客户信息管理

## Tauri API 使用

前端通过 Tauri invoke API 调用后端功能：

```javascript
import { invoke } from '@tauri-apps/api/core';

// SKU 相关 API
const skus = await invoke('sku_list');
const sku = await invoke('sku_get', { id: 1 });
await invoke('sku_create', { data: { name: '产品A', ... } });
await invoke('sku_update', { id: 1, data: { name: '产品A更新', ... } });
await invoke('sku_delete', { id: 1 });
const results = await invoke('sku_search', { keyword: '关键词' });

// 订单相关 API
const orders = await invoke('order_list');
const order = await invoke('order_get', { id: 1 });
await invoke('order_create', { data: { ... } });
await invoke('order_update', { id: 1, data: { ... } });
await invoke('order_delete', { id: 1 });

// 客户相关 API
const customers = await invoke('customer_list');

// 分类相关 API
const categories = await invoke('category_list');
```

### API 封装

项目中已封装统一的 API 调用接口，位于 `src/renderer/api/` 目录：

```javascript
import tauriAPI from '@/api/tauri-api';

// 使用封装后的 API
const skus = await tauriAPI.sku.list();
const sku = await tauriAPI.sku.get(1);
await tauriAPI.sku.create({ name: '产品A', ... });
```

## Tauri 架构说明

Tauri 采用前后端分离架构：

- **前端 (src/renderer/)**: Vue 3 + Vite，运行在 WebView 中
- **后端 (src-tauri/src/)**: Rust，负责数据库操作和系统调用
- **通信**: 通过 Tauri IPC 机制，前端使用 `invoke()` 调用后端命令

### 前后端通信流程

```
Vue 前端
    ↓ invoke('sku_list')
Tauri IPC
    ↓
Rust commands.rs (sku_list)
    ↓
Rust db.rs (get_all_skus)
    ↓
SQLite 数据库
    ↓
返回数据
```

## Tauri 优势

相比传统桌面框架的优势：

1. **更小的包体**: 应用通常只有 5-10MB
2. **更好的性能**: Rust 后端提供高性能数据处理
3. **更低的内存占用**: 通常比 Electron 低 50-70%
4. **更好的安全性**: 默认启用安全特性
5. **更简单的配置**: 与 Vite 完美集成
6. **更好的开发体验**: 快速热重载和类型安全

## 常见问题

### Q: Rust 编译失败？
A: 确保 Rust 工具链正确安装：
```bash
rustup update
rustup install stable
```

### Q: Tauri 窗口无法打开？
A: 确保 Vite 开发服务器已启动（端口 5173），检查是否有端口冲突

### Q: 数据库连接失败？
A: 检查数据库文件路径是否正确，确保数据库已初始化

### Q: 前端 API 调用报错？
A: 确保在 Tauri 环境下运行，浏览器模式下需要使用 mock 数据

### Q: 端口 5173 被占用？
A: 清理端口后重新启动：
```bash
lsof -ti:5173 | xargs kill -9
npm run tauri
```

## 打包发布

使用 Tauri 打包应用：

```bash
npm run tauri:build
```

打包产物位于 `src-tauri/target/release/bundle/` 目录。

## 技术支持

- [Tauri 文档](https://tauri.app/)
- [Vue 3 文档](https://vuejs.org/)
- [Element Plus 文档](https://element-plus.org/)
