<template>
  <el-container class="app-container">
    <el-header class="app-header">
      <div class="header-left">
        <h1>超级订单管理系统</h1>
      </div>
      <div class="header-right">
        <el-button type="primary" @click="handleImport">导入数据</el-button>
        <el-button @click="handleExport">导出数据</el-button>
      </div>
    </el-header>
    
    <el-container>
      <el-aside width="200px" class="app-aside">
        <el-menu
          :default-active="activeMenu"
          router
          class="sidebar-menu"
        >
          <el-menu-item index="/">
            <el-icon><HomeFilled /></el-icon>
            <span>首页</span>
          </el-menu-item>
          <el-menu-item index="/sku">
            <el-icon><Goods /></el-icon>
            <span>SKU 管理</span>
          </el-menu-item>
          <el-menu-item index="/orders">
            <el-icon><Document /></el-icon>
            <span>订单管理</span>
          </el-menu-item>
          <el-menu-item index="/customers">
            <el-icon><User /></el-icon>
            <span>客户管理</span>
          </el-menu-item>
        </el-menu>
      </el-aside>
      
      <el-main class="app-main">
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { HomeFilled, Goods, Document, User } from '@element-plus/icons-vue';

const router = useRouter();
const route = useRoute();

const activeMenu = ref(route.path);

const handleImport = async () => {
  try {
    const filePaths = await window.electronAPI.openFile();
    if (filePaths && filePaths.length > 0) {
      console.log('Import file:', filePaths[0]);
    }
  } catch (error) {
    console.error('Import failed:', error);
  }
};

const handleExport = async () => {
  try {
    const filePath = await window.electronAPI.saveFile('export.xlsx');
    if (filePath) {
      console.log('Export to:', filePath);
    }
  } catch (error) {
    console.error('Export failed:', error);
  }
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

#app {
  height: 100vh;
  overflow: hidden;
}

.app-container {
  height: 100vh;
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #409eff;
  color: white;
  padding: 0 20px;
  border-bottom: 1px solid #e6e6e6;
}

.header-left h1 {
  font-size: 20px;
  font-weight: 500;
}

.header-right {
  display: flex;
  gap: 10px;
}

.app-aside {
  background-color: #f5f5f5;
  border-right: 1px solid #e6e6e6;
}

.sidebar-menu {
  border-right: none;
}

.app-main {
  padding: 20px;
  background-color: #f0f2f5;
}
</style>
