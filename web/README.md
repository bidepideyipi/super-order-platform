# Super Order Web 版本

这是 Super Order 订单管理系统的 Web 版本，从 Desktop 版本迁移而来。

## 技术栈

- **后端**: Rust (Actix-web)
- **数据库**: SQLite
- **文件存储**: 阿里云 OSS
- **前端**: Vue 3 (需要单独部署)

## 快速开始

### 1. 配置环境变量

复制 `.env.example` 为 `.env` 并填写配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件，填写阿里云 OSS 凭证：

```env
ALIBABA_CLOUD_ACCESS_KEY_ID=your_access_key_id
ALIBABA_CLOUD_ACCESS_KEY_SECRET=your_access_key_secret
```

### 2. 准备数据库

从 Desktop 版本复制数据库：

```bash
mkdir -p data
cp ../desktop/data/super_order.db data/
```

### 3. 运行服务器

```bash
cargo run
```

服务器将在 `http://127.0.0.1:8080` 启动。

## API 文档

### SKU 管理

- `GET /api/sku/list` - 获取所有 SKU
- `GET /api/sku/get/{id}` - 获取单个 SKU
- `POST /api/sku/create` - 创建 SKU
- `PUT /api/sku/update/{id}` - 更新 SKU
- `DELETE /api/sku/delete/{id}` - 删除 SKU
- `GET /api/sku/search?keyword={keyword}` - 搜索 SKU

### 采购管理

- `GET /api/purchase/processing-orders` - 获取处理中的订单
- `GET /api/purchase/order-items/{order_id}` - 获取订单明细
- `GET /api/purchase/search-sku?keyword={keyword}` - 搜索 SKU
- `POST /api/purchase/create-order-item` - 创建订单明细
- `PUT /api/purchase/update-order-item/{id}` - 更新订单明细
- `DELETE /api/purchase/delete-order-item/{id}` - 删除订单明细

### 其他 API

更多 API 请参考 `src/api/mod.rs` 文件。

## 迁移说明

本项目从 Desktop 版本迁移而来，主要改动：

1. **后端框架**: Tauri → Actix-web
2. **通信方式**: IPC → HTTP API
3. **文件存储**: 本地文件系统 → 阿里云 OSS
4. **数据库**: SQLite (保持不变)

## 开发计划

- [ ] 完善所有 API 实现
- [ ] 添加用户认证
- [ ] 添加前端项目
- [ ] 添加单元测试
- [ ] 添加 API 文档

## 许可证

MIT
