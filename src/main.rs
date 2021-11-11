// use std::sync::mpsc::channel;
use std::{thread, time::Duration};
use log::{info, debug, error};
use env_logger::Env;

mod bme280;
use bme280::BME280;

fn main() {
    let env = Env::filter_or(Env::default(), "APP_LOG_LEVEL", "debug")
        .write_style_or("APP_LOG_STYLE", "always");    
    env_logger::init_from_env(env);

    info!("i2c test on rpi0");

    let mut bme280 = BME280::new();
    bme280.init().unwrap();
    
    loop {
        info!("----------------------------");
        let measurements = bme280.measure().unwrap();
        info!("Rel. humidity: {} %", measurements.humidity);
        info!("Temperature:   {} C", measurements.temperature);
        info!("Pressure:      {} Pa", measurements.pressure);
        thread::sleep(Duration::from_secs(120)); 
    }
}

