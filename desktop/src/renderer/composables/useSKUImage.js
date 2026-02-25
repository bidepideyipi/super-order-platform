import { ref } from 'vue';
import { ElMessage } from 'element-plus';

export function useSKUImage() {
  const imageUrls = ref({});

  const getImageUrl = (skuCode) => {
    if (!skuCode) return '';
    return imageUrls.value[skuCode] || '';
  };

  const loadImageUrl = async (skuCode) => {
    if (!skuCode || imageUrls.value[skuCode]) {
      return;
    }
    try {
      const result = await window.tauriAPI.sku.getImage(skuCode);
      if (result) {
        imageUrls.value[skuCode] = result;
      }
    } catch (error) {
      console.error('Failed to load image for', skuCode, error);
    }
  };

  const loadImageUrls = async (skuList) => {
    const urls = {};
    for (const sku of skuList) {
      if (sku.sku_code) {
        try {
          const result = await window.tauriAPI.sku.getImage(sku.sku_code);
          if (result) {
            urls[sku.sku_code] = result;
          }
        } catch (error) {
          console.error('Failed to load image for', sku.sku_code, error);
        }
      }
    }
    imageUrls.value = urls;
    return skuList;
  };

  const handleImageChange = (file) => {
    return new Promise((resolve, reject) => {
      try {
        const reader = new FileReader();
        reader.onload = (e) => {
          const dataUrl = e.target.result;
          const base64Data = dataUrl.split(',')[1];
          resolve(base64Data);
        };
        reader.onerror = (error) => {
          console.error('Image upload error:', error);
          ElMessage.error('图片上传失败');
          reject(error);
        };
        reader.readAsDataURL(file.raw);
      } catch (error) {
        console.error('Image upload error:', error);
        ElMessage.error('图片上传失败');
        reject(error);
      }
    });
  };

  return {
    imageUrls,
    getImageUrl,
    loadImageUrl,
    loadImageUrls,
    handleImageChange
  };
}
