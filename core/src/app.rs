use std::time::Duration;

use crate::scheduler::{IntoSystem, Scheduler, SystemParam};
use crate::{Plugin, Storage};
use macroquad::prelude::*;

pub struct App {
    pub(crate) storage: Storage,
    pub(crate) scheduler: Scheduler,
    pub(crate) config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
        let scheduler: Scheduler = Scheduler::new();
        let storage: Storage = Storage::new();

        Self {
            scheduler,
            config,
            storage,
        }
    }

    pub fn add_system<F: IntoSystem<Params> + 'static, Params: SystemParam>(
        &mut self,
        system: F,
    ) -> &mut Self {
        self.scheduler.add_system(system);
        self
    }

    pub fn add_startup_system<F: IntoSystem<Params> + 'static, Params: SystemParam>(
        &mut self,
        system: F,
    ) -> &mut Self {
        self.scheduler.add_startup_system(system);
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
        self.scheduler.add_interval_system(system, call_interval);
        self
    }

    pub fn add_plugin<P: Plugin>(&mut self) -> &mut Self {
        self.scheduler.add_plugin::<P>();
        self
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(self.config.background_color);
            self.scheduler.run(&mut self.storage);
            next_frame().await;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub background_color: Color,
}
