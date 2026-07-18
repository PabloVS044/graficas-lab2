mod framebuffer;
mod conway;

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
    fb.set_current_color(conway::DEAD);
    fb.clear();

    // Temporary seed to show the engine running: a glider and a blinker.
    // A proper organism library and initial pattern come next.
    fb.set_current_color(conway::ALIVE);
    for &(x, y) in &[(11, 80), (12, 79), (10, 78), (11, 78), (12, 78)] {
        fb.set_pixel(x, y); // glider
    }
    for &(x, y) in &[(50, 50), (51, 50), (52, 50)] {
        fb.set_pixel(x, y); // blinker
    }

    while !rl.window_should_close() {
        // Draw the current generation, then compute the next one.
        // The framebuffer is never cleared: the game logic owns every cell.
        fb.swap_buffers(&mut rl, &thread);
        conway::step(&mut fb);
    }
}
