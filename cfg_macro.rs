/*!
 * Rust 条件编译完整演示
 *
 * 这个文件全面展示了 Rust 条件编译的各种用法，包括：
 * - #[cfg(...)] 属性：编译时条件控制
 * - cfg! 宏：运行时条件检查
 * - 复杂条件组合：all(), any(), not()
 * - 平台特定代码：跨平台开发技术
 *
 * 学习条件编译的重要性：
 * 1. 跨平台开发的核心技术
 * 2. 减少二进制文件大小
 * 3. 提升性能（编译时优化）
 * 4. 灵活的功能控制
 *
 * 相关文档：
 * - https://doc.rust-lang.org/reference/conditional-compilation.html
 * - https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
 * - https://doc.rust-lang.org/std/macro.cfg.html
 */

use std::env;

// ========== 第一部分：基础条件编译属性 ==========

/**
 * Linux 平台专用代码示例
 *
 * #[cfg(target_os = "linux")] 是条件编译属性，表示这段代码只会在
 * 目标操作系统为 Linux 时被编译器包含。在其他平台上，这段代码
 * 完全不存在，不会占用任何空间。
 *
 * 条件编译的作用：
 * - 零开销：不满足条件的代码不会被编译
 * - 平台优化：可以为不同平台提供最优实现
 * - 代码隔离：避免在不支持的平台上编译错误
 */
#[cfg(target_os = "linux")]
pub struct LinuxCode {
    /// 版本信息字符串
    version: String,
    /// 平台特定功能标志
    features_enabled: bool,
}

#[cfg(target_os = "linux")]
impl LinuxCode {
    /// 创建新的 LinuxCode 实例
    ///
    /// 这个构造函数也只会在 Linux 平台上编译
    ///
    /// # 返回值
    /// 返回一个初始化的 LinuxCode 实例
    pub fn new() -> Self {
        Self {
            version: "Linux Edition v2.0".to_string(),
            features_enabled: true, // Linux 默认启用所有功能
        }
    }

    /// 获取平台特定功能
    #[cfg(target_os = "linux")]
    pub fn get_linux_features(&self) -> Vec<&'static str> {
        vec![
            "epoll 支持",
            "inotify 文件监控",
            "信号处理",
            "Unix 域套接字"
        ]
    }
}

/**
 * macOS 平台专用代码示例
 *
 * 与 LinuxCode 类似，这个结构体只在 macOS 平台上编译
 * 展示了条件编译在不同平台上的应用
 */
#[cfg(target_os = "macos")]
pub struct MacOSCode {
    /// 版本信息
    version: String,
    /// 系统集成等级
    integration_level: u8,
}

#[cfg(target_os = "macos")]
impl MacOSCode {
    /// 创建新的 MacOSCode 实例
    pub fn new() -> Self {
        Self {
            version: "macOS Edition v2.0".to_string(),
            integration_level: 3, // macOS 系统集成等级
        }
    }

    /// 获取 macOS 特有功能
    #[cfg(target_os = "macos")]
    pub fn get_macos_features(&self) -> Vec<&'static str> {
        vec![
            "Grand Central Dispatch",
            "Metal 图形 API",
            "Core Foundation",
            "Mach 端口通信"
        ]
    }
}

/**
 * Windows 平台专用代码示例（预留扩展）
 *
 * 这个结构体目前为空，展示如何为其他平台预留接口
 * 当需要支持 Windows 时，可以补充实现
 */
#[cfg(target_os = "windows")]
pub struct WindowsCode {
    version: String,
    registry_access: bool,
}

#[cfg(target_os = "windows")]
impl WindowsCode {
    pub fn new() -> Self {
        Self {
            version: "Windows Edition v2.0".to_string(),
            registry_access: true,
        }
    }
}

// ========== 第二部分：跨平台通用代码 ==========

/**
 * 跨平台通用代码示例
 *
 * 使用 any() 条件组合，这个结构体会在支持的任何平台上编译
 * 这种方式适合实现平台无关的核心功能
 *
 * 支持的平台：Linux, macOS, Windows
 * 不支持的平台：iOS, Android, WebAssembly 等
 */
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
pub struct CrossPlatformCode {
    /// 当前平台标识
    platform: String,
    /// 架构信息
    architecture: String,
    /// 功能支持标志
    capabilities: PlatformCapabilities,
}

/// 平台能力枚举
#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
#[derive(Debug)]
pub enum PlatformCapabilities {
    Linux {
        epoll: bool,
        signals: bool,
        unix_sockets: bool
    },
    MacOS {
        metal: bool,
        gcd: bool,
        core_foundation: bool
    },
    Windows {
        win32: bool,
        com: bool,
        registry: bool
    },
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
impl CrossPlatformCode {
    /// 创建跨平台代码实例
    ///
    /// 这个方法在所有支持的平台上都会编译
    /// 通过运行时检测来提供平台特定的信息
    pub fn new() -> Self {
        let platform = env::consts::OS.to_string();
        let architecture = env::consts::ARCH.to_string();

        // 根据平台设置不同的能力
        let capabilities = if cfg!(target_os = "linux") {
            PlatformCapabilities::Linux {
                epoll: true,
                signals: true,
                unix_sockets: true,
            }
        } else if cfg!(target_os = "macos") {
            PlatformCapabilities::MacOS {
                metal: true,
                gcd: true,
                core_foundation: true,
            }
        } else if cfg!(target_os = "windows") {
            PlatformCapabilities::Windows {
                win32: true,
                com: true,
                registry: true,
            }
        } else {
            // 理论上不会执行到这里，因为前面的条件已经限制了平台
            panic!("Unsupported platform!");
        };

        Self {
            platform,
            architecture,
            capabilities,
        }
    }

    /// 打印平台问候信息
    pub fn greet(&self) {
        println!("🌍 Hello from {} platform!", self.platform);
        println!("🏗️  Architecture: {}", self.architecture);

        // 打印平台特定能力
        match &self.capabilities {
            PlatformCapabilities::Linux { epoll, signals, unix_sockets } => {
                println!("🐧 Linux Capabilities:");
                println!("   epoll support: {}", epoll);
                println!("   signal handling: {}", signals);
                println!("   Unix sockets: {}", unix_sockets);
            },
            PlatformCapabilities::MacOS { metal, gcd, core_foundation } => {
                println!("🍎 macOS Capabilities:");
                println!("   Metal API: {}", metal);
                println!("   GCD support: {}", gcd);
                println!("   Core Foundation: {}", core_foundation);
            },
            PlatformCapabilities::Windows { win32, com, registry } => {
                println!("🪟 Windows Capabilities:");
                println!("   Win32 API: {}", win32);
                println!("   COM support: {}", com);
                println!("   Registry access: {}", registry);
            }
        }
    }

    /// 检查特定功能是否支持
    pub fn supports_feature(&self, feature: &str) -> bool {
        match &self.capabilities {
            PlatformCapabilities::Linux { epoll, signals, unix_sockets } => {
                match feature {
                    "epoll" => *epoll,
                    "signals" => *signals,
                    "unix_sockets" => *unix_sockets,
                    _ => false,
                }
            },
            PlatformCapabilities::MacOS { metal, gcd, core_foundation } => {
                match feature {
                    "metal" => *metal,
                    "gcd" => *gcd,
                    "core_foundation" => *core_foundation,
                    _ => false,
                }
            },
            PlatformCapabilities::Windows { win32, com, registry } => {
                match feature {
                    "win32" => *win32,
                    "com" => *com,
                    "registry" => *registry,
                    _ => false,
                }
            }
        }
    }
}

// ========== 第三部分：编译模式相关代码 ==========

/**
 * 调试模式专用函数
 *
 * debug_assertions 是一个内置的条件，当编译器不包含优化时为 true
 * 这意味着：
 * - cargo build (debug模式) -> debug_assertions = true
 * - cargo build --release (release模式) -> debug_assertions = false
 *
 * 适用场景：
 * - 开发期间的调试信息
 * - 性能测试代码
 * - 开发工具和辅助函数
 */
#[cfg(debug_assertions)]
fn debug_info() {
    println!("🔍 这是调试模式下的信息");
    println!("✅ 调试功能已启用");
    println!("📊 性能监控已开启");
    println!("🪲 日志级别: DEBUG");

    // 在调试模式下，我们可以包含更多检查和输出
    println!("🔧 开发者选项:");
    println!("   - 详细的错误堆栈跟踪");
    println!("   - 内存分配跟踪");
    println!("   - 性能分析工具");
}

/**
 * 发布模式专用函数
 *
 * not(debug_assertions) 表示不在调试模式，即发布模式
 * 发布模式的特点：
 * - 编译器优化开启
 * - 调试信息被移除
 * - panic 信息可能被简化
 * - 更小的二进制文件和更好的性能
 *
 * 适用场景：
 * - 生产环境信息
 * - 性能优化通知
 * - 用户友好的错误消息
 */
#[cfg(not(debug_assertions))]
fn release_info() {
    println!("🚀 这是发布模式下的信息");
    println!("⚡ 优化版本正在运行");
    println!("🔒 生产环境配置已加载");
    println!("📈 性能优化已启用");

    // 发布模式的特性
    println!("🎯 生产特性:");
    println!("   - 编译器优化已开启");
    println!("   - 内存使用优化");
    println!("   - 执行速度优化");
    println!("   - 错误处理简化");
}

// ========== 第四部分：复杂条件组合示例 ==========

/**
 * 高级条件组合示例
 *
 * 展示如何使用复杂的多重条件来精确控制编译
 * 这些示例展示了实际项目中可能遇到的条件组合场景
 */

// ========== 第五部分：高级条件组合示例 ==========

/**
 * 以下示例展示了更复杂的条件组合用法
 * 注意：这些是高级用法示例，在实际项目中需要根据具体需求调整
 */

// 只有在 Linux 系统且为 64 位架构时才编译
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
pub struct Linux64Code {
    processor_count: usize,
    memory_size: u64,
}

// Linux 64位专用的实现
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
impl Linux64Code {
    pub fn new() -> Self {
        Self {
            processor_count: 4, // 示例值，实际可以从 /proc/cpuinfo 读取
            memory_size: 0, // 这里可以添加内存检测逻辑
        }
    }

    pub fn get_optimal_thread_count(&self) -> usize {
        std::cmp::min(self.processor_count, 16) // 限制最大线程数
    }
}

// 所有苹果平台（macOS, iOS）都编译的扩展结构体
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub struct ApplePlatformCode {
    platform_type: String,
    metal_available: bool,
    core_foundation_version: u32,
}

// 苹果平台实现
#[cfg(any(target_os = "macos", target_os = "ios"))]
impl ApplePlatformCode {
    pub fn new() -> Self {
        Self {
            platform_type: std::env::consts::OS.to_string(),
            metal_available: cfg!(target_arch = "aarch64"), // Apple Silicon 默认支持 Metal
            core_foundation_version: 1500, // 示例版本号
        }
    }

    pub fn supports_metal(&self) -> bool {
        self.metal_available
    }
}

// 非 Windows 平台编译（Unix-like 系统）的增强版
#[cfg(not(target_os = "windows"))]
pub struct UnixLikeCode {
    unix_features: Vec<String>,
    posix_compliance: bool,
}

// Unix-like 系统实现
#[cfg(not(target_os = "windows"))]
impl UnixLikeCode {
    pub fn new() -> Self {
        let mut features = vec![
            "POSIX 兼容".to_string(),
            "Unix 信号处理".to_string(),
            "文件描述符".to_string(),
        ];

        // 根据具体系统添加特性
        if cfg!(target_os = "linux") {
            features.push("inotify 文件监控".to_string());
            features.push("epoll I/O 多路复用".to_string());
        } else if cfg!(target_os = "macos") {
            features.push("FSEvents 文件系统事件".to_string());
            features.push("kqueue 事件通知".to_string());
        }

        Self {
            unix_features: features,
            posix_compliance: true,
        }
    }

    pub fn get_features(&self) -> &Vec<String> {
        &self.unix_features
    }
}

// 需要自定义 feature 启用的代码（在 Cargo.toml 中定义）
#[cfg(feature = "advanced-features")]
pub struct AdvancedFeatureCode {
    experimental_api: bool,
    beta_features: bool,
    debug_mode_enabled: bool,
}

// 高级功能实现
#[cfg(feature = "advanced-features")]
impl AdvancedFeatureCode {
    pub fn new() -> Self {
        Self {
            experimental_api: cfg!(debug_assertions), // 调试模式下启用实验性 API
            beta_features: true,
            debug_mode_enabled: cfg!(debug_assertions),
        }
    }

    pub fn is_experimental_enabled(&self) -> bool {
        self.experimental_api
    }
}

// 复杂的多重条件组合示例
// Unix 系统 + 非 macOS + (x86_64 或 aarch64 架构)
#[cfg(all(
    unix,
    not(target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
pub struct SpecificUnixCode {
    variant: String,
    optimization_level: u8,
    supported_features: Vec<String>,
}

// 特定 Unix 系统实现
#[cfg(all(
    unix,
    not(target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
impl SpecificUnixCode {
    pub fn new() -> Self {
        let variant = if cfg!(target_os = "linux") {
            "Linux".to_string()
        } else if cfg!(target_os = "freebsd") {
            "FreeBSD".to_string()
        } else {
            "Unknown Unix".to_string()
        };

        let mut features = vec![
            "POSIX 标准".to_string(),
            "64位架构支持".to_string(),
        ];

        if cfg!(target_os = "linux") {
            features.push("Linux 特定优化".to_string());
        }

        Self {
            variant,
            optimization_level: 2, // 针对特定架构优化
            supported_features: features,
        }
    }

    pub fn get_variant(&self) -> &str {
        &self.variant
    }
}

fn main() {
    println!("=== 条件编译和 cfg! 宏演示 ===\n");

    // ========== cfg! 宏示例（运行时检查）==========
    println!("1. cfg! 宏运行时检查：");

    if cfg!(target_os = "macos") {
        println!("→ 运行 macOS 特定的代码");

        // 因为前面已经用 #[cfg(target_os = "macos")] 条件编译了 MacOSCode
        // 所以在这里可以安全使用
        #[cfg(target_os = "macos")]
        {
            let macos_code = MacOSCode::new();
            println!("→ 创建了 macOS 专用代码: {}", macos_code.version);
        }
    }
    else if cfg!(target_os = "linux") {
        println!("→ 运行 Linux 特定的代码");

        #[cfg(target_os = "linux")]
        {
            let linux_code = LinuxCode::new();
            println!("→ 创建了 Linux 专用代码: {}", linux_code.version);
        }
    } else {
        println!("→ 我们不完全支持这个操作系统");
    }

    println!();

    // ========== 跨平台代码示例 ==========
    println!("2. 跨平台代码示例：");

    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
    {
        let cross_platform = CrossPlatformCode::new();
        cross_platform.greet();
    }

    println!();

    // ========== 系统信息显示 ==========
    println!("3. 当前目标系统信息：");
    println!("   操作系统: {}", std::env::consts::OS);
    println!("   架构: {}", std::env::consts::ARCH);
    println!("   系列家族: {}", std::env::consts::FAMILY);

    // ========== 条件编译功能演示 ==========
    println!("\n4. 条件编译功能演示：");

    // 使用 cfg! 宏检查各种条件
    println!("   是否为 64 位系统: {}", cfg!(target_pointer_width = "64"));
    println!("   是否为 Unix 系统: {}", cfg!(unix));
    println!("   是否为 Windows 系统: {}", cfg!(windows));
    println!("   是否为调试模式: {}", cfg!(debug_assertions));

    // 根据架构显示不同信息
    if cfg!(target_arch = "x86_64") {
        println!("   运行在 x86_64 架构上");
    } else if cfg!(target_arch = "aarch64") {
        println!("   运行在 ARM64/aarch64 架构上");
    } else if cfg!(target_arch = "x86") {
        println!("   运行在 x86 架构上");
    }

    // ========== 条件编译的函数调用 ==========
    println!("\n5. 条件编译函数调用：");

    // 这些函数只有在相应的条件下才会被编译
    #[cfg(debug_assertions)]
    debug_info();

    #[cfg(not(debug_assertions))]
    release_info();

    println!("\n=== 演示完成 ===");
}

// ========== 其他条件编译示例 ==========

// 使用 all() 组合多个条件
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
pub struct Linux64Code;

// 使用 any() 满足任一条件
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub struct AppleCode;

// 使用 not() 排除条件
#[cfg(not(target_os = "windows"))]
pub struct NonWindowsCode;

// 使用自定义 feature（需要通过 Cargo.toml 启用）
#[cfg(feature = "custom-feature")]
pub struct CustomFeatureCode;

// 复杂条件组合
#[cfg(all(
    unix,
    not(target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
pub struct SpecificUnixCode;

