#![no_std]
#![no_main]

mod disk;
mod debug;

use core::arch::asm;
use crate::debug::debug;

use core::panic::PanicInfo;

pub const NEXT_STAGE_RAM: u32 = 0x10_0000;
pub const NEXT_STAGE_LBA: u64 = 5120;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".start")]
pub extern "C" fn _start() -> ! {
    let ebx: u16;

    unsafe {
        asm!(
            "mov {0:e}, 0x10",
            "mov ds, {0:e}",
            "mov es, {0:e}",
            "mov ss, {0:e}",

            "mov esp, {1:e}",

            out(reg) _,
            in(reg) 0x30_0000,
            out("ebx") ebx,

            options(nostack),
        );
    }

    disk::read(NEXT_STAGE_LBA, 2048, NEXT_STAGE_RAM as *mut u8);

    debug("[+] Jumping to kernel ...\n");

    unsafe {
        asm!(
            "push {1:e}",
            "call {0:e}",
            in(reg) NEXT_STAGE_RAM,
            in(reg) ebx as u32,
            options(nostack),
        );
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    debug("[x] Bootloader panicked at stage 3!");
    loop {}
}