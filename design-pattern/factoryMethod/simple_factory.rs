pub trait PointGuard {
    fn assist(&self);
}

struct Paul;

impl PointGuard for Paul {
    fn assist(&self) {
        println!("Paul assist");
    }
}

struct Nash;

impl PointGuard for Nash {
    fn assist(&self) {
        println!("Nash assist");
    }
}

pub struct SimplePointGuardFactory;

impl SimplePointGuardFactory {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create_product(&self, product_type: &str) -> Box<dyn PointGuard> {
        match product_type {
            "P" => Box::new(Paul),
            "N" => Box::new(Nash),
            _ => panic!("Invalid point guard"),
        }
    }
}
