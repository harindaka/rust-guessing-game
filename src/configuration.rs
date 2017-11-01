extern crate serde;
extern crate serde_json;

use config;

pub fn build_config(){
    let config_build = include_str!("config/build.json");
    let config = include_str!("config/config.json");
    let config_qa = include_str!("config/config_qa.json");
    let config_prod = include_str!("config/config_prod.json");

    let config: config::Config = serde_json::from_str(&config).unwrap();
    let config_qa: config::Config = serde_json::from_str(&config_qa).unwrap();
    let config_prod: config::Config = serde_json::from_str(&config_prod).unwrap();

    // #[derive(Serialize, Deserialize)]  
    // let merged_config = {
    //     ..config_qa,
    //     ..config
    // };
    
    let json = serde_json::to_string(&config).unwrap();
    
    println!("{}", json);
    // println!("{}", config);
    // println!("{}", config_qa);
    // println!("{}", config_prod);
}