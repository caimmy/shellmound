pub mod migrator;
pub mod models;
pub mod quant;

use simplelog::*;
use std::fs::File;

pub fn init_once() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("shellmound.log").unwrap())
        ]
    ).unwrap();
}