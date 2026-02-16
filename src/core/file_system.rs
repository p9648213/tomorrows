use super::database::DATABASE;
use crate::core::entity::FileNode;
use dashmap::DashMap;
use dioxus::logger::tracing;
use r2d2_sqlite::rusqlite::params;
use std::sync::Arc;

#[derive(Clone)]
pub struct FileSystem {
    pub nodes: Arc<DashMap<String, FileNode>>,
}

impl FileSystem {
    pub fn new() -> Self {
        let fs = Self {
            nodes: Arc::new(DashMap::new()),
        };
        fs.hydrate();
        tracing::info!("✅ File system loaded into memory");
        fs
    }

    pub fn hydrate(&self) {
        let connection = DATABASE.get().unwrap();
        let mut statement = connection
            .prepare(
                "SELECT id, parent_id, name, kind, path, extension, content_hash, x, y FROM file_system",
            )
            .unwrap();

        let file_nodes = statement
            .query_map([], |row| {
                Ok(FileNode {
                    id: row.get(0)?,
                    parent_id: row.get(1)?,
                    name: row.get(2)?,
                    kind: row.get(3)?,
                    path: row.get(4)?,
                    extension: row.get(4)?,
                    content_hash: row.get(5)?,
                    x: row.get(7)?,
                    y: row.get(8)?,
                    children: Vec::new(),
                })
            })
            .unwrap();

        for node in file_nodes {
            let node = node.unwrap();
            self.nodes.insert(node.id.clone(), node);
        }

        let all_nodes: Vec<(String, Option<String>)> = self
            .nodes
            .iter()
            .map(|r| (r.key().clone(), r.value().parent_id.clone()))
            .collect();

        for (child_id, parent_id_opt) in all_nodes {
            if let Some(parent_id) = parent_id_opt {
                if let Some(mut parent) = self.nodes.get_mut(&parent_id) {
                    parent.children.push(child_id);
                }
            }
        }
    }

    pub fn get_children(&self, parent_id: &str) -> Vec<FileNode> {
        if let Some(parent) = self.nodes.get(parent_id) {
            parent
                .children
                .iter()
                .filter_map(|child_id| self.nodes.get(child_id).map(|n| n.value().clone()))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn create_folder(
        &mut self,
        parent_id: &str,
        base_name: &str,
        x: i32,
        y: i32,
    ) -> Result<FileNode, String> {
        let parent_path = self
            .nodes
            .get(parent_id)
            .map(|n| n.path.clone())
            .ok_or_else(|| "Parent folder not found".to_string())?;

        let connection = DATABASE.get().unwrap();

        let mut final_name = base_name.to_string();
        let mut final_path = format!("{}/{}", parent_path, final_name);
        let mut counter = 1;

        loop {
            let exists: bool = connection
                .query_row(
                    "SELECT 1 FROM file_system WHERE path = ?1",
                    params![final_path],
                    |_| Ok(true),
                )
                .unwrap_or(false);

            if !exists {
                break;
            }

            counter += 1;
            final_name = format!("{} ({})", base_name, counter);
            final_path = format!("{}/{}", parent_path, final_name);
        }

        let id = uuid::Uuid::new_v4().to_string();

        connection
                .execute(
                    "INSERT INTO file_system (id, parent_id, name, kind, path, x, y) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![id, parent_id, final_name, "folder", final_path, x, y],
                )
                .map_err(|e| format!("Database error: {}", e))?;

        let file_node = FileNode {
            id: id.clone(),
            parent_id: Some(parent_id.to_string()),
            name: final_name, // Use the resolved name
            kind: "folder".to_string(),
            path: final_path,
            content_hash: None,
            extension: None,
            x,
            y,
            children: vec![],
        };

        self.nodes.insert(id.clone(), file_node.clone());

        if let Some(mut parent) = self.nodes.get_mut(parent_id) {
            parent.children.push(id);
        }

        Ok(file_node)
    }

    pub fn update_position(&self, id: &str, x: i32, y: i32) -> Result<(), String> {
        let connection = DATABASE.get().unwrap();

        connection
            .execute(
                "UPDATE file_system SET x = ?1, y = ?2 WHERE id = ?3",
                params![x, y, id],
            )
            .map_err(|e| e.to_string())?;

        if let Some(mut node) = self.nodes.get_mut(id) {
            node.x = x;
            node.y = y;
        }

        Ok(())
    }
}
