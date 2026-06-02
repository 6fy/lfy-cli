# 授权与登录

安装完 `lfy-cli`（`npm install -g @6fy/cli`）后，**第一步是完成 LFY 系统授权**：需要申请 **user key** 与 **user secret**，再用它们登录绑定账号。

## 第一步：申请 user key 与 user secret

有两个途径，任选其一：

- **方式一（自助获取）**：访问 [https://app.6fenyi.com/v2/](https://app.6fenyi.com/v2/)，点击右上角 **【CLI】 → 【获取授权】**，即可得到授权命令（含 user key / user secret）。
- **方式二（人工申请）**：直接联系 LFY 销售客服，提出 CLI 申请授权。

## 第二步：登录绑定

拿到 key / secret 后执行登录（仅需一次）：

```bash
lfy-cli login --user-key "your_user_key" --user-secret "your_user_secret"
```

## 第三步：验证

```bash
lfy-cli status
```

用于查看当前授权/绑定状态。完成后即可正常执行各品类命令。

## 常见错误

- 未授权 / 未登录就执行品类命令时，命令会提示需要先登录或凭证缺失；按上文完成授权与 `lfy-cli login` 即可。
