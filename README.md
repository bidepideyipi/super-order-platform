# Super Order 订单管理系统

订单管理系统，支持 SKU 管理、订单管理、Excel 数据导入、PDF 对账单生成等功能。

## 技术栈

- 桌面端：Tauri + Rust
- 前端：Vue 3 + Element Plus
- 数据库：SQLite

## 快速开始

### 前置要求

- Node.js 18+
- Rust 1.70+
- 系统依赖：
  - macOS: Xcode Command Line Tools
  - Linux: `libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libayatana2.0-dev`
  - Windows: WebView2 (Windows 11+) 或 Microsoft Visual C++ Redistributable

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 启动桌面应用

1. 进入桌面应用目录：

```bash
cd desktop
```

2. 安装依赖：

```bash
npm install
```

3. 初始化数据库：

```bash
python ../scripts/init_db.py
```

4. 启动桌面应用：

确保当前工作目录在 `desktop/` 下，然后执行：

```bash
cd /Users/anthony/Documents/github/super-order/desktop
npm run tauri dev
```

**注意**：必须使用 `npx tauri dev` 而不是 `npm run tauri`，因为 `tauri` 命令可能不在系统 PATH 中。

这将启动：
- Vite 开发服务器（端口 5173）
- Tauri 桌面窗口
- 自动热重载

**其他启动选项**：

- 仅前端开发（浏览器模式，使用 mock 数据）：
  ```bash
  npm run dev
  ```

- 构建 Tauri 应用：
  ```bash
  npm run tauri:build
  ```

详细的桌面应用开发文档、运行命令、API 使用方法请参考 [desktop/README.md](./desktop/README.md)

### 数据导入

数据库初始化后，可以导入 Excel 数据：

```bash
cd scripts
python import_sku.py
```

## 项目结构

```
super-order/
├── desktop/            # 桌面应用
│   ├── src/
│   │   ├── renderer/  # Vue 3 前端
│   │   └── tauri/     # Tauri 后端 (Rust)
│   ├── src-tauri/      # Tauri 后端目录
│   ├── package.json
│   └── README.md       # 桌面应用文档
├── scripts/            # 数据导入脚本
│   ├── import_orders.py
│   ├── import_sku.py
│   ├── init_db.py
│   └── config.py
├── doc/               # Excel 文件
├── PRD.md            # 产品需求文档
└── 开发计划.md        # 开发计划文档
```

## API 文档

Tauri API 通过 IPC 机制在 Rust 后端和 Vue 前端之间通信。详细的 API 使用方法请参考 [desktop/README.md](./desktop/README.md)。

## 桌面应用开发

详细的桌面应用开发文档、运行命令、API 使用方法请参考 [desktop/README.md](./desktop/README.md)。

## 数据导入说明

### 整箱数量字段处理

- Excel 中没有"整箱数量"字段
- 导入时特殊处理：
  - 当 `unit != "箱"` 时，整箱数量 = quantity
  - 当 `unit == "箱"` 时，整箱数量留空，由用户手工补充

### SKU 编号生成规则

- 格式：分类ID（2位）+ 流水号（4位）
- 示例：010001、010002、020001

### 订单编号生成规则

- 格式：客户ID（3位）+ yyyyMMdd
- 示例：FZC20250221、SE820250221

## 开发计划

详见 [开发计划.md](./开发计划.md)

## 产品需求

详见 [PRD.md](./PRD.md)
