use warp::Filter;
use warp::http::Method;
use serde::{Serialize, Deserialize};
use obliterator_discovery::device_discovery::discover_all_devices;
use obliterator_discovery::DiscoveryReport;

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

    // Device discovery endpoints
    // Lightweight hint endpoint
    let discover = warp::path!("api" / "discover")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "success": true,
                "devices": [],
                "message": "Call /api/discover/scan to run a full discovery"
            }))
        });

    // Endpoint that triggers a full discovery scan and returns results
    let discover_scan = warp::path!("api" / "discover" / "scan")
        .and(warp::get())
        .and_then(|| async move {
            match discover_all_devices().await {
                Ok(report) => Ok::<_, warp::Rejection>(warp::reply::json(&report)),
                Err(e) => Ok(warp::reply::json(&serde_json::json!({
                    "success": false,
                    "error": format!("Discovery failed: {}", e)
                })))
            }
        });

    let routes = health.or(status).or(discover).or(discover_scan);

    // Allow CORS so the UI (served on a different local port) can call the API
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(vec!["content-type"])
        .build();

    let routes = routes.with(cors);

    println!("====================================");
    println!("OBLITERATOR GENESIS v0.2.0");
    println!("Desktop Application Backend");
    println!("====================================");
    println!();
    println!("Backend API: http://127.0.0.1:3030 (listening on all interfaces)");
    println!("Health Check: GET /health");
    println!("Status: GET /api/status");
    println!("Discover: GET /api/discover");
    println!();
    println!("Web UI served from: ./desktop/ui/");
    println!("Open http://127.0.0.1:8080 in browser");
    println!();

    // Listen on all interfaces so the UI (served on a different local port
    // or host) can reach the backend during development.
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
