# Mire

> Super simple error handling in Rust


## Usage

```rs
use mire::{Result, bail, ensure, ContextExt};

fn read_config() -> Result<String> {
    let config_path = "config.toml";

    ensure!(config_path.ends_with(".toml"),
        "Invalid configuration file format, expected .toml");

    if rand::random::<bool>() {
        bail!("Random failure for demonstration");
    }

    let mut file = File::open(config_path)
        .context("Failed to open configuration file")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read contents of '{}'", config_path))?;

    ensure!(!contents.is_empty(), "Configuration file is empty");

    Ok(contents)
}
```
