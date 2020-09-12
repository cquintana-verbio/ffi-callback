use warp::{http::StatusCode, Filter, Rejection, Reply};

mod ws;

async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}

async fn ws_handler(ws: warp::ws::Ws) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(ws::ws_handler))
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let health_route = warp::path!("health").and_then(health_handler);
    let ws_route = warp::path("ws").and(warp::ws()).and_then(ws_handler);
    let routes = health_route
        .or(ws_route)
        .with(warp::cors().allow_any_origin());

    let port = 8000;
    log::info!("Server listening at :{}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
