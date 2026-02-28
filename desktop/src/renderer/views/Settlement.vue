<template>
  <div class="settlement-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>结算管理</span>
          <div class="header-actions">
            <el-button @click="exportPDF" :disabled="!selectedUnsettledOrderId || orderItems.length === 0">导出PDF</el-button>
          </div>
        </div>
      </template>
      
      <div class="toolbar">
        <el-select
          v-model="selectedUnsettledOrderId"
          placeholder="请选择订单编号"
          @change="handleOrderChangeWithImages"
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
        
        <el-button 
          v-if="selectedUnsettledOrderId && currentOrder" 
          type="success" 
          @click="markAsSettled"
          :disabled="currentOrder.is_settled || currentOrder.total_cost_amount <= 0"
        >结算</el-button>
      </div>
      
      <div v-if="currentOrder" class="order-info">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="订单编号">{{ currentOrder.order_no }}</el-descriptions-item>
          <el-descriptions-item label="客户名称">{{ currentOrder.customer_name }}</el-descriptions-item>
          <el-descriptions-item label="订单日期">{{ currentOrder.order_date }}</el-descriptions-item>
          <el-descriptions-item label="结算状态">
            <el-tag :type="currentOrder.is_settled ? 'success' : 'warning'">
              {{ currentOrder.is_settled ? '已结算' : '未结算' }}
            </el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </div>
      
      <div class="table-container">
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
        <el-table-column label="产品规格" width="120">
          <template #default="{ row }">
            <div v-if="row.box_quantity > 1">{{row.spec}}*{{ row.box_spec }}/{{ row.unit }}</div>
            <div v-else>{{row.spec}}/{{ row.unit }}</div>
          </template>
        </el-table-column>
        <el-table-column label="数量" width="100" align="right">
          <template #default="{ row }">
            <div>{{ row.quantity }}{{ row.unit }}</div>
          </template>
        </el-table-column>
        <el-table-column label="单价" width="120" align="right">
          <template #default="{ row }">
            <div style="color: #67C23A;">¥{{ row.cost_price.toFixed(2) }}</div>
            <div style="color: #409EFF;">¥{{ row.sale_price.toFixed(2) }}</div>
          </template>
        </el-table-column>
        <el-table-column label="总价" width="120" align="right">
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
        <el-table-column label="结余" width="120" align="right">
          <template #default="{ row, $index }">
            <div>¥{{ calculateBalance($index) }}</div>
          </template>
        </el-table-column>
      </el-table>
        
        <el-empty v-else description="请先到采购管理界面新增明细吧！" style="margin-top: 20px;" />
      </div>

      <div v-if="currentOrder" class="remarks-section">
        <el-card shadow="hover" style="margin-bottom: 20px; margin-top: 20px;">
          <template #header>
            <div class="card-header">
              <span>备注信息</span>
            </div>
          </template>
          <div class="remarks-content" v-html="currentOrder.remarks || '-' "></div>
        </el-card>
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useSettlementList } from '../composables/useSettlementList';
import { useSKUImage } from '../composables/useSKUImage';
import { useSettlementExport } from '../composables/useSettlementExport';

const {
  processingOrders,
  currentOrder,
  orderItems,
  selectedUnsettledOrderId,
  latestBalance,
  loadUnsettledOrders,
  handleOrderChange,
  markAsSettled,
  loadLatestBalance
} = useSettlementList();

const {
  imageUrls,
  getImageUrl,
  loadImageUrls
} = useSKUImage();

const { exportPDF } = useSettlementExport({
  processingOrders,
  selectedOrderId: selectedUnsettledOrderId,
  orderItems,
  imageUrls,
  latestBalance
});

const calculateBalance = (index) => {
  let balance = latestBalance.value;
  for (let i = 0; i <= index; i++) {
    balance -= orderItems.value[i].total_cost_amount;
  }
  return balance.toFixed(2);
};

const handleOrderChangeWithImages = async (orderId) => {
  await handleOrderChange(orderId);
  if (orderItems.value.length > 0) {
    await loadImageUrls(orderItems.value);
  }
};

onMounted(async () => {
  await loadUnsettledOrders();
  await loadLatestBalance();
});
</script>

<style scoped>
.settlement-page {
  padding: 20px;
  height: calc(100vh - 40px);
  overflow-y: auto;
}

.settlement-page::-webkit-scrollbar {
  width: 8px;
}

.settlement-page::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.settlement-page::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.settlement-page::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-actions {
  display: flex;
  gap: 10px;
}

.toolbar {
  display: flex;
  gap: 10px;
  margin-bottom: 20px;
}

.order-info {
  margin-bottom: 20px;
}

.remarks-section {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.table-container {
  margin-top: 20px;
}

.remarks-content {
  min-height: 100px;
  padding: 10px;
  white-space: pre-wrap;
  color: #87898f;
}

</style>
