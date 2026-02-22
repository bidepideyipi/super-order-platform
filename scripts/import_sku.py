import pandas as pd
import openpyxl
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
        self.sku_counters = {
            '01': 0,
            '02': 0,
            '03': 0,
            '04': 0,
            '05': 0,
            '06': 0
        }

    def _get_session(self):
        return self.SessionLocal()

    def _upload_image_from_excel(self, file_path: str, row_idx: int, sku_code: str, sheet_name: str) -> str:
        try:
            wb = openpyxl.load_workbook(file_path)
            
            if sheet_name not in wb.sheetnames:
                logger.warning(f"Sheet {sheet_name} not found in workbook")
                return None
            
            ws = wb[sheet_name]
            
            if not hasattr(ws, '_images') or not ws._images:
                return None
            
            for img in ws._images:
                if hasattr(img.anchor, '_from'):
                    img_row = img.anchor._from.row
                    
                    if img_row == row_idx:
                        img_data = img._data()
                        
                        filename = f"{sku_code}.{img.format}"
                        
                        try:
                            self.minio_client.upload_from_memory(img_data, f"sku/{filename}")
                            url = self.minio_client.get_file_url(f"sku/{filename}")
                            logger.info(f"Uploaded image for {sku_code}: {url}")
                            return url
                        except Exception as e:
                            logger.error(f"Failed to upload image for {sku_code}: {e}")
                            return None
            
            return None
            
        except Exception as e:
            logger.error(f"Error extracting image from Excel: {e}")
            return None

    def _init_sku_counters(self):
        query = text(f"""
            SELECT category_id, MAX(CAST(SUBSTRING(sku_code, 3) AS UNSIGNED)) as max_seq
            FROM sku 
            GROUP BY category_id
        """)
        
        with self._get_session() as session:
            results = session.execute(query).fetchall()
            for result in results:
                category_id = result[0]
                max_seq = result[1] if result[1] else 0
                if category_id in self.sku_counters:
                    self.sku_counters[category_id] = max_seq

    def _generate_sku_code(self, category_id: str) -> str:
        self.sku_counters[category_id] += 1
        new_sku_code = f"{category_id}{self.sku_counters[category_id]:04d}"
        return new_sku_code

    def import_sku_from_excel(self, file_path: str):
        try:
            self._init_sku_counters()
            xls = pd.ExcelFile(file_path)
            logger.info(f"Excel file has {len(xls.sheet_names)} sheets: {xls.sheet_names}")
            
            all_dfs = []
            
            for sheet_name in xls.sheet_names:
                df = pd.read_excel(file_path, sheet_name=sheet_name, header=1)
                
                if len(df) == 0 or len(df.columns) == 0:
                    logger.info(f"Skipping empty sheet: {sheet_name}")
                    continue
                
                try:
                    df.columns = df.columns.str.strip()
                except (AttributeError, TypeError):
                    logger.info(f"Skipping sheet with non-string columns: {sheet_name}")
                    continue
                
                if '产品名称' not in df.columns:
                    if len(df.columns) >= 3 and not str(df.columns[2]).startswith('Unnamed'):
                        df = df.rename(columns={df.columns[2]: '产品名称'})
                        if len(df.columns) >= 1 and str(df.columns[0]).isdigit():
                            df = df.rename(columns={df.columns[0]: '产品类目'})
                    else:
                        df_raw = pd.read_excel(file_path, sheet_name=sheet_name, header=None)
                        
                        if len(df_raw) > 1:
                            df.columns = df_raw.iloc[0]
                            df = df_raw[1:].reset_index(drop=True)
                            df.columns = df.columns.str.strip()
                            
                            if len(df.columns) >= 3:
                                df = df.rename(columns={df.columns[2]: '产品名称'})
                            if len(df.columns) >= 1:
                                if str(df.columns[0]).isdigit():
                                    df = df.rename(columns={df.columns[0]: '产品类目'})
                                elif df.columns[0] == '产品分类':
                                    df = df.rename(columns={df.columns[0]: '产品类目'})
                
                if '产品名称' not in df.columns:
                    logger.info(f"Skipping sheet without product name column: {sheet_name}")
                    continue
                
                if df.columns.duplicated().any():
                    df.columns = [f"{col}_{i}" if col in df.columns[:i] else col for i, col in enumerate(df.columns)]
                
                df['_sheet_name'] = sheet_name
                df['_original_row'] = df.index + 2
                
                all_dfs.append(df)
                logger.info(f"Loaded {len(df)} rows from sheet: {sheet_name}")
            
            if all_dfs:
                df = pd.concat(all_dfs, ignore_index=True)
                df = df.loc[:, ~df.columns.duplicated()]
                logger.info(f"Total {len(df)} rows from all sheets")
            else:
                df = pd.DataFrame()
            
            df_unique = df.drop_duplicates(subset=['产品名称'], keep='first').reset_index(drop=True)
            logger.info(f"After deduplication: {len(df_unique)} unique products (from {len(df)} total rows)")
            
            success_count = 0
            failed_count = 0
            errors = []
            
            with self._get_session() as session:
                for idx, row in df_unique.iterrows():
                    sheet_name = row.get('_sheet_name', '')
                    excel_row_idx = int(row.get('_original_row', idx + 2))
                    try:
                        product_name = row.get('产品名称', '')
                        if pd.isna(product_name) or not product_name:
                            errors.append(f"Row {idx+2}: Missing product name")
                            failed_count += 1
                            continue
                        
                        check_sku = text("SELECT sku_code, category_id FROM sku WHERE name = :name LIMIT 1")
                        existing_sku = session.execute(check_sku, {"name": product_name}).fetchone()
                        
                        if existing_sku:
                            sku_code = existing_sku[0]
                            category_id = existing_sku[1]
                        else:
                            category_num = row.get('产品类目', row.get('产品分类', 4))
                            if pd.isna(category_num):
                                category_num = 4
                            category_id = f"{int(category_num):02d}"
                            sku_code = self._generate_sku_code(category_id)
                        
                        cost_price = row.get('成本单价/元', 0) or row.get('成本单价', 0) or row.get('成本价/箱', 0)
                        sale_price = row.get('单价/元', 0) or row.get('销售单价', 0) or row.get('报价/元', 0)
                        unit = row.get('单位', '个')
                        if pd.isna(unit):
                            unit = '个'
                        spec = row.get('箱规', '')
                        box_spec = row.get('箱规', '')
                        
                        if pd.isna(cost_price):
                            cost_price = 0
                        if pd.isna(sale_price):
                            sale_price = 0
                        if pd.isna(spec):
                            spec = ''
                        if pd.isna(box_spec):
                            box_spec = ''
                        
                        image_url = self._upload_image_from_excel(file_path, excel_row_idx, sku_code, sheet_name)
                        if image_url:
                            image_urls = f'["{image_url}"]'
                        else:
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
                        
                        if not existing_sku:
                            success_count += 1
                            logger.info(f"Processed row {idx+2}: {product_name} -> {sku_code}")
                        else:
                            logger.info(f"Skipped existing product: {product_name} -> {sku_code}")
                        
                    except Exception as e:
                        error_msg = f"Row {idx+2}: {str(e)}"
                        errors.append(error_msg)
                        failed_count += 1
                        logger.error(error_msg)
                
                session.commit()
            
            result = {
                "total_count": len(df_unique),
                "success_count": success_count,
                "failed_count": failed_count,
                "errors": errors[:10]
            }
            
            logger.info(f"Import completed: {result}")
            return result
            
        except Exception as e:
            logger.error(f"Import failed: {e}")
            raise

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
        excel_file = f"{EXCEL_DIR}/SKU.xlsx"
    
    importer = SKUImporter()
    importer.import_single_file(excel_file)
