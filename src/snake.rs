use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;


// red, green, blue, opacidy
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.00];


// directions of snake
#[derive(Copy, Clone, PartialEq)]   // PartialEq makes equivalence evaluation easier
pub enum Direction {
    Up,
    Down, 
    Left,
    Right,
}


impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}


// body component of the snake
#[derive(Debug, Clone)]
struct Block {
    // position of the block
    x: i32,
    y: i32,
}


pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,      // tail needs to be Option
}


impl Snake {
    // similar to a constructor
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // the initial snake has a length of 3
        body.push_back(Block{
            x: x+2,
            y,
        });
        body.push_back(Block{
            x: x+1,
            y,
        });
        body.push_back(Block{
            x,
            y,
        });

        Snake {
            direction: Direction::Right,  // initial snake direction is Right
            body,
            tail: None,
        }
    }

    // draw the body of the snake
    pub fn draw(&self, con: &Context, g:&mut G2d) {
        // query element in list by "for e in LinkedList {}"
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // return the head position of the snake
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // move snake forward along a given direction (or not given)
    // no return type or return value
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        // since dir is Option<Direction>, it can be None
        match dir {
            Some(d) => self.direction = d,
            None => (),    // do nothing
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            // move upward, 
            // the y of the new head is y-1 of the current head
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            // move downward, 
            // the y of the new head is y+1 of the current head
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            // move left, 
            // the y of the new head is x-1 of the current head
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            // move right, 
            // the y of the new head is x+1 of the current head
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        // Realize the movement of snake by adding one block to the front
        // and remove one block from the back
        // the new block is pushed into the LinkedList as the new head
        self.body.push_front(new_block);
        // the tail is poped
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // look ahead according to a tentative direction
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => (),
        }

        // return the next head position
        match moving_dir {
            Direction::Up => (head_x, head_y-1),
            Direction::Down => (head_x, head_y+1),
            Direction::Left => (head_x-1, head_y),
            Direction::Right => (head_x+1, head_y),
        }
    }

    // if we eat some food, re run this method to increase one block to the body
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    // According to the Snake rules, the snake cannot have a cycle formed by its body
    // The snake is not allowed to touch the border, either.
    // return true if the snake body is overlapped with the provided x,y
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        // if the position (x,y) is the same as one of the body blocks of the snake,
        // return true
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            
            // skip the snake tail
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        return false;
    }
}