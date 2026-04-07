export function useOrderStatus() {
  const statusMap = {
    'pending': {
      type: 'warning',
      text: '待确认'
    },
    'processing': {
      type: 'primary',
      text: '采购中'
    },
    'delivering': {
      type: 'info',
      text: '货运中'
    },
    'delivered': {
      type: 'success',
      text: '已送达'
    },
    'cancelled': {
      type: 'danger',
      text: '已取消'
    }
  };

  const getStatusType = (status) => {
    return statusMap[status]?.type || 'info';
  };

  const getStatusText = (status) => {
    return statusMap[status]?.text || status;
  };

  const statusOptions = [
    { label: '待确认', value: 'pending' },
    { label: '采购中', value: 'processing' },
    { label: '货运中', value: 'delivering' },
    { label: '已送达', value: 'delivered' },
    { label: '已取消', value: 'cancelled' }
  ];

  return {
    getStatusType,
    getStatusText,
    statusOptions
  };
}
