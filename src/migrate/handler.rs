use reqwest::Client;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::JoinSet;
use tracing::error;

use crate::migrate::config;
use crate::migrate::util::migrate;

pub async fn handler(config_path: &str) -> anyhow::Result<()> {
    let (mut config, migration_folder_name, token) = config::read_config(config_path).await?;

    let migration_folder_name = Arc::new(migration_folder_name);
    let git_lab_url = Arc::new(config.git_lab_url);
    let token = Arc::new(token);
    let report_file_name = format!("{}/{}", migration_folder_name, "report.yaml");

    let http_client = Arc::new(Client::new());

    let mut report_items = Vec::new();
    let mut migration_set = JoinSet::new();

    while let Some(migration) = config.migrations.pop() {
        let http_client = Arc::clone(&http_client);
        let migration_folder_name = Arc::clone(&migration_folder_name);
        let git_lab_url = Arc::clone(&git_lab_url);
        let git_lab_token = Arc::clone(&token);

        migration_set.spawn(async move {
            migrate(
                migration,
                migration_folder_name,
                git_lab_url,
                git_lab_token,
                http_client,
            )
            .await
        });
    }

    while let Some(join_result) = migration_set.join_next().await {
        match join_result {
            Ok(value) => match value {
                Ok(report_item) => {
                    report_items.push(report_item);

                    // write to the report.yaml
                    let report_yaml = serde_yaml::to_string(&report_items)?;
                    File::create(&report_file_name)
                        .await?
                        .write_all(report_yaml.as_bytes())
                        .await?;
                }
                Err(e) => {
                    error!("Error while migrating repo. Error: {}", e);
                }
            },
            Err(e) => {
                error!("Error while joining the task. Error: {}", e);
            }
        }
    }

    Ok(())
}
