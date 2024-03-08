use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard, Once};

#[derive(Debug)]
pub struct Singleton {
    data: String,
}

impl Singleton {
    fn new(data: String) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &str {
        &self.data
    }
    pub fn log(&self, thread_name: String) {
        println!("{}: step1", thread_name);
        println!("{}: step2", thread_name);
    }
}

static INIT: Once = Once::new();
static INSTANCE: OnceCell<Mutex<Singleton>> = OnceCell::new();

pub fn get_instance(data: String) -> MutexGuard<'static, Singleton> {
    INIT.call_once(|| {
        let instance = Mutex::new(Singleton::new(data));
        INSTANCE.set(instance).expect("Fail to set instance");
    });

    INSTANCE.get().unwrap().lock().unwrap()
}
