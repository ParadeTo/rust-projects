mod no_arc;
mod parameters;

use crate::parameters::{get_instance, Singleton};
use lazy_static::lazy_static;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::spawn;

// 使用示例
fn main() {
    let threads: Vec<_> = (0..10)
        .map(|i| {
            spawn(move || {
                let instance = get_instance(format!("hello{}", i));
                instance.log(format!("thread{}", i));
            })
        })
        .collect();

    for handle in threads {
        handle.join().unwrap();
    }

    let instance = get_instance("".to_string());
    println!("{}", instance.get_data());
    // let mut instance1 = get_instance("hello".to_string());
    // let mut instance2 = get_instance("world".to_string());
    // // instance1.init("hello".to_string());
    // // instance1.init("hello".to_string());
    // // instance2.init("world".to_string());
    // let d1 = instance1.get_data();
    // let d2 = instance2.get_data();
    // println!("{} {}", d1, d2);
    // // let b = instance1.deref();
    // // instance1.some_method();
    // // instance2.some_method();
    // println!("{:p} {:p}", instance1, instance2);
    //
    // let thread1 = spawn(|| {
    //     let mut instance2 = get_instance("world3".to_string());
    //     let d2 = instance2.get_data();
    //     println!("{}", d2);
    // });
    // let thread2 = spawn(|| {
    //     let mut instance2 = get_instance("world3".to_string());
    //     let d2 = instance2.get_data();
    //     println!("{}", d2);
    // });
    // let thread3 = spawn(|| {
    //     let mut instance2 = get_instance("world3".to_string());
    //     let d2 = instance2.get_data();
    //     println!("{}", d2);
    // });
    // //
    // thread1.join();
    // thread2.join();
    // thread3.join();
    // println!("{:?}", Arc::ptr_eq(&instance1, &instance2)); // 输出: true，只有一个实例
}
