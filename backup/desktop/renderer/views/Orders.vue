<template>
  <div class="orders-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>订单管理</span>
          <el-button type="primary" @click="handleAdd">新增订单</el-button>
        </div>
      </template>
      
      <el-table
        :data="orders"
        border
        stripe
        style="width: 100%"
      >
        <el-table-column prop="order_no" label="订单编号" width="150" />
        <el-table-column prop="customer_name" label="客户名称" width="150" />
        <el-table-column prop="order_date" label="订单日期" width="120" />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="{ row }">
            <el-tag :type="getStatusType(row.status)">
              {{ getStatusText(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="total_amount" label="总金额" width="120" align="right">
          <template #default="{ row }">
            {{ row.total_amount.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="remarks" label="备注" min-width="200" />
        <el-table-column label="操作" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleView(row)">查看</el-button>
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row.id)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage } from 'element-plus';

const orders = ref([]);

const getStatusType = (status) => {
  const types = {
    'pending': 'warning',
    'processing': 'primary',
    'completed': 'success',
    'cancelled': 'danger'
  };
  return types[status] || 'info';
};

const getStatusText = (status) => {
  const texts = {
    'pending': '待处理',
    'processing': '处理中',
    'completed': '已完成',
    'cancelled': '已取消'
  };
  return texts[status] || status;
};

const loadData = async () => {
  try {
    orders.value = await window.tauriAPI.order.list();
  } catch (error) {
    ElMessage.error('加载订单失败');
    console.error(error);
  }
};

const handleAdd = () => {
  ElMessage.info('新增订单功能开发中');
};

const handleView = (row) => {
  ElMessage.info('查看订单详情功能开发中');
};

const handleEdit = (row) => {
  ElMessage.info('编辑订单功能开发中');
};

const handleDelete = async (id) => {
  ElMessage.info('删除订单功能开发中');
};

onMounted(() => {
  loadData();
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
</style>
