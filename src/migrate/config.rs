use std::env;

use tokio::fs;
use tracing::info;

use crate::migrate::Config;

pub async fn read_config(config_path: &str) -> anyhow::Result<(Config, String, String)> {
    // read from file
    info!("Reading config from file {}", config_path);
    let contents = fs::read_to_string(config_path).await?;

    // parse config
    let mut config = if contents.starts_with('{') {
        info!("Trying to read json configuration");
        serde_json::from_str::<Config>(&contents)?
    } else {
        info!("Trying to read yaml configuration");
        serde_yaml::from_str::<Config>(&contents)?
    };

    if config.git_lab_url.ends_with('/') {
        info!("Removing back slash from the GitLab url");
        let last_char_index = config.git_lab_url.len() - 1;
        config.git_lab_url = config.git_lab_url[..last_char_index].to_string();
        info!("GitLab url changed to: {}", config.git_lab_url);
    }

    let migration_folder_name = format!("mig-{}", chrono::Utc::now().timestamp_millis());
    let token =
        env::var("GTOGL_GITLAB_TOKEN").expect("GTOGL_GITLAB_TOKEN environment variable missing");

    Ok((config, migration_folder_name, token))
}
