import { ref } from 'vue';
import { ElMessage } from 'element-plus';

export function useSKUForm() {
  const dialogVisible = ref(false);
  const dialogMode = ref('add');
  const form = ref({
    name: '',
    category_id: '',
    unit: '个',
    box_spec: '',
    cost_price: 0,
    sale_price: 0,
    spec: '',
    image_path: '',
    image_file: null
  });

  const resetForm = () => {
    form.value = {
      name: '',
      category_id: '',
      unit: '个',
      box_spec: '',
      cost_price: 0,
      sale_price: 0,
      spec: '',
      image_path: ''
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

  const handleSave = async (onSuccess) => {
    try {
      const imageBase64 = form.value.image_file || null;
      const skuData = {
        name: form.value.name,
        category_id: form.value.category_id,
        unit: form.value.unit,
        spec: form.value.spec,
        box_spec: form.value.box_spec,
        cost_price: form.value.cost_price,
        sale_price: form.value.sale_price
      };
      
      if (dialogMode.value === 'add') {
        await window.tauriAPI.sku.create(skuData, imageBase64);
        ElMessage.success('新增成功');
      } else {
        skuData.sku_code = form.value.sku_code;
        await window.tauriAPI.sku.update(String(form.value.id), skuData, imageBase64);
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
    handleSave
  };
}
