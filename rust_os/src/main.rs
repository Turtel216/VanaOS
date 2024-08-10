#![no_std] // disable standard libray
#![no_main] // disable standard entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry Point
#[no_mangle] // don't auto generate function names
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello Kernel!")
        .unwrap();

    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 15.3).unwrap();

    loop {}
}
//############
