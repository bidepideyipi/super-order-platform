import { ref } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';

/**
 * 结算管理列表功能
 */
export function useSettlementList() {
  const unsettledOrders = ref([]);
  const allCustomers = ref([]);
  const currentOrder = ref(null);
  const orderItems = ref([]);
  const selectedUnsettledOrderId = ref('');

  /**
   * 加载客户列表
   */
  const loadCustomers = async () => {
    try {
      const customers = await window.tauriAPI.customer.list();
      allCustomers.value = customers;
    } catch (error) {
      console.error('加载客户列表失败:', error);
      ElMessage.error('加载客户列表失败');
    }
  };

  /**
   * 根据客户ID获取客户名称
   */
  const getCustomerNameById = (customerId) => {
    const customer = allCustomers.value.find(c => c.customer_id === customerId);
    return customer ? customer.customer_name : customerId;
  };

  /**
   * 加载未结算订单列表
   */
  const loadUnsettledOrders = async () => {
    try {
      const orders = await window.tauriAPI.purchase.getUnsettledOrders();
      
      const ordersWithCustomerNames = orders.map(order => ({
        ...order,
        customer_name: getCustomerNameById(order.customer_id)
      }));
      
      unsettledOrders.value = ordersWithCustomerNames;
    } catch (error) {
      console.error('加载未结算订单失败:', error);
      ElMessage.error('加载未结算订单失败');
    }
  };

  loadCustomers();

  /**
   * 处理订单选择变化
   */
  const handleOrderChange = async (orderId) => {
    if (!orderId) {
      currentOrder.value = null;
      orderItems.value = [];
      return;
    }

    try {
      const order = await window.tauriAPI.order.get(String(orderId));
      
      const orderWithCustomerName = {
        ...order,
        customer_name: getCustomerNameById(order.customer_id)
      };
      
      currentOrder.value = orderWithCustomerName;

      const items = await window.tauriAPI.purchase.getOrderItems(String(orderId));
      orderItems.value = items;
    } catch (error) {
      console.error('加载订单详情失败:', error);
      ElMessage.error('加载订单详情失败');
    }
  };

  /**
   * 标记订单为已结算
   */
  const markAsSettled = async () => {
    if (!selectedUnsettledOrderId.value) {
      ElMessage.warning('请选择订单');
      return;
    }

    try {
      await ElMessageBox.confirm('确定要将此订单标记为已结算吗？', '确认操作', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      });

      const updateData = {
        ...currentOrder.value,
        is_settled: 1
      };

      await window.tauriAPI.order.update(String(selectedUnsettledOrderId.value), updateData);

      currentOrder.value.is_settled = 1;
      
      await loadUnsettledOrders();
      
      ElMessage.success('订单已标记为已结算');
    } catch (error) {
      if (error !== 'cancel') {
        console.error('标记结算失败:', error);
        ElMessage.error('标记结算失败');
      }
    }
  };

  return {
    unsettledOrders,
    currentOrder,
    orderItems,
    selectedUnsettledOrderId,
    loadUnsettledOrders,
    handleOrderChange,
    markAsSettled
  };
}