pub static CONFIG_RAW_SCHEMA: &str = r#"{
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
}"#;
