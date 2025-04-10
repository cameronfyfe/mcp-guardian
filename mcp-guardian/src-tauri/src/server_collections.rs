use std::path::PathBuf;

use mcp_guardian_core::server_collection::{
    claude_config::ClaudeConfig, NamedServerCollection, ServerCollection,
};

use crate::Result;

#[tauri::command]
pub async fn list_server_collections() -> Result<Vec<NamedServerCollection>> {
    mcp_guardian_core::server_collection::list_server_collections()
        .map_err(|e| format!("list_server_collections() failed: {e}"))
}

#[tauri::command]
pub async fn get_server_collection(namespace: &str, name: &str) -> Result<ServerCollection> {
    mcp_guardian_core::server_collection::load_server_collection(namespace, name)
        .map_err(|e| {
            format!("get_server_collection(namespace={namespace}, name={name}) failed: {e}")
        })?
        .ok_or_else(|| "error: server collection not found".to_owned())
}

#[tauri::command]
pub async fn set_server_collection(
    namespace: &str,
    name: &str,
    server_collection: ServerCollection,
) -> Result<()> {
    mcp_guardian_core::server_collection::save_server_collection(
        namespace,
        name,
        &server_collection,
    )
    .map_err(|e| {
        format!("set_server_collection(namespace={namespace}, name={name}, ..) failed: {e}")
    })
}

#[tauri::command]
pub async fn generate_claude_config_for_server_collection(
    namespace: &str,
    name: &str,
    proxy_path: Option<&str>,
) -> Result<ClaudeConfig> {
    mcp_guardian_core::server_collection::claude_config::generate_claude_config_for_server_collection(
        namespace,
        name,
        proxy_path.map(PathBuf::from),
    )
    .map_err(|e| {
        format!(
            "generate_claude_config_for_server_collection(namespace={namespace}, name={name}, ..) failed: {e}"
        )
    })
}

#[tauri::command]
pub async fn apply_claude_config_for_server_collection(
    namespace: &str,
    name: &str,
    proxy_path: Option<&str>,
) -> Result<()> {
    mcp_guardian_core::server_collection::claude_config::apply_claude_config_for_server_collection(
        namespace,
        name,
        proxy_path.map(PathBuf::from),
    )
    .map_err(|e| {
        format!(
            "apply_claude_config_for_server_collection(namespace={namespace}, name={name}, ..) failed: {e}"
        )
    })
}

#[tauri::command]
pub async fn delete_server_collection(namespace: &str, name: &str) -> Result<()> {
    mcp_guardian_core::server_collection::delete_server_collection(namespace, name).map_err(|e| {
        format!("delete_server_collection(namespace={namespace}, name={name}, ..) failed: {e}")
    })
}
