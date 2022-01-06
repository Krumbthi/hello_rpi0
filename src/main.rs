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
    let shmem_flink = "environment.json";

    let shmem = match ShmemConf::new().size(4096).flink(shmem_flink).create() {
        Ok(m) => m,
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(shmem_flink).open()?,
        Err(e) => {
            eprintln!("Unable to create or open shmem flink {} : {}",shmem_flink, e);
            return;
        }
    };

    // Get pointer to the shared memory
    let raw_ptr = shmem.as_ptr();

    loop {
        info!("----------------------------");
        let meas = bme280.measure().unwrap();
        info!("Rel. humidity: {} %", meas.humidity);
        info!("Temperature:   {} C", meas.temperature);
        info!("Pressure:      {} Pa", meas.pressure);
        *raw_ptr = format!("{{\"Data\": {{\"Temperature\":{}, \"Humidity\":{}, \"Pressure\":{} }}}}", meas.temperature, meas.humidity, meas.pressure);

        thread::sleep(Duration::from_secs(120)); 
    }

    std::process::exit(0);
}

