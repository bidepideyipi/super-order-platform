import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token') || '');
  const username = ref(localStorage.getItem('username') || '');
  const isAuthenticated = ref(!!token.value);

  const login = async (user, password) => {
    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          username: user,
          password: password
        })
      });

      const data = await response.json();

      if (!response.ok || !data.success) {
        throw new Error(data.message || '登录失败');
      }

      token.value = data.token;
      username.value = user;
      isAuthenticated.value = true;

      localStorage.setItem('token', data.token);
      localStorage.setItem('username', user);

      return data;
    } catch (error) {
      throw error;
    }
  };

  const logout = async () => {
    try {
      await fetch('/api/auth/logout', {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${token.value}`
        }
      });
    } catch (error) {
      console.error('Logout error:', error);
    } finally {
      token.value = '';
      username.value = '';
      isAuthenticated.value = false;
      localStorage.removeItem('token');
      localStorage.removeItem('username');
    }
  };

  const checkAuth = async () => {
    if (!token.value) {
      return false;
    }

    try {
      const response = await fetch('/api/auth/check', {
        headers: {
          'Authorization': `Bearer ${token.value}`
        }
      });

      return response.ok;
    } catch (error) {
      return false;
    }
  };

  return {
    token,
    username,
    isAuthenticated,
    login,
    logout,
    checkAuth
  };
});
