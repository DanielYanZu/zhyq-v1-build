# NazaPlus 模板仓库创建指南

## 📦 模板已准备完毕

模板文件位置: `/tmp/NazaPlus-build-template.tar.gz`

## 🚀 快速开始

### 步骤 1: 解压模板文件

```bash
cd /tmp
tar -xzf NazaPlus-build-template.tar.gz
cd NazaPlus-build-template
```

### 步骤 2: 在 GitHub 创建新仓库

1. 访问 https://github.com/new
2. 填写仓库信息:
   - **Repository name**: `NazaPlus-build-template` (可自定义)
   - **Description**: NazaPlus 云端打包模板仓库
   - **Public** (必须是公开仓库)
   - ✅ 勾选 **Template repository** (重要!)

3. 点击 "Create repository"

### 步骤 3: 推送代码到 GitHub

```bash
# 初始化 Git 仓库
git init
git add .
git commit -m "Initial commit: NazaPlus build template"

# 添加远程仓库 (替换为您的用户名和仓库名)
git remote add origin https://github.com/YOUR_USERNAME/NazaPlus-build-template.git

# 推送代码
git branch -M main
git push -u origin main
```

### 步骤 4: 在 NazaPlus 中配置

打开 NazaPlus 应用,进入"云端构建"页面,填写:

- **Owner**: 您的 GitHub 用户名 (例如: `YourUsername`)
- **Repo**: 您创建的仓库名称 (例如: `NazaPlus-build-template`)

## 📋 模板仓库包含的文件

```
NazaPlus-build-template/
├── .github/workflows/
│   └── build.yml           # GitHub Actions 构建流程
├── src/
│   ├── App.vue            # Vue 主组件
│   └── main.js            # 入口文件
├── src-tauri/
│   ├── src/
│   │   └── main.rs        # Rust 主程序
│   ├── Cargo.toml         # Rust 依赖配置
│   ├── tauri.conf.json    # Tauri 配置
│   └── build.rs           # 构建脚本
├── package.json           # Node.js 依赖
├── vite.config.js         # Vite 配置
├── index.html             # HTML 入口
├── .gitignore
└── README.md
```

## ⚙️ 工作原理

1. **模板生成**: NazaPlus 会从您的模板仓库创建新仓库
2. **配置注入**: 自动替换应用名称、版本、图标等配置
3. **触发构建**: 通过 GitHub Actions 在云端编译多平台安装包
4. **下载产物**: 构建完成后从 Releases 页面下载

## 📱 平台支持

### macOS 版本兼容性

**最低支持版本：macOS 11.0 (Big Sur)**

由于 Rust 工具链限制,实际编译出的应用支持 macOS 11.0+,覆盖 95%+ 的用户设备。

**详细配置说明**：请查看 [MACOS_SUPPORT.md](MACOS_SUPPORT.md)

### 其他平台

- **Windows**: 支持 Windows 10+ (x64 和 ARM64)
- **Linux**: 支持主流发行版 (x64 和 ARM64)


## 🔧 自定义模板

您可以根据需要修改模板:

### 修改构建平台

编辑 `.github/workflows/build.yml` 中的 `matrix.include` 部分

### 添加自定义依赖

编辑 `package.json` 和 `src-tauri/Cargo.toml`

### 修改默认样式

编辑 `src/App.vue` 中的样式

## ❓ 常见问题

### Q: 为什么必须设置为 Template repository?

A: 这样 NazaPlus 才能使用 GitHub 的模板功能快速创建新仓库。

### Q: 可以使用私有仓库吗?

A: 不可以,GitHub Actions 在私有仓库上有使用限制,且 NazaPlus 需要公开访问模板。

### Q: 构建失败怎么办?

A: 检查 GitHub Actions 日志,确保所有依赖配置正确。

## 📚 相关链接

- [NazaPlus 项目主页](https://github.com/Sjj1024/NazaPlus)
- [Tauri 官方文档](https://tauri.app/)
- [GitHub Actions 文档](https://docs.github.com/actions)

## 📄 许可证

MIT License
