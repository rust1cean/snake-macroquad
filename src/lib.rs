use cfg::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use macroquad::{prelude::Color, shapes::draw_rectangle, window::Conf};

pub use game::Game;

pub mod cfg;
pub mod food;
pub mod game;
pub mod player;

pub fn window_config() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        icon: None,
        ..Default::default()
    }
}

pub trait Shape: Send + 'static {
    fn draw(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rect {
    pub const fn new(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }
}

impl Shape for Rect {
    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
    }
}

impl Rect {
    fn draw(x: f32, y: f32, width: f32, height: f32, color: Color) {
        draw_rectangle(x, y, width, height, color);
    }
}
