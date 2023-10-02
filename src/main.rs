use axum::Router;
use axum::TypedHeader;
use axum::{
    response::sse::{Event, Sse},
    routing::get,
};

use futures::stream::{self, Stream};
use std::process::Command;
use std::{convert::Infallible, net::SocketAddr, path::PathBuf, time::Duration};
use tokio_stream::StreamExt as _;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_sse=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    let app = Router::new()
        .fallback_service(static_files_service)
        .route("/sse", get(sse_handler))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn command() -> String {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--format")
        .arg("{{json .}}")
        .output()
        .expect("ls command failed to start");
    let res = String::from_utf8_lossy(&output.stdout);
    res.to_string()
}

async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let stream = stream::repeat_with(|| Event::default().data(command()))
        .map(Ok)
        .throttle(Duration::from_secs(5));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}
