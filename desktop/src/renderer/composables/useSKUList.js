import { ref } from 'vue';
import { ElMessage } from 'element-plus';

export function useSKUList() {
  const skus = ref([]);
  const categories = ref([]);
  const searchKeyword = ref('');
  const currentPage = ref(1);
  const pageSize = ref(10);
  const total = ref(0);

  const loadData = async () => {
    try {
      console.log('开始加载 SKU 数据，页码:', currentPage.value, '每页:', pageSize.value);
      const [result, categoryList] = await Promise.all([
        window.tauriAPI.sku.listPaginated(currentPage.value, pageSize.value),
        window.tauriAPI.category.list()
      ]);
      console.log('SKU 数据加载完成:', result.data.length, '个 SKU');
      console.log('总记录数:', result.total, '总页数:', result.total_pages);
      console.log('分类数据加载完成:', categoryList.length, '个分类');
      
      skus.value = result.data;
      total.value = result.total;
      categories.value = categoryList;
      return result.data;
    } catch (error) {
      console.error('加载数据失败:', error);
      ElMessage.error('加载数据失败: ' + (error.message || error));
      throw error;
    }
  };

  const handleSearch = async () => {
    currentPage.value = 1;
    if (searchKeyword.value) {
      try {
        const result = await window.tauriAPI.sku.searchPaginated(searchKeyword.value, currentPage.value, pageSize.value);
        skus.value = result.data;
        total.value = result.total;
        return result.data;
      } catch (error) {
        console.error('搜索失败:', error);
        ElMessage.error('搜索失败: ' + (error.message || error));
        throw error;
      }
    } else {
      return loadData();
    }
  };

  const handlePageChange = async (page) => {
    console.log('页码变化:', page);
    currentPage.value = page;
    return loadData();
  };

  const handleSizeChange = async (size) => {
    console.log('每页条数变化:', size);
    pageSize.value = size;
    currentPage.value = 1;
    return handleSearch();
  };

  return {
    skus,
    categories,
    searchKeyword,
    currentPage,
    pageSize,
    total,
    loadData,
    handleSearch,
    handlePageChange,
    handleSizeChange
  };
}
