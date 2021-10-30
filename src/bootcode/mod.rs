global_asm!(include_str!("bootcode.s"));

#[no_mangle]
pub fn _kernel_init() -> ! {
    // zero bss
    // unsafe { zero_bss() };

    // jump to main
    crate::main()
}
