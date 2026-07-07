---
description: 前端设计方案生成器。加载 frontend-design + ui-ux-pro-max 两个 skill，读取项目文档和计划生成多套方案，HTML 对比预览，自动打开浏览器查看。
---

# 前端设计方案生成器

需要同时加载 **frontend-design** 和 **ui-ux-pro-max** 两个 skill 执行以下流程。

---

## 步骤一：获取项目信息

**优先从当前对话上下文获取已有项目信息，不要重复询问。**

1. 检查对话历史中是否已有项目目录、需求、设计文档、架构说明等信息
2. 如果已有 → 直接从已有信息中提取：产品类型、目标用户、功能模块、设计风格倾向、品牌色等，**跳过询问**
3. 如果没有 → 再问用户：
   - **项目目录在哪？** — 让用户输入绝对路径
   - **有没有计划和文档？给我看看** — 让用户提供项目中的设计文档、需求文档、README、计划等
     - 扫描项目目录下的 `.md` 文件和 `docs/` 文件夹
     - 读取这些文件内容
     - 从中提取：产品类型、目标用户、功能模块、设计风格倾向、品牌色等

如果用户也没有文档，就改为口头问清需求（产品类型、目标用户、风格倾向）。

---

## 步骤二：生成方案列表

基于从文档/用户口中提取的需求，从两个 skill 中提取兼容元素组合，生成 **8-15 个完整设计方案**。

### 资源来源

**frontend-design：**
- 12+ 美学方向（brutally minimal, maximalist chaos, retro-futuristic, organic/natural, luxury/refined, playful/toy-like, editorial/magazine, brutalist/raw, art deco/geometric, soft/pastel, industrial/utilitarian 等）
- 排版、色彩、动效、空间、背景指南

**ui-ux-pro-max：**
- 50+ 风格（glassmorphism, claymorphism, minimalism, brutalism, neumorphism, bento grid, skeuomorphism, flat design, dark mode 等）
- 161 色调搭配（按产品类型匹配）
- 57 字体搭配
- 99 UX 指南
- 布局模式

### 在终端输出方案列表

```
┌─────────────────────────────────────────────────────┐
│ 🎨 方案 1: 玻璃拟态 · 暖橙渐变                        │
│ 风格: glassmorphism + cozy                          │
│ 色板: #F97316 #FFEDD5 #78350F #FFF7ED               │
│ 字体: Fredoka + Nunito                              │
│ 布局: Bento Grid                                    │
│ UX: 大圆角 + 毛玻璃 + 柔和阴影 + 温暖交互反馈          │
│ 一句话: 适合注重亲和力的 C 端产品，温暖且有质感          │
└─────────────────────────────────────────────────────┘
```

**规则：**
- 每个方案必须真正不同（不同风格、色系、字体）
- 按匹配度排序，最合适排前面
- 色板直接给 HEX 色号
- 字体用 Google Fonts 真实字体

---

## 步骤三：创建 test-schemes 并生成 HTML 对比页

1. 在项目目录下创建 `test-schemes\` 文件夹
2. 生成包含全部方案的 HTML 对比页 — **完整写入文件后再启动服务器**
3. 用端口 8765 启动本地服务（被占用则换 8766/8767）
4. 确认服务器就绪后再打开浏览器

### HTML 对比页要求

在一个页面中展示所有方案，每个方案一个卡片区，从上到下排列。

```html
<!DOCTYPE html>
<html lang="zh-CN">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>前端方案对比 - OpenCode</title>
  <link href="https://fonts.googleapis.com/css2?family=..." rel="stylesheet">
  <style>
    body { font-family: system-ui, sans-serif; background: #f5f5f5; margin: 0; padding: 24px; }
    .scheme-card { 
      background: #fff; border-radius: 16px; padding: 24px; margin-bottom: 24px;
      box-shadow: 0 2px 12px rgba(0,0,0,0.08);
    }
    .scheme-title { font-size: 1.3rem; font-weight: 700; margin-bottom: 12px; }
    .color-swatches { display: flex; gap: 8px; margin: 12px 0; }
    .swatch { width: 48px; height: 48px; border-radius: 8px; border: 1px solid #e5e7eb; }
    .font-preview { margin: 8px 0; }
    .font-heading { font-size: 1.5rem; font-weight: 700; }
    .font-body { font-size: 1rem; line-height: 1.6; }
    .component-demo { 
      border: 1px solid #e5e7eb; border-radius: 12px; padding: 16px; margin: 12px 0;
      display: flex; gap: 12px; flex-wrap: wrap; align-items: center;
    }
    .btn { padding: 8px 20px; border-radius: 8px; border: none; cursor: pointer; font-size: 0.9rem; }
    .card-sample { padding: 16px; border-radius: 12px; width: 200px; }
  </style>
</head>
<body>
  <h1>[产品类型] — 前端方案对比</h1>
  <p>共 N 个方案，按匹配度排序</p>

  <!-- 每个方案一个卡片 -->
  <div class="scheme-card">
    <div class="scheme-title">🎨 方案 1: [方案名]</div>
    <div style="display:flex;gap:16px;flex-wrap:wrap;">
      <div style="min-width:200px;">
        <div>风格: [风格名]</div>
        <div>字体: [标题字体] + [正文字体]</div>
        <div>布局: [布局模式]</div>
        <div>UX: [3 条要点]</div>
      </div>
      <div>
        <div class="color-swatches">
          <div class="swatch" style="background:[色1]" title="[色1]"></div>
          <div class="swatch" style="background:[色2]" title="[色2]"></div>
          <div class="swatch" style="background:[色3]" title="[色3]"></div>
          <div class="swatch" style="background:[色4]" title="[色4]"></div>
          <div class="swatch" style="background:[色5]" title="[色5]"></div>
        </div>
        <div class="font-preview" style="font-family: '[标题字体]';">
          <div class="font-heading">标题预览 — 你好，OpenCode</div>
          <div class="font-body" style="font-family: '[正文字体]';">正文预览：这是一个示例段落，展示正文字体在实际阅读中的效果。</div>
        </div>
        <div class="component-demo">
          <button class="btn" style="background:[主色];color:#fff;">主要按钮</button>
          <button class="btn" style="border:2px solid [主色];color:[主色];background:transparent;">次要按钮</button>
          <div class="card-sample" style="background:[表面色];border:1px solid [边框色];">
            <div style="font-weight:700;">示例卡片</div>
            <div style="font-size:0.85rem;">这是卡片内容...</div>
          </div>
        </div>
      </div>
    </div>
  </div>
  <!-- 重复以上 card 结构 8-15 次 -->
</body>
</html>
```

### 启动本地服务

必须等 HTML 文件**完全写入磁盘后再启动服务器和浏览器**：

```powershell
# 确定 schemes 目录
$schemesDir = "项目目录/test-schemes"
New-Item -ItemType Directory -Force -Path $schemesDir | Out-Null

# 写入 HTML 文件（这里写入完整内容）
$htmlContent = @'
[完整的 HTML 内容]
'@
Set-Content -Path "$schemesDir\schemes-compare.html" -Value $htmlContent -Encoding UTF8

# 确认文件已写入
Start-Sleep -Milliseconds 500
if (-not (Test-Path "$schemesDir\schemes-compare.html")) {
  Write-Error "文件写入失败"
  return
}

# 检查端口是否被占用，被占用则换端口
$port = 8765
while ((Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue).Count -gt 0) {
  $port++
}

# 启动服务器 — 检查可用工具
$hasNode = Get-Command npx -ErrorAction SilentlyContinue
if ($hasNode) {
  Start-Process -NoNewWindow powershell -Args "-Command npx --yes http-server `"$schemesDir`" -p $port --silent"
} else {
  Start-Process -NoNewWindow powershell -Args "-Command python -m http.server $port -d `"$schemesDir`""
}

# 等待服务器就绪（轮询检测）
$maxRetries = 10
$ready = $false
for ($i = 0; $i -lt $maxRetries; $i++) {
  Start-Sleep -Seconds 1
  try {
    $req = [System.Net.WebRequest]::Create("http://localhost:$port/schemes-compare.html")
    $req.Timeout = 1000
    $resp = $req.GetResponse()
    if ($resp.StatusCode -eq 200) { $ready = $true; break }
  } catch { }
}

if (-not $ready) {
  Write-Output "⚠️ 服务器启动较慢，请手动打开 http://localhost:$port"
} else {
  Write-Output "✅ 服务器就绪，正在打开浏览器..."
}

# 打开浏览器
Start-Process "http://localhost:$port/schemes-compare.html"
```

---

## 步骤四：满意度确认

**终端等待用户反馈，问：**

> **"看完方案了吗？这些方案满意吗？"**

- **满意** → "第几号方案感兴趣？" → 进入步骤五
- **不满意** → "好，重新给你出一批新方案"
  - 回到步骤二重新生成 8-15 个
  - 覆盖写入 `test-schemes/schemes-compare.html`
  - 服务器已经在运行，刷新浏览器即可看到新内容
  - 提醒用户：**"已重新生成，请刷新浏览器查看"**

---

## 步骤五：用户选择方案

用户说"第 X 号"后：

1. 问"是否用这套方案继续优化前端？"
2. **是** → 在 `test-schemes/` 下生成**详细方案 HTML**
   - 文件名：`scheme-detailed-方案名.html`
   - 包含完整设计系统 tokens（CSS 变量）
   - 更多组件示例（导航、表单、列表、弹窗等）
   - 页面布局示意（header/main/footer）
   - 如果项目已有前端代码（有 `index.html`/`src/`/`package.json`），同时输出应用了该方案的**修改后的组件代码**
   - 自动刷新打开 `http://localhost:$port/scheme-detailed-方案名.html`
3. **否** → "你想怎么调整？" → 调整后重新出方案

---

## 步骤六：退出清理

用户确认完成所有操作后，问：

> **"是否关闭本地服务（端口 $port）？"**

- **是** → 
  ```powershell
  $procId = (Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue).OwningProcess
  if ($procId) { Stop-Process -Id $procId -Force }
  ```
- **否** → 保持运行

---

## 注意事项

- 每个方案卡片中的色板 swatch 用内联 `style="background:xxx"`，不要用 class 映射
- 按钮/卡片等组件一定要带入该方案的主色/辅色/字体
- 字体用 Google Fonts 可加载的真实中英文字体
- 检查端口是否被占用，被占用了自动换 8766/8767
- 必须先完整写入 HTML 文件，确认文件存在后再启动服务器
- 服务器启动后必须轮询确认就绪，再打开浏览器
- 如果用户不满意方案，重新生成后覆盖同文件，服务器不用重启，用户刷新浏览器即可
