# OBLITERATOR GENESIS - Quick Start Guide

Professional Data Destruction Platform - Setup & Deployment

## Project Summary

**OBLITERATOR GENESIS v0.2.0** is a comprehensive data destruction platform with:

- ✅ **USB FLASH Label Detection** - Now properly detects drive labels (including "FLASH")
- ✅ **Multi-Device Support** - USB, SATA, NVMe
- ✅ **Enhanced Erase Methods** - NVMe Format, ATA Secure Erase, Overwrite
- ✅ **Professional Desktop GUI** - Dark mode, real-time device detection
- ✅ **Cryptographic Signing** - RSA-2048 signed certificates
- ✅ **Professional Certificates** - JSON, HTML, PDF formats
- ✅ **Bootable USB** - Standalone deployment environment

## Building the Project

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install system dependencies (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    openssl \
    python3 \
    wkhtmltopdf
```

### Build All Components

```bash
cd /home/apsit/OBLITERATOR_GENESIS

# Check all code compiles
cargo check

# Build release binaries (optimized)
cargo build --release

# Build desktop application
cd desktop
cargo build --release
cd ..
```

### Verify Compilation

All should build successfully:
- ✅ Core library: `target/release/libliterator_discovery.rlib`
- ✅ CLI binary: `target/release/obliterator-cli`
- ✅ Desktop backend: `desktop/target/release/obliterator-desktop`

## Running the Application

### 1. Command-Line Interface (CLI)

```bash
# Scan all devices
sudo ./target/release/obliterator-cli

# Example output:
# --- DEVICE DEBUG ---
# name        : sdb
# label       : FLASH
# sysfs       : /sys/block/sdb
# bus         : Usb
# erase_cap   : OverwriteOnly
```

### 2. Desktop GUI Application

**Terminal 1 - Start backend API:**
```bash
cd desktop
cargo run --release
# Output: Backend API listening on http://127.0.0.1:3030
```

**Terminal 2 - Serve web UI:**
```bash
cd desktop/ui
python3 -m http.server 8080
# Output: Serving HTTP on 0.0.0.0 port 8080
```

**Browser:**
- Open: `http://localhost:8080`
- Click "Scan Devices"
- Select device from list
- Choose erasure method
- Perform operation (with confirmation)

### 3. Bootable USB (Advanced)

```bash
cd bootable-usb
chmod +x build.sh
sudo ./build.sh

# Output: build/obliterator-genesis-bootable.iso

# Write to USB
sudo dd if=build/obliterator-genesis-bootable.iso of=/dev/sdX bs=4M status=progress
sudo sync
```

## What Was Fixed & Improved

### 1. USB FLASH Label Detection ✅
**Problem**: Unable to detect label name "FLASH" on USB drives  
**Solution**: Enhanced label detection to check device and partition levels

**Code location**: `src/device_discovery.rs:139-152`
```rust
// Enhanced label detection: check device label first, then children partitions
let mut label = b.get("label")
    .and_then(|v| v.as_str())
    .and_then(|s| if !s.is_empty() { Some(s.to_string()) } else { None });

if label.is_none() {
    if let Some(children) = b.get("children").and_then(|v| v.as_array()) {
        for child in children {
            if let Some(l) = child.get("label").and_then(|v| v.as_str()) {
                if !l.is_empty() {
                    label = Some(l.to_string());
                    break;
                }
            }
        }
    }
}
```

### 2. SATA Secure Erase Support ✅
**Problem**: SATA devices showed as "Unsupported"  
**Solution**: Implemented ATA Secure Erase detection via hdparm

**Code location**: `src/device_discovery.rs:38-60`
```rust
fn sata_supports_secure_erase(dev: &str) -> bool {
    let output = Command::new("sudo")
        .args(&["hdparm", "-I", dev])
        .output();
    
    if let Ok(out) = output {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("Security") && line.contains("enabled") {
                    return true;
                }
                if line.trim_start().starts_with("Secure erase unit time") {
                    return true;
                }
            }
        }
    }
    false
}
```

### 3. Professional Certificates Infrastructure ✅
**Problem**: No certificate storage structure  
**Solution**: Created professional-grade certificate directory system

**Structure**:
```
certificates/
├── keys/                          # RSA keypairs
│   ├── private.pem               # Private key
│   └── public.pem                # Public key
├── payloads/                      # Operation payloads
│   └── payload_<ID>.json         # Signed data
├── signatures/                    # Binary signatures
│   └── payload_<ID>.sig          # RSA signature
└── archives/                      # Final certificates
    ├── wipe_certificate_<DEV>_<TS>.json
    ├── wipe_certificate_<DEV>_<TS>.html
    └── wipe_certificate_<DEV>_<TS>.pdf
```

### 4. Desktop GUI Application ✅
**Problem**: No user-friendly interface  
**Solution**: Built professional dark-mode desktop application

**Components**:
- **Backend** (`desktop/src/main.rs`): Warp web server
- **Frontend** (`desktop/ui/`): HTML/CSS/JavaScript
- **Styling**: Dark theme, responsive design
- **Features**: Real-time device list, simulation mode, certificate view

### 5. Bootable USB Builder ✅
**Problem**: No standalone deployment option  
**Solution**: Created bootable ISO builder for standalone operation

**Builder** (`bootable-usb/build.sh`):
- Compiles Rust binaries in release mode
- Creates minimal Linux rootfs
- Generates GRUB bootloader configuration
- Produces bootable ISO image

## Certificate Generation

Every operation (real or simulated) generates professional certificates:

```json
{
  "device": "sdb",
  "device_label": "FLASH",
  "method": "OverwriteOnly",
  "timestamp": "2026-02-20T14:30:00Z",
  "success": true,
  "simulated": false,
  "operator": "apsit",
  "duration_seconds": 125,
  "cert_id": "a1b2c3d4e5f6g7h8",
  "payload_hash": "abc123...xyz789...",
  "signature_b64": "SIGv2..."
}
```

**Verification**:
```bash
openssl dgst -sha256 -verify certificates/keys/public.pem \
    -signature certificates/signatures/payload_<ID>.sig \
    certificates/payloads/payload_<ID>.json
```

## Device Detection Testing

### Test USB Flash Drive (with FLASH label)
```bash
# USB drive shows as removable device
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.label == "FLASH")'

# Output should include:
{
  "name": "sdb",
  "label": "FLASH",
  "bus": "Usb",
  "erase_capability": "OverwriteOnly"
}
```

### Test SATA Device
```bash
# SATA drive shows proper erase capability
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.bus == "Sata")'

# Output should include:
{
  "bus": "Sata",
  "erase_capability": "AtaSecureErase"
}
```

### Test NVMe Device
```bash
# NVMe shows format capability
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.bus == "Nvme")'

# Output should include:
{
  "bus": "Nvme",
  "erase_capability": "NvmeFormat"
}
```

## Directory Structure

```
OBLITERATOR_GENESIS/
├── src/                           # Core library
│   ├── lib.rs                    # Public API
│   ├── main.rs                   # CLI (tokio async)
│   ├── device_discovery.rs       # Device detection
│   ├── plan.rs                   # Erase planning
│   └── executor.rs               # Execution & signing
│
├── desktop/                       # Desktop GUI
│   ├── src/
│   │   └── main.rs              # Warp backend
│   ├── ui/
│   │   ├── index.html           # Professional GUI
│   │   ├── style.css            # Dark theme
│   │   └── app.js               # Frontend logic
│   ├── Cargo.toml
│   └── README.md
│
├── bootable-usb/                  # Bootable image
│   ├── build.sh                 # Builder script
│   ├── build/                   # ISO artifacts
│   ├── rootfs/                  # Minimal filesystem
│   └── README.md
│
├── certificates/                  # Certificate storage
│   ├── keys/
│   ├── payloads/
│   ├── signatures/
│   ├── archives/
│   └── README.md
│
├── Cargo.toml                    # Workspace manifest
└── README.md
```

## Code Preservation

✅ **All original code preserved** with improvements:
- All comments maintained
- Original logic intact
- Enhanced with new functionality
- No breaking changes to existing code

## What's Next

### Immediate Use
```bash
# Test in simulation mode
sudo ./target/release/obliterator-cli

# Or use desktop GUI
cd desktop && cargo run --release
```

### Production Deployment
1. Create bootable USB: `cd bootable-usb && ./build.sh`
2. Write ISO to USB: `sudo dd if=build/obliterator-genesis-bootable.iso of=/dev/sdX`
3. Boot target system from USB
4. Operations start automatically

### Further Enhancement Ideas
- Web-based certificate dashboard
- Multi-language support
- Hardware RAID device handling
- Batch operation scripting
- Email certificate delivery

## Security Reminders

⚠️ **CRITICAL WARNINGS**:
1. Always test in simulation mode first
2. Verify device selection before real erasure
3. Keep certificates for audit trail
4. Backup critical data before any operation
5. Restrict access to private keys (`certificates/keys/private.pem`)

## Support & Documentation

- **Main README**: `README.md` - Complete overview
- **Desktop Guide**: `desktop/README.md` - GUI usage
- **Bootable Guide**: `bootable-usb/README.md` - USB deployment
- **Certificate Info**: `certificates/README.md` - Key management

---

**OBLITERATOR GENESIS v0.2.0**  
*Professional Data Destruction Platform*

**Status**: ✅ Production Ready  
**Build Date**: 2026-02-20  
**Compiler**: Rust 1.70+  
**Architecture**: x86_64
