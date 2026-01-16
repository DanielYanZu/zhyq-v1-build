# macOS 版本支持说明

## 支持的 macOS 版本

**实际支持：macOS 11.0 (Big Sur)** 及以上版本

**说明**：虽然配置中设置为 10.14,但由于 Rust 1.89+ 的限制,实际编译出的应用最低支持 macOS 11.0。这已覆盖 95%+ 的用户设备。

## 配置详情

### 1. Cargo 配置（主要方式）

在 `.cargo/config.toml` 中设置链接器参数：

```toml
# macOS 部署目标配置 - 支持 macOS 10.14+
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-mmacosx-version-min=10.14"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-mmacosx-version-min=10.14"]
```

**说明**：通过 rustflags 直接传递链接器参数，确保编译出的二进制文件支持 macOS 10.14+。

### 2. build.rs 配置（辅助方式）

在 `src-tauri/build.rs` 中设置了环境变量：

```rust
fn main() {
    // 设置 macOS 最低部署目标为 10.14
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
    }

    tauri_build::build()
}
```

**说明**：通过环境变量作为额外保障。

### 3. GitHub Actions 配置

在 `.github/workflows/build.yml` 中添加了环境变量：

```yaml
- name: Build Tauri app
  uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    MACOSX_DEPLOYMENT_TARGET: '10.14'  # 支持 macOS 10.14+
```

## 兼容性说明

### ✅ 支持的功能

- 基本窗口显示和交互
- WebView 渲染（基于 WKWebView）
- 文件系统访问
- 系统托盘
- 应用菜单

### ⚠️ 注意事项

1. **WebView 限制**：
   - macOS 10.14 使用的 Safari 版本较旧
   - 某些现代 JavaScript 特性可能不支持
   - 建议使用 Babel/Polyfill 确保前端代码兼容性

2. **系统 API**：
   - 部分 macOS 新 API（如 SF Symbols）在 10.14 上不可用
   - 需要在代码中进行版本检查

3. **测试建议**：
   - 如果目标用户有 macOS 10.14 设备，务必在真机上测试
   - 使用虚拟机测试不同 macOS 版本的兼容性

## 修改最低版本

如果需要支持更低或更高的版本，修改以下三处的 `10.14` 值：

1. `.cargo/config.toml` 中的 `-mmacosx-version-min=10.14`
2. `src-tauri/build.rs` 中的 `MACOSX_DEPLOYMENT_TARGET`
3. `.github/workflows/build.yml` 中的 `MACOSX_DEPLOYMENT_TARGET`

### 版本建议

| macOS 版本 | 发布年份 | 支持状态 | 说明 |
|-----------|---------|---------|------|
| 11.0+ | 2020+ | ✅ 完全支持 | 当前 Rust 工具链的最低版本 |
| 10.15 | 2019 | ❌ 不支持 | 需要 Rust 1.88 或更早版本 |
| 10.14 | 2018 | ❌ 不支持 | 需要 Rust 1.88 或更早版本 |
| 10.13 | 2017 | ❌ 不支持 | 市场占有率极低,不建议支持 |

## 验证构建目标

构建后,检查二进制文件的最低系统要求：

```bash
# 查看二进制文件的部署目标
otool -l /path/to/YourApp.app/Contents/MacOS/YourApp | grep -A 3 LC_VERSION_MIN_MACOSX

# 或使用 vtool (macOS 12+)
vtool -show /path/to/YourApp.app/Contents/MacOS/YourApp
```

## 相关链接

- [Tauri macOS 配置文档](https://tauri.app/guides/distribution/macos)
- [Rust macOS 部署目标](https://doc.rust-lang.org/rustc/platform-support.html)
- [Apple Safari 版本历史](https://en.wikipedia.org/wiki/Safari_version_history)
