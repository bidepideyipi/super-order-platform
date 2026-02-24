<template>
  <div class="test-container">
    <h1>Tauri API 测试页面</h1>
    
    <div id="test-results">
      <div v-for="(result, index) in results" :key="index" :class="['test-result', result.type]">
        <div v-html="result.message"></div>
      </div>
    </div>
    
    <div class="test-buttons">
      <el-button @click="testEnvironment" type="primary">测试环境</el-button>
      <el-button @click="testGetVersion">测试版本</el-button>
      <el-button @click="testSkuList">测试 SKU 列表</el-button>
      <el-button @click="testCategoryList">测试分类列表</el-button>
      <el-button @click="testCustomerList">测试客户列表</el-button>
      <el-button @click="testOrderList">测试订单列表</el-button>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const results = ref([]);

function log(message, type = 'info') {
  results.value.push({ message, type });
}

function testEnvironment() {
  log(`<strong>环境测试：</strong><br>
       window.__TAURI__: ${!!window.__TAURI__}<br>
       window.__TAURI__?.core: ${!!window.__TAURI__?.core}<br>
       window.__TAURI__?.core?.invoke: ${!!window.__TAURI__?.core?.invoke}`, 'info');
}

async function testGetVersion() {
  try {
    if (!window.__TAURI__?.core?.invoke) {
      log('Tauri API 不可用', 'error');
      return;
    }
    const version = await window.__TAURI__.core.invoke('get_version');
    log(`<strong>版本测试成功：</strong> ${version}`, 'success');
  } catch (error) {
    log(`<strong>版本测试失败：</strong> ${error.message}`, 'error');
  }
}

async function testSkuList() {
  try {
    if (!window.__TAURI__?.core?.invoke) {
      log('Tauri API 不可用', 'error');
      return;
    }
    const skus = await window.__TAURI__.core.invoke('sku_list');
    log(`<strong>SKU 列表测试成功：</strong> 获取到 ${skus.length} 个 SKU`, 'success');
    if (skus.length > 0) {
      log(`<strong>第一个 SKU：</strong> ${JSON.stringify(skus[0])}`, 'info');
    }
  } catch (error) {
    log(`<strong>SKU 列表测试失败：</strong> ${error.message}`, 'error');
  }
}

async function testCategoryList() {
  try {
    if (!window.__TAURI__?.core?.invoke) {
      log('Tauri API 不可用', 'error');
      return;
    }
    const categories = await window.__TAURI__.core.invoke('category_list');
    log(`<strong>分类列表测试成功：</strong> 获取到 ${categories.length} 个分类`, 'success');
  } catch (error) {
    log(`<strong>分类列表测试失败：</strong> ${error.message}`, 'error');
  }
}

async function testCustomerList() {
  try {
    if (!window.__TAURI__?.core?.invoke) {
      log('Tauri API 不可用', 'error');
      return;
    }
    const customers = await window.__TAURI__.core.invoke('customer_list');
    log(`<strong>客户列表测试成功：</strong> 获取到 ${customers.length} 个客户`, 'success');
  } catch (error) {
    log(`<strong>客户列表测试失败：</strong> ${error.message}`, 'error');
  }
}

async function testOrderList() {
  try {
    if (!window.__TAURI__?.core?.invoke) {
      log('Tauri API 不可用', 'error');
      return;
    }
    const orders = await window.__TAURI__.core.invoke('order_list');
    log(`<strong>订单列表测试成功：</strong> 获取到 ${orders.length} 个订单`, 'success');
  } catch (error) {
    log(`<strong>订单列表测试失败：</strong> ${error.message}`, 'error');
  }
}

onMounted(() => {
  log('页面加载完成，点击按钮开始测试', 'info');
  testEnvironment();
});
</script>

<style scoped>
.test-container {
  padding: 20px;
}

#test-results {
  margin: 20px 0;
}

.test-result {
  margin: 10px 0;
  padding: 10px;
  border-radius: 5px;
  border: 1px solid;
}

.test-result.success {
  background-color: #d4edda;
  border-color: #c3e6cb;
  color: #155724;
}

.test-result.error {
  background-color: #f8d7da;
  border-color: #f5c6cb;
  color: #721c24;
}

.test-result.info {
  background-color: #d1ecf1;
  border-color: #bee5eb;
  color: #0c5460;
}

.test-buttons {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}
</style>