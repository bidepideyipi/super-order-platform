import pandas as pd
from sqlalchemy import create_engine, text
from sqlalchemy.orm import sessionmaker
from minio_client import MinIOClient
from config import *
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class SKUImporter:
    def __init__(self):
        self.engine = create_engine(DATABASE_URL)
        self.SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=self.engine)
        self.minio_client = MinIOClient()
        self.category_mapping = {
            '食品': '01',
            '纸品': '02',
            '个护': '03',
            '百货': '04'
        }

    def _get_session(self):
        return self.SessionLocal()

    def _generate_sku_code(self, category_id: str) -> str:
        query = text(f"""
            SELECT sku_code FROM sku 
            WHERE category_id = :category_id 
            ORDER BY sku_code DESC 
            LIMIT 1
        """)
        
        with self._get_session() as session:
            result = session.execute(query, {"category_id": category_id}).fetchone()
            
            if result and result[0]:
                last_code = result[0]
                sequence = int(last_code[2:])
                new_sequence = sequence + 1
            else:
                new_sequence = 1
            
            new_sku_code = f"{category_id}{new_sequence:04d}"
            return new_sku_code

    def import_sku_from_excel(self, file_path: str):
        try:
            df = pd.read_excel(file_path)
            df.columns = df.columns.str.strip()
            logger.info(f"Loaded {len(df)} rows from Excel file")
            
            success_count = 0
            failed_count = 0
            errors = []
            
            with self._get_session() as session:
                for idx, row in df.iterrows():
                    try:
                        product_name = row.get('产品名称', '')
                        if pd.isna(product_name) or not product_name:
                            errors.append(f"Row {idx+2}: Missing product name")
                            failed_count += 1
                            continue
                        
                        category_name = self._determine_category(product_name)
                        category_id = self.category_mapping.get(category_name, '04')
                        sku_code = self._generate_sku_code(category_id)
                        
                        cost_price = row.get('成本单价', 0) or row.get('成本价/箱', 0)
                        sale_price = row.get('销售单价', 0) or row.get('报价/元', 0)
                        unit = row.get('单位', '个')
                        spec = row.get('规格', '')
                        box_spec = row.get('箱规', '')
                        
                        if pd.isna(cost_price):
                            cost_price = 0
                        if pd.isna(sale_price):
                            sale_price = 0
                        if pd.isna(spec):
                            spec = ''
                        if pd.isna(box_spec):
                            box_spec = ''
                        
                        image_urls = '[]'
                        
                        insert_sku = text("""
                            INSERT IGNORE INTO sku 
                            (sku_code, name, description, spec, unit, category_id, box_spec, cost_price, sale_price, image_urls)
                            VALUES (:sku_code, :name, :description, :spec, :unit, :category_id, :box_spec, :cost_price, :sale_price, :image_urls)
                        """)
                        
                        session.execute(insert_sku, {
                            "sku_code": sku_code,
                            "name": product_name,
                            "description": spec,
                            "spec": spec,
                            "unit": unit,
                            "category_id": category_id,
                            "box_spec": box_spec,
                            "cost_price": float(cost_price),
                            "sale_price": float(sale_price),
                            "image_urls": image_urls
                        })
                        
                        success_count += 1
                        logger.info(f"Processed row {idx+2}: {product_name} -> {sku_code}")
                        
                    except Exception as e:
                        error_msg = f"Row {idx+2}: {str(e)}"
                        errors.append(error_msg)
                        failed_count += 1
                        logger.error(error_msg)
                
                session.commit()
            
            result = {
                "total_count": len(df),
                "success_count": success_count,
                "failed_count": failed_count,
                "errors": errors[:10]
            }
            
            logger.info(f"Import completed: {result}")
            return result
            
        except Exception as e:
            logger.error(f"Import failed: {e}")
            raise

    def _determine_category(self, product_name: str) -> str:
        keywords = {
            '食品': ['食品', '肉肠', '辣条', '火腿'],
            '纸品': ['纸', '巾', '裤'],
            '个护': ['护', '甲', '美'],
            '百货': ['拖鞋', '手套', '清洁']
        }
        
        for category, items in keywords.items():
            if any(keyword in product_name for keyword in items):
                return category
        
        return '百货'

    def import_single_file(self, file_path: str):
        try:
            result = self.import_sku_from_excel(file_path)
            print("\n" + "="*50)
            print("SKU 导入结果:")
            print(f"总记录数: {result['total_count']}")
            print(f"成功数: {result['success_count']}")
            print(f"失败数: {result['failed_count']}")
            
            if result['errors']:
                print("\n错误详情（前10条）:")
                for error in result['errors']:
                    print(f"  - {error}")
            else:
                print("\n导入成功！")
                
            print("="*50 + "\n")
            
        except Exception as e:
            print(f"\n导入失败: {e}\n")
            raise


if __name__ == "__main__":
    import sys
    
    if len(sys.argv) > 1:
        excel_file = sys.argv[1]
    else:
        excel_file = f"{EXCEL_DIR}/方舟产品订单表（原始记录不外发）260112.xlsx"
    
    importer = SKUImporter()
    importer.import_single_file(excel_file)
