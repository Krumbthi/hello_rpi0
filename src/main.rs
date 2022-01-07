use std::io::Write;
// use std::sync::mpsc::channel;
use std::{thread, time::Duration};
use log::{info, debug, error};

use std::fs::File;
use std::io::{Seek, SeekFrom};

use std::path::Path;
use serde_json::json;

mod bme280;
use bme280::BME280;

fn main() -> std::io::Result<()> {
    info!("i2c test on rpi0");

    let mut bme280 = BME280::new();
    bme280.init().unwrap();
    let shmem_flink = "/dev/shm/environment.json";

    let path = Path::new(shmem_flink);
    let path_disp = path.display();
    debug!("{}", &path_disp);

    /*let mut out_file = match File::create(&path) {
        Err(err) => panic!("couldn't create {}: {}", path_disp, err),
        Ok(file) => file,
    };*/

    let mut out_file = File::create(&path)?;

    loop {
        info!("----------------------------");
        let meas = bme280.measure().unwrap();
        info!("Rel. humidity: {} %", meas.humidity);
        info!("Temperature:   {} C", meas.temperature);
        info!("Pressure:      {} Pa", meas.pressure);
        //let payload = format!("{{\"Data\": {{\"Temperature\":{}, \"Humidity\":{}, \"Pressure\":{} }}}}", meas.temperature, meas.humidity, meas.pressure);
        let payload = json!({
            "Data": {
                "Temperature": meas.temperature, 
                "Humidity": meas.humidity, 
                "Pressure": meas.pressure
            }
        });
        
        /*match out_file.write(serde_json::to_string(&payload).unwrap().as_bytes()) {
            Err(err) => error!("Could not write to {}: {}", path_disp, err),
            Ok(_) => debug!("Data written to file"),
        }*/
        out_file.seek(SeekFrom::Start(0))?;
        out_file.write_all(serde_json::to_string(&payload).unwrap().as_bytes())?;
        out_file.sync_all()?;
        
        thread::sleep(Duration::from_secs(120)); 
    }

    std::process::exit(0);
}

