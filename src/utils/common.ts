/**
 * 清理文本中的多余空格和特殊字符
 * @param text 原始文本
 * @returns 清理后的文本
 */
export const cleanText = (text: string): string => {
  if (!text) return '';
  return text.replace(/\s+/g, ' ').trim();
};

/**
 * 获取搜索提示的样式类
 * @param searchResult 搜索结果文本
 * @returns 样式类名
 */
export const getSearchTipClass = (searchResult: string): string => {
  if (!searchResult) return '';
  if (searchResult.includes('找到')) return 'search-tip-success';
  if (searchResult.includes('请输入')) return 'search-tip-warning';
  return 'search-tip-error';
};