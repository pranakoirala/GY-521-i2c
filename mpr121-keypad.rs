use rust_i2c::I2CDevice;
use std::io::{self};

const KEYPAD_I2C_PATH: &str = "/dev/i2c-1"; // Update to the correct I2C bus
const KEYPAD_ADDRESS: u8 = 0x5a; // Keypad I2C address

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize I²C device for MPR121 on i2c bus and address
    let mut mpr121 = I2CDevice::new(KEYPAD_I2C_PATH, KEYPAD_ADDRESS)?;

    // Configure the MPR121 (enable electrodes and set thresholds)
    configure_mpr121(&mut mpr121)?;

    println!("Waiting for touch input...");

    // Infinite loop to monitor touch status
    loop {
        let status = read_touch_status(&mut mpr121)?;
        for i in 0..12 {
            if status & (1 << i) != 0 {
                println!("Electrode {} touched!", i);
            }
        }

        // Sleep for 100ms before polling again
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
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
fn read_touch_status(mpr121: &mut I2CDevice) -> io::Result<u16> {
    let data = mpr121.read_registers(0x00, 2)?; // Read TOUCH_STATUS_L and H
    Ok(u16::from(data[0]) | (u16::from(data[1]) << 8)) // Combine the bytes
}
