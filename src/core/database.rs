use dioxus::logger::tracing;
use r2d2::Pool;
use r2d2_sqlite::{rusqlite::params, SqliteConnectionManager};
use std::{fs, path::Path, sync::LazyLock};

use crate::core::constant::DESKTOP_ID;

pub type DbPool = Pool<SqliteConnectionManager>;

pub static DATABASE: LazyLock<DbPool> = LazyLock::new(|| {
    let db_path = "database/sqlite.db";

    if let Some(parent) = Path::new(db_path).parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).expect("Failed to create database directory");
            tracing::info!("📁 Created database directory");
        }
    }

    let manager = SqliteConnectionManager::file(db_path)
        .with_init(|c| c.execute_batch("PRAGMA journal_mode=WAL;"));

    Pool::new(manager).expect("Failed to create pool")
});

pub fn init() {
    let connection = DATABASE.get().unwrap();

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS file_system (
            id TEXT PRIMARY KEY,
            parent_id TEXT,
            name TEXT NOT NULL,
            kind TEXT NOT NULL,
            path TEXT UNIQUE NOT NULL,
            extension TEXT,
            content_hash TEXT,
            x INTEGER DEFAULT 0,
            y INTEGER DEFAULT 0
        )",
            [],
        )
        .unwrap();

    let exists = connection
        .query_row(
            "SELECT 1 FROM file_system WHERE path = ?1",
            params!["/desktop"],
            |_| Ok(()),
        )
        .is_ok();

    if !exists {
        connection
            .execute(
                "INSERT INTO file_system (id, name, kind, path) VALUES (?1, ?2, ?3, ?4)",
                params![DESKTOP_ID, "Desktop", "folder", "/desktop"],
            )
            .expect("Failed to create Desktop folder");
        tracing::info!("📁 Created default 'Desktop' folder");
    }

    tracing::info!("✅ Database initialized");
}
