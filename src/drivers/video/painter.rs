use super::{pixel::Pixel, FrameBuffer};

pub struct Painter<'a>(&'a mut FrameBuffer<'a>);

impl<'a> Painter<'a> {
    pub fn new(fb: &'a mut FrameBuffer<'a>) -> Painter<'a> {
        Painter(fb)
    }

    fn buffer(&mut self) -> &mut [Pixel] {
        self.0.get_buffer()
    }

    pub fn pixel(&mut self, x: u32, y: u32, pix: Pixel) {
        let col = self.0.get_columns();
        self.buffer()[(y * col + x) as usize] = pix
    }

    pub fn pixel_rgb(&mut self, x: u32, y: u32, rgb: u32) {
        self.pixel(x, y, Pixel::hex(rgb))
    }

    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Pixel) {
        let (dx, dy) = ((x1 - x0).abs(), -((y1 - y0).abs()));

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            self.pixel(x0 as u32, y0 as u32, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let err2 = 2 * err;
            if err2 >= dy {
                err += dy;
                x0 += sx;
            }
            if err2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn clear_screen(&mut self) {
        self.buffer().iter_mut().for_each(|p| *p = Pixel::hex(0));
    }
}
