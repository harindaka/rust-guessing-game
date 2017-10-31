//extern crate serde;
extern crate serde_json;

//#[macro_use]
//extern crate serde_derive;
mod config;
use self::serde_json::Error;


pub fn build_config(){
    let config_build = include_str!("config/build.json");
    let config = include_str!("config/config.json");
    let config_qa = include_str!("config/config_qa.json");
    let config_prod = include_str!("config/config_prod.json");

    let deserialized_config:config::Config = serde_json::from_str(config)?;

    
    println!("{}", config_build);
    println!("{}", config);
    println!("{}", config_qa);
    println!("{}", config_prod);
}