import { ref } from 'vue';
import { ElMessage } from 'element-plus';
import { useSKUImage } from './useSKUImage';

export function usePurchaseList() {
  const processingOrders = ref([]);
  const selectedOrderId = ref(null);
  const orderItems = ref([]);
  const loading = ref(false);
  const currentOrder = ref(null);
  const allCustomers = ref([]);

  const { imageUrls, getImageUrl, loadImageUrls } = useSKUImage();

  const loadCustomers = async () => {
    try {
      const customers = await window.tauriAPI.customer.list();
      allCustomers.value = customers;
    } catch (error) {
      console.error('加载客户列表失败:', error);
      ElMessage.error('加载客户列表失败');
    }
  };

  const getCustomerNameById = (customerId) => {
    const customer = allCustomers.value.find(c => c.customer_id === customerId);
    return customer ? customer.customer_name : customerId;
  };

  const loadProcessingOrders = async () => {
    try {
      const result = await window.tauriAPI.purchase.getProcessingOrders();
      const ordersWithCustomerNames = result.map(order => ({
        ...order,
        customer_name: getCustomerNameById(order.customer_id)
      }));
      processingOrders.value = ordersWithCustomerNames;
      return ordersWithCustomerNames;
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
    if (orderId) {
      const order = processingOrders.value.find(order => order.id === orderId) || null;
      if (order) {
        currentOrder.value = {
          ...order,
          customer_name: getCustomerNameById(order.customer_id)
        };
      } else {
        currentOrder.value = null;
      }
    } else {
      currentOrder.value = null;
    }
    return loadOrderItems(orderId);
  };

  const refreshOrderItems = () => {
    if (selectedOrderId.value) {
      return loadOrderItems(selectedOrderId.value);
    }
  };

  loadCustomers();

  return {
    processingOrders,
    selectedOrderId,
    orderItems,
    loading,
    currentOrder,
    imageUrls,
    getImageUrl,
    loadProcessingOrders,
    loadOrderItems,
    handleOrderChange,
    refreshOrderItems
  };
}
