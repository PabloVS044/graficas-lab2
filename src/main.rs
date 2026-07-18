mod framebuffer;
mod conway;
mod patterns;

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

    rl.set_target_fps(12);

    let mut fb = Framebuffer::new(GRID_W, GRID_H);
    fb.set_current_color(conway::DEAD);
    fb.clear();

    patterns::seed(&mut fb);

    while !rl.window_should_close() {
        fb.swap_buffers(&mut rl, &thread);
        conway::step(&mut fb);
    }
}
