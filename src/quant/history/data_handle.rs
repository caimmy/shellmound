/// 
/// via tushare get stock's history data 
/// 

use reqwest;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};
use sea_orm::{Database, DbErr, ActiveValue, EntityTrait, QueryFilter, ColumnTrait};
use crate::models::entities::{prelude::*, daily_data};
use colored::*;

const DATA_SOURCE_API: &'static str = "https://api.tushare.pro/";

#[derive(Debug, Deserialize)]
struct ResponseTushare {
    code: i32,
    msg: String,
    message: Option<String>,
    data: Option<Value>
}

type Params = HashMap<String, String>;

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

/// History data of stock code
/// 
#[derive(Debug, Deserialize, Clone)]
pub struct StockDailyData {
    ts_code: String,
    trade_date: Option<String>,
    open: Option<f32>,
    high: Option<f32>,
    low: Option<f32>,
    close: Option<f32>,
    pre_close: Option<f32>,
    change: Option<f32>,
    pct_chg: Option<f32>,
    vol: Option<f32>,
    amount: Option<f32>
}

impl StockDailyData {
    fn build_from_vec (
        ts_code: String,
        trade_date: Option<String>,
        open: Option<f32>,
        high: Option<f32>,
        low: Option<f32>,
        close: Option<f32>,
        pre_close: Option<f32>,
        change: Option<f32>,
        pct_chg: Option<f32>,
        vol: Option<f32>,
        amount: Option<f32>
    ) -> Self {
        StockDailyData {
            ts_code,
            trade_date, 
            open, 
            high, 
            low, 
            close, 
            pre_close, 
            change, 
            pct_chg, 
            vol, 
            amount
        }
    }    
}

/// Load stock data from tushare platform
pub async fn load_stock_history_data(ts_code: String, start_date: String, end_date: String) -> Result<Vec<StockDailyData>, String> {
    let mut _param = HashMap::<String, String>::new();
    _param.insert("ts_code".to_owned(), ts_code);
    _param.insert("start_date".to_owned(), start_date);
    _param.insert("end_date".to_owned(), end_date);

    let mut has_more: bool = true;

    let _request_data = RequestData::new("daily".to_owned(), _param, "".to_owned());

    let client = reqwest::Client::new();
    let _request_res: Result<Vec<StockDailyData>, String> = match client.post(DATA_SOURCE_API).json(&_request_data).send().await {
        Ok(_req) => {
            let mut data_vec: Vec<StockDailyData> = vec![];
            if let Ok(_unwrap_data) = _req.json::<ResponseTushare>().await {
                if _unwrap_data.code == 0 {
                    // TODO: construct Vector there
                    match _unwrap_data.data {
                        Some(_data_detail) => {
                            let _has_more = _data_detail.get("has_more").unwrap().as_bool().unwrap();
                            has_more = _has_more;
                            if let Some(data_list) = _data_detail.get("items") {
                                
                                if let Some(_daily_vec) = data_list.as_array() {
                                    for _daily_data in _daily_vec.iter() {
                                        let _v = StockDailyData::build_from_vec(
                                            _daily_data[0].as_str().unwrap().to_string(), 
                                            Some(_daily_data[1].as_str().expect("").to_string()), 
                                            Some(_daily_data[2].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[3].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[4].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[5].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[6].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[7].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[8].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[9].to_string().parse::<f32>().unwrap()), 
                                            Some(_daily_data[10].to_string().parse::<f32>().unwrap())
                                        );
                                        data_vec.push(_v);
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                };
            };
            println!("length is {}", data_vec.len());
            Ok(data_vec.clone())
        },
        Err(e) => Err(e.to_string())
    };

    println!("has more is {}", has_more);
    _request_res
}


/// save stock daily data in mysql database
pub async fn save_daily_data(daily_data: &Vec<StockDailyData>) -> Result<u32, DbErr>{
    let _DB_URL: String = std::env::var("DBURL").unwrap();
    
    if daily_data.len() > 0 {
        let _tips = format!("handle data with length {}", daily_data.len()).red();
        println!("{}", _tips);
        let db = Database::connect(_DB_URL).await?;
        
        for _item in daily_data.iter() {
            let _cp_ts_code = _item.ts_code.clone().to_string();
            let _cp_trade_date = _item.trade_date.as_ref().unwrap().clone().to_string();
            let _exists_item = DailyData::find().filter(
                daily_data::Column::TsCode.eq(&_cp_ts_code)
            ).filter(
                daily_data::Column::TradeDate.eq(&_cp_trade_date)
            ).one(&db).await?;
            match _exists_item {
                Some(_) => {
                    println!("{}", format!("{}, {} had exists!", &_cp_ts_code, _cp_trade_date).green());
                },
                None => {
                    let _daily_stock_data = daily_data::ActiveModel {
                        ts_code: ActiveValue::set(_item.ts_code.clone()),
                        trade_date: ActiveValue::set(_item.trade_date.clone()),
                        open:ActiveValue::set(_item.open),
                        high: ActiveValue::set(_item.high),
                        low: ActiveValue::set(_item.low),
                        close: ActiveValue::set(_item.close),
                        change: ActiveValue::set(_item.change),
                        pct_chg: ActiveValue::set(_item.pct_chg),
                        vol: ActiveValue::set(_item.vol),
                        amount: ActiveValue::set(_item.amount),
                        pre_close: ActiveValue::set(_item.pre_close),
                        interval: ActiveValue::set(Some("daily".to_owned())),
                        ..Default::default()
                    };
                    let _ins_res = DailyData::insert(_daily_stock_data).exec(&db).await?;
                    if _ins_res.last_insert_id > 0 {
                        println!("{}", format!("{}, {} inserted successfully", _item.ts_code.clone(), _item.trade_date.as_ref().expect("").clone()).blue());
                    }
                }
            }
        }
    }
    
    
    Ok(0)
}