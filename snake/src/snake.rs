use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList; //used for manipulating snake head and tail
                                  // the imports used in the current file and previous one are for Context -> context window, G2d -> graphics buffer

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 1.0, 0.00, 1.0]; // follow % scale of rgba standard

// handles the movement of snake and keyboard input
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // if user tries to go in an exact opposite direction, return a value which is reverse, thus not enabling the user to do so
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
// generates blocks for the snake to grow upon
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}
// class that controls the snake
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>, // helps the snake grow
    tail: Option<Block>, // tail is not deleted when tail moves forward, but grows when it eats the apple, thus it needs to be an Option
}

// functions for the class Snake
impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // making snake body of 3 blocks, in horizontal direction
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x: x, y });
        // returning a snake object
        Snake {
            direction: Direction::Right, // going from left to right
            body,                        // having a 3 block body
            tail: None,                  // no length increase in the tail
        }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g); // drawing the snake with iteration for each block
        }
    }
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap(); // unwrapping position of head onto the head_block
        (head_block.x, head_block.y) // returning x,y coordinates of the head block
    }
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // setting up direction, if a valid direction is given
        match dir {
            Some(d) => self.direction = d,
            None => {}
        }
        // storing previous head in temp variable
        let (last_x, last_y) = self.head_position();
        // creating a new block based on the direction
        // that the snake will go in
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        // applying the new block to be the new front of the snake
        self.body.push_front(new_block);
        // removing and storing the last block onto the a temp
        let removed_block = self.body.pop_back().unwrap();
        // assigning the last old block onto the tail of the snake
        // to allow the tail to trail along with the snake
        self.tail = Some(removed_block);
    }

    // returning direction of the head
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // placing the position of the next head
    pub fn next_head(&mut self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position(); // copying old head position onto a temp
        let mut moving_dir = self.direction; // making a moving_dir temp that has old direction of snake
        match dir {
            Some(d) => moving_dir = d, //assigning new direction to snake
            None => {}
        }
        // increasing accuracy in predicting position of head
        // after the movement of the snake
        // by setting values of enums as the following coordinates
        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }
    // increasing tail size after snake eats apple
    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap(); // copying the old tail value onto a block as its tuple
        self.body.push_back(block); // increasing the tail to trail behind snake
    }

    // checking if the snake overlaps with itself
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true; //checks if the snake overlaps with itself
            }
            ch += 1;
            // at an instance, snakes head might overlap with its tail unwantedly
            // thus check for such a condition and break the loop
            // since the process of moving blocks of snake is sequential
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}
