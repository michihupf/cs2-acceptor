use core::time;
use std::{
    process::exit,
    thread::{self, sleep},
    time::Duration,
};

use image::Pixel;
use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use xcap::Monitor;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // let button = image::load_from_memory(include_bytes!("button.png"))
    //     .unwrap()
    //     .into_rgb8();

    // let button_color = button.pixels().next().unwrap().channels();
    // println!("{:?}", button_color);
    println!("CS2 Match Acceptor v{}", VERSION);

    let button_color = [55, 182, 82];
    let monitors = Monitor::all().unwrap();

    // On Windows monitors are positioned relative to the primary screen so clicking needs to be offset.
    let offset_x = -&monitors.iter().map(|m| m.x()).min().unwrap();
    let offset_y = -&monitors.iter().map(|m| m.y()).min().unwrap();
    let monitor = &monitors.iter().find(|m| m.is_primary()).unwrap();
    println!(
        "Found monitor {}. x,y: ({}, {})",
        monitor.name(),
        monitor.x() + offset_x,
        monitor.y() + offset_y
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
                if (color[0] as i16 - button_color[0] as i16).abs() <= 2
                    && (color[1] as i16 - button_color[1] as i16).abs() <= 2
                    && (color[2] as i16 - button_color[2] as i16).abs() <= 2
                {
                    same_count += 1;
                } else {
                    same_count = 0;
		}

                if same_count == 10 {
                    // click on the button
                    let x = monitor.x() as f64 + x as f64 + 10f64 + offset_x as f64;
                    let y = monitor.y() as f64 + y as f64 + 20f64 + offset_y as f64;

                    println!("I found the button! ({}, {})", x, y);

                    send(&EventType::MouseMove { x, y });
                    send(&EventType::ButtonPress(rdev::Button::Left));
                    sleep(Duration::from_millis(20));
                    send(&EventType::ButtonRelease(rdev::Button::Left));
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
