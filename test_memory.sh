#!/bin/bash

cd "$(dirname "$0")"

echo "Testing TinyOS Memory Management..."
echo "Building kernel..."
cargo build --target aarch64-unknown-none --release

if [ $? -eq 0 ]; then
    echo "Build successful! Testing memory management..."
    
    # Test sequence: show stats, allocate 3 blocks, show stats, free one, show stats, run memory test
    echo -e 'm\na\na\na\nm\nf\nm\nx\nq' | qemu-system-aarch64 -M raspi4b -nographic -kernel target/aarch64-unknown-none/release/tiny_os
    
    echo "Memory test complete!"
else
    echo "Build failed!"
    exit 1
fi
