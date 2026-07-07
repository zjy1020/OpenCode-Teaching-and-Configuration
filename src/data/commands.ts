export interface Command {
  name: string
  description: string
  content: string
}

export type CommandStatus = 'installed' | 'available' | 'local-only'

export interface CommandWithStatus extends Command {
  status: CommandStatus
}

export const embeddedCommands: Command[] = [
  {
    name: 'see',
    description: '分析图片（截图/拖拽/URL 均可，自动收集所有图片走视觉模型分析）',
    content: `按优先级收集图片（URL/路径/嵌入/剪贴板），通过 bash 调用视觉模型分析，汇总结果回复用户。`,
  },
  {
    name: 'frontend',
    description: '前端设计方案生成器。读取项目文档，结合 frontend-design + ui-ux-pro-max 产出多套方案+HTML预览，自动打开浏览器。',
    content: `加载 frontend-design 和 ui-ux-pro-max 两个 skill，按产品类型生成 8-15 个前端方案并生成 HTML 对比预览页。`,
  },
]
