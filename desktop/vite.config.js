import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';
import path from 'path';

export default defineConfig({
  plugins: [vue()],
  root: 'src/renderer',
  base: '/',
  publicDir: '../../public',
  build: {
    outDir: '../../build',
    emptyOutDir: true
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src/renderer')
    }
  },
  server: {
    port: 5173,
    strictPort: true,
    hmr: {
      overlay: false
    },
    cors: true,
    origin: 'http://localhost:5173'
  },
  optimizeDeps: {
    include: ['vue', 'vue-router', 'pinia', 'element-plus', '@element-plus/icons-vue']
  }
});
