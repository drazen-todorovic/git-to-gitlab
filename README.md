# Git to GitLab Migration Tool (GTOGL)

The purpose of the Git to GitLab application is to migrate any Git 
repositories to GitLab based on a configuration file. 
An additional feature application will replace all Terragrunt and 
Terraform module URLs with newly migrated ones. The application is built as a statically 
linked binary and depends on Git binary; 
to use the tool, you need to install Git.

## Installation

Since the application is a statically linked binary, 
the installation will require only 
copying and adding the executable to the PATH environment variable.

## Authentication

The application requires authentication set up in advance. 
That means all Git repositories migrating from and all 
migrating to should have credentials stored; this you can 
achieve using Git credentials helper. The application requires 
access to the GitLab API; to authenticate against GitLab API, 
you need to set the GTOGL_GITLAB_TOKEN environment variable 
with an appropriate GitLab token. The token requires access 
to the namespace with the following permissions:

 - Project creation (https://docs.gitlab.com/ee/api/projects.html#create-project)
 - Project deletion (https://docs.gitlab.com/ee/api/projects.html#delete-project)

Before running the application, export the environment variable with
the following command:
```shell
export GTOGL_GITLAB_TOKEN=<git_lab_token>
```

## Commands

The list of commands will print by executing the following command:
```shell
gtogl --help
```
The output will look like this:
```text
The program for the migration from any Git repository to GitLab

Usage: gtogl [COMMAND]

Commands:
  migrate   Execute the migration described in a config file
  replace   Replace source URLs of the Terraform and Terragrunt files based on migration report file
  delete    Delete GitLab projects
  generate  Generate tool
  schema    Print json schemas of the application documents
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

```

### Migrate Command
The description of the migrate command will print 
by executing the following command:
```shell
gtogl migrate --help
```
The output will look like this:
```text
Execute the migration described in a config file

Usage: gtogl migrate --config <file>

Options:
  -c, --config <file>  The path to the config file
  -h, --help           Print help information
```

The config argument will accept a path to the config file.

Config file example:
```yaml
version: v1
gitLabUrl: https://git_lab_endpoint
migrations:
- source: https://path_to_repo1_migrating_from
  destinationNamespaceId: '1'
  destinationName: some_project_name1
- source: https://path_to_repo2_migrating_from
  destinationNamespaceId: '1'
  destinationName: some_project_name2
```

The config file contains the following fields:

- Version contains the version of the config file (currently v1).
- GitLabUrl contains the URL of the GitLab instance to migrate.
- Migrations contain an array of migration descriptions.
  - The migration source field contains the URL to the Git repository from which you are migrating to GitLab.
  - The migration destination namespace ID contains the GitLab group ID migrating to or GitLab user ID.
  - The migration destination name contains the name of the GitLab project to migrate. The migrate command 
  will create the GitLab project via API automatically.

The migrate command will create a working directory with the 
following name pattern "mig-<unix_time>". 
Also, the migrate command will generate the "report.yaml" 
file inside the root of the migration folder.

Report file example:
```yaml
- gitLabProject:
    id: 1
    name: some_project_name1
    path_with_namespace: test1/some_project_name1
  oldUrl: https://path_to_old_repo1_migrated_from
  newUrl: https://path_to_new_repo1_migrated_to
- gitLabProject:
    id: 2
    name: some_project_name2
    path_with_namespace: test1/some_project_name2
  oldUrl: https://path_to_old_repo2_migrated_from
  newUrl: https://path_to_new_repo2_migrated_to
```

The report file is a collection of migrated 
repositories with the following fields:
- The GitLab project field will contain ID, name and name with namespace attributes of the newly created GitLab project.
- The old URL contains the URL of the old repository.
- The new URL contains the URL of the new GitLab repository.

The replace and delete commands use the 
report file as an input argument.

### Replace Command
If repositories use Terraform and Terragrunt as IOC, 
replace command will replace old source URLs with 
newly migrated ones based on the report.yaml file.

The description of the replace command will print
by executing the following command:
```shell
gtogl replace --help
```
The output will look like this:
```text
Replace source URLs of the Terraform and Terragrunt files based on migration report file

Usage: gtogl replace --report <report file path>

Options:
  -r, --report <report file path>  The path to the report file
  -h, --help                       Print help information

```

### Delete Command
The delete command deletes 
all GitLab projects based on the report.yaml file.
It is helpful in case you need to rollback and migrate again.

The description of the delete command will print
by executing the following command:
```shell
gtogl delete from-report --help
```
The output will look like this:
```text
Delete all GitLab projects from based on migration report file

Usage: gtogl delete from-report --report <report file path> --gitlab-url <GitLab URL>

Options:
  -r, --report <report file path>  The path to the report file
  -u, --gitlab-url <GitLab URL>    The URL of the GitLab
  -h, --help                       Print help information

```

### Generate Command
The generate command's purpose is to automatically create 
a config file (input config file to migrate command).
The generate command currently only supports config 
file creation from AWS Code Commit.
The application uses AWS SDK, so you can use the default 
AWS authentication toolchain to authenticate.
For example, you can do authentication to AWS via the 
AWS environment variables. For more information, please 
visit https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html

The description of the generate command will print
by executing the following command:
```shell
gtogl generate config codecommit --help
```
The output will look like this:
```text
Generate a config yaml file for the AWS Code Commit repositories

Usage: gtogl generate config codecommit --namespace-id <Namespace ID> --gitlab-url <GitLab URL> --out <Out file>

Options:
  -g, --namespace-id <Namespace ID>  The ID of the GitLab namespace
  -u, --gitlab-url <GitLab URL>      The URL of the GitLab
  -o, --out <Out file>               The path to the output file
  -h, --help                         Print help information
```

The generate command has the following arguments:
- Namespace ID, the ID of the GitLab namespace, 
  if omitted defaults to the current user’s namespace.
- GitLab URL
- Config file output path

### Schema Command
The schema command prints JSON schema for config and report 
files.
The description of the schema command will print
by executing the following command:
```shell
gtogl schema --help
```

The output will look like this:
```text
Print json schemas of the application documents

Usage: gtogl schema [COMMAND]

Commands:
  config  Print schema of the config file used as input of migrate command
  report  Print json schema of the report file used as output of migrate command
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

#### Config File Schema
Print config file JSON schema with following command:
```shell
gtogl schema config
```

The output will look like this:
```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "object",
  "properties": {
    "version": {
      "type": "string",
      "description": "The version of the config document"
    },
    "gitLabUrl": {
      "type": "string",
      "description": "The URL of the GitLab"
    },
    "migrations": {
      "type": "array",
      "description": "The array contains migration items",
      "items": [
        {
          "type": "object",
          "properties": {
            "source": {
              "type": "string",
              "description": "The url of the git repository to migrate"
            },
            "destinationNamespaceId": {
              "type": "string",
              "description": "The destination GitLab namespace ID"
            },
            "destinationName": {
              "type": "string",
              "description": "The GitLab project name to be created"
            }
          },
          "required": [
            "source",
            "destinationName"
          ]
        }
      ]
    }
  },
  "required": [
    "version",
    "gitLabUrl",
    "migrations"
  ]
}
```

#### Report File Schema
Print report file JSON schema with following command:
```shell
gtogl schema report
```

The output will look like this:
```json
{
  "$schema": "http://json-schema.org/draft-04/schema#",
  "type": "array",
  "description": "The list of migrated repositories to the GitLab",
  "items": [
    {
      "type": "object",
      "properties": {
        "gitLabProject": {
          "type": "object",
          "properties": {
            "id": {
              "type": "integer",
              "description": "The ID of the GitLab project"
            },
            "name": {
              "type": "string",
              "description": "The name of the GitLab project"
            },
            "path_with_namespace": {
              "type": "string",
              "description": "The path of the GitLab project"
            }
          },
          "required": [
            "id",
            "name",
            "path_with_namespace"
          ]
        },
        "oldUrl": {
          "type": "string",
          "description": "The old git URL of the repository"
        },
        "newUrl": {
          "type": "string",
          "description": "The new URL of the migrated GitLab repository"
        }
      },
      "required": [
        "gitLabProject",
        "oldUrl",
        "newUrl"
      ]
    }
  ]
}
```

## Important Notice

The application is only tested for HTTPS-based repositories 
on both sides, and SSH was never tested. The first iteration
of the application was tested against the GitLab CE edition 
version 15.0, deployed on-premise.