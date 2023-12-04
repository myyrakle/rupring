pub use rupring_macro::{Controller, Delete, Get, Injectable, Module, Patch, Post, Put};

pub trait IModule {
    fn child_modules(&self) -> Vec<Box<dyn IModule>>;
    fn controllers(&self) -> Vec<Box<dyn IController>>;
}

pub trait IController {
    fn routes(&self) -> Vec<Box<dyn IRoute>>;
}

pub trait IRoute {
    fn method(&self) -> String;
    fn path(&self) -> String;
    fn handler(&self) -> Box<dyn IHandler>;
}

pub trait IHandler {
    fn handle(&self);
}

pub struct Rupring<T: IModule> {
    root_module: T,
}

impl<T: IModule> Rupring<T> {
    pub fn create(module: T) -> Self {
        Rupring {
            root_module: module,
        }
    }

    pub async fn listen(&self, port: u16) {}
}
