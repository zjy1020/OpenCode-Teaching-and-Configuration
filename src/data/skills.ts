export type Category = 'superpowers' | 'frontend' | 'utility'

export interface Skill {
  name: string
  description: string
  category: Category
  repo: string | null
}

export type SkillStatus = 'installed' | 'available' | 'local-only'

export interface SkillWithStatus extends Skill {
  status: SkillStatus
}

export const categoryLabels: Record<Category, string> = {
  superpowers: 'Superpowers 流水线',
  frontend: '前端设计',
  utility: '实用工具',
}

export const embeddedSkills: Skill[] = [
  // Superpowers 流水线
  { name: 'brainstorming', description: '把想法变成设计的协作对话工具', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'writing-plans', description: '将需求拆分为可执行的 step-by-step 计划', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'test-driven-development', description: 'Red-Green-Refactor TDD 流程', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'executing-plans', description: '按计划逐任务执行开发', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'obra-superpowers-subagent-driven-development', description: '子 agent 并行执行独立任务', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'using-git-worktrees', description: 'Git worktree 隔离开发环境', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'finishing-a-development-branch', description: '开发完成后合并/PR/清理', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'verification-before-completion', description: '完成声明前强制运行验证', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },
  { name: 'systematic-debugging', description: '4 阶段系统化 bug 定位与修复', category: 'superpowers', repo: 'https://github.com/obra/superpowers' },

  // 前端设计
  { name: 'frontend-design', description: '创建高质量、有风格的前端界面', category: 'frontend', repo: 'https://github.com/anthropics/claude-code' },
  { name: 'ui-ux-pro-max', description: '全面 UI/UX 设计（50+ 样式/161 配色）', category: 'frontend', repo: 'https://github.com/nextlevelbuilder/ui-ux-pro-max-skill' },

  // 实用工具
  { name: 'concise-response-skill', description: '极致省 token 的精简回复', category: 'utility', repo: null },
  { name: 'find-skills', description: '搜索并安装社区 Skill', category: 'utility', repo: 'https://github.com/vercel-labs/skills' },
  { name: 'karpathy-guidelines', description: 'Karpathy 编码原则：简单优先', category: 'utility', repo: 'https://github.com/multica-ai/andrej-karpathy-skills' },
  { name: 'web-access', description: '联网搜索、网页抓取与登录操作', category: 'utility', repo: 'https://github.com/eze-is/web-access' },
  { name: 'agent-skill-creator', description: '从描述自动创建跨平台 Skill', category: 'utility', repo: 'https://github.com/FrancyJGLisboa/agent-skill-creator' },
]
