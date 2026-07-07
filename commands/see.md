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
