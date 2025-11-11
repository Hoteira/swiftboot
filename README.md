<div align="center">
  <br>
  <img src="icon/icon.svg" alt="Swiftboot Logo" width="120" height="120">

# Swiftboot

**Three-stage x86 bootloader written in Rust**

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![no_std](https://img.shields.io/badge/no__std-compatible-success.svg)](https://docs.rust-embedded.org/book/)

<sub>🚀 Three-Stage Boot • 🦀 Pure Rust • 💻 Real Mode → Protected Mode</sub>
</div>

<br>

## Quick Start
```bash
# Clone and build
git clone https://github.com/Hoteira/swiftboot.git
cd swiftboot

# Install nightly and components
rustup toolchain install nightly
rustup component add rust-src llvm-tools-preview
cargo install cargo-binutils

# Build bootloader
cargo compile

# Attach your kernel (replace with your kernel path)
dd if=path/to/your/kernel.bin of=build/disk.img bs=512 seek=5120 conv=notrunc

# Run
qemu-system-x86_64 -drive file=build/disk.img,format=raw -m 1G -serial stdio
```

## Features

- 🚀 **Three-Stage Boot** — Modular 512B → 16KB → 16KB → Kernel progression
- 🔧 **Hardware Setup** — Configures GDT, TSS, memory map (E820), RSDP, and VBE graphics
- 💾 **Disk I/O** — BIOS interrupts (16-bit) and ATA PIO (32-bit)
- 🦀 **Pure Rust** — Minimal assembly, custom target specs for 16/32-bit

## Architecture
```
Stage 1 (0x7c00)  →  Stage 2 (0x7e00)  →  Stage 3 (0xfe00)  →  Kernel (0x10_0000)
  512 bytes            16KB real mode       16KB protected       Your kernel here
  BIOS loads           Sets up hardware     Loads kernel         Receives boot info
```

**Disk Layout:**
- LBA 0: Stage 1 (MBR)
- LBA 2048: Stage 2
- LBA 3072: Stage 3
- LBA 5120: Kernel (1MB reserved)

**Boot Info Passed to Kernel:**
```rust
struct BootInfo {
    mmap: MemoryMap,        // E820 memory map
    rsdp: Rsdp,             // ACPI table
    tss: u16,               // TSS selector
    vbe: VbeInfoBlock,      // VBE info
    mode: VbeModeInfoBlock, // Graphics mode
}

#[unsafe(no_mangle)]
extern "C" fn _start(bootinfo_ptr: *const BootInfo) { ... }
```

## Custom Build System

The `cargo-compile` tool orchestrates the build:
```bash
cargo compile  # Builds all stages, converts to raw binaries, assembles disk.img
```

Uses custom target specs:
- `bits16.json` — 16-bit real mode (Stage 1, 2)
- `bits32.json` — 32-bit protected mode (Stage 3)

## Future Features

- [ ] 64-bit mode
- [ ] Multiboot2 compliant

## License

Licensed under the [MIT License](LICENSE).

---

<div align="center">
  <sub>Boots fast 🚀 or breaks trying 😰</sub>
</div>
