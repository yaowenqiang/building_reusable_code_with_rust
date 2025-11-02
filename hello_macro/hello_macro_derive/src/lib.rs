// 引入过程宏所需的核心库
use proc_macro::TokenStream;  // 编译器提供的 TokenStream 类型
use syn::DeriveInput;         // syn 库提供的派生输入结构体
use quote::quote;             // quote 库提供的 quote! 宏

/// HelloMacro 自定义派生宏的入口函数
///
/// 这个函数通过 #[proc_macro_derive(HelloMacro)] 属性标记为过程宏，
/// 当编译器遇到 #[derive(HelloMacro)] 时会调用这个函数。
///
/// # 参数
/// * `input` - 编译器传入的 TokenStream，包含被标记的结构体或枚举的代码
///
/// # 返回值
/// 返回生成的 impl 代码的 TokenStream
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 步骤1: 解析输入的 TokenStream 为抽象语法树 (AST)
    // syn::parse 将原始的 TokenStream 转换为结构化的 DeriveInput
    let ast = syn::parse(input).unwrap();

    // 步骤2: 根据解析得到的 AST 生成实现代码
    let gen = impl_hello_macro(&ast);

    // 步骤3: 将生成的代码转换为 TokenStream 返回给编译器
    gen.into()
}

/// 生成 HelloMacro trait 实现的核心函数
///
/// 这个函数负责为给定的结构体或枚举生成 HelloMacro trait 的实现代码。
///
/// # 参数
/// * `ast` - 解析后的抽象语法树，包含结构体/枚举的信息
///
/// # 返回值
/// 返回生成的 impl 代码的 TokenStream
fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    // 从 AST 中提取结构体/枚举的标识符（名称）
    // 例如：对于 struct Cat，&ast.ident 就是 "Cat"
    let name = &ast.ident;

    // 使用 quote! 宏生成 Rust 代码
    // quote! 允许我们在代码中使用模板语法 #{} 来插入变量
    let gen = quote! {
        // 为指定的结构体实现 HelloMacro trait
        impl HelloMacro for #name {
            // 实现 hello_macro 方法
            fn hello_macro() {
                // 打印包含结构体名称的问候消息
                // stringify!(#name) 将标识符转换为字符串字面量
                println!("Hello, Macro! I'm a {}!", stringify!(#name));
            }
        }
    };

    // 将生成的代码转换为 TokenStream 并返回
    gen.into()
}

/*
过程宏工作流程详解：

1. 编译器阶段：编译器遇到 #[derive(HelloMacro)]
   ↓
2. 宏展开：调用 hello_macro_derive 函数
   ↓
3. 代码解析：syn::parse 将 "struct Cat;" 转换为 DeriveInput
   ↓
4. 代码生成：impl_hello_macro 生成以下代码：
   impl HelloMacro for Cat {
       fn hello_macro() {
           println!("Hello, Macro! I'm a Cat!");
       }
   }
   ↓
5. 编译继续：编译器将生成的代码插入到原位置，继续编译

这种技术称为"元编程"，即在编译时生成代码，
可以大大减少重复代码，提高代码的可维护性。
*/
