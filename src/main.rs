mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    // Low-resolution simulation grid, upscaled to a larger window.
    const GRID_W: i32 = 100;
    const GRID_H: i32 = 100;
    const SCALE: i32 = 8;

    let win_w = GRID_W * SCALE;
    let win_h = GRID_H * SCALE;

    let (mut rl, thread) = raylib::init()
        .size(win_w, win_h)
        .title("Lab 2 - Conway's Game of Life")
        .build();

    // One frame per generation; 12 fps is a comfortable viewing speed.
    rl.set_target_fps(12);

    let mut fb = Framebuffer::new(GRID_W, GRID_H);
    fb.set_current_color(Color::BLACK);
    fb.clear();

    // Temporary: a diagonal to confirm the low-res buffer scales to the window.
    // The game logic will replace this once Conway's rules are in place.
    fb.set_current_color(Color::WHITE);
    for i in 0..GRID_W.min(GRID_H) {
        fb.set_pixel(i, i);
    }

    while !rl.window_should_close() {
        fb.swap_buffers(&mut rl, &thread);
    }
}
