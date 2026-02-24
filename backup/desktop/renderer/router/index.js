import { createRouter, createWebHashHistory } from 'vue-router';

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => {
      console.log('Loading Home component');
      return import('../views/Home.vue');
    }
  },
  {
    path: '/sku',
    name: 'SKU',
    component: () => {
      console.log('Loading SKU component');
      return import('../views/SKU.vue');
    }
  },
  {
    path: '/orders',
    name: 'Orders',
    component: () => {
      console.log('Loading Orders component');
      return import('../views/Orders.vue');
    }
  },
  {
    path: '/customers',
    name: 'Customers',
    component: () => {
      console.log('Loading Customers component');
      return import('../views/Customers.vue');
    }
  },
  {
    path: '/test-tauri',
    name: 'TestTauri',
    component: () => {
      console.log('Loading TestTauri component');
      return import('../views/TestTauri.vue');
    }
  }
];

const router = createRouter({
  history: createWebHashHistory(),
  routes
});

router.beforeEach((to, from, next) => {
  console.log('Router navigation:', { from: from.path, to: to.path });
  next();
});

export default router;
