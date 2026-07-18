use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

/// Color of a living cell.
pub const ALIVE: Color = Color::new(0, 255, 120, 255);
/// Color of a dead cell.
pub const DEAD: Color = Color::new(10, 10, 20, 255);

/// A cell is alive if it is not the (dark) dead color. Reading the green channel
/// is enough to classify it.
fn is_alive(fb: &Framebuffer, x: i32, y: i32) -> bool {
    fb.get_color(x, y).g > 128
}

/// Advance the board one generation, writing the result back into `fb`.
///
/// Conway's rules on a toroidal (wrap-around) grid: neighbors that fall off one
/// edge are read from the opposite edge. A separate `next` buffer is built first
/// because every cell must be evaluated against the *current* generation.
pub fn step(fb: &mut Framebuffer) {
    let w = fb.width;
    let h = fb.height;
    let mut next = vec![false; (w * h) as usize];

    for y in 0..h {
        for x in 0..w {
            let mut neighbors = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = (x + dx + w) % w;
                    let ny = (y + dy + h) % h;
                    if is_alive(fb, nx, ny) {
                        neighbors += 1;
                    }
                }
            }

            let alive = is_alive(fb, x, y);
            // survival: 2-3 neighbors; reproduction: dead cell with exactly 3.
            let next_alive = matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            next[(y * w + x) as usize] = next_alive;
        }
    }

    for y in 0..h {
        for x in 0..w {
            let color = if next[(y * w + x) as usize] { ALIVE } else { DEAD };
            fb.set_current_color(color);
            fb.set_pixel(x, y);
        }
    }
}
