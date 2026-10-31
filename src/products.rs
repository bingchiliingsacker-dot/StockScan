use rusqlite::{Connection, OptionalExtension};
use std::path::{PathBuf};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub product_id: i64,
    pub product_name: String,
    pub price: f64,
    pub stock: i64,
}

fn database_path() -> rusqlite::Result<PathBuf> {
    let mut path = dirs::download_dir()
        .ok_or_else(|| rusqlite::Error::InvalidPath(
            "Downloads directory not found".into()
        ))?;

    path.push("stockscan.db");
    Ok(path)
}

pub fn search_product(product_id: i64) -> rusqlite::Result<Option<Product>> {
    let path = database_path()?;
    let conn = Connection::open(path)?;

    conn.query_row(
        "SELECT product_id, product_name, price, stock
         FROM products
         WHERE product_id = ?",
        [product_id],
        |row| {
            Ok(Product {
                product_id: row.get(0)?,
                product_name: row.get(1)?,
                price: row.get(2)?,
                stock: row.get(3)?,
            })
        },
    ).optional()
}

pub fn insert_product(product: &Product) -> rusqlite::Result<()> {
    let path = database_path()?;
    let conn = Connection::open(path)?;

    conn.execute(
        "INSERT INTO products (product_id, product_name, price, stock)
     VALUES (?1, ?2, ?3, ?4)",
        (
            product.product_id,
            &product.product_name,
            product.price,
            product.stock,
        ),
    )?;

    Ok(())
}

pub async fn create_product(
    Json(product): Json<Product>,
) -> Result<Json<Product>, (StatusCode, Json<Value>)> {
    match insert_product(&product) {
        Ok(()) => Ok(Json(product)),

        Err(rusqlite::Error::SqliteFailure(error, _))
        if error.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({
                    "error": "E003",
                    "message": "Product ID already exists"
                })),
                ))
            }

        Err(error) => {
            eprintln!("Database error: {error}");

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "E001",
                    "message": "Database operation failed"
                })),
            ))
        }
    }
}

pub fn get_products() -> rusqlite::Result<Vec<Product>> {
    let path = database_path()?;
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare(
        "SELECT * FROM products"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Product {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            price: row.get(2)?,
            stock: row.get(3)?,
        })
    })?;

    rows.collect()
}

pub fn get_all_products() -> rusqlite::Result<Vec<Product>> {
    let path = database_path()?;
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare(
        "SELECT * FROM products"
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Product {
            product_id: row.get(0)?,
            product_name: row.get(1)?,
            price: row.get(2)?,
            stock: row.get(3)?,
        })
    })?;

    rows.collect()
}

pub async fn list_products() -> Result<Json<Vec<Product>>, (StatusCode, Json<Value>)> {
    match get_all_products() {
        Ok(products) => Ok(Json(products)),
        Err(error) => {
            eprintln!("Database error: {error}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "E001",
                "message": "Database operation failed"
            }))))
        }
    }
}

pub fn update_product(
    id: i64,
    product: &Product
) -> rusqlite::Result<usize> {
    let path = database_path()?;

    let conn = Connection::open(path)?;

    conn.execute(
        "UPDATE products SET product_name = ?1, price = ?2, stock = ?3 WHERE product_id = ?4",
        (&product.product_name, product.price, product.stock, id),
    )
}

pub async fn update_product_handler(
    Path(id): Path<i64>,
    Json(product): Json<Product>,
) -> Result<Json<Product>, (StatusCode, Json<Value>)> {
    match update_product(id, &product) {
        Ok(1) => Ok(Json(product)),
        Ok(0) => {
            eprintln!("Product not found");

            Err((StatusCode::NOT_FOUND, Json(json!({
                "error": "E002",
                "message": "Product not found"
            }))))
        }
        Ok(_) => {
            eprintln!("Unexpected number of rows updated.");

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "E003",
                "message": "Unexpected number of rows updated."
            }))))
        },
        Err(error) => {
            eprintln!("Database error: {error}");

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "E001",
                "message": "Database operation failed"
            }))))
        }
    }
}

pub fn delete_product(
    id: i64,
) -> rusqlite::Result<usize> {
    let path = database_path()?;

    let conn = Connection::open(path)?;

    conn.execute("DELETE FROM products WHERE product_id = ?1", [id])
}

pub async fn delete_product_handler(
    Path(id): Path<i64>
) -> Result<StatusCode, (StatusCode, Json<Value>)> {
    match delete_product(id) {
        Ok(1) => Ok(StatusCode::NO_CONTENT),
        Ok(0) => {
            eprintln!("Product not found");

            Err((StatusCode::NOT_FOUND, Json(json!({
                "error": "E002",
                "message": "Product not found"
            }))))
        }
        Ok(_) => {
            eprintln!("Unexpected number of rows deleted.");

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "E003",
                "message": "Unexpected number of rows deleted."
            }))))
        }
        Err(error) => {
            eprintln!("Database error: {error}");

            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "error": "E001",
                "message": "Database operation failed"
            }))))
        }
    }
}
