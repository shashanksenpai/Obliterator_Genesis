# OBLITERATOR GENESIS - Command Reference

Quick reference for all common commands to use OBLITERATOR GENESIS v0.2.0

---

## 🏗️ Building

### Build All Components (Debug)
```bash
cd /home/apsit/OBLITERATOR_GENESIS
cargo build
```

### Build All Components (Release - Optimized)
```bash
cd /home/apsit/OBLITERATOR_GENESIS
cargo build --release
```

### Build Desktop Application Only
```bash
cd /home/apsit/OBLITERATOR_GENESIS/desktop
cargo build --release
```

### Check Compilation Without Building
```bash
cd /home/apsit/OBLITERATOR_GENESIS
cargo check
```

---

## 🔍 Device Scanning

### Scan All Devices (JSON Format)
```bash
sudo ./target/release/obliterator-cli
```

### Scan and Pretty-Print JSON
```bash
sudo ./target/release/obliterator-cli | python3 -m json.tool
```

### Find USB Devices Only
```bash
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.bus == "Usb")'
```

### Find SATA Devices Only
```bash
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.bus == "Sata")'
```

### Find NVMe Devices Only
```bash
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.bus == "Nvme")'
```

### Find Device by Label
```bash
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.label == "FLASH")'
```

### Find Device by Name
```bash
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.name == "sdb")'
```

### List All Device Names
```bash
sudo ./target/release/obliterator-cli | jq '.devices[].name'
```

### List All Device Labels
```bash
sudo ./target/release/obliterator-cli | jq '.devices[] | select(.label != null) | .label'
```

---

## 🖥️ Desktop GUI Application

### Terminal 1: Start Backend API Server
```bash
cd /home/apsit/OBLITERATOR_GENESIS/desktop
cargo run --release
```

### Terminal 2: Serve Web UI (In Another Terminal)
```bash
cd /home/apsit/OBLITERATOR_GENESIS/desktop/ui
python3 -m http.server 8080
```

### Open in Browser
```
http://localhost:8080
```

### Alternative Port (if 8080 is in use)
```bash
cd /home/apsit/OBLITERATOR_GENESIS/desktop/ui
python3 -m http.server 9090  # Use port 9090 instead
# Then open: http://localhost:9090
```

### Backend on Different Port (if 3030 is in use)
Modify `desktop/src/main.rs` line 55:
```rust
.run(([127, 0, 0, 1], 3031))  // Changed from 3030 to 3031
```
Then rebuild:
```bash
cargo build --release
```

---

## 📜 Certificate Management

### List All Certificates
```bash
ls -la certificates/archives/
```

### View Latest Certificate (JSON)
```bash
ls -1t certificates/archives/*.json | head -1 | xargs cat | python3 -m json.tool
```

### View Latest Certificate (HTML)
```bash
ls -1t certificates/archives/*.html | head -1 | xargs firefox  # or chrome
```

### Check Public Key
```bash
cat certificates/keys/public.pem
```

### Verify a Certificate Signature
```bash
CERT_ID="abc123def456"  # Replace with actual ID
openssl dgst -sha256 -verify certificates/keys/public.pem \
    -signature certificates/signatures/payload_${CERT_ID}.sig \
    certificates/payloads/payload_${CERT_ID}.json
```

### Extract Certificate Metadata
```bash
cat certificates/archives/wipe_certificate_sdb_*.json | jq '.timestamp, .device_label, .erase_capability'
```

### Count Total Certificates
```bash
ls certificates/archives/*.json | wc -l
```

### Backup All Certificates
```bash
tar czf obliterator-certificates-backup.tar.gz certificates/
```

---

## 🚀 Bootable USB Creation

### Build Bootable ISO Image
```bash
cd /home/apsit/OBLITERATOR_GENESIS/bootable-usb
sudo chmod +x build.sh
sudo ./build.sh
```

### List USB Devices
```bash
lsblk | grep disk
# Or
sudo fdisk -l | grep ^/dev/sd
```

### Write ISO to USB (Replace sdX with actual device)
```bash
sudo dd if=/home/apsit/OBLITERATOR_GENESIS/bootable-usb/build/obliterator-genesis-bootable.iso of=/dev/sdX bs=4M status=progress
sudo sync
```

### Examples:
```bash
# USB at /dev/sdb
sudo dd if=bootable-usb/build/obliterator-genesis-bootable.iso of=/dev/sdb bs=4M status=progress

# USB at /dev/sdc
sudo dd if=bootable-usb/build/obliterator-genesis-bootable.iso of=/dev/sdc bs=4M status=progress
```

### Verify USB Write
```bash
sudo dd if=/dev/sdX bs=4M | md5sum
```

---

## 🔐 Cryptographic Operations

### Regenerate RSA Keys
```bash
rm certificates/keys/private.pem certificates/keys/public.pem
# Next operation will regenerate them
sudo ./target/release/obliterator-cli
```

### Display Private Key (Careful!)
```bash
cat certificates/keys/private.pem
```

### Display Public Key
```bash
cat certificates/keys/public.pem
```

### Manually Sign a File
```bash
FILE="certificates/payloads/payload_abc123.json"
openssl dgst -sha256 -sign certificates/keys/private.pem -out signature.sig "$FILE"
```

### Manually Verify Signature
```bash
openssl dgst -sha256 -verify certificates/keys/public.pem \
    -signature signature.sig payload.json
```

---

## 📊 System Information

### Check Rust Version
```bash
rustc --version
cargo --version
```

### Check System Architecture
```bash
uname -m  # Should show x86_64
```

### Check lsblk Available
```bash
lsblk --version
```

### Check OpenSSL Available
```bash
openssl version
```

### Check Python Version
```bash
python3 --version
```

---

## 🐛 Troubleshooting Commands

### Check Backend Connection
```bash
curl http://127.0.0.1:3030/health
```

### Check API Status
```bash
curl http://127.0.0.1:3030/api/status
```

### Check Device Discovery Endpoint
```bash
curl http://127.0.0.1:3030/api/discover
```

### View Recent Logs
```bash
journalctl -xe | tail -50
```

### Check Disk Space
```bash
df -h
```

### Check Port Usage
```bash
sudo lsof -i :3030      # Check port 3030
sudo lsof -i :8080      # Check port 8080
```

### Kill Process on Port (if needed)
```bash
sudo lsof -i :3030 | grep LISTEN | awk '{print $2}' | xargs kill -9
```

### Check File Permissions
```bash
ls -la certificates/
ls -la certificates/keys/
```

### Test Device Access
```bash
sudo blkid
sudo lsblk -f
```

---

## 📁 File Navigation

### Go to Project Directory
```bash
cd /home/apsit/OBLITERATOR_GENESIS
```

### Go to Desktop App Directory
```bash
cd /home/apsit/OBLITERATOR_GENESIS/desktop
```

### Go to Bootable USB Directory
```bash
cd /home/apsit/OBLITERATOR_GENESIS/bootable-usb
```

### Go to UI Directory
```bash
cd /home/apsit/OBLITERATOR_GENESIS/desktop/ui
```

### Go to Certificates Directory
```bash
cd /home/apsit/OBLITERATOR_GENESIS/certificates
```

### List Source Code Files
```bash
ls -la src/
```

### View Cargo.toml
```bash
cat Cargo.toml
```

---

## 📖 Documentation Commands

### View Main README
```bash
cat README.md | less
```

### View Quick Start Guide
```bash
cat QUICKSTART.md | less
```

### View Completion Report
```bash
cat COMPLETION_REPORT.md | less
```

### View Implementation Summary
```bash
cat IMPLEMENTATION_SUMMARY.md | less
```

### View Desktop README
```bash
cat desktop/README.md | less
```

### View Bootable USB README
```bash
cat bootable-usb/README.md | less
```

### View Certificate README
```bash
cat certificates/README.md | less
```

---

## 🧪 Testing Commands

### Test CLI Device Scanning
```bash
sudo ./target/release/obliterator-cli 2>&1 | grep -c '"name"'
# Output: Number of devices found
```

### Test GUI Responsiveness
```bash
time curl http://127.0.0.1:3030/health
```

### Test Certificate Generation
```bash
ls -lt certificates/archives/ | head -5
```

### Run All Tests (if available)
```bash
cargo test
```

---

## 🔧 Development Commands

### Format Code
```bash
cargo fmt
```

### Check Formatting
```bash
cargo fmt -- --check
```

### Lint Code
```bash
cargo clippy
```

### Generate Documentation
```bash
cargo doc --open
```

### Check Dependencies
```bash
cargo tree
```

### Update Dependencies
```bash
cargo update
```

---

## 🎯 Quick Start Templates

### Complete Development Workflow
```bash
cd /home/apsit/OBLITERATOR_GENESIS
cargo build --release
sudo ./target/release/obliterator-cli
```

### Complete GUI Workflow
```bash
# Terminal 1
cd /home/apsit/OBLITERATOR_GENESIS/desktop
cargo run --release

# Terminal 2
cd /home/apsit/OBLITERATOR_GENESIS/desktop/ui
python3 -m http.server 8080

# Browser
# Open http://localhost:8080
```

### Complete Bootable USB Workflow
```bash
cd /home/apsit/OBLITERATOR_GENESIS/bootable-usb
sudo chmod +x build.sh
sudo ./build.sh
lsblk | grep disk
sudo dd if=build/obliterator-genesis-bootable.iso of=/dev/sdX bs=4M status=progress
```

---

## ⌨️ Keyboard Shortcuts (GUI)

- **F5**: Refresh device list (in browser)
- **Ctrl+A**: Select all text (browser standard)
- **Ctrl+C**: Copy text (browser standard)
- **Esc**: Close modal dialogs

---

## 🆘 Emergency Commands

### Kill Backend Server
```bash
pkill -f obliterator-desktop
```

### Force Kill Port 3030
```bash
sudo lsof -i :3030 -t | xargs kill -9
```

### Force Kill Port 8080
```bash
sudo lsof -i :8080 -t | xargs kill -9
```

### Reset Certificates (DESTRUCTIVE)
```bash
rm -rf certificates/  # WARNING: Deletes all certificates
mkdir certificates/
```

### Clear Build Cache
```bash
cargo clean
```

---

## 📞 Support

**For issues**, check these in order:
1. `QUICKSTART.md` - Common setup issues
2. `IMPLEMENTATION_SUMMARY.md` - Technical details
3. `desktop/README.md` - GUI-specific issues
4. `bootable-usb/README.md` - Bootable USB issues
5. `certificates/README.md` - Certificate issues

---

**OBLITERATOR GENESIS v0.2.0**  
Professional Data Destruction Platform

*When data must disappear.*
