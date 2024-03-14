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

pub trait CentreForward {
    fn slam_dunk(&self);
}

pub struct ONeal;

impl CentreForward for ONeal {
    fn slam_dunk(&self) {
        println!("ONeal slam dunk");
    }
}

struct YaoMing;

impl CentreForward for YaoMing {
    fn slam_dunk(&self) {
        println!("YaoMing slam dunk");
    }
}

pub trait TeamFactory {
    fn create_point_guard(&self) -> Box<dyn PointGuard>;
    fn create_centre_forward(&self) -> Box<dyn CentreForward>;
}

pub struct LakersFactory;

impl TeamFactory for LakersFactory {
    fn create_point_guard(&self) -> Box<dyn PointGuard> {
        Box::new(Paul)
    }

    fn create_centre_forward(&self) -> Box<dyn CentreForward> {
        Box::new(ONeal)
    }
}

pub struct RocketFactory;

impl TeamFactory for RocketFactory {
    fn create_point_guard(&self) -> Box<dyn PointGuard> {
        Box::new(Nash)
    }

    fn create_centre_forward(&self) -> Box<dyn CentreForward> {
        Box::new(YaoMing)
    }
}
