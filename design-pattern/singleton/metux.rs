use lazy_static::lazy_static;
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
}

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Singleton>> = Arc::new(Mutex::new(Singleton::new()));
}

fn get_instance() -> Arc<Mutex<Singleton>> {
    INSTANCE.clone()
}

// 使用示例
fn main() {
    let instance1 = get_instance();
    let instance2 = get_instance();

    println!("{:?}", Arc::ptr_eq(&instance1, &instance2)); // 输出: true，只有一个实例
}
