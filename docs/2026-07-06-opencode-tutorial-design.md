# OpenCode 教学工具 — 设计文档

## 概述

一个 Vue 3 前端教学工具，用于 OpenCode 的学习、技能管理和命令教学。初期为纯前端 Web 应用，后续迁移至 Tauri v2 打包为 EXE。

## 核心功能

### 1. 教学文档（📖 教学）

内嵌在项目 `src/content/` 中的 Markdown 文件，按树形目录组织，在侧边栏展示章节结构。点击切换时通过 ContentPage 渲染 Markdown 内容。

### 2. Skills 管理（🧰 技能）

扫描用户 `~/.opencode/skills/` 目录，对比内嵌资源清单，以三种状态展示：
- **已安装** — 本地已有该技能
- **可导入** — 内嵌资源中有的技能，本地没有，可选择性导入
- **仅本地** — 本地有但内嵌清单未收录

导入操作为批量选择后复制文件到目录。

### 3. Commands 管理（⚡ 命令）

与 Skills 相同的"检测 + 导入"模式，扫描 `~/.opencode/commands/` 目录。解析 .md frontmatter 获取描述信息，支持查看命令详情和预留视觉模型教学区域。

## 目标用户

OpenCode 用户（含他人），路径自动适配 `~/.opencode/`，不硬编码。

## 技术栈

- **框架**: Vue 3 + TypeScript + Vite
- **路由**: Vue Router
- **Markdown 渲染**: markdown-it 或同类库
- **样式**: Tailwind CSS 4（沿用用户现有项目风格）
- **后期桌面**: Tauri v2（Rust 侧 fs API 实现文件读写）

## 组件树

```
App.vue
├── Sidebar.vue（导航树，三板块）
└── <router-view>
    ├── ContentPage.vue（教学文档渲染）
    ├── SkillList.vue
    │   └── SkillCard.vue * N
    ├── CommandList.vue
    │   └── CommandCard.vue * N
    └── SearchBar.vue（全局全文搜索）
```

## 数据流

### 扫描流程

```
启动应用
  -> 读取内嵌资源清单 (skills/commands)
  -> 尝试读取 ~/.opencode/skills/ 和 ~/.opencode/commands/（浏览器阶段返回 mock）
  -> 合并为完整列表（标注每项状态）
  -> 渲染到 UI
```

### 导入流程

```
用户勾选可导入项
  -> 点击"导入选中"
  -> 纯前端阶段：标记为"已安装"（mock）
  -> Tauri 阶段：fs.copyFile 到目标目录
```

## 目录结构

```
opencode-tutorial/
├── src/
│   ├── App.vue
│   ├── main.ts
│   ├── router.ts
│   ├── components/
│   │   ├── Sidebar.vue
│   │   ├── ContentPage.vue
│   │   ├── SkillList.vue
│   │   ├── SkillCard.vue
│   │   ├── CommandList.vue
│   │   ├── CommandCard.vue
│   │   └── SearchBar.vue
│   ├── composables/
│   │   ├── useScanner.ts
│   │   ├── useImport.ts
│   │   └── useSearch.ts
│   ├── data/
│   │   ├── skills/
│   │   └── commands/
│   └── content/
├── public/
├── package.json
└── vite.config.ts
```

## 阶段规划

### Phase 1 — 纯前端

- 搭建 Vue 3 + Vite + Tailwind 项目
- 实现侧边栏导航和三板块路由
- 内嵌示例教学文档和 Skills/Commands 清单
- useScanner / useImport 为 mock 实现
- 全文搜索功能（搜索范围：所有教学文档 + 内嵌技能/命令名称与描述）
- useScanner 通过检测 `~/.opencode/` 定位路径（可通过环境变量 OPENCODE_HOME 覆盖）

### Phase 2 — Tauri 集成

- 初始化 Tauri v2 项目
- 用 Rust fs API 替换 mock 扫描和导入
- 打包为 EXE

## 约束

- TypeScript 严格模式
- 中文 UI 描述
- 遵循用户现有项目的代码风格（Tailwind CSS 4, 函数组件 + Hooks）
