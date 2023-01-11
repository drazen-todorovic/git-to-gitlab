use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub version: String,
    pub git_lab_url: String,
    pub migrations: Vec<Migration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Migration {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_namespace_id: Option<String>,
    pub destination_name: String,
}

#[derive(Clone)]
pub struct MigratedRepository {
    pub migration: Migration,
    pub destination_url: String,
    pub rename_url: String,
    pub repo_path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationReportItem {
    pub git_lab_project: Project,
    pub old_url: String,
    pub new_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: u32,
    pub name: String,
    pub path_with_namespace: String,
}
