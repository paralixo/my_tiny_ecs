use std::any::Any;
use crate::entities::Entities;
use crate::resources::Resources;
use crate::system::System;

pub struct World {
    entities: Entities,
    resources: Resources,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Entities::new(),
            resources: Resources::new(),
            systems: Vec::new(),
        }
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&mut self) {
        for system in self.systems.iter_mut() {
            system.run(&mut self.entities, &mut self.resources)
        }
    }

    pub fn register_component<T: Any + 'static>(&mut self) {
        self.entities.register_component::<T>()
    }

    pub fn create_entity(&mut self) -> &mut Entities {
        self.entities.create_entity()
    }

    pub fn create_resource(&mut self, resource: impl Any) {
        self.resources.add(resource)
    }

    pub fn remove_resource<T: Any>(&mut self) {
        self.resources.remove::<T>()
    }

    pub fn get_resource<T: Any>(&self) -> &T {
        self.resources.get_ref::<T>()
    }

    pub fn get_resource_mut<T: Any>(&mut self) -> &mut T {
        self.resources.get_mut::<T>()
    }
}
