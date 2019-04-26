use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Adapters {
    #[serde(rename = "type")]
    _type: String,
    config: Config,
    output: Output,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Output {}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    adapters: Vec<Adapters>,
}

pub mod file_reader {
    use config::*;
    use std::fs::File;
    use std::io::Read;

    pub fn read_config_from_file(file: &str) -> AppConfig {
        let mut file = File::open(file).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        println!("{}", content);
        let cfg = serde_json::from_str::<AppConfig>(&content);
        println!("{:?}", cfg);
        let adapters: Vec<Adapters> = vec![];
        return AppConfig { adapters };
    }
}
