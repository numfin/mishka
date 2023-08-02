use std::{hash::Hash, sync::Arc};

pub struct ProjectConfig {
    pub template_filepaths: Vec<String>,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub parent: String,
    pub field_name: String,
    pub prompt: Option<String>,
    pub declaration: FieldDeclaration,
}
impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.parent.hash(state);
        self.field_name.hash(state);
    }
}
pub enum FieldDeclaration {
    Str {
        variants: Option<Vec<String>>,
        default_value: Option<String>,
        templates: Vec<TemplateCondition<String>>,
    },
    Bool {
        default_value: Option<bool>,
        templates: Vec<TemplateCondition<bool>>,
    },
}
/// MY SUPER CONDITION
pub struct TemplateCondition<T> {
    pub value: T,
    pub files: Vec<String>,
}
impl Field {
    pub fn from_field(group: &str, field: super::mishka::FieldConfig) -> eyre::Result<Self> {
        let declaration = match field.kind.as_str() {
            "string" => FieldDeclaration::Str {
                variants: field.variants,
                default_value: field.default_value.map(|v| v.to_string()),
                templates: field
                    .template
                    .into_iter()
                    .flatten()
                    .map(|v| TemplateCondition {
                        files: v.files,
                        value: v.value.to_string(),
                    })
                    .collect(),
            },
            "bool" => FieldDeclaration::Bool {
                default_value: field.default_value.and_then(|v| v.as_bool()),
                templates: field
                    .template
                    .into_iter()
                    .flatten()
                    .map(|v| TemplateCondition {
                        files: v.files,
                        value: v.value.as_bool().unwrap_or_default(),
                    })
                    .collect(),
            },
            _ => return Err(eyre::eyre!("Kind {} not supported", field.kind)),
        };

        let field = Field {
            parent: group.to_string(),
            declaration,
            field_name: field.name,
            prompt: field.prompt,
        };
        Ok(field)
    }
}

pub fn init_project(config: &super::mishka::MishkaConfig) -> eyre::Result<()> {
    let mut globals = liquid::Object::new();
    for field in &config.project.field {
        let field = Field::from_field("project", field.to_owned())?;
    }

    let template = liquid::ParserBuilder::with_stdlib()
        .build()?
        .parse("Liquid {{test.dot}}")?;
    let output = template.render(&globals).unwrap();
    println!("{output}");
    Ok(())
}
