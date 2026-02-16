use crate::core::{constant::DESKTOP_ID, entity::FileNode};
use dioxus::prelude::*;

#[server]
pub async fn get_desktop_files() -> Result<Vec<FileNode>, ServerFnError> {
    use crate::core::file_system::FileSystem;
    use dioxus::fullstack::FullstackContext;
    use dioxus::server::axum::Extension;

    let fs: Extension<FileSystem> = FullstackContext::extract().await?;
    let files = fs.get_children(DESKTOP_ID);

    Ok(files)
}
