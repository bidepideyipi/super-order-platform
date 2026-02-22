import os
from dotenv import load_dotenv

load_dotenv()

DATABASE_URL = os.getenv("DATABASE_URL", "mysql+pymysql://root:root123456@localhost:3307/super_order?charset=utf8mb4")
MINIO_ENDPOINT = os.getenv("MINIO_ENDPOINT", "localhost:9000")
MINIO_ACCESS_KEY = os.getenv("MINIO_ACCESS_KEY", "minioadmin")
MINIO_SECRET_KEY = os.getenv("MINIO_SECRET_KEY", "minioadmin")
MINIO_BUCKET_NAME = os.getenv("MINIO_BUCKET_NAME", "super-order")
MINIO_SECURE = os.getenv("MINIO_SECURE", "False").lower() == "true"

EXCEL_DIR = os.path.join(os.path.dirname(os.path.dirname(__file__)), "doc")
IMAGE_DIR = os.path.join(os.path.dirname(__file__), "images")
