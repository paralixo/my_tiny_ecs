use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Resources {
    data: HashMap<TypeId, Box<dyn Any>>,
}
impl Resources {
    pub fn new() -> Self {
        Resources {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, data: impl Any) {
        let type_id = data.type_id();
        self.data.insert(type_id, Box::new(data));
    }

    pub fn remove<T: Any>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.data.remove(&type_id);
    }

    pub fn get_ref<T: Any>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        self.data.get(&type_id).unwrap().downcast_ref().unwrap()
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        let type_id = TypeId::of::<T>();
        self.data.get_mut(&type_id).unwrap().downcast_mut().unwrap()
    }
}