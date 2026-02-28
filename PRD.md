# 订单管理系统 - 产品需求文档 (PRD)

## 文档信息

| 项目 | 内容 |
|------|------|
| 文档版本 | v2.0 |
| 创建日期 | 2025-02-21 |
| 更新日期 | 2026-02-22 |
| 项目名称 | Super Order 订单管理系统（桌面版）|
| 文档类型 | 产品需求文档 (PRD) |

---

## 1. 项目背景与目标

### 1.1 项目背景
客户需要一个完整的订单管理系统，用于管理SKU（库存单位）、订单和SKU价格。系统需要支持以下核心业务场景：
- 从历史Excel订单数据中导入并自动生成SKU和订单记录
- 日常订单录入，支持SKU快速模糊匹配
- 生成PDF格式的对账单
- SKU图片存储和管理
- 类似Excel的操作界面，提供直观的数据管理体验

### 1.2 项目目标
- **主要目标**: 构建一个高效、易用的桌面订单管理系统，提高订单处理效率
- **核心价值**: 
  - 类似Excel的操作界面，降低学习成本
  - 支持键盘快捷操作，提升录入效率
  - 智能去重和SKU自动生成
  - 灵活的Excel数据导入
  - 专业的PDF对账单生成
  - 离线数据存储，支持无网络环境使用

### 1.3 目标用户
- **主要用户**: 订单管理员、销售助理
- **用户特征**: 
  - 日常处理大量订单
  - 习惯使用Excel操作方式
  - 习惯使用键盘快捷操作
  - 需要快速查询和录入SKU信息
  - 可能在无网络环境下工作

### 1.4 本期范围说明
**不在本期实现的功能**:
- 客户管理功能（新增、编辑、删除客户）
- SKU分类管理功能（新增、编辑、删除分类）

**说明**: 本期使用固定的客户数据和SKU分类数据，客户管理和分类管理功能将在后续版本中实现。

---

## 2. 功能需求

### 2.1 SKU管理模块

#### 2.1.1 SKU基础信息管理
**功能描述**: 管理SKU的基础信息，采用Excel风格的数据表格界面

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
- 图片（支持多张）
- 创建时间、更新时间

**功能点**:
- [ ] SKU新增（支持直接在表格中录入）
- [ ] SKU编辑（双击单元格编辑）
- [ ] SKU删除（支持多选删除）
- [ ] SKU列表查询（支持多条件筛选）
- [ ] SKU详情查看
- [ ] SKU图片上传（支持拖拽上传）
- [ ] SKU图片预览和管理
- [ ] Excel风格的数据操作（复制、粘贴、撤销）
- [ ] 单元格编辑验证（数据格式校验）
- [ ] 数据导出为Excel

#### 2.1.2 SKU价格管理
**功能描述**: 管理SKU的价格修改历史

**字段要求**:
- 记录ID（系统生成）
- SKU编号
- 产品名称（快照）
- 修改类型（成本单价、销售单价）
- 修改前价格
- 修改后价格
- 修改原因（选填）
- 修改人
- 修改时间

**功能点**:
- [ ] 查看SKU价格修改历史
- [ ] 按SKU查询价格历史
- [ ] 按日期范围查询价格历史
- [ ] 按修改人查询价格历史
- [ ] 价格变更趋势图表

**说明**: 当用户修改SKU的成本单价或销售单价时，系统自动记录价格变更历史，支持追溯和审计

#### 2.1.3 SKU分类管理
**功能描述**: SKU分类管理（本期不实现管理功能，仅使用固定数据）

**说明**: 本期不实现分类的新增、编辑、删除功能，仅使用预设的固定分类数据

**固定分类数据**:
| 分类ID | 分类名称 |
|--------|----------|
| 01 | 食品 |
| 02 | 纸品 |
| 03 | 个护 |
| 04 | 百货 |
| 05 | 电器 |
| 06 | 服饰床品 |

### 2.2 订单管理模块

#### 2.2.1 订单录入
**功能描述**: 日常订单录入，采用Excel风格的数据表格界面

**字段要求**:
- 订单编号（系统生成）
- 客户名称（从固定客户列表选择）
- 订单日期（默认今日）
- 订单项列表（SKU、数量、单价）
- 订单金额（自动计算）
- 备注（选填）
- 创建时间、更新时间

**功能点**:
- [ ] 新建订单
- [ ] SKU快速模糊匹配（输入关键字自动匹配）
- [ ] 键盘快捷操作支持
- [ ] 订单金额自动计算
- [ ] 订单保存
- [ ] 订单编辑
- [ ] 订单删除
- [ ] 订单复制
- [ ] Excel风格的数据操作
- [ ] 单元格编辑验证

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

### 2.2.2 订单查询
**功能描述**: 订单历史查询

**功能点**:
- [ ] 按订单日期范围查询
- [ ] 按客户名称查询
- [ ] 按订单编号查询
- [ ] 订单详情查看
- [ ] 订单导出为Excel

#### 2.2.3 对账单生成
**功能描述**: 生成PDF格式的对账单

**功能点**:
- [ ] 选择订单生成对账单
- [ ] 按客户生成对账单
- [ ] 按日期范围生成对账单
- [ ] 对账单预览
- [ ] 对账单导出为PDF
- [ ] 对账单打印

### 2.4 结算管理模块

#### 2.4.1 结算管理
**功能描述**: 管理订单的结算状态，查看未结算订单的明细信息

**功能点**:
- [ ] 显示未结算订单下拉列表（is_settled=0的订单）
- [ ] 选择订单后显示订单明细（order_item列表）
- [ ] 标记订单为已结算
- [ ] 查看结算状态
- [ ] 订单明细信息展示

### 2.5 数据导入模块

#### 2.3.1 Excel订单导入
**功能描述**: 从Excel文件导入历史订单数据

**功能点**:
- [ ] 支持选择Excel文件导入
- [ ] 自动识别Excel中的产品信息
- [ ] 支持多Sheet读取
- [ ] 产品名称去重
- [ ] SKU自动生成
- [ ] 图片提取和上传
- [ ] 导入结果展示（成功/失败统计）
- [ ] 导入错误提示

---

## 3. 技术架构

### 3.1 整体架构

```
┌─────────────────────────────────────────┐
│  桌面应用 (Electron + Vue 3)        │
│   ├─ 主进程 (Node.js)               │
│   │   ├─ 窗口管理                   │
│   │   ├─ 文件系统访问                │
│   │   └─ 本地存储 (SQLite/IndexedDB)  │
│   └─ 渲染进程 (Vue 3 + Element Plus) │
│       ├─ UI 组件                    │
│       ├─ 状态管理 (Pinia)             │
│       ├─ 路由 (Vue Router)            │
│       └─ API 通信 (Axios)            │
├─────────────────────────────────────────┤
│  可选：后端 API (FastAPI)             │
├─────────────────────────────────────────┤
│  可选：数据库 (MySQL 8.0)           │
└─────────────────────────────────────────┘
```

### 3.2 技术栈

#### 桌面应用技术栈
| 技术 | 版本 | 用途 |
|------|------|------|
| Electron | 最新 | 桌面应用框架 |
| Node.js | 18+ | 运行时环境 |
| Vue.js | 3.x | 前端框架 |
| Element Plus | 最新 | UI组件库 |
| Pinia | 最新 | 状态管理 |
| Vue Router | 最新 | 路由管理 |
| Axios | 最新 | HTTP客户端 |
| TypeScript | 5.x | 类型系统 |
| electron-builder | 最新 | 应用打包 |

#### 本地数据存储
| 技术 | 版本 | 用途 |
|------|------|------|
| SQLite | 3.x | 本地数据库 |
| IndexedDB | - | 浏览器本地存储 |
| electron-store | 最新 | 应用配置存储 |

#### 可选：后端技术栈
| 技术 | 版本 | 用途 |
|------|------|------|
| Python | 3.11+ | 编程语言 |
| FastAPI | 最新 | Web框架 |
| Pydantic | v2 | 数据验证 |
| SQLAlchemy | 2.x | ORM框架 |
| openpyxl | 最新 | Excel处理 |
| pandas | 最新 | 数据处理 |
| reportlab / WeasyPrint | 最新 | PDF生成 |
| Pillow | 最新 | 图片处理 |

#### 可选：数据库与存储
| 技术 | 版本 | 用途 |
|------|------|------|
| MySQL | 8.0+ | 关系型数据库 |

#### 开发工具
| 技术 | 用途 |
|------|------|
| Docker | 容器化部署 |
| Git | 版本控制 |
| pytest | 单元测试 |
| ruff | 代码检查 |
| mypy | 类型检查 |
| electron-packager | 应用打包 |

### 3.3 项目目录结构

```
super-order/
├── app/                      # Electron 主进程
│   ├── main.js              # 主进程入口
│   ├── preload.js           # 预加载脚本
│   ├── window/              # 窗口管理
│   ├── ipc/                 # IPC 通信处理
│   └── database/            # SQLite 数据库操作
├── src/                      # Vue 3 渲染进程
│   ├── main.ts              # Vue 入口
│   ├── App.vue              # 根组件
│   ├── api/                 # API 接口
│   ├── components/          # 通用组件
│   │   ├── DataGrid/        # Excel 风格数据表格
│   │   ├── SKUForm/         # SKU 表单组件
│   │   └── OrderForm/       # 订单表单组件
│   ├── views/               # 页面视图
│   │   ├── SKU/             # SKU 管理页面
│   │   ├── Order/           # 订单管理页面
│   │   └── Report/          # 报表页面
│   ├── stores/              # Pinia 状态管理
│   ├── router/              # 路由配置
│   ├── utils/               # 工具函数
│   └── types/              # TypeScript 类型定义
├── public/                   # 静态资源
│   ├── icons/               # 应用图标
│   └── images/              # 图片资源
├── scripts/                  # 工具脚本
│   ├── import_sku.py        # SKU 导入脚本
│   └── setup_minio_policy.py # MinIO 配置
├── docker/                   # Docker 配置（可选后端）
│   └── docker-compose.yml
├── docs/                     # 文档
├── package.json              # Electron 项目配置
├── electron-builder.yml      # 打包配置
├── PRD.md                    # 产品需求文档
├── 开发计划.md                # 开发计划文档
└── README.md                 # 项目说明
```

### 3.4 Excel风格数据表格组件

#### 组件功能要求
- 支持单元格直接编辑（双击编辑）
- 支持键盘导航（方向键、Tab、Enter）
- 支持单元格复制/粘贴
- 支持行/列的批量操作
- 支持数据验证和格式化
- 支持虚拟滚动（大数据量优化）
- 支持列宽调整
- 支持列冻结/锁定
- 支持数据筛选和排序

#### 技术实现建议
- 使用 Element Plus 的 Table 组件作为基础
- 扩展编辑功能实现 Excel 风格交互
- 考虑使用成熟的 Excel 风格组件库（如 Luckysheet、Handsontable）

### 3.5 桌面应用特性

#### 文件系统访问
- 通过 Electron 的 IPC 通信实现文件读写
- 支持选择本地 Excel 文件导入
- 支持导出数据到本地文件
- 支持本地图片上传和预览

#### 本地数据存储
- 使用 SQLite 作为本地数据库
- 实现数据持久化
- 支持离线模式使用
- 可选：支持云端数据同步

#### 窗口管理
- 主窗口设计
- 窗口大小和位置记忆
- 系统托盘支持
- 多窗口支持（可选）

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
('04', '百货'),
('05', '电器'),
('06', '服饰床品');
```

#### 4.1.2 sku (SKU表)
```sql
CREATE TABLE sku (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sku_code VARCHAR(6) NOT NULL UNIQUE COMMENT 'SKU编号 (分类ID2位+流水号4位)',
    product_name VARCHAR(200) NOT NULL COMMENT '产品名称',
    product_desc TEXT COMMENT '产品描述',
    spec_model VARCHAR(100) COMMENT '规格型号',
    unit VARCHAR(20) DEFAULT '个' COMMENT '单位',
    category_id VARCHAR(2) NOT NULL COMMENT '分类ID',
    box_spec VARCHAR(100) COMMENT '箱规（如：48包、136g*40包）',
    box_quantity INTEGER DEFAULT 1 COMMENT '每箱数量，它是箱规的量化字段，代表每箱会有多少件商品',
    cost_price DECIMAL(10, 2) COMMENT '成本单价（元）',
    sale_price DECIMAL(10, 2) COMMENT '销售单价（元）',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 4.1.3 sku_price_log (SKU价格修改记录表)
```sql
CREATE TABLE sku_price_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sku_id INTEGER NOT NULL COMMENT 'SKU ID',
    sku_code VARCHAR(6) NOT NULL COMMENT 'SKU编号',
    product_name VARCHAR(200) NOT NULL COMMENT '产品名称快照',
    change_type TEXT NOT NULL COMMENT '修改类型：cost_price/sale_price',
    old_price DECIMAL(10, 2) COMMENT '修改前价格',
    new_price DECIMAL(10, 2) COMMENT '修改后价格',
    reason VARCHAR(500) COMMENT '修改原因',
    changed_by VARCHAR(50) COMMENT '修改人',
    changed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '修改时间'
);
```

**说明**: 当SKU的成本单价或销售单价发生变化时，自动插入一条价格变更记录

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

#### 4.2.2 `order` (订单表)
```sql
CREATE TABLE `order` (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_no TEXT UNIQUE NOT NULL COMMENT '订单编号',
    customer_id TEXT NOT NULL COMMENT '客户ID',
    order_date TEXT NOT NULL COMMENT '订单日期',
    status TEXT DEFAULT 'pending' COMMENT '订单状态',
    is_settled INTEGER DEFAULT 0 COMMENT '是否结算',
    total_cost_amount REAL DEFAULT 0 COMMENT '总成本',
    total_sale_amount REAL DEFAULT 0 COMMENT '总售价',
    remarks TEXT COMMENT '备注',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (customer_id) REFERENCES customer(customer_id)
);
```

#### 4.2.3 order_item (订单明细表)
```sql
CREATE TABLE order_item (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    order_id INTEGER NOT NULL COMMENT '订单ID',
    sku_id INTEGER COMMENT 'SKU ID',
    sku_code TEXT NOT NULL COMMENT 'SKU编号',
    product_name TEXT NOT NULL COMMENT '产品名称快照',
    quantity INTEGER NOT NULL COMMENT '数量',
    cost_price REAL NOT NULL COMMENT '成本价',
    sale_price REAL NOT NULL COMMENT '销售价',
    total_cost_amount REAL NOT NULL COMMENT '总成本',
    total_sale_amount REAL NOT NULL COMMENT '总售价',
    settled_amount REAL DEFAULT 0 COMMENT '已结算金额',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (order_id) REFERENCES `order`(id),
    FOREIGN KEY (sku_id) REFERENCES sku(id)
);
```

### 4.3 用户表

#### 4.3.1 user (用户表)
```sql
CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username VARCHAR(50) UNIQUE NOT NULL COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '密码哈希',
    full_name VARCHAR(100) COMMENT '全名',
    email VARCHAR(100) COMMENT '邮箱',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否激活',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

#### 4.3.2 financial_transaction (财务收支表)
```sql
CREATE TABLE financial_transaction (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT NOT NULL COMMENT '分类（如：收入、支出）',
    description TEXT COMMENT '说明',
    amount_change REAL NOT NULL COMMENT '金额变化（正数为收入，负数为支出）',
    balance REAL NOT NULL COMMENT '结余',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间'
);
```

---

## 5. 非功能性需求

### 5.1 性能要求
- SKU列表加载时间 < 1秒（1000条以内）
- 订单列表加载时间 < 1秒（1000条以内）
- SKU搜索响应时间 < 500ms
- Excel导入处理速度 > 1000条/分钟
- 应用启动时间 < 5秒

### 5.2 可用性要求
- 界面简洁直观，类似Excel操作体验
- 支持键盘快捷键操作
- 提供操作提示和帮助文档
- 支持撤销/重做操作
- 错误提示清晰明确

### 5.3 兼容性要求
- Windows 10/11
- macOS 11+
- Linux (可选)

### 5.4 安全性要求
- 本地数据加密存储
- 敏感操作需要确认
- 数据备份机制

### 5.5 可维护性要求
- 代码结构清晰
- 组件化设计
- 完善的错误处理
- 日志记录

---

## 6. 开发里程碑

### 6.1 第一阶段：基础架构搭建（Week 1-2）
- [ ] Electron 项目初始化
- [ ] Vue 3 + Element Plus 集成
- [ ] 主进程和渲染进程通信搭建
- [ ] SQLite 数据库初始化
- [ ] 基础 UI 框架搭建

### 6.2 第二阶段：核心功能开发（Week 3-5）
- [ ] Excel 风格数据表格组件开发
- [ ] SKU 管理 CRUD 功能
- [ ] SKU 导入功能
- [ ] 订单录入功能
- [ ] SKU 模糊搜索功能

### 6.3 第三阶段：高级功能开发（Week 6-7）
- [ ] 对账单生成功能
- [ ] 价格历史记录功能
- [ ] 数据导出功能
- [ ] 图片管理功能

### 6.4 第四阶段：测试和优化（Week 8）
- [ ] 功能测试
- [ ] 性能优化
- [ ] 用户体验优化
- [ ] Bug 修复

### 6.5 第五阶段：打包发布（Week 9）
- [ ] 应用打包配置
- [ ] 安装包制作
- [ ] 用户文档编写
- [ ] 发布准备

---

## 7. IPC接口设计（可选后端）

### 7.1 SKU管理IPC接口

| 接口路径 | 说明 |
|---------|------|
| sku:list | 获取SKU列表 |
| sku:get | 获取SKU详情 |
| sku:create | 创建SKU |
| sku:update | 更新SKU |
| sku:delete | 删除SKU |
| sku:search | SKU模糊搜索 |
| sku:upload-image | 上传SKU图片 |
| sku:category | 获取分类列表 |

### 7.2 客户管理IPC接口

| 接口路径 | 说明 |
|---------|------|
| customer:list | 获取客户列表 |

### 7.3 订单管理IPC接口

| 接口路径 | 说明 |
|---------|------|
| order:list | 获取订单列表 |
| order:get | 获取订单详情 |
| order:create | 创建订单 |
| order:update | 更新订单 |
| order:items | 管理订单明细 |

### 7.4 报表IPC接口

| 接口路径 | 说明 |
|---------|------|
| report:statement | 生成对账单PDF |
| report:statistics | 获取统计数据 |

---

## 8. 风险和注意事项

### 8.1 技术风险
- Excel 风格数据表格组件开发复杂度较高
- Electron 应用包体积较大
- SQLite 和 MySQL 数据库结构差异（如果后续需要云端同步）

### 8.2 业务风险
- 用户对 Excel 操作习惯的差异
- 数据导入的兼容性问题
- 性能优化需求

### 8.3 缓解措施
- 优先实现核心数据表格功能
- 提供详细的用户操作指南
- 进行充分的兼容性测试
- 预留性能优化空间

---

## 9. 附录

### 9.1 键盘快捷键
| 快捷键 | 功能 |
|--------|------|
| Ctrl+N | 新建订单 |
| Ctrl+S | 保存 |
| Ctrl+F | 搜索 |
| Ctrl+E | 导出 |
| Delete | 删除选中项 |
| Enter | 编辑/确认 |
| Esc | 取消/关闭 |

### 9.2 数据格式说明
- **SKU编号**: 6位数字，如 "000001"
- **订单编号**: "ORD" + 8位日期 + 4位序号，如 "ORD202602220001"
- **日期格式**: YYYY-MM-DD
- **金额格式**: 2位小数，如 "100.00"

### 9.3 参考资料
- Electron 官方文档: https://www.electronjs.org/
- Vue 3 官方文档: https://vuejs.org/
- Element Plus 文档: https://element-plus.org/
- SQLite 文档: https://www.sqlite.org/

---

**文档结束**

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

| 产品类目 | 产品图片 | 产品名称 | 成本价/箱 | 报价/元 | 数量 | 单位 | 成本金额/元 | 出货金额/元 | 状态 | 每箱 | 单位 | 采购单据 | 利润 | 支出 | 收入 | 余额 | 备注 |
|---------|---------|---------|---------|---------|------|------|-----------|-----------|------|------|--------|---------|------|------|------|------|------|
| 01 | | 消毒卫生巾（日用） | 924.08 | 20.0 | 48 | 箱 | 924.8 | 960.0 | 已出货 | 48 | 包 | 1 | 35.2 | 924.8 | 2572.0 | 1647.2 | 2024/3/12微信收奋燕2572元 |
| 01 | | 消毒卫生巾（夜用） | 958.72 | 20.8 | 48 | 箱 | 958.0 | 998.4 | 已出货 | 48 | 包 | 1 | 40.4 | 958.0 | 0 | 689.2 | |

---

**文档结束**
