use crate::{cfg::CELL_SIZE, food::Food, player::Player};
use core::{storage::Component, Plugin, PluginBuilder};
use rand::Rng;
use std::{any::Any, ops::Range};

pub struct Game;

impl Plugin for Game {
    fn new(builder: &mut PluginBuilder) {
        builder.add_plugin::<Player>().add_plugin::<Food>();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn compute(value: i32) -> f32 {
        value as f32 * CELL_SIZE.floor()
    }

    pub fn rand(x: Range<i32>, y: Range<i32>) -> Self {
        let mut rng = rand::thread_rng();

        // Random position at grid
        let x = rng.gen_range(x);
        let y = rng.gen_range(y);

        Self(x, y)
    }
}

// TODO: Clean up this crap after adding macros to the repository...

impl Component for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
