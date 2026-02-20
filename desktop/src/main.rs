use warp::Filter;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StatusResponse {
    status: String,
    version: String,
    api_version: String,
}

fn format_bytes(bytes: Option<u64>) -> String {
    match bytes {
        Some(b) => {
            let units = ["B", "KB", "MB", "GB", "TB"];
            let mut size = b as f64;
            let mut idx = 0;
            while size >= 1024.0 && idx < units.len() - 1 {
                size /= 1024.0;
                idx += 1;
            }
            format!("{:.2} {}", size, units[idx])
        }
        None => "Unknown".to_string(),
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    // Health check endpoint
    let health = warp::path!("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&HealthResponse {
                status: "ok".to_string(),
                version: "0.2.0".to_string(),
            })
        });

    // Status endpoint
    let status = warp::path!("api" / "status")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&StatusResponse {
                status: "running".to_string(),
                version: "0.2.0".to_string(),
                api_version: "1.0".to_string(),
            })
        });

    // Device discovery endpoint - simplified for Warp compatibility
    let discover = warp::path!("api" / "discover")
        .and(warp::get())
        .map(|| {
            // For now, return empty device list - GUI can call /api/discover/scan
            warp::reply::json(&serde_json::json!({
                "success": true,
                "devices": [],
                "message": "Use /api/discover/scan to scan for devices"
            }))
        });

    let routes = health.or(status).or(discover);

    println!("====================================");
    println!("OBLITERATOR GENESIS v0.2.0");
    println!("Desktop Application Backend");
    println!("====================================");
    println!();
    println!("Backend API: http://127.0.0.1:3030");
    println!("Health Check: GET /health");
    println!("Status: GET /api/status");
    println!("Discover: GET /api/discover");
    println!();
    println!("Web UI served from: ./desktop/ui/");
    println!("Open http://127.0.0.1:8080 in browser");
    println!();

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
