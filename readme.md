# Generate

### Template structure (for visualization)

```rust
// Abstract, just for visualisation
struct ProjectConfig {
  version: 1, // currently only 1 version
  files: Vec<String>, // file patterns to copy and transform on initialization
  field: Vec<FieldConfig>, // list of fields
}
// Each field value will be asked on init
struct FieldConfig<T> {
  name: String,
  kind: String, // "string" or "bool"
  variants: Option<Vec<T>>, // if string, you can specify available values
  default_value: Option<T>,
  template: Vec<TemplateConfig<T>>, // granular template control. Available for every field
}
struct TemplateConfig<T> {
  value: T, // specify value when config will be enabled
  files: Vec<String>, // file patterns to copy and transform on initialization
}

```

### Create template

`mishka.config.toml`

```toml
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
```

`required_folder/{{project.field1 | capitalize}}.rs` (transforms into `required_folder/A.rs`)

```rust
fn main()  {
  println!("{{ project.field1 }}")
}
```

### Add template

```bash
mishka add <template_link>
```

### Create project

```bash
mishka init
```

### Add module

```bash
mishka gen
...questions(Y/n)
```
