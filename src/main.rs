use std::{
    time,
    process::exit,
    thread::{self, sleep},
    time::Duration,
};

use image::Pixel;
use rdev::{listen, simulate, Event, EventType, Key, SimulateError};
use xcap::{Monitor, Window};

const VERSION: &str = env!("CARGO_PKG_VERSION");

// button color when not and when hovering
const BUTTON_COLOR: [u8; 3] = [55, 182, 82];
const HOVER_COLOR: [u8; 3] = [58, 198, 89];

fn main() {
    println!("CS2 Match Acceptor v{}", VERSION);
    println!("Setting up failsafe key [B].");
    thread::spawn(|| {
        if let Err(e) = listen(callback) {
            println!("Error: {:?}", e);
        }
    });

    println!("Looking for CS2...");
    // title should be same on all platforms
    while Window::all().unwrap().into_iter().find(|w| w.title() == "Counter-Strike 2").is_none() {
        sleep(Duration::from_secs(3));
    }

    let Some(mut cs2) = Window::all().unwrap().into_iter().find(|w| w.title() == "Counter-Strike 2") else {
        println!("CS2 is not running");
        sleep(Duration::from_secs(3));
        exit(0);
    };

    // On Windows monitors are positioned relative to the primary screen so clicking needs to be offset.
    let monitors = Monitor::all().unwrap();
    let offset_x = -&monitors.iter().map(|m| m.x()).min().unwrap();
    let offset_y = -&monitors.iter().map(|m| m.y()).min().unwrap();

    let monitor = cs2.current_monitor();
    println!(
        "Found CS2 on monitor {}. x,y: ({}, {})",
        monitor.name(),
        monitor.x() + offset_x,
        monitor.y() + offset_y
    );

    // update offsets for clicking later
    let offset_x = (monitor.x() + offset_x + 10) as f64;
    let offset_y = (monitor.y() + offset_y + 10) as f64;

    println!("Acceptor is ready!");

    loop {
        if let Err(err) = cs2.refresh() {
            // user probably closed CS2
            println!("ERROR: {}", err);
            exit(0);
        }
        if cs2.is_minimized() {
            println!("CS2 is minimized. Skipping screengrab.");
            sleep(Duration::from_secs(2));
            continue;
        }

        // capture screen and find button
        let image = monitor.capture_image().unwrap();
        let mut same_count = 0;
        'outer: for y in 0..image.height() {
            for x in 0..image.width() {
                let color = image.get_pixel(x, y).channels();
                if is_same_color(color, &BUTTON_COLOR) || is_same_color(color, &HOVER_COLOR) {
                    same_count += 1;
                } else {
                    same_count = 0;
                }

                if same_count == 10 {
                    // click on the button
                    println!("I found the button! x,y: ({}, {})", x, y);

                    send(&EventType::MouseMove {
                        x: x as f64 + offset_x,
                        y: y as f64 + offset_y,
                    });
                    send(&EventType::ButtonPress(rdev::Button::Left));
                    send(&EventType::ButtonRelease(rdev::Button::Left));
                    break 'outer;
                }
            }
        }

        sleep(Duration::from_secs(1));
    }
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(30);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }

    // let OS catch up
    thread::sleep(delay);
}

fn is_same_color(c1: &[u8], c2: &[u8]) -> bool {
    // maybe this is better? c1.iter().zip(c2.iter()).all(|x| x[0].abs_diff(x[1]) <= 2)
    c1[0].abs_diff(c2[0]) <= 2 && c1[1].abs_diff(c2[1]) <= 2 && c1[2].abs_diff(c2[2]) <= 2
}

fn callback(event: Event) {
    if let EventType::KeyPress(Key::KeyB) = event.event_type {
        println!("Nuking process!");
        exit(0);
    }
}
