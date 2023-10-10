pub use rupring_macro::{Controller, Delete, Get, Injectable, Module, Patch, Post, Put};

pub trait IModule {}

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
