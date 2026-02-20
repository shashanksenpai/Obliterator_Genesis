# OBLITERATOR GENESIS - Bootable USB Setup Guide

Professional-grade bootable USB image for secure data destruction without operating system dependencies.

## Overview

The OBLITERATOR GENESIS bootable USB provides a complete, self-contained environment for secure data erasure:

- **Independent from OS**: Runs directly from USB boot
- **Professional Desktop GUI**: Dark-mode interface with device management
- **Secure Signing**: All operations cryptographically verified
- **Immediate Deployment**: No installation required

## Quick Start

### 1. Build Bootable Image

```bash
cd bootable-usb
chmod +x build.sh
sudo ./build.sh
```

This creates: `build/obliterator-genesis-bootable.iso`

### 2. Write to USB

```bash
# Identify USB device
lsblk | grep disk

# Write image (replace sdX with your device)
sudo dd if=build/obliterator-genesis-bootable.iso of=/dev/sdX bs=4M status=progress
sudo sync
```

### 3. Boot from USB

- Insert USB into target system
- Restart and enter BIOS/UEFI
- Select USB as boot device
- Application launches automatically

## System Requirements

### To Build Bootable Image
- Linux (Ubuntu 20.04+ recommended)
- 4GB+ free disk space
- Dependencies: `grub-mkrescue`, `xorriso`, Rust toolchain

Install dependencies:
```bash
sudo apt-get install grub-pc grub-efi xorriso
```

### To Run from USB
- x86-64 compatible CPU
- BIOS or UEFI firmware
- 256MB+ RAM
- USB 2.0+ drive
- Target devices (USB/SATA/NVMe)

## Boot Modes

### Standard Mode (Default)
```
GRUB> OBLITERATOR GENESIS v0.2.0
```
Full operation with all features enabled.

### Simulation Mode
```
GRUB> OBLITERATOR GENESIS (Simulation Mode)
```
Non-destructive testing of device detection and erasure flows.

### Shell Fallback
If GUI fails to start, shell is available for manual operations:
```bash
# Scan devices
/bin/obliterator-cli

# Display device details
/bin/obliterator-cli | grep -A 5 erase_capability
```

## Directory Structure

```
ISO Root (/)
├── boot/
│   ├── kernel (linux image)
│   └── grub/
│       └── grub.cfg
├── bin/
│   ├── obliterator-cli
│   ├── obliterator-desktop
│   ├── lsblk
│   ├── hdparm
│   ├── nvme
│   └── [system tools]
├── certificates/
│   ├── keys/
│   ├── payloads/
│   ├── signatures/
│   └── archives/
└── [system directories]
```

## Deployment Workflow

### 1. Prepare Target System
- Ensure system is fully shut down
- Backup all data (ISO-side erasure is irreversible)
- Insert bootable USB
- Restart system

### 2. Boot into OBLITERATOR
- Access BIOS/UEFI (usually F2, F10, Del, or Esc)
- Select USB as primary boot device
- Save and exit
- System boots from USB

### 3. Device Detection
- Application scans available devices
- Storage devices appear in interface
- Verify device labels before proceeding

### 4. Perform Erasure
- Select target device
- Verify capacity and details
- Choose erase method (auto-detected)
- Enable Simulation Mode first to verify
- Perform real erasure when confident
- Certificate generated automatically

### 5. Collect Certificates
- Certificates stored in `/certificates/archives/`
- Save to external drive for records
- Includes JSON, HTML, and PDF formats

## Certificate Management in Bootable Environment

Certificates are automatically generated with:
- Device name and label
- Erasure method used
- Timestamp and operator
- RSA-2048 digital signature
- SHA-256 payload hash

### Accessing Certificates

From running system:
```bash
mount -t iso9660 -o ro,loop obliterator-genesis-bootable.iso /mnt
ls -la /mnt/certificates/archives/
```

### Verifying Signatures

```bash
# Extract public key
cp /certificates/keys/public.pem ./

# Verify payload
openssl dgst -sha256 -verify public.pem \
    -signature payload_XXX.sig \
    payload_XXX.json
```

## Building from Source

### Full Custom Build

```bash
# Clone and prepare
git clone <repo>
cd OBLITERATOR_GENESIS

# Build release binaries
cargo build --release

# Build bootable image
cd bootable-usb
chmod +x build.sh
./build.sh
```

### Customize Kernel

Replace Linux kernel in `build/iso/boot/obliterator-kernel`:

```bash
# Extract Linux kernel
wget https://mirrors.edge.kernel.org/pub/linux/kernel/v5.x/linux-5.15.tar.xz
tar xf linux-5.15.tar.xz
cd linux-5.15

# Configure (USB, SATA, NVMe drivers)
make menuconfig

# Build
make -j$(nproc) bzImage
cp arch/x86/boot/bzImage ../bootable-usb/build/iso/boot/obliterator-kernel
```

## Safety Procedures

### Before Any Erasure

1. **Verify Device Detection**
   - Ensure correct device is identified
   - Check label matches expected device
   - Confirm capacity is accurate

2. **Test with Simulation Mode**
   - Run erasure in simulation first
   - Verify no unexpected devices targeted
   - Check certificate generation

3. **Create Backup**
   - Backup critical data before real erasure
   - Document device serial numbers
   - Keep certificates for audit trail

### During Erasure

- Do not disconnect USB drive
- Do not power off system
- Monitor completion percentage
- Wait for certificate generation

### After Erasure

- Wait for "Operation Complete" message
- Retrieve and archive certificates
- Verify target device is blank
- Document completion timestamp

## Troubleshooting

### System Won't Boot from USB

**Symptoms**: Boots to Windows/existing OS instead of USB

**Solutions**:
1. Rewrite USB image: `sudo dd if=iso of=/dev/sdX bs=4M status=progress`
2. Check BIOS boot order - move USB to first position
3. Disable Secure Boot temporarily in UEFI
4. Try different USB port (preferably 2.0)
5. Try different USB drive

### No Devices Detected

**Symptoms**: "No devices detected" message in GUI

**Solutions**:
1. Ensure devices are fully connected
2. Try different USB cables/ports
3. Restart system and rescan
4. Check BIOS for any boot options
5. Verify devices aren't already mounted

### GUI Won't Start

**Symptoms**: Boots to shell instead of GUI

**Solutions**:
1. Start backend manually: `/bin/obliterator-desktop`
2. Open browser to `http://127.0.0.1:8080`
3. Check backend logs for errors
4. Use CLI instead: `/bin/obliterator-cli`

### Certificate Generation Failed

**Symptoms**: Operation complete but no certificate

**Solutions**:
1. Verify disk space: `df -h /certificates/`
2. Check permissions: `ls -la /certificates/`
3. Ensure OpenSSL available: `which openssl`
4. Check `/proc/sys/fs/file-max` has room

## Performance Metrics

### Device Detection
- Initial scan: ~1-2 seconds
- Device enumeration: ~2-5 seconds per device

### Erasure Operations
- USB overwrite (1GB): ~30-60 seconds
- SATA secure erase (500GB): ~10-30 minutes
- NVMe format (512GB): ~1-2 minutes

### Certificate Operations
- JSON generation: <1 second
- Signing: ~1-3 seconds
- HTML/PDF rendering: ~2-5 seconds

## Advanced Usage

### Manual Device Erasure via Shell

```bash
# Scan available devices
lsblk

# Overwrite with zeros
dd if=/dev/zero of=/dev/sdX bs=4M status=progress

# NVMe secure erase
nvme format /dev/nvme0n1

# SATA secure erase
hdparm --security-erase NULL /dev/sda
```

### Batch Operations

Create script:
```bash
#!/bin/bash
devices="/dev/sdb /dev/sdc /dev/sdd"
for dev in $devices; do
    echo "Erasing $dev..."
    /bin/obliterator-cli --device $dev --simulate false
done
```

### Network Operations (Advanced)

To enable network in bootable environment:
1. Modify `rootfs/etc/network/interfaces`
2. Include network tools: `ip`, `dhclient`
3. Rebuild ISO

## Security Considerations

### During Development
- Keep private keys secure
- Don't include PII in certificates
- Test on non-critical devices
- Verify signature verification works

### In Production
- Restrict USB access (physical security)
- Archive certificates securely
- Rotate RSA keys periodically
- Monitor for unauthorized use
- Document all operations

### Compliance
- Certificates serve as proof of destruction
- SHA-256 hashing provides integrity
- RSA-2048 signing provides authenticity
- Timestamps for audit trail

## Support & Documentation

For issues or questions:
1. Check `/certificates/README.md` on ISO
2. Review operation certificates for error details
3. Check application logs in `/tmp/`
4. Consult main repository documentation

## Version History

**v0.2.0** (Current)
- Desktop GUI with dark theme
- Multi-device support (USB, SATA, NVMe)
- Professional certificate generation
- RSA-2048 signing
- Bootable USB image

**v0.1.0**
- Initial CLI implementation
- Basic device detection
- Overwrite-only erasure

---

**OBLITERATOR GENESIS** - Professional Data Destruction  
*Version 0.2.0 - Bootable Edition*
