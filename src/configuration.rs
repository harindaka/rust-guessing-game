
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

    //merge(&mut config, &config_qa);
    merge(&mut config, config_qa);
    println!("{}", config.to_string());
}

// fn merge(a: &mut Value, b: &Value) {
//     match (a, b) {
//         (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {
//             for (k, v) in b {
//                 merge(a.entry(k.clone()).or_insert(Value::Null), v);
//             }
//         }
//         (a, b) => {
//             *a = b.clone();
//         }
//     }
// }

fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}