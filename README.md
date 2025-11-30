<div align="center">
  <img src="icon/icon.svg" alt="Swiftboot Logo" width="120" height="120">

# Swiftboot

**A Multistage x86_64 Bootloader in Pure Rust**

[![Rust](https://img.shields.io/badge/rust-nightly-blue.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-x86__64-lightgrey.svg?style=flat-square)](https://en.wikipedia.org/wiki/X86-64)

<sub>Real Mode • Protected Mode • Long Mode • VBE Graphics • E820 Memory Map</sub>
</div>

<br>

## 📖 Overview

**Swiftboot** is a custom, modular bootloader written in Rust that transitions an x86 CPU from 16-bit Real Mode through 32-bit Protected Mode and finally into 64-bit Long Mode.

Unlike standard bootloaders like GRUB or Limine, Swiftboot is designed to be a transparent, educational reference implementation. It demonstrates how to handle the lowest levels of hardware initialization—GDT, IDT, Paging, VBE Graphics, and Disk I/O—using Rust's safety features in a bare-metal environment.

## 🏗️ Boot Stage Pipeline

The boot process is divided into four distinct stages, utilizing custom target specifications (`bits16`, `bits32`, `bits64`) to ensure correct code generation.

| Stage | Address | Mode | Description |
| :--- | :--- | :--- | :--- |
| **Stage 1** | `0x7C00` | 16-bit Real | **MBR Stub:** Fits in 512 bytes. Sets up the stack and loads Stage 2 from disk using BIOS interrupts. |
| **Stage 2** | `0x7E00` | 16-bit Real | **Hardware Prep:** Queries BIOS for memory map (E820), enables A20 line, sets up GDT, enters Protected Mode. |
| **Stage 3** | `0xFE00` | 32-bit Protected | **Paging Setup:** Configures PAE, builds PML4/PDPT/PD tables for identity mapping, enables Long Mode. |
| **Stage 4** | `0x17E00` | 64-bit Long | **Kernel Loader:** Final trampoline. Loads the kernel from disk into memory (`0x100000`) and jumps to it. |

## ✨ Technical Features

-   **Memory Management:** Manual configuration of 4-level paging structures (PML4) and parsing of E820 memory maps.
-   **Graphics:** Initialization of VESA BIOS Extensions (VBE) for high-resolution linear framebuffers.
-   **Disk I/O:** Implementation of both 16-bit BIOS `int 0x13` calls and 32/64-bit ATA PIO drivers.
-   **Custom Build Tooling:** Includes `cargo-compile`, a bespoke build orchestrator that manages the multi-architecture compilation and disk image assembly.

## 🚀 Quick Start

### Prerequisites
-   Rust Nightly Toolchain
-   `llvm-tools-preview` component
-   QEMU (for testing)

### Build & Run

```bash
# 1. Clone the repository
git clone https://github.com/Hoteira/swiftboot.git
cd swiftboot

# 2. Build the bootloader using the custom compile tool
# This builds all stages and packs them into 'build/disk.img'
cargo compile

# 3. Attach your kernel (optional)
# Swiftboot expects a flat binary kernel at LBA 6144
dd if=path/to/kernel.bin of=build/disk.img bs=512 seek=5120 conv=notrunc

# 4. Emulate
qemu-system-x86_64 -drive file=build/disk.img,format=raw -m 1G -serial stdio
```

## 📜 License

Distributed under the [MIT](LICENSE) license.