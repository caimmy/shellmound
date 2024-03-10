
use clap::Parser;
use chrono::Local;
use dotenv::dotenv;
use colored::Colorize;
use log::{trace, error};
use shellmound::init_once;

use shellmound::quant::history::data_handle::{load_stock_history_data, save_daily_data};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ClientArgs {
    #[arg(short, long)]
    ts_code: String,

    #[arg(short, long)]
    start_date: String,

    #[arg(short, long, default_value_t = String::from(""))]
    end_date: String
}

#[tokio::main]
async fn main() {
    init_once();
    trace!("trace infor");
    error!("error infor");
    dotenv().ok();
    let args = ClientArgs::parse();

    let ts_code = args.ts_code;
    let start_date = args.start_date;
    let mut end_date = args.end_date;

    if end_date == "" {
        end_date = Local::now().format("%Y%m%d").to_string();
    }
    
    let _start_time = chrono::Local::now().timestamp_millis();

    // println!("{}, {}, {}", ts_code, start_date, end_date);
    if let Ok(stock_daily_data_list) = load_stock_history_data(ts_code, start_date, end_date).await {
        let _ = save_daily_data(&stock_daily_data_list).await;
    }
    let _duration = (chrono::Local::now().timestamp_millis() - _start_time) / 1000;
    println!("{}", format!("this operation tooks {} millis", _duration).yellow());
}