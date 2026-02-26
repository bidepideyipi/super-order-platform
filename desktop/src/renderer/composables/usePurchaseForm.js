import { ref, watch } from 'vue';
import { ElMessage } from 'element-plus';

export function usePurchaseForm() {
  const dialogVisible = ref(false);
  const dialogMode = ref('add');
  const skuSearchKeyword = ref('');
  const skuOptions = ref([]);
  const selectedSku = ref(null);

  const defaultForm = {
    id: null,
    order_id: 0,
    sku_id: null,
    sku_code: '',
    product_name: '',
    quantity: 1,
    cost_price: 0,
    sale_price: 0,
    total_cost_amount: 0,
    total_sale_amount: 0,
    unit: '',
    box_spec: '',
    box_quantity: 1,
    sync_to_sku: true
  };

  const form = ref({ ...defaultForm });

  const calculateTotals = () => {
    form.value.total_cost_amount = form.value.quantity * form.value.cost_price * form.value.box_quantity;
    form.value.total_sale_amount = form.value.quantity * form.value.sale_price * form.value.box_quantity;
  };

  watch(() => [form.value.quantity, form.value.cost_price, form.value.sale_price, form.value.box_quantity], calculateTotals);

  const searchSku = async (queryString, cb) => {
    if (!queryString) {
      cb([]);
      return;
    }

    try {
      const results = await window.tauriAPI.purchase.searchSkuByCode(queryString);
      skuOptions.value = results;
      cb(results.map(item => ({ ...item, value: item.sku_code })));
    } catch (error) {
      console.error('搜索SKU失败:', error);
      cb([]);
    }
  };

  const handleSkuSelect = (item) => {
    selectedSku.value = item;
    form.value.sku_id = item.id;
    form.value.sku_code = item.sku_code;
    form.value.product_name = item.name;
    form.value.cost_price = item.cost_price;
    form.value.sale_price = item.sale_price;
    form.value.unit = item.unit;
    form.value.box_spec = item.box_spec;
    form.value.box_quantity = item.box_quantity;
    calculateTotals();
  };

  const handleSkuEnter = () => {
    if (skuOptions.value.length > 0) {
      handleSkuSelect(skuOptions.value[0]);
    }
  };

  const openAddDialog = () => {
    dialogMode.value = 'add';
    resetForm();
    dialogVisible.value = true;
  };

  const openEditDialog = async (row) => {
    dialogMode.value = 'edit';
    form.value = { ...row, sync_to_sku: true };
    skuSearchKeyword.value = row.sku_code;
    
    if (row.sku_id) {
      try {
        const sku = await window.tauriAPI.sku.get(String(row.sku_id));
        if (sku) {
          selectedSku.value = sku;
        }
      } catch (error) {
        console.error('加载SKU失败:', error);
      }
    }
    
    dialogVisible.value = true;
  };

  const saveOrderItem = async (orderId) => {
    if (!form.value.sku_code) {
      ElMessage.warning('请选择SKU');
      return false;
    }

    try {
      form.value.order_id = orderId;

      if (dialogMode.value === 'add') {
        await window.tauriAPI.purchase.createOrderItem(form.value);
        ElMessage.success('新增成功');
      } else {
        await window.tauriAPI.purchase.updateOrderItem(String(form.value.id), form.value);
        ElMessage.success('更新成功');
      }

      if (form.value.sync_to_sku && form.value.sku_id) {
        try {
          const skuData = {
            sku_code: form.value.sku_code,
            name: selectedSku.value?.name || form.value.product_name,
            category_id: selectedSku.value?.category_id || '01',
            unit: selectedSku.value?.unit || form.value.unit,
            spec: selectedSku.value?.spec,
            box_spec: selectedSku.value?.box_spec || form.value.box_spec,
            box_quantity: selectedSku.value?.box_quantity || form.value.box_quantity,
            cost_price: form.value.cost_price,
            sale_price: form.value.sale_price
          };
          await window.tauriAPI.sku.update(String(form.value.sku_id), skuData, null);
          ElMessage.success('已同步到SKU');
        } catch (error) {
          console.error('同步SKU失败:', error);
          ElMessage.warning('保存成功，但同步到SKU失败');
        }
      }

      dialogVisible.value = false;
      return true;
    } catch (error) {
      ElMessage.error(dialogMode.value === 'add' ? '新增失败' : '更新失败');
      console.error(error);
      return false;
    }
  };

  const deleteOrderItem = async (id) => {
    try {
      await window.tauriAPI.purchase.deleteOrderItem(String(id));
      ElMessage.success('删除成功');
      return true;
    } catch (error) {
      ElMessage.error('删除失败');
      console.error(error);
      return false;
    }
  };

  const resetForm = () => {
    form.value = { ...defaultForm };
    skuSearchKeyword.value = '';
    selectedSku.value = null;
  };

  return {
    dialogVisible,
    dialogMode,
    form,
    skuSearchKeyword,
    skuOptions,
    selectedSku,
    searchSku,
    handleSkuSelect,
    handleSkuEnter,
    openAddDialog,
    openEditDialog,
    saveOrderItem,
    deleteOrderItem,
    resetForm
  };
}
