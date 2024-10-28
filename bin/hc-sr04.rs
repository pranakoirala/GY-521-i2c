use std::error::Error;
use std::thread::sleep;
use std::time::{Duration, Instant};
use sysfs_gpio::{Pin, Direction};

const TRIG_PIN: u64 = 529; // GPIO pin to connect sensor TRIG (in raspberry PI4B, update for your board)
const ECHO_PIN: u64 = 539; // GPIO pin to connect sensor ECHO (in raspberry PI4B, update for your board)

pub struct HCSR04 {
    trig: Pin,
    echo: Pin,
}

impl HCSR04 {
    pub fn new(trig_pin_num: u64, echo_pin_num: u64) -> Result<Self, Box<dyn Error>> {
        let trig = Pin::new(trig_pin_num);
        trig.export()?;
        trig.set_direction(Direction::Out)?;

        let echo = Pin::new(echo_pin_num);
        echo.export()?;
        echo.set_direction(Direction::In)?;

        Ok(Self { trig, echo})
    }

    pub fn read_dist_cm(&self) -> Result<f64, Box<dyn Error>> {
        // Send trigger pulse
        self.trig.set_value(1)?; // Set trigger high
        sleep(Duration::from_micros(10)); // Trigger pulse for 10 microseconds
        self.trig.set_value(0)?; // Set trigger low

        // Wait for echo pin to go high
        self.wait_for_edge(true)?;

        // Start timing
        let start = Instant::now();

        // Wait for echo pin to go low
        self.wait_for_edge(false)?;

        // End timing
        let duration = start.elapsed();

        // Calculate distance in centimeters with speed of sound in cm/us
        Ok(duration.as_secs_f64() * 17150.0)
    }

    fn wait_for_edge(&self, rising: bool) -> Result<(), Box<dyn Error>> {
        loop {
            let value = self.echo.get_value()?;
            if value == (rising as u8) {
                break;
            }
            sleep(Duration::from_micros(10)); // Polling delay
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sensor = HCSR04::new(TRIG_PIN, ECHO_PIN)?;

    // Main loop to measure distance
    loop {
        // Read the object distance in cm
        let distance = sensor.read_dist_cm()?;
        println!("Distance: {:.2} cm", distance);

        // Sleep before the next measurement
        sleep(Duration::from_millis(500));
    }
}
