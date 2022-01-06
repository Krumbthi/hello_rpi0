// use std::sync::mpsc::channel;
use std::{thread, time::Duration};
use log::{info, debug, error};
use env_logger::Env;

use shared_memory::*;
use std::sync::atomic::{AtomicU8, Ordering};
use raw_sync::locks::*;
use bytes::Bytes;

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
        Err(ShmemError::LinkExists) => ShmemConf::new().flink(shmem_flink).open().unwrap(),
        Err(e) => {
            eprintln!("Unable to create or open shmem flink {} : {}", shmem_flink, e);
            return;
        }
    };
    debug!("{}", String::from(shmem.get_os_id()));

    // Get pointer to the shared memory
    let mut raw_ptr = shmem.as_ptr();
    let is_init: &mut AtomicU8;
    let mutex: Box<dyn LockImpl>;

    unsafe {
        is_init = &mut *(raw_ptr as *mut u8 as *mut AtomicU8);
    };

    // Initialize or wait for initialized mutex
    if shmem.is_owner() {
        is_init.store(0, Ordering::Relaxed);
        // Initialize the mutex
        let (lock, _bytes_used) = unsafe {
            Mutex::new(
                raw_ptr,                                    // Base address of Mutex
                raw_ptr.add(Mutex::size_of(Some(raw_ptr))), // Address of data protected by mutex
            )
            .unwrap()
        };
        is_init.store(1, Ordering::Relaxed);
        mutex = lock;
    } else {
        // wait until mutex is initialized
        while is_init.load(Ordering::Relaxed) != 1 {}
        // Load existing mutex
        let (lock, _bytes_used) = unsafe {
            Mutex::from_existing(
                raw_ptr,                                    // Base address of Mutex
                raw_ptr.add(Mutex::size_of(Some(raw_ptr))), // Address of data  protected by mutex
            )
            .unwrap()
        };
        mutex = lock;
    }

    loop {
        let mut guard = mutex.lock().unwrap();
        // Cast mutex data to &mut u8
        let val: &mut u8 = unsafe { &mut **guard };

        info!("----------------------------");
        let meas = bme280.measure().unwrap();
        info!("Rel. humidity: {} %", meas.humidity);
        info!("Temperature:   {} C", meas.temperature);
        info!("Pressure:      {} Pa", meas.pressure);
        let payload = format!("{{\"Data\": {{\"Temperature\":{}, \"Humidity\":{}, \"Pressure\":{} }}}}", meas.temperature, meas.humidity, meas.pressure);
        let mut data = Bytes::from(payload);
        unsafe {
            raw_ptr = raw_ptr.add(data.len());
        }
        
        //*val = <&[u8; data.len()]>::try_from(data);
        let v = data.slice(0..data.len());
        debug!("{:?}", v);

        thread::sleep(Duration::from_secs(120)); 
    }

    std::process::exit(0);
}

