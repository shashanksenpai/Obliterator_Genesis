# OBLITERATOR GENESIS

**Professional Data Destruction & Secure Erasure Platform**

A comprehensive, production-grade solution for secure data erasure with support for USB, SATA, and NVMe devices. Features cryptographically signed certificates, professional GUI, and bootable USB deployment.

## Overview

OBLITERATOR GENESIS is a multi-layered data destruction platform designed for security professionals, IT operations, and enterprises requiring verifiable, secure data destruction:

- **Desktop Application**: Modern GUI for device detection and erasure
- **Command-Line Interface**: Automated and scripted operations
- **Bootable USB Image**: Standalone environment without OS dependencies
- **Professional Certificates**: RSA-2048 signed, audit-ready documentation
- **Multi-Device Support**: USB Flash, SATA HDD/SSD, NVMe

## Key Features

### Core Capabilities
✅ **Multi-Device Support**
- USB Flash drives (with FLASH label detection)
- SATA hard drives and SSDs
- NVMe M.2 storage devices

✅ **Multiple Erasure Methods**
- NVMe Format (fastest, most secure)
- ATA Secure Erase (hardware-level, SATA devices)
- Overwrite with zeros (universal fallback)

✅ **Professional Desktop GUI**
- Dark-mode professional interface
- Real-time device detection
- Simulation mode for testing
- One-click erasure with confirmation
- Live operation logging

✅ **Security & Verification**
- RSA-2048 cryptographic signing
- SHA-256 payload hashing
- Automatic keypair generation
- Comprehensive audit trail
- Tamper-evident certificates

✅ **Certificate Management**
- JSON format (machine-readable)
- HTML format (human-readable)
- PDF format (professional report)
- Signed payload verification
- Complete operation metadata

✅ **Bootable Deployment**
- Standalone USB environment
- No OS installation required
- Minimal rootfs (highly secure)
- GRUB bootloader with modes
- Immediate device access

## Project Structure

```
OBLITERATOR_GENESIS/
├── src/                    # Core Rust library
│   ├── lib.rs             # Public API
│   ├── main.rs            # CLI interface
│   ├── device_discovery.rs # Device detection & scanning
│   ├── plan.rs            # Erasure planning
│   └── executor.rs        # Erasure execution & signing
├── desktop/               # Desktop GUI application
│   ├── src/
│   │   └── main.rs        # Warp web server backend
│   ├── ui/
│   │   ├── index.html     # Professional interface
│   │   ├── style.css      # Dark-mode styling
│   │   └── app.js         # Frontend logic
│   ├── Cargo.toml
│   └── README.md
├── bootable-usb/          # Bootable image builder
│   ├── build.sh           # ISO builder script
│   ├── build/             # Build artifacts
│   ├── rootfs/            # Minimal filesystem
│   └── README.md
├── certificates/          # Certificate storage infrastructure
│   ├── keys/              # RSA keypairs
│   ├── payloads/          # Operation payloads
│   ├── signatures/        # Binary signatures
│   ├── archives/          # Final certificates
│   └── README.md
├── Cargo.toml
└── README.md
```

## Quick Start

### 1. Desktop Application

```bash
# Build backend
cd desktop
cargo build --release

# Run backend server
cargo run

# In another terminal, serve UI
cd ui
python3 -m http.server 8080

# Open browser: http://localhost:8080
```

### 2. Command-Line Interface

```bash
# Build CLI
cargo build --release

# Run
sudo ./target/release/obliterator-cli
```

### 3. Bootable USB

```bash
# Build bootable image
cd bootable-usb
chmod +x build.sh
sudo ./build.sh

# Write to USB (replace sdX)
sudo dd if=build/obliterator-genesis-bootable.iso of=/dev/sdX bs=4M

# Boot from USB on target system
```

## Detailed Documentation

### For Desktop GUI Users
See: `desktop/README.md`
- Installation & setup
- API endpoints
- Device detection
- Certificate management
- Troubleshooting

### For Command-Line Operations
See: `src/` directory
- CLI usage
- Automation examples
- Library integration
- Advanced options

### For Bootable USB Deployment
See: `bootable-usb/README.md`
- Building bootable image
- USB deployment
- Boot modes
- Safety procedures
- Troubleshooting

### For Certificate Infrastructure
See: `certificates/README.md`
- Storage structure
- RSA key management
- Signing verification
- Audit trail
- Compliance

## Architecture

### Backend (Rust)
- **Device Discovery Module** (`device_discovery.rs`)
  - Parses lsblk JSON output
  - Detects bus type (USB, SATA, NVMe)
  - Identifies erase capabilities
  - Handles label detection (including FLASH drives)

- **Planning Module** (`plan.rs`)
  - Builds optimal erasure plan based on device capabilities
  - Selects appropriate method (NVMe Format > ATA Erase > Overwrite)
  - Determines destructiveness level

- **Executor Module** (`executor.rs`)
  - Executes erasure operations
  - Generates cryptographic signatures
  - Creates professional certificates
  - Manages directory structure

- **Web Server** (Warp + Tokio)
  - REST API for device discovery
  - Health checks
  - Async operation handling
  - CORS-enabled endpoints

### Frontend (HTML/CSS/JavaScript)
- Modern responsive design
- Dark-mode professional aesthetic
- Real-time device updates
- Modal-based workflows
- Simulation vs. real mode toggle
- Certificate download integration

### Bootable Environment
- Minimal Linux kernel
- Essential system tools
- Application binaries
- Certificate infrastructure
- GRUB bootloader with multiple modes

## Device Detection

### Label Detection (Especially USB FLASH Drives)

The system now properly detects USB drive labels:

```json
{
  "name": "sdb",
  "label": "FLASH",
  "size_bytes": 15728640000,
  "bus": "Usb",
  "erase_capability": "OverwriteOnly"
}
```

### Bus Type Detection
- **NVMe**: Identified via `/sys/block/*/device` symlink
- **SATA**: Non-removable block devices
- **USB**: Removable block devices

### Erase Capability Detection
- **NVMe Format**: Verified with `nvme id-ctrl` command
- **ATA Secure Erase**: Checked via `hdparm -I`
- **Overwrite**: Available on all devices

## Certificate System

### Automatic Generation
Every erasure operation (real or simulated) generates:

1. **JSON Certificate** - Machine readable
   ```json
   {
     "device": "sdb",
     "device_label": "FLASH",
     "method": "OverwriteOnly",
     "timestamp": "2026-02-20T14:30:00Z",
     "simulated": false,
     "success": true,
     "payload_hash": "abc123...",
     "signature_b64": "xyz789..."
   }
   ```

2. **HTML Certificate** - Professional report
   - Formatted table with all details
   - Success/failure status highlighted
   - Suitable for printing

3. **PDF Certificate** (when available)
   - Professional document format
   - High-quality printing
   - Secure storage compatibility

### Cryptographic Signing
- RSA-2048 key pair (auto-generated)
- SHA-256 payload hashing
- Base64-encoded signatures
- Verification via public key

### Storage Structure
```
certificates/
├── keys/
│   ├── private.pem    # RSA-2048 private key
│   └── public.pem     # Public key for verification
├── payloads/
│   └── payload_<ID>.json  # Canonical operation data
├── signatures/
│   └── payload_<ID>.sig   # Binary RSA signature
└── archives/
    ├── wipe_certificate_sdb_2026-02-20T...json
    ├── wipe_certificate_sdb_2026-02-20T...html
    └── wipe_certificate_sdb_2026-02-20T...pdf
```

## Security Considerations

### Device Detection Strictness
- Validates device paths (`/dev/*`)
- Confirms sysfs entries exist
- Verifies removable flag for USB
- Checks bus controller for NVMe
- Rejects loop devices automatically

### Erasure Safety
- Requires explicit confirmation
- Simulation mode for testing
- Shows device label and capacity
- Prevents accidental overwrites
- Logs all operations

### Cryptographic Security
- RSA-2048 (2048-bit keys)
- SHA-256 hashing
- Automatic key generation
- Secure key storage
- Signature verification

### Before Production Use
1. ✓ Test on non-critical devices
2. ✓ Verify device detection accuracy
3. ✓ Validate certificate generation
4. ✓ Check signature verification
5. ✓ Run in simulation mode first

## Performance

| Operation | Device | Time |
|-----------|--------|------|
| Device Discovery | 5 devices | 2-5s |
| NVMe Format | 500GB | 1-2m |
| ATA Secure Erase | 2TB | 10-30m |
| USB Overwrite | 64GB | 2-5m |
| Certificate Generation | Any | <5s |

## Building & Deployment

### Development Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Docker Deployment
```bash
docker build -t obliterator-genesis:latest .
docker run -it --privileged obliterator-genesis:latest
```

### Bootable USB
```bash
cd bootable-usb
./build.sh
sudo dd if=build/obliterator-genesis-bootable.iso of=/dev/sdX
```

## API Reference

### Desktop Backend (`/api/discover`)
```bash
GET /api/discover
Content-Type: application/json

{
  "success": true,
  "devices": [
    {
      "name": "sda",
      "label": "MyDrive",
      "size_bytes": 1099511627776,
      "size_human": "1.00 TB",
      "bus": "Sata",
      "erase_capability": "AtaSecureErase"
    }
  ]
}
```

### System Status (`/api/status`)
```bash
GET /api/status
```

### Health Check (`/health`)
```bash
GET /health

{"status": "ok"}
```

## CLI Usage Examples

### Scan Devices
```bash
./target/release/obliterator-cli
```

### Filter by Bus Type
```bash
./target/release/obliterator-cli | jq '.devices[] | select(.bus == "Usb")'
```

### Check Specific Device
```bash
./target/release/obliterator-cli | jq '.devices[] | select(.name == "sdb")'
```

## Troubleshooting

### Common Issues

**Desktop won't start backend**
- Check port 3030: `lsof -i :3030`
- Ensure sudo access: `sudo cargo run`
- Check logs in `/tmp/`

**Devices not detected**
- Verify connection: `lsblk`
- Check permissions: Run with `sudo`
- Try different USB ports
- Rescan from GUI

**Certificate generation fails**
- OpenSSL required: `apt-get install openssl`
- Check disk space: `df -h ./certificates/`
- Verify permissions: `chmod 755 ./certificates/`

**Signature verification fails**
- Public key file exists: `ls certificates/keys/public.pem`
- File not corrupted: `file certificates/keys/public.pem`
- Manual verification: Use `openssl dgst`

## Contributing

Improvements welcome! Areas for contribution:
- Additional erase methods
- GUI enhancements
- Documentation
- Device-specific drivers
- Language translations

## Version History

### v0.2.0 (Current) - Desktop Edition
- ✅ Professional desktop GUI
- ✅ Multi-device support (USB, SATA, NVMe)
- ✅ Enhanced label detection (FLASH drives)
- ✅ SATA secure erase support
- ✅ Professional certificate infrastructure
- ✅ RSA-2048 cryptographic signing
- ✅ Bootable USB image builder
- ✅ Dark-mode professional styling

### v0.1.0 - Initial Release
- Basic CLI interface
- USB device detection
- Overwrite erasure
- Simple certificate generation

## License

Professional-grade secure data destruction tool  
OBLITERATOR GENESIS v0.2.0

## Security Notice

⚠️ **WARNING**: This tool performs PERMANENT DATA DESTRUCTION. 

- Always backup critical data
- Test on non-critical devices first
- Verify device selection before erasure
- Use simulation mode for testing
- Keep certificates for audit purposes
- Understand your local regulations

## Support

For documentation and troubleshooting:
1. See detailed READMEs in respective directories
2. Check certificate output for operation details
3. Review system logs for errors
4. Test in simulation mode first

---

**OBLITERATOR GENESIS** - When Data Must Disappear  
*Professional Data Destruction Platform v0.2.0*

**Built with Rust for Security & Performance**
