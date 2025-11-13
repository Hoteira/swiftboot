#![no_std]
#![no_main]

mod debug;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".start")]
pub extern "C" fn _start() -> ! {

    println!("Hello, world!");

    loop {}
}

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Panic!");

    loop {}
}

