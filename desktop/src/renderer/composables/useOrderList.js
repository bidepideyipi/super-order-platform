import { ref } from 'vue';
import { ElMessage } from 'element-plus';

export function useOrderList() {
  const orders = ref([]);
  const searchKeyword = ref('');
  const currentPage = ref(1);
  const pageSize = ref(10);
  const total = ref(0);

  const loadData = async () => {
    try {
      console.log('开始加载订单数据');
      orders.value = await window.tauriAPI.order.list();
      total.value = orders.value.length;
      console.log('订单数据加载完成:', orders.value.length, '个订单');
      return orders.value;
    } catch (error) {
      console.error('加载订单失败:', error);
      ElMessage.error('加载订单失败: ' + (error.message || error));
      throw error;
    }
  };

  const handleSearch = async () => {
    if (searchKeyword.value) {
      const keyword = searchKeyword.value.toLowerCase();
      orders.value = orders.value.filter(order => 
        order.order_no.toLowerCase().includes(keyword) ||
        order.customer_id.toLowerCase().includes(keyword)
      );
      total.value = orders.value.length;
    } else {
      await loadData();
    }
  };

  const handlePageChange = async (page) => {
    console.log('页码变化:', page);
    currentPage.value = page;
  };

  const handleSizeChange = async (size) => {
    console.log('每页条数变化:', size);
    pageSize.value = size;
    currentPage.value = 1;
  };

  const getPaginatedOrders = () => {
    const start = (currentPage.value - 1) * pageSize.value;
    const end = start + pageSize.value;
    return orders.value.slice(start, end);
  };

  return {
    orders,
    searchKeyword,
    currentPage,
    pageSize,
    total,
    loadData,
    handleSearch,
    handlePageChange,
    handleSizeChange,
    getPaginatedOrders
  };
}
