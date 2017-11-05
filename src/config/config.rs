extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    application_keys: Vec<String>,
    apis: APIs,
    data: Data
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct APIs {
    default: APIsDefault
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct APIsDefault {
    base_address: String,
    timeout: i32,
    endpoints: APIEndpointsDefault
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct APIEndpoint {
    method: String,
    url: String
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct APIEndpointsDefault{
    retrieve_users: APIEndpoint
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Data {
    relational: DataRelational
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct DataRelational {
    host: String,
    port: i32
}