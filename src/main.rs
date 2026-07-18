mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;

fn main() {
    // Base del proyecto: un framebuffer por software de baja resolución.
    // Por ahora solo verifica la primitiva `set_pixel` exportando a PNG;
    // el renderizado en tiempo real y el Game of Life llegan después.
    let mut fb = Framebuffer::new(100, 100);

    fb.set_current_color(Color::BLACK);
    fb.clear();

    fb.set_current_color(Color::WHITE);
    for i in 0..100 {
        fb.set_pixel(i, i);
    }

    fb.render_to_file("out.png");
}
