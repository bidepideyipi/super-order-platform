from minio import Minio
from minio.error import S3Error
from pathlib import Path
import logging
from config import *

logger = logging.getLogger(__name__)


class MinIOClient:
    def __init__(self):
        self.client = Minio(
            MINIO_ENDPOINT,
            MINIO_ACCESS_KEY,
            MINIO_SECRET_KEY,
            secure=MINIO_SECURE
        )
        self.bucket_name = MINIO_BUCKET_NAME
        self._ensure_bucket_exists()

    def _ensure_bucket_exists(self):
        try:
            if not self.client.bucket_exists(self.bucket_name):
                self.client.make_bucket(self.bucket_name)
                logger.info(f"Created bucket: {self.bucket_name}")
        except S3Error as e:
            logger.error(f"Error creating bucket: {e}")
            raise

    def upload_file(self, file_path: str, object_name: str = None):
        if object_name is None:
            object_name = Path(file_path).name

        try:
            self.client.fput_object(
                self.bucket_name,
                object_name,
                file_path,
            )
            logger.info(f"Uploaded {file_path} as {object_name}")
            return f"{MINIO_ENDPOINT}/{self.bucket_name}/{object_name}"
        except S3Error as e:
            logger.error(f"Error uploading file: {e}")
            raise

    def upload_from_memory(self, data: bytes, object_name: str):
        try:
            self.client.put_object(
                self.bucket_name,
                object_name,
                data,
                length=len(data),
            )
            logger.info(f"Uploaded data as {object_name}")
            return f"{MINIO_ENDPOINT}/{self.bucket_name}/{object_name}"
        except S3Error as e:
            logger.error(f"Error uploading from memory: {e}")
            raise

    def list_files(self, prefix: str = ""):
        try:
            objects = self.client.list_objects(self.bucket_name, prefix=prefix)
            return [obj.object_name for obj in objects]
        except S3Error as e:
            logger.error(f"Error listing files: {e}")
            return []

    def get_file_url(self, object_name: str):
        return f"{MINIO_ENDPOINT}/{self.bucket_name}/{object_name}"
