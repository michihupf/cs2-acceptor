use core::time;
use std::{
    process::exit,
    thread::{self, sleep},
    time::Duration,
};

use image::Pixel;
use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use xcap::Monitor;

fn main() {
    // let button = image::load_from_memory(include_bytes!("button.png"))
    //     .unwrap()
    //     .into_rgb8();

    // let button_color = button.pixels().next().unwrap().channels();
    // println!("{:?}", button_color);
    let button_color = [55, 182, 82];

    let monitors = Monitor::all().unwrap();
    let monitor = monitors.iter().find(|m| m.is_primary()).unwrap();
    println!(
        "Found monitor {}. x,y: ({}, {})",
        monitor.name(),
        monitor.x(),
        monitor.y()
    );

    println!("Setting up failsafe key [B].");
    thread::spawn(|| {
        if let Err(e) = listen(callback) {
            println!("Error: {:?}", e);
        }
    });

    println!("Starting CS2 Acceptor in 3 seconds.");
    sleep(Duration::from_secs(3));
    println!("Acceptor started.");

    loop {
        // capture screen and find button
        let image = monitor.capture_image().unwrap();
        let mut same_count = 0;
        'outer: for y in 0..image.height() {
            for x in 0..image.width() {
                let color = image.get_pixel(x, y).channels();
                if color[0] == button_color[0]
                    && color[1] == button_color[1]
                    && color[2] == button_color[2]
                {
                    same_count += 1;
                }

                if same_count == 10 {
                    // click on the button
                    let x = monitor.x() as f64 + x as f64 + 10f64;
                    let y = monitor.y() as f64 + y as f64 + 20f64;
                    send(&EventType::MouseMove { x, y });
                    send(&EventType::ButtonPress(rdev::Button::Left));
                    break 'outer;
                }
            }
        }

        sleep(Duration::from_secs(1));

        // look for loading screen to quit automatically [FUTURE VERSION]
        // let image = image::DynamicImage::from(image).into_luma8();
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }

    // let OS catch up
    thread::sleep(delay);
}

fn callback(event: Event) {
    if let EventType::KeyPress(Key::KeyB) = event.event_type {
        println!("Nuking process!");
        exit(0);
    }
}
