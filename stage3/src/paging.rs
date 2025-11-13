#[repr(align(4096))]
pub struct PageTable {
    pub entries: [u64; 512],
}

pub static mut PML4: PageTable = PageTable { entries: [0; 512] };
static mut PDPT: PageTable = PageTable { entries: [0; 512] };
static mut PD: PageTable = PageTable { entries: [0; 512] };

pub(crate) fn setup_paging() {
    unsafe {
        // PML4[0] -> PDPT
        PML4.entries[0] = core::ptr::addr_of!(PDPT) as u64 | 0b11; // Present + Writable

        // PDPT[0] -> PD
        PDPT.entries[0] = core::ptr::addr_of!(PD) as u64 | 0b11;

        // Identity map first 1GB using 2MB pages
        for i in 0..512 {
            PD.entries[i] = (i as u64 * 0x20_0000) | 0b10000011; // PS + Present + Writable
        }
    }
}