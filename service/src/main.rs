use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    routing::{get, post},
    Router,
};
use std::fmt::Debug;
use tracing::{error, info, instrument};
use tracing_subscriber::fmt::format::FmtSpan;

trait WithHttpStatus<T, E> {
    fn with_http_status(self, code: StatusCode) -> Result<T, StatusCode>;
}

impl<T, E> WithHttpStatus<T, E> for Result<T, E>
where
    E: Debug,
{
    fn with_http_status(self, code: StatusCode) -> Result<T, StatusCode> {
        self.map_err(|e| {
            error!(?e, status = ?code, "error");
            code
        })
    }
}

#[instrument(level = "info")]
async fn index() -> String {
    tracing::info!("inside index!");
    String::from("homepage")
}

#[instrument(level = "info", skip_all)]
async fn webhook(headers: HeaderMap, body: Bytes) -> Result<String, StatusCode> {
    if let Some(evt) = headers.get("x-github-event") {
        let evt_name = evt.to_str().with_http_status(StatusCode::BAD_REQUEST)?;
        match evt_name {
            "ping" => {
                info!("received ping hook");
                let evt_data: github_webhook::payload_types::PingEvent =
                    serde_json::from_slice(&body).with_http_status(StatusCode::BAD_REQUEST)?;

                info!(?evt_data)
            }
            "pull_request" => {
                info!("received PR hook");
                let evt_data: github_webhook::payload_types::PullRequestEvent =
                    serde_json::from_slice(&body).with_http_status(StatusCode::BAD_REQUEST)?;

                info!(?evt_data)
            }
            _ => {
                error!("balls");
            }
        }
        return Ok("foo".to_string());
    } else {
        error!("missing X-GitHub-Event header");
        Err(StatusCode::BAD_REQUEST)
    }
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
        .route("/", get(index))
        .route("/webhook", post(webhook));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
