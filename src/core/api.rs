use crate::core::{constant::DESKTOP_ID, entity::FileNode};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use {
    crate::core::file_system::FileSystem,
    dioxus::server::axum::Extension,
};

#[server]
pub async fn get_desktop_files() -> Result<Vec<FileNode>, ServerFnError> {
    let fs: Extension<FileSystem> = FullstackContext::extract().await?;
    let files = fs.get_children(DESKTOP_ID);
    Ok(files)
}

#[server]
pub async fn move_file(id: String, x: i32, y: i32) -> Result<(), ServerFnError> {
    let fs: Extension<FileSystem> = FullstackContext::extract().await?;
    fs.update_position(&id, x, y).map_err(ServerFnError::new)?;
    Ok(())
}

#[server]
pub async fn create_new_folder(name: String, x: i32, y: i32) -> Result<FileNode, ServerFnError> {
    let mut fs: Extension<FileSystem> = FullstackContext::extract().await?;
    let node = fs.create_folder(DESKTOP_ID, &name, x, y).map_err(ServerFnError::new)?;
    Ok(node)
}
