<template>
  <div class="purchase-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>采购管理</span>
          <div class="header-actions">
            <el-button type="primary" @click="handleAdd" :disabled="!selectedOrderId">新增明细</el-button>
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
      
      <el-table
        :data="orderItems"
        border
        stripe
        style="width: 100%; margin-top: 20px;"
        v-loading="loading"
      >
        <el-table-column label="商品信息" min-width="250">
          <template #default="{ row }">
            <div style="display: flex; align-items: center; gap: 10px;">
              <img v-if="row.sku_code" :src="getImageUrl(row.sku_code)" style="width: 32px; height: 32px; object-fit: cover;" />
              <span v-else>-</span>
              <div>
                <div>{{ row.product_name }}</div>
                <div style="color: #999; font-size: 12px;">({{ row.sku_code }})</div>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="单位/箱规" width="120">
          <template #default="{ row }">
            {{ row.unit }} / {{ row.box_spec || '无' }}
          </template>
        </el-table-column>
        <el-table-column prop="quantity" label="数量" width="100" align="right" />
        <el-table-column prop="cost_price" label="成本价" width="100" align="right">
          <template #default="{ row }">
            {{ row.cost_price.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="sale_price" label="销售价" width="100" align="right">
          <template #default="{ row }">
            {{ row.sale_price.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="total_cost_amount" label="总成本" width="100" align="right">
          <template #default="{ row }">
            {{ row.total_cost_amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="total_sale_amount" label="总售价" width="100" align="right">
          <template #default="{ row }">
            {{ row.total_sale_amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right" align="center">
          <template #default="{ row }">
            <el-button size="small" :icon="Edit" @click="handleEdit(row)" />
            <el-button size="small" type="danger" :icon="Delete" @click="handleDelete(row.id)" />
          </template>
        </el-table-column>
      </el-table>
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
        <el-form-item label="数量">
          <el-input-number v-model="form.quantity" :min="1" />
        </el-form-item>
        <el-form-item label="成本价">
          <el-input-number v-model="form.cost_price" :precision="2" :min="0" />
        </el-form-item>
        <el-form-item label="销售价">
          <el-input-number v-model="form.sale_price" :precision="2" :min="0" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ElMessageBox } from 'element-plus';
import { usePurchaseList } from '../composables/usePurchaseList';
import { usePurchaseForm } from '../composables/usePurchaseForm';
import { Search, Plus, Edit, Delete } from '@element-plus/icons-vue';

const {
  processingOrders,
  selectedOrderId,
  orderItems,
  loading,
  imageUrls,
  getImageUrl,
  loadProcessingOrders,
  handleOrderChange,
  refreshOrderItems
} = usePurchaseList();

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
}

.sku-option {
  display: flex;
  align-items: center;
  width: 100%;
}
</style>
