# 足球比赛分组系统

一个足球分组 & 比赛记录工具，支持按位置/能力值智能随机分队，纯静态前端，数据保存在浏览器 localStorage。

## 项目结构

```
football-group/
├── web/                        # Vue 3 + TypeScript 前端
│   ├── public/
│   │   ├── players.csv         # 初始球员数据（首次加载自动导入）
│   │   └── reports.csv         # 小作文（静态文件，构建时打包）
│   ├── src/
│   │   ├── api/index.ts        # 数据层（localStorage）
│   │   ├── lib/storage.ts      # localStorage 工具
│   │   ├── stores/             # Pinia stores
│   │   ├── components/         # Vue 组件
│   │   └── utils/              # OVR 计算、位置工具
│   ├── package.json
│   └── vite.config.ts
├── players.csv                 # 球员数据源（同步到 web/public/）
├── 小作文.csv                  # 小作文数据源（同步到 web/public/reports.csv）
└── src/                        # Rust 后端（已弃用，仅作备份）
```

## 快速开始

### 依赖

- [Node.js](https://nodejs.org/) 18+

### 安装

```bash
cd web && npm install
```

### 本地开发

```bash
cd web && npm run dev
```

打开 **http://localhost:5173**，支持热更新。

### 构建生产版本

```bash
cd web && npm run build
```

产物输出到 `web/dist/`，把该目录部署到任意静态托管服务即可。

### 本地预览构建产物

```bash
cd web && npm run preview
```

---

## 更新数据

### 更新球员名单

编辑根目录的 `players.csv`，格式如下（支持多行同名以设置多位置）：

```
name,position,pac,sho,pas,dri,def,phy
张三,fw,75,80,60,70,40,65
李四,mf,65,60,75,65,55,65
李四,df,60,45,60,55,75,70   ← 同名第二行 = 第二位置
```

位置取值：`fw`（前锋）、`mf`（中场）、`df`（后卫）、`gk`（守门员）

> **注意**：`players.csv` 只在浏览器 localStorage **为空时**自动导入一次。
> 若要重新导入，在浏览器 DevTools 中清除 localStorage（以 `fg_` 开头的键），刷新页面即可。

然后同步到 `web/public/` 并重新构建：

```bash
cp players.csv web/public/players.csv
cd web && npm run build
```

### 更新小作文

编辑 `小作文.csv`（格式：`日期时间,字数,内容`），同步并构建：

```bash
cp 小作文.csv web/public/reports.csv
cd web && npm run build
```

---

## 部署

构建完成后将 `web/dist/` 目录部署到以下任意平台：

| 平台 | 方式 |
|------|------|
| **Netlify** | 拖拽 `dist/` 上传，或连接仓库设置构建命令 `npm run build`、发布目录 `web/dist` |
| **Vercel** | 同上，Root Directory 设置为 `web` |
| **GitHub Pages** | 将 `dist/` 内容推送到 `gh-pages` 分支 |
| **任意 Web 服务器** | 将 `dist/` 内容放到 nginx / Caddy 静态目录 |

---

## 数据说明

所有数据（球员、比赛记录）保存在**浏览器 localStorage**（键名前缀 `fg_`），头像以 base64 格式内嵌存储。

- 数据仅在当前浏览器可见，换设备/换浏览器不会同步
- localStorage 容量约 5MB，对球员名单和比赛记录完全够用
- 清空 localStorage 会丢失所有录入数据，下次刷新会重新从 `players.csv` 导入初始球员
