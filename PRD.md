# 订单管理系统 - 产品需求文档 (PRD)

## 文档信息

| 项目 | 内容 |
|------|------|
| 文档版本 | v1.5 |
| 创建日期 | 2025-02-21 |
| 更新日期 | 2025-02-21 |
| 项目名称 | Super Order 订单管理系统 |
| 文档类型 | 产品需求文档 (PRD) |

---

## 1. 项目背景与目标

### 1.1 项目背景
客户需要一个完整的订单管理系统，用于管理SKU（库存单位）、订单和SKU价格。系统需要支持以下核心业务场景：
- 从历史Excel订单数据中导入并自动生成SKU和订单记录
- 日常订单录入，支持SKU快速模糊匹配
- 生成PDF格式的对账单
- SKU图片存储和管理

### 1.2 项目目标
- **主要目标**: 构建一个高效、易用的订单管理系统，提高订单处理效率
- **核心价值**: 
  - 支持键盘快捷操作，提升录入效率
  - 智能去重和SKU自动生成
  - 灵活的Excel数据导入
  - 专业的PDF对账单生成

### 1.3 目标用户
- **主要用户**: 订单管理员、销售助理
- **用户特征**: 
  - 日常处理大量订单
  - 习惯使用键盘快捷操作
  - 需要快速查询和录入SKU信息

### 1.4 本期范围说明
**不在本期实现的功能**:
- 客户管理功能（新增、编辑、删除客户）
- SKU分类管理功能（新增、编辑、删除分类）

**说明**: 本期使用固定的客户数据和SKU分类数据，客户管理和分类管理功能将在后续版本中实现。

---

## 2. 功能需求

### 2.1 SKU管理模块

#### 2.1.1 SKU基础信息管理
**功能描述**: 管理SKU的基础信息

**字段要求**:
- SKU编号（系统生成，唯一）
- 产品名称（必填）
- 产品描述（选填）
- 规格型号（选填）
- 单位（个、箱、件等）
- 分类（多级分类）
- 箱规（如：48包、136g*40包）
- 成本单价（元）
- 销售单价（元）
- 创建时间、更新时间

**功能点**:
- [ ] SKU新增、编辑、删除
- [ ] SKU列表查询（支持多条件筛选）
- [ ] SKU详情查看
- [ ] SKU图片上传（支持多张，存储在MinIO）
- [ ] SKU图片预览和管理

**说明**: 成本单价和销售单价直接在SKU表中维护，不需要单独的价格管理模块

#### 2.1.2 SKU分类管理
**功能描述**: SKU分类管理（本期不实现管理功能，仅使用固定数据）

**说明**: 本期不实现分类的新增、编辑、删除功能，仅使用预设的固定分类数据

**固定分类数据**:
| 分类ID | 分类名称 |
|-------|---------|
| 01 | 食品 |
| 02 | 纸品 |
| 03 | 个护 |
| 04 | 百货 |

### 2.2 订单管理模块

#### 2.2.1 订单录入
**功能描述**: 快速录入订单信息

**字段要求**:
- 订单编号（系统生成，唯一）
- 客户ID（必填，从固定客户列表中选择）
- 客户名称（根据客户ID自动带出）
- 订单日期（默认当天）
- 订单状态（待处理、处理中、已完成、已取消）
- 总收入
- 总支出
- 余额
- 订单备注

**固定客户数据**:
| 客户ID | 客户名称 |
|-------|---------|
| FZC | 方舟 |
| SE8 | SE8加拿大 |

**订单明细字段**:
- SKU编号
- 产品名称（自动带出）
- 成本单价（自动带出，可修改）
- 销售单价（自动带出，可修改）
- 数量
- 整箱数量（Excel导入时特殊处理：当unit不等于"箱"时，整箱数量=quantity；否则留空，由用户手工补充）
- 单位
- 成本金额（自动计算）
- 销售金额（自动计算）
- 利润（自动计算）
- 箱规
- 采购单据号
- 备注

**功能点**:
- [ ] 订单新增、编辑、删除
- [ ] SKU快速模糊匹配（输入SKU编号或产品名称）
- [ ] 键盘上下键选择SKU候选，回车确认
- [ ] 支持Tab键在字段间快速切换
- [ ] 订单明细动态添加/删除
- [ ] 自动计算订单总金额
- [ ] 订单草稿保存

#### 2.3.2 订单查询
**功能描述**: 多维度查询订单信息

**功能点**:
- [ ] 按订单编号查询
- [ ] 按客户名称查询
- [ ] 按日期范围查询
- [ ] 按订单状态筛选
- [ ] 按SKU筛选（查询包含某SKU的订单）
- [ ] 订单列表展示（支持分页）

#### 2.3.3 订单详情
**功能描述**: 查看订单完整信息

**功能点**:
- [ ] 订单基础信息展示
- [ ] 订单明细列表
- [ ] 订单金额汇总
- [ ] 订单状态变更
- [ ] 订单操作日志

### 2.3 数据导入模块

#### 2.3.1 Excel订单导入
**功能描述**: 从Excel文件导入历史订单数据

**Excel模板要求**:
```
出货日期 | 产品图片 | 产品名称 | 成本价/箱 | 报价/元 | 数量 | 单位 | 成本金额/元 | 出货金额/元 | 状态 | 每箱 | 单位.1 | 采购单据 | 利润 | 支出 | 收入 | 余额 | 备注
```

**功能点**:
- [ ] Excel文件上传
- [ ] 模板下载
- [ ] 数据预览
- [ ] 数据校验（必填字段、格式校验）
- [ ] 产品名称去重和SKU自动生成
- [ ] 重复订单检测（按订单编号）
- [ ] 整箱数量特殊处理（当unit不等于"箱"时，整箱数量=quantity；否则留空，由用户手工补充）
- [ ] 导入结果反馈（成功/失败记录数）
- [ ] 导入失败原因展示

#### 2.3.2 SKU去重和自动生成
**功能描述**: 根据产品名称自动生成SKU

**功能点**:
- [ ] 产品名称哈希生成SKU编号
- [ ] 检测重复产品名称
- [ ] SKU编号冲突检测
- [ ] SKU生成规则配置（前缀、后缀）

### 2.4 报表与对账单模块

#### 2.4.1 PDF对账单生成
**功能描述**: 生成客户对账单

**对账单内容**:
- 客户基本信息
- 对账日期范围
- 订单汇总表（订单编号、日期、总成本金额、总销售金额、总利润、总收入、总支出、余额）
- 订单明细表（SKU、产品名称、成本单价、销售单价、数量、整箱数量、单位、成本金额、销售金额、利润、箱规、采购单据号）
- 合计金额（总成本、总销售、总利润、总收入、总支出、余额）
- 公司信息（可配置）

**功能点**:
- [ ] 选择客户
- [ ] 选择日期范围
- [ ] 对账单预览
- [ ] PDF下载
- [ ] 对账单打印
- [ ] 对账单模板自定义

#### 2.4.2 数据统计报表
**功能描述**: 提供数据统计功能

**功能点**:
- [ ] 订单量统计（按日期、客户、SKU）
- [ ] 销售额统计（销售金额、成本金额、利润）
- [ ] 收支统计（总收入、总支出、余额）
- [ ] SKU销售排行
- [ ] 客户销售排行
- [ ] 报表导出（Excel、PDF）

---

## 3. 技术架构

### 3.1 整体架构

```
┌─────────────────────────────────────────┐
│           前端 (Vue 3 + Element Plus)    │
├─────────────────────────────────────────┤
│           API网关 (可选)                  │
├─────────────────────────────────────────┤
│       后端 (FastAPI + Pydantic)          │
├─────────────────────────────────────────┤
│    数据库 (MySQL 8.0)   |   存储 (MinIO) │
└─────────────────────────────────────────┘
```

### 3.2 技术栈

#### 前端技术栈
| 技术 | 版本 | 用途 |
|------|------|------|
| Vue.js | 3.x | 前端框架 |
| Element Plus | 最新 | UI组件库 |
| Pinia | 最新 | 状态管理 |
| Vue Router | 最新 | 路由管理 |
| Axios | 最新 | HTTP客户端 |
| TypeScript | 5.x | 类型系统 |

#### 后端技术栈
| 技术 | 版本 | 用途 |
|------|------|------|
| Python | 3.11+ | 编程语言 |
| FastAPI | 最新 | Web框架 |
| Pydantic | v2 | 数据验证 |
| SQLAlchemy | 2.x | ORM框架 |
| Alembic | 最新 | 数据库迁移 |
| openpyxl | 最新 | Excel处理 |
| pandas | 最新 | 数据处理 |
| reportlab / WeasyPrint | 最新 | PDF生成 |
| Pillow | 最新 | 图片处理 |

#### 数据库与存储
| 技术 | 版本 | 用途 |
|------|------|------|
| MySQL | 8.0+ | 关系型数据库 |
| MinIO | 最新 | 对象存储（SKU图片） |

#### 开发工具
| 技术 | 用途 |
|------|------|
| Docker | 容器化部署 |
| Git | 版本控制 |
| pytest | 单元测试 |
| ruff | 代码检查 |
| mypy | 类型检查 |

### 3.3 项目目录结构

```
super-order/
├── frontend/                 # 前端项目
│   ├── src/
│   │   ├── api/             # API接口
│   │   ├── components/      # 通用组件
│   │   ├── views/           # 页面视图
│   │   ├── stores/          # Pinia状态管理
│   │   ├── router/          # 路由配置
│   │   ├── utils/           # 工具函数
│   │   └── types/           # TypeScript类型定义
│   ├── package.json
│   └── vite.config.ts
├── backend/                  # 后端项目
│   ├── app/
│   │   ├── api/             # API路由
│   │   ├── models/          # 数据模型
│   │   ├── schemas/         # Pydantic模型
│   │   ├── services/        # 业务逻辑
│   │   ├── core/            # 核心配置
│   │   ├── db/              # 数据库配置
│   │   └── utils/           # 工具函数
│   ├── alembic/             # 数据库迁移
│   ├── tests/               # 测试文件
│   ├── requirements.txt
│   └── main.py
├── docker/                   # Docker配置
├── docs/                     # 文档
└── README.md
```

---

## 4. 数据模型设计

### 4.1 SKU相关表

#### 4.1.1 sku_category (SKU分类表)
```sql
CREATE TABLE sku_category (
    category_id VARCHAR(2) PRIMARY KEY COMMENT '分类ID (2位)',
    category_name VARCHAR(100) NOT NULL COMMENT '分类名称',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

**固定分类数据初始化**:
```sql
INSERT INTO sku_category (category_id, category_name) VALUES
('01', '食品'),
('02', '纸品'),
('03', '个护'),
('04', '百货');
```

#### 4.1.2 sku (SKU表)
```sql
CREATE TABLE sku (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    sku_code VARCHAR(6) NOT NULL UNIQUE COMMENT 'SKU编号 (分类ID2位+流水号4位)',
    name VARCHAR(200) NOT NULL COMMENT '产品名称',
    description TEXT COMMENT '产品描述',
    spec VARCHAR(100) COMMENT '规格型号',
    unit VARCHAR(20) DEFAULT '个' COMMENT '单位',
    category_id VARCHAR(2) NOT NULL COMMENT '分类ID',
    box_spec VARCHAR(100) COMMENT '箱规（如：48包、136g*40包）',
    cost_price DECIMAL(10, 2) COMMENT '成本单价（元）',
    sale_price DECIMAL(10, 2) COMMENT '销售单价（元）',
    image_urls JSON COMMENT '图片URL列表',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_sku_code (sku_code),
    INDEX idx_name (name),
    INDEX idx_category_id (category_id)
);
```

### 4.2 订单相关表

#### 4.2.1 customer (客户表)
```sql
CREATE TABLE customer (
    customer_id VARCHAR(10) PRIMARY KEY COMMENT '客户ID (3位)',
    customer_name VARCHAR(100) NOT NULL COMMENT '客户名称',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_customer_id (customer_id)
);
```

**固定客户数据初始化**:
```sql
INSERT INTO customer (customer_id, customer_name) VALUES
('FZC', '方舟'),
('SE8', 'SE8加拿大');
```

#### 4.2.2 order (订单表)
```sql
CREATE TABLE `order` (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    order_no VARCHAR(20) NOT NULL UNIQUE COMMENT '订单编号 (客户ID3位+yyyyMMdd)',
    customer_id VARCHAR(10) NOT NULL COMMENT '客户ID',
    customer_name VARCHAR(100) COMMENT '客户名称快照',
    order_date DATE NOT NULL COMMENT '订单日期',
    status ENUM('pending', 'processing', 'completed', 'cancelled') DEFAULT 'pending' COMMENT '订单状态',
    total_cost_amount DECIMAL(12, 2) NOT NULL DEFAULT 0 COMMENT '订单总成本金额',
    total_sale_amount DECIMAL(12, 2) NOT NULL DEFAULT 0 COMMENT '订单总销售金额',
    total_profit DECIMAL(12, 2) NOT NULL DEFAULT 0 COMMENT '订单总利润',
    total_income DECIMAL(12, 2) NOT NULL DEFAULT 0 COMMENT '总收入',
    total_expense DECIMAL(12, 2) NOT NULL DEFAULT 0 COMMENT '总支出',
    balance DECIMAL(12, 2) NOT NULL DEFAULT 0 COMMENT '余额',
    remark TEXT COMMENT '订单备注',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_order_no (order_no),
    INDEX idx_customer_id (customer_id),
    INDEX idx_order_date (order_date),
    INDEX idx_status (status)
);
```

#### 4.2.3 order_item (订单明细表)
```sql
CREATE TABLE order_item (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    order_id BIGINT NOT NULL COMMENT '订单ID',
    sku_id BIGINT COMMENT 'SKU ID',
    sku_code VARCHAR(50) COMMENT 'SKU编号快照',
    product_name VARCHAR(200) NOT NULL COMMENT '产品名称快照',
    cost_price DECIMAL(10, 2) COMMENT '成本单价（元）',
    sale_price DECIMAL(10, 2) COMMENT '销售单价（元）',
    quantity DECIMAL(10, 2) COMMENT '数量',
    box_quantity DECIMAL(10, 2) COMMENT '整箱数量（Excel中无此字段，导入时特殊处理：当unit不等于"箱"时，整箱数量=quantity；否则留空）',
    unit VARCHAR(20) COMMENT '单位',
    cost_amount DECIMAL(10, 2) COMMENT '成本金额（元）',
    sale_amount DECIMAL(10, 2) COMMENT '销售金额（元）',
    profit DECIMAL(10, 2) COMMENT '利润',
    box_spec VARCHAR(100) COMMENT '箱规',
    purchase_no VARCHAR(50) COMMENT '采购单据号',
    remark TEXT COMMENT '备注',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    INDEX idx_order_id (order_id),
    INDEX idx_sku_id (sku_id)
);
```

### 4.3 系统表

#### 4.3.1 user (用户表)
```sql
CREATE TABLE user (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '密码哈希',
    real_name VARCHAR(50) COMMENT '真实姓名',
    role ENUM('admin', 'user') DEFAULT 'user' COMMENT '角色',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否启用',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

---

## 5. 用户交互设计

### 5.1 SKU快速匹配交互

**场景**: 订单录入时输入SKU编号或产品名称

**交互流程**:
1. 用户在SKU输入框中输入关键字
2. 系统实时显示匹配的SKU候选列表（下拉框）
3. 用户使用键盘↓↑键在候选列表中导航
4. 按下Enter键确认选择
5. 系统自动填充SKU相关信息（产品名称、单价等）

**技术实现**:
- 使用 Element Plus Autocomplete 组件
- 监听键盘事件（keydown、keyup）
- 支持防抖（debounce）减少请求次数

### 5.2 Excel导入交互

**场景**: 导入历史订单数据

**交互流程**:
1. 用户点击"导入订单"按钮
2. 上传Excel文件
3. 系统解析并预览数据
4. 用户确认导入
5. 显示导入进度和结果
6. 展示成功/失败明细

### 5.3 对账单生成交互

**场景**: 生成客户对账单

**交互流程**:
1. 选择客户
2. 选择日期范围
3. 点击"生成对账单"
4. 预览对账单
5. 确认无误后下载PDF

---

## 6. 接口设计 (API)

### 6.1 SKU管理接口

| 接口路径 | 方法 | 说明 |
|---------|------|------|
| /api/sku | GET | 获取SKU列表 |
| /api/sku/{id} | GET | 获取SKU详情 |
| /api/sku | POST | 创建SKU |
| /api/sku/{id} | PUT | 更新SKU |
| /api/sku/{id} | DELETE | 删除SKU |
| /api/sku/search | GET | SKU模糊搜索（用于自动完成） |
| /api/sku/{id}/image | POST | 上传SKU图片 |
| /api/sku/category | GET | 获取分类列表（固定数据） |

### 6.2 客户管理接口

| 接口路径 | 方法 | 说明 |
|---------|------|------|
| /api/customer | GET | 获取客户列表（固定数据） |

### 6.3 订单管理接口

| 接口路径 | 方法 | 说明 |
|---------|------|------|
| /api/order | GET | 获取订单列表 |
| /api/order/{id} | GET | 获取订单详情 |
| /api/order | POST | 创建订单 |
| /api/order/{id} | PUT | 更新订单 |
| /api/order/{id}/status | PATCH | 更新订单状态 |
| /api/order/{id}/items | POST | 添加订单明细 |
| /api/order/{id}/items/{item_id} | PUT | 更新订单明细 |
| /api/order/{id}/items/{item_id} | DELETE | 删除订单明细 |

### 6.4 导入接口

| 接口路径 | 方法 | 说明 |
|---------|------|------|
| /api/import/order | POST | 导入Excel订单 |
| /api/import/template | GET | 下载导入模板 |

### 6.5 报表接口

| 接口路径 | 方法 | 说明 |
|---------|------|------|
| /api/report/statement | POST | 生成对账单PDF |
| /api/report/statistics | GET | 获取统计数据 |

---

## 7. 非功能性需求

### 7.1 性能要求
- **响应时间**: 页面加载时间 < 2秒
- **并发能力**: 支持50个并发用户
- **查询性能**: SKU搜索响应时间 < 500ms
- **导入性能**: 1000条订单导入时间 < 30秒

### 7.2 安全性要求
- **认证**: JWT Token认证
- **授权**: 基于角色的访问控制（RBAC）
- **数据加密**: 密码使用bcrypt加密
- **SQL注入防护**: 使用ORM参数化查询
- **XSS防护**: 前端输入过滤和转义

### 7.3 可用性要求
- **系统可用性**: ≥ 99%
- **数据备份**: 每日自动备份
- **错误处理**: 友好的错误提示

### 7.4 兼容性要求
- **浏览器支持**: Chrome、Firefox、Safari、Edge（最新版本）
- **分辨率**: 最低支持1366x768
- **移动端**: 响应式设计，支持平板访问

### 7.5 可维护性要求
- **代码规范**: 遵循PEP8（Python）、ESLint（JavaScript）
- **文档**: 完整的API文档和开发文档
- **日志**: 详细的操作日志和错误日志
- **测试**: 单元测试覆盖率 ≥ 70%

---

## 8. 交付物清单

### 8.1 代码交付
- [ ] 前端项目代码
- [ ] 后端项目代码
- [ ] 数据库迁移脚本
- [ ] Docker部署文件

### 8.2 文档交付
- [ ] API接口文档
- [ ] 部署文档
- [ ] 用户操作手册
- [ ] 开发文档

### 8.3 测试交付
- [ ] 单元测试报告
- [ ] 集成测试报告
- [ ] 测试用例

---

## 9. 项目里程碑

| 阶段 | 内容 | 预计工期 |
|------|------|---------|
| 需求分析 | 需求确认、PRD评审 | 3天 |
| 技术设计 | 架构设计、数据库设计 | 5天 |
| 后端开发 | API开发、数据库实现 | 15天 |
| 前端开发 | 页面开发、交互实现 | 15天 |
| 联调测试 | 前后端联调、功能测试 | 7天 |
| 部署上线 | 环境部署、数据迁移 | 3天 |
| **总计** | | **48天** |

---

## 10. 风险与应对

| 风险 | 影响 | 应对措施 |
|------|------|---------|
| Excel数据格式复杂 | 导入失败率高 | 提供标准模板，加强数据校验 |
| SKU去重逻辑复杂 | 数据冲突 | 明确去重规则，提供冲突处理方案 |
| 用户不熟悉键盘操作 | 使用体验差 | 提供操作指引，保留鼠标操作方式 |
| PDF生成中文乱码 | 对账单无法使用 | 使用支持中文的PDF库，测试中文字体 |
| MinIO配置复杂 | 图片上传失败 | 提供详细部署文档，使用Docker简化部署 |

---

## 11. 附录

### 11.1 SKU编号生成规则
```
格式: 分类ID(2位) + 流水号(4位，0001-9999)
示例: 010001（分类01的第1个SKU）、020123（分类02的第123个SKU）
说明: 每个分类独立维护流水号，从0001开始递增
```

### 11.2 订单编号生成规则
```
格式: 客户ID(3位) + yyyyMMdd
示例: FZC20250221（方舟客户2025年2月21日的订单）、SE820250221（SE8加拿大客户2025年2月21日的订单）
说明: 同一客户同一天可能有多笔订单，需要支持后续添加序号或时间戳
```

### 11.3 固定数据说明

#### 客户固定数据
| 客户ID | 客户名称 |
|-------|---------|
| FZC | 方舟 |
| SE8 | SE8加拿大 |

#### SKU分类固定数据
| 分类ID | 分类名称 |
|-------|---------|
| 01 | 食品 |
| 02 | 纸品 |
| 03 | 个护 |
| 04 | 百货 |

### 11.4 Excel导入模板示例

| 出货日期 | 产品图片 | 产品名称 | 成本价/箱 | 报价/元 | 数量 | 单位 | 成本金额/元 | 出货金额/元 | 状态 | 每箱 | 单位.1 | 采购单据 | 利润 | 支出 | 收入 | 余额 | 备注 |
|---------|---------|---------|---------|---------|------|------|-----------|-----------|------|------|--------|---------|------|------|------|------|------|
| 2024-04-13 | | 消毒卫生巾（日用） | 924.08 | 20.0 | 48 | 箱 | 924.8 | 960.0 | 已出货 | 48 | 包 | 1 | 35.2 | 924.8 | 2572.0 | 1647.2 | 2024/3/12微信收奋燕2572元 |
| 2024-04-13 | | 消毒卫生巾（夜用） | 958.72 | 20.8 | 48 | 箱 | 958.0 | 998.4 | 已出货 | 48 | 包 | 1 | 40.4 | 958.0 | 0 | 689.2 | |

---

**文档结束**
