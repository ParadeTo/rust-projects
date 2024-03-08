use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

struct Singleton {
    // 单例类的数据和方法
}

impl Singleton {
    fn new() -> Self {
        Singleton {
            // 初始化数据
        }
    }

    fn some_method(&self) {
        // 单例类的方法
    }

    fn mut_method(&mut self) {}
}

lazy_static! {
    static ref INSTANCE: Arc<Singleton> = Arc::new(Singleton::new());
}

fn get_instance() -> Arc<Singleton> {
    INSTANCE.clone()
}

// 使用示例
fn main() {
    let instance1 = get_instance();
    let mut instance2 = get_instance();
    instance2.mut_method();
    // let b = instance1.deref();
    // instance1.some_method();
    // instance2.some_method();
    println!("{:?}", Arc::ptr_eq(&instance1, &instance2)); // 输出: true，只有一个实例
}
