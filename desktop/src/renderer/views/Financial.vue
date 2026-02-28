<template>
  <div class="financial-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>财务收支</span>
          <el-button type="primary" @click="handleAdd">新增收支</el-button>
        </div>
      </template>
      
      <div class="toolbar">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索说明"
          @input="handleSearch"
          style="width: 300px;"
        />
        <el-select v-model="filterCategory" placeholder="筛选分类" @change="handleSearch" style="width: 150px;">
          <el-option label="全部" value="" />
          <el-option :label="FINANCIAL_CATEGORY_ADVANCE_PAYMENT" :value="FINANCIAL_CATEGORY_ADVANCE_PAYMENT" />
          <el-option :label="FINANCIAL_CATEGORY_PROFIT_SETTLEMENT" :value="FINANCIAL_CATEGORY_PROFIT_SETTLEMENT" />
          <el-option :label="FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT" :value="FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT" />
          <el-option :label="FINANCIAL_CATEGORY_REVERSAL" :value="FINANCIAL_CATEGORY_REVERSAL" />
        </el-select>
      </div>
      
      <div class="summary">
        <el-statistic title="当前结余" :value="currentBalance" :precision="2" />
        <el-statistic title="总采购支出" :value="totalExpense" :precision="2" />
        <el-statistic title="总利润" :value="totalProfit" :precision="2" />
        <el-statistic title="总冲正" :value="totalReversal" :precision="2" />
        <el-statistic title="总预收货款" :value="totalIncome" :precision="2" />
      </div>
      
      <el-table
        :data="filteredTransactions"
        border
        stripe
        style="width: 100%; margin-top: 20px;"
      >
        <el-table-column prop="category" label="分类" width="100">
          <template #default="{ row }">
            <el-tag :type="getCategoryType(row.category)">
              {{ row.category }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="description" label="说明" min-width="200" />
        <el-table-column prop="amount_change" label="金额变化" width="120" align="right">
          <template #default="{ row }">
            <span :style="{ color: row.amount_change >= 0 ? '#67c23a' : '#f56c6c' }">
              {{ row.amount_change >= 0 ? '+' : '' }}{{ row.amount_change.toFixed(2) }}
            </span>
          </template>
        </el-table-column>
        <el-table-column prop="balance" label="结余" width="120" align="right">
          <template #default="{ row }">
            {{ row.balance.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="created_at" label="创建时间" width="180" />
        <el-table-column label="操作" width="200" fixed="right" align="center">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row.id)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
    
    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'add' ? '新增收支' : '编辑收支'"
      width="500px"
    >
      <el-form :model="form" label-width="100px">
        <el-form-item label="分类" required>
          <el-select v-model="form.category" placeholder="请选择分类">
            <el-option :label="FINANCIAL_CATEGORY_ADVANCE_PAYMENT" :value="FINANCIAL_CATEGORY_ADVANCE_PAYMENT" />
            <el-option :label="FINANCIAL_CATEGORY_PROFIT_SETTLEMENT" :value="FINANCIAL_CATEGORY_PROFIT_SETTLEMENT" />
            <el-option :label="FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT" :value="FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT" />
            <el-option :label="FINANCIAL_CATEGORY_REVERSAL" :value="FINANCIAL_CATEGORY_REVERSAL" />
          </el-select>
        </el-form-item>
        <el-form-item label="说明">
          <el-input v-model="form.description" type="textarea" :rows="3" placeholder="请输入说明" />
        </el-form-item>
        <el-form-item label="金额" required>
          <el-input-number v-model="form.amount_change" :precision="2" :min="1" :max="999999999" />
        </el-form-item>
        <el-form-item label="结算状态">
          <el-switch v-model="form.is_settled" active-text="已结算" inactive-text="未结算" :disabled="true" />
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
import { ref, computed, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useFinancialTransaction } from '../composables/useFinancialTransaction';

const {
  transactions,
  searchKeyword,
  filterCategory,
  dialogVisible,
  dialogMode,
  form,
  currentBalance,
  totalIncome,
  totalExpense,
  totalProfit,
  totalReversal,
  loadData,
  handleSearch,
  handleAdd,
  handleEdit,
  handleSave,
  handleDelete,
  FINANCIAL_CATEGORY_ADVANCE_PAYMENT,
  FINANCIAL_CATEGORY_PROFIT_SETTLEMENT,
  FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT,
  FINANCIAL_CATEGORY_REVERSAL
} = useFinancialTransaction();

const filteredTransactions = computed(() => {
  let result = transactions.value;
  
  if (filterCategory.value) {
    result = result.filter(t => t.category === filterCategory.value);
  }
  
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase();
    result = result.filter(t => 
      (t.description && t.description.toLowerCase().includes(keyword))
    );
  }
  
  return result;
});

const getCategoryType = (category) => {
  switch (category) {
    case FINANCIAL_CATEGORY_ADVANCE_PAYMENT:
      return 'success';
    case FINANCIAL_CATEGORY_PROFIT_SETTLEMENT:
      return 'warning';
    case FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT:
      return 'danger';
    case FINANCIAL_CATEGORY_REVERSAL:
      return 'info';
    default:
      return 'info';
  }
};

onMounted(async () => {
  await loadData();
});
</script>

<style scoped>
.financial-page {
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
  margin-top: 20px;
}

.summary {
  display: flex;
  gap: 20px;
  margin-top: 20px;
  padding: 20px;
  background-color: #f5f7fa;
  border-radius: 4px;
  flex-wrap: wrap;
}

@media (max-width: 768px) {
  .summary {
    flex-direction: column;
    gap: 10px;
  }
}
</style>
