use std::collections::HashMap;
use std::process::Command;
use std::sync::Arc;

use reqwest::header::HeaderValue;
use reqwest::Client;
use tracing::{error, info};

use crate::migrate::model::MigrationReportItem;
use crate::migrate::{Migration, Project};

pub async fn migrate(
    migration: Migration,
    migration_folder_name: Arc<String>,
    git_lab_url: Arc<String>,
    git_lab_token: Arc<String>,
    http_client: Arc<Client>,
) -> anyhow::Result<MigrationReportItem> {
    info!("Cloning repository {}", migration.source);
    let repo_path = format!("{}/{}", migration_folder_name, migration.destination_name);
    let clone_command = Command::new("git")
        .args(["clone", "--bare", &migration.source, &repo_path])
        .output();

    if let Err(err) = clone_command {
        error!(
            "Error occurred while cloning old repository {}. Error: {}",
            migration.destination_name, err
        );
        return Err(anyhow::Error::new(err));
    }

    // create the GitLab project
    let create_project_url = format!("{}/api/v4/projects", git_lab_url);
    let mut payload = HashMap::<&str, &str>::new();
    payload.insert("name", &migration.destination_name);
    if let Some(namespace_id) = &migration.destination_namespace_id {
        payload.insert("namespace_id", namespace_id);
    }
    payload.insert("initialize_with_readme", "false");

    info!("Creating project: {}", &migration.destination_name);
    let response = http_client
        .post(&create_project_url)
        .header("PRIVATE-TOKEN", HeaderValue::from_str(&git_lab_token)?)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_server_error() {
        error!(
            "Error while contacting GitLab server. Error code: {}",
            response.status().as_u16()
        );
    }

    let project = response.json::<Project>().await?;

    let destination_repo_url = format!("{}/{}", git_lab_url, &project.path_with_namespace);

    // push the repo
    info!("Pushing to the new repo: {}", destination_repo_url);
    let push_command = Command::new("git")
        .args(["push", "--mirror", &destination_repo_url])
        .current_dir(&repo_path)
        .output();

    if let Err(err) = push_command {
        error!(
            "Error occurred while pushing mirror to the new repository {}. Error: {}",
            migration.destination_name, err
        );
        return Err(anyhow::Error::new(err));
    }

    Ok(MigrationReportItem {
        git_lab_project: project,
        old_url: migration.source,
        new_url: destination_repo_url,
    })
}
