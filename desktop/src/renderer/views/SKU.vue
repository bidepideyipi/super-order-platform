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
        style="width: 100%; margin-top: 20px;"
        @selection-change="handleSelectionChange"
      >
        <el-table-column type="selection" width="55" />
        <el-table-column prop="sku_code" label="SKU编号" width="100" />
        <el-table-column label="产品图片" width="100">
          <template #default="{ row }">
            <img v-if="row.image_path" :src="getImageUrl(row.image_path)" style="width: 60px; height: 60px; object-fit: cover;" />
            <span v-else>-</span>
          </template>
        </el-table-column>
        <el-table-column prop="name" label="产品名称" min-width="200" />
        <el-table-column prop="category_name" label="分类" width="80" />
        <el-table-column prop="unit" label="单位" width="80" />
        <el-table-column prop="box_spec" label="箱规" width="160" />
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
        <el-table-column label="操作" width="150" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="handleEdit(row)">编辑</el-button>
            <el-button size="small" type="danger" @click="handleDelete(row.id)">删除</el-button>
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
              <img v-if="form.image_path" :src="getImageUrl(form.image_path)" class="image-preview" />
              <el-icon v-else class="image-uploader-icon"><Plus /></el-icon>
            </el-upload>
            <el-button v-if="form.image_path" size="small" type="danger" @click="handleRemoveImage" style="margin-left: 10px;">
              删除图片
            </el-button>
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
          <el-input v-model="form.unit" placeholder="请输入单位" />
        </el-form-item>
        <el-form-item label="规格">
          <el-input v-model="form.spec" placeholder="请输入规格" />
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
import { Search, Plus } from '@element-plus/icons-vue';
import { readBinaryFile, writeBinaryFile, BaseDirectory, createDir } from '@tauri-apps/api/fs';

const skus = ref([]);
const categories = ref([]);
const searchKeyword = ref('');
const selectedRows = ref([]);
const dialogVisible = ref(false);
const dialogMode = ref('add');
const currentPage = ref(1);
const pageSize = ref(10);
const total = ref(0);
const form = ref({
  name: '',
  category_id: '',
  unit: '个',
  box_spec: '',
  cost_price: 0,
  sale_price: 0,
  spec: '',
  image_path: ''
});

const isSearching = computed(() => {
  return !!searchKeyword.value;
});

const filteredSKUs = computed(() => {
  return skus.value;
});

const loadImageUrls = (skuList) => {
  return skuList.map(sku => {
    if (sku.image_path && sku.image_path.startsWith('data/')) {
      const imagePath = sku.image_path.replace('data/', '/images/');
      const imageUrl = `.${imagePath}`;
      console.log(`Converted ${sku.image_path} -> ${imageUrl}`);
      return {
        ...sku,
        image_url: imageUrl
      };
    }
    return {
      ...sku,
      image_url: sku.image_path
    };
  });
};

const loadData = async () => {
  try {
    console.log('开始加载 SKU 数据，页码:', currentPage.value, '每页:', pageSize.value);
    const [result, categoryList] = await Promise.all([
      window.electronAPI.sku.listPaginated(currentPage.value, pageSize.value),
      window.electronAPI.category.list()
    ]);
    console.log('SKU 数据加载完成:', result.data.length, '个 SKU');
    console.log('总记录数:', result.total, '总页数:', result.total_pages);
    console.log('分类数据加载完成:', categoryList.length, '个分类');
    
    skus.value = await loadImageUrls(result.data);
    total.value = result.total;
    categories.value = categoryList;
  } catch (error) {
    console.error('加载数据失败:', error);
    ElMessage.error('加载数据失败: ' + (error.message || error));
  }
};

const handleSearch = async () => {
  currentPage.value = 1;
  
  if (searchKeyword.value) {
    try {
      const result = await window.electronAPI.sku.search(searchKeyword.value);
      skus.value = await loadImageUrls(result);
      total.value = result.length;
    } catch (error) {
      console.error('搜索失败:', error);
      ElMessage.error('搜索失败: ' + (error.message || error));
    }
  } else {
    loadData();
  }
};

const handlePageChange = (page) => {
  currentPage.value = page;
  handleSearch();
};

const handleSizeChange = (size) => {
  pageSize.value = size;
  currentPage.value = 1;
  handleSearch();
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
    spec: '',
    image_path: ''
  };
  dialogVisible.value = true;
};

const handleEdit = (row) => {
  dialogMode.value = 'edit';
  form.value = { ...row };
  dialogVisible.value = true;
};

const getImageUrl = (imagePath) => {
  if (!imagePath) return '';
  if (imagePath.startsWith('data/')) {
    const imagePathClean = imagePath.replace('data/', '/images/');
    return `.${imagePathClean}`;
  }
  return imagePath;
};

const handleImageChange = async (file) => {
  try {
    const reader = new FileReader();
    reader.onload = async (e) => {
      const imageData = e.target.result;
      const imageBytes = new Uint8Array(atob(imageData.split(',')[1]).split('').map(c => c.charCodeAt(0)));
      
      const fileName = `sku_${Date.now()}_${file.name}`;
      const imagePath = `data/${fileName}`;
      
      try {
        await createDir('data', { recursive: true, dir: BaseDirectory.Resource });
      } catch (error) {
        console.log('Data directory may already exist:', error);
      }
      
      await writeBinaryFile(imagePath, imageBytes, { dir: BaseDirectory.Resource });
      form.value.image_path = imagePath;
    };
    reader.readAsDataURL(file.raw);
  } catch (error) {
    console.error('Image upload error:', error);
    ElMessage.error('图片上传失败');
  }
};

const handleRemoveImage = () => {
  form.value.image_path = '';
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
    handleSearch();
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
    handleSearch();
  } catch (error) {
    ElMessage.error('保存失败');
    console.error(error);
  }
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
</style>
