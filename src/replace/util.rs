use std::process::Command;
use std::sync::Arc;

use tokio::fs;
use tracing::{error, info, warn};
use walkdir::WalkDir;

use crate::migrate::MigrationReportItem;

pub async fn replace_url(
    migrated_repo: Arc<MigrationReportItem>,
    migrated_repos: Arc<Vec<Arc<MigrationReportItem>>>,
    replace_folder_path: &str,
) -> anyhow::Result<()> {
    let tt_changes_path = format!(
        "{}/{}-tt-changes",
        replace_folder_path, migrated_repo.git_lab_project.name
    );
    info!(
        "Cloning repo {} for terraform and terragrunt changes",
        migrated_repo.new_url
    );

    let clone_command = Command::new("git")
        .args(["clone", &migrated_repo.new_url, &tt_changes_path])
        .output();

    if let Err(err) = clone_command {
        error!(
            "Error occurred while cloning new repository {}. Error: {}",
            migrated_repo.git_lab_project.path_with_namespace, err
        );
        return Err(anyhow::Error::new(err));
    }

    let tt_files_iter = WalkDir::new(&tt_changes_path)
        .into_iter()
        .filter_map(|e| e.ok());

    info!("Listing change candidate files");
    for entry in tt_files_iter {
        let path = match entry.path().to_str() {
            None => {
                warn!("Cannot extract path from DirEntry: {:?}", entry);
                continue;
            }
            Some(value) => value,
        };

        // ignore terraform lock files and other files (not .hcl and .tf files)
        if path.ends_with(".terraform.lock.hcl")
            || !(path.ends_with(".hcl") || path.ends_with(".tf"))
        {
            continue;
        }

        info!("Trying to update file: {}", path);
        let mut file_content = fs::read_to_string(&path).await?;
        let file_create_path = format!("{}.tmp", path);

        for repo in migrated_repos.iter() {
            file_content = file_content.replace(
                format!("git::{}", repo.old_url).as_str(),
                format!("git::{}", repo.new_url).as_str(),
            );
        }

        fs::write(&file_create_path, file_content).await?;
        fs::remove_file(&path).await?;
        fs::rename(&file_create_path, &path).await?;
    }
    // add files to the git
    let add_command = Command::new("git")
        .current_dir(&tt_changes_path)
        .args(["add", "-A"])
        .output();

    if let Err(err) = add_command {
        error!(
            "Error occurred while adding changed files, repository {}. Error: {}",
            migrated_repo.git_lab_project.path_with_namespace, err
        );
        return Err(anyhow::Error::new(err));
    }

    // create commit
    let commit_command = Command::new("git")
        .current_dir(&tt_changes_path)
        .args(["commit", "-m", "Migration changes"])
        .output();

    if let Err(err) = commit_command {
        error!(
            "Error occurred while committing changed files, repository {}. Error: {}",
            migrated_repo.git_lab_project.path_with_namespace, err
        );
        return Err(anyhow::Error::new(err));
    }

    // push to the remote
    info!(
        "Pushing migration changes to the repository: {}",
        migrated_repo.new_url
    );
    let push_command = Command::new("git")
        .current_dir(&tt_changes_path)
        .args(["push"])
        .output();

    if let Err(err) = push_command {
        error!(
            "Error occurred while pushing changed files, repository {}. Error: {}",
            migrated_repo.git_lab_project.path_with_namespace, err
        );
        return Err(anyhow::Error::new(err));
    }

    Ok(())
}
