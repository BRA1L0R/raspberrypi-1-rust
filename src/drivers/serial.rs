use core::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{bases::PHYS_ADDR, utils::data_barrier};

macro_rules! register_define {
    ($name:tt, $type:tt, $val:expr) => {
        const $name: *mut $type = ($val) as *mut $type;
    };
}

const AUX_BASE: u32 = 0x21_5000 + PHYS_ADDR;

register_define!(AUX_ENB, u32, AUX_BASE + 0x04);
register_define!(MU_IO, u32, AUX_BASE + 0x40);
register_define!(MU_IER, u32, AUX_BASE + 0x44);
// register_define!(MU_IIR, u32, AUX_BASE + 0x48);
register_define!(MU_LCR, u32, AUX_BASE + 0x4C);
register_define!(MU_MCR, u32, AUX_BASE + 0x50);
register_define!(MU_CNTL, u32, AUX_BASE + 0x60);
register_define!(MU_BAUD, u32, AUX_BASE + 0x68);
register_define!(MU_LSR, u32, AUX_BASE + 0x54);

register_define!(GPIO1_FSEL, u32, PHYS_ADDR + 0x20_0004);
register_define!(GPPUD, u32, PHYS_ADDR + 0x20_0094);
register_define!(GPPUDCLK0, u32, PHYS_ADDR + 0x20_0098);
// register_define!()

const BAUD_DERIV: u32 = ((250_000_000 / 115_200) / 8) - 1;

pub struct SerialConsole();

static mut SERIAL_AVAIL: AtomicBool = AtomicBool::new(true);

impl SerialConsole {
    pub fn take() -> Option<SerialConsole> {
        unsafe {
            data_barrier();
            if !SERIAL_AVAIL.load(Ordering::Relaxed) {
                None
            } else {
                SERIAL_AVAIL.store(false, Ordering::Relaxed);
                data_barrier();

                Some(SerialConsole())
            }
        }
    }

    pub fn enable(&self) {
        unsafe {
            // alternate function set
            let mut gpio_fsel = GPIO1_FSEL.read_volatile();
            gpio_fsel &= !(0b111 << 12); // clear af of GPIO14
            gpio_fsel |= 0b010 << 12; // set af 5
            GPIO1_FSEL.write_volatile(gpio_fsel);

            // pull-down unset
            GPPUD.write_volatile(0);
            (0..150).for_each(|_| asm!("nop"));
            GPPUDCLK0.write_volatile((1 << 14) | (1 << 15));
            (0..150).for_each(|_| asm!("nop"));
            GPPUDCLK0.write_volatile(0);

            AUX_ENB.write_volatile(1);

            // clear a bunch of interrupt and ignored settings
            MU_CNTL.write_volatile(0);
            MU_IER.write_volatile(0);
            MU_LCR.write_volatile(1);
            MU_MCR.write_volatile(0);
            MU_BAUD.write_volatile(BAUD_DERIV); // baud rate

            MU_CNTL.write_volatile(2); // enable transmitter
        }
    }

    fn wait_serial(&self) {
        // let is_empty = || (unsafe { MU_LSR.read_volatile() } & 0x20) != 0;
        while (unsafe { MU_LSR.read_volatile() } & 0x20) == 0 {}
    }

    pub fn write_byte(&self, data: u8) {
        self.wait_serial();
        unsafe { MU_IO.write_volatile(data as u32) }
    }
}

impl fmt::Write for SerialConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.bytes().for_each(|c| self.write_byte(c));
        fmt::Result::Ok(())
    }
}

impl Drop for SerialConsole {
    fn drop(&mut self) {
        unsafe {
            data_barrier();
            SERIAL_AVAIL.store(false, Ordering::Relaxed);
        }
    }
}
