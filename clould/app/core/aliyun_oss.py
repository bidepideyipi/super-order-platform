import logging
import alibabacloud_oss_v2 as oss

logger = logging.getLogger(__name__)

class OSS:
    def __init__(self):
        # 从环境变量中加载凭证信息，用于身份验证
        credentials_provider = oss.credentials.EnvironmentVariableCredentialsProvider()

        # 加载SDK的默认配置，并设置凭证提供者
        cfg = oss.config.load_default()
        cfg.credentials_provider = credentials_provider

        # 方式一：只填写Region（推荐）
        # 必须指定Region ID，SDK会根据Region自动构造HTTPS访问域名
        cfg.region = settings.OSS_REGION

        # 使用配置好的信息创建OSS客户端
        self.client = oss.Client(cfg)
        self.bucket = settings.OSS_BUCKET_NAME

    def upload(self, key, data):
        """
        上传文件到OSS
        
        阿里云 OSS 上传如果 key 重复， 默认会覆盖旧文件
        
        :param key: 文件在OSS中的路径（对象名称）
        :param data: 文件内容（字节流）
        :return: None
        """
        result = self.client.put_object(oss.PutObjectRequest(
            bucket=self.bucket,
            key=key,
            body=data,
        ))
        
        # 输出请求的结果状态码、请求ID、ETag，用于检查请求是否成功
        logger.info(f'upload completed - status code: {result.status_code}, '
                   f'request id: {result.request_id}, '
                   f'etag: {result.etag}')
    
    def get_url(self, key, expires=3600):
        """
        获取文件的访问URL地址
        
        :param key: 文件在OSS中的路径（对象名称）
        :param expires: URL过期时间，单位秒，默认1小时
        :return: 文件访问URL地址
        """
        try:
            result = self.client.sign_url(oss.SignUrlRequest(
                method='GET',
                bucket=self.bucket,
                key=key,
                expires=int(expires),
            ))
            logger.info(f'get url completed - key: {key}, url: {result.url}')
            return result.url
        except Exception as e:
            logger.error(f'get url failed - key: {key}, error: {str(e)}')
            return None