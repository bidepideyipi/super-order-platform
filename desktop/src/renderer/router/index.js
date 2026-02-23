import { createRouter, createWebHashHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },
  {
    path: '/sku',
    name: 'SKU',
    component: () => import('../views/SKU.vue')
  },
  {
    path: '/orders',
    name: 'Orders',
    component: () => import('../views/Orders.vue')
  },
  {
    path: '/customers',
    name: 'Customers',
    component: () => import('../views/Customers.vue')
  },
  {
    path: '/test-tauri',
    name: 'TestTauri',
    component: () => import('../views/TestTauri.vue')
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

export default router;
