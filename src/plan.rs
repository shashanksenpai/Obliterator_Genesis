use serde::Serialize;

#[derive(Debug, Serialize, Copy, Clone)]
pub enum WipeMethod {
    NvmeFormat,
    AtaSecureErase,
    Overwrite,
}

#[derive(Debug, Serialize, Clone)]
pub struct WipePlan {
    pub device: String,
    pub method: WipeMethod,
    pub destructive: bool,
}

use crate::device_discovery::{EraseCapability, Device};

/// Build a wipe plan for a device based on its erase capabilities
pub fn build_plan(dev: &Device) -> Option<WipePlan> {
    match dev.erase_capability {
        EraseCapability::NvmeFormat => Some(WipePlan {
            device: dev.name.clone(),
            method: WipeMethod::NvmeFormat,
            destructive: true,
        }),
        EraseCapability::AtaSecureErase => Some(WipePlan {
            device: dev.name.clone(),
            method: WipeMethod::AtaSecureErase,
            destructive: true,
        }),
        EraseCapability::OverwriteOnly => Some(WipePlan {
            device: dev.name.clone(),
            method: WipeMethod::Overwrite,
            destructive: true,
        }),
        EraseCapability::Unsupported => None,
    }
}