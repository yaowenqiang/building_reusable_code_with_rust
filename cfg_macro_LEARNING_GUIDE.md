# Rust 条件编译学习指南

## 🎯 学习目标

通过本指南，您将系统性地掌握 Rust 条件编译技术，成为一名能够编写高质量跨平台代码的 Rust 开发者。

## 📚 学习路径

### 第一阶段：基础概念（30分钟）

#### 1.1 理解条件编译的意义

**为什么需要条件编译？**
- **跨平台需求**: 不同的操作系统有不同的API和特性
- **性能优化**: 根据目标平台选择最优算法
- **功能控制**: 根据编译模式启用/禁用功能
- **代码组织**: 避免复杂的运行时判断

**条件编译 vs 运行时检查**
```rust
// 条件编译：编译时决定，零运行时开销
#[cfg(target_os = "linux")]
fn linux_specific() {
    // 这段代码只在 Linux 上存在
}

// 运行时检查：运行时判断，有微小开销
fn runtime_check() {
    if cfg!(target_os = "linux") {
        // 这段代码在所有平台上都存在
    }
}
```

#### 1.2 核心语法学习

**基本语法结构**
```rust
// 属性语法：控制代码编译
#[cfg(condition)]
fn function_name() {}

// 宏语法：运行时检查
if cfg!(condition) {
    // 条件成立时执行的代码
}
```

**常用条件类型**
- `target_os`: 目标操作系统
- `target_arch`: 目标架构
- `target_pointer_width`: 指针宽度
- `target_family`: 系统家族（unix、windows）
- `debug_assertions`: 是否调试模式

### 第二阶段：实践操作（45分钟）

#### 2.1 运行演示程序

```bash
# 编译并运行（调试模式）
rustc cfg_macro.rs && ./cfg_macro

# 编译并运行（发布模式）
rustc -O cfg_macro.rs -o cfg_macro_release && ./cfg_macro_release
```

**观察要点**：
1. 两种模式的输出差异
2. 当前平台的识别结果
3. 条件编译的效果

#### 2.2 修改和实验

**实验1：添加新平台支持**
```rust
#[cfg(target_os = "freebsd")]
pub struct FreeBSDCode {
    version: String,
}

#[cfg(target_os = "freebsd")]
impl FreeBSDCode {
    pub fn new() -> Self {
        Self {
            version: "FreeBSD Edition".to_string(),
        }
    }
}
```

**实验2：复杂条件组合**
```rust
#[cfg(all(unix, any(target_arch = "x86_64", target_arch = "aarch64")))]
pub struct ModernUnixCode {
    capabilities: Vec<String>,
}
```

#### 2.3 创建自己的条件编译项目

**项目：跨平台文件监控器**

```rust
// file_monitor.rs
use std::fs;
use std::path::Path;

#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt;

#[cfg(target_os = "macos")]
use std::os::macos::fs::MetadataExt;

#[cfg(target_os = "windows")]
use std::os::windows::fs::MetadataExt;

pub struct FileMonitor {
    #[cfg(target_os = "linux")]
    use_inotify: bool,

    #[cfg(target_os = "macos")]
    use_fsevents: bool,

    #[cfg(target_os = "windows")]
    use_read_directory_changes: bool,
}

impl FileMonitor {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "linux")]
            use_inotify: true,

            #[cfg(target_os = "macos")]
            use_fsevents: true,

            #[cfg(target_os = "windows")]
            use_read_directory_changes: true,
        }
    }

    pub fn get_backend(&self) -> &'static str {
        if cfg!(target_os = "linux") {
            "inotify"
        } else if cfg!(target_os = "macos") {
            "FSEvents"
        } else if cfg!(target_os = "windows") {
            "ReadDirectoryChangesW"
        } else {
            "polling"
        }
    }
}
```

### 第三阶段：深入理解（60分钟）

#### 3.1 条件编译的工作原理

**编译流程图**：
```
源代码 → 条件评估 → 代码过滤 → 生成中间代码 → 编译 → 二进制文件

#[cfg(target_os = "linux")] 条件
↓
目标平台是否为 Linux？
├─ 是 → 包含代码
└─ 否 → 删除代码
```

**内存布局影响**：
```rust
// 在 macOS 上编译后的内存布局
struct CompiledCode {
    macos_code: MacOSCode,        // 存在
    // linux_code: LinuxCode,      // 不存在，不占用内存
    // windows_code: WindowsCode,  // 不存在，不占用内存
    common_code: CrossPlatformCode, // 存在
}
```

#### 3.2 高级条件组合

**all() - 所有条件都必须满足**
```rust
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn linux_64_optimized() {
    // 只在 64 位 Linux 上编译
}
```

**any() - 任一条件满足即可**
```rust
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn bsd_like_system() {
    // 在 Linux 或 FreeBSD 上编译
}
```

**not() - 条件取反**
```rust
#[cfg(not(target_os = "windows"))]
fn non_windows_system() {
    // 在非 Windows 系统上编译
}
```

**复杂嵌套组合**
```rust
#[cfg(all(
    unix,
    not(target_os = "macos"),
    any(
        all(target_arch = "x86_64", target_pointer_width = "64"),
        all(target_arch = "aarch64", target_pointer_width = "64")
    )
))]
fn specific_unix_64bit() {
    // Unix 系统 + 非 macOS + 64 位架构
}
```

### 第四阶段：最佳实践（45分钟）

#### 4.1 代码组织策略

**分层架构**：
```rust
// platform/mod.rs
pub mod linux;
pub mod macos;
pub mod windows;
pub mod common;

// 使用条件编译重新导出
#[cfg(target_os = "linux")]
pub use linux::PlatformSpecific as PlatformImpl;

#[cfg(target_os = "macos")]
pub use macos::PlatformSpecific as PlatformImpl;

#[cfg(target_os = "windows")]
pub use windows::PlatformSpecific as PlatformImpl;
```

**特性门控（Feature Flags）**：
```toml
# Cargo.toml
[features]
default = ["basic"]
basic = []
advanced = ["networking", "database"]
networking = []
database = []
```

```rust
#[cfg(feature = "networking")]
mod networking {
    pub fn connect() {
        println!("Network functionality enabled");
    }
}

#[cfg(feature = "database")]
mod database {
    pub fn connect() {
        println!("Database functionality enabled");
    }
}
```

#### 4.2 性能优化技巧

**零成本抽象**：
```rust
// 优化前：运行时检查
pub fn get_optimal_implementation() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux_optimized"
    } else if cfg!(target_os = "macos") {
        "macos_optimized"
    } else {
        "generic"
    }
}

// 优化后：编译时选择
#[cfg(target_os = "linux")]
pub const OPTIMAL_IMPL: &str = "linux_optimized";

#[cfg(target_os = "macos")]
pub const OPTIMAL_IMPL: &str = "macos_optimized";

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub const OPTIMAL_IMPL: &str = "generic";
```

**内联优化**：
```rust
#[cfg(target_os = "linux")]
#[inline]
fn linux_syscall() -> i32 {
    // Linux 特定的系统调用
    unsafe { libc::syscall(...) }
}
```

#### 4.3 错误处理策略

**优雅降级**：
```rust
#[cfg(target_os = "linux")]
fn get_file_info_linux(path: &Path) -> Result<FileInfo, Error> {
    // Linux 特定的实现
    Ok(FileInfo { inode: file.st_ino() })
}

#[cfg(not(target_os = "linux"))]
fn get_file_info_fallback(path: &Path) -> Result<FileInfo, Error> {
    // 通用实现，功能有限
    Ok(FileInfo { inode: 0 })
}

// 统一接口
pub fn get_file_info(path: &Path) -> Result<FileInfo, Error> {
    #[cfg(target_os = "linux")]
    return get_file_info_linux(path);

    #[cfg(not(target_os = "linux"))]
    return get_file_info_fallback(path);
}
```

### 第五阶段：高级应用（60分钟）

#### 5.1 构建系统集成

**Cargo 配置示例**：
```toml
# Cargo.toml
[package]
name = "cross-platform-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# 平台特定依赖
[target.'cfg(target_os = "linux")'.dependencies]
inotify = "0.10"

[target.'cfg(target_os = "macos")'.dependencies]
fsevent = "0.4"

[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["fileapi"] }

# 条件依赖
[dependencies.tokio]
version = "1.0"
features = ["full"]

[target.'cfg(not(target_os = "windows"))'.dependencies]
tokio.features = ["fs", "io-util"]

[target.'cfg(target_os = "windows")'.dependencies]
tokio.features = ["fs", "io-util", "windows-sys"]
```

**构建脚本**：
```rust
// build.rs
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // 根据目标平台设置环境变量
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "linux" {
        println!("cargo:rustc-cfg=linux_specific");
    }

    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "x86_64" {
        println!("cargo:rustc-cfg=x86_64_optimized");
    }
}
```

#### 5.2 测试策略

**平台特定测试**：
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_specific() {
        // Linux 特定功能的测试
        assert!(cfg!(target_os = "linux"));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_specific() {
        // macOS 特定功能的测试
        assert!(cfg!(target_os = "macos"));
    }

    #[test]
    fn test_cross_platform() {
        // 跨平台功能的测试
        let code = CrossPlatformCode::new();
        assert!(!code.platform.is_empty());
    }
}
```

**集成测试**：
```rust
// tests/integration_test.rs
use cfg_macro::CrossPlatformCode;

#[test]
fn test_platform_detection() {
    let platform = CrossPlatformCode::new();

    match std::env::consts::OS {
        "linux" => assert!(platform.supports_feature("epoll")),
        "macos" => assert!(platform.supports_feature("metal")),
        "windows" => assert!(platform.supports_feature("win32")),
        _ => {} // 其他平台的处理
    }
}
```

#### 5.3 调试和诊断

**宏展开查看**：
```bash
# 安装 cargo-expand
cargo install cargo-expand

# 查看条件编译结果
cargo expand --target x86_64-unknown-linux-gnu
cargo expand --target x86_64-pc-windows-msvc
cargo expand --target x86_64-apple-darwin
```

**编译时诊断**：
```rust
#[cfg(target_os = "linux")]
compile_error!("Linux support is currently experimental");

#[cfg(all(target_pointer_width = "32", feature = "heavy-computation"))]
compile_error!("Heavy computation requires 64-bit architecture");
```

**调试宏**：
```rust
macro_rules! cfg_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

fn debug_function() {
    cfg_debug!("This is a debug message: {}", 42);
}
```

## 🛠️ 实用工具和资源

### 开发工具
- **cargo-expand**: 查看宏展开结果
- **cargo tree**: 分析依赖关系
- **rustc --print target-list**: 查看支持的目标平台

### 参考资料
- [Rust Reference - Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [The Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Rust by Example - cfg!](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html)

### 社区资源
- [crossbeam](https://github.com/crossbeam-rs/crossbeam): 跨平台并发库
- [tokio](https://github.com/tokio-rs/tokio): 异步运行时
- [serde](https://github.com/serde-rs/serde): 序列化框架

## 📋 学习检查清单

### 基础能力
- [ ] 理解条件编译 vs 运行时检查的区别
- [ ] 掌握 `#[cfg(...)]` 属性的用法
- [ ] 学会使用 `cfg!` 宏
- [ ] 了解常用条件类型

### 进阶能力
- [ ] 掌握 `all()`, `any()`, `not()` 组合
- [ ] 学会使用特性门控
- [ ] 理解条件编译的性能影响
- [ ] 能够设计跨平台抽象

### 高级能力
- [ ] 掌握构建系统集成
- [ ] 学会编写平台特定测试
- [ ] 能够调试条件编译问题
- [ ] 理解条件编译的底层原理

## 🎯 项目实践建议

### 初级项目
1. **跨平台文件操作工具**
   - 实现基本的文件读写
   - 使用条件编译处理路径差异
   - 支持不同操作系统的权限检查

2. **系统信息收集器**
   - 收集CPU、内存信息
   - 显示平台特定的特性
   - 生成系统报告

### 中级项目
1. **跨平台网络监控器**
   - 实现网络连接监控
   - 使用不同平台的API
   - 提供统一的接口

2. **条件编译配置管理器**
   - 支持多种配置源
   - 平台特定的配置选项
   - 运行时配置验证

### 高级项目
1. **跨平台异步运行时抽象层**
   - 统一的异步API
   - 平台特定的事件循环
   - 性能优化和测试

2. **条件编译代码生成器**
   - 自动生成平台适配代码
   - 配置驱动的代码生成
   - 集成到构建流程

---

**学习建议**:
1. 循序渐进，先掌握基础语法
2. 多动手实践，修改示例代码
3. 关注性能，理解零成本抽象
4. 阅读优秀的跨平台项目源码
5. 参与社区讨论，分享经验

🦀 **祝您学习愉快，成为跨平台开发专家！**