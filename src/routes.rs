use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};

use serde_json::{json, Value};

use crate::products::{search_product, Product};

pub async fn get_product(Path(product_id): Path<i64>) -> Result<Json<Product>, (StatusCode, Json<Value>)> {
    let product = search_product(product_id);

    match product {
        Ok(Some(product)) => {
            Ok(Json(product))
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({
                "error": "E002",
                "message": "Product not found"
                }))
            ))
        }
        Err(error) => {
            eprintln!("Database error: {error}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": "E001",
                "message": "Database operation failed"
                }))
            ))
        }
    }
}
