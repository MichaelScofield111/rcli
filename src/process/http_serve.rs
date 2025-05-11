use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    // start server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serveing {:?} on {}", path, addr);

    let state = HttpServeState { path };
    // axum route
    let router = Router::new()
        .route("/{*path}", get(file_handle))
        .with_state(Arc::new(state));

    info!("Serving {:?} on {}", addr, port);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn file_handle(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, "Not found".to_string())
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                tracing::warn!("Read by bytes {:?}", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                tracing::warn!("Read file  failed {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        }
    }
}
