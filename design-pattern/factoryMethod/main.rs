use crate::abstract_factory::{LakersFactory, RocketFactory, TeamFactory};
use crate::factory_method::{Factory, NashFactory, Paul, PaulFactory};
use simple_factory::SimplePointGuardFactory;

mod abstract_factory;
mod factory_method;
mod simple_factory;

fn assist(factory: &dyn Factory) {
    let point_guard = factory.create_point_guard();
    point_guard.assist();
}

fn play(team_factory: &dyn TeamFactory) {
    let point_guard = team_factory.create_point_guard();
    let centre_forward = team_factory.create_centre_forward();
    point_guard.assist();
    centre_forward.slam_dunk();
}

fn main() {
    play(&RocketFactory);
    play(&LakersFactory);
}
