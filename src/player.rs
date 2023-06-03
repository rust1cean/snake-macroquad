use macroquad::prelude::{is_key_pressed, KeyCode};

use crate::{
    cfg::{CELL_COUNT, GAME_OVER, SNAKE_COLOR, SNAKE_SIZE, SNAKE_STEP_INTERVAL, SNAKE_X, SNAKE_Y},
    food::Food,
    game::Position,
    Rect, Shape,
};
use core::{storage::Component, Plugin, PluginBuilder, Storage};
use std::any::Any;

#[derive(Debug)]
pub struct Player {
    head: Rect,
    position: Position,
    direction: Direction,
    tail: Vec<Position>,
}

impl Plugin for Player {
    fn new(builder: &mut PluginBuilder) {
        builder
            .add_startup_system(Self::init)
            .add_system(Self::translate_position)
            .add_system(Self::draw)
            .add_system(Self::controls)
            .add_system(Self::out_bounds)
            .add_interval_system(Self::eat, SNAKE_STEP_INTERVAL)
            .add_interval_system(Self::moving_at_grid, SNAKE_STEP_INTERVAL)
            .add_interval_system(Self::cannibalism, SNAKE_STEP_INTERVAL);
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            head: Rect {
                x: 0.,
                y: 0.,
                width: SNAKE_SIZE,
                height: SNAKE_SIZE,
                color: SNAKE_COLOR,
            },
            position: Position(SNAKE_X, SNAKE_Y),
            direction: Direction::None,
            tail: Vec::new(),
        }
    }
}

// Systmes
impl Player {
    pub fn init(storage: &mut Storage) {
        storage.add(Self::default());
    }

    pub fn draw(storage: &mut Storage) {
        if let Some(snake) = storage.get_first_mut::<Player>() {
            // Draw head
            snake.head.draw();

            // Draw tail
            snake.tail.iter().for_each(|segment| {
                Rect::draw(
                    Position::compute(segment.0),
                    Position::compute(segment.1),
                    snake.head.width,
                    snake.head.height,
                    snake.head.color,
                )
            });
        }
    }

    pub fn moving_at_grid(storage: &mut Storage) {
        if let Some(snake) = storage.get_first_mut::<Player>() {
            // Move tail
            if snake.tail.len() > 0 {
                let Position(mut x, mut y) = snake.position;

                snake.tail.iter_mut().for_each(|segment| {
                    (x, segment.0) = (segment.0, x);
                    (y, segment.1) = (segment.1, y);
                });
            }

            // Move head
            {
                snake.position.0 += match snake.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                    _ => 0,
                };

                snake.position.1 += match snake.direction {
                    Direction::Top => -1,
                    Direction::Down => 1,
                    _ => 0,
                };
            }
        }
    }

    pub fn translate_position(storage: &mut Storage) {
        if let Some(snake) = storage.get_first_mut::<Player>() {
            snake.head.x = Position::compute(snake.position.0);
            snake.head.y = Position::compute(snake.position.1);
        }
    }

    pub fn controls(storage: &mut Storage) {
        if let Some(snake) = storage.get_first_mut::<Player>() {
            if is_key_pressed(KeyCode::W) && snake.direction != Direction::Down {
                snake.direction = Direction::Top;
            } else if is_key_pressed(KeyCode::A) && snake.direction != Direction::Right {
                snake.direction = Direction::Left;
            } else if is_key_pressed(KeyCode::S) && snake.direction != Direction::Top {
                snake.direction = Direction::Down;
            } else if is_key_pressed(KeyCode::D) && snake.direction != Direction::Left {
                snake.direction = Direction::Right;
            }
        }
    }

    pub fn eat(storage: &mut Storage) {
        let mut can_grow = false;

        // Eat
        if let Some((snake, food)) = storage.get_several_mut::<Player, Food>() {
            let mut index = None;

            for (idx, f) in food.iter().enumerate() {
                if *f.position() == snake[0].position {
                    index = Some(idx);
                    break;
                }
            }

            if let Some(idx) = index {
                storage.remove::<Food>(idx);

                can_grow = true;
            }
        }

        // Snake growth
        if can_grow {
            if let Some(snake) = storage.get_first_mut::<Player>() {
                let segment: Position = match snake.tail.len() {
                    0 => snake.position.clone(),
                    len => snake.tail[len - 1].clone(),
                };

                snake.tail.push(segment);
            }
        }
    }

    pub fn out_bounds(storage: &mut Storage) {
        if let Some(snake) = storage.get_first::<Player>() {
            if (snake.position.0 < 0 || snake.position.0 > (CELL_COUNT - 1_u8).into())
                || (snake.position.1 < 0 || snake.position.1 > (CELL_COUNT - 1_u8).into())
            {
                panic!("{GAME_OVER}\nYour snake is out of bounds");
            }
        }
    }

    pub fn cannibalism(storage: &mut Storage) {
        if let Some(snake) = storage.get_first::<Player>() {
            snake.tail.iter().for_each(|segment| {
                if *segment == snake.position {
                    panic!("{GAME_OVER}\nYour snake has eaten its tail")
                }
            })
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Top,
    Down,
    Left,
    Right,
    None,
}

// TODO: Clean up this crap after adding macros to the repository...

impl Component for Player {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
