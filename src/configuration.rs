extern crate serde;
extern crate serde_json;

use config;

pub fn build_config(){
    let config_build = include_str!("config/build.json");
    let config = include_str!("config/config.json");
    let config_qa = include_str!("config/config_qa.json");
    let config_prod = include_str!("config/config_prod.json");

    let deserialized_config: config::Config = serde_json::from_str(config);

    
    println!("{}", config_build);
    println!("{}", config);
    println!("{}", config_qa);
    println!("{}", config_prod);
}