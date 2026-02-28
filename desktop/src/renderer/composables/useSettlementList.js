import { ref } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT, FINANCIAL_CATEGORY_PROFIT_SETTLEMENT } from './useFinancialTransaction';

/**
 * 结算管理列表功能
 */
export function useSettlementList() {
  const processingOrders = ref([]);
  const allCustomers = ref([]);
  const currentOrder = ref(null);
  const orderItems = ref([]);
  const selectedUnsettledOrderId = ref('');
  const latestBalance = ref(0);

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
   * 加载采购中订单列表
   */
  const loadUnsettledOrders = async () => {
    try {
      const orders = await window.tauriAPI.purchase.getProcessingOrders();
      
      const ordersWithCustomerNames = orders.map(order => ({
        ...order,
        customer_name: getCustomerNameById(order.customer_id)
      }));
      
      processingOrders.value = ordersWithCustomerNames;
    } catch (error) {
      console.error('加载采购中订单失败:', error);
      ElMessage.error('加载采购中订单失败');
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
      
      // 加载最新余额
      await loadLatestBalance();
    } catch (error) {
      console.error('加载订单详情失败:', error);
      ElMessage.error('加载订单详情失败');
    }
  };

  /**
   * 加载最新余额
   */
  const loadLatestBalance = async () => {
    try {
      const balance = await window.tauriAPI.financial.getBalance();
      latestBalance.value = balance;
    } catch (error) {
      console.error('加载待结余金额失败:', error);
      ElMessage.error('加载待结余金额失败');
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
      await ElMessageBox.confirm('结算后订单将锁定编辑并进行结余，请确认列表展示内容无误后再点确定按键？', '确认操作', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      });

      // 计算并更新每个订单明细的已结算金额
      let currentBalance = latestBalance.value;
      for (const item of orderItems.value) {
        currentBalance -= item.total_cost_amount;
        const updateData = {
          ...item,
          settled_amount: currentBalance
        };
        await window.tauriAPI.purchase.updateOrderItem(String(item.id), updateData);
      }

      // 计算总成本和利润
      const totalCostAmount = currentOrder.value.total_cost_amount || 0;
      const totalSaleAmount = currentOrder.value.total_sale_amount || 0;
      const profit = totalSaleAmount - totalCostAmount;
      const profitSettlement = profit / 2;

      // 计算新余额
      const costBalance = latestBalance.value + (totalCostAmount * -1);
      const finalBalance = costBalance + (profitSettlement * -1);

      // 创建财务交易记录
      // 1. 成本价结算记录
      await window.tauriAPI.financial.create({
        category: FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT,
        description: `采购单${currentOrder.value.order_no}成本结算`,
        amount_change: totalCostAmount * -1,
        balance: costBalance,
        is_settled: true
      });

      // 2. 利润结算记录
      await window.tauriAPI.financial.create({
        category: FINANCIAL_CATEGORY_PROFIT_SETTLEMENT,
        description: `采购单${currentOrder.value.order_no}利润结算`,
        amount_change: profitSettlement * -1,
        balance: finalBalance,
        is_settled: true
      });

      // 更新订单状态为已结算
      const updateData = {
        ...currentOrder.value,
        is_settled: true
      };

      await window.tauriAPI.order.update(String(selectedUnsettledOrderId.value), updateData);

      currentOrder.value.is_settled = true;
      
      await loadUnsettledOrders();
      await loadLatestBalance();
      
      ElMessage.success('订单已标记为已结算');
    } catch (error) {
      if (error !== 'cancel') {
        console.error('标记结算失败:', error);
        ElMessage.error('标记结算失败');
      }
    }
  };

  return {
    processingOrders,
    currentOrder,
    orderItems,
    selectedUnsettledOrderId,
    latestBalance,
    loadUnsettledOrders,
    handleOrderChange,
    markAsSettled,
    loadLatestBalance
  };
}
