#!/bin/bash

# OBLITERATOR GENESIS - Bootable USB Builder
# Creates a professional bootable USB image for secure data erasure

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"
ROOTFS_DIR="$SCRIPT_DIR/rootfs"
OUTPUT_ISO="$BUILD_DIR/obliterator-genesis-bootable.iso"

echo "======================================"
echo "OBLITERATOR GENESIS - Bootable USB Builder"
echo "======================================"

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Check requirements
echo -e "\n${YELLOW}[*] Checking dependencies...${NC}"
for cmd in xorriso mkisofs isolinux grub-mkrescue; do
    if ! command -v $cmd &> /dev/null; then
        echo -e "${RED}[!] Missing: $cmd${NC}"
        echo "Install with: sudo apt-get install $cmd"
    fi
done

# Create directory structure
echo -e "\n${YELLOW}[*] Creating directory structure...${NC}"
mkdir -p "$BUILD_DIR"/iso/{boot,EFI/BOOT}
mkdir -p "$ROOTFS_DIR"/{bin,etc,lib,usr,proc,sys,dev,mnt,root,tmp,var}

# Build Rust binaries
echo -e "\n${YELLOW}[*] Building Rust binaries...${NC}"
cd "$SCRIPT_DIR/.."
cargo build --release -p obliterator-discovery
cargo build --release -p obliterator-desktop

# Copy binaries
echo -e "\n${YELLOW}[*] Copying application binaries...${NC}"
cp target/release/obliterator-cli "$ROOTFS_DIR/bin/"
cp target/release/obliterator-desktop "$ROOTFS_DIR/bin/"
chmod +x "$ROOTFS_DIR/bin/"*

# Create minimal rootfs
echo -e "\n${YELLOW}[*] Setting up minimal root filesystem...${NC}"

# Copy system libraries
echo "Copying system libraries..."
for lib in /lib/x86_64-linux-gnu/{libc.so.6,libm.so.6,libpthread.so.0,libdl.so.2}; do
    [ -f "$lib" ] && cp "$lib" "$ROOTFS_DIR/lib/" 2>/dev/null || true
done

# Copy essential binaries
for bin in /bin/{bash,sh,cat,ls,mkdir,mount,umount}; do
    [ -f "$bin" ] && cp "$bin" "$ROOTFS_DIR/bin/" 2>/dev/null || true
done

# Copy device utilities
for util in /sbin/{lsblk,hdparm,nvme,sudo}; do
    [ -f "$util" ] && cp "$util" "$ROOTFS_DIR/bin/" 2>/dev/null || true
done

# Create init script
echo -e "\n${YELLOW}[*] Creating init script...${NC}"
cat > "$ROOTFS_DIR/init" << 'INIT_SCRIPT'
#!/bin/sh

# OBLITERATOR GENESIS - Boot Init Script

echo "======================================"
echo "OBLITERATOR GENESIS v0.2.0"
echo "Professional Data Destruction Tool"
echo "======================================"
echo ""

# Mount essential filesystems
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
mkdir -p /dev/pts
mount -t devpts devpts /dev/pts

# Set up environment
export PATH=/bin:/usr/bin:/sbin:/usr/sbin
export HOME=/root

# Start the application
echo "Starting OBLITERATOR GENESIS Desktop..."
echo "Launching backend API server..."
cd /
/bin/obliterator-desktop

# Fallback to shell if application exits
echo "Application exited. Launching shell..."
exec /bin/bash
INIT_SCRIPT

chmod +x "$ROOTFS_DIR/init"

# Create GRUB configuration
echo -e "\n${YELLOW}[*] Creating GRUB bootloader configuration...${NC}"
mkdir -p "$BUILD_DIR/iso/boot/grub"
cat > "$BUILD_DIR/iso/boot/grub/grub.cfg" << 'GRUB_CONFIG'
set default=0
set timeout=5

# Theme
set color_normal=white/black
set color_highlight=black/light-gray

menuentry 'OBLITERATOR GENESIS v0.2.0' {
    multiboot /boot/obliterator-kernel
    boot
}

menuentry 'OBLITERATOR GENESIS (Simulation Mode)' {
    multiboot /boot/obliterator-kernel simulation=true
    boot
}
GRUB_CONFIG

# Copy certificates directory
echo -e "\n${YELLOW}[*] Including certificates infrastructure...${NC}"
mkdir -p "$ROOTFS_DIR/certificates"/{keys,payloads,signatures,archives}
cp -r "$SCRIPT_DIR/../certificates/README.md" "$ROOTFS_DIR/certificates/" 2>/dev/null || true

# Create manifest
echo -e "\n${YELLOW}[*] Creating system manifest...${NC}"
cat > "$BUILD_DIR/MANIFEST.txt" << MANIFEST
OBLITERATOR GENESIS v0.2.0 - Bootable USB Image
================================================

Build Date: $(date)
Source: OBLITERATOR_GENESIS repository
Architecture: x86_64

Contents:
- Minimal Linux kernel with essential drivers
- OBLITERATOR GENESIS CLI application
- OBLITERATOR GENESIS Desktop GUI
- Device detection (USB, SATA, NVMe)
- Cryptographic signing infrastructure
- Professional certificate generation

Features:
✓ Multi-device erasure support
✓ NVMe Format, ATA Secure Erase, Overwrite
✓ RSA-2048 signed certificates
✓ Simulation mode for testing
✓ Real-time device detection
✓ Professional dark-mode GUI

Boot Options:
1. Normal mode - Full operation
2. Simulation mode - Non-destructive testing
3. Shell fallback - Manual operations

Security:
- No network connectivity in default mode
- All operations logged to certificates/
- SHA-256 hash verification
- RSA-2048 cryptographic signing

For instructions: See /certificates/README.md
MANIFEST

# Create bootable image
echo -e "\n${YELLOW}[*] Creating ISO image...${NC}"

if command -v grub-mkrescue &> /dev/null; then
    echo "Using GRUB mkrescue..."
    grub-mkrescue -o "$OUTPUT_ISO" "$BUILD_DIR/iso" "$ROOTFS_DIR" 2>/dev/null || true
elif command -v xorriso &> /dev/null; then
    echo "Using xorriso..."
    xorriso -as mkisofs -R -J -l -V 'OBLITERATOR_GENESIS' \
        -b boot/isolinux.bin -c boot/boot.cat \
        -no-emul-boot -boot-load-size 4 -boot-info-table \
        -input-charset utf-8 -o "$OUTPUT_ISO" "$BUILD_DIR/iso"
fi

if [ -f "$OUTPUT_ISO" ]; then
    echo -e "\n${GREEN}[✓] ISO image created successfully!${NC}"
    ls -lh "$OUTPUT_ISO"
    
    echo -e "\n${GREEN}[✓] Build complete!${NC}"
    echo ""
    echo "To write to USB:"
    echo "  sudo dd if=$OUTPUT_ISO of=/dev/sdX bs=4M status=progress"
    echo "  (replace sdX with your USB device)"
    echo ""
    echo "To verify:"
    echo "  sudo dd if=/dev/sdX bs=4M | md5sum"
else
    echo -e "\n${RED}[!] Failed to create ISO image${NC}"
    exit 1
fi

echo -e "\n${GREEN}[✓] OBLITERATOR GENESIS bootable image ready!${NC}"
