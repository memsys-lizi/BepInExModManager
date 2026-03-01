# BepInEx Mod Manager

一个仿照 UnityModManager 设计的 BepInEx 模组管理器，基于 Tauri 2 + Vue 3 + Rust 构建。支持多游戏管理、BepInEx 自动下载安装、Mod 安装与启用/禁用、配置文件可视化编辑等功能。

---

## 功能概览

- **多游戏管理**：添加任意数量的游戏，各自独立管理
- **BepInEx 管理**：一键下载并安装 BepInEx（支持 x64 / x86），检测安装状态与完整性，支持重装和卸载
- **Mod 管理**：以文件夹为单位识别 Mod，支持启用/禁用（通过重命名 dll 实现）、删除、查看内部 dll 列表
- **Mod 安装**：拖拽 `.zip` / `.dll` 文件安装，或输入直链 URL 下载安装，自动处理命名冲突
- **配置文件编辑**：可视化编辑 BepInEx 及各 Mod 的 `.cfg` 配置文件，自动保存，内置 `BepInEx.cfg` 中文字段说明
- **游戏启动**：直接从软件内启动游戏
- **主题切换**：深色 / 浅色 / 跟随系统
- **国际化**：支持中文 / 英文界面切换
- **HTTP 代理**：支持配置代理，解决国内 GitHub 下载慢的问题

---

## 使用方法

### 1. 添加游戏

1. 点击左侧边栏顶部的 **首页**
2. 点击 **添加游戏** 卡片，在弹出的文件选择框中直接选择游戏的 `.exe` 可执行文件
3. 软件会自动识别游戏名称和可执行文件名，确认后点击 **添加**
4. 游戏卡片出现在首页和左侧边栏

> 若需要移除游戏，将鼠标悬停在游戏卡片上，点击右上角的删除按钮即可（不会删除游戏文件）。

---

### 2. 安装 BepInEx

1. 点击侧栏中的目标游戏，进入游戏主页
2. 点击顶部状态栏中的 **BepInEx** 按钮，或点击 **去安装** 进入 BepInEx 管理页
3. 根据提示选择版本：
   - **x64**：适用于绝大多数现代游戏（64 位进程）
   - **x86**：仅当游戏为 32 位进程时选择，不确定优先选 x64
4. 点击 **安装**，等待下载和安装完成
5. 安装完成后返回游戏主页，状态栏显示绿点即为已安装

> 如果下载速度慢，可在 **设置 → HTTP 代理** 中配置本地代理（例如 `http://127.0.0.1:7890`）。

---

### 3. 管理 Mod

进入游戏主页后，下方会列出所有已安装的 Mod（扫描 `BepInEx/plugins` 目录下的文件夹）。

| 操作 | 方法 |
|------|------|
| 启用 / 禁用 Mod | 点击 Mod 行左侧的开关 |
| 查看内部 dll | 点击 Mod 名称左侧的展开箭头 |
| 删除 Mod | 悬停 Mod 行，点击右侧垃圾桶图标 |
| 刷新列表 | 点击顶栏 **刷新** 按钮 |
| 打开插件目录 | 点击顶栏 **打开目录** |

---

### 4. 安装新 Mod

**方式一：拖拽安装**

将 `.zip` 压缩包或 `.dll` 文件直接拖拽到软件窗口上，松开即可安装：
- 拖入 `.zip`：解压后在 plugins 目录下创建同名文件夹
- 拖入 `.dll`：在 plugins 目录下创建同名文件夹并放入 dll

**方式二：本地文件选择**

点击顶栏 **安装 Mod** 按钮，在文件选择框中选择 `.zip` 或 `.dll` 文件。

**方式三：URL 下载安装**

点击顶栏 **URL 安装** 按钮，输入 `.zip` 或 `.dll` 文件的直链地址，软件自动下载并安装。

**命名冲突处理**

若安装时 plugins 目录中已存在同名文件夹，会弹出冲突对话框，提供三种选择：
- **自动重命名**：在原名称后追加 `_1`、`_2` 等后缀
- **覆盖**：删除旧文件夹并替换为新内容
- **取消**：放弃本次安装

---

### 5. 编辑配置文件

1. 在游戏主页点击顶部状态栏的 **配置文件** 按钮
2. 左侧列出所有 `.cfg` 文件（BepInEx 本体及各 Mod）
3. 点击文件名进入编辑界面，修改后自动保存（无需手动点保存）
4. 打开 `BepInEx.cfg` 时，每个配置项下方会显示中文说明

---

### 6. 启动游戏

在游戏主页点击顶栏右侧的 **启动游戏** 按钮，直接从软件内启动对应游戏。

---

### 7. 设置

点击左侧边栏底部的 **设置** 进入设置页：

| 设置项 | 说明 |
|--------|------|
| BepInEx 下载源 | GitHub 官方 或 镜像源（国内加速） |
| HTTP 代理 | 格式：`http://127.0.0.1:7890`，留空不使用 |
| 界面语言 | 中文 / English |
| 主题 | 深色 / 浅色 / 跟随系统 |

---

## 开发

### 环境要求

| 工具 | 版本要求 | 说明 |
|------|----------|------|
| [Node.js](https://nodejs.org/) | ≥ 18 | 前端运行环境 |
| [pnpm](https://pnpm.io/) | ≥ 8 | 包管理器 |
| [Rust](https://www.rust-lang.org/) | stable | 后端编译 |
| [Tauri CLI](https://tauri.app/) | 2.x | 由 pnpm 脚本调用，无需全局安装 |
| Visual Studio Build Tools | 2019+ | Windows 编译 Rust 所需的 MSVC 工具链 |

> Windows 用户还需安装 [WebView2](https://developer.microsoft.com/microsoft-edge/webview2/)（Win11 已内置）。

---

### 克隆与安装依赖

```bash
git clone https://github.com/memsys-lizi/BepInExModManager.git
cd BepInExModManager
pnpm install
```

---

### 开发模式启动

```bash
pnpm tauri dev
```

- 前端（Vite）热重载，修改 `.vue` 文件后界面即时更新
- Rust 后端修改后会自动重新编译并重启应用

---

### 项目结构

```
BepInExModManager/
├── src/                        # 前端 Vue 源码
│   ├── api/                    # Tauri 命令封装（invoke 调用层）
│   ├── assets/                 # 全局 CSS、字体
│   ├── components/
│   │   ├── layout/             # 布局组件（标题栏、侧栏、顶栏）
│   │   └── ui/                 # 通用 UI 组件（按钮、开关、弹窗等）
│   ├── i18n/                   # 国际化翻译文件（zh.ts / en.ts）
│   ├── pages/                  # 页面组件
│   │   ├── HomePage.vue        # 游戏列表首页
│   │   ├── GameDetailPage.vue  # 游戏 Mod 管理页
│   │   ├── BepInExPage.vue     # BepInEx 安装管理页
│   │   ├── GameConfigPage.vue  # 配置文件编辑页
│   │   └── SettingsPage.vue    # 设置页
│   ├── router/                 # Vue Router 路由配置
│   ├── store/                  # Pinia 状态管理
│   └── types/                  # TypeScript 类型定义
│
└── src-tauri/                  # Rust 后端源码
    ├── icons/                  # 应用图标（由 pnpm tauri icon 生成）
    ├── src/
    │   ├── commands/
    │   │   ├── game.rs         # 游戏管理命令（图标提取、启动游戏等）
    │   │   ├── bepinex.rs      # BepInEx 管理命令（状态检测、下载安装）
    │   │   ├── mods.rs         # Mod 管理命令（扫描、启用禁用、安装）
    │   │   └── config.rs       # 配置文件读写命令
    │   ├── models/             # 共享数据结构定义
    │   └── lib.rs              # 命令注册入口
    └── tauri.conf.json         # Tauri 配置文件
```

---

### 构建打包

```bash
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`：

```
bundle/
├── msi/        # Windows MSI 安装包
└── nsis/       # Windows NSIS 安装程序（.exe）
```

> 首次构建需编译所有 Rust 依赖，耗时约 10~20 分钟，后续增量构建速度较快。

---

### 更换应用图标

准备一张 **1024×1024 或更大的正方形 PNG**，执行：

```bash
pnpm tauri icon your-icon.png
```

命令会自动生成 `src-tauri/icons/` 下所有平台所需的图标文件。

---

## 技术栈

| 层 | 技术 |
|----|------|
| 桌面框架 | [Tauri 2](https://tauri.app/) |
| 前端框架 | [Vue 3](https://vuejs.org/) + TypeScript |
| 构建工具 | [Vite](https://vitejs.dev/) |
| 状态管理 | [Pinia](https://pinia.vuejs.org/) |
| 路由 | [Vue Router 4](https://router.vuejs.org/) |
| 图标库 | [Lucide Vue Next](https://lucide.dev/) |
| 后端语言 | [Rust](https://www.rust-lang.org/) |
| HTTP 客户端 | [reqwest](https://github.com/seanmonstar/reqwest) |
| 压缩包处理 | [zip](https://github.com/zip-rs/zip2) |
| 异步运行时 | [Tokio](https://tokio.rs/) |
