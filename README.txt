Usage
find [Options...] [-p|--path starting-point...] -e|--expression expression...

Options
  -v, --verbose: 不仅输出找到的文件路径，还输出其内容
  -u, --unique: 搜索结果将按字母序排列，并去重
  -h, --help: 获取帮助
  -p, --path: 搜索的起始位置，支持同时搜索多个path
  -e, --expression: 匹配的正则表达式，支持多个正则表达式

支持命令行彩色输出
使用tracing库打印从命令行中解析的starting-points和expressions