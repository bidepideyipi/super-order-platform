<template>
  <div class="purchase-page">
    <el-card>
      <template #header>
        <div class="header-content">
            <span>采购管理</span>
            <div class="header-actions">
              <el-button @click="exportPDF" :disabled="!selectedOrderId || orderItems.length === 0">导出PDF</el-button>
              <el-button type="primary" @click="handleAdd" :disabled="!selectedOrderId || currentOrder?.is_settled">新增明细</el-button>
            </div>
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
      
      <div v-if="currentOrder" class="order-info">
        <el-descriptions :column="2" border>
          <el-descriptions-item label="订单编号">{{ currentOrder.order_no }}</el-descriptions-item>
          <el-descriptions-item label="客户名称">{{ currentOrder.customer_name }}</el-descriptions-item>
          <el-descriptions-item label="订单日期">{{ currentOrder.order_date }}</el-descriptions-item>
          <el-descriptions-item label="总金额">¥{{ currentOrder.total_sale_amount?.toFixed(2) || '0.00' }}&nbsp;<el-tag :type="currentOrder.is_settled ? 'success' : 'warning'">
              {{ currentOrder.is_settled ? '已结算' : '未结算' }}
            </el-tag></el-descriptions-item>
        </el-descriptions>
      </div>
      
      <div class="table-container">
        <template v-if="orderItems.length > 0">
          <el-table
            :data="orderItems"
            border
            stripe
            style="width: 100%;"
            v-loading="loading"
            max-height="600"
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
             <el-table-column label="数量" width="80" align="right">
              <template #default="{ row }">
                <div>{{ row.quantity}}{{ row.unit }}</div>
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
            <el-table-column label="利润" width="120" align="right">
              <template #default="{ row }">
                <div>¥{{ (row.total_sale_amount - row.total_cost_amount).toFixed(2) }}</div>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="200" fixed="right" align="center">
              <template #default="{ row }">
                <el-button size="small" :icon="Edit" @click="handleEdit(row)" :disabled="currentOrder?.is_settled" />
                <el-button size="small" type="danger" :icon="Delete" @click="handleDelete(row.id)" :disabled="currentOrder?.is_settled" />
              </template>
            </el-table-column>
          </el-table>
        </template>
        <el-empty v-else description="老板点右上角新增明细吧！" style="margin-top: 20px;" />
      </div>

      <div v-if="currentOrder" class="remarks-section">
        <el-card shadow="hover" style="margin-bottom: 20px; margin-top: 20px;">
          <template #header>
            <div class="card-header">
              <span>备注</span>
            </div>
          </template>
          <div class="remarks-content" v-html="currentOrder.remarks || '-' "></div>
        </el-card>
      </div>
    </el-card>
    
    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'add' ? '新增明细' : '编辑明细'"
      width="600px"
    >
      <el-form :model="form" label-width="100px">
        <el-form-item label="SKU编码">
          <el-autocomplete
            v-model="skuSearchKeyword"
            :fetch-suggestions="searchSku"
            placeholder="请输入SKU编码"
            @select="handleSkuSelect"
            @keyup.enter="handleSkuEnter"
            clearable
          >
            <template #default="{ item }">
              <div class="sku-option">
                <span>{{ item.sku_code }}</span>
                <span style="margin-left: 10px; color: #999;">{{ item.name }}</span>
              </div>
            </template>
          </el-autocomplete>
        </el-form-item>
        <el-form-item label="商品名称">
          <el-input v-model="form.product_name" disabled />
        </el-form-item>
        <el-form-item label="单位">
          <el-input v-model="form.unit" disabled />
        </el-form-item>
        <el-form-item label="规格">
          <el-input v-model="form.spec" disabled />
        </el-form-item>
        <el-form-item label="箱规">
          <el-input v-model="form.box_spec" disabled />
        </el-form-item>
        <el-form-item label="每箱数量">
          <el-input-number v-model="form.box_quantity" :min="1" disabled />
        </el-form-item>
        <el-form-item label="数量">
          <el-input-number v-model="form.quantity" :min="1" />
        </el-form-item>
        <el-form-item label="成本价">
          <el-input-number v-model="form.cost_price" :precision="2" :min="0" />
        </el-form-item>
        <el-form-item label="销售价">
          <el-input-number v-model="form.sale_price" :precision="2" :min="0" />
        </el-form-item>
        <el-form-item>
          <el-checkbox v-model="form.sync_to_sku">同步到SKU</el-checkbox>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave" :disabled="currentOrder?.is_settled">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ElMessageBox } from 'element-plus';
import { usePurchaseList } from '../composables/usePurchaseList';
import { usePurchaseForm } from '../composables/usePurchaseForm';
import { usePurchaseExport } from '../composables/usePurchaseExport';
import { Search, Plus, Edit, Delete } from '@element-plus/icons-vue';

const {
  processingOrders,
  selectedOrderId,
  orderItems,
  loading,
  currentOrder,
  imageUrls,
  getImageUrl,
  loadProcessingOrders,
  handleOrderChange,
  refreshOrderItems,
  refreshCurrentOrder
} = usePurchaseList();

const { exportPDF } = usePurchaseExport({
  processingOrders,
  selectedOrderId,
  orderItems,
  imageUrls
});

const {
  dialogVisible,
  dialogMode,
  form,
  skuSearchKeyword,
  searchSku,
  handleSkuSelect,
  handleSkuEnter,
  openAddDialog,
  openEditDialog,
  saveOrderItem,
  deleteOrderItem
} = usePurchaseForm();

const handleAdd = () => {
  openAddDialog();
};

const handleEdit = (row) => {
  openEditDialog(row);
};

const handleSave = async () => {
  const success = await saveOrderItem(selectedOrderId.value);
  if (success) {
    await refreshOrderItems();
    await refreshCurrentOrder();
  }
};

const handleDelete = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除该明细吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    });
    
    const success = await deleteOrderItem(id);
    if (success) {
      await refreshOrderItems();
      await refreshCurrentOrder();
    }
  } catch (error) {
    if (error !== 'cancel') {
      console.error(error);
    }
  }
};



loadProcessingOrders();
</script>

<style scoped>
.purchase-page {
  padding: 20px;
  height: calc(100vh - 40px);
  overflow-y: auto;
}

.purchase-page::-webkit-scrollbar {
  width: 8px;
}

.purchase-page::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.purchase-page::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

.purchase-page::-webkit-scrollbar-thumb:hover {
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

.sku-option {
  display: flex;
  align-items: center;
  width: 100%;
}

.remarks-content {
  min-height: 100px;
  padding: 10px;
  white-space: pre-wrap;
  color: #87898f;
}
</style>
