use piston_window::{rectangle, Context, G2d};  // like from piston_window import ...
use piston_window::types::Color;


const BLOCK_SIZE: f64 = 25.0;


// scale up the discreate game coordinates into 
// the grapical coordinates on the screen
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE   // return 
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32   // return 
}


// draw a rectangle given its coordinates of O and its width and height
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    // draws a rectangle window on the screen
    rectangle(
        color,
        [
            gui_x, 
            gui_y, 
            BLOCK_SIZE * (width as f64), 
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}


// draw a single block given its coordinatese
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    // draws a rectangle window on the screen
    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}