#!/bin/bash

echo "Testing Phase 2 commands directly..."

echo "Testing # command:"
printf '#' | timeout 5 qemu-system-aarch64 -M raspi4b -cpu cortex-a72 -m 2048M -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none -monitor none 2>/dev/null | tail -10

echo "Testing $ command:"
printf '$' | timeout 5 qemu-system-aarch64 -M raspi4b -cpu cortex-a72 -m 2048M -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none -monitor none 2>/dev/null | tail -10

echo "Testing % command:"
printf '%' | timeout 5 qemu-system-aarch64 -M raspi4b -cpu cortex-a72 -m 2048M -kernel target/aarch64-unknown-none/debug/tiny_os -serial stdio -display none -monitor none 2>/dev/null | tail -10
