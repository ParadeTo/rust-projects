use lazy_static::lazy_static;
use std::sync::{Arc, RwLock};

struct Singleton {
    // 在这里定义需要共享的数据
    data: String,
    // 在这里定义读写锁
    lock: RwLock<()>,
}

impl Singleton {
    fn new() -> Self {
        Self {
            data: String::new(),
            lock: RwLock::new(()),
        }
    }

    fn modify_data(&self) {
        let _lock = self.lock.write().unwrap();
        // 在这里可以修改共享的数据
        self.data.push_str("Modified");
    }

    fn get_data(&self) -> &str {
        &self.data
    }
}

lazy_static! {
    static ref INSTANCE: Arc<RwLock<Singleton>> = Arc::new(RwLock::new(Singleton::new()));
}

fn main() {
    let instance = INSTANCE.clone();
    let mut instance_guard = instance.write().unwrap();
    instance_guard.modify_data();

    let data = instance_guard.get_data();
    println!("Modified data: {}", data);
}
