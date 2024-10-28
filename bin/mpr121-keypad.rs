use rust_i2c::I2CDevice;
use sysfs_gpio::{Direction, Edge, Pin, PinPoller};
use std::error::Error;

const KEYPAD_I2C_PATH: &str = "/dev/i2c-1"; // Update to the correct I2C bus
const KEYPAD_ADDRESS: u8 = 0x5a; // Keypad I2C address
const INTERRUPT_PIN: u64 = 516; // GPIO pin for interrupt (in raspberry PI4B, update for your board)

struct MPR121 {
    dev: I2CDevice,
    pin_poller: PinPoller,
}

impl MPR121 {
    pub fn new(i2c_dev_path: &str, dev_addr: u8, interrupt_pin_num: u64) -> Result<Self, Box<dyn Error>> {
        let mut dev = I2CDevice::new(&i2c_dev_path, dev_addr)?;
        
        // Set touch and release thresholds for all electrodes
        for electrode in 0..12 {
            dev.write(&[0x41 + electrode * 2, 12])?; // Touch threshold
            dev.write(&[0x42 + electrode * 2, 6])?;  // Release threshold
        }

        // Enable all 12 electrodes by writing to the ELE_CFG register
        dev.write(&[0x5E, 0x0C])?; // 0x0C enables electrodes 0-11

        // Set up the interrupt pin
        let interrupt_pin = Pin::new(interrupt_pin_num);
        interrupt_pin.export()?;
        interrupt_pin.set_direction(Direction::Out)?;
        interrupt_pin.set_edge(Edge::BothEdges)?;
        
        // Set up poller for the interrupt pin
        let pin_poller = interrupt_pin.get_poller()?;
        Ok(Self {dev, pin_poller})
    }

    /// Reads the touch status (2 bytes) from the MPR121.
    pub fn check_touch(&mut self) -> Result<u16, Box<dyn Error>> {
        // Poll for 1000 milliseconds
        match self.pin_poller.poll(1000)? {
            Some(_) => {
                let data = self.dev.read_registers(0x00, 2)?; // Read TOUCH_STATUS_L and H
                let status = u16::from(data[0]) | (u16::from(data[1]) << 8); // Combine the bytes
                for i in 0..12 {
                    if status & (1 << i) != 0 {
                        return Ok(i); 
                    }
                }
            }
            None => {
            }
        }
        Err("Touch not detected".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sensor = MPR121::new(KEYPAD_I2C_PATH, KEYPAD_ADDRESS, INTERRUPT_PIN)?;

    println!("Waiting for touch input ...");

    loop {
        match sensor.check_touch() {
            Ok(val) => println!("Key: {} touched", val),
            Err(e) => println!("{}", e)
        }
    }
}
