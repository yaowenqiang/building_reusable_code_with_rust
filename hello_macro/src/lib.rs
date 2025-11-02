/// HelloMacro trait 定义
///
/// 这个 trait 定义了一个简单的行为：打印问候消息。
/// 通过过程宏，我们可以为任意结构体自动实现这个 trait，
/// 而无需手动编写重复的代码。
///
/// # 示例
///
/// ```rust
/// use hello_macro::HelloMacro;
///
/// #[derive(HelloMacro)]
/// struct MyStruct;
///
/// fn main() {
///     MyStruct::hello_macro(); // 自动生成的实现
/// }
/// ```
pub trait HelloMacro {
    /// 打印问候消息的方法
    ///
    /// 这个方法的具体实现由过程宏自动生成，
    /// 通常会打印包含结构体名称的问候消息。
    fn hello_macro();
}
