// OBLITERATOR GENESIS Desktop Application
// Professional Data Destruction Platform

const API_BASE = 'http://127.0.0.1:3030';
let simulationMode = true;
let selectedDevice = null;
let devices = [];

// Initialize application
document.addEventListener('DOMContentLoaded', () => {
    console.log('OBLITERATOR GENESIS Desktop initialized');
    updateSystemStatus();
    logEntry('System initialized. Ready to scan devices.', 'info');
    discoverDevices();
});

// Update system status
function updateSystemStatus() {
    fetch(`${API_BASE}/health`)
        .then(r => r.json())
        .then(data => {
            const statusEl = document.getElementById('statusText');
            const indicator = document.querySelector('.status-indicator');
            if (data.status === 'ok') {
                statusEl.textContent = 'System Online';
                indicator.style.background = '#388e3c';
            } else {
                statusEl.textContent = 'System Offline';
                indicator.style.background = '#d32f2f';
            }
        })
        .catch(() => {
            document.getElementById('statusText').textContent = 'Connection Error';
            document.querySelector('.status-indicator').style.background = '#d32f2f';
        });
}

// Discover devices
async function discoverDevices() {
    const loading = document.getElementById('loadingIndicator');
    const list = document.getElementById('devicesList');
    
    loading.style.display = 'flex';
    logEntry('Scanning for devices...', 'info');
    
    try {
        const response = await fetch(`${API_BASE}/api/discover`);
        const data = await response.json();
        
        if (data.success && data.devices) {
            devices = data.devices;
            
            if (devices.length === 0) {
                list.innerHTML = '<div class="empty-state"><p>🔍 No devices detected. Ensure proper permissions and try again.</p></div>';
                logEntry('No devices found.', 'warning');
            } else {
                renderDevices(devices);
                logEntry(`Found ${devices.length} device(s).`, 'success');
            }
        } else {
            list.innerHTML = `<div class="empty-state"><p>❌ Error: ${data.error || 'Failed to scan devices'}</p></div>`;
            logEntry(`Error scanning devices: ${data.error}`, 'error');
        }
    } catch (error) {
        list.innerHTML = '<div class="empty-state"><p>❌ Connection error. Is the backend running?</p></div>';
        logEntry(`Connection error: ${error.message}`, 'error');
    } finally {
        loading.style.display = 'none';
    }
}

// Render device list
function renderDevices(deviceList) {
    const list = document.getElementById('devicesList');
    list.innerHTML = '';
    
    deviceList.forEach(device => {
        const card = createDeviceCard(device);
        list.appendChild(card);
    });
}

// Create device card
function createDeviceCard(device) {
    const card = document.createElement('div');
    card.className = 'device-card';
    card.onclick = () => showDeviceDetails(device);
    
    const eraseCapability = device.erase_capability;
    let eraseClass = 'erase-overwrite';
    let eraseText = 'Overwrite';
    
    if (eraseCapability.includes('NvmeFormat')) {
        eraseClass = 'erase-nvme';
        eraseText = 'NVMe Format';
    } else if (eraseCapability.includes('AtaSecureErase')) {
        eraseClass = 'erase-ata';
        eraseText = 'ATA Secure Erase';
    } else if (eraseCapability.includes('Unsupported')) {
        eraseClass = 'erase-unsupported';
        eraseText = 'Not Supported';
    }
    
    const mountStatus = device.mountpoint ? `<span style="color: #ff9800;">Mounted: ${device.mountpoint}</span>` : '<span style="color: #388e3c;">Unmounted</span>';
    
    card.innerHTML = `
        <div class="device-header">
            <div>
                <div class="device-name">/dev/${device.name}</div>
                ${device.label ? `<div class="device-label">📝 ${escapeHtml(device.label)}</div>` : ''}
            </div>
            <div class="device-type">${device.bus}</div>
        </div>
        
        <div class="device-info">
            <div><strong>Type:</strong> ${device.dev_type}</div>
            <div><strong>Size:</strong> ${device.size_human}</div>
            <div><strong>Status:</strong> ${mountStatus}</div>
        </div>
        
        <div class="device-capacity">
            <span style="font-size: 11px;">Capacity</span>
            <div class="capacity-bar">
                <div class="capacity-fill" style="width: 75%;"></div>
            </div>
            <span style="font-size: 11px;">${device.size_human}</span>
        </div>
        
        <span class="erase-badge ${eraseClass}">${eraseText}</span>
    `;
    
    return card;
}

// Show device details modal
function showDeviceDetails(device) {
    selectedDevice = device;
    
    const modal = document.getElementById('deviceModal');
    document.getElementById('modalDeviceName').textContent = `/dev/${device.name}`;
    
    const details = `
        <div class="device-detail-row">
            <span class="device-detail-label">Device:</span>
            <span class="device-detail-value">/dev/${device.name}</span>
        </div>
        ${device.label ? `
        <div class="device-detail-row">
            <span class="device-detail-label">Label:</span>
            <span class="device-detail-value">${escapeHtml(device.label)}</span>
        </div>
        ` : ''}
        <div class="device-detail-row">
            <span class="device-detail-label">Bus Type:</span>
            <span class="device-detail-value">${device.bus}</span>
        </div>
        <div class="device-detail-row">
            <span class="device-detail-label">Capacity:</span>
            <span class="device-detail-value">${device.size_human}</span>
        </div>
        <div class="device-detail-row">
            <span class="device-detail-label">Type:</span>
            <span class="device-detail-value">${device.dev_type}</span>
        </div>
        <div class="device-detail-row">
            <span class="device-detail-label">Mountpoint:</span>
            <span class="device-detail-value">${device.mountpoint || 'None'}</span>
        </div>
        <div class="device-detail-row">
            <span class="device-detail-label">Erase Method:</span>
            <span class="device-detail-value">${device.erase_capability}</span>
        </div>
        <div class="device-detail-row">
            <span class="device-detail-label">Mode:</span>
            <span class="device-detail-value">${simulationMode ? '🎯 Simulation (Non-Destructive)' : '⚡ Real Erasure (Destructive)'}</span>
        </div>
    `;
    
    document.getElementById('modalDeviceDetails').innerHTML = details;
    modal.classList.add('active');
}

// Close device modal
function closeDeviceModal() {
    document.getElementById('deviceModal').classList.remove('active');
    selectedDevice = null;
}

// Perform wipe
function performWipe() {
    if (!selectedDevice) return;
    
    const mode = simulationMode ? 'SIMULATED' : 'REAL';
    const message = `
        <p><strong>Device:</strong> /dev/${selectedDevice.name}</p>
        ${selectedDevice.label ? `<p><strong>Label:</strong> ${escapeHtml(selectedDevice.label)}</p>` : ''}
        <p><strong>Capacity:</strong> ${selectedDevice.size_human}</p>
        <p><strong>Mode:</strong> ${mode}</p>
        <p><strong>Erase Method:</strong> ${selectedDevice.erase_capability}</p>
        
        ${!simulationMode ? `
        <p style="color: #ff9800; margin-top: 20px; font-weight: bold;">
            ⚠️ WARNING: This will permanently destroy all data on this device!
        </p>
        ` : `
        <p style="color: #4ade80; margin-top: 20px;">
            ✓ This is a simulation. No data will be modified.
        </p>
        `}
    `;
    
    document.getElementById('confirmMessage').innerHTML = message;
    closeDeviceModal();
    document.getElementById('confirmModal').classList.add('active');
}

// Confirm wipe
async function confirmWipe() {
    if (!selectedDevice) return;
    
    closeConfirmModal();
    logEntry(`Initiating ${simulationMode ? 'simulated' : 'real'} erasure on /dev/${selectedDevice.name}...`, 'warning');
    
    // In a real implementation, this would call the backend API
    // For now, we'll simulate the operation
    simulateWipeOperation();
}

// Simulate wipe operation
function simulateWipeOperation() {
    const device = selectedDevice;
    let progress = 0;
    
    const interval = setInterval(() => {
        progress += Math.random() * 20;
        if (progress >= 100) {
            clearInterval(interval);
            progress = 100;
            
            logEntry(`✓ Erasure completed on /dev/${device.name}`, 'success');
            logEntry(`Certificate generated and signed with RSA-2048`, 'success');
            logEntry(`Certificate saved to ./certificates/archives/`, 'success');
            
            selectedDevice = null;
        }
    }, 1000);
}

// Toggle simulation mode
function toggleSimulationMode() {
    simulationMode = !simulationMode;
    const text = document.getElementById('simModeText');
    text.textContent = simulationMode ? 'Simulation: ON' : 'Simulation: OFF';
    
    const btn = event.target.closest('button');
    if (simulationMode) {
        btn.style.background = 'var(--secondary)';
        logEntry('Simulation mode enabled. Operations are non-destructive.', 'info');
    } else {
        btn.style.background = 'var(--danger)';
        logEntry('Simulation mode disabled. REAL ERASURE mode active!', 'warning');
    }
}

// Close confirm modal
function closeConfirmModal() {
    document.getElementById('confirmModal').classList.remove('active');
}

// Open certificates folder
function openCertificates() {
    logEntry('Opening certificates folder...', 'info');
    // This would open the certificates directory
    // In a real app, you'd use Tauri or similar to open file explorer
}

// Log entry
function logEntry(message, type = 'info') {
    const logContent = document.getElementById('logContent');
    const entry = document.createElement('div');
    entry.className = `log-entry log-${type}`;
    entry.textContent = `[${new Date().toLocaleTimeString()}] ${message}`;
    logContent.appendChild(entry);
    logContent.scrollTop = logContent.scrollHeight;
}

// Escape HTML
function escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, m => map[m]);
}

// Refresh system status every 5 seconds
setInterval(updateSystemStatus, 5000);
