use core::app::Config;
use macroquad::prelude::{Color, BLACK, GREEN, PURPLE};
use std::time::Duration;

// App config
pub const APP_CONFIG: Config = Config {
    background_color: WINDOW_BACKGROUND,
};

// Game config
pub const GAME_OVER: &str = "Game over.";

// Window
pub const WINDOW_TITLE: &str = "Snake game";
pub const WINDOW_WIDTH: i32 = 400;
pub const WINDOW_HEIGHT: i32 = WINDOW_WIDTH;
pub const WINDOW_BACKGROUND: Color = BLACK;

// Grid
pub const GRID_SIZE: f32 = WINDOW_WIDTH as f32;
pub const CELL_COUNT: u8 = 16;
pub const CELL_SIZE: f32 = GRID_SIZE / CELL_COUNT as f32;

// Snake
pub const SNAKE_X: i32 = 0;
pub const SNAKE_Y: i32 = 0;
pub const SNAKE_SIZE: f32 = CELL_SIZE;
pub const SNAKE_COLOR: Color = GREEN;
pub const SNAKE_STEP_INTERVAL: Duration = Duration::from_millis(125);

// Food
pub const MAX_FOOD: u8 = 1;
pub const FOOD_SIZE: f32 = CELL_SIZE;
pub const FOOD_COLOR: Color = PURPLE;
pub const FOOD_SPAWN_INTERVAL: Duration = SNAKE_STEP_INTERVAL;
