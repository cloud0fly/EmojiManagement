# 表情仓库管理 (Emoji Management)

一个基于 Tauri + Vue 3 + TypeScript + Rust 构建的跨平台桌面表情管理应用。支持表情的导入、分类、拖拽排序和快速复制功能。

![版本](https://img.shields.io/badge/版本-0.5.0-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.x-9933FF)
![Vue](https://img.shields.io/badge/Vue-3.5-4FC08D)
![Rust](https://img.shields.io/badge/Rust-2021-orange)

## ✨ 功能特性

- 📦 **表情管理** - 支持导入和管理各类表情包图片
- 📂 **分组系统** - 自定义表情分组，支持拖拽排序
- 🖼️ **格式支持** - 支持 GIF、PNG、JPG 等多种图片格式
- 🔍 **智能去重** - 基于 MD5 哈希值自动检测重复图片
- 📋 **快速复制** - 一键复制表情到剪贴板
- 🎨 **现代 UI** - 采用 TailwindCSS 打造的简洁美观界面
- 🚀 **跨平台** - 支持 Windows、macOS、Linux
- 📱 **系统托盘** - 最小化到托盘，快速唤醒
- 🗄️ **本地存储** - 使用 SQLite 数据库，数据安全可靠

## 📦 安装与开发

### 环境要求

- Node.js 18+ 
- pnpm 8+
- Rust 1.70+
- 系统依赖参考 [Tauri 文档](https://tauri.app/start/prerequisites/)

### 克隆项目

```bash
git clone <repository-url>
cd EmojiManagement
```

### 安装依赖

```bash
# 安装前端依赖
pnpm install
```

### 开发模式

```bash
# 启动开发服务器
pnpm tauri dev
```

### 构建发布版本

```bash
# 构建生产版本
pnpm tauri build
```

构建完成后，可执行文件位于 `src-tauri/target/release/` 目录。

## 🎯 核心功能说明

### 表情导入
- 支持单张图片导入
- 支持文件夹批量导入
- 自动检测文件格式 (GIF/PNG/JPG 等)
- 智能去重 (基于 MD5 哈希)
- 拖拽文件到窗口快速导入

### 分组管理
- 创建/重命名/删除分组
- 分组拖拽排序
- 表情跨分组移动
- 默认分组：我的表情

### 表情操作
- 拖拽排序
- 一键复制到剪贴板

### 系统托盘
- 点击托盘图标打开/隐藏窗口
- 托盘菜单：打开、退出
- 关闭窗口时自动隐藏到托盘

## 🗄️ 数据存储

应用使用 SQLite 数据库存储表情元数据：

- **数据库位置**: 系统应用数据目录
- **图片存储**: 应用数据目录的 `original` 文件夹
- **数据表**: 
  - `memes` - 表情信息表
  - `categories` - 分组信息表

## ⌨️ Tauri 命令 (Rust ↔ Vue)

| 命令 | 说明 |
|------|------|
| `import_single_image` | 导入单张图片 |
| `import_image_from_folder` | 从文件夹批量导入 |
| `get_memes_by_category` | 获取指定分组的表情 |
| `update_memes_order` | 更新表情排序 |
| `copy_image_to_clipboard` | 复制图片到剪贴板 |
| `create_category` | 创建新分组 |
| `rename_category` | 重命名分组 |
| `delete_category` | 删除分组 |

## 🎨 UI 设计

- **配色方案**: 清新简约的蓝白色调
- **响应式布局**: 适配不同窗口尺寸
- **动画效果**: 平滑的过渡和淡入淡出
- **图标系统**: Emoji + SVG 图标结合
- **侧边栏**: 支持折叠/展开

## 📝 开发注意事项

### IDE 推荐配置

- **VS Code** + 以下插件:
  - Vue - Official
  - Tauri
  - rust-analyzer
  - Tailwind CSS IntelliSense

### 调试技巧

- 前端：使用浏览器 DevTools (开发模式下)
- 后端：查看日志输出 (stdout 或 日志文件)
- 日志级别：Debug (可在 `lib.rs` 中调整)

### 常见问题

1. **Rust 编译错误**: 确保 Rust 版本 >= 1.70
2. **Tauri 启动失败**: 检查系统依赖是否安装完整
3. **图片导入失败**: 检查文件权限和路径

## 📄 许可证

本项目采用 Apache 许可证。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📧 联系方式

如有问题或建议，请通过 Issue 联系作者。

