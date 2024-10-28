use std::error::Error;
use rust_i2c::I2CDevice;
use std::thread;
use std::time::Duration;

const GY521_I2C_PATH: &str = "/dev/i2c-1"; // Update to the correct I2C bus
const GY521_ADDRESS: u8 = 0x68; // GY-521 I2C address

pub struct Cordinate {
    x: i16,
    y: i16,
    z: i16,
}

pub struct GY521 {
    dev: I2CDevice,
}

impl GY521 {
    pub fn new(i2c_dev_path: &str, dev_addr: u8) -> Result<Self, Box<dyn Error>> {
        let mut dev = I2CDevice::new(&i2c_dev_path, dev_addr)?;
        
        // Wake the device by writing to the power management register
        dev.write(&[0x6B, 0])?;

        Ok(Self {dev})
    }

    pub fn read_accel(&mut self) -> Result<Cordinate, Box<dyn Error>> {
        let accel_data = self.dev.read_registers(0x3B, 6)?;
        let ax = ((accel_data[0] as i16) << 8) | (accel_data[1] as i16);
        let ay = ((accel_data[2] as i16) << 8) | (accel_data[3] as i16);
        let az = ((accel_data[4] as i16) << 8) | (accel_data[5] as i16);

        Ok(Cordinate{x: ax, y: ay, z: az})
    }

    pub fn read_gyro(&mut self) -> Result<Cordinate, Box<dyn Error>> {
        let gyro_data = self.dev.read_registers(0x43, 6)?;
        let gx = ((gyro_data[0] as i16) << 8) | (gyro_data[1] as i16);
        let gy = ((gyro_data[2] as i16) << 8) | (gyro_data[3] as i16);
        let gz = ((gyro_data[4] as i16) << 8) | (gyro_data[5] as i16);

        Ok(Cordinate{x: gx, y: gy, z: gz})
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new I2C device
    let mut sensor = GY521::new(GY521_I2C_PATH, GY521_ADDRESS)?;

    loop {
         // Read accelerometer values
        let accel_data = sensor.read_accel()?;

        // Read gyroscope values
        let gyro_data = sensor.read_gyro()?;

        // Print accelerometer and gyroscope values
        println!("Accelerometer: x = {}, y = {}, z = {}", accel_data.x, accel_data.y, accel_data.z);
        println!("Gyroscope: x = {}, y = {}, z = {}", gyro_data.x, gyro_data.y, gyro_data.z);
        thread::sleep(Duration::from_secs(1));
    }
}
