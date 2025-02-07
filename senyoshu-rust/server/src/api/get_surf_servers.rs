use axum::Json;

use senyoshu_common::types::api::api::SurfServer;

pub async fn get_surf_servers_api(
    Json(()): Json<()>,
) -> Json<Vec<SurfServer>> {
    let servers = Vec::from([
        SurfServer {
            name: "日本-1".to_string(),
            server: "103.75.70.120".to_string(),
            server_port: 34274,
            password: "xmzxis43axpa6h21".to_string(),
            method: "aes-256-gcm".to_string(),
        },
    ]);


    Json(servers)
}