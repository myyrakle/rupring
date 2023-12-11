use std::any::Any;
use std::{any::TypeId, collections::HashMap};

pub struct DIContext {
    pub containers: HashMap<TypeId, Box<dyn Any>>,
    wait_list: Vec<(Vec<TypeId>, Box<dyn Provider>)>,
}

impl DIContext {
    pub fn new() -> Self {
        DIContext {
            containers: HashMap::new(),
            wait_list: vec![],
        }
    }

    pub fn register<T: 'static>(&mut self, value: T) {
        if self.containers.contains_key(&TypeId::of::<T>()) {
            return;
        }

        self.containers.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn register_lazy<T: 'static>(
        &mut self,
        dependencies: Vec<TypeId>,
        injectable: Box<dyn Provider>,
    ) {
        self.wait_list.push((dependencies, injectable));
    }

    pub fn resolve<T: 'static>(&self) -> Option<&T> {
        match self.containers.get(&TypeId::of::<T>()) {
            Some(value) => value.downcast_ref::<T>(),
            None => None,
        }
    }
}

impl DIContext {}

pub trait Provider {
    fn provide(&self, di_context: &DIContext) -> Box<dyn Any>;
}
