#[macro_export]
macro_rules! my_vec {
    // 没带任何参数的 my_vec，我们创建一个空的 vec
    () => {
        std::vec::Vec::new()
    };
    // 处理 my_vec![1, 2, 3, 4]
    ($($el:expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*
        v
    });
    // 处理 my_vec![0; 10]
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    }
}
