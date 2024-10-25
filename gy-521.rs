use std::error::Error;
use rust_i2c::I2CDevice;
use std::thread;
use std::time::Duration;

const GY521_I2C_PATH: &str = "/dev/i2c-1"; // Update to the correct I2C bus
const GY521_ADDRESS: u8 = 0x68; // GY-521 I2C address

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new I2C device
    let mut device = I2CDevice::new(GY521_I2C_PATH, GY521_ADDRESS)?;

    // Wake the device by writing to the power management register
    let register: u8 = 0x6B;
    let value: u8 = 0;
    device.write(&[register, value])?;

    loop {
         // Read accelerometer values
        let accel_data = device.read_registers(0x3B, 6)?;
        let ax = ((accel_data[0] as i16) << 8) | (accel_data[1] as i16);
        let ay = ((accel_data[2] as i16) << 8) | (accel_data[3] as i16);
        let az = ((accel_data[4] as i16) << 8) | (accel_data[5] as i16);

        // Read gyroscope values
        let gyro_data = device.read_registers(0x43, 6)?;
        let gx = ((gyro_data[0] as i16) << 8) | (gyro_data[1] as i16);
        let gy = ((gyro_data[2] as i16) << 8) | (gyro_data[3] as i16);
        let gz = ((gyro_data[4] as i16) << 8) | (gyro_data[5] as i16);

        // Print accelerometer and gyroscope values
        println!("Accelerometer: x = {}, y = {}, z = {}", ax, ay, az);
        println!("Gyroscope: x = {}, y = {}, z = {}", gx, gy, gz);
        thread::sleep(Duration::from_secs(1));
    }
}
