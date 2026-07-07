# OpenCode 免费让 DeepSeek 调用视觉模型

DeepSeek V4 Flash Free 本身**不支持视觉模型**，但 OpenCode 的 Free 模型里支持视觉的只有 **MiMO v2.5 Free**。通过 commands 机制，可以让 DeepSeek 间接调用视觉能力。

## 原理

在 `~/.opencode/commands/` 下放一个命令文件，AI 在处理时会自动调用指定的视觉模型分析图片，再把结果交给当前模型（DeepSeek）理解后回复。

## 安装

把 `see.md` 放到 `~/.opencode/commands/` 即可。

```
~/.opencode/commands/see.md
```

`see.md` 文件内容如下：

````markdown
---
description: 分析图片（截图/拖拽/URL 均可，自动收集所有图片走视觉模型分析）
---

**在回答用户之前，必须先执行以下图片收集流程。收集到任何图片就调用视觉模型分析，收集不到再正常回答问题。**

## 第一步：收集所有图片

必须按顺序检查以下来源：

1. **用户消息中的图片 URL** — 检查消息里是否有 http/https 的图片链接
2. **用户消息中的本地文件路径** — 检查消息里是否有指向图片的本地路径
3. **用户消息中的嵌入图片数据** — 检查是否有 base64 等嵌入图片数据
4. **Windows 剪贴板** — 立即运行以下命令检查剪贴板是否有截图：
   ```powershell
   Add-Type -AssemblyName System.Windows.Forms -ErrorAction Stop
   $img = [Windows.Forms.Clipboard]::GetImage()
   if ($img) {
     $tmp = "$env:TEMP\opencode_see_$(Get-Date -Format 'yyyyMMddHHmmssfff').png"
     $img.Save($tmp, [System.Drawing.Imaging.ImageFormat]::Png)
     Write-Output $tmp
   }
   ```

## 第二步：处理

如果找到图片，**不要在当前模型下分析**，调用视觉模型：

```bash
opencode run -m opencode/mimo-v2.5-free "请用中文回答用户的问题。用户的问题是：「<用户消息>」。分析这张图片：<图片路径/URL>。**重要：只给出分析和修改方案，不要修改任何文件。**"
```

如果没找到任何图片，直接回答用户问题。

## 第三步：汇总

拿到视觉模型的分析结果后，你先理解一遍再用自己的话回复用户，别直接搬运。
````

## 使用

输入 `/see` 会自动补全（命令名就是文件名）。

```
/see 【粘贴截图】 问题
```

截图复制后直接粘贴，比依赖剪贴板自动检测更稳定。

![](/images/Pasted%20image%2020260707130302.png)
输入 `/see` 会自动显示

![](/images/Pasted%20image%2020260707130759.png)
随便截一张图试试

![](/images/Pasted%20image%2020260707130910.png)
粘贴截图后发送

### 执行流程

触发后会自动调用视觉模型分析，再把结果交给当前模型思考后输出。

![](/images/Pasted%20image%2020260707131008.png)
![](/images/Pasted%20image%2020260707131110.png)

## 注意

MiMO v2.5 Free 能力有限——识别截图文字还行，但前端界面等复杂场景可能理解不到位。有总比没有好。

注意视觉模型只能读取剪贴板中**最新的那一个**，所以在 AI 回复之前不要再去复制其他文字或图片，否则会覆盖掉需要分析的截图。
