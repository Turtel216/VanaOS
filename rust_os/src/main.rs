#![no_std] // disable standard libray
#![no_main] // disable standard entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprintln!("{}", _info);
    loop {}
}

// Entry Point
#[no_mangle] // don't auto generate function names
pub extern "C" fn _start() -> ! {
    kprintln!("Hello Kernel");

    loop {}
}
//############
