use crate::entities::Entities;
use crate::resources::Resources;

pub trait System {
    fn run(&self, ecs: &mut Entities, resources: &mut Resources);
}
