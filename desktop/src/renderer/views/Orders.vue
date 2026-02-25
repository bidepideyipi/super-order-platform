<template>
  <div class="orders-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>订单管理</span>
          <el-button type="primary" @click="handleAdd">新增订单</el-button>
        </div>
      </template>
      
      <div class="toolbar">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索订单编号或客户编号"
          @input="handleSearch"
          style="width: 300px;"
        />
      </div>
      
      <el-table
        :data="paginatedOrders"
        border
        stripe
        style="width: 100%; margin-top: 20px;"
      >
        <el-table-column prop="order_no" label="订单编号" width="150" />
        <el-table-column prop="customer_id" label="客户编号" width="150" />
        <el-table-column prop="order_date" label="订单日期" width="120" />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="total_cost_amount" label="总成本" width="120" align="right">
          <template #default="{ row }">
            {{ row.total_cost_amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="total_sale_amount" label="总售价" min-width="120" align="right">
          <template #default="{ row }">
            {{ row.total_sale_amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleView(row)">查看</el-button>
            <el-button size="small" @click="handleEdit(row)">状态管理</el-button>
          </template>
        </el-table-column>
      </el-table>
      
      <div class="pagination-container">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>
    
    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'view' ? '查看订单' : (dialogMode === 'add' ? '新增订单' : '编辑订单')"
      width="600px"
    >
      <el-form :model="form" label-width="100px" :disabled="dialogMode === 'view'">
        <el-form-item label="订单编号">
          <el-input v-model="form.order_no" placeholder="自动生成" disabled />
        </el-form-item>
        <el-form-item label="客户编号">
          <el-select v-model="form.customer_id" placeholder="请选择客户" :disabled="dialogMode !== 'add'">
            <el-option
              v-for="customer in customers"
              :key="customer.customer_id"
              :label="customer.customer_name"
              :value="customer.customer_id"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="订单日期">
          <el-date-picker
            v-model="form.order_date"
            type="date"
            placeholder="选择日期"
            format="YYYY-MM-DD"
            value-format="YYYY-MM-DD"
            :disabled="dialogMode !== 'add'"
          />
        </el-form-item>
        <el-form-item label="状态">
          <el-select v-model="form.status" placeholder="请选择状态" :disabled="dialogMode === 'view'">
            <el-option
              v-for="option in statusOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="备注">
          <el-input v-model="form.remarks" type="textarea" :rows="3" placeholder="请输入备注" :disabled="dialogMode === 'view'" />
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="handleSave" v-if="dialogMode !== 'view'">保存</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useOrderList } from '../composables/useOrderList';
import { useOrderForm } from '../composables/useOrderForm';
import { useOrderStatus } from '../composables/useOrderStatus';

const {
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
} = useOrderList();

const {
  dialogVisible,
  dialogMode,
  form,
  openAddDialog,
  openEditDialog,
  openViewDialog,
  handleSave: saveForm
} = useOrderForm();

const {
  getStatusType,
  getStatusText,
  statusOptions
} = useOrderStatus();

const customers = ref([]);

const paginatedOrders = computed(() => {
  return getPaginatedOrders();
});

const loadCustomers = async () => {
  try {
    customers.value = await window.tauriAPI.customer.list();
  } catch (error) {
    console.error('加载客户列表失败:', error);
  }
};

const handleAdd = () => {
  openAddDialog();
};

const handleView = (row) => {
  openViewDialog(row);
};

const handleEdit = (row) => {
  openEditDialog(row);
};

const handleSave = async () => {
  await saveForm(() => {
    loadData();
  });
};

onMounted(async () => {
  await loadCustomers();
  await loadData();
});
</script>

<style scoped>
.orders-page {
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

.pagination-container {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}
</style>
