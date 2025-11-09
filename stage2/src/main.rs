#![no_std]
#![no_main]

mod disk;
mod gdt;
mod tss;
mod mmap;
mod vbe;
mod debug;

static mut BOOT: BootInfo = unsafe { core::mem::zeroed() };
static mut VBE_MODE: VbeModeInfoBlock = unsafe { core::mem::zeroed() };

use core::ptr::addr_of;
use gdt::GDT;

use core::arch::asm;
use core::panic::PanicInfo;
use crate::debug::debug;
use crate::mmap::{get_mmap, MemoryMap};
use crate::vbe::{find_vbe_mode, get_vbe_info, VbeInfoBlock, VbeModeInfoBlock};

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct Rsdp {
    signature: [u8; 8],
    checksum: u8,
    oem_id: [u8; 6],
    revision: u8,
    rsdt_address: u32,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct BootInfo {
    mmap: MemoryMap,
    rsdp: Rsdp,
    tss: u16,
    vbe: VbeInfoBlock,
    mode: VbeModeInfoBlock,
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".start")]
pub extern "C" fn _start() -> ! {
    debug("[+] Loading stage 3 ...\n");

    disk::read_stub();

    debug("[+] Jumping to protected mode ...\n");

    protected_mode();

    loop {}
}

fn protected_mode() {
    unsafe {
        let tss_addr = (*(&raw mut GDT)).write_tss();
        (*(&raw mut GDT)).load();

        BOOT.rsdp = get_rsdp();
        BOOT.vbe = get_vbe_info();
        BOOT.tss = tss_addr;
        get_mmap();

        let best_mode = find_vbe_mode();

        asm!(
            "int 0x10",

            in("ax") 0x4F02,
            in("bx") best_mode
        );

        asm!("mov eax, cr0", "or eax, 1 << 0", "mov cr0, eax",);

        asm!("mov bx, {0:x}", in(reg) addr_of!(BOOT) as u16);

        asm!("ljmp $0x8, $0xfe00", options(att_syntax));
    }
}

#[inline(never)]
fn get_rsdp() -> Rsdp {
    let mut addr = 0xE0000 as *const u8;
    let end = 0xFFFFF as *const u8;

    unsafe {
        while addr <= end {
            let sig = core::slice::from_raw_parts(addr, 8);
            if sig == b"RSD PTR " {
                let rsdp = (addr as *const Rsdp).read();
                return rsdp;
            }
            addr = addr.add(16);
        }
    }

    Rsdp {
        signature: [0; 8],
        checksum: 0,
        oem_id: [0; 6],
        revision: 0,
        rsdt_address: 0,
    }
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    debug("[x] Bootloader stage 2 panicked! x_x");
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn fail() -> ! {
    panic!("[x] Disk failed!");
}