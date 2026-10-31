pub mod database;
pub mod routes;
pub mod products;
pub mod auth;

use axum::{Router, routing::get};
use rusqlite::Connection;
use tokio::net::TcpListener;

use axum::http::{
    header::CONTENT_TYPE,
    HeaderValue,
    Method,
};
use axum::routing::post;
use tower_http::cors::CorsLayer;


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:63342"),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE]);

    let mut path = dirs::download_dir()
        .expect("Could not find download directory");

    path.push("stockscan.db");

    let conn = Connection::open(path).expect("Could not open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            product_id INTEGER PRIMARY KEY,
            product_name TEXT NOT NULL,
            price INTEGER NOT NULL,
            stock INTEGER NOT NULL
        )",
        []
    ).expect("Could not create database");

    database::create_database()
        .expect("Failed to initialize StockScan database");
    let app = Router::new()
        // Products side
        .route(
            "/api/products/{product_id}",
            get(routes::get_product)
                .put(products::update_product_handler)
                .delete(products::delete_product_handler),
        )
        .route(
            "/api/products",
            get(products::list_products)
                .post(products::create_product),
        )
        
        // Authentication side
        .route("/api/auth/register", post(auth::handle_create_account))
        .route("/api/auth/login", post(auth::handle_verify_login))
        .layer(cors);
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}