use super::{
    mail::Mail,
    structs::{MailStatus, Mailable},
};
use crate::{bases::PHYS_ADDR, utils::data_barrier};

const VIDEO_ADDR: u32 = PHYS_ADDR + 0xB880;
const MAIL0_READ: *mut u32 = VIDEO_ADDR as _;
const MAIL0_WRITE: *mut u32 = (VIDEO_ADDR + 0x20) as _;
const MAIL0_STATUS: *mut u32 = (VIDEO_ADDR + 0x18) as _;

pub struct MailBox();

impl MailBox {
    pub fn new() -> MailBox {
        MailBox()
    }

    pub unsafe fn write_mail(&mut self, mail: &mut impl Mailable) {
        // unsafe {
        while self.mail_status().is_full() {}

        MAIL0_WRITE.write_volatile(mail.write_mail().into());
        data_barrier();
        // }
    }

    pub unsafe fn read_mail(&mut self) -> Mail {
        // unsafe {
        while self.mail_status().is_empty() {}

        data_barrier();
        MAIL0_READ.read_volatile().into()
        // }
    }

    pub unsafe fn mail_status(&self) -> MailStatus {
        data_barrier();
        MAIL0_STATUS.read_volatile().into()
    }
}
