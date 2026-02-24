<template>
  <div class="customers-page">
    <el-card>
      <template #header>
        <span>客户管理</span>
      </template>
      
      <el-table
        :data="customers"
        border
        stripe
        style="width: 100%"
      >
        <el-table-column prop="customer_id" label="客户ID" width="120" />
        <el-table-column prop="customer_name" label="客户名称" min-width="200" />
        <el-table-column label="操作" width="100" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { ElMessage } from 'element-plus';

const customers = ref([]);

const loadData = async () => {
  try {
    customers.value = await window.tauriAPI.customer.list();
  } catch (error) {
    ElMessage.error('加载客户失败');
    console.error(error);
  }
};

const handleEdit = (row) => {
  ElMessage.info('编辑客户功能开发中');
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.customers-page {
  padding: 20px;
}
</style>
