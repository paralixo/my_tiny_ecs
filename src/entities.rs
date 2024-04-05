use std::any::{Any, TypeId};
use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::Rc;

type Component = Rc<RefCell<dyn Any>>;
type Components = HashMap<TypeId, Vec<Option<Component>>>;

pub struct Entities {
    components: Components,
    component_bitmasks: HashMap<TypeId, u32>,
    pub entity_bitmasks: Vec<u32>,
}

impl Entities {
    pub fn new() -> Self {
        Entities {
            components: HashMap::new(),
            component_bitmasks: HashMap::new(),
            entity_bitmasks: Vec::new(),
        }
    }

    pub fn register_component<T: Any + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.component_bitmasks.insert(type_id, 1 << self.component_bitmasks.len());
        self.components.insert(type_id, vec![]);
    }

    pub fn create_entity(&mut self) -> &mut Self {
        for (_key, components) in self.components.iter_mut() {
            components.push(None);
        }

        self.entity_bitmasks.push(0);

        self
    }


    pub fn with_component(&mut self, data: impl Any) -> &mut Self {
        let type_id = data.type_id();
        if let Some(components) = self.components.get_mut(&type_id) {
            let last = components.last_mut().unwrap();
            *last = Some(Rc::new(RefCell::new(data)));
            let component_bitmask = self.component_bitmasks.get(&type_id).unwrap();
            let entity_index = self.entity_bitmasks.len() - 1;
            self.entity_bitmasks[entity_index] |= *component_bitmask;
        }

        self
    }

    pub fn get_component_bitmask(&self, type_id: &TypeId) -> u32 {
        self.component_bitmasks.get(type_id).copied().unwrap()
    }

    pub fn get_components(&self, type_id: TypeId) -> &Vec<Option<Rc<RefCell<dyn Any>>>> {
        self.components.get(&type_id).unwrap()
    }

    pub fn remove_entity_by_id(&mut self, index: usize) {
        *self.entity_bitmasks.get_mut(index).unwrap() = 0;
    }

    pub fn remove_component_by_entity_id<T: Any>(&mut self, index: usize) {
        let type_id = TypeId::of::<T>();
        let component_mask = self.get_component_bitmask(&type_id);

        if self.has_component(index, component_mask) {
            self.entity_bitmasks[index] ^= component_mask
        }
    }

    pub fn add_component_by_entity_id<T: Any>(&mut self, index: usize, data: impl Any) {
        let type_id = TypeId::of::<T>();
        let component_mask = self.get_component_bitmask(&type_id);
        self.entity_bitmasks[index] |= component_mask;

        self.components.get_mut(&type_id).unwrap()[index] = Some(Rc::new(RefCell::new(data)));
    }

    fn has_component(&self, index: usize, mask: u32) -> bool {
        self.entity_bitmasks[index] & mask == mask
    }
}