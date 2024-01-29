use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
/// Schema for adding a datapack
pub struct AddDatapackSchema {
    /// Datapack git url
    pub url: String,
    /// Target branch name
    pub branch: Option<String>,
    /// Name of the datapack
    ///
    /// (Saved as the datapack folder)
    pub name: Option<String>,
}

#[derive(Clone, Serialize, Debug)]
/// Schema for updating a datapack
pub struct UpdateDatapackSchema {
    /// Which pack
    pub pack: String,
    /// Target branch name, if any
    ///
    /// defaults to master
    pub branch: Option<String>,
    // Todo: add commit checkout
}

#[derive(Clone, Deserialize)]
/// Application config
pub struct Config {
    /// Key to use for auth
    pub api_key: String,
    /// Api base base url
    pub api_url: String,
}

impl Config {
    /// Tries to load the config from ~/.mctlcli/config.toml
    ///
    /// Panics if any part fails
    // This is okay, because this is a simple cli app and panicing is fine if we encounter an error
    pub fn load() -> Config {
        let config_path = home::home_dir().unwrap().join(".mctlcli/config.toml");

        let config_result = std::fs::read_to_string(config_path.clone());

        if let Err(ref e) = config_result {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    println!(
                        "Config not found, please make one at {:?} and rerun.",
                        config_path
                    );
                }
                std::io::ErrorKind::PermissionDenied => {
                    println!("Could not read config file due to permission issue");
                }
                _ => {
                    println!("An error occured when loading config: {}", e);
                }
            }
            std::process::exit(3);
        }

        let config_contents = config_result.unwrap();

        let deserialized = toml::from_str::<Config>(&config_contents).unwrap();

        return deserialized;
    }
}
