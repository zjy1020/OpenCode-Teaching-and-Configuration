# OpenCode 教学工具

基于 Tauri v2 的 OpenCode 配置教学桌面应用。技能/命令资源嵌入在二进制中，生成单文件便携 EXE。

## 前置要求

- Node.js 20+
- Rust（最新 stable）
- Windows：WebView2（Win11 自带，Win10 需安装）

## 运行

```bash
# 安装前端依赖
npm install

# 仅 Web 开发（浏览器预览）
npm run dev

# 桌面开发（自动启动窗口 + HMR）
npm run tauri:dev
```

## 打包

```bash
# 打包为单文件 EXE（输出在 src-tauri/target/release/bundle/）
npm run tauri:build
```

## 项目结构

```
├── src/                    # Vue 3 前端
│   ├── components/         # 通用组件
│   ├── content/            # 教学文档（Markdown）
│   ├── data/               # 技能/命令元数据
│   ├── lib/                # Tauri 调用封装
│   ├── pages/              # 页面组件
│   └── router/             # 路由配置
├── src-tauri/              # Rust 后端
│   └── src/lib.rs          # Tauri 命令（扫描、导入、删除、打开文件夹）
├── commands/               # OpenCode 命令定义
├── docs/                   # 设计文档
├── skills/                 # OpenCode 技能（SKILL.md + 素材）
└── public/images/          # 教学文档截图
```
