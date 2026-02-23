import { invoke } from '@tauri-apps/api/tauri';
import { open, save } from '@tauri-apps/api/dialog';
import { open as openExternal } from '@tauri-apps/api/shell';

console.log('Tauri API initialization from @tauri-apps/api');

const tauriAPI = {
  getVersion: () => invoke('get_version'),
  openFile: async () => {
    try {
      const result = await open({
        multiple: true,
        filters: [
          { name: 'Excel Files', extensions: ['xlsx', 'xls'] },
          { name: 'All Files', extensions: ['*'] }
        ]
      });
      return result || [];
    } catch (error) {
      console.error('Open file error:', error);
      return [];
    }
  },
  saveFile: async (defaultName) => {
    try {
      const result = await save({
        defaultPath: defaultName,
        filters: [
          { name: 'Excel Files', extensions: ['xlsx'] },
          { name: 'PDF Files', extensions: ['pdf'] },
          { name: 'All Files', extensions: ['*'] }
        ]
      });
      return result;
    } catch (error) {
      console.error('Save file error:', error);
      return null;
    }
  },
  openExternal: async (url) => {
    await openExternal(url);
  },
  
  sku: {
    list: () => invoke('sku_list'),
    get: (id) => invoke('sku_get', { id }),
    create: (data) => invoke('sku_create', { data }),
    update: (id, data) => invoke('sku_update', { id, data }),
    delete: (id) => invoke('sku_delete', { id }),
    search: (keyword) => invoke('sku_search', { keyword })
  },
  
  category: {
    list: () => invoke('category_list')
  },
  
  customer: {
    list: () => invoke('customer_list')
  },
  
  order: {
    list: () => invoke('order_list'),
    get: (id) => invoke('order_get', { id }),
    create: (data) => invoke('order_create', { data }),
    update: (id, data) => invoke('order_update', { id, data }),
    delete: (id) => invoke('order_delete', { id })
  }
};

if (!window.electronAPI) {
  window.electronAPI = tauriAPI;
  console.log('Using Tauri API from @tauri-apps/api');
}
