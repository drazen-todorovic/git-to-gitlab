use std::env;
use std::sync::Arc;

use reqwest::header::HeaderValue;
use reqwest::Client;
use tokio::task::JoinSet;
use tracing::{error, info};

use crate::migrate::{MigrationReportItem, Project};

pub async fn handler(file_path: &str, git_lab_url: &str) -> anyhow::Result<()> {
    let token = Arc::new(
        env::var("GTOGL_GITLAB_TOKEN").expect("GTOGL_GITLAB_TOKEN environment variable missing"),
    );

    let mut git_lab_url_refined = String::from(git_lab_url);
    if git_lab_url_refined.ends_with('/') {
        let last_char_index = git_lab_url_refined.len() - 1;
        git_lab_url_refined = git_lab_url_refined[..last_char_index].to_string();
    }

    let file_content = tokio::fs::read_to_string(&file_path).await?;
    let migration_items = serde_yaml::from_str::<Vec<MigrationReportItem>>(&file_content)?;
    let projects = migration_items.into_iter().map(|x| x.git_lab_project);

    let http_client = Arc::new(Client::new());

    delete_projects(projects, http_client, token, &git_lab_url_refined).await?;

    Ok(())
}

pub async fn delete_projects(
    projects: impl Iterator<Item = Project>,
    http_client: Arc<Client>,
    token: Arc<String>,
    git_lab_url: &str,
) -> anyhow::Result<()> {
    let mut delete_project_set = JoinSet::new();

    for project in projects {
        let delete_project_url = format!("{}/api/v4/projects/{}/", git_lab_url, project.id);
        let http_client = Arc::clone(&http_client);
        let token = Arc::clone(&token);
        delete_project_set.spawn(async move {
            delete_project(project, delete_project_url, http_client, token).await
        });
    }

    while delete_project_set.join_next().await.is_some() {}
    Ok(())
}

async fn delete_project(
    project: Project,
    delete_project_url: String,
    http_client: Arc<Client>,
    token: Arc<String>,
) -> anyhow::Result<()> {
    info!(
        "Deleting project from GitLab with the following path: {}",
        project.path_with_namespace
    );
    let response = http_client
        .delete(delete_project_url)
        .header("PRIVATE-TOKEN", HeaderValue::from_str(&token)?)
        .send()
        .await?;

    if response.status().is_server_error() {
        let data = response.text().await?;
        error!(
            "Error deleting project with the following path: {}, message: {}",
            project.path_with_namespace, data
        )
    }

    Ok(())
}
