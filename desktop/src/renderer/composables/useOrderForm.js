import { ref } from 'vue';
import { ElMessage } from 'element-plus';

export function useOrderForm() {
  const dialogVisible = ref(false);
  const dialogMode = ref('add');
  const form = ref({
    order_no: '',
    customer_id: '',
    order_date: '',
    status: 'pending',
    total_cost_amount: 0,
    total_sale_amount: 0,
    remarks: ''
  });

  const resetForm = () => {
    form.value = {
      order_no: '',
      customer_id: '',
      order_date: new Date().toISOString().split('T')[0],
      status: 'pending',
      total_cost_amount: 0,
      total_sale_amount: 0,
      remarks: ''
    };
  };

  const openAddDialog = () => {
    dialogMode.value = 'add';
    resetForm();
    dialogVisible.value = true;
  };

  const openEditDialog = (row) => {
    dialogMode.value = 'edit';
    form.value = { ...row };
    dialogVisible.value = true;
  };

  const openViewDialog = (row) => {
    dialogMode.value = 'view';
    form.value = { ...row };
    dialogVisible.value = true;
  };

  const handleSave = async (onSuccess) => {
    try {
      const orderData = {
        order_no: form.value.order_no,
        customer_id: form.value.customer_id,
        order_date: form.value.order_date,
        status: form.value.status,
        total_cost_amount: form.value.total_cost_amount,
        total_sale_amount: form.value.total_sale_amount,
        remarks: form.value.remarks
      };
      
      if (dialogMode.value === 'add') {
        await window.tauriAPI.order.create(orderData);
        ElMessage.success('新增成功');
      } else {
        await window.tauriAPI.order.update(String(form.value.id), orderData);
        ElMessage.success('更新成功');
      }
      dialogVisible.value = false;
      onSuccess?.();
    } catch (error) {
      ElMessage.error('保存失败');
      console.error(error);
      throw error;
    }
  };

  return {
    dialogVisible,
    dialogMode,
    form,
    openAddDialog,
    openEditDialog,
    openViewDialog,
    handleSave
  };
}
