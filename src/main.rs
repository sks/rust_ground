mod config;
extern crate serde;
extern crate serde_derive;

fn main() {
    let file: &str = "data/config.json";
    config::file_reader::read_config_from_file(file);
}
