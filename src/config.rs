//extern crate serde;
extern crate serde_json;

//#[macro_use]
//extern crate serde_derive;

use self::serde_json::Error;

#[derive(Serialize, Deserialize)]
pub struct Config {
    applicationKeys: Vec<String>,
    apis: APIs
    data: Data
}

#[derive(Serialize, Deserialize)]
pub struct APIs {
    default: APIs_Default
}

#[derive(Serialize, Deserialize)]
pub struct API {
    baseAddress: String,
    timeout: i32,
    endpoints: API_Endpoints_Default
}

#[derive(Serialize, Deserialize)]
pub struct API_Endpoint {
    method: String,
    url: String
}

#[derive(Serialize, Deserialize)]
pub struct API_Endpoints_Default{
    retrieveUsers: API_Endpoint
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    relational: Data_Relational
}

#[derive(Serialize, Deserialize)]
pub struct Data_Relational {
    host: String,
    port: i32
}