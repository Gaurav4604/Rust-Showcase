// getting external libraries into the system
extern crate piston_window;
extern crate rand;

// binding the files to the main file
mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord;
use crate::game::Game;

const BACK_COLOR: Color = [0.0, 0.0, 0.0, 1.0];

fn main() {
    // making a game board of size 30 px by 30px
    let (width, height) = (30, 30);
    // making a window for rendering the graphics
    // with name and coordinates
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord(width), to_coord(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    // initializing game variable
    let mut game = Game::new(width, height);
    // while event happens, refresh the window
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            // looking for the event type
            game.key_pressed(key); // if it matchs the required up down left right operation, then do something
        }
        // rendering the actual graphics
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g); // clearing the screen after every instance
            game.draw(&c, g); // redrawing graphics again
        });
        // updating the changes onto the graphics screen
        event.update(|arg| {
            game.update(arg.dt); // calling game update function to update game variables
        });
    }
}
