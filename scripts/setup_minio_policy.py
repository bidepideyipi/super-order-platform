from minio_client import MinIOClient
from config import *
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def setup_bucket_policy():
    try:
        client = MinIOClient()
        
        policy = {
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Sid": "PublicReadGetObject",
                    "Effect": "Allow",
                    "Principal": {
                        "AWS": ["*"]
                    },
                    "Action": [
                        "s3:GetObject"
                    ],
                    "Resource": [
                        f"arn:aws:s3:::{client.bucket_name}/*"
                    ]
                }
            ]
        }
        
        import json
        
        from minio.error import S3Error
        
        try:
            client.client.set_bucket_policy(
                client.bucket_name,
                json.dumps(policy)
            )
            logger.info(f"Bucket policy set to public read for: {client.bucket_name}")
        except S3Error as e:
            logger.error(f"Error setting bucket policy: {e}")
            logger.info("Policy might already be set, checking current policy...")
            try:
                current_policy = client.client.get_bucket_policy(client.bucket_name)
                logger.info(f"Current policy: {current_policy}")
            except S3Error:
                logger.info("No existing policy found")
        
        return True
        
    except Exception as e:
        logger.error(f"Failed to setup bucket policy: {e}")
        return False


if __name__ == "__main__":
    setup_bucket_policy()
