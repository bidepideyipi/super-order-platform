# 超级订单管理系统 - Tauri 桌面版

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
│   │   ├── tauri-api.js # Tauri API 封装
│   │   └── electron-mock.js # 浏览器开发 Mock
│   └── tauri/          # Tauri 后端 (Rust)
│       ├── src/
│       │   ├── main.rs      # 主入口
│       │   ├── db.rs        # 数据库操作
│       │   └── commands.rs   # Tauri 命令
│       └── tauri.conf.json # Tauri 配置
├── public/               # 静态资源
└── package.json         # 项目配置
```

## 开发环境运行

### 前置要求

1. **Node.js**: 18+ 
2. **Rust**: 1.70+ 
3. **系统依赖**:
   - macOS: Xcode Command Line Tools
   - Linux: `libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libayatana2.0-dev`
   - Windows: WebView2 (Windows 11+) 或 Microsoft Visual C++ Redistributable

### 安装依赖

```bash
cd desktop
npm install
```

### 启动开发服务器

```bash
npm run dev
```

这将启动 Vite 开发服务器（端口 5173）。

### 启动 Tauri 应用

```bash
npm run tauri
```

这将同时启动 Vite 开发服务器和 Tauri 窗口。

## 构建应用

### 构建前端

```bash
npm run build
```

### 构建 Tauri 应用

```bash
npm run tauri:build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录。

## 数据库准备

确保数据库文件存在：

```bash
# 从项目根目录运行
.venv/bin/python scripts/init_db.py
```

数据库位置：`~/.super-order/super_order.db`

## 功能模块

- **首页**: 数据统计和快速导航
- **SKU 管理**: 产品的增删改查、搜索、导入导出
- **订单管理**: 订单的创建、查看、编辑
- **客户管理**: 客户信息管理

## 开发说明

### API 调用

前端通过 `window.electronAPI` 调用后端功能：

```javascript
// 列表
const skus = await window.electronAPI.sku.list();

// 详情
const sku = await window.electronAPI.sku.get(1);

// 创建
await window.electronAPI.sku.create({ name: '产品A', ... });

// 更新
await window.electronAPI.sku.update(1, { name: '产品A更新', ... });

// 删除
await window.electronAPI.sku.delete(1);

// 搜索
const results = await window.electronAPI.sku.search('关键词');
```

### 环境适配

代码会自动适配不同环境：

- **Tauri 环境**: 使用真实的 Tauri API
- **浏览器环境**: 使用 mock 数据进行开发
- **构建环境**: 使用 Tauri 打包的 API

## Tauri 优势

相比 Electron 的优势：

1. **更小的包体**: Tauri 应用通常只有 5-10MB，而 Electron 应用通常 100MB+
2. **更好的性能**: Rust 后端比 Node.js 更快更高效
3. **更低的内存占用**: 通常比 Electron 低 50-70%
4. **更好的安全性**: 默认启用安全特性，不需要复杂配置
5. **更简单的配置**: 与 Vite 完美集成，不需要复杂的模块加载配置
6. **更好的开发体验**: 热重载更快，配置更简单

## 常见问题

### Q: Rust 编译失败？
A: 确保 Rust 工具链正确安装：
```bash
rustup update
rustup install stable
```

### Q: Tauri 窗口无法打开？
A: 确保 Vite 开发服务器已启动（端口 5173）

### Q: 数据库连接失败？
A: 检查数据库文件路径是否正确，确保数据库已初始化

## 技术支持

- [Tauri 文档](https://tauri.app/)
- [Vue 3 文档](https://vuejs.org/)
- [Element Plus 文档](https://element-plus.org/)
