
extern crate serde;
extern crate serde_json;

use config;
use serde_json::Value;
use serde_json::from_str;

pub fn build_config(){
    
    let mut config_build: Value =from_str(include_str!("config/build.json")).unwrap();
    let mut config: Value = from_str(include_str!("config/config.json")).unwrap();
    let mut config_qa: Value = from_str(include_str!("config/config_qa.json")).unwrap();
    let mut config_prod: Value = from_str(include_str!("config/config_prod.json")).unwrap();

    // let mut config_map = config.as_object_mut().unwrap();
    // let config_map_qa = config_qa.as_object_mut().unwrap();
    
    // config_map.extend(config_map_qa);

    merge(&mut config, &config_qa);
    println!("{}", config.to_string());

    let mut john = json!({
      "name": "John Doe",
      "age": 43,
      "phones": [
        "+44 1234567",
        "+44 2345678"
      ]
    });

    let jane = json!({
      "name": "Jane Doe",
      "phones": [
        "+44 9876543"
      ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());

    merge(&mut john, &jane);

    println!("{}", john);
    
    // println!("{}", json);
}

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}