use std::{env, fs, path::Path};

use mishka::configurator::{cli::init_project, mishka::MishkaConfig};

fn main() -> eyre::Result<()> {
    let cwd = env::current_dir()?;
    let config_path = Path::new(&cwd).join(Path::new("mishka.config.toml"));
    let project_config = fs::read_to_string(config_path)?;
    let parsed_config: MishkaConfig = toml::from_str(&project_config)?;
    let initialized_project = init_project(&parsed_config)?;
    Ok(())
}
