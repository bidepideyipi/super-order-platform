<template>
  <div class="home">
    <el-row :gutter="20">
      <el-col :span="6">
        <el-card class="stat-card">
          <el-statistic title="总 SKU 数量" :value="stats.totalSKU" />
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <el-statistic title="总订单数" :value="stats.totalOrders" />
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <el-statistic title="客户数量" :value="stats.totalCustomers" />
        </el-card>
      </el-col>
      <el-col :span="6">
        <el-card class="stat-card">
          <el-statistic title="分类数量" :value="stats.totalCategories" />
        </el-card>
      </el-col>
    </el-row>
    
    <el-row :gutter="20" style="margin-top: 20px;">
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>分类统计</span>
          </template>
          <el-table :data="categoryStats" style="width: 100%">
            <el-table-column prop="category_id" label="分类ID" width="100" />
            <el-table-column prop="category_name" label="分类名称" />
            <el-table-column prop="count" label="SKU数量" width="120" />
          </el-table>
        </el-card>
      </el-col>
      <el-col :span="12">
        <el-card>
          <template #header>
            <span>欢迎使用</span>
          </template>
          <div class="welcome-content">
            <h3>超级订单管理系统</h3>
            <p>版本 1.0.0</p>
            <p>桌面版应用</p>
            <el-divider />
            <p>快速开始：</p>
            <ul>
              <li><router-link to="/sku">管理 SKU</router-link></li>
              <li><router-link to="/orders">管理订单</router-link></li>
              <li><router-link to="/customers">管理客户</router-link></li>
            </ul>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const stats = ref({
  totalSKU: 0,
  totalOrders: 0,
  totalCustomers: 0,
  totalCategories: 0
});

const categoryStats = ref([]);

onMounted(async () => {
  try {
    const skus = await window.tauriAPI.sku.list();
    const categories = await window.tauriAPI.category.list();
    const customers = await window.tauriAPI.customer.list();
    
    stats.value.totalSKU = skus.length;
    stats.value.totalCategories = categories.length;
    stats.value.totalCustomers = customers.length;
    
    const categoryMap = {};
    categories.forEach(cat => {
      categoryMap[cat.category_id] = {
        category_id: cat.category_id,
        category_name: cat.category_name,
        count: 0
      };
    });
    
    skus.forEach(sku => {
      if (categoryMap[sku.category_id]) {
        categoryMap[sku.category_id].count++;
      }
    });
    
    categoryStats.value = Object.values(categoryMap);
  } catch (error) {
    console.error('Failed to load stats:', error);
  }
});
</script>

<style scoped>
.home {
  padding: 20px;
}

.stat-card {
  text-align: center;
}

.welcome-content {
  padding: 20px;
}

.welcome-content h3 {
  margin-bottom: 10px;
  color: #409eff;
}

.welcome-content ul {
  list-style: none;
  padding: 0;
}

.welcome-content li {
  margin: 10px 0;
}

.welcome-content a {
  color: #409eff;
  text-decoration: none;
}

.welcome-content a:hover {
  text-decoration: underline;
}
</style>
