> rustup show

iterator adaptor - iterator => iterator
    [1,2,3] plus 1 => [2,3,4]
    [1,2,3] squared =>[1,4,9]
consuming adaptor - iterator => something else
    sum of [1,2,3] => 6
    Max of [1,2,3] => 3


Operator Overloading

| Operator | Trait | Used by |
| ------------- | -------------- | -------------- |
| x == y, x != y | PartialEq/Eq | Vec::contains() |
| x < y(and <=, =>, >)  | PartialOrd/Ord | [T]::sort() |
| x + y | Addition | std::time::SystemTime |
| x - y | Subtraction |  |
| -x | Negative |  |
| x * y | Multiplication |  |
| x / y | Division |  |
| x % y | Remainder |  |
| !x | Not |  |
| x & y | BitAnd |  |
| x | y | BitOr |  |
| x ^ y | BitXor |  |
| x << y | Shl |  |
| x >> y | Shr |  |
| container[index] | Index/IndexMut | std::ops::Range |
| (destructor) | Drop |  |
| (function call) | Fn/fnMut/FnOnce |  |
| *pointer | Deref/DerefMut |  |


> https://doc.rust-lang.org/std/ops/index.html


Borrowing

+ Owned to Reference - Borrow/BorrowMut
+ Reference to Owned - ToOwned
+ Reference to Reference - AsRef/AsMut
  + example: if a function takes Asref<str>, then it can accept both 
    + String
    + &str

Conversion

+ From/Into
+ Many other modules use similar naming:
  + Fromiterator/Intoiterator
  + FromStr/IntoStr


Send:
    Safe to send to another thread
Sync:
    Safe to share between threads
Automatically derived by the compiler

macros

https://doc.rust-lang.org/book/ch20-05-macros.html
https://danielkeep.github.io/tlborm/book/index.html
