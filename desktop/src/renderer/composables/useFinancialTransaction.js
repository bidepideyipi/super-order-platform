import { ref, computed } from 'vue';
import { ElMessage } from 'element-plus';

export function useFinancialTransaction() {
  const transactions = ref([]);
  const searchKeyword = ref('');
  const filterCategory = ref('');
  const dialogVisible = ref(false);
  const dialogMode = ref('add');
  
  const defaultForm = {
    id: null,
    category: '收入',
    description: '',
    amount_change: 0,
    balance: 0
  };
  
  const form = ref({ ...defaultForm });

  const currentBalance = computed(() => {
    if (transactions.value.length === 0) return 0;
    return transactions.value[0]?.balance || 0;
  });

  const totalIncome = computed(() => {
    return transactions.value
      .filter(t => t.category === '收入')
      .reduce((sum, t) => sum + t.amount_change, 0);
  });

  const totalExpense = computed(() => {
    return transactions.value
      .filter(t => t.category === '支出')
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
    loadData,
    handleSearch,
    handleAdd,
    handleEdit,
    handleSave,
    handleDelete,
    resetForm
  };
}
