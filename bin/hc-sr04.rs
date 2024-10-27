use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sysfs_gpio::{Pin, Direction};

const TRIG_PIN: u64 = 529; // GPIO pin to connect sensor TRIG (in raspberry PI4B, update for your board)
const ECHO_PIN: u64 = 539; // GPIO pin to connect sensor ECHO (in raspberry PI4B, update for your board)

fn main() -> Result<(), Box<dyn Error>> {
    // Set up the trigger and echo pins
    let trigger = Pin::new(TRIG_PIN);
    let echo = Pin::new(ECHO_PIN);

    // Export pins and set directions
    trigger.export()?;
    echo.export()?;
    sleep(Duration::from_millis(500)); // Allow time for the pin to be exported

    trigger.set_direction(Direction::Out)?;
    echo.set_direction(Direction::In)?;

    // Main loop to measure distance
    loop {
        // Send trigger pulse
        trigger.set_value(1)?; // Set trigger high
        sleep(Duration::from_micros(10)); // Trigger pulse for 10 microseconds
        trigger.set_value(0)?; // Set trigger low

        // Wait for echo pin to go high
        wait_for_edge(&echo, true)?;

        // Start timing
        let start = Instant::now();

        // Wait for echo pin to go low
        wait_for_edge(&echo, false)?;

        // End timing
        let duration = start.elapsed();

        // Calculate distance in centimeters
        let distance = duration.as_secs_f64() * 17150.0; // Speed of sound in cm/us
        println!("Distance: {:.2} cm", distance);

        // Sleep before the next measurement
        sleep(Duration::from_millis(500));
    }
}

fn wait_for_edge(pin: &Pin, rising: bool) -> Result<(), Box<dyn Error>> {
    loop {
        let value = pin.get_value()?;
        if value == (rising as u8) {
            break;
        }
        sleep(Duration::from_micros(10)); // Polling delay
    }
    Ok(())
}
