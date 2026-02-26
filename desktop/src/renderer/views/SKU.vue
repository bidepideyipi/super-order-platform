<template>
  <div class="sku-page">
    <el-card>
      <template #header>
        <div class="header-content">
          <span>SKU 管理</span>
          <div class="header-actions">
            <el-button type="primary" @click="handleAdd">新增 SKU</el-button>
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
      </div>
      
      <el-table
        :data="filteredSKUs"
        border
        stripe
        height="calc(100vh - 320px)"
        style="width: 100%; margin-top: 20px;"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="category_name" label="分类" width="100" />
        <el-table-column label="商品信息" min-width="250">
          <template #default="{ row }">
            <div style="display: flex; align-items: center; gap: 10px;">
              <img v-if="row.sku_code" :src="getImageUrl(row.sku_code)" style="width: 32px; height: 32px; object-fit: cover;" />
              <span v-else>-</span>
              <div>
                <div>{{ row.name }}</div>
                <div style="color: #999; font-size: 12px;">{{ row.sku_code }}</div>
              </div>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="每(单位)N件" width="200">
          <template #default="{ row }">
            <div v-if="row.box_quantity > 1">每{{ row.unit }}{{ row.box_spec }}</div>
            <div v-else>{{ row.unit }}</div>
          </template>
        </el-table-column>
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
        <el-table-column label="操作" width="200" fixed="right" align="center">
          <template #default="{ row }">
            <el-button size="small" :icon="Edit" @click="handleEdit(row)" />
            <el-button size="small" type="danger" :icon="Delete" @click="handleDelete(row.id)" />
          </template>
        </el-table-column>
      </el-table>
      
      <div class="pagination-container" v-if="!isSearching">
        <el-pagination
          v-model:current-page="currentPage"
          v-model:page-size="pageSize"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>
    
    <el-dialog
      v-model="dialogVisible"
      :title="dialogMode === 'add' ? '新增 SKU' : '编辑 SKU'"
      width="600px"
    >
      <el-form :model="form" label-width="100px">
        <el-form-item label="产品图片">
          <div class="image-upload-container">
            <el-upload
              :auto-upload="false"
              :show-file-list="false"
              accept="image/*"
              :on-change="handleImageChange"
              class="image-uploader"
            >
              <img v-if="form.sku_code" :src="getImageUrl(form.sku_code)" class="image-preview" />
              <el-icon v-else class="image-uploader-icon"><Plus /></el-icon>
            </el-upload>
          </div>
        </el-form-item>
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
          <el-input v-if="form.box_quantity <= 1" v-model="form.unit" placeholder="请输入单位" />
          <span v-else>箱</span>
        </el-form-item>
        <el-form-item label="规格">
          <el-input v-model="form.spec" placeholder="请输入规格" />
        </el-form-item>
        <el-form-item label="箱规">
          <el-input v-model="form.box_spec" placeholder="请输入箱规" />
        </el-form-item>
        <el-form-item label="每箱数量">
          <el-input-number v-model="form.box_quantity" :min="1" />
        </el-form-item>
        <el-form-item label="成本价">
          <el-input-number v-model="form.cost_price" :precision="2" :min="0" />
        </el-form-item>
        <el-form-item label="销售价">
          <el-input-number v-model="form.sale_price" :precision="2" :min="0" />
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
import { ref, computed, onMounted, watch } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Search, Plus, Edit, Delete } from '@element-plus/icons-vue';
import { useSKUList } from '../composables/useSKUList';
import { useSKUForm } from '../composables/useSKUForm';
import { useSKUImage } from '../composables/useSKUImage';

const {
  skus,
  categories,
  searchKeyword,
  currentPage,
  pageSize,
  total,
  loadData: loadDataFromList,
  handleSearch,
  handlePageChange,
  handleSizeChange
} = useSKUList();

const {
  dialogVisible,
  dialogMode,
  form,
  openAddDialog,
  openEditDialog,
  handleSave: saveForm
} = useSKUForm();

watch(() => form.value.box_quantity, (newVal) => {
  if (newVal > 1) {
    form.value.unit = '箱';
  }
});

const {
  imageUrls,
  getImageUrl,
  loadImageUrls,
  handleImageChange: handleImageUpload
} = useSKUImage();

const selectedRows = ref([]);

const isSearching = computed(() => {
  return !!searchKeyword.value;
});

const filteredSKUs = computed(() => {
  return skus.value;
});

const loadData = async () => {
  const data = await loadDataFromList();
  await loadImageUrls(data);
};

const handleSearchWithImages = async () => {
  const data = await handleSearch();
  await loadImageUrls(data);
};

const handlePageChangeWithImages = async (page) => {
  const data = await handlePageChange(page);
  await loadImageUrls(data);
};

const handleSizeChangeWithImages = async (size) => {
  const data = await handleSizeChange(size);
  await loadImageUrls(data);
};

const handleAdd = () => {
  openAddDialog();
};

const handleEdit = async (row) => {
  if (row.sku_code) {
    await loadImageUrls([row]);
  }
  openEditDialog(row);
};

const handleDelete = async (id) => {
  try {
    await ElMessageBox.confirm('确定要删除该 SKU 吗？', '提示', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    });
    
    await window.tauriAPI.sku.delete(String(id));
    ElMessage.success('删除成功');
    await handleSearchWithImages();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('删除失败');
      console.error(error);
    }
  }
};

const handleSave = async () => {
  await saveForm(() => handleSearchWithImages());
};

const handleImageChange = async (file) => {
  const base64Data = await handleImageUpload(file);
  form.value.image_file = base64Data;
};

const handleSelectionChange = (selection) => {
  selectedRows.value = selection;
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

.pagination-container {
  margin-top: 20px;
  display: flex;
  justify-content: flex-end;
}

.image-upload-container {
  display: flex;
  align-items: center;
}

.image-uploader {
  display: inline-block;
  border: 1px dashed #d9d9d9;
  border-radius: 6px;
  cursor: pointer;
  overflow: hidden;
  transition: border-color 0.3s;
}

.image-uploader:hover {
  border-color: #409eff;
}

.image-preview {
  width: 100px;
  height: 100px;
  object-fit: cover;
  display: block;
}

.image-uploader-icon {
  font-size: 28px;
  color: #8c939d;
  width: 100px;
  height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
}

:deep(.el-dialog__close) {
  position: absolute;
  right: 25px;
  top: 25px;
}
</style>
