<template>
  <div class="sku-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>SKU 管理</span>
          <div class="header-actions">
            <el-button type="primary" @click="handleAdd">新增 SKU</el-button>
            <el-button @click="handleImport">导入</el-button>
            <el-button @click="handleExport">导出</el-button>
          </div>
        </div>
      </template>
      
      <div class="toolbar">
        <el-input
          v-model="searchKeyword"
          placeholder="搜索 SKU 编号或产品名称"
          @input="handleSearch"
          style="width: 300px;"
        >
          <template #prefix>
            <el-icon><Search /></el-icon>
          </template>
        </el-input>
        
        <el-select v-model="filterCategory" placeholder="选择分类" clearable style="width: 150px;">
          <el-option
            v-for="cat in categories"
            :key="cat.category_id"
            :label="cat.category_name"
            :value="cat.category_id"
          />
        </el-select>
      </div>
      
      <el-table
        :data="filteredSKUs"
        border
        stripe
        style="width: 100%; margin-top: 20px;"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="sku_code" label="SKU编号" width="100" />
        <el-table-column prop="name" label="产品名称" min-width="200" />
        <el-table-column prop="category_name" label="分类" width="100" />
        <el-table-column prop="unit" label="单位" width="80" />
        <el-table-column prop="box_spec" label="箱规" width="100" />
        <el-table-column prop="cost_price" label="成本价" width="100" align="right">
          <template #default="{ row }">
            {{ row.cost_price.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="sale_price" label="销售价" width="100" align="right">
          <template #default="{ row }">
            {{ row.sale_price.toFixed(2) }}
          </template>
        </el-table-column>
        <el-table-column prop="image_path" label="图片" width="80">
          <template #default="{ row }">
            <el-image
              v-if="row.image_path"
              :src="getImageUrl(row.image_path)"
              :preview-src-list="[getImageUrl(row.image_path)]"
              fit="cover"
              style="width: 40px; height: 40px;"
            />
          </template>
        </el-table-column>
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row.id)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
    
    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'add' ? '新增 SKU' : '编辑 SKU'"
      width="600px"
    >
      <el-form :model="form" label-width="100px">
        <el-form-item label="产品名称">
          <el-input v-model="form.name" placeholder="请输入产品名称" />
        </el-form-item>
        <el-form-item label="分类">
          <el-select v-model="form.category_id" placeholder="请选择分类">
            <el-option
              v-for="cat in categories"
              :key="cat.category_id"
              :label="cat.category_name"
              :value="cat.category_id"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="单位">
          <el-input v-model="form.unit" placeholder="请输入单位" />
        </el-form-item>
        <el-form-item label="箱规">
          <el-input v-model="form.box_spec" placeholder="请输入箱规" />
        </el-form-item>
        <el-form-item label="成本价">
          <el-input-number v-model="form.cost_price" :precision="2" :min="0" />
        </el-form-item>
        <el-form-item label="销售价">
          <el-input-number v-model="form.sale_price" :precision="2" :min="0" />
        </el-form-item>
        <el-form-item label="规格">
          <el-input v-model="form.spec" type="textarea" placeholder="请输入规格" />
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
import { ref, onMounted, computed } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Search } from '@element-plus/icons-vue';

const skus = ref([]);
const categories = ref([]);
const searchKeyword = ref('');
const filterCategory = ref('');
const selectedRows = ref([]);
const dialogVisible = ref(false);
const dialogMode = ref('add');
const form = ref({
  name: '',
  category_id: '',
  unit: '个',
  box_spec: '',
  cost_price: 0,
  sale_price: 0,
  spec: ''
});

const filteredSKUs = computed(() => {
  let result = skus.value;
  
  if (filterCategory.value) {
    result = result.filter(sku => sku.category_id === filterCategory.value);
  }
  
  if (searchKeyword.value) {
    const keyword = searchKeyword.value.toLowerCase();
    result = result.filter(sku => 
      sku.sku_code.toLowerCase().includes(keyword) || 
      sku.name.toLowerCase().includes(keyword)
    );
  }
  
  return result;
});

const getImageUrl = (path) => {
  if (!path) return '';
  return path.startsWith('data/') ? `file://${process.cwd()}/${path}` : path;
};

const loadData = async () => {
  try {
    const [skuList, categoryList] = await Promise.all([
      window.electronAPI.sku.list(),
      window.electronAPI.category.list()
    ]);
    skus.value = skuList;
    categories.value = categoryList;
  } catch (error) {
    ElMessage.error('加载数据失败');
    console.error(error);
  }
};

const handleSearch = () => {
  
};

const handleAdd = () => {
  dialogMode.value = 'add';
  form.value = {
    name: '',
    category_id: '',
    unit: '个',
    box_spec: '',
    cost_price: 0,
    sale_price: 0,
    spec: ''
  };
  dialogVisible.value = true;
};

const handleEdit = (row) => {
  dialogMode.value = 'edit';
  form.value = { ...row };
  dialogVisible.value = true;
};

const handleDelete = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除该 SKU 吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    });
    
    await window.electronAPI.sku.delete(id);
    ElMessage.success('删除成功');
    loadData();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
      console.error(error);
    }
  }
};

const handleSave = async () => {
  try {
    if (dialogMode.value === 'add') {
      await window.electronAPI.sku.create(form.value);
      ElMessage.success('新增成功');
    } else {
      await window.electronAPI.sku.update(form.value.id, form.value);
      ElMessage.success('更新成功');
    }
    dialogVisible.value = false;
    loadData();
  } catch (error) {
    ElMessage.error('保存失败');
    console.error(error);
  }
};

const handleSelectionChange = (selection) => {
  selectedRows.value = selection;
};

const handleImport = async () => {
  try {
    const filePaths = await window.electronAPI.openFile();
    if (filePaths && filePaths.length > 0) {
      ElMessage.success('导入功能开发中');
    }
  } catch (error) {
    console.error(error);
  }
};

const handleExport = async () => {
  try {
    const filePath = await window.electronAPI.saveFile('sku_export.xlsx');
    if (filePath) {
      ElMessage.success('导出功能开发中');
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.sku-page {
  padding: 20px;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-actions {
  display: flex;
  gap: 10px;
}

.toolbar {
  display: flex;
  gap: 10px;
}
</style>
