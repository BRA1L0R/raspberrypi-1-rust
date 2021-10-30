use super::pixel::Pixel;

pub struct FrameBuffer<'a> {
    pitch: u32,

    rows: u32,
    columns: u32,

    buffer: &'a mut [Pixel],
}

impl<'a> FrameBuffer<'a> {
    pub fn uninitialized() -> FrameBuffer<'a> {
        FrameBuffer {
            buffer: &mut [],
            pitch: 0,
            rows: 0,
            columns: 0,
        }
    }

    pub fn new(pitch: u32, rows: u32, columns: u32, buffer: &'a mut [Pixel]) -> FrameBuffer<'a> {
        FrameBuffer {
            pitch,
            rows,
            buffer,
            columns,
        }
    }

    pub fn get_pitch(&self) -> u32 {
        self.pitch
    }

    pub fn get_rows(&self) -> u32 {
        self.rows
    }

    pub fn get_buffer(&mut self) -> &mut [Pixel] {
        self.buffer
    }

    pub fn get_columns(&mut self) -> u32 {
        self.columns
    }
}
