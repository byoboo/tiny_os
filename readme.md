# install nightly tool chain

rustup install nightly

# set nightly for project

rustup override set nightly

# linker fix windows 'note: LINK : fatal error LNK1561: entry point must be defined'

cargo rustc -- -C link-arg=/ENTRY:\_start

# linker fix windows 'error: linking with `link.exe` failed: exit code: 1221'

cargo rustc -- -C link-args="/ENTRY:\_start /SUBSYSTEM:console"

# Build command

cargo build --target x86_64-tiny_os.json

# bootimage install

cargo install bootimage

# llvm-tools-preview for building the bootloader

rustup component add llvm-tools-preview

# To create a bootable image

cargo bootimage

# Run in Qemu

qemu-system-x86_64 -drive format=raw,file=target/x86_64-tiny_os/debug/bootimage-tiny_os.bin -L "C:\Program Files\qemu"

# Running with normal 'cargo run' should build and run in QEMU
