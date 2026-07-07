# OpenCode 每天白嫖额度

OpenCode 安装后默认是**游客模式**，每天可完整开发 1-2 个项目。额度用完后切换到 API 模式，接续使用免费模型，实现每天双倍额度。

> ⚠️ **额度上限问题**：有时游客模式额度用完后，切换到 API 模式仍提示受限。这是因为 OpenCode 通过 IP 判断额度状态。切换 API 登录**前需要先换 IP**（切换流量/热点，或开 VPN），否则 API 登录后仍会显示额度上限。VPN 设置见底部。

## 流程概览

```
游客模式 → 额度用完 → 切 IP → API 模式登录 → 继续用 Free 模型
    ↑                                           |
    └────────── 次日 logout 循环 ──────────────┘
```

---

## 一、获取 API 密钥

1. 打开 [OpenCode Zen](https://opencode.ai/zh/zen)
2. 点击 **登录**，使用 GitHub 或 Google 账号登录
   ![](/images/Pasted%20image%2020260707123338.png)
   ![](/images/Pasted%20image%2020260707123511.png)
3. 登录后点击 **API 密钥** → **创建** → 复制密钥
   ![](/images/Pasted%20image%2020260707123644.png)

---

## 二、登录 API 模式

```bash
opencode auth login
```

![](/images/Pasted%20image%2020260707123731.png)

1. 键盘方向键选择 `OpenCode Zen`，回车
   ![](/images/Pasted%20image%2020260707123900.png)
2. 粘贴刚复制的 API 密钥，回车
   ![](/images/Pasted%20image%2020260707123915.png)
3. 输入 `/models` 查看可用模型
   ![](/images/Pasted%20image%2020260707124018.png)
   ![](/images/Pasted%20image%2020260707124046.png)
4. 选择 **DeepSeek V4 Flash Free**（推荐，能力较强）
5. 按 `Ctrl+T` 切换思考模式
   ![](/images/Pasted%20image%2020260707124200.png)
   ![](/images/Pasted%20image%2020260707124222.png)

---

## 三、次日切回游客模式

```bash
opencode auth logout
```

![](/images/Pasted%20image%2020260707124441.png)

回车退出 API 模式。次日游客模式额度用完后，切 IP 再按第二步重新登录 API 即可循环。

> 💡 **多账号倍增**：注册多个 GitHub 账号，每个生成一个 API 密钥，轮换使用可进一步突破额度限制。

---

## 四、换 IP 解决额度上限

切换到 API 模式前如果没换 IP，会提示额度上限。需要用 VPN 换 IP 后再执行 `opencode auth login`。

### 推荐客户端：Clash Verge Rev

[Clash Verge Rev](https://github.com/Clash-Verge-rev/clash-verge-rev)（GitHub 下载慢的话，可从蓝奏云下载）

📦 [蓝奏云下载](https://wwbbt.lanzout.com/iTcdq3uybyxg) 密码：`6xx0`

![](/images/Pasted%20image%2020260707184310.png)

### 设置步骤

1. **填写订阅链接** — 在「订阅」页的连接位置输入你买的订阅链接
   ![](/images/Pasted%20image%2020260707184437.png)

2. **选择代理模式** — 首页有两个模式：**系统代理**（推荐，省心）和**虚拟网卡**（高级）。不懂就选系统代理
   ![](/images/Pasted%20image%2020260707184622.png)

3. **选择节点** — 点击「节点」栏，选择一个节点
   ![](/images/Pasted%20image%2020260707184650.png)

   > 💡 如果节点列表未显示状态，先随便选一个再点开就能正常显示
   > ![](/images/Pasted%20image%2020260707184720.png)

4. **测速切换** — 选择延迟低的节点即可
   ![](/images/Pasted%20image%2020260707184835.png)

5. **验证 IP** — 切换后刷新，IP 变了就说明成功。此时执行 `opencode auth login` 就不会再提示额度上限了

---

### 节点推荐

| 机场 | 价格 | 特点 |
|------|------|------|
| [NanoCloud](https://edu.360buyimg.men/app/dashboard) | 低至 1 元 / 100G | 2 台设备，节点少 |
| [赔钱机场](https://xn--mes358aby2apfg.com/dashboard) | 低至 1.5 元 / 100G | 最多 10 台设备，节点多 |
| [良心云](https://xn--9kqz23b19z.com/#/shop) | 低至 2 元 / 100G | 8 台设备，节点多 |
| [一分机场](https://xn--4gqx1hgtfdmt.com/#/dashboard) | 低至 2 元 / 100G | 10 台设备 |
