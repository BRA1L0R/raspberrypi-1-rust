#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
}

impl Pixel {
    pub fn hex(rgb: u32) -> Pixel {
        Pixel {
            blue: rgb as u8,
            green: (rgb >> 8) as u8,
            red: (rgb >> 16) as u8,
        }
    }

    pub fn to_u32(self) -> u32 {
        (self.blue as u32) + ((self.green as u32) << 8) + ((self.red as u32) << 16)
    }
}
