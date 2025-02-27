use axum::extract::{self, Query};
use axum::{response, routing, Router};
use serde_json::{json, Value};
use std::collections::HashMap;
// use tower_sessions::{self as tsession, session, session_store};
use tower_sessions::ExpiredDeletion;
use tower_sessions_sqlx_store as tsqlstore;

mod users;

const COUNTER_KEY: &str = "counter";

#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Counter(usize);

async fn get_session_store() -> tsqlstore::SqliteStore {
    let pool = tsqlstore::sqlx::SqlitePool::connect("sqlite::memory:")
        .await
        .unwrap();

    let session_store = tsqlstore::SqliteStore::new(pool);
    session_store.migrate().await.unwrap();

    session_store
}

async fn get_app(session_store: tsqlstore::SqliteStore) -> Router {
    let session_layer = tower_sessions::SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_expiry(tower_sessions::Expiry::OnInactivity(
            time::Duration::seconds(60),
        ));

    let app = Router::new()
        .route("/", routing::get(home))
        .route("/user", routing::get(user_page))
        .route("/session", routing::get(handler))
        .layer(session_layer);

    app
}

async fn get_listener() -> tokio::net::TcpListener {
    let socket_addr = std::net::SocketAddr::from(([127, 0, 0, 1], 5000));
    let listener = tokio::net::TcpListener::bind(&socket_addr).await.unwrap();
    println!("running server on localhost:5000");
    listener
}

async fn shutdown_signal(deletion_task_abort_handle: tokio::task::AbortHandle) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install ctrl+c handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort()},
        _ = terminate => {deletion_task_abort_handle.abort()}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session_store = get_session_store().await;
    let session_deletion_time = tokio::time::Duration::from_secs(5 * 60);

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(session_deletion_time),
    );

    let app = get_app(session_store.clone()).await;
    let listener = get_listener().await;
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    deletion_task.await??;

    Ok(())
}

async fn home() -> response::Json<Value> {
    response::Json(json!({"message": "welcome"}))
}

// /user?uid=
async fn user_page(
    extract::Query(params): Query<HashMap<String, String>>,
) -> response::Json<Value> {
    if !params.contains_key("uid") {
        return response::Json(json!({"message": "missing uid"}));
    }

    let uid = params.get("uid").unwrap();

    response::Json(json!({"message": format!("uid is {uid}")}))
}

// /session
async fn handler(session: tower_sessions::Session) -> response::Json<Value> {
    let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();

    session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();

    response::Json(json!({"message": format!("Current count: {}", counter.0)}))
}
