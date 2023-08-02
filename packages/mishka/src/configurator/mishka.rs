use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct MishkaConfig {
    pub project: ProjectConfig,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct ProjectConfig {
    pub version: u16,
    pub files: Vec<String>,
    pub field: Vec<FieldConfig>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct FieldConfig {
    pub name: String,
    pub kind: String,
    pub prompt: Option<String>,
    pub variants: Option<Vec<String>>,
    pub default_value: Option<toml::Value>,
    pub template: Option<Vec<TemplateConfig>>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TemplateConfig {
    pub value: toml::Value,
    pub files: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml::Value;
    static PROJECT_CONFIG: &str = r#"
# [PROJECT SETUP]
[project]
version = 1
files = ["required_folder/*"]
# [field1]
[[project.field]]
name = "field1"
kind = "string"
prompt = "What field1 would be?"
variants = ["a", "b"]
  [[project.field.template]]
    value = "a"
    files = ["another_folder/file"]
# [field2]
[[project.field]]
name = "field2"
kind = "bool"
default_value = true
  [[project.field.template]]
    value = false
    files = ["!required_folder/disabled_file"]
"#;

    #[test]
    fn project_config() {
        let expected = MishkaConfig {
            project: ProjectConfig {
                version: 1,
                files: vec!["required_folder/*".to_string()],
                field: vec![
                    FieldConfig {
                        name: "field1".to_string(),
                        kind: "string".to_string(),
                        prompt: Some("What field1 would be?".to_string()),
                        variants: Some(vec!["a".to_string(), "b".to_string()]),
                        default_value: None,
                        template: Some(vec![TemplateConfig {
                            value: Value::String("a".to_string()),
                            files: vec!["another_folder/file".to_string()],
                        }]),
                    },
                    FieldConfig {
                        name: "field2".to_string(),
                        kind: "bool".to_string(),
                        prompt: None,
                        variants: None,
                        default_value: Some(Value::Boolean(true)),
                        template: Some(vec![TemplateConfig {
                            value: Value::Boolean(false),
                            files: vec!["!required_folder/disabled_file".to_string()],
                        }]),
                    },
                ],
            },
        };
        let parsed_config: MishkaConfig = toml::from_str(PROJECT_CONFIG).unwrap();
        assert_eq!(parsed_config, expected);
    }
}
