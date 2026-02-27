import { ElMessageBox, ElMessage } from 'element-plus';
import html2pdf from 'html2pdf.js';

/**
 * 采购管理PDF导出功能
 * @param {Object} options 配置选项
 * @param {Ref} options.processingOrders 处理中的订单列表
 * @param {Ref} options.selectedOrderId 当前选中的订单ID
 * @param {Ref} options.orderItems 订单明细列表
 * @param {Ref} options.imageUrls 图片URL映射
 */
export function usePurchaseExport({ processingOrders, selectedOrderId, orderItems, imageUrls }) {
  const getImageUrl = (skuCode) => {
    if (!skuCode) return '';
    return imageUrls.value ? imageUrls.value[skuCode] || '' : '';
  };

  const generateRowHtml = (item, index) => {
    const imageUrl = getImageUrl(item.sku_code);
    const imageHtml = imageUrl 
      ? `<img src="${imageUrl}" style="width: 32px; height: 32px; object-fit: cover;" />`
      : '<span>-</span>';
    const specHtml = item.box_quantity > 1 
      ? `${item.spec || ''}*${item.box_spec}/${item.unit}`
      : `${item.spec || ''}/${item.unit}`;
    
    return `
      <tr>
        <td style="border: 1px solid #dcdfe6; padding: 6px; text-align: center;">${index + 1}</td>
        <td style="border: 1px solid #dcdfe6; padding: 6px;">
          <div style="display: flex; align-items: center; gap: 10px;">
            ${imageHtml}
            <div>
              <div style="font-weight: bold;">${item.product_name}</div>
              <div style="color: #999; font-size: 11px;">${item.sku_code}</div>
            </div>
          </div>
        </td>
        <td style="border: 1px solid #dcdfe6; padding: 6px;">
          ${specHtml}
        </td>
        <td style="border: 1px solid #dcdfe6; padding: 6px; text-align: right;">
          ${item.quantity}${item.unit}
        </td>
        <td style="border: 1px solid #dcdfe6; padding: 6px; text-align: right;">
          <div style="color: #409EFF;">¥${item.sale_price.toFixed(2)}</div>
        </td>
        <td style="border: 1px solid #dcdfe6; padding: 6px; text-align: right;">
          <div style="color: #409EFF;">¥${item.total_sale_amount.toFixed(2)}</div>
        </td>
      </tr>
    `;
  };

  const generateTableHtml = (title, rows, totalCost, totalSale, currentPage, totalPages) => `
    <div style="padding: 10px;">
      <h2 style="text-align: center; margin-bottom: 30px;">${title}</h2>
      <table style="width: 100%; border-collapse: collapse; font-size: 12px;">
        <thead>
          <tr style="background-color: #f5f7fa;">
            <th style="border: 1px solid #dcdfe6; padding: 8px; text-align: center;">序号</th>
            <th style="border: 1px solid #dcdfe6; padding: 8px; text-align: center;">商品信息</th>
            <th style="border: 1px solid #dcdfe6; padding: 8px; text-align: center;">产品规格</th>
            <th style="border: 1px solid #dcdfe6; padding: 8px; text-align: center;">数量</th>
            <th style="border: 1px solid #dcdfe6; padding: 8px; text-align: center;">单价</th>
            <th style="border: 1px solid #dcdfe6; padding: 8px; text-align: center;">总价</th>
          </tr>
        </thead>
        <tbody>
          ${rows}
          ${currentPage === totalPages ? `
          <tr style="background-color: #c7510eff; font-weight: bold;">
            <td colspan="4" style="color: #fff; border: 1px solid #dcdfe6; padding: 6px; text-align: right;">合计</td>
            <td style="color: #fff; border: 1px solid #dcdfe6; padding: 6px; text-align: right;">-</td>
            <td style="color: #fff; border: 1px solid #dcdfe6; padding: 6px; text-align: right;">
              <div style="color: #fff; font-weight: bold;">¥${totalSale.toFixed(2)}</div>
            </td>
          </tr>
          ` : `
          <tr style="background-color: #f5f7fa; font-weight: bold;">
            <td colspan="6" style="border: 1px solid #dcdfe6; padding: 6px; text-align: right;"></td>
          </tr>
          ` }
        </tbody>
      </table>
      <div style="text-align: center; margin-top: 20px; color: #999; font-size: 11px;">
        第 ${currentPage} 页 / 共 ${totalPages} 页
      </div>
    </div>
  `;

  const exportPDF = async () => {
    try {
      console.log('开始导出PDF');
      
      if (!orderItems.value || orderItems.value.length === 0) {
        ElMessageBox.alert('没有数据可导出', '提示');
        return;
      }
      
      const orderNo = processingOrders.value.find(o => o.id === selectedOrderId.value)?.order_no || '订单';
      console.log('订单编号:', orderNo);
      
      const totalCost = orderItems.value.reduce((sum, item) => sum + item.total_cost_amount, 0);
      const totalSale = orderItems.value.reduce((sum, item) => sum + item.total_sale_amount, 0);
      
      const pageSize = 20;
      const totalPages = Math.ceil(orderItems.value.length / pageSize);
      
      let allPagesHtml = '';
      
      for (let page = 0; page < totalPages; page++) {
        const startIndex = page * pageSize;
        const endIndex = Math.min(startIndex + pageSize, orderItems.value.length);
        const pageItems = orderItems.value.slice(startIndex, endIndex);
        
        const rows = pageItems.map((item, index) => generateRowHtml(item, startIndex + index)).join('');
        const tableHtml = generateTableHtml(`出货明细单 - ${orderNo}`, rows, totalCost, totalSale, page + 1, totalPages);
        
        allPagesHtml += tableHtml;
        
        if (page < totalPages - 1) {
          allPagesHtml += '<div style="page-break-after: always;"></div>';
        }
      }
      
      const opt = {
        margin: 6,
        filename: `${orderNo}.pdf`,
        image: { type: 'jpeg', quality: 0.98 },
        html2canvas: { 
          scale: 2,
          useCORS: true,
          logging: true
        },
        jsPDF: { 
          unit: 'mm', 
          format: 'a4', 
          orientation: 'portrait' 
        }
      };
      
      await html2pdf().set(opt).from(allPagesHtml).save();
      
      ElMessage.success('PDF已导出到下载目录');
      console.log('PDF导出成功');
    } catch (error) {
      console.error('PDF导出失败:', error);
      ElMessageBox.alert(`PDF导出失败: ${error.message}`, '错误');
    }
  };

  return {
    exportPDF
  };
}
