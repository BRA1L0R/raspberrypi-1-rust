use crate::utils::AddressTraslation;

use super::mail::Mail;

pub trait Mailable {
    fn write_mail(&mut self) -> Mail;
}

#[repr(C)]
#[derive(Debug)]
pub struct FrameBufferMail {
    pub width: u32,
    pub height: u32,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub pitch: u32,
    pub depth: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub pointer: u32,
    pub size: u32,
}

impl Mailable for FrameBufferMail {
    fn write_mail(&mut self) -> Mail {
        Mail::new()
            .data()
            .set_noshift((self as *mut Self as u32).physical_to_uncachedbus())
            .chan()
            .set(1)
    }
}

pub struct MailStatus(u32);

impl MailStatus {
    pub fn is_empty(&self) -> bool {
        ((self.0 & (1 << 30)) != 0) as bool
    }

    pub fn is_full(&self) -> bool {
        ((self.0 & (1 << 31)) != 0) as bool
    }
}

impl From<u32> for MailStatus {
    fn from(status: u32) -> Self {
        Self(status)
    }
}
