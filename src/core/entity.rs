use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub kind: String,
    pub path: String,
    pub extension: Option<String>,
    pub content_hash: Option<String>,
    pub children: Vec<String>,
    pub x: i32,
    pub y: i32,
}
