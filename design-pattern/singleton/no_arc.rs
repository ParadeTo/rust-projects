use lazy_static::lazy_static;
use once_cell::sync::{Lazy, OnceCell};
use std::ops::DerefMut;
use std::string::ToString;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
//
// #[derive(Default)]
// pub struct Singleton {
//     data: String,
// }
//
// impl Singleton {
//     fn new() -> Self {
//         Default::default()
//     }
//
//     pub fn init(&mut self, data: String) {
//         self.data = data
//     }
// }
//
// lazy_static! {
//     pub static ref INSTANCE: Mutex<Singleton> = Mutex::new(Singleton::new());
// }
//
// pub fn get_instance() -> &'static mut Singleton {
//     &mut *INSTANCE.lock().unwrap()
// }

#[derive(Default, Debug)]
pub struct Singleton {
    data: String,
}

impl Singleton {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &str {
        self.data.as_str()
    }
}

// static mut INSTANCE: Lazy<Singleton> = Lazy::new(|| Singleton::default());

// static mut INITIALIZED: bool = false;

static mut INITIALIZED: Mutex<bool> = Mutex::new(false);
static INSTANCE: OnceCell<Singleton> = OnceCell::new();
pub fn get_instance(data: String) -> &'static Singleton {
    unsafe {
        let mut a = INITIALIZED.lock().unwrap();
        if (!*a) {
            println!("initlize");
            let instance = Singleton::new(data);
            INSTANCE.set(instance).expect("Failed to set");
            *a = true;
        }

        &INSTANCE.get().unwrap()
    }
}
