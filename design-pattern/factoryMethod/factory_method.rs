pub trait PointGuard {
    fn assist(&self);
}

pub struct Paul;

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

pub trait Factory {
    fn create_point_guard(&self) -> Box<dyn PointGuard>;
}

pub struct PaulFactory;

impl Factory for PaulFactory {
    fn create_point_guard(&self) -> Box<dyn PointGuard> {
        Box::new(Paul)
    }
}

pub struct NashFactory;

impl Factory for NashFactory {
    fn create_point_guard(&self) -> Box<dyn PointGuard> {
        Box::new(Nash)
    }
}
