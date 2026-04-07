import { createRouter, createWebHashHistory } from 'vue-router';
import { useAuthStore } from '@/stores/auth';

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: () => import('../views/Login.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    name: 'Home',
    component: () => {
      console.log('Loading Home component');
      return import('../views/Home.vue');
    },
    meta: { requiresAuth: true }
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
    path: '/purchases',
    name: 'Purchases',
    component: () => {
      console.log('Loading Purchase component');
      return import('../views/Purchase.vue');
    }
  },
  {
    path: '/settlement',
    name: 'Settlement',
    component: () => {
      console.log('Loading Settlement component');
      return import('../views/Settlement.vue');
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
    path: '/financial',
    name: 'Financial',
    component: () => {
      console.log('Loading Financial component');
      return import('../views/Financial.vue');
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
  
  const authStore = useAuthStore();
  const requiresAuth = to.meta.requiresAuth !== false;
  
  if (requiresAuth && !authStore.isAuthenticated) {
    next('/login');
  } else if (to.path === '/login' && authStore.isAuthenticated) {
    next('/');
  } else {
    next();
  }
});

export default router;
