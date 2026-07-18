use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    color_buffer: Image,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let color_buffer = Image::gen_image_color(width, height, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            current_color: Color::WHITE,
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer =
            Image::gen_image_color(self.width, self.height, self.current_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            // flip y: incoming coords use bottom-left origin, image uses top-left
            let flipped_y = self.height - 1 - y;
            self.color_buffer.draw_pixel(x, flipped_y, self.current_color);
        }
    }

    /// Read the color of a cell. Uses the same bottom-left origin / Y-flip
    /// convention as `set_pixel`, so `get_color` reads back what `set_pixel` wrote.
    /// Used by the Game of Life engine to inspect each cell's state.
    #[allow(dead_code)]
    pub fn get_color(&self, x: i32, y: i32) -> Color {
        let flipped_y = self.height - 1 - y;
        self.color_buffer.get_color(x, flipped_y)
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    /// Upload the small framebuffer to the GPU and draw it stretched to fill the
    /// window (nearest-neighbor, so cells stay crisp squares). This is what lets
    /// the framebuffer be smaller than the window.
    #[allow(dead_code)]
    pub fn swap_buffers(&self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(thread, &self.color_buffer) {
            let scale = window.get_screen_width() as f32 / self.width as f32;
            let mut d = window.begin_drawing(thread);
            d.clear_background(Color::BLACK);
            d.draw_texture_ex(&texture, Vector2::new(0.0, 0.0), 0.0, scale, Color::WHITE);
        }
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
