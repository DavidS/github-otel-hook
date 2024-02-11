use axum::{routing::get, Router};
use tracing::instrument;
use tracing_subscriber::fmt::format::FmtSpan;

#[instrument(level = "info")]
async fn index() -> String {
    tracing::info!("inside index!");
    String::from("homepage")
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(index));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
