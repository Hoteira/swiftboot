#![no_std]
#![no_main]

mod disk;
mod debug;
mod boot;
mod paging;
mod gdt;
mod tss;

use core::arch::asm;
use crate::debug::debug;

use core::panic::PanicInfo;
use core::ptr::addr_of;
use crate::boot::BootInfo;
use crate::gdt::GDT;

pub const NEXT_STAGE_RAM: u64 = 0x1_7e00;
pub const NEXT_STAGE_LBA: u64 = 5120;
pub const KERNEL_RAM: u32 = 0x10_0000;


const STACK_ADDRESS: u64 = 0x30_0000;
const BOOT_MODE: u8 = 64; //32 or 64 bits

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
            in(reg) STACK_ADDRESS as u32,
            out("ebx") ebx,

            options(nostack),
        );
    }

    let mut bootinfo = ebx as *mut BootInfo;
    unsafe {
        (*bootinfo).kernel_stack = STACK_ADDRESS;
    }

    debug("[+] Jumping to kernel ...\n");

    if BOOT_MODE == 32 {

        disk::read(NEXT_STAGE_LBA, 2048, KERNEL_RAM as *mut u8);

        unsafe {
            asm!(
                "push {1:e}",
                "call {0:e}",
                in(reg) KERNEL_RAM,
                in(reg) ebx as u32,
                options(nostack),
            );
        }
    } else if BOOT_MODE == 64 {
        debug("Mode unsupported");

        disk::read(NEXT_STAGE_LBA, 1024, NEXT_STAGE_RAM as *mut u8);

        //paging::setup_paging();

        unsafe {

            (*bootinfo).pml4 = addr_of!(paging::PML4) as u64;

            /*asm!(
                "mov cr3, {0:e}",
                in(reg) pml4_address as u32,
            );

            // Enable PAE (CR4.PAE = 1)
            asm!(
                "mov eax, cr4",
                "or eax, 1 << 5",
                "mov cr4, eax",
            );

            // Set LME bit in EFER MSR
            asm!(
                "mov ecx, 0xC0000080",  // EFER MSR
                "rdmsr",
                "or eax, 1 << 8",       // LME bit
                "wrmsr",
            );

            // Enable paging (CR0.PG = 1), which activates long mode
            asm!(
                "mov eax, cr0",
                "or eax, 1 << 31",
                "mov cr0, eax",
            );

            (*(&raw mut GDT)).write_tss();
            (*(&raw mut GDT)).load();

            asm!("mov ebx, {0:e}", in(reg) ebx);
            asm!("ljmp $0x8, ${}", const NEXT_STAGE_RAM, options(att_syntax));*/
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    debug("[x] Bootloader panicked at stage 3!");
    loop {}
}