// 引入外部 crate：hello_macro（包含 trait 定义）和 hello_macro_derive（包含过程宏实现）
extern crate hello_macro;
#[macro_use]
extern crate hello_macro_derive;

// 导入 HelloMacro trait，以便可以使用其方法
use hello_macro::HelloMacro;

// 使用自定义派生宏 HelloMacro 为 Cat 结构体自动实现 HelloMacro trait
// 这将在编译时生成 impl HelloMacro for Cat 的代码
#[derive(HelloMacro)]
#[derive(Debug)]  // 同时派生 Debug trait，便于调试
struct Cat;

/*
#[derive(HelloMacro)] 属性宏会自动生成以下代码实现：

impl HelloMacro for Cat {
    fn hello_macro() {
        println!("Hello, Macro! I'm a Cat!");
    }
}

这个实现是由过程宏在编译时自动生成的，无需手动编写。
*/

// 主函数：程序入口点
fn main() {
    // 调用自动生成的 hello_macro 方法
    // 这会输出: "Hello, Macro! I'm a Cat!"
    Cat::hello_macro();
}
