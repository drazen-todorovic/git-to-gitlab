use std::sync::Arc;

use tokio::fs;
use tokio::task::JoinSet;
use tracing::error;

use crate::migrate::MigrationReportItem;
use crate::replace::util::replace_url;

pub async fn handler(report_file_path: &str) -> anyhow::Result<()> {
    let replace_folder_name = Arc::new(format!("rep-{}", chrono::Utc::now().timestamp_millis()));
    let file_content = fs::read_to_string(&report_file_path).await?;
    let migrated_repos = serde_yaml::from_str::<Arc<Vec<Arc<MigrationReportItem>>>>(&file_content)?;

    let mut rename_set = JoinSet::new();

    for migrated_repo in migrated_repos.iter() {
        //let migrated_repo_arc = Arc::new(migrated_repo);
        let replace_folder_name = Arc::clone(&replace_folder_name);
        let migrated_repo = Arc::clone(migrated_repo);
        let migrated_repos = Arc::clone(&migrated_repos);
        rename_set.spawn(async move {
            replace_url(migrated_repo, migrated_repos, &replace_folder_name).await
        });
    }

    while let Some(join_result) = rename_set.join_next().await {
        match join_result {
            Ok(value) => match value {
                Ok(_) => {}
                Err(e) => {
                    error!("Error while renaming repo. Error:{}", e);
                }
            },
            Err(e) => {
                error!("Error while joining the task. Error: {}", e);
            }
        }
    }
    Ok(())
}
