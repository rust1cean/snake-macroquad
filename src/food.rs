use crate::{
    cfg::{CELL_COUNT, FOOD_COLOR, FOOD_SIZE, FOOD_SPAWN_INTERVAL, MAX_FOOD},
    game::Position,
    Rect, Shape,
};
use core::{storage::Component, Plugin, PluginBuilder, Storage};
use std::any::Any;

#[derive(Debug)]
pub struct Food {
    shape: Rect,
    position: Position,
}

impl Plugin for Food {
    fn new(builder: &mut PluginBuilder) {
        builder
            .add_interval_system(Food::spawn, FOOD_SPAWN_INTERVAL)
            .add_system(Food::draw);
    }
}

impl Default for Food {
    fn default() -> Self {
        let position = Position::rand(0..CELL_COUNT as i32, 0..CELL_COUNT as i32);

        Food {
            shape: Rect {
                x: Position::compute(position.0),
                y: Position::compute(position.1),
                width: FOOD_SIZE,
                height: FOOD_SIZE,
                color: FOOD_COLOR,
            },
            position,
        }
    }
}

// Methods
impl Food {
    pub fn position(&self) -> &Position {
        &self.position
    }
}

// Systmes
impl Food {
    pub fn spawn(storage: &mut Storage) {
        // If there is more food - ignore
        if let Some(food) = storage.get_all::<Food>() {
            if (food.len()) as u8 >= MAX_FOOD {
                return ();
            }
        }

        storage.add(Food::default());
    }

    pub fn draw(storage: &mut Storage) {
        if let Some(mut food) = storage.get_all_mut::<Food>() {
            food.iter_mut().for_each(|f| f.shape.draw());
        }
    }
}

// TODO: Clean up this crap after adding macros to the repository...

impl Component for Food {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
