use crate::conway::ALIVE;
use crate::framebuffer::Framebuffer;

/// Set a single cell alive at (x, y).
fn set(fb: &mut Framebuffer, x: i32, y: i32) {
    fb.set_current_color(ALIVE);
    fb.set_pixel(x, y);
}

/// Place a list of relative live cells at an origin offset.
fn place(fb: &mut Framebuffer, ox: i32, oy: i32, cells: &[(i32, i32)]) {
    for &(x, y) in cells {
        set(fb, ox + x, oy + y);
    }
}

// ---------------------------------------------------------------------------
// Still lifes
// ---------------------------------------------------------------------------

/// Block — 2x2 still life.
pub fn block(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (1, 1)]);
}

/// Beehive — still life.
pub fn beehive(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (2, 2)]);
}

/// Loaf — still life.
pub fn loaf(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (0, 1), (3, 1), (1, 2), (3, 2), (2, 3)]);
}

/// Boat — still life.
pub fn boat(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)]);
}

/// Tub — still life.
pub fn tub(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (0, 1), (2, 1), (1, 2)]);
}

// ---------------------------------------------------------------------------
// Oscillators
// ---------------------------------------------------------------------------

/// Blinker — period 2.
pub fn blinker(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(0, 0), (1, 0), (2, 0)]);
}

/// Toad — period 2.
pub fn toad(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]);
}

/// Beacon — period 2 (two blocks touching at a corner).
pub fn beacon(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(
        fb,
        ox,
        oy,
        &[(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)],
    );
}

/// Pulsar — period 3, 13x13.
pub fn pulsar(fb: &mut Framebuffer, ox: i32, oy: i32) {
    #[rustfmt::skip]
    let cells = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    place(fb, ox, oy, &cells);
}

/// Pentadecathlon — period 15.
pub fn pentadecathlon(fb: &mut Framebuffer, ox: i32, oy: i32) {
    #[rustfmt::skip]
    let cells = [
        (2, 0), (7, 0),
        (0, 1), (1, 1), (3, 1), (4, 1), (5, 1), (6, 1), (8, 1), (9, 1),
        (2, 2), (7, 2),
    ];
    place(fb, ox, oy, &cells);
}

// ---------------------------------------------------------------------------
// Spaceships
// ---------------------------------------------------------------------------

/// Glider — smallest spaceship, travels diagonally.
pub fn glider(fb: &mut Framebuffer, ox: i32, oy: i32) {
    place(fb, ox, oy, &[(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]);
}

/// Lightweight spaceship (LWSS) — travels orthogonally.
pub fn lwss(fb: &mut Framebuffer, ox: i32, oy: i32) {
    #[rustfmt::skip]
    let cells = [
        (1, 0), (4, 0),
        (0, 1),
        (0, 2), (4, 2),
        (0, 3), (1, 3), (2, 3), (3, 3),
    ];
    place(fb, ox, oy, &cells);
}

/// Middleweight spaceship (MWSS).
pub fn mwss(fb: &mut Framebuffer, ox: i32, oy: i32) {
    #[rustfmt::skip]
    let cells = [
        (3, 0),
        (1, 1), (5, 1),
        (0, 2),
        (0, 3), (5, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4),
    ];
    place(fb, ox, oy, &cells);
}

/// Heavyweight spaceship (HWSS).
pub fn hwss(fb: &mut Framebuffer, ox: i32, oy: i32) {
    #[rustfmt::skip]
    let cells = [
        (3, 0), (4, 0),
        (1, 1), (6, 1),
        (0, 2),
        (0, 3), (6, 3),
        (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4),
    ];
    place(fb, ox, oy, &cells);
}

// ---------------------------------------------------------------------------
// Guns
// ---------------------------------------------------------------------------

/// Gosper glider gun — emits a new glider every 30 generations.
pub fn gosper_gun(fb: &mut Framebuffer, ox: i32, oy: i32) {
    #[rustfmt::skip]
    let cells = [
        (0, 4), (0, 5), (1, 4), (1, 5),
        (10, 4), (10, 5), (10, 6),
        (11, 3), (11, 7),
        (12, 2), (12, 8), (13, 2), (13, 8),
        (14, 5),
        (15, 3), (15, 7),
        (16, 4), (16, 5), (16, 6),
        (17, 5),
        (20, 2), (20, 3), (20, 4), (21, 2), (21, 3), (21, 4),
        (22, 1), (22, 5),
        (24, 0), (24, 1), (24, 5), (24, 6),
        (34, 2), (34, 3), (35, 2), (35, 3),
    ];
    place(fb, ox, oy, &cells);
}
