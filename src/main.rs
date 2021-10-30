#![feature(global_asm)]
#![feature(asm)]
#![no_std]
#![no_main]

mod bases;
mod console;
mod drivers;
mod utils;

// use console::println;
use console::init_serial;
use core::panic::PanicInfo;

use crate::drivers::video::pixel::Pixel;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        unsafe { asm!("wfe") }
    }
}

/// setup stack pointer and jump to _kernel_init
mod bootcode;

pub fn main() -> ! {
    init_serial();

    println!("~~~");
    println!("Rust Kernel");

    // unsafe { println!("{:X} {:X}", bootcode::__bss_start, bootcode::__bss_end) };

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;

    let mut video = drivers::video::VideoDriver::take().unwrap();
    video.init(WIDTH, HEIGHT);

    let mut painter = video.painter();
    painter.clear_screen();

    painter.line(
        ((WIDTH / 2) - 200) as i32,
        ((HEIGHT / 2) - 200) as i32,
        ((WIDTH / 2) + 200) as i32,
        ((HEIGHT / 2) + 200) as i32,
        Pixel::hex(0xFF0000),
    );

    painter.line(
        ((WIDTH / 2) + 200) as i32,
        ((HEIGHT / 2) - 200) as i32,
        ((WIDTH / 2) - 200) as i32,
        ((HEIGHT / 2) + 200) as i32,
        Pixel::hex(0xFF0000),
    );

    panic!("End of kernel.");
}
