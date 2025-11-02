# Rust 条件编译完整指南

## 📖 项目概述

这个项目全面演示了 Rust 条件编译的各种用法，包括 `#[cfg(...)]` 属性和 `cfg!` 宏的使用。条件编译是 Rust 跨平台开发的核心技术，允许开发者根据不同的目标平台、架构、编译模式等条件来控制代码的编译和执行。

## 🎯 学习目标

通过本项目，您将学会：
- 理解条件编译的基本概念和用途
- 掌握 `#[cfg(...)]` 属性的使用方法
- 学会使用 `cfg!` 宏进行运行时条件检查
- 了解复杂条件组合的技巧
- 掌握跨平台开发的核心技术

## 🚀 快速开始

```bash
# 编译并运行（debug 模式）
rustc cfg_macro.rs && ./cfg_macro

# 编译并运行（release 模式）
rustc -O cfg_macro.rs -o cfg_macro_release && ./cfg_macro_release

# 查看所有可用选项
rustc --help
```

## 📚 核心概念

### 1. 条件编译 vs 运行时检查

| 特性 | `#[cfg(...)]` | `cfg!` |
|------|---------------|--------|
| **执行时机** | 编译时 | 运行时 |
| **代码大小** | 不满足条件的代码不会编译 | 所有代码都会编译 |
| **性能影响** | 无运行时开销 | 有微小运行时开销 |
| **用途** | 平台特定代码、功能开关 | 动态行为调整、调试信息 |

### 2. 支持的条件类型

#### 系统相关条件
- `target_os = "linux"` - 目标操作系统
- `target_arch = "x86_64"` - 目标架构
- `target_pointer_width = "64"` - 指针宽度
- `target_endian = "little"` - 字节序
- `target_family = "unix"` - 系统家族

#### 编译相关条件
- `debug_assertions` - 是否启用调试断言
- `feature = "name"` - 自定义功能特性
- `test` - 是否在测试模式下编译

#### 布尔条件
- `unix` - Unix 系统（Linux、macOS）
- `windows` - Windows 系统
- `target_pointer_width = "64"` - 64位系统

## 🔧 语法详解

### 基本语法

```rust
// 条件编译属性
#[cfg(target_os = "linux")]
fn linux_only_function() {
    println!("This function only compiles on Linux");
}

// 运行时检查宏
fn runtime_check() {
    if cfg!(target_os = "linux") {
        println!("Running on Linux");
    }
}
```

### 条件组合

```rust
// 所有条件都必须满足
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
fn linux_64_only() {}

// 任一条件满足即可
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_like() {}

// 条件取反
#[cfg(not(target_os = "windows"))]
fn non_windows() {}

// 复杂组合
#[cfg(all(
    unix,
    not(target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
fn specific_unix_system() {}
```

## 📋 项目结构

```
cfg_macro.rs - 主演示文件
├── 条件编译属性示例
│   ├── 平台特定结构体（Linux、macOS）
│   ├── 跨平台代码示例
│   └── 调试/发布模式代码
├── cfg! 宏运行时检查示例
├── 系统信息展示
└── 高级条件组合示例
```

## 🎓 示例解析

### 1. 平台特定代码

```rust
#[cfg(target_os = "linux")]
pub struct LinuxCode {
    version: String,
}

#[cfg(target_os = "macos")]
pub struct MacOSCode {
    version: String,
}
```

**说明**：这些结构体只会在对应的操作系统下被编译，在其他平台上完全不存在。

### 2. 条件编译的函数调用

```rust
fn main() {
    if cfg!(target_os = "macos") {
        #[cfg(target_os = "macos")]
        {
            let macos_code = MacOSCode::new();
            println!("Created macOS code: {}", macos_code.version);
        }
    }
}
```

**关键点**：必须使用嵌套的条件编译块，确保类型存在时才使用。

### 3. 调试/发布模式

```rust
#[cfg(debug_assertions)]
fn debug_info() {
    println!("Debug mode enabled");
}

#[cfg(not(debug_assertions))]
fn release_info() {
    println!("Release mode optimized");
}
```

**用途**：调试信息、性能检查、开发工具等。

## 🛠️ 实用技巧

### 1. 平台抽象层

```rust
// 创建平台抽象
#[cfg(target_os = "linux")]
pub use LinuxImpl as PlatformImpl;

#[cfg(target_os = "macos")]
pub use MacOSImpl as PlatformImpl;

#[cfg(target_os = "windows")]
pub use WindowsImpl as PlatformImpl;
```

### 2. 功能标志（Feature Flags）

```rust
// 在 Cargo.toml 中定义
// [features]
// default = ["basic"]
// advanced = []

#[cfg(feature = "advanced")]
fn advanced_features() {
    println!("Advanced functionality enabled");
}

#[cfg(not(feature = "advanced"))]
fn basic_features() {
    println!("Basic functionality only");
}
```

### 3. 测试专用代码

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_specific_test() {
        // 只在测试中运行的代码
        assert!(cfg!(target_pointer_width = "64"));
    }
}
```

## 🔍 调试技巧

### 1. 查看条件编译结果

```bash
# 使用 cargo-expand 查看宏展开（需要安装）
cargo install cargo-expand
cargo expand

# 或者查看编译器输出
rustc --emit asm cfg_macro.rs -o cfg_macro.s
```

### 2. 编译时调试信息

```rust
// 编译时打印信息（需要 nightly Rust）
#![feature(compiler_builtins)]
#[cfg(target_os = "linux")]
compile_error!("This code is Linux only!");

// 或者使用宏来输出编译信息
macro_rules! cfg_println {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($tt)*);
    };
}
```

### 3. 条件检查工具函数

```rust
fn check_environment() {
    println!("Environment check:");
    println!("  OS: {}", std::env::consts::OS);
    println!("  Arch: {}", std::env::consts::ARCH);
    println!("  Family: {}", std::env::consts::FAMILY);
    println!("  Pointer width: {}", cfg!(target_pointer_width = "64"));
    println!("  Debug: {}", cfg!(debug_assertions));
}
```

## 📊 运行示例

### Debug 模式输出
```
=== 条件编译和 cfg! 宏演示 ===

1. cfg! 宏运行时检查：
→ 运行 macOS 特定的代码
→ 创建了 macOS 专用代码: macOS Edition

2. 跨平台代码示例：
Hello from macos platform!

3. 当前目标系统信息：
   操作系统: macos
   架构: aarch64
   系列家族: unix

4. 条件编译功能演示：
   是否为 64 位系统: true
   是否为 Unix 系统: true
   是否为 Windows 系统: false
   是否为调试模式: true
   运行在 ARM64/aarch64 架构上

5. 条件编译函数调用：
这是调试模式下的信息
启用了调试功能

=== 演示完成 ===
```

### Release 模式输出
```
=== 条件编译和 cfg! 宏演示 ===

1. cfg! 宏运行时检查：
→ 运行 macOS 特定的代码
→ 创建了 macOS 专用代码: macOS Edition

2. 跨平台代码示例：
Hello from macos platform!

3. 当前目标系统信息：
   操作系统: macos
   架构: aarch64
   系列家族: unix

4. 条件编译功能演示：
   是否为 64 位系统: true
   是否为 Unix 系统: true
   是否为 Windows 系统: false
   是否为调试模式: false  ← 不同！
   运行在 ARM64/aarch64 架构上

5. 条件编译函数调用：
这是发布模式下的信息    ← 不同！
优化版本正在运行

=== 演示完成 ===
```

## 🎯 最佳实践

### 1. 代码组织
- 将平台特定代码放在单独的模块中
- 使用条件编译创建统一的接口
- 避免条件代码过度嵌套

### 2. 性能考虑
- 优先使用 `#[cfg(...)]` 而不是运行时检查
- 对于频繁调用的代码，避免使用 `cfg!` 宏
- 合理使用功能标志控制二进制大小

### 3. 可维护性
- 为复杂的条件组合添加注释
- 使用有意义的名称组织条件代码
- 定期测试不同平台和配置

### 4. 错误处理
- 提供清晰的错误消息
- 为不支持的平台给出明确提示
- 考虑提供默认实现或降级方案

## 📖 进阶学习

### 相关文档
- [Rust Reference - Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [The Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Rust by Example - cfg!](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html)

### 实际项目参考
- [std::arch](https://doc.rust-lang.org/std/arch/) - 平台特定指令集
- [crossbeam](https://github.com/crossbeam-rs/crossbeam) - 跨平台并发库
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时

### 练习项目
1. 实现一个跨平台的文件监控器
2. 创建支持多操作系统的系统信息工具
3. 开发一个条件编译的性能基准测试库

## ❓ 常见问题

### Q1: 为什么 `cfg!(debug_assertions)` 在 release 模式下返回 false？
A1: 这是 Rust 的标准行为。Release 模式默认不包含调试断言，因此 `debug_assertions` 为 false。

### Q2: 条件编译的代码如何调试？
A2: 使用 `cargo expand` 查看展开后的代码，或者在调试模式下编译来包含更多调试信息。

### Q3: 如何检测运行时的操作系统而不是编译时？
A3: 使用 `std::env::consts::OS` 等常量，它们在运行时提供系统信息。

### Q4: 条件编译会增加编译时间吗？
A4: 轻微增加，因为编译器需要评估条件。但相比运行时检查，整体性能更好。

### Q5: 如何在条件编译中使用自定义变量？
A5: 使用功能标志（features）或环境变量，在 Cargo.toml 中定义。

---

**总结**: 条件编译是 Rust 强大的跨平台开发工具，掌握它可以让你的代码在任何平台上都能高效运行。建议多实践，逐步掌握各种条件和组合的使用技巧。

🦀 **享受 Rust 条件编译的强大功能！**