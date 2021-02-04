use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0; // 25 px block

pub fn to_coord(game_coord: i32) -> f64 {
    // making the function public using pub keyword
    (game_coord as f64) * BLOCK_SIZE // recasting a coordinate based on the block size of the game
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    //making coordinates as per block size
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], //passing coordinates and px size in width and height
        con.transform,                          // passes 2d context transform
        g,                                      // passing the graphics reference
    )
}

// similar function to help control the size of board used
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    )
}
