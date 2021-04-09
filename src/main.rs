#![no_std] // disable standard library
#![no_main] // for using different entry point than main function.

use core::panic::PanicInfo;

// Invoked when panic occurs
// PanicInfo contains file, line and where it happened and optional panic message
// diverging functions (returns !(never(not allowed to ever return)) type)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// overwriting entry point
// `no mangle` to ensure rust compiler outputs function with _start() name and not _Zasdasd_start_asdadbvsd
// we need function name because linker need the name of starting point
// entry point is not called by any function, but invoked directly by the operating system or bootloader.
// Instead of returning, the entry point should e.g. invoke the exit system call of the operating system.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point
    //linker looks for a function named `_start` by default
    let vga_buffer = 0x0b8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
