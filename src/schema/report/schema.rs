pub const REPORT_RAW_SCHEMA: &str = r#"{
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
}"#;
