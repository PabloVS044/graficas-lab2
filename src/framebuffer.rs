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

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
    }
}
