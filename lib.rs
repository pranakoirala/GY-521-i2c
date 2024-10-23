use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

use nix::ioctl_write_int_bad; // Import for IOCTL operations

// Define the IOCTL request to set the I2C slave address
const I2C_SLAVE: u32 = 0x0703; // IOCTL command

// Use the ioctl macros directly to define I2C slave
ioctl_write_int_bad!(i2c_set_slave, I2C_SLAVE);

/// Represents an I2C device.
pub struct I2CDevice {
    file: std::fs::File,
}

impl I2CDevice {
    /// Creates a new I2C device at the given path with the specified address.
    pub fn new(path: &str, address: u8) -> io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).open(path)?;
        let fd = file.as_raw_fd();

        // Use the ioctl function to set the slave address
        unsafe { i2c_set_slave(fd, address as i32)? };
        Ok(Self { file })
    }

    /// Writes data to the I2C device.
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.file.write_all(data)
    }

    /// Reads data from the I2C device.
    pub fn read(&mut self, buffer: &mut [u8]) -> io::Result<()> {
        self.file.read_exact(buffer)
    }

    /// Read multiple registers value from I2C device at starting register address up to given length.
    pub fn read_registers(&mut self, start_reg: u8, length: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; length];
        self.write(&[start_reg])?; // Send the starting register address
        self.read(&mut buffer)?; // Read the data into the buffer
        Ok(buffer)
    }
}
