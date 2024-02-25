use piston_window::*;

use rand::{thread_rng, Rng};
use snake::{Direction, Snake};
use utils::{draw_block, draw_rectange};

use crate::snake::Block;

const MOVING_SPEED: f64 = 0.1;
const NUM_FOODS: usize = 3;
const FOOD_DECAY_SPEED: f32 = 0.002;
const INIT_FOOD_LIFE: f32 = 1.0;

#[derive(Debug)]
pub struct Food {
    pub x: i32,
    pub y: i32,
    pub time: f32,
}

pub struct Game {
    pub snake: Snake,

    pub foods: Vec<Food>,

    width: i32,
    height: i32,

    pub is_game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(w: i32, h: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            foods: Vec::new(),
            width: w,
            height: h,
            is_game_over: false,
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.is_game_over = false;
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.is_game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if let Some(d) = dir {
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        for food in &self.foods {
            draw_block(
                [
                    0.67 * 1.0 / food.time,
                    0.48 * food.time,
                    0.34 * food.time,
                    food.time,
                ],
                food.x,
                food.y,
                con,
                g,
            );
        }

        let border_color = [0.0, 0.0, 0.0, 1.0];

        draw_rectange(border_color, 0, 0, self.width, 1, con, g);
        draw_rectange(border_color, 0, self.height - 1, self.width, 1, con, g);
        draw_rectange(border_color, 0, 0, 1, self.height, con, g);
        draw_rectange(border_color, self.width - 1, 0, 1, self.height, con, g);

        if self.is_game_over {
            draw_rectange(
                [0.91, 0.30, 0.24, 0.5],
                0,
                0,
                self.width,
                self.height,
                con,
                g,
            );
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        self.update_food_expired();
        self.update_food_life();

        if self.is_game_over {
            if self.waiting_time > 1.0 {
                self.restart();
            }
            return;
        }

        self.update_food();

        if self.waiting_time > MOVING_SPEED {
            self.update_snake(None);
        }
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.snake.moving_direction = d;
        }
        
        if self.check_snake_alive() {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.is_game_over = true;
        }
        self.waiting_time = 0.0;
    }

    /// Remove expired foods
    pub fn update_food_expired(&mut self) {
        let mut i = 0;
        loop {
            if i >= self.foods.len() {
                break;
            }

            if self.foods[i].time < 0.1 {
                self.foods.remove(i);
                continue;
            }
            i += 1;
        } 
    }

    /// Subtract food time parameter
    fn update_food_life(&mut self) {
        for i in 0..self.foods.len() {
            self.foods[i].time -= FOOD_DECAY_SPEED;
        }
    }

    /// Check if the snake is eating any food
    fn check_eating(&mut self) {
        let pos = self.snake.head_position();

        for i in 0..self.foods.len() {
            let food = & self.foods[i];
            if food.x == pos.0 && food.y == pos.1 {
                self.snake.increase_length();
                self.foods.remove(i);
                return;
            }
        }
    }

    /// Check if the snake is alive
    fn check_snake_alive(&self) -> bool {
        let head_pos = self.snake.head_position();
    
        let next_pos = match self.snake.head_direction() {
            Direction::Up => Block {
                x: head_pos.0,
                y: head_pos.1 + 1
            },
            Direction::Down => Block {
                x: head_pos.0,
                y: head_pos.1 - 1
            },
            Direction::Left => Block {
                x: head_pos.0 - 1,
                y: head_pos.1 
            },
            Direction::Right => Block {
                x: head_pos.0 + 1,
                y: head_pos.1
            }
        };

        if next_pos.x >= self.width || next_pos.x <= 0 || next_pos.y >= self.height || next_pos.y <= 0 {
            return false;
        }

        // iter does not take ownership of the object, therefore since it wasn't assigned to any var , it would have gone out of scope and therefore used into_iter
        let mut body_iter = self.snake.get_body().into_iter();
        let mut next_iter = body_iter.next();
        
        loop {
            if next_iter.is_none() {
                break;
            }

            if next_pos.is_equal(&next_iter.unwrap()) {
                return false;
            }

            next_iter = body_iter.next();
        }

        true
    }

    /// Add food at NUM_FOODS number of places
    fn update_food(&mut self) {
        if self.foods.len() >= NUM_FOODS {
            return;
        }

        let insert_num = NUM_FOODS - self.foods.len();
        let mut rng = thread_rng();

        for _i in 0..insert_num {
            let food = Food {
                x: rng.gen::<i32>(),
                y: rng.gen::<i32>(),
                time: INIT_FOOD_LIFE
            };
            println!("New food: {:?}", food);
            self.foods.push(food);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_collision() {
        let mut game = Game::new(20, 20);

        for _ in 0..14 {
            game.update(1.0);
            assert_eq!(game.is_game_over, false);
        }

        game.update(1.0);
        assert_eq!(game.is_game_over, true);
    }

    #[test]
    fn test_food_collect() {
        let mut game = Game::new(20, 20);

        game.foods.push(Food {
            x: 6,
            y: 2,
            time: 5.0,
        });

        for _ in 0..2 {
            let f = &game.foods[0];
            assert!(f.x == 6 && f.y == 2);

            game.update(1.0);
        }

        game.update(1.0);

        let f = &game.foods[0];
        assert!(!(f.x == 6 && f.y == 2));
    }

    #[test]
    fn test_food_regenerate() {
        let mut game = Game::new(20, 20);
        assert_eq!(game.foods.len(), 0);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);

        game.foods.pop();
        game.foods.pop();
        assert_eq!(game.foods.len(), NUM_FOODS - 2);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);
    }

    #[test]
    fn test_remove_expired_food() {
        let mut game = Game::new(20, 20);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);

        for food in game.foods.iter_mut() {
            food.time = 0.0;
        }

        game.update_food_expired();
        assert_eq!(game.foods.len(), 0);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);
    }

    #[test]
    fn test_decay_food() {
        let mut game = Game::new(20, 20);

        game.update(1.0);
        assert_eq!(game.foods.len(), NUM_FOODS);

        for food in game.foods.iter() {
            assert_eq!(food.time as f32, INIT_FOOD_LIFE);
        }

        game.update(1.0);

        for food in game.foods.iter() {
            assert_eq!(food.time as f32, INIT_FOOD_LIFE - FOOD_DECAY_SPEED);
        }

        game.update(1.0);
    }
}
