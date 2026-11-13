#![no_std]
#![no_main]

#[unsafe(no_mangle)]
pub extern "C" fn hmain() {}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    /* write to the console, then trap */
    loop {}
}
