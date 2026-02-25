import { ref } from 'vue';
import { ElMessage } from 'element-plus';
import { useSKUImage } from './useSKUImage';

export function usePurchaseList() {
  const processingOrders = ref([]);
  const selectedOrderId = ref(null);
  const orderItems = ref([]);
  const loading = ref(false);

  const { imageUrls, getImageUrl, loadImageUrls } = useSKUImage();

  const loadProcessingOrders = async () => {
    try {
      const result = await window.tauriAPI.purchase.getProcessingOrders();
      processingOrders.value = result;
      return result;
    } catch (error) {
      console.error('加载采购中订单失败:', error);
      ElMessage.error('加载采购中订单失败');
      throw error;
    }
  };

  const loadOrderItems = async (orderId) => {
    if (!orderId) {
      orderItems.value = [];
      return [];
    }

    loading.value = true;
    try {
      const result = await window.tauriAPI.purchase.getOrderItems(String(orderId));
      orderItems.value = result;
      await loadImageUrls(result);
      return result;
    } catch (error) {
      console.error('加载订单明细失败:', error);
      ElMessage.error('加载订单明细失败');
      orderItems.value = [];
      throw error;
    } finally {
      loading.value = false;
    }
  };

  const handleOrderChange = async (orderId) => {
    selectedOrderId.value = orderId;
    return loadOrderItems(orderId);
  };

  const refreshOrderItems = () => {
    if (selectedOrderId.value) {
      return loadOrderItems(selectedOrderId.value);
    }
  };

  return {
    processingOrders,
    selectedOrderId,
    orderItems,
    loading,
    imageUrls,
    getImageUrl,
    loadProcessingOrders,
    loadOrderItems,
    handleOrderChange,
    refreshOrderItems
  };
}
