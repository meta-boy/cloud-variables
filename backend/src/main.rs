use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use cloud_variables::{
    api::{
        admin::{
            create_tier, delete_tier, delete_user, list_tiers as admin_list_tiers,
            list_users, promote_user, update_tier, update_user,
        },
        auth::{login, register},
        health::health_check,
        users::{
            change_password, create_api_key, delete_api_key, get_profile, list_api_keys,
            revoke_api_key,
        },
        variables::{
            create_variable, delete_variable, get_variable, list_variables, update_variable,
        },
    },
    db::{create_pool_from_env, DbConfig},
    middleware::{admin_middleware, auth_middleware, request_logger_middleware},
    storage::FileStorage,
};
use sqlx::{migrate::MigrateDatabase, Postgres, Pool};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    pool: Pool<Postgres>,
    storage: FileStorage,
}

impl axum::extract::FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for FileStorage {
    fn from_ref(state: &AppState) -> Self {
        state.storage.clone()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cloud_variables=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    info!("Starting Cloud Variables Backend");

    // Get database configuration
    let db_config = DbConfig::from_env();
    let database_url = &db_config.database_url;

    // Ensure database exists
    if !Postgres::database_exists(database_url).await? {
        info!("Creating database...");
        Postgres::create_database(database_url).await?;
        info!("Database created successfully");
    }

    // Create connection pool
    info!("Connecting to database...");
    let pool = create_pool_from_env().await?;
    info!("Database connection established");

    // Run migrations
    info!("Running database migrations...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => info!("Migrations completed successfully"),
        Err(e) => {
            error!("Migration error: {}", e);
            return Err(e.into());
        }
    }

    // Initialize file storage
    let storage_path = env::var("STORAGE_PATH").unwrap_or_else(|_| "./data/variables".to_string());
    let storage = FileStorage::new(&storage_path);
    storage.init().await?;
    info!("Storage initialized at: {}", storage_path);

    // Create shared app state
    let state = AppState {
        pool: pool.clone(),
        storage,
    };

    // Build public routes (no authentication required)
    let public_routes = Router::new()
        .route("/health", get(health_check))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login));

    // Build protected user routes (requires authentication)
    let protected_routes = Router::new()
        .route("/api/profile", get(get_profile))
        .route("/api/profile/password", put(change_password))
        .route("/api/variables", post(create_variable))
        .route("/api/variables", get(list_variables))
        .route("/api/variables/{id}", get(get_variable))
        .route("/api/variables/{id}", patch(update_variable))
        .route("/api/variables/{id}", delete(delete_variable))
        .route("/api/api-keys", post(create_api_key))
        .route("/api/api-keys", get(list_api_keys))
        .route("/api/api-keys/{id}/revoke", post(revoke_api_key))
        .route("/api/api-keys/{id}", delete(delete_api_key))
        .layer(middleware::from_fn(auth_middleware));

    // Build admin routes (requires admin role)
    let admin_routes = Router::new()
        .route("/admin/users", get(list_users))
        .route("/admin/users/{id}", patch(update_user))
        .route("/admin/users/{id}", delete(delete_user))
        .route("/admin/users/{id}/promote", post(promote_user))
        .route("/admin/tiers", post(create_tier))
        .route("/admin/tiers", get(admin_list_tiers))
        .route("/admin/tiers/{id}", patch(update_tier))
        .route("/admin/tiers/{id}", delete(delete_tier))
        .layer(middleware::from_fn(admin_middleware))
        .layer(middleware::from_fn(auth_middleware));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .merge(admin_routes)
        .layer(middleware::from_fn(request_logger_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    // Get server configuration
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = format!("{}:{}", host, port);

    info!("Server listening on http://{}", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // Start the server
    axum::serve(listener, app)
        .await?;

    info!("Shutting down...");

    Ok(())
}
