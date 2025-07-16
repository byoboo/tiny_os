#!/bin/bash
# Create a FAT32 SD card image for QEMU testing

SD_IMAGE="fat32_test.img"
MOUNT_DIR="sd_mount"

echo "Creating FAT32 SD card image for QEMU testing..."

# Create 64MB image file
dd if=/dev/zero of=$SD_IMAGE bs=1M count=64

# Format as FAT32
mkfs.fat -F 32 $SD_IMAGE

# Mount the image
mkdir -p $MOUNT_DIR
sudo mount -o loop $SD_IMAGE $MOUNT_DIR

# Add test files
echo "Hello from TinyOS!" > $MOUNT_DIR/readme.txt
echo "This is a test file for the FAT32 filesystem implementation." > $MOUNT_DIR/test.txt
mkdir -p $MOUNT_DIR/testdir
echo "File in subdirectory" > $MOUNT_DIR/testdir/subfile.txt

# Add some files with long names to test LFN support
echo "Long filename test" > "$MOUNT_DIR/this_is_a_very_long_filename_to_test_lfn_support.txt"
echo "Unicode test" > "$MOUNT_DIR/test_file_with_spaces.txt"

# Create a simple boot file
echo "TinyOS Boot Test" > $MOUNT_DIR/boot.txt

# Unmount
sudo umount $MOUNT_DIR
rmdir $MOUNT_DIR

echo "✅ FAT32 SD image created: $SD_IMAGE (64MB)"
echo "   Contains: readme.txt, test.txt, testdir/, and test files"
echo "   Ready for QEMU testing with TinyOS"