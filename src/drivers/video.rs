mod buffer;
mod mail;
mod mailbox;
pub mod painter;
pub mod pixel;
mod structs;

use crate::{drivers::video::structs::FrameBufferMail, utils::data_barrier};
use core::{
    slice::from_raw_parts_mut,
    sync::atomic::{AtomicBool, Ordering},
};

use self::{buffer::FrameBuffer, mailbox::MailBox, painter::Painter, pixel::Pixel};

static mut VIDEO_AVAIL: AtomicBool = AtomicBool::new(true);

pub struct VideoDriver<'a> {
    pub buffer: FrameBuffer<'a>,
    mailbox: MailBox,
}

impl<'a> VideoDriver<'a> {
    pub fn take() -> Option<VideoDriver<'a>> {
        unsafe {
            data_barrier();
            if !VIDEO_AVAIL.load(Ordering::Relaxed) {
                None
            } else {
                VIDEO_AVAIL.store(false, Ordering::Relaxed);
                data_barrier();

                Some(VideoDriver {
                    buffer: FrameBuffer::uninitialized(),
                    mailbox: MailBox::new(),
                })
            }
        }
    }

    pub fn init(&mut self, width: u32, height: u32) {
        const BIT_DEPTH: u32 = 24; // bit depth with RGB each 8bit

        let fb = unsafe { &mut *((1 << 22) as *mut FrameBufferMail) };

        *fb = structs::FrameBufferMail {
            width,
            height,
            virtual_width: width,
            virtual_height: height,
            pitch: 0,
            depth: BIT_DEPTH,
            x_offset: 0,
            y_offset: 0,
            pointer: 0,
            size: 0,
        };

        loop {
            unsafe { self.mailbox.write_mail(fb) };
            let confirmation = unsafe { self.mailbox.read_mail() };

            // Condition to pass: confirmation data is 0
            // and pointer is something other than 0
            if confirmation.data().get() == 0 && fb.pointer != 0 {
                break;
            }
        }

        let buffer =
            unsafe { from_raw_parts_mut(fb.pointer as *mut Pixel, (fb.size / 3) as usize) };

        let fb = FrameBuffer::new(fb.pitch, fb.height, fb.width, buffer);
        self.buffer = fb;
    }

    pub fn painter(&'a mut self) -> Painter<'a> {
        Painter::new(&mut self.buffer)
    }
    // pub fn buffer_painter(&'a mut self) -> Painter<'a> {
    //     self.buffer.painter()
    // }
}
