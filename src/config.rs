const CONFIG_PATH: &str = "im.toml";

use confy::ConfyError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Default, Deserialize)]
pub struct ImConfig {
    pub location: Option<String>,
    pub web_hook: Option<String>
}

pub fn read_config() -> Option<ImConfig> {
    let c: Result<ImConfig, ConfyError> = confy::load_path(dirs::config_dir().unwrap().join(CONFIG_PATH).as_path());
    if c.is_err() {
        return None;
    }
    Some(c.unwrap())
}


#[cfg(test)]
mod tests {
    use crate::config::read_config;

    #[test]
    fn test() {
        let c = read_config();
        println!("{:?}", c);
    }
}