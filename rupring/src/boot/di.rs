use std::any::Any;
use std::{any::TypeId, collections::HashMap};

pub struct DIContext {
    pub containers: HashMap<TypeId, Box<dyn Any>>,
    wait_list: Vec<Box<dyn IProvider + 'static>>,
}

unsafe impl Send for DIContext {}
unsafe impl Sync for DIContext {}

impl std::fmt::Debug for DIContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DIContext")
            .field("containers", &self.containers)
            .field("wait_list.len", &self.wait_list.len())
            .finish()
    }
}

impl DIContext {
    pub fn new() -> Self {
        DIContext {
            containers: HashMap::new(),
            wait_list: vec![],
        }
    }

    pub fn register(&mut self, value: Box<dyn Any>) {
        let type_id = (&*value).type_id();
        if self.containers.contains_key(&type_id) {
            return;
        }

        self.containers.insert(type_id, value);
    }

    pub fn register_lazy<T: 'static>(&mut self, injectable: Box<dyn IProvider>) {
        self.wait_list.push(injectable);
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self.containers.get(&TypeId::of::<T>()) {
            Some(value) => value.downcast_ref::<T>(),
            None => None,
        }
    }
}

impl DIContext {
    fn import_from_modules(&mut self, root_module: Box<dyn crate::IModule>) {
        let providers = root_module.providers();

        for provider in providers {
            self.wait_list.push(provider);
        }

        let child_modules = root_module.child_modules();

        for child_module in child_modules {
            self.import_from_modules(child_module);
        }
    }

    pub fn initialize(&mut self, root_module: Box<dyn crate::IModule>) {
        self.import_from_modules(root_module);

        while self.wait_list.len() > 0 {
            let mut has_ready_provider = false;

            let mut i = 0;
            for provider in self.wait_list.iter() {
                let dependencies = provider.dependencies();

                let mut is_ready = true;

                for dependency in dependencies {
                    if !self.containers.contains_key(&dependency) {
                        is_ready = false;
                        break;
                    }
                }

                if is_ready {
                    let provided_value = provider.provide(self);
                    self.register(provided_value);
                    self.wait_list.remove(i);
                    has_ready_provider = true;

                    break;
                }

                i += 1;
            }

            if !has_ready_provider {
                panic!("No provider is ready");
            }
        }
    }
}

pub trait IProvider {
    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn provide(&self, di_context: &DIContext) -> Box<dyn Any>;
}
