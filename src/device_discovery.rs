use std::process::Command;
use std::error::Error;
use std::fs;
use serde::Serialize;

#[derive(Serialize, Debug, Copy, Clone)]
pub enum BusType {
    Nvme,
    Sata,
    Usb,
   
}

#[derive(Serialize, Debug, Copy, Clone)]
pub enum EraseCapability {
    NvmeFormat,
    AtaSecureErase,
    OverwriteOnly,
    Unsupported,
}

#[derive(Serialize, Debug, Clone)]
pub struct Device {
    pub name: String,
    pub label: Option<String>,
    pub size_bytes: Option<u64>,
    pub dev_type: String,
    pub mountpoint: Option<String>,
    pub sysfs_path: String,
    pub bus: BusType,
    pub erase_capability: EraseCapability,
}

#[derive(Serialize, Debug)]
pub struct DiscoveryReport {
    pub tool: &'static str,
    pub version: &'static str,
    pub devices: Vec<Device>,
}

/// ---------- Utilities ----------

fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let out = Command::new(cmd).args(args).output()?;
    if !out.status.success() {
        return Err(format!("command failed: {} {:?}", cmd, args).into());
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn try_lsblk_json() -> Result<String, Box<dyn Error>> {
    // Request LABEL as well so we can detect filesystem/partition labels (e.g. pen drive names)
    let attempts = [&["-b", "-J", "-o", "NAME,SIZE,TYPE,MOUNTPOINT,LABEL"] as &[&str]];
    for args in attempts {
        if let Ok(out) = run_cmd("lsblk", args) {
            return Ok(out);
        }
    }
    Err("lsblk JSON output unavailable".into())
}

fn bool_from_sysfs(path: &str) -> Option<bool> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .and_then(|v| match v.as_str() {
            "0" => Some(false),
            "1" => Some(true),
            _ => None,
        })
}

fn nvme_supports_format(dev: &str) -> bool {
    // First try running nvme id-ctrl directly
    let try_parse = |output: &std::process::Output| -> Option<bool> {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.trim_start().starts_with("oacs") {
                    if let Some(hex) = line.split("0x").nth(1) {
                        if let Ok(val) = u32::from_str_radix(hex.trim(), 16) {
                            return Some((val & 0x2) != 0);
                        }
                    }
                }
            }
            return Some(false);
        }
        None
    };

    match Command::new("nvme").args(["id-ctrl", dev]).output() {
        Ok(output) => {
            if let Some(res) = try_parse(&output) {
                return res;
            }
            // If it failed (permission denied or other), fall through to try sudo
            eprintln!("nvme id-ctrl returned non-success for {}: {}", dev, String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("nvme CLI not found: install 'nvme-cli' to enable NVMe format detection");
                return false;
            } else {
                eprintln!("failed to execute nvme id-ctrl: {}", e);
            }
        }
    }

    // Retry with sudo -n (non-interactive) to avoid hanging for password prompts
    match Command::new("sudo").args(["-n", "nvme", "id-ctrl", dev]).output() {
        Ok(output) => {
            if let Some(res) = try_parse(&output) {
                return res;
            }
            let stderr = String::from_utf8_lossy(&output.stderr).to_lowercase();
            if stderr.contains("a password is required") || stderr.contains("password") {
                eprintln!("nvme id-ctrl requires sudo password for {}. Prompting for password...", dev);
                // Run interactive sudo: inherit stdin so the user can type the password, capture stdout for parsing,
                // and inherit stderr so the password prompt is visible.
                match Command::new("sudo").args(["nvme", "id-ctrl", dev])
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::inherit())
                    .output()
                {
                    Ok(out2) => {
                        if let Some(res2) = try_parse(&out2) {
                            return res2;
                        }
                        eprintln!("interactive sudo nvme id-ctrl failed for {}", dev);
                    }
                    Err(e) => eprintln!("failed to execute interactive sudo nvme id-ctrl: {}", e),
                }
            } else {
                eprintln!("sudo nvme id-ctrl failed for {}: {}", dev, String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            eprintln!("failed to execute sudo nvme id-ctrl: {}", e);
        }
    }

    false
}

fn sata_supports_secure_erase(dev: &str) -> bool {
    // Check if the device supports ATA Secure Erase via hdparm
    // hdparm -I /dev/sdX returns device info; check for security feature
    // Call hdparm directly; do not force `sudo` here so the caller can
    // decide to run the process with appropriate privileges. Using `sudo`
    // inside library code can prompt for a password and hang the program.
    let try_parse = |out: &std::process::Output| -> Option<bool> {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                if line.contains("Security") && line.contains("enabled") {
                    return Some(true);
                }
                if line.trim_start().starts_with("Secure erase unit time") {
                    return Some(true);
                }
            }
            return Some(false);
        }
        None
    };

    match Command::new("hdparm").args(&["-I", dev]).output() {
        Ok(out) => {
            if let Some(res) = try_parse(&out) {
                return res;
            }
            eprintln!("hdparm -I returned non-success for {}: {}", dev, String::from_utf8_lossy(&out.stderr));
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("hdparm not found: install 'hdparm' or run discovery with sufficient privileges");
                return false;
            } else {
                eprintln!("failed to execute hdparm -I: {}", e);
            }
        }
    }

    // Retry with sudo -n (non-interactive)
    match Command::new("sudo").args(&["-n", "hdparm", "-I", dev]).output() {
        Ok(out) => {
            if let Some(res) = try_parse(&out) {
                return res;
            }
            let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
            if stderr.contains("a password is required") || stderr.contains("password") {
                eprintln!("hdparm -I requires sudo password for {}. Prompting for password...", dev);
                match Command::new("sudo").args(&["hdparm", "-I", dev])
                    .stdin(std::process::Stdio::inherit())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::inherit())
                    .output()
                {
                    Ok(out2) => {
                        if let Some(res2) = try_parse(&out2) {
                            return res2;
                        }
                        eprintln!("interactive sudo hdparm -I failed for {}", dev);
                    }
                    Err(e) => eprintln!("failed to execute interactive sudo hdparm -I: {}", e),
                }
            } else {
                eprintln!("sudo hdparm -I failed for {}: {}", dev, String::from_utf8_lossy(&out.stderr));
            }
        }
        Err(e) => eprintln!("failed to execute sudo hdparm -I: {}", e),
    }

    false
}

/// Discovers all block devices on the system
pub async fn discover_all_devices() -> Result<DiscoveryReport, Box<dyn Error>> {
    let lsblk_json = try_lsblk_json()?;
    let parsed: serde_json::Value = serde_json::from_str(&lsblk_json)?;

    let blocks = parsed
        .get("blockdevices")
        .and_then(|v| v.as_array())
        .ok_or("unexpected lsblk JSON format")?;

    let mut devices = Vec::new();

    for b in blocks {
        let name = b.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if name.is_empty() {
            continue;
        }

        let dev_type = b.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        if dev_type == "loop" {
            continue;
        }

        let sysfs = format!("/sys/block/{}", name);
        let dev_link = fs::read_link(format!("{}/device", sysfs))
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
            .unwrap_or_default();
        let removable = bool_from_sysfs(&format!("{}/removable", sysfs)).unwrap_or(false);

        let bus = if dev_link.contains("nvme") {
            BusType::Nvme
        } else if removable {
            BusType::Usb
        } else {
            BusType::Sata
        };

        let erase_capability = match bus {
            BusType::Nvme => {
                if nvme_supports_format(&format!("/dev/{}", name)) {
                    EraseCapability::NvmeFormat
                } else {
                    EraseCapability::Unsupported
                }
            }
            BusType::Sata => {
                if sata_supports_secure_erase(&format!("/dev/{}", name)) {
                    EraseCapability::AtaSecureErase
                } else {
                    EraseCapability::OverwriteOnly
                }
            }
            BusType::Usb => EraseCapability::OverwriteOnly,
            
        };

        // Enhanced label detection: check device label first, then children partitions
        // This handles cases where USB drives have labels at different hierarchy levels
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

        let device = Device {
            name: name.clone(),
            label,
            // `size` may be a number or string in lsblk JSON; handle both
            size_bytes: {
                b.get("size").and_then(|v| {
                    v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                })
            },
            dev_type: dev_type.to_string(),
            mountpoint: b.get("mountpoint").and_then(|v| v.as_str()).map(|s| s.to_string()),
            sysfs_path: sysfs.clone(),
            bus,
            erase_capability,
        };

        devices.push(device);
    }

    let report = DiscoveryReport {
        tool: "obliterator-genesis",
        version: "0.2.0",
        devices,
    };

    Ok(report)
}
