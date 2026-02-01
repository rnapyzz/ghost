use axum::{
    Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use axum::http::{HeaderValue, Method};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use ghost_api::{
    presentation::handlers::{
        account_items, auth, health, pl_entries, plan_nodes, scenarios, services, users,
    },
    state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    tracing::info!("Migration success");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>()?).allow_methods(
        [Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE]
    ).allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    let state = AppState::new(pool);
    let app = Router::new()
        .route("/health_check", get(health::health_check))
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login", post(auth::login))
        .route("/users/me", get(users::get_me))
        .route("/account-items", get(account_items::list))
        .route("/account-items", post(account_items::create))
        .route("/scenarios", get(scenarios::list))
        .route("/scenarios", post(scenarios::create))
        .route("/services", get(services::list))
        .route("/services", post(services::create))
        .route("/plan-nodes", get(plan_nodes::list))
        .route("/plan-nodes", post(plan_nodes::create))
        .route("/pl-entries", get(pl_entries::list))
        .route("/pl-entries", post(pl_entries::save))
        .route("/pl-entries/bulk", post(pl_entries::bulk_save))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    tracing::info!("Signal received, starting graceful shutdown");
}
