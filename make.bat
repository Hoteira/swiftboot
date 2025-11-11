wsl rm -rf build/*

cargo run --package=cargo-compile
cargo build --package=kernel --target=bits32.json

wsl sh -c "objcopy -I elf32-i386 -O binary target/bits32/debug/kernel build/kernel.bin"

wsl dd if=build/kernel.bin of=build/disk.img bs=512 seek=5120 conv=notrunc

qemu-system-x86_64 -drive file=".\build\disk.img",format=raw -m 1G -serial stdio -no-reboot

pause