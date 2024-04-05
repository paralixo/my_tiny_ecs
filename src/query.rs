use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use crate::entities::Entities;

pub struct Query<'a> {
    entities: &'a Entities,
    mask: u32,
    type_ids: Vec<TypeId>,
}

impl<'a> Query<'a> {
    pub fn new(entities: &'a Entities) -> Self {
        Query {
            entities,
            mask: 0,
            type_ids: vec![],
        }
    }

    pub fn with_component<T: Any>(&mut self) -> &mut Self {
        let type_id = TypeId::of::<T>();
        self.mask |= self.entities.get_component_bitmask(&type_id);
        self.type_ids.push(type_id);

        self
    }

    pub fn run(&mut self) -> Vec<Vec<&Rc<RefCell<dyn Any>>>> {
        let indexes: Vec<usize> = self.entities.entity_bitmasks.iter().enumerate().filter_map(|(index, entity_bitmask)| {
            if entity_bitmask & self.mask == self.mask {
                return Some(index)
            }
            None
        }).collect();

        let mut result = vec![];
        for type_id in self.type_ids.iter() {
            let components = self.entities.get_components(*type_id);
            let mut filtered_components = vec![];
            for index in &indexes {
                filtered_components.push(components[*index].as_ref().unwrap());
            }
            result.push(filtered_components)
        }

        result
    }

    pub fn run_entity(&self) -> Vec<QueryEntity> {
        self.entities.entity_bitmasks.iter().enumerate().filter_map(|(index, entity_bitmask)| {
            if entity_bitmask & self.mask == self.mask {
                return Some(QueryEntity::new(index, self.entities))
            }
            None
        }).collect()
    }
}

pub struct QueryEntity<'a> {
    pub id: usize,
    entities: &'a Entities,
}

type ExtractedComponents<'a> = &'a Vec<Option<Rc<RefCell<dyn Any>>>>;

impl<'a> QueryEntity<'a> {
    pub fn new(id: usize, entities: &'a Entities) -> Self {
        Self { id, entities }
    }

    fn extract_components<T: Any>(&self) -> ExtractedComponents {
        let type_id = TypeId::of::<T>();
        self.entities.get_components(type_id)
    }

    pub fn get_component<T: Any>(&self) -> Ref<T> {
        let components = self.extract_components::<T>();
        let borrowed_component = components[self.id].as_ref().unwrap().borrow();
        Ref::map(borrowed_component, |any| {
            any.downcast_ref::<T>().unwrap()
        })
    }

    pub fn get_component_mut<T: Any>(&self) -> RefMut<T> {
        let components = self.extract_components::<T>();
        let borrowed_component = components[self.id].as_ref().unwrap().borrow_mut();
        RefMut::map(borrowed_component, |any| {
            any.downcast_mut::<T>().unwrap()
        })
    }
}
