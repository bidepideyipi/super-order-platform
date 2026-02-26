import { ElMessageBox } from 'element-plus';
import * as XLSX from 'xlsx';

/**
 * 采购管理Excel导出功能
 * @param {Object} options 配置选项
 * @param {Ref} options.processingOrders 处理中的订单列表
 * @param {Ref} options.selectedOrderId 当前选中的订单ID
 * @param {Ref} options.orderItems 订单明细列表
 */
export function usePurchaseExport({ processingOrders, selectedOrderId, orderItems }) {
  /**
   * 导出采购明细为Excel文件
   */
  const exportExcel = () => {
    try {
      console.log('开始导出Excel');
      
      if (!orderItems.value || orderItems.value.length === 0) {
        ElMessageBox.alert('没有数据可导出', '提示');
        return;
      }
      
      const orderNo = processingOrders.value.find(o => o.id === selectedOrderId.value)?.order_no || '订单';
      console.log('订单编号:', orderNo);
      
      const exportData = orderItems.value.map(item => ({
        '商品名称': item.product_name,
        'SKU编码': item.sku_code,
        '每(单位)N件': item.box_quantity > 1 ? `每${item.unit}${item.box_quantity}件` : item.unit,
        '箱规': item.box_spec || '',
        '数量': `${item.quantity}${item.unit}`,
        '成本价': item.cost_price,
        '销售价': item.sale_price,
        '总成本': item.total_cost_amount,
        '总售价': item.total_sale_amount,
        '利润': item.total_sale_amount - item.total_cost_amount
      }));
      
      const ws = XLSX.utils.json_to_sheet(exportData);
      const wb = XLSX.utils.book_new();
      XLSX.utils.book_append_sheet(wb, ws, '采购明细');
      
      XLSX.writeFile(wb, `${orderNo}.xlsx`);
      
      console.log('Excel导出成功');
    } catch (error) {
      console.error('Excel导出失败:', error);
      ElMessageBox.alert(`Excel导出失败: ${error.message}`, '错误');
    }
  };

  return {
    exportExcel
  };
}