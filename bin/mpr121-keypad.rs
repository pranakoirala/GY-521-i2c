use rust_i2c::I2CDevice;
use std::io::{self};
use sysfs_gpio::{Pin, Direction, Edge};

const KEYPAD_I2C_PATH: &str = "/dev/i2c-1"; // Update to the correct I2C bus
const KEYPAD_ADDRESS: u8 = 0x5a; // Keypad I2C address
const INTERRUPT_PIN: u64 = 516; // GPIO pin for interrupt (in raspberry PI4B, update for your board)

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize I2C device for MPR121
    let mut mpr121 = I2CDevice::new(KEYPAD_I2C_PATH, KEYPAD_ADDRESS)?;

    // Configure the MPR121 (enable electrodes and set thresholds)
    configure_mpr121(&mut mpr121)?;

    println!("Waiting for touch input...");

    // Set up the interrupt pin
    let interrupt_pin = Pin::new(INTERRUPT_PIN);
    interrupt_pin.with_exported(|| {
        interrupt_pin.set_direction(Direction::In)?;
        interrupt_pin.set_edge(Edge::BothEdges)?;
        let mut poller = interrupt_pin.get_poller()?;

        loop {
            match poller.poll(1000)? {
                Some(_) => {
                    if let Err(e) = read_touch_status(&mut mpr121) {
                        eprintln!("Error reading touch status: {}", e);
                    }
                }
                None => {
                }
            }
            
        }
    })?;

    Ok(())
}

/// Configures the MPR121 sensor by writing to necessary registers.
fn configure_mpr121(mpr121: &mut I2CDevice) -> io::Result<()> {
    // Set touch and release thresholds for all electrodes
    for electrode in 0..12 {
        mpr121.write(&[0x41 + electrode * 2, 12])?; // Touch threshold
        mpr121.write(&[0x42 + electrode * 2, 6])?;  // Release threshold
    }

    // Enable all 12 electrodes by writing to the ELE_CFG register
    mpr121.write(&[0x5E, 0x0C]) // 0x0C enables electrodes 0-11
}

/// Reads the touch status (2 bytes) from the MPR121.
fn read_touch_status(mpr121: &mut I2CDevice) -> io::Result<()> {
    let data = mpr121.read_registers(0x00, 2)?; // Read TOUCH_STATUS_L and H
    let status = u16::from(data[0]) | (u16::from(data[1]) << 8); // Combine the bytes
    for i in 0..12 {
        if status & (1 << i) != 0 {
            println!("Key {} touched!", i);
        }
    }
    Ok(())
}
