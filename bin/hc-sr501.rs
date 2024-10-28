use sysfs_gpio::{Direction, Pin};
use std::error::Error;
use std::thread;
use std::time::Duration;

const MOTION_SENSOR_PIN: u64 = 534; // GPIO pin to connect sensor OUT pin (in raspberry PI4B, update for your board)

pub struct MotionSensor {
    pin: Pin,
}

impl MotionSensor {
    // Creates a new MotionSensor instance on the specified GPIO pin.
    pub fn new(pin_number: u64) -> Result<Self, Box<dyn Error>> {
        let pin = Pin::new(pin_number);
        pin.export()?;
        pin.set_direction(Direction::In)?; // Set pin as input
        Ok(Self { pin })
    }

    // Check if motion is detected.
    pub fn detect_motion(&self) -> Result<bool, Box<dyn Error>> {
        Ok(self.pin.get_value()? == 1)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sensor = MotionSensor::new(MOTION_SENSOR_PIN)?;

    println!("Starting Motion Sensor ...");
    loop {
        // Check if a motion is detected
        if sensor.detect_motion()? {
            println!("Motion detected!");
            thread::sleep(Duration::from_millis(1000));
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }
}