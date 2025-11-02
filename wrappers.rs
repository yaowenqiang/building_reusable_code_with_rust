use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    println!("=== Rust 智能指针演示 ===\n");

    // ===== 1. Box<T> - 堆分配指针 =====
    // Box<T> 在堆上分配内存，提供单一所有权
    // 文档: https://doc.rust-lang.org/std/boxed/struct.Box.html
    println!("1. Box<T> - 堆分配指针:");
    let mybox: Box<i32> = Box::new(42);
    println!("   Box 内容: {}", mybox);
    println!("   Box 解引用: {}", *mybox);
    println!("   当 Box 离开作用域时，堆内存会自动释放\n");

    // ===== 2. Rc<T> - 引用计数指针 =====
    // Rc<T> 允许多个所有者共享数据（单线程环境）
    // 文档: https://doc.rust-lang.org/std/rc/struct.Rc.html
    println!("2. Rc<T> - 引用计数指针:");
    let myrc: Rc<i32> = Rc::new(42);
    let rc_clone1 = Rc::clone(&myrc);
    let rc_clone2 = Rc::clone(&myrc);
    println!("   原始值: {}", myrc);
    println!("   克隆1: {}", rc_clone1);
    println!("   克隆2: {}", rc_clone2);
    println!("   引用计数: {}", Rc::strong_count(&myrc));
    drop(rc_clone1);
    println!("   释放一个后引用计数: {}", Rc::strong_count(&myrc));
    println!();

    // ===== 3. Cell<T> - 内部可变性 =====
    // Cell<T> 允许在不可变引用下修改数据，但只能用于 Copy 类型
    // 文档: https://doc.rust-lang.org/std/cell/struct.Cell.html
    println!("3. Cell<T> - 内部可变性:");
    let mycell: Cell<i32> = Cell::new(42);
    println!("   初始值: {}", mycell.get());
    mycell.set(100);
    println!("   修改后: {}", mycell.get());
    println!();

    // ===== 4. RefCell<T> - 运行时借用检查 =====
    // RefCell<T> 提供内部可变性，有运行时借用检查
    // 文档: https://doc.rust-lang.org/std/cell/struct.RefCell.html
    println!("4. RefCell<T> - 运行时借用检查:");
    let myrefcell: RefCell<String> = RefCell::new(String::from("Hello"));

    // 不可变借用
    let borrow1 = myrefcell.borrow();
    println!("   不可变借用: {}", borrow1);

    // 可变借用（需要先释放不可变借用）
    drop(borrow1);
    let mut borrow2 = myrefcell.borrow_mut();
    borrow2.push_str(", World!");
    println!("   可变借用后: {}", borrow2);
    println!();

    // ===== 5. Arc<T> - 原子引用计数 =====
    // Arc<T> 是线程安全版本的 Rc<T>
    // 文档: https://doc.rust-lang.org/std/sync/struct.Arc.html
    println!("5. Arc<T> - 原子引用计数 (线程安全):");
    let myarc: Arc<i32> = Arc::new(42);
    let arc_clone1 = Arc::clone(&myarc);

    let handle = thread::spawn(move || {
        println!("   线程中访问 Arc 值: {}", arc_clone1);
    });

    handle.join().unwrap();
    println!("   主线程 Arc 值: {}", myarc);
    println!();

    // ===== 6. Mutex<T> - 互斥锁 =====
    // Mutex<T> 确保同一时间只有一个线程可以访问数据
    // 文档: https://doc.rust-lang.org/std/sync/struct.Mutex.html
    println!("6. Mutex<T> - 互斥锁:");
    let mymutex: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let mutex_clone = Arc::clone(&mymutex);
        let handle = thread::spawn(move || {
            let mut num = mutex_clone.lock().unwrap();
            *num += 1;
            println!("   线程 {} 修改后值: {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("   最终值: {}", *mymutex.lock().unwrap());
    println!();

    // ===== 7. RwLock<T> - 读写锁 =====
    // RwLock<T> 允许多个读者或一个写者
    // 文档: https://doc.rust-lang.org/std/sync/struct.RwLock.html
    println!("7. RwLock<T> - 读写锁:");
    let myrwlock: Arc<RwLock<String>> = Arc::new(RwLock::new(String::from("初始值")));

    // 读操作
    let read_handle = {
        let rwlock_clone = Arc::clone(&myrwlock);
        thread::spawn(move || {
            let reader = rwlock_clone.read().unwrap();
            println!("   读线程: {}", *reader);
        })
    };

    // 写操作
    let write_handle = {
        let rwlock_clone = Arc::clone(&myrwlock);
        thread::spawn(move || {
            let mut writer = rwlock_clone.write().unwrap();
            *writer = String::from("修改后的值");
            println!("   写线程: {}", *writer);
        })
    };

    read_handle.join().unwrap();
    write_handle.join().unwrap();

    println!("   最终值: {}", *myrwlock.read().unwrap());
    println!();

    // ===== 8. 组合模式演示 =====
    // 常见的组合模式：Rc<RefCell<T>> 提供共享所有权和内部可变性
    println!("8. 组合模式 - Rc<RefCell<T>>:");

    #[derive(Debug)]
    struct Node {
        value: i32,
        // 使用弱引用避免循环引用
        neighbors: Vec<Rc<RefCell<Node>>>,
        parent: Option<Weak<RefCell<Node>>>,
    }

    use std::rc::Weak;

    let node1 = Rc::new(RefCell::new(Node { value: 1, neighbors: vec![], parent: None }));
    let node2 = Rc::new(RefCell::new(Node { value: 2, neighbors: vec![], parent: None }));
    let node3 = Rc::new(RefCell::new(Node { value: 3, neighbors: vec![], parent: None }));

    // 创建单向链表结构（避免循环引用）
    node1.borrow_mut().neighbors.push(Rc::clone(&node2));
    node2.borrow_mut().neighbors.push(Rc::clone(&node3));

    // 设置父节点引用（使用弱引用）
    node2.borrow_mut().parent = Some(Rc::downgrade(&node1));
    node3.borrow_mut().parent = Some(Rc::downgrade(&node2));

    println!("   节点1: value={}, neighbors={}",
             node1.borrow().value,
             node1.borrow().neighbors.len());
    println!("   节点2: value={}, neighbors={}, parent={}",
             node2.borrow().value,
             node2.borrow().neighbors.len(),
             node2.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().value);
    println!("   节点3: value={}, neighbors={}, parent={}",
             node3.borrow().value,
             node3.borrow().neighbors.len(),
             node3.borrow().parent.as_ref().unwrap().upgrade().unwrap().borrow().value);

    println!("\n=== 演示完成 ===");

    println!("\n=== 相关学习资源 ===");
    println!("📚 Rust 智能指针相关文档:");
    println!("• 智能指针指南: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html");
    println!("• Rc 文档: https://doc.rust-lang.org/std/rc/struct.Rc.html");
    println!("• Arc 文档: https://doc.rust-lang.org/std/sync/struct.Arc.html");
    println!("• Cell 文档: https://doc.rust-lang.org/std/cell/struct.Cell.html");
    println!("• RefCell 文档: https://doc.rust-lang.org/std/cell/struct.RefCell.html");
    println!("• Mutex 文档: https://doc.rust-lang.org/std/sync/struct.Mutex.html");
    println!("• RwLock 文档: https://doc.rust-lang.org/std/sync/struct.RwLock.html");
    println!("• Weak 文档: https://doc.rust-lang.org/std/rc/struct.Weak.html");
}

