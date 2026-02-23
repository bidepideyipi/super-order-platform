<template>
  <el-container class="app-container">
    <el-header class="app-header">
      <div class="header-left">
        <h1>超级订单管理系统</h1>
      </div>
    </el-header>
    
    <el-container>
      <el-aside width="200px" class="app-aside">
        <el-menu
          :default-active="activeMenu"
          @select="handleMenuSelect"
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
import { ref, watch } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { HomeFilled, Goods, Document, User } from '@element-plus/icons-vue';

const router = useRouter();
const route = useRoute();

const activeMenu = ref(route.path);

watch(() => route.path, (newPath) => {
  console.log('Route path changed to:', newPath, 'current activeMenu:', activeMenu.value);
  activeMenu.value = newPath;
}, { immediate: true });

const handleMenuSelect = (index) => {
  console.log('Menu selected:', index, 'current route:', route.path, 'current activeMenu:', activeMenu.value);
  activeMenu.value = index;
  router.push(index).catch(err => {
    console.error('Router push error:', err);
  });
};
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
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
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 0 24px;
  height: 60px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  position: relative;
  z-index: 10;
}

.header-left h1 {
  font-size: 22px;
  font-weight: 600;
  letter-spacing: 0.5px;
  background: linear-gradient(90deg, rgba(255,255,255,0.95), rgba(255,255,255,0.85));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.app-aside {
  background: #ffffff;
  border-right: 1px solid #e8eaec;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.04);
}

.sidebar-menu {
  border-right: none;
  padding: 8px 0;
}

.sidebar-menu .el-menu-item {
  margin: 4px 12px;
  border-radius: 8px;
  height: 44px;
  line-height: 44px;
  color: #606266;
  transition: all 0.3s ease;
}

.sidebar-menu .el-menu-item:hover {
  background-color: #f5f7fa;
  color: #667eea;
  transform: translateX(4px);
}

.sidebar-menu .el-menu-item.is-active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 2px 6px rgba(102, 126, 234, 0.3);
}

.sidebar-menu .el-menu-item .el-icon {
  margin-right: 8px;
  font-size: 18px;
}

.sidebar-menu .el-menu-item span {
  font-weight: 500;
  font-size: 14px;
}

.app-main {
  padding: 24px;
  background: linear-gradient(180deg, #f8f9fc 0%, #f0f2f5 100%);
  overflow-y: auto;
}

.el-card {
  border-radius: 12px;
  border: none;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
  transition: all 0.3s ease;
}

.el-card:hover {
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
}

.el-card .el-card__header {
  border-bottom: 1px solid #f0f0f0;
  padding: 18px 20px;
  background: #fafbfc;
}

.el-card .el-card__body {
  padding: 20px;
}

.el-button--primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  padding: 10px 20px;
  font-weight: 500;
  border-radius: 6px;
  transition: all 0.3s ease;
}

.el-button--primary:hover {
  background: linear-gradient(135deg, #5568d3 0%, #6a3f9c 100%);
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.el-button--primary:active {
  transform: translateY(0);
}

.el-table {
  border-radius: 8px;
  overflow: hidden;
  border: none;
}

.el-table th {
  background: #764ba2 !important;
  color: white !important;
  font-weight: 600;
  padding: 14px 0;
}


.el-table td {
  padding: 12px 0;
  background: white !important;
}

.el-table .el-table__row:hover > td {
  background-color: #f8f9fc !important;
}

.el-table .el-table__fixed td {
  background: white !important;
}

.el-table .el-table__fixed-right td {
  background: white !important;
}

.el-pagination {
  margin-top: 20px;
}

.el-pagination .el-pager li {
  border-radius: 6px;
  font-weight: 500;
}

.el-pagination .el-pager li.is-active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.el-input__wrapper {
  border-radius: 6px;
  transition: all 0.3s ease;
}

.el-input__wrapper:hover {
  box-shadow: 0 0 0 1px #667eea inset;
}

.el-input__wrapper.is-focus {
  box-shadow: 0 0 0 1px #667eea inset;
}

.el-dialog {
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
}

.el-dialog__header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 20px 24px;
  border-radius: 12px 12px 0 0;
}

.el-dialog__title {
  color: white;
  font-weight: 600;
  font-size: 18px;
}

.el-dialog__headerbtn .el-dialog__close {
  color: white;
  font-size: 20px;
}

.el-dialog__headerbtn:hover .el-dialog__close {
  color: rgba(255, 255, 255, 0.8);
}

.el-dialog__body {
  padding: 24px;
}

.el-form-item__label {
  font-weight: 500;
  color: #333;
}
</style>
