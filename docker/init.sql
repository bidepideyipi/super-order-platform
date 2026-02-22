SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

CREATE TABLE IF NOT EXISTS sku_category (
    category_id VARCHAR(2) PRIMARY KEY COMMENT '分类ID (2位)',
    category_name VARCHAR(100) NOT NULL COMMENT '分类名称',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS customer (
    customer_id VARCHAR(10) PRIMARY KEY COMMENT '客户ID (3位)',
    customer_name VARCHAR(100) NOT NULL COMMENT '客户名称',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS sku (
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
    INDEX idx_category_id (category_id),
    FOREIGN KEY (category_id) REFERENCES sku_category(category_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS `order` (
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
    INDEX idx_status (status),
    FOREIGN KEY (customer_id) REFERENCES customer(customer_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS order_item (
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
    INDEX idx_sku_id (sku_id),
    FOREIGN KEY (order_id) REFERENCES `order`(id),
    FOREIGN KEY (sku_id) REFERENCES sku(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE IF NOT EXISTS user (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE COMMENT '用户名',
    password_hash VARCHAR(255) NOT NULL COMMENT '密码哈希',
    real_name VARCHAR(50) COMMENT '真实姓名',
    role ENUM('admin', 'user') DEFAULT 'user' COMMENT '角色',
    is_active BOOLEAN DEFAULT TRUE COMMENT '是否启用',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

INSERT IGNORE INTO sku_category (category_id, category_name) VALUES
('01', '食品'),
('02', '纸品'),
('03', '个护'),
('04', '百货'),
('05', '电器'),
('06', '服饰床品');

INSERT IGNORE INTO customer (customer_id, customer_name) VALUES
('FZC', '方舟'),
('SE8', 'SE8加拿大');

SET FOREIGN_KEY_CHECKS = 1;
