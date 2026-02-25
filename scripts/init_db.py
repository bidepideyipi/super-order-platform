import os
from sqlalchemy import create_engine, text
from config import DATABASE_URL, BASE_DIR

def init_database():
    engine = create_engine(DATABASE_URL)
    
    with engine.connect() as conn:
        
        conn.execute(text("""
            CREATE TABLE IF NOT EXISTS sku_category (
                category_id TEXT PRIMARY KEY,
                category_name TEXT NOT NULL
            )
        """))
        
        conn.execute(text("""
            INSERT OR IGNORE INTO sku_category (category_id, category_name) VALUES
            ('01', '食品'),
            ('02', '纸品'),
            ('03', '个护'),
            ('04', '百货'),
            ('05', '电器'),
            ('06', '服饰床品')
        """))
        
        conn.execute(text("""
            CREATE TABLE IF NOT EXISTS sku (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sku_code TEXT UNIQUE NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                spec TEXT,
                unit TEXT DEFAULT '个',
                category_id TEXT NOT NULL,
                box_spec TEXT,
                cost_price REAL DEFAULT 0,
                sale_price REAL DEFAULT 0,
                is_deleted INTEGER DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (category_id) REFERENCES sku_category(category_id)
            )
        """))
        
        conn.execute(text("""
            CREATE INDEX IF NOT EXISTS idx_sku_code ON sku(sku_code)
        """))
        
        conn.execute(text("""
            CREATE INDEX IF NOT EXISTS idx_category_id ON sku(category_id)
        """))
        
        conn.execute(text("""
            CREATE TABLE IF NOT EXISTS sku_price_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                sku_id INTEGER NOT NULL,
                sku_code TEXT NOT NULL,
                product_name TEXT NOT NULL,
                change_type TEXT NOT NULL,
                old_price REAL,
                new_price REAL,
                reason TEXT,
                changed_by TEXT,
                changed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (sku_id) REFERENCES sku(id)
            )
        """))
        
        conn.execute(text("""
            CREATE TABLE IF NOT EXISTS customer (
                customer_id TEXT PRIMARY KEY,
                customer_name TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        """))
        
        conn.execute(text("""
            INSERT OR IGNORE INTO customer (customer_id, customer_name) VALUES
            ('FZC', '方舟'),
            ('SE8', 'SE8加拿大')
        """))
        
        conn.execute(text("""
            CREATE TABLE IF NOT EXISTS `order` (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                order_no TEXT UNIQUE NOT NULL,
                customer_id TEXT NOT NULL,
                order_date TEXT NOT NULL,
                status TEXT DEFAULT 'pending',
                total_cost_amount REAL DEFAULT 0,
                total_sale_amount REAL DEFAULT 0,
                remarks TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (customer_id) REFERENCES customer(customer_id)
            )
        """))
        
        conn.execute(text("""
            CREATE INDEX IF NOT EXISTS idx_order_no ON `order`(order_no)
        """))
        
        conn.execute(text("""
            CREATE INDEX IF NOT EXISTS idx_customer_id ON `order`(customer_id)
        """))
        
        conn.execute(text("""
            CREATE TABLE IF NOT EXISTS order_item (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                order_id INTEGER NOT NULL,
                sku_id INTEGER,
                sku_code TEXT NOT NULL,
                product_name TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price REAL NOT NULL,
                subtotal REAL NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (order_id) REFERENCES `order`(id),
                FOREIGN KEY (sku_id) REFERENCES sku(id)
            )
        """))
        
        conn.execute(text("""
            CREATE INDEX IF NOT EXISTS idx_order_id ON order_item(order_id)
        """))
        
        conn.commit()
        
        print("Database initialized successfully!")
        print(f"Database location: {BASE_DIR}/data/super_order.db")

if __name__ == "__main__":
    init_database()
