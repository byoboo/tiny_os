#!/bin/bash
# TinyOS Interactive Test Script
echo "Starting TinyOS with FAT32 filesystem..."
echo "Building..."
cargo build --target aarch64-unknown-none
echo "Running in QEMU..."
echo "Press Ctrl+A, X to exit QEMU"
echo "Available commands once booted:"
echo "  h - Help"
echo "  d - List directory (FAT32)"
echo "  n - Show filesystem info (FAT32)"
echo "  u - Read file (tries readme.txt)"
echo "  k - Go to root directory"
echo ""
qemu-system-aarch64 -M raspi4b -cpu cortex-a72 -kernel target/aarch64-unknown-none/debug/tiny_os -nographic
