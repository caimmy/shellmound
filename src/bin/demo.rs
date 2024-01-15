use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use reqwest;

type Params = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
enum ResponseDailyData {
    ts_code(String),
    date(String),
    price(f32)
}

#[derive(Debug, Deserialize)]
struct DailyData {
    fields: Vec<String>,
    items: Value,
    has_more: bool
}

#[derive(Debug, Deserialize)]
struct ResponseData {
    code: u32,
    msg: String,
    request_id: String,
    data: DailyData
    // data: Value
}

#[derive(Debug, Serialize)]
struct RequestData {
    token: String,
    api_name: String,
    params: Params,
    fields: String
}

impl RequestData {
    fn new(api_name: String, params: Params, fields: String) -> RequestData {
        RequestData{
            token: String::from("6a96df2c38e99ec7255cd8aee1c233cb373641bd8bbc8cc612433bb8"),
            api_name: api_name,
            params: params, 
            fields: fields
        }
    }
}


fn main() {
    let mut param = HashMap::new();
    param.insert("ts_code".to_string(), "601318.SH".to_string());
    param.insert("start_date".to_string(), "20220101".to_string());
    param.insert("end_date".to_string(), "20220109".to_string());

    let req_data = RequestData::new(
        String::from("daily"), 
        param, 
        "".to_string());

    let client = reqwest::blocking::Client::new();

    let s = client.post("https://api.tushare.pro/")
        .json(&req_data)
        .send().unwrap()
        .text().unwrap();
    println!("{:#?}", s);
    let m: ResponseData = serde_json::from_str(&s).unwrap();
    println!("{:#?}", m);
    println!("{:#?}", m.data.items[0][0].to_string());
    // println!("{:#?}", m.get("data").unwrap().get("fields").unwrap().as_array().unwrap()[1].as_str().unwrap());
}

