use aws_sdk_codecommit::Client;
use tracing::error;

use crate::migrate::{Config, Migration};

pub async fn handler(
    git_lab_url: &str,
    namespace_id: Option<&String>,
    out_file: &str,
) -> anyhow::Result<()> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let namespace_id = namespace_id.map(String::from);

    let response = client.list_repositories().send().await?;

    let mut names = Vec::new();
    for repository in response.repositories().unwrap_or(&[]) {
        let name = match repository.repository_name() {
            None => {
                error!("Repository name cannot be empty. Skipping");
                continue;
            }
            Some(value) => value,
        };
        names.push(String::from(name));
    }

    let mut config = Config {
        version: String::from("v1"),
        git_lab_url: String::from(git_lab_url),
        migrations: vec![],
    };

    for name_chunk in names.chunks(25) {
        let names = Vec::from(name_chunk);
        let resp = client
            .batch_get_repositories()
            .set_repository_names(Some(names))
            .send()
            .await?;

        for repository in resp.repositories().unwrap_or(&[]) {
            //let namespace_id = namespace_id.clone();
            let repo_source = match repository.clone_url_http() {
                None => {
                    error!("Clone url cannot be empty. Skipping");
                    continue;
                }
                Some(value) => value,
            };
            let repo_name = match repository.repository_name() {
                None => {
                    error!("Repository name cannot be empty. Skipping");
                    continue;
                }
                Some(value) => value,
            };

            let migration = Migration {
                source: String::from(repo_source),
                destination_name: String::from(repo_name),
                destination_namespace_id: namespace_id.clone(),
            };

            config.migrations.push(migration);
        }
    }

    let file = std::fs::File::create(out_file)?;
    serde_yaml::to_writer(file, &config)?;

    Ok(())
}
