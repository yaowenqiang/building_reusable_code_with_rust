# Rust 条件编译快速参考手册

## 📖 基础语法

### 条件编译属性
```rust
#[cfg(condition)]
fn function_name() {}

#[cfg(all(condition1, condition2))]
fn all_conditions_must_be_true() {}

#[cfg(any(condition1, condition2))]
fn any_condition_can_be_true() {}

#[cfg(not(condition))]
fn condition_must_be_false() {}
```

### 运行时检查宏
```rust
if cfg!(condition) {
    // 条件成立时执行的代码
}

let is_debug = cfg!(debug_assertions);
```

## 🎯 常用条件

### 系统相关
```rust
#[cfg(target_os = "linux")]     // Linux 系统
#[cfg(target_os = "macos")]      // macOS 系统
#[cfg(target_os = "windows")]    // Windows 系统
#[cfg(target_os = "freebsd")]    // FreeBSD 系统

#[cfg(target_family = "unix")]   // Unix 家族（Linux、macOS等）
#[cfg(target_family = "windows")] // Windows 家族

#[cfg(unix)]                     // Unix 系统（等价于 target_family = "unix"）
#[cfg(windows)]                  // Windows 系统
```

### 架构相关
```rust
#[cfg(target_arch = "x86")]      // x86 架构
#[cfg(target_arch = "x86_64")]   // x86_64 架构
#[cfg(target_arch = "aarch64")]  // ARM64 架构
#[cfg(target_arch = "arm")]      // ARM 架构
#[cfg(target_arch = "wasm32")]   // WebAssembly 32位

#[cfg(target_pointer_width = "32")]  // 32位系统
#[cfg(target_pointer_width = "64")]  // 64位系统

#[cfg(target_endian = "little")]     // 小端序
#[cfg(target_endian = "big")]        // 大端序
```

### 编译模式
```rust
#[cfg(debug_assertions)]         // 调试模式
#[cfg(not(debug_assertions))]    // 发布模式

#[cfg(test)]                     // 测试模式
#[cfg(not(test))]                // 非测试模式
```

### 功能特性
```rust
#[cfg(feature = "feature_name")] // 特定功能启用
#[cfg(not(feature = "feature_name"))] // 特定功能禁用

#[cfg(all(feature = "feature1", feature = "feature2"))] // 多功能都启用
```

## 🔧 实用模式

### 平台特定实现
```rust
// 统一接口
trait PlatformSpecific {
    fn get_platform_name(&self) -> &str;
}

// Linux 实现
#[cfg(target_os = "linux")]
struct LinuxImpl;
#[cfg(target_os = "linux")]
impl PlatformSpecific for LinuxImpl {
    fn get_platform_name(&self) -> &str { "Linux" }
}

// macOS 实现
#[cfg(target_os = "macos")]
struct MacOSImpl;
#[cfg(target_os = "macos")]
impl PlatformSpecific for MacOSImpl {
    fn get_platform_name(&self) -> &str { "macOS" }
}

// 统一工厂函数
fn create_platform_impl() -> Box<dyn PlatformSpecific> {
    #[cfg(target_os = "linux")]
    return Box::new(LinuxImpl);

    #[cfg(target_os = "macos")]
    return Box::new(MacOSImpl);

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    compile_error!("Unsupported platform!");
}
```

### 功能门控
```rust
#[cfg(feature = "advanced")]
mod advanced_features {
    pub fn process_data() {
        println!("Advanced processing enabled");
    }
}

#[cfg(not(feature = "advanced"))]
mod basic_features {
    pub fn process_data() {
        println!("Basic processing only");
    }
}

// 重新导出统一接口
pub use advanced_features::process_data as process_data;
pub use basic_features::process_data as process_data;
```

### 调试/发布模式
```rust
#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($($arg:tt)*) => {};
}
```

## 📊 条件组合示例

### 复杂组合
```rust
// Linux + 64位系统
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]

// Unix 系统 + 非 macOS
#[cfg(all(unix, not(target_os = "macos")))]

// Windows 或 Linux
#[cfg(any(target_os = "windows", target_os = "linux"))]

// 非 Windows + 64位
#[cfg(all(not(target_os = "windows"), target_pointer_width = "64"))]

// 调试模式 + Unix 系统
#[cfg(all(debug_assertions, unix))]
```

### 嵌套组合
```rust
#[cfg(all(
    unix,
    any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "macos", target_arch = "aarch64")
    )
))]
```

## 🛠️ Cargo 集成

### 平台特定依赖
```toml
[dependencies]
# 通用依赖
serde = "1.0"

# Linux 特定依赖
[target.'cfg(target_os = "linux")'.dependencies]
inotify = "0.10"
libc = "0.2"

# macOS 特定依赖
[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"

# Windows 特定依赖
[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["winuser"] }
```

### 功能特性
```toml
[features]
default = ["basic"]
basic = []
advanced = ["networking", "database"]
networking = ["tokio"]
database = ["sqlx"]
```

## 🚀 性能技巧

### 编译时常量
```rust
// 好的做法：编译时确定
#[cfg(target_os = "linux")]
const OPTIMAL_BUFFER_SIZE: usize = 8192;

#[cfg(target_os = "macos")]
const OPTIMAL_BUFFER_SIZE: usize = 4096;

// 避免：运行时判断
fn get_buffer_size() -> usize {
    if cfg!(target_os = "linux") {
        8192
    } else {
        4096
    }
}
```

### 内联优化
```rust
#[cfg(target_os = "linux")]
#[inline(always)]
fn linux_specific_syscall() -> i32 {
    // 内联的 Linux 特定代码
    unsafe { libc::syscall(...) }
}
```

## 🐛 调试技巧

### 查看宏展开
```bash
# 安装工具
cargo install cargo-expand

# 查看展开结果
cargo expand --target x86_64-unknown-linux-gnu
```

### 编译时错误检查
```rust
#[cfg(all(target_pointer_width = "32", feature = "heavy-computation"))]
compile_error!("Heavy computation requires 64-bit architecture");

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
compile_error!("This platform is not supported");
```

### 调试宏
```rust
macro_rules! cfg_debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!("[CFG_DEBUG] {}", format!($($arg)*));
    };
}

fn debug_function() {
    cfg_debug!("Target OS: {}", std::env::consts::OS);
    cfg_debug!("Arch: {}", std::env::consts::ARCH);
}
```

## 📋 常用平台常量

### 系统信息常量
```rust
use std::env::consts;

fn show_system_info() {
    println!("OS: {}", consts::OS);
    println!("Arch: {}", consts::ARCH);
    println!("Family: {}", consts::FAMILY);
    println!("Pointer Width: {}", consts::PTR_WIDTH);
    println!("Endian: {}", consts::ENDIAN);
}
```

### 条件检查函数
```rust
fn is_supported_platform() -> bool {
    match std::env::consts::OS {
        "linux" | "macos" | "windows" => true,
        _ => false,
    }
}

fn is_64_bit() -> bool {
    std::env::consts::PTR_WIDTH == "64"
}
```

## ⚠️ 注意事项

### 常见错误
```rust
// 错误：在条件外使用条件编译的类型
fn wrong_example() {
    if cfg!(target_os = "linux") {
        let linux_code = LinuxCode; // 编译错误！LinuxCode 在 macOS 上不存在
    }
}

// 正确：嵌套条件编译
fn correct_example() {
    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            let linux_code = LinuxCode; // 正确
        }
    }
}
```

### 最佳实践
1. **优先使用条件编译**：减少运行时开销
2. **提供通用实现**：为不支持的平台提供备选方案
3. **文档化平台限制**：清楚说明哪些平台支持哪些功能
4. **测试所有支持的平台**：确保条件编译代码的正确性
5. **使用有意义的功能名称**：提高代码可读性

## 📖 更多资源

### 官方文档
- [Rust Reference: Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [Cargo Book: Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [cfg! Macro Documentation](https://doc.rust-lang.org/std/macro.cfg.html)

### 实际项目
- [crossbeam](https://github.com/crossbeam-rs/crossbeam) - 跨平台并发库
- [tokio](https://github.com/tokio-rs/tokio) - 异步运行时
- [serde](https://github.com/serde-rs/serde) - 序列化框架

---

**快速提示**: 使用 `rustc --print target-list` 查看所有支持的目标平台！ 🦀