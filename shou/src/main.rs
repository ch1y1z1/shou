mod servo;
mod wifi;

use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::Context;
use embedded_svc::{
    http::{Headers, Method},
    io::{Read, Write},
};
use esp_idf_hal::{
    delay::FreeRtos,
    ledc::{LedcTimerDriver, Resolution},
    prelude::*,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::{Configuration, EspHttpServer},
    nvs::EspDefaultNvsPartition,
    wifi::{BlockingWifi, EspWifi},
};
use esp_idf_sys as _;
use log::*;
use serde::Deserialize;

use crate::wifi::connect_wifi;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    connect_wifi(&mut wifi, "818", "18251710519")?;
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    info!("Wifi DHCP info: {:?}", ip_info);

    info!("initializing servos");

    let config = esp_idf_hal::ledc::config::TimerConfig::new()
        .frequency(50.Hz().into())
        .resolution(Resolution::Bits12);
    let timer0 = Arc::new(LedcTimerDriver::new(peripherals.ledc.timer0, &config)?);
    let fingers = [
        servo::Servo::new(
            peripherals.ledc.channel0,
            timer0.clone(),
            peripherals.pins.gpio10,
        )?,
        servo::Servo::new(
            peripherals.ledc.channel1,
            timer0.clone(),
            peripherals.pins.gpio11,
        )?,
        servo::Servo::new(
            peripherals.ledc.channel2,
            timer0.clone(),
            peripherals.pins.gpio12,
        )?,
        servo::Servo::new(
            peripherals.ledc.channel3,
            timer0.clone(),
            peripherals.pins.gpio13,
        )?,
        servo::Servo::new(
            peripherals.ledc.channel4,
            timer0.clone(),
            peripherals.pins.gpio14,
        )?,
    ];
    let fingers = Arc::new(Mutex::new(fingers));

    info!("start server");

    let mut server = EspHttpServer::new(&Configuration {
        http_port: 80,
        https_port: 443,
        max_sessions: 10,
        session_timeout: Duration::from_secs(5),
        stack_size: 7000,
        ..Default::default()
    })?;

    server.fn_handler("/", Method::Get, |req| {
        let mut response = req.into_ok_response()?;
        response.write_all(include_str!("../static/index.html").as_bytes())?;
        Ok(())
    })?;

    #[derive(Deserialize, Debug)]
    struct AngleData {
        num: usize,
        angle: u32,
    }
    let fingers1 = fingers.clone();
    server.fn_handler("/api/set_angle", Method::Post, move |mut req| {
        let len = req.content_len().unwrap() as usize;

        if len > 512 {
            req.into_status_response(413)?
                .write_all("Payload too large".as_bytes())?;
            return Ok(());
        }
        let mut buf = vec![0; len];
        req.read_exact(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        // println!("Received: {:?}", buf);

        if let Ok(AngleData { num, angle }) = serde_json::from_slice(&buf) {
            info!("num:{num}, angle:{angle}");
            if num >= 5 {
                resp.write_all("Invalid num".as_bytes())?;
            } else if angle > 90 {
                resp.write_all("Invalid angle".as_bytes())?;
            } else {
                let mut fingers = fingers1.lock()?;
                fingers
                    .get_mut(num)
                    .context("num invalid")?
                    .set_angle(angle)?;
                resp.write_all("OK".as_bytes())?;
            }
        } else {
            resp.write_all("Invalid request".as_bytes())?;
        }
        Ok(())
    })?;

    #[derive(Deserialize, Debug)]
    struct AngleDatas {
        data: Vec<AngleData>,
    }
    server.fn_handler("/api/set_angles", Method::Post, move |mut req| {
        let len = req.content_len().unwrap() as usize;

        if len > 512 {
            req.into_status_response(413)?
                .write_all("Payload too large".as_bytes())?;
            return Ok(());
        }
        let mut buf = vec![0; len];
        req.read_exact(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        // println!("Received: {:?}", buf);

        if let Ok(AngleDatas { data }) = serde_json::from_slice(&buf) {
            info!("data: {:?}", data);
            for AngleData { num, angle } in data {
                if num >= 5 {
                    resp.write_all("Invalid num".as_bytes())?;
                    return Ok(());
                } else if angle > 90 {
                    resp.write_all("Invalid angle".as_bytes())?;
                    return Ok(());
                } else {
                    let mut fingers = fingers.lock()?;
                    fingers
                        .get_mut(num)
                        .context("num invalid")?
                        .set_angle(angle)?;
                }
            }
            resp.write_all("OK".as_bytes())?;
        } else {
            resp.write_all("Invalid request".as_bytes())?;
        }
        Ok(())
    })?;

    info!("all status ok");

    loop {
        FreeRtos::delay_ms(500);
    }
    // Ok(())
}
