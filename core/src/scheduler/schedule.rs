use std::time::Duration;

use crate::{
    system::{IntoSystem, System, SystemParam},
    Plugin, PluginBuilder, Storage,
};

pub type Systems = Vec<Box<dyn System>>;

pub struct Scheduler {
    startup_systems: Systems,
    systems: Systems,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            systems: Vec::new(),
            startup_systems: Vec::new(),
        }
    }

    pub fn add_system<F, Params>(&mut self, system: F) -> &mut Self
    where
        F: IntoSystem<Params> + 'static,
        Params: SystemParam,
    {
        self.systems.push(Box::new(system.into(None)));
        self
    }

    pub fn add_startup_system<F, Params>(&mut self, system: F) -> &mut Self
    where
        F: IntoSystem<Params> + 'static,
        Params: SystemParam,
    {
        self.startup_systems.push(Box::new(system.into(None)));
        self
    }

    pub fn add_interval_system<F, Params>(
        &mut self,
        system: F,
        call_interval: Duration,
    ) -> &mut Self
    where
        F: IntoSystem<Params> + 'static,
        Params: SystemParam,
    {
        self.systems
            .push(Box::new(system.into(Some(call_interval))));
        self
    }

    pub fn merge_systems(&mut self, mut systems: Systems) {
        self.systems.append(&mut systems);
    }

    pub fn merge_startup_systems(&mut self, mut systems: Systems) {
        self.startup_systems.append(&mut systems);
    }

    pub fn add_plugin<P: Plugin>(&mut self) -> &mut Self {
        let mut plugin = PluginBuilder::new();

        // Fill the instance of plugin
        P::new(&mut plugin);

        self.merge_systems(plugin.systems);
        self.merge_startup_systems(plugin.startup_systems);

        self
    }

    pub fn run(&mut self, storage: &mut Storage) {
        Self::run_systems(&mut self.startup_systems, storage);
        Self::run_systems(&mut self.systems, storage);

        self.startup_systems.clear();
    }

    pub fn run_systems(systems: &mut Systems, storage: &mut Storage) {
        systems.iter_mut().for_each(|system| {
            system.run(storage);
        });
    }
}
