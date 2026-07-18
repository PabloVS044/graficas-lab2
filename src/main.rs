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

    // Still lifes
    patterns::block(&mut fb, 5, 5);
    patterns::beehive(&mut fb, 14, 5);
    patterns::loaf(&mut fb, 24, 5);
    patterns::boat(&mut fb, 34, 5);
    patterns::tub(&mut fb, 42, 5);

    // Oscillators
    patterns::blinker(&mut fb, 6, 20);
    patterns::toad(&mut fb, 14, 20);
    patterns::beacon(&mut fb, 24, 20);
    patterns::pulsar(&mut fb, 40, 18);
    patterns::pentadecathlon(&mut fb, 60, 24);

    // Spaceships
    patterns::glider(&mut fb, 5, 55);
    patterns::lwss(&mut fb, 20, 55);
    patterns::mwss(&mut fb, 40, 55);
    patterns::hwss(&mut fb, 60, 55);

    // Gun
    patterns::gosper_gun(&mut fb, 5, 80);

    while !rl.window_should_close() {
        fb.swap_buffers(&mut rl, &thread);
        conway::step(&mut fb);
    }
}
