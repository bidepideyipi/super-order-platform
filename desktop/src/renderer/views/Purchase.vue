<template>
  <div class="purchase-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>采购管理</span>
        </div>
      </template>
      
      <div class="toolbar">
        <el-select
          v-model="selectedOrderId"
          placeholder="请选择订单编号"
          @change="handleOrderChange"
          style="width: 200px;"
          clearable
        >
          <el-option
            v-for="order in processingOrders"
            :key="order.id"
            :label="`${order.order_no}`"
            :value="order.id"
          />
        </el-select>
      </div>
      
      <el-table
        :data="orderItems"
        border
        stripe
        style="width: 100%; margin-top: 20px;"
        v-loading="loading"
      >
        <el-table-column prop="sku_code" label="SKU编码" width="150" />
        <el-table-column prop="product_name" label="商品名称" min-width="200" />
        <el-table-column prop="quantity" label="数量" width="100" align="right" />
        <el-table-column prop="unit_price" label="单价" width="120" align="right">
          <template #default="{ row }">
            {{ row.unit_price.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="subtotal" label="小计" width="120" align="right">
          <template #default="{ row }">
            {{ row.subtotal.toFixed(2) }}
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage } from 'element-plus';

const processingOrders = ref([]);
const selectedOrderId = ref(null);
const orderItems = ref([]);
const loading = ref(false);

const loadProcessingOrders = async () => {
  try {
    processingOrders.value = await window.tauriAPI.purchase.getProcessingOrders();
  } catch (error) {
    console.error('加载采购中订单失败:', error);
    ElMessage.error('加载采购中订单失败');
  }
};

const handleOrderChange = async () => {
  if (!selectedOrderId.value) {
    orderItems.value = [];
    return;
  }

  loading.value = true;
  try {
    orderItems.value = await window.tauriAPI.purchase.getOrderItems(String(selectedOrderId.value));
  } catch (error) {
    console.error('加载订单明细失败:', error);
    ElMessage.error('加载订单明细失败');
    orderItems.value = [];
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadProcessingOrders();
});
</script>

<style scoped>
.purchase-page {
  padding: 20px;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toolbar {
  display: flex;
  gap: 10px;
}
</style>
