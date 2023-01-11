use clap::{command, Arg, Command};
use tracing::info;

use crate::{generate, replace, schema};

use super::delete;
use super::migrate;

pub async fn run() -> anyhow::Result<()> {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand(
            Command::new("migrate")
                .about("Execute the migration described in a config file")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("config")
                        .long("config")
                        .short('c')
                        .value_name("file")
                        .help("The path to the config file")
                        .required(true),
                )
        )
        .subcommand(
            Command::new("replace")
                .about("Replace source URLs of the Terraform and Terragrunt files based on migration report file")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("report")
                        .long("report")
                        .short('r')
                        .value_name("report file path")
                        .help("The path to the report file")
                        .required(true)
                )
        )
        .subcommand(
            Command::new("delete")
                .about("Delete GitLab projects")
                .arg_required_else_help(true)
                .subcommand(
                Command::new("from-report")
                    .about("Delete all GitLab projects from based on migration report file")
                    .arg_required_else_help(true)
                    .arg(
                        Arg::new("report")
                            .long("report")
                            .short('r')
                            .value_name("report file path")
                            .help("The path to the report file")
                            .required(true)

                    )
                    .arg(
                    Arg::new("gitlab-url")
                        .long("gitlab-url")
                        .short('u')
                        .value_name("GitLab URL")
                        .help("The URL of the GitLab")
                        .required(true)
                    )
            ),
        )
        .subcommand(
            Command::new("generate")
                .about("Generate tool")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("config")
                        .about("Generate config file")
                        .arg_required_else_help(true)
                        .subcommand(
                            Command::new("codecommit")
                                .about("Generate a config yaml file for the AWS Code Commit repositories")
                                .arg_required_else_help(true)
                                .arg(
                                    Arg::new("namespace-id")
                                        .long("namespace-id")
                                        .short('g')
                                        .value_name("Namespace ID")
                                        .help("The ID of the GitLab namespace"),
                                )
                                .arg(
                                    Arg::new("gitlab-url")
                                        .long("gitlab-url")
                                        .short('u')
                                        .value_name("GitLab URL")
                                        .help("The URL of the GitLab")
                                        .required(true),
                                )
                                .arg(
                                    Arg::new("out")
                                        .long("out")
                                        .short('o')
                                        .value_name("Out file")
                                        .help("The path to the output file")
                                        .required(true),
                                ),
                        ),
                ),
        )
        .subcommand(Command::new("schema")
            .about("Print json schemas of the application documents")
            .arg_required_else_help(true)
            .subcommand(Command::new("config")
                .about("Print schema of the config file used as input of migrate command"))
            .subcommand(Command::new("report")
                .about("Print json schema of the report file used as output of migrate command"))
        )

        .get_matches();

    if let Some(("migrate", submatches)) = matches.subcommand() {
        info!("Migration started");

        // read config file path
        let config_file_path = submatches
            .get_one::<String>("config")
            .expect("Config file path should not be empty");

        migrate::handler(config_file_path).await?;
    }

    if let Some(("replace", submatches)) = matches.subcommand() {
        info!("Replacement started");

        // read report file path
        let report = submatches
            .get_one::<String>("report")
            .expect("Report file path should not be empty");

        replace::handler(report).await?;
    }

    if let Some(("delete", submatches)) = matches.subcommand() {
        if let Some(("from-report", submatches)) = submatches.subcommand() {
            info!("Deletion started");

            // read file path
            let report = submatches
                .get_one::<String>("report")
                .expect("Report file path should not be empty");

            //read GitLab url
            let gitlab_url = submatches
                .get_one::<String>("gitlab-url")
                .expect("GitLab URL should not be empty");

            delete::from_report::handler(report, gitlab_url).await?;
        }
    }

    if let Some(("generate", submatches)) = matches.subcommand() {
        if let Some(("config", submatches)) = submatches.subcommand() {
            if let Some(("codecommit", submatches)) = submatches.subcommand() {
                info!("Config generation started");

                // read group id
                let namespace_id = submatches.get_one::<String>("namespace-id");

                //read GitLab url
                let gitlab_url = submatches
                    .get_one::<String>("gitlab-url")
                    .expect("GitLab URL should not be empty");

                //read output file
                let out_file = submatches
                    .get_one::<String>("out")
                    .expect("Output file should not be empty");

                generate::config::codecommit::handler(gitlab_url, namespace_id, out_file).await?;
            }
        }
    }

    if let Some(("schema", submatches)) = matches.subcommand() {
        if let Some(("config", _submatches)) = submatches.subcommand() {
            schema::config::handle()?;
        }
        if let Some(("report", _submatches)) = submatches.subcommand() {
            schema::report::handle()?;
        }
    }

    Ok(())
}
