const mockData = {
  sku: [
    { id: 1, sku_code: '010001', name: '产品A', category_id: 1, category_name: '分类A', unit: '个', box_spec: '12个/箱', cost_price: 10.00, sale_price: 15.00, spec: '规格A', image_path: 'data/images/sku/010001.jpeg' },
    { id: 2, sku_code: '010002', name: '产品B', category_id: 2, category_name: '分类B', unit: '个', box_spec: '24个/箱', cost_price: 20.00, sale_price: 30.00, spec: '规格B', image_path: 'data/images/sku/010002.jpeg' },
    { id: 3, sku_code: '020001', name: '产品C', category_id: 1, category_name: '分类A', unit: '个', box_spec: '6个/箱', cost_price: 15.00, sale_price: 25.00, spec: '规格C', image_path: 'data/images/sku/020001.png' }
  ],
  category: [
    { category_id: 1, category_name: '分类A' },
    { category_id: 2, category_name: '分类B' }
  ],
  customer: [
    { customer_id: 1, customer_name: '客户A' },
    { customer_id: 2, customer_name: '客户B' }
  ],
  order: [
    { id: 1, order_no: 'ORD001', customer_id: 1, customer_name: '客户A', order_date: '2024-01-01', status: 'completed', total_amount: 100.00, remarks: '备注A' },
    { id: 2, order_no: 'ORD002', customer_id: 2, customer_name: '客户B', order_date: '2024-01-02', status: 'pending', total_amount: 200.00, remarks: '备注B' }
  ]
};

const electronAPIMock = {
  getVersion: () => Promise.resolve('1.0.0'),
  openFile: () => Promise.resolve([]),
  saveFile: (defaultName) => Promise.resolve(null),
  openExternal: (url) => Promise.resolve(),
  
  sku: {
    list: () => Promise.resolve(mockData.sku),
    get: (id) => Promise.resolve(mockData.sku.find(s => s.id === id)),
    create: (data) => {
      const newSku = { ...data, id: mockData.sku.length + 1, sku_code: `010${String(mockData.sku.length + 1).padStart(3, '0')}` };
      mockData.sku.push(newSku);
      return Promise.resolve(newSku);
    },
    update: (id, data) => {
      const index = mockData.sku.findIndex(s => s.id === id);
      if (index !== -1) {
        mockData.sku[index] = { ...mockData.sku[index], ...data };
      }
      return Promise.resolve(mockData.sku[index]);
    },
    delete: (id) => {
      mockData.sku = mockData.sku.filter(s => s.id !== id);
      return Promise.resolve();
    },
    search: (keyword) => {
      return Promise.resolve(mockData.sku.filter(s => 
        s.name.toLowerCase().includes(keyword.toLowerCase()) || 
        s.sku_code.toLowerCase().includes(keyword.toLowerCase())
      ));
    },
    import: (filePath) => Promise.resolve(),
    export: () => Promise.resolve()
  },
  
  category: {
    list: () => Promise.resolve(mockData.category)
  },
  
  customer: {
    list: () => Promise.resolve(mockData.customer)
  },
  
  order: {
    list: () => Promise.resolve(mockData.order),
    get: (id) => Promise.resolve(mockData.order.find(o => o.id === id)),
    create: (data) => {
      const newOrder = { ...data, id: mockData.order.length + 1 };
      mockData.order.push(newOrder);
      return Promise.resolve(newOrder);
    },
    update: (id, data) => {
      const index = mockData.order.findIndex(o => o.id === id);
      if (index !== -1) {
        mockData.order[index] = { ...mockData.order[index], ...data };
      }
      return Promise.resolve(mockData.order[index]);
    },
    delete: (id) => {
      mockData.order = mockData.order.filter(o => o.id !== id);
      return Promise.resolve();
    }
  }
};

if (typeof window !== 'undefined' && !window.electronAPI) {
  window.electronAPI = electronAPIMock;
  console.log('Using mock electronAPI for development');
}
