import os
from dotenv import load_dotenv

load_dotenv()

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DATA_DIR = os.path.join(BASE_DIR, "data")
DATABASE_PATH = os.path.join(DATA_DIR, "super_order.db")
DESKTOP_DIR = os.path.join(BASE_DIR, "desktop")
PUBLIC_DIR = os.path.join(DESKTOP_DIR, "public")
PUBLIC_IMAGES_DIR = os.path.join(PUBLIC_DIR, "images")

DATABASE_URL = os.getenv("DATABASE_URL", f"sqlite:///{DATABASE_PATH}")
EXCEL_DIR = os.path.join(BASE_DIR, "doc")
IMAGE_DIR = os.path.join(PUBLIC_IMAGES_DIR, "sku")
