let tauriAPI;

if (typeof window !== 'undefined' && window.__TAURI__) {
  import('@tauri-apps/api/tauri').then(({ invoke }) => {
    import('@tauri-apps/api/dialog').then(({ open, save }) => {
      import('@tauri-apps/api/shell').then(({ open: openExternal }) => {
        console.log('Tauri API initialization from @tauri-apps/api');

        tauriAPI = {
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
            listPaginated: (page, pageSize) => invoke('sku_list_paginated', { page, page_size: pageSize }),
            get: (id) => invoke('sku_get', { id }),
            create: (data) => invoke('sku_create', { data }),
            update: (id, data) => invoke('sku_update', { id, data }),
            delete: (id) => invoke('sku_delete', { id }),
            search: (keyword) => invoke('sku_search', { keyword }),
            searchPaginated: (keyword, page, pageSize) => invoke('sku_search_paginated', { keyword, page, page_size: pageSize }),
            searchWithCategory: (keyword, categoryId) => invoke('sku_search_with_category', { keyword, categoryId })
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

        if (!window.tauriAPI) {
          window.tauriAPI = tauriAPI;
          console.log('Using Tauri API from @tauri-apps/api');
        }
      });
    });
  });
} else {
  console.log('Browser environment - using mock API');
  tauriAPI = {
    getVersion: () => Promise.resolve('1.0.0'),
    openFile: () => Promise.resolve([]),
    saveFile: () => Promise.resolve(null),
    openExternal: () => Promise.resolve(),
    
    sku: {
      list: () => Promise.resolve([]),
      listPaginated: () => Promise.resolve({ data: [], total: 0, total_pages: 0 }),
      get: () => Promise.resolve(null),
      create: () => Promise.reject(new Error('Not implemented in browser')),
      update: () => Promise.reject(new Error('Not implemented in browser')),
      delete: () => Promise.reject(new Error('Not implemented in browser')),
      search: () => Promise.resolve([]),
      searchPaginated: () => Promise.resolve({ data: [], total: 0, total_pages: 0 }),
      searchWithCategory: () => Promise.resolve([])
    },
    
    category: {
      list: () => Promise.resolve([])
    },
    
    customer: {
      list: () => Promise.resolve([])
    },
    
    order: {
      list: () => Promise.resolve([]),
      get: () => Promise.resolve(null),
      create: () => Promise.reject(new Error('Not implemented in browser')),
      update: () => Promise.reject(new Error('Not implemented in browser')),
      delete: () => Promise.reject(new Error('Not implemented in browser'))
    }
  };

  if (!window.tauriAPI) {
    window.tauriAPI = tauriAPI;
  }
}
