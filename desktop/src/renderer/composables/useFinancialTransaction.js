import { ref, computed } from 'vue';
import { ElMessage } from 'element-plus';

// Financial transaction category constants
export const FINANCIAL_CATEGORY_ADVANCE_PAYMENT = '预收货款';
export const FINANCIAL_CATEGORY_PROFIT_SETTLEMENT = '利润结算';
export const FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT = '采购结算';
export const FINANCIAL_CATEGORY_REVERSAL = '冲正';

export function useFinancialTransaction() {
  const transactions = ref([]);
  const searchKeyword = ref('');
  const filterCategory = ref('');
  const dialogVisible = ref(false);
  const dialogMode = ref('add');
  
  const defaultForm = {
    id: null,
    category: FINANCIAL_CATEGORY_ADVANCE_PAYMENT,
    description: '',
    amount_change: 0,
    balance: 0,
    is_settled: false
  };
  
  const form = ref({ ...defaultForm });

  const currentBalance = computed(() => {
    if (transactions.value.length === 0) return 0;
    return transactions.value[0]?.balance || 0;
  });

  const totalIncome = computed(() => {
    return transactions.value
      .filter(t => t.category === FINANCIAL_CATEGORY_ADVANCE_PAYMENT)
      .reduce((sum, t) => sum + t.amount_change, 0);
  });

  const totalExpense = computed(() => {
    return transactions.value
      .filter(t => t.category === FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT)
      .reduce((sum, t) => sum + Math.abs(t.amount_change), 0);
  });

  const totalProfit = computed(() => {
    return transactions.value
      .filter(t => t.category === FINANCIAL_CATEGORY_PROFIT_SETTLEMENT)
      .reduce((sum, t) => sum + Math.abs(t.amount_change), 0);
  });

  const totalReversal = computed(() => {
    return transactions.value
      .filter(t => t.category === FINANCIAL_CATEGORY_REVERSAL)
      .reduce((sum, t) => sum + Math.abs(t.amount_change), 0);
  });

  const loadData = async () => {
    try {
      const result = await window.tauriAPI.financial.list();
      transactions.value = result;
    } catch (error) {
      console.error('加载财务收支失败:', error);
      ElMessage.error('加载财务收支失败');
    }
  };

  const handleSearch = () => {
  };

  const handleAdd = () => {
    dialogMode.value = 'add';
    resetForm();
    const balance = currentBalance.value;
    form.value.balance = balance;
    dialogVisible.value = true;
  };

  const handleEdit = (row) => {
    dialogMode.value = 'edit';
    form.value = { ...row };
    dialogVisible.value = true;
  };

  const handleSave = async () => {
    if (!form.value.category) {
      ElMessage.warning('请选择分类');
      return false;
    }

    try {
      // 对于冲正、利润结算、采购结算，金额变化为负值
      if (
        form.value.category === FINANCIAL_CATEGORY_REVERSAL ||
        form.value.category === FINANCIAL_CATEGORY_PROFIT_SETTLEMENT ||
        form.value.category === FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT
      ) {
        form.value.amount_change = Math.abs(form.value.amount_change) * -1;
      }

      const newBalance = dialogMode.value === 'add' 
        ? currentBalance.value + form.value.amount_change
        : form.value.balance;
      
      form.value.balance = newBalance;

      if (dialogMode.value === 'add') {
        await window.tauriAPI.financial.create(form.value);
        ElMessage.success('新增成功');
      } else {
        await window.tauriAPI.financial.update(String(form.value.id), form.value);
        ElMessage.success('更新成功');
      }

      dialogVisible.value = false;
      await loadData();
      return true;
    } catch (error) {
      ElMessage.error(dialogMode.value === 'add' ? '新增失败' : '更新失败');
      console.error(error);
      return false;
    }
  };

  const handleDelete = async (id) => {
    try {
      await ElMessageBox.confirm('确定要删除这条记录吗？', '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      });
      
      await window.tauriAPI.financial.delete(String(id));
      ElMessage.success('删除成功');
      await loadData();
    } catch (error) {
      if (error !== 'cancel') {
        ElMessage.error('删除失败');
        console.error(error);
      }
    }
  };

  const resetForm = () => {
    form.value = { ...defaultForm };
  };

  return {
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
    resetForm,
    FINANCIAL_CATEGORY_ADVANCE_PAYMENT,
    FINANCIAL_CATEGORY_PROFIT_SETTLEMENT,
    FINANCIAL_CATEGORY_PURCHASE_SETTLEMENT,
    FINANCIAL_CATEGORY_REVERSAL
  };
}
