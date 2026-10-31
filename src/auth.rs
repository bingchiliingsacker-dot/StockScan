use std::path::PathBuf;

use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
    Argon2,
};
use axum::{http::StatusCode, Json};
use chrono::prelude::*;
use rusqlite::{Connection, Error as SqliteError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserAuth {
    pub username: String,
    pub password: String,
}

pub enum CreateAccountError {
    UsernameTaken,
    InvalidInput,
    DatabaseError,
    HashingError,
}

pub enum VerifyLoginError {
    InvalidCredentials,
    InvalidInput,
    DatabaseError,
    HashingError,
}

fn database_path() -> Result<PathBuf, CreateAccountError> {
    let mut path = dirs::download_dir()
        .ok_or(CreateAccountError::DatabaseError)?;

    path.push("stockscan_login.db");
    Ok(path)
}

fn initialize_table(conn: &Connection) -> Result<(), CreateAccountError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stockscan_login (
            user_id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        [],
    )
        .map_err(|_| CreateAccountError::DatabaseError)?;

    Ok(())
}

pub fn create_account(
    user_auth: &UserAuth,
) -> Result<(), CreateAccountError> {
    if user_auth.username.trim().is_empty()
        || user_auth.password.is_empty()
    {
        return Err(CreateAccountError::InvalidInput);
    }

    let utc = Utc::now();
    let password_hash = Argon2::default()
        .hash_password(user_auth.password.as_bytes())
        .map_err(|_| CreateAccountError::HashingError)?
        .to_string();

    let conn = Connection::open(database_path()?)
        .map_err(|_| CreateAccountError::DatabaseError)?;

    initialize_table(&conn)?;

    conn.execute(
        "INSERT INTO stockscan_login (username, password, created_at)
         VALUES (?1, ?2, ?3)",
        (
            &user_auth.username,
            password_hash,
            utc.timestamp(),
        ),
    )
        .map_err(|error| match error {
            SqliteError::SqliteFailure(ref code, _)
            if code.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    CreateAccountError::UsernameTaken
                }
            _ => CreateAccountError::DatabaseError,
        })?;

    Ok(())
}

pub fn verify_login(
    user_auth: &UserAuth,
) -> Result<(), VerifyLoginError> {
    let conn = Connection::open(
        dirs::download_dir()
            .ok_or_else(|| std::io::Error::other("Download directory unavailable"))
            .map_err(|_| VerifyLoginError::DatabaseError)?
            .join("stockscan_login.db"),
    ).map_err(|_| VerifyLoginError::DatabaseError)?;

    initialize_table(&conn)
        .map_err(|_| VerifyLoginError::DatabaseError)?;

    let stored_hash: String = conn.query_row(
        "SELECT password FROM stockscan_login WHERE username = ?1",
        [&user_auth.username],
        |row| row.get(0),
    ).map_err(|error| match error {
        rusqlite::Error::QueryReturnedNoRows => {
            VerifyLoginError::InvalidCredentials
        }
        _ => VerifyLoginError::DatabaseError,
    })?;

    let parsed_hash = PasswordHash::new(&stored_hash)
        .map_err(|_| VerifyLoginError::HashingError)?;

    Argon2::default().verify_password(
        user_auth.password.as_bytes(),
        &parsed_hash,
    ).map_err(|_| VerifyLoginError::InvalidCredentials)?;

    Ok(())
}

// Handles create_product for axum to safely connect to HTML
pub async fn handle_create_account(
    Json(user_auth): Json<UserAuth>
) -> StatusCode {
    match create_account(&user_auth) {
        // 201: Account Created
        Ok(()) => StatusCode::CREATED,

        // Conflict in the database
        Err(CreateAccountError::UsernameTaken) => StatusCode::CONFLICT,

        // Bad Request
        Err(CreateAccountError::InvalidInput) => StatusCode::BAD_REQUEST,

        // Internal Server Error/s
        Err(CreateAccountError::DatabaseError)
        | Err(CreateAccountError::HashingError) => {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn handle_verify_login(
    Json(user_auth): Json<UserAuth>
) -> StatusCode {
    match verify_login(&user_auth) {
        // 200: Ok
        Ok(()) => StatusCode::OK,

        // Invalid credentials: Password input is incorrect
        Err(VerifyLoginError::InvalidCredentials) => StatusCode::UNAUTHORIZED,

        // Bad Request
        Err(VerifyLoginError::InvalidInput) => StatusCode::BAD_REQUEST,

        // Internal Server Error/s
        Err(VerifyLoginError::DatabaseError)
        | Err(VerifyLoginError::HashingError) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
