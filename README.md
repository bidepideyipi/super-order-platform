# Super Order 订单管理系统

订单管理系统，支持 SKU 管理、订单管理、Excel 数据导入、PDF 对账单生成等功能。

## 技术栈

- 后端：FastAPI + Python 3.11
- 数据库：MySQL 8.0
- 对象存储：MinIO
- 前端：Vue 3 + Element Plus

## 快速开始

### 前置要求

- Docker & Docker Compose
- Python 3.11+（本地开发）

### 使用 Docker 启动

1. 复制环境变量配置文件：

```bash
cp docker/.env.example .env
```

2. 启动服务：

```bash
docker-compose up -d
```

3. 检查服务状态：

```bash
docker-compose ps
```

4. 访问服务：

- 后端 API：http://localhost:8000
- API 文档：http://localhost:8000/docs
- MySQL：localhost:3306
- MinIO Console：http://localhost:9001

### 数据导入

启动 Docker 后，执行数据导入脚本：

```bash
cd scripts
python import_orders.py ../doc/方舟产品订单表（原始记录不外发）251117.xlsx
```

或导入第二个文件：

```bash
python import_sku.py ../doc/方舟产品订单表（原始记录不外发）260112.xlsx
```

### 健康检查

```bash
curl http://localhost:8000/health
```

## 项目结构

```
super-order/
├── backend/              # 后端项目
│   ├── main.py          # FastAPI 入口
│   ├── app/             # 应用模块
│   │   ├── api/        # API 路由
│   │   ├── models/     # 数据模型
│   │   ├── schemas/    # Pydantic 模型
│   │   ├── services/   # 业务逻辑
│   │   └── core/       # 核心配置
│   ├── requirements.txt # Python 依赖
│   └── Dockerfile
├── scripts/            # 数据导入脚本
│   ├── import_orders.py
│   ├── import_sku.py
│   ├── minio_client.py
│   └── config.py
├── docker/             # Docker 配置
│   ├── docker-compose.yml
│   ├── init.sql
│   └── .env.example
├── doc/               # Excel 文件
├── PRD.md            # 产品需求文档
└── 开发计划.md        # 开发计划文档
```

## API 文档

启动服务后，访问 http://localhost:8000/docs 查看 Swagger API 文档。

## 常用命令

### Docker

```bash
# 启动服务
docker-compose up -d

# 停止服务
docker-compose down

# 查看日志
docker-compose logs -f backend

# 重启服务
docker-compose restart backend
```

### 本地开发

```bash
# 安装依赖
cd backend
pip install -r requirements.txt

# 启动后端服务
python main.py
```

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
