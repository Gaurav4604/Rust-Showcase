use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng}; // for using Random generator, using OS

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake}; // for implementing snake object // for drawing the graphics onto the screen

// defining item colors
const FOOD_COLOR: Color = [1.0, 0.0, 0.0, 1.0]; //red
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; //black
const GAME_OVER: Color = [0.90, 0.0, 0.0, 0.5]; // redder, more transparent red

const MOVING_PERIOD: f64 = 0.1; // moves one block in 0.1 sec
const RESTART_TIME: f64 = 1.0; // restarts game after a second of game over

pub struct Game {
    snake: Snake,
    food_exists: bool,
    // coordinates of food
    food_x: i32,
    food_y: i32,
    //size of game board
    width: i32,
    height: i32,
    // attributes of game struct
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    // making a new game object
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2), // two px from top and left, is position of snake on board
            // placing the food onto game board
            food_exists: true,
            food_x: 6,
            food_y: 4,
            // specifying game board with
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }
        // storing the direction onto a variable
        // by extracting value of pressed key
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };
        // if movement to be done is in exact opposite direction of movement
        // then exit the function
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }
        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        // borders for all the directions of the game
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            draw_rectangle(GAME_OVER, 0, 0, self.width, self.height, con, g);
        }
    }
    // updation function to check if particular dependency
    // has been satisfied or not
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            self.add_food();
        }
        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }
    // check if snake has eaten the food
    // if so, append the tail
    // and set food exists to false
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position(); // unwrapping coordinates of the head of snake
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            //checking if coordinates of head and food are same
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }
    // checking if snake overlaps with itself or collides with the boundaries of the game
    fn check_if_snake_alive(&mut self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
    // adding new food each time snake consumes any food
    fn add_food(&mut self) {
        let mut rng = thread_rng(); // instantiating a random generator
                                    // generating random coordinates for the food to appear
        let mut new_x: i32 = rng.gen_range(1..=self.width - 1);
        let mut new_y: i32 = rng.gen_range(1..=self.height - 1);

        // check if food generated is not on the body of snake
        // if so, keep generating new food till it is not on the body
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..=self.width - 1);
            new_y = rng.gen_range(1..=self.width - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }
    // updating the snake graphic
    fn update_snake(&mut self, dir: Option<Direction>) {
        // checking if snake is alive
        // if so, make it move forward
        // and check if it eats the food
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    // restarting the game at the exact same coordinates
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.waiting_time = 0.0;
    }
}
