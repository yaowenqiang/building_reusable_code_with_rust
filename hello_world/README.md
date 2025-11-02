# Rust 过程宏项目 - Hello Macro 示例

## 项目概述

这个项目演示了 Rust 过程宏（Procedural Macros）的使用，特别是自定义派生宏（Custom Derive Macros）。项目实现了一个 `HelloMacro` trait，通过过程宏自动为任意结构体生成实现代码。

## 项目结构

```
hello_world/
├── src/
│   └── main.rs              # 主程序，使用自定义宏
├── Cargo.toml               # 主项目配置
└── README.md               # 项目文档

../hello_macro/
├── src/
│   └── lib.rs               # HelloMacro trait 定义
└── Cargo.toml               # 库配置

../hello_macro_derive/
├── src/
│   └── lib.rs               # 过程宏实现
└── Cargo.toml               # 过程宏配置
```

## 核心组件

### 1. HelloMacro Trait (`hello_macro/src/lib.rs`)

定义了一个简单的 trait，包含一个方法：
```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

### 2. 过程宏实现 (`hello_macro_derive/src/lib.rs`)

使用 `proc_macro`、`syn` 和 `quote` 库实现自定义派生宏：

- **proc_macro**: Rust 标准库，提供过程宏的基础功能
- **syn**: 解析 Rust 代码，将 TokenStream 转换为抽象语法树
- **quote**: 将抽象语法树转换回 Rust 代码

### 3. 主程序 (`src/main.rs`)

演示如何使用自定义派生宏：
```rust
#[derive(HelloMacro)]
#[derive(Debug)]
struct Cat;

fn main() {
    Cat::hello_macro(); // 输出: "Hello, Macro! I'm a Cat!"
}
```

## 技术要点

### 过程宏的工作原理

1. **输入**: 编译器将 `#[derive(HelloMacro)]` 的结构体转换为 TokenStream
2. **解析**: 使用 `syn` 解析 TokenStream 为抽象语法树 (AST)
3. **生成**: 使用 `quote!` 宏生成实现代码
4. **输出**: 将生成的代码转换回 TokenStream 返回给编译器

### 依赖库说明

- **syn v2.0.108**: 现代 Rust 代码解析库
- **quote v1.0.41**: 代码生成库
- **proc_macro**: Rust 内置过程宏支持

## 学习价值

这个项目展示了以下重要概念：

1. **元编程**: 在编译时生成代码
2. **代码生成**: 自动化样板代码的编写
3. **编译器插件**: 扩展编译器功能
4. **抽象语法树**: 理解 Rust 代码结构

## 相关文档链接

### 官方文档
- [Rust Book - 第20章: 宏](https://doc.rust-lang.org/book/ch20-00-macros.html)
- [过程宏 API 文档](https://doc.rust-lang.org/proc_macro/)
- [syn 库文档](https://docs.rs/syn/latest/syn/)
- [quote 库文档](https://docs.rs/quote/latest/quote/)

### 进阶资源
- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)
- [Rust by Example - 宏](https://doc.rust-lang.org/rust-by-example/macros.html)
- [Procedural Macro Workshop](https://github.com/dtolnay/proc-macro-workshop)

## 编译和运行

```bash
# 编译并运行项目
cargo run

# 或者分步执行
cargo build
./target/debug/hello_world
```

## 扩展练习

1. 为 `HelloMacro` trait 添加更多方法
2. 支持枚举类型的派生
3. 添加自定义属性来控制生成的代码
4. 实现其他类型的派生宏（如 `Serialize`, `Deserialize`）

## 故障排除

如果遇到编译错误，请检查：

1. Rust 版本是否支持过程宏 (需要 Rust 1.31+)
2. 所有 `Cargo.toml` 文件中的 edition 是否正确
3. `hello_macro_derive` 是否设置了 `proc-macro = true`
4. 依赖版本是否兼容

## 贡献指南

欢迎提交 Issue 和 Pull Request 来改进这个示例项目！

---

*该项目是学习 Rust 高级特性的绝佳示例，特别适合想要理解元编程和编译器插件开发的开发者。*