
use std::any::Any;
use bevy::app::App;
use crate::Registry;

pub trait AppRegistryExt {
    fn init_registry<I: Any>(&mut self) -> &mut Self;
    fn insert_registry<I: Any>(&mut self, registry: Registry<I>) -> &mut Self;
}

impl AppRegistryExt for App {
    fn init_registry<I: Any>(&mut self) -> &mut Self {
        self.insert_resource(Registry::<I>::new());
        self
    }

    fn insert_registry<I: Any>(&mut self, registry: Registry<I>) -> &mut Self {
        self.insert_resource(registry);
        self
    }
}

