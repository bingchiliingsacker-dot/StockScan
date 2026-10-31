use rusqlite::{Connection, Result};
use dirs;

pub fn create_database() -> Result<()> {
    let mut path = dirs::download_dir()
        .expect("Could not find download directory");

    path.push("stockscan.db");

    let conn = Connection::open(path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            product_id INTEGER PRIMARY KEY,
            product_name TEXT NOT NULL,
            price INTEGER NOT NULL,
            stock INTEGER NOT NULL
        )",
        []
    )?;

    Ok(())
}
