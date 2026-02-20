# OBLITERATOR GENESIS - Desktop Application

**Professional-Grade Data Destruction & Secure Erasure Platform**

A comprehensive desktop application for secure data erasure with support for USB, SATA, and NVMe devices with cryptographically signed certificates.

## Features

### Core Capabilities
- ✅ **Multi-Device Support**: USB, SATA, NVMe detection
- ✅ **Multiple Erase Methods**: 
  - NVMe Format (fastest, most secure)
  - ATA Secure Erase (hardware-level)
  - Overwrite with zeros (compatible fallback)
- ✅ **Simulation Mode**: Non-destructive testing before real operations
- ✅ **Professional Certificates**: RSA-2048 signed JSON, HTML, PDF reports
- ✅ **Secure Device Detection**: Strict verification of device capabilities
- ✅ **Modern GUI**: Dark-mode professional interface

### Security Features
- **Cryptographic Signing**: All operations signed with RSA-2048
- **Payload Integrity**: SHA-256 hashing of all operation data
- **Audit Trail**: Comprehensive logging with timestamps
- **Key Management**: Automatic RSA keypair generation and storage
- **Operation Verification**: Distinguish real vs. simulated operations

## Project Structure

```
desktop/
├── src/
│   └── main.rs           # Warp web server backend
├── ui/
│   ├── index.html        # Main interface
│   ├── style.css         # Professional styling
│   └── app.js            # Frontend logic
├── Cargo.toml
└── README.md             # This file
```

## Building & Running

### Prerequisites
- Rust 1.70+ with Cargo
- libssl-dev (for OpenSSL)
- Node.js (optional, for development server)

### Quick Start

```bash
# Build the desktop application
cd desktop
cargo build --release

# Run the backend server
cargo run

# In another terminal, serve the UI
cd ui
python3 -m http.server 8080

# Open in browser: http://127.0.0.1:8080
```

### Docker Deployment

```bash
# Build Docker image
docker build -t obliterator-genesis:desktop -f Dockerfile.desktop .

# Run container
docker run -it --privileged obliterator-genesis:desktop
```

## Architecture

### Backend (Rust + Warp)
- **Device Discovery**: Uses `lsblk` for robust device detection
- **REST API**: Warp web framework for HTTP endpoints
- **Secure Signing**: OpenSSL integration for RSA operations
- **Cross-Platform**: Works on Linux, macOS (partial), Windows (WSL)

### Frontend (HTML/CSS/JavaScript)
- **Real-Time Monitoring**: Live device list updates
- **Professional UI**: Dark mode with intuitive controls
- **Modal Workflows**: Step-by-step confirmation for erasures
- **Responsive Design**: Works on desktop and tablet

## API Endpoints

### `/api/discover`
Scans for and returns available storage devices.

**Response:**
```json
{
  "success": true,
  "devices": [
    {
      "name": "sda",
      "label": "FLASH",
      "size_bytes": 15728640000,
      "size_human": "14.66 GB",
      "dev_type": "disk",
      "bus": "Usb",
      "erase_capability": "OverwriteOnly"
    }
  ]
}
```

### `/api/status`
Returns application status and version.

### `/health`
Health check endpoint for connectivity verification.

## Certificate Management

All erasure operations generate professional certificates:

### Certificate Storage
```
certificates/
├── keys/
│   ├── private.pem       # RSA-2048 private key
│   └── public.pem        # Public key for verification
├── payloads/
│   └── payload_<ID>.json # Operation payload
├── signatures/
│   └── payload_<ID>.sig  # Binary signature
└── archives/
    ├── wipe_certificate_<DEV>_<TS>.json
    ├── wipe_certificate_<DEV>_<TS>.html
    └── wipe_certificate_<DEV>_<TS>.pdf
```

### Certificate Content
Each certificate includes:
- Device name and label
- Erasure method used
- Timestamp and operator
- Success/failure status
- Simulated vs. real operation flag
- SHA-256 payload hash
- RSA-SHA256 signature (base64)
- Duration in seconds

## Device Detection

### Bus Type Detection
- **NVMe**: via `/sys/block/*/device` symlink
- **SATA**: non-removable, non-NVMe
- **USB**: removable block devices

### Erase Capability Detection
- **NVMe Format**: Checked via `nvme id-ctrl` for format support
- **ATA Secure Erase**: Verified through `hdparm -I` for security features
- **Overwrite**: Fallback option for all devices

## Usage Guide

### 1. Scan Devices
Click "Scan Devices" to detect connected storage.

### 2. Select Device
Click on a device card to view detailed information.

### 3. Choose Mode
- **Simulation Mode (Default)**: Test without destroying data
- **Real Erasure**: Destructive mode (requires confirmation)

### 4. Perform Erasure
- Click "Perform Erasure" in device modal
- Review confirmation dialog
- Operation begins with progress indication

### 5. Review Certificate
Certificates are automatically generated and saved in `./certificates/archives/`

## Command Line Usage

### Scan Only
```bash
./obliterator-cli
```

### With Full Device Details
```bash
./obliterator-cli | jq '.devices'
```

## Security Considerations

### Before Production Deployment
1. ✓ Run all operations from a live boot environment
2. ✓ Verify device detection before erasure
3. ✓ Keep certificates for audit purposes
4. ✓ Restrict access to `certificates/keys/` directory
5. ✓ Test on non-critical devices first

### Permissions
- Requires `root` or `sudo` access for device operations
- OpenSSL must be available for signing
- `/sys/block/` filesystem must be readable

## Troubleshooting

### Backend Connection Error
- Ensure backend is running: `cargo run` in `desktop/` directory
- Check port 3030 is not in use: `lsof -i :3030`

### No Devices Detected
- Verify USB/SATA devices are properly connected
- Check permissions: `sudo ./target/debug/obliterator-desktop`
- Inspect logs for detailed errors

### Certificate Generation Failed
- Ensure OpenSSL is installed: `openssl version`
- Check directory permissions on `./certificates/`

### Signature Verification Failed
- Public key must be present: `certificates/keys/public.pem`
- For manual verification: `openssl dgst -sha256 -verify public.pem -signature sig payload.json`

## Performance

- Device discovery: ~1-2 seconds
- NVMe format: ~30-60 seconds (varies by device)
- ATA Secure Erase: ~5-30 minutes (device dependent)
- Overwrite (1GB): ~30-60 seconds

## Browser Compatibility

- Chrome/Chromium 90+
- Firefox 88+
- Safari 14+
- Edge 90+

## Building for Bootable USB

### Create Bootable Image
```bash
# Build for release
cargo build --release -p obliterator-discovery

# Create bootable ISO with custom kernel and filesystem
# See bootable-usb/ directory for detailed instructions
```

### Boot Environment Setup
The application can be embedded in:
- Linux live environment (Ubuntu Core, Alpine)
- Custom initramfs
- GRUB/UEFI bootloader

## Advanced Configuration

### Custom Certificate Location
Set environment variable before running:
```bash
export CERT_DIR=/path/to/certificates
./target/release/obliterator-desktop
```

### Disable Signing (for testing)
Remove OpenSSL requirement by modifying executor.rs (not recommended for production).

## License

Professional grade - Secure Data Destruction Platform

## Version

**v0.2.0** - Multi-device support with desktop GUI

---

**OBLITERATOR GENESIS** - *When data must disappear.*
