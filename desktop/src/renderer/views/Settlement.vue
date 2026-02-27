<template>
  <div class="settlement-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>结算管理</span>
        </div>
      </template>
      
      <div class="toolbar">
        <el-select
          v-model="selectedUnsettledOrderId"
          placeholder="请选择未结算订单"
          @change="handleOrderChangeWithImages"
          style="width: 250px;"
          clearable
        >
          <el-option
            v-for="order in unsettledOrders"
            :key="order.id"
            :label="`${order.order_no} (${order.customer_name})`"
            :value="order.id"
          />
        </el-select>
        
        <el-button 
          v-if="selectedUnsettledOrderId && currentOrder" 
          type="success" 
          @click="markAsSettled"
          :disabled="currentOrder.is_settled === 1"
        >
          {{ currentOrder.is_settled === 1 ? '已结算' : '标记为已结算' }}
        </el-button>
      </div>
      
      <div v-if="currentOrder" class="order-info">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="订单编号">{{ currentOrder.order_no }}</el-descriptions-item>
          <el-descriptions-item label="客户名称">{{ currentOrder.customer_name }}</el-descriptions-item>
          <el-descriptions-item label="订单日期">{{ currentOrder.order_date }}</el-descriptions-item>
          <el-descriptions-item label="结算状态">
            <el-tag :type="currentOrder.is_settled === 1 ? 'success' : 'warning'">
              {{ currentOrder.is_settled === 1 ? '已结算' : '未结算' }}
            </el-tag>
          </el-descriptions-item>
          <el-descriptions-item label="总成本金额">¥{{ currentOrder.total_cost_amount?.toFixed(2) || '0.00' }}</el-descriptions-item>
          <el-descriptions-item label="总销售金额">¥{{ currentOrder.total_sale_amount?.toFixed(2) || '0.00' }}</el-descriptions-item>
          <el-descriptions-item label="备注" :span="2">{{ currentOrder.remarks || '-' }}</el-descriptions-item>
        </el-descriptions>
      </div>
      
      <el-table
        v-if="orderItems.length > 0"
        :data="orderItems"
        border
        stripe
        style="width: 100%; margin-top: 20px;"
      >
        <el-table-column label="商品信息" min-width="250">
          <template #default="{ row }">
            <div style="display: flex; align-items: center; gap: 10px;">
              <img v-if="row.sku_code" :src="getImageUrl(row.sku_code)" style="width: 32px; height: 32px; object-fit: cover;" />
              <span v-else>-</span>
              <div>
                <div>{{ row.product_name }}</div>
                <div style="color: #999; font-size: 12px;">{{ row.sku_code }}</div>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="每(单位)N件" width="200">
          <template #default="{ row }">
            <div v-if="row.box_quantity > 1">每{{ row.unit }}{{ row.box_spec }}</div>
            <div v-else>{{ row.unit }}</div>
          </template>
        </el-table-column>
        <el-table-column label="数量" width="100" align="right">
          <template #default="{ row }">
            <div>{{ row.quantity }}{{ row.unit }}</div>
          </template>
        </el-table-column>
        <el-table-column label="成本价/销售价" width="120" align="right">
          <template #default="{ row }">
            <div style="color: #67C23A;">¥{{ row.cost_price.toFixed(2) }}</div>
            <div style="color: #409EFF;">¥{{ row.sale_price.toFixed(2) }}</div>
          </template>
        </el-table-column>
        <el-table-column label="总成本/总售价" width="120" align="right">
          <template #default="{ row }">
            <div style="color: #67C23A;">¥{{ row.total_cost_amount.toFixed(2) }}</div>
            <div style="color: #409EFF;">¥{{ row.total_sale_amount.toFixed(2) }}</div>
          </template>
        </el-table-column>
        <el-table-column label="利润" width="100" align="right">
          <template #default="{ row }">
            <div>¥{{ (row.total_sale_amount - row.total_cost_amount).toFixed(2) }}</div>
          </template>
        </el-table-column>
      </el-table>
      
      <el-empty v-else description="请选择订单查看明细" />
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useSettlementList } from '../composables/useSettlementList';
import { useSKUImage } from '../composables/useSKUImage';

const {
  unsettledOrders,
  currentOrder,
  orderItems,
  selectedUnsettledOrderId,
  loadUnsettledOrders,
  handleOrderChange,
  markAsSettled
} = useSettlementList();

const {
  getImageUrl,
  loadImageUrls
} = useSKUImage();

const handleOrderChangeWithImages = async (orderId) => {
  await handleOrderChange(orderId);
  if (orderItems.value.length > 0) {
    await loadImageUrls(orderItems.value);
  }
};

onMounted(() => {
  loadUnsettledOrders();
});
</script>

<style scoped>
.settlement-page {
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
  margin-bottom: 20px;
}

.order-info {
  margin-bottom: 20px;
}
</style>