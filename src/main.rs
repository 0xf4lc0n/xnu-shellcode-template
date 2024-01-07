#![no_std]
#![no_main]

mod syscall;
use syscall::*;

#[no_mangle]
fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
